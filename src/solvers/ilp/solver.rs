//! ILP solver implementation using HiGHS.

use crate::models::optimization::{Comparison, ObjectiveSense, ILP};
use crate::rules::{ReduceTo, ReductionResult};
use good_lp::{default_solver, variable, ProblemVariables, Solution, SolverModel, Variable};

/// An ILP solver using the HiGHS backend.
///
/// This solver solves Integer Linear Programming problems directly using the HiGHS solver.
///
/// # Example
///
/// ```rust,ignore
/// use problemreductions::models::optimization::{ILP, VarBounds, LinearConstraint, ObjectiveSense};
/// use problemreductions::solvers::ILPSolver;
///
/// // Create a simple ILP: maximize x0 + 2*x1 subject to x0 + x1 <= 1
/// let ilp = ILP::binary(
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
    /// represents the offset from the lower bound for that variable.
    pub fn solve(&self, problem: &ILP) -> Option<Vec<usize>> {
        let n = problem.num_vars;
        if n == 0 {
            return Some(vec![]);
        }

        // Create integer variables with bounds
        let mut vars_builder = ProblemVariables::new();
        let vars: Vec<Variable> = problem
            .bounds
            .iter()
            .map(|bounds| {
                let mut v = variable().integer();

                // Apply lower bound
                if let Some(lo) = bounds.lower {
                    v = v.min(lo as f64);
                }

                // Apply upper bound
                if let Some(hi) = bounds.upper {
                    v = v.max(hi as f64);
                }

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

        // Extract solution values and convert to configuration
        // Configuration is offset from lower bound: config[i] = value[i] - lower_bound[i]
        let result: Vec<usize> = vars
            .iter()
            .enumerate()
            .map(|(i, v)| {
                let val = solution.value(*v);
                // Round to nearest integer and compute offset from lower bound
                let int_val = val.round() as i64;
                let lower_bound = problem.bounds[i].lower.unwrap_or(0);
                let offset = int_val - lower_bound;
                offset.max(0) as usize
            })
            .collect();

        Some(result)
    }

    /// Solve any problem that reduces to ILP.
    ///
    /// This method first reduces the problem to an ILP, solves the ILP,
    /// and then extracts the solution back to the original problem space.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use problemreductions::prelude::*;
    /// use problemreductions::solvers::ILPSolver;
    /// use problemreductions::topology::SimpleGraph;
    ///
    /// // Create a problem that reduces to ILP (e.g., Independent Set)
    /// let problem = IndependentSet::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
    ///
    /// // Solve using ILP solver
    /// let solver = ILPSolver::new();
    /// if let Some(solution) = solver.solve_reduced(&problem) {
    ///     println!("Solution: {:?}", solution);
    /// }
    /// ```
    pub fn solve_reduced<P>(&self, problem: &P) -> Option<Vec<usize>>
    where
        P: ReduceTo<ILP>,
    {
        let reduction = problem.reduce_to();
        let ilp_solution = self.solve(reduction.target_problem())?;
        Some(reduction.extract_solution(&ilp_solution))
    }
}

#[cfg(test)]
#[path = "../../unit_tests/solvers/ilp/solver.rs"]
mod tests;
