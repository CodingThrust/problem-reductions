//! ILP solver implementation using HiGHS.

use crate::models::algebraic::{Comparison, ObjectiveSense, VariableDomain, ILP};
use crate::rules::{ReduceTo, ReductionResult};
#[cfg(not(feature = "ilp-highs"))]
use good_lp::default_solver;
#[cfg(feature = "ilp-highs")]
use good_lp::highs;
#[cfg(feature = "ilp-highs")]
use good_lp::solvers::highs::HighsParallelType;
use good_lp::{variable, ProblemVariables, Solution, SolverModel, Variable};

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
/// if let Some(solution) = solver.solve(&ilp) {
///     println!("Solution: {:?}", solution);
/// }
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
    /// Returns `None` if the problem is infeasible or the solver fails.
    /// The returned solution is a configuration vector where each element
    /// is the variable value (config index = value).
    pub fn solve<V: VariableDomain>(&self, problem: &ILP<V>) -> Option<Vec<usize>> {
        let n = problem.num_vars;
        if n == 0 {
            return Some(vec![]);
        }

        // Create integer variables with bounds from variable domain
        let mut vars_builder = ProblemVariables::new();
        let vars: Vec<Variable> = (0..n)
            .map(|_| {
                let mut v = variable().integer();
                v = v.min(0.0);
                v = v.max((V::DIMS_PER_VAR - 1) as f64);
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
        let solution = model.solve().ok()?;

        // Extract solution: config index = value (no lower bound offset)
        let result: Vec<usize> = vars
            .iter()
            .map(|v| {
                let val = solution.value(*v);
                val.round().max(0.0) as usize
            })
            .collect();

        Some(result)
    }

    /// Solve any problem that reduces to `ILP<bool>`.
    ///
    /// This method first reduces the problem to a binary ILP, solves the ILP,
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
    /// if let Some(solution) = solver.solve_reduced(&problem) {
    ///     println!("Solution: {:?}", solution);
    /// }
    /// ```
    pub fn solve_reduced<P>(&self, problem: &P) -> Option<Vec<usize>>
    where
        P: ReduceTo<ILP<bool>>,
    {
        let reduction = problem.reduce_to();
        let ilp_solution = self.solve(reduction.target_problem())?;
        Some(reduction.extract_solution(&ilp_solution))
    }
}

#[cfg(test)]
#[path = "../../unit_tests/solvers/ilp/solver.rs"]
mod tests;
