//! ILP solver implementation using HiGHS.

use crate::models::algebraic::{Comparison, ObjectiveSense, VariableDomain, ILP};
use crate::rules::{ReduceTo, ReductionResult};
#[cfg(not(feature = "ilp-highs"))]
use good_lp::default_solver;
#[cfg(feature = "ilp-highs")]
use good_lp::highs;
#[cfg(feature = "ilp-highs")]
use good_lp::solvers::highs::HighsParallelType;
use good_lp::{
    variable, ProblemVariables, ResolutionError, Solution, SolutionStatus, SolverModel, Variable,
};

/// A failure to produce a proven-optimal ILP solution.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum ILPSolveError {
    /// The constraints have no feasible assignment.
    #[error("the ILP is infeasible")]
    Infeasible,
    /// The objective is unbounded.
    #[error("the ILP objective is unbounded")]
    Unbounded,
    /// The configured time limit was reached before optimality was proven.
    #[error("the ILP solver reached its time limit before proving optimality")]
    Timeout,
    /// The selected backend failed for another reason.
    #[error("the ILP backend failed: {0}")]
    BackendFailure(String),
    /// Type-erased dispatch received a value other than a supported ILP variant.
    #[error("the ILP backend supports only ILP<bool> and ILP<i32>")]
    UnsupportedProblemType,
    /// A target witness could not be mapped back to the source problem.
    #[error(transparent)]
    Extraction(#[from] crate::rules::ExtractionError),
}

fn classify_backend_error(error: ResolutionError, time_limit: Option<f64>) -> ILPSolveError {
    match error {
        ResolutionError::Infeasible => ILPSolveError::Infeasible,
        ResolutionError::Unbounded => ILPSolveError::Unbounded,
        ResolutionError::Other("NoSolutionFound") if time_limit.is_some() => ILPSolveError::Timeout,
        other => ILPSolveError::BackendFailure(other.to_string()),
    }
}

/// An ILP solver using the HiGHS backend.
///
/// This solver solves Integer Linear Programming problems directly using the HiGHS solver.
///
/// # Example
///
/// ```rust,ignore
/// use problemreductions::models::algebraic::{ILP, LinearConstraint, ObjectiveSense};
/// use problemreductions::solvers::ILPSolver;
///
/// // Create a simple binary ILP: maximize x0 + 2*x1 subject to x0 + x1 <= 1
/// let ilp = ILP::<bool>::new(
///     2,
///     vec![LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0)],
///     vec![(0, 1.0), (1, 2.0)],
///     ObjectiveSense::Maximize,
/// );
///
/// let solver = ILPSolver::new();
/// let solution = solver.solve(&ilp)?;
/// println!("Solution: {:?}", solution);
/// # Ok::<(), problemreductions::solvers::ILPSolveError>(())
/// ```
#[derive(Debug, Clone, Default)]
pub struct ILPSolver {
    /// Time limit in seconds (None = no limit).
    pub time_limit: Option<f64>,
}

impl ILPSolver {
    /// Create a new ILP solver with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create an ILP solver with a time limit.
    pub fn with_time_limit(seconds: f64) -> Self {
        Self {
            time_limit: Some(seconds),
        }
    }

    /// Solve an ILP problem directly.
    ///
    /// Returns a classified error when the problem is infeasible, the time
    /// limit is reached, or the backend fails.
    /// The returned solution is a configuration vector where each element
    /// is the variable value (config index = value).
    pub fn solve<V: VariableDomain>(&self, problem: &ILP<V>) -> Result<Vec<usize>, ILPSolveError> {
        let n = problem.num_vars;
        if n == 0 {
            return problem
                .is_feasible(&[])
                .then_some(vec![])
                .ok_or(ILPSolveError::Infeasible);
        }

        // Derive tighter per-variable upper bounds from single-variable ≤ constraints.
        // This avoids giving HiGHS the full domain (e.g. 2^31 for i32), which can
        // cause severe performance degradation even when constraints already bound
        // the variable to a small range.
        let default_ub = (V::DIMS_PER_VAR - 1) as f64;
        let mut upper_bounds = vec![default_ub; n];
        for constraint in &problem.constraints {
            if constraint.cmp == crate::models::algebraic::Comparison::Le
                && constraint.terms.len() == 1
            {
                let (var_idx, coef) = constraint.terms[0];
                if coef > 0.0 && var_idx < n {
                    let ub = constraint.rhs / coef;
                    if ub < upper_bounds[var_idx] {
                        upper_bounds[var_idx] = ub;
                    }
                }
            }
        }

        // Create integer variables with tightened bounds
        let mut vars_builder = ProblemVariables::new();
        let vars: Vec<Variable> = (0..n)
            .map(|i| {
                let mut v = variable().integer();
                v = v.min(0.0);
                v = v.max(upper_bounds[i]);
                vars_builder.add(v)
            })
            .collect();

        // Build objective expression
        let objective: good_lp::Expression = problem
            .objective
            .iter()
            .map(|&(var_idx, coef)| coef * vars[var_idx])
            .sum();

        // Build the model with objective
        let unsolved = match problem.sense {
            ObjectiveSense::Maximize => vars_builder.maximise(&objective),
            ObjectiveSense::Minimize => vars_builder.minimise(&objective),
        };

        // Create the solver model
        #[cfg(feature = "ilp-highs")]
        let mut model = {
            let mut model = unsolved
                .using(highs)
                .set_option("random_seed", 0i32)
                .set_option("presolve", "off")
                .set_parallel(HighsParallelType::Off)
                .set_threads(1);
            if let Some(seconds) = self.time_limit {
                model = model.set_time_limit(seconds);
            }
            model
        };

        #[cfg(not(feature = "ilp-highs"))]
        let mut model = unsolved.using(default_solver);

        // Add constraints
        for constraint in &problem.constraints {
            // Build left-hand side expression
            let lhs: good_lp::Expression = constraint
                .terms
                .iter()
                .map(|&(var_idx, coef)| coef * vars[var_idx])
                .sum();

            // Create the constraint based on comparison type
            let good_lp_constraint = match constraint.cmp {
                Comparison::Le => lhs.leq(constraint.rhs),
                Comparison::Ge => lhs.geq(constraint.rhs),
                Comparison::Eq => lhs.eq(constraint.rhs),
            };

            model = model.with(good_lp_constraint);
        }

        // Solve
        #[cfg(feature = "ilp-highs")]
        let effective_time_limit = self.time_limit;
        #[cfg(not(feature = "ilp-highs"))]
        let effective_time_limit = None;
        let solution = model
            .solve()
            .map_err(|error| classify_backend_error(error, effective_time_limit))?;

        match solution.status() {
            SolutionStatus::Optimal => {}
            SolutionStatus::TimeLimit => return Err(ILPSolveError::Timeout),
            SolutionStatus::GapLimit => {
                return Err(ILPSolveError::BackendFailure(
                    "the backend stopped at its gap limit before proving optimality".to_string(),
                ));
            }
        }

        // Extract solution: config index = value (no lower bound offset)
        let result: Vec<usize> = vars
            .iter()
            .map(|v| {
                let val = solution.value(*v);
                val.round().max(0.0) as usize
            })
            .collect();

        Ok(result)
    }

    /// Solve any problem that reduces directly to `ILP<V>`.
    ///
    /// This method first reduces the problem to the selected ILP domain, solves the ILP,
    /// and then extracts the solution back to the original problem space.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use problemreductions::prelude::*;
    /// use problemreductions::solvers::ILPSolver;
    ///
    /// // Create a problem that reduces directly to ILP.
    /// let problem = MaximumSetPacking::<i32>::new(vec![
    ///     vec![0, 1],
    ///     vec![1, 2],
    ///     vec![3, 4],
    /// ]);
    ///
    /// // Solve using ILP solver
    /// let solver = ILPSolver::new();
    /// let solution = solver.solve_reduced::<bool, _>(&problem)?;
    /// println!("Solution: {:?}", solution);
    /// # Ok::<(), problemreductions::solvers::ILPSolveError>(())
    /// ```
    pub fn solve_reduced<V, P>(&self, problem: &P) -> Result<Vec<usize>, ILPSolveError>
    where
        V: VariableDomain,
        P: ReduceTo<ILP<V>>,
    {
        let reduction = problem.reduce_to();
        let ilp_solution = self.solve(reduction.target_problem())?;
        Ok(reduction.extract_solution(&ilp_solution)?)
    }

    /// Solve a type-erased supported ILP variant directly.
    pub(crate) fn solve_dyn(&self, any: &dyn std::any::Any) -> Result<Vec<usize>, ILPSolveError> {
        if let Some(ilp) = any.downcast_ref::<ILP<bool>>() {
            return self.solve(ilp);
        }
        if let Some(ilp) = any.downcast_ref::<ILP<i32>>() {
            return self.solve(ilp);
        }
        Err(ILPSolveError::UnsupportedProblemType)
    }
}

#[cfg(test)]
#[path = "../../unit_tests/solvers/ilp/solver.rs"]
mod tests;
