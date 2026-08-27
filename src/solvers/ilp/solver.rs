//! ILP solver implementation using HiGHS.

use crate::models::algebraic::{Comparison, ObjectiveSense, VariableDomain, ILP};
use crate::rules::{ReduceTo, ReductionResult};
use crate::types::{i64_to_exact_f64, MAX_EXACT_F64_INTEGER};
use good_lp::highs;
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
    #[error("the ILP backend supports only ILP<bool> and ILP<i64>")]
    UnsupportedProblemType,
    /// HiGHS reported an optimal solution that is invalid after integer rounding.
    #[error("the ILP backend returned an invalid rounded solution: {0}")]
    InvalidSolution(String),
    /// An exact integer in the model cannot be transported through the f64 backend API.
    #[error("the ILP backend cannot represent an exact model integer: {0}")]
    InexactTransport(#[from] crate::types::ExactI64ToF64Error),
    /// A target witness could not be mapped back to the source problem.
    #[error(transparent)]
    Extraction(#[from] crate::rules::ExtractionError),
    /// A registered reduction could not construct its target instance.
    #[error(transparent)]
    Reduction(#[from] crate::rules::ReductionError),
}

fn classify_backend_error(
    error: ResolutionError,
    time_limit: Option<f64>,
    has_unbounded_variable: bool,
) -> ILPSolveError {
    match error {
        ResolutionError::Infeasible if has_unbounded_variable => ILPSolveError::BackendFailure(
            "good_lp cannot distinguish an infeasible backend result from an infeasible-or-unbounded result for a model with unbounded variable domains".into(),
        ),
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
/// ```rust
/// use problemreductions::models::algebraic::{ILP, LinearConstraint, ObjectiveSense};
/// use problemreductions::solvers::ILPSolver;
///
/// // Create a simple binary ILP: maximize x0 + 2*x1 subject to x0 + x1 <= 1
/// let ilp = ILP::<bool>::new(
///     2,
///     vec![LinearConstraint::le(vec![(0, 1), (1, 1)], 1)],
///     vec![(0, 1.0), (1, 2.0)],
///     ObjectiveSense::Maximize,
/// )?;
///
/// let solver = ILPSolver::new();
/// let solution = solver.solve(&ilp)?;
/// println!("Solution: {:?}", solution);
/// # Ok::<(), Box<dyn std::error::Error>>(())
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
    /// The returned solution contains the mathematical integer value of each
    /// variable in model order.
    pub fn solve<V: VariableDomain>(&self, problem: &ILP<V>) -> Result<Vec<i64>, ILPSolveError> {
        let n = problem.num_vars();
        if n == 0 {
            return if problem
                .is_feasible(&[])
                .map_err(|error| ILPSolveError::InvalidSolution(error.to_string()))?
            {
                Ok(vec![])
            } else {
                Err(ILPSolveError::Infeasible)
            };
        }

        let mut vars_builder = ProblemVariables::new();
        let vars: Vec<Variable> = problem
            .variables()
            .iter()
            .map(|variable_bounds| {
                let mut definition = variable().integer();
                if let Some(lower) = variable_bounds.lower_bound() {
                    definition = definition.min(i64_to_exact_f64(lower)?);
                }
                if let Some(upper) = variable_bounds.upper_bound() {
                    definition = definition.max(i64_to_exact_f64(upper)?);
                }
                Ok(vars_builder.add(definition))
            })
            .collect::<Result<_, ILPSolveError>>()?;

        // Build objective expression
        let objective: good_lp::Expression = problem
            .objective()
            .iter()
            .map(|&(var_idx, coef)| coef * vars[var_idx])
            .sum();

        // Build the model with objective
        let unsolved = match problem.sense() {
            ObjectiveSense::Maximize => vars_builder.maximise(&objective),
            ObjectiveSense::Minimize => vars_builder.minimise(&objective),
        };

        // Create the solver model
        let mut model = {
            let mut model = unsolved
                .using(highs)
                .set_option("random_seed", 0i32)
                .set_parallel(HighsParallelType::Off)
                .set_threads(1);
            if let Some(seconds) = self.time_limit {
                model = model.set_time_limit(seconds);
            }
            model
        };

        // Add constraints
        for constraint in problem.constraints() {
            // Build left-hand side expression
            let lhs: good_lp::Expression = constraint.terms().iter().try_fold(
                good_lp::Expression::from(0.0),
                |lhs, &(var_idx, coefficient)| {
                    Ok::<_, ILPSolveError>(lhs + i64_to_exact_f64(coefficient)? * vars[var_idx])
                },
            )?;

            let rhs = i64_to_exact_f64(constraint.rhs())?;

            // Create the constraint based on comparison type
            let good_lp_constraint = match constraint.comparison() {
                Comparison::Le => lhs.leq(rhs),
                Comparison::Ge => lhs.geq(rhs),
                Comparison::Eq => lhs.eq(rhs),
            };

            model = model.with(good_lp_constraint);
        }

        // Solve
        let effective_time_limit = self.time_limit;
        let has_unbounded_variable = problem
            .variables()
            .iter()
            .any(|variable| variable.lower_bound().is_none() || variable.upper_bound().is_none());
        let solution = model.solve().map_err(|error| {
            classify_backend_error(error, effective_time_limit, has_unbounded_variable)
        })?;

        match solution.status() {
            SolutionStatus::Optimal => {}
            SolutionStatus::TimeLimit => return Err(ILPSolveError::Timeout),
            SolutionStatus::GapLimit => {
                return Err(ILPSolveError::BackendFailure(
                    "the backend stopped at its gap limit before proving optimality".to_string(),
                ));
            }
        }

        let result: Vec<i64> = vars
            .iter()
            .enumerate()
            .map(|(index, v)| {
                let value = solution.value(*v);
                if !value.is_finite() {
                    return Err(ILPSolveError::InvalidSolution(format!(
                        "variable {index} is non-finite"
                    )));
                }
                let rounded = value.round();
                if (value - rounded).abs() > 1e-6 {
                    return Err(ILPSolveError::InvalidSolution(format!(
                        "variable {index} has non-integral value {value}"
                    )));
                }
                if rounded.abs() > MAX_EXACT_F64_INTEGER as f64 {
                    return Err(ILPSolveError::InvalidSolution(format!(
                        "variable {index} value {rounded} exceeds exact f64 integer transport"
                    )));
                }
                Ok(rounded as i64)
            })
            .collect::<Result<_, _>>()?;

        if !problem
            .is_feasible(&result)
            .map_err(|error| ILPSolveError::InvalidSolution(error.to_string()))?
        {
            return Err(ILPSolveError::InvalidSolution(
                "the rounded assignment violates the ILP".into(),
            ));
        }

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
    /// let problem = MaximumSetPacking::<i64>::new(vec![
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
    pub fn solve_reduced<V, P>(
        &self,
        problem: &P,
    ) -> Result<<P as crate::traits::Problem>::Solution, ILPSolveError>
    where
        V: VariableDomain,
        P: ReduceTo<ILP<V>>,
    {
        let reduction = problem.reduce_to()?;
        let ilp_solution = self.solve(reduction.target_problem())?;
        Ok(reduction.extract_solution(&ilp_solution)?)
    }

    /// Solve a type-erased supported ILP variant directly.
    pub(crate) fn solve_dyn(&self, any: &dyn std::any::Any) -> Result<Vec<i64>, ILPSolveError> {
        if let Some(ilp) = any.downcast_ref::<ILP<bool>>() {
            return self.solve(ilp);
        }
        if let Some(ilp) = any.downcast_ref::<ILP<i64>>() {
            return self.solve(ilp);
        }
        Err(ILPSolveError::UnsupportedProblemType)
    }
}

#[cfg(test)]
#[path = "../../unit_tests/solvers/ilp/solver.rs"]
mod tests;
