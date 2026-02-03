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
mod tests {
    use super::*;
    use crate::models::optimization::{LinearConstraint, VarBounds};
    use crate::solvers::{BruteForce, Solver};
    use crate::traits::Problem;

    #[test]
    fn test_ilp_solver_basic_maximize() {
        // Maximize x0 + 2*x1 subject to x0 + x1 <= 1, binary vars
        let ilp = ILP::binary(
            2,
            vec![LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0)],
            vec![(0, 1.0), (1, 2.0)],
            ObjectiveSense::Maximize,
        );

        let solver = ILPSolver::new();
        let solution = solver.solve(&ilp);

        assert!(solution.is_some());
        let sol = solution.unwrap();

        // Solution should be valid
        let result = ilp.solution_size(&sol);
        assert!(result.is_valid, "ILP solution should be valid");

        // Optimal: x1=1, x0=0 => objective = 2
        assert!((result.size - 2.0).abs() < 1e-9);
    }

    #[test]
    fn test_ilp_solver_basic_minimize() {
        // Minimize x0 + x1 subject to x0 + x1 >= 1, binary vars
        let ilp = ILP::binary(
            2,
            vec![LinearConstraint::ge(vec![(0, 1.0), (1, 1.0)], 1.0)],
            vec![(0, 1.0), (1, 1.0)],
            ObjectiveSense::Minimize,
        );

        let solver = ILPSolver::new();
        let solution = solver.solve(&ilp);

        assert!(solution.is_some());
        let sol = solution.unwrap();

        // Solution should be valid
        let result = ilp.solution_size(&sol);
        assert!(result.is_valid, "ILP solution should be valid");

        // Optimal: one variable = 1, other = 0 => objective = 1
        assert!((result.size - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_ilp_solver_matches_brute_force() {
        // Maximize x0 + x1 + x2 subject to:
        //   x0 + x1 <= 1
        //   x1 + x2 <= 1
        let ilp = ILP::binary(
            3,
            vec![
                LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0),
                LinearConstraint::le(vec![(1, 1.0), (2, 1.0)], 1.0),
            ],
            vec![(0, 1.0), (1, 1.0), (2, 1.0)],
            ObjectiveSense::Maximize,
        );

        let bf = BruteForce::new();
        let ilp_solver = ILPSolver::new();

        let bf_solutions = bf.find_best(&ilp);
        let ilp_solution = ilp_solver.solve(&ilp).unwrap();

        // Both should find optimal value (2)
        let bf_size = ilp.solution_size(&bf_solutions[0]).size;
        let ilp_size = ilp.solution_size(&ilp_solution).size;
        assert!(
            (bf_size - ilp_size).abs() < 1e-9,
            "ILP should find optimal solution"
        );
    }

    #[test]
    fn test_ilp_empty_problem() {
        let ilp = ILP::empty();
        let solver = ILPSolver::new();
        let solution = solver.solve(&ilp);
        assert_eq!(solution, Some(vec![]));
    }

    #[test]
    fn test_ilp_equality_constraint() {
        // Minimize x0 subject to x0 + x1 == 1, binary vars
        let ilp = ILP::binary(
            2,
            vec![LinearConstraint::eq(vec![(0, 1.0), (1, 1.0)], 1.0)],
            vec![(0, 1.0)],
            ObjectiveSense::Minimize,
        );

        let solver = ILPSolver::new();
        let solution = solver.solve(&ilp).unwrap();

        let result = ilp.solution_size(&solution);
        assert!(result.is_valid);
        // Optimal: x0=0, x1=1 => objective = 0
        assert!((result.size - 0.0).abs() < 1e-9);
    }

    #[test]
    fn test_ilp_non_binary_bounds() {
        // Variables with larger ranges
        // x0 in [0, 3], x1 in [0, 2]
        // Maximize x0 + x1 subject to x0 + x1 <= 4
        let ilp = ILP::new(
            2,
            vec![VarBounds::bounded(0, 3), VarBounds::bounded(0, 2)],
            vec![LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 4.0)],
            vec![(0, 1.0), (1, 1.0)],
            ObjectiveSense::Maximize,
        );

        let solver = ILPSolver::new();
        let solution = solver.solve(&ilp).unwrap();

        let result = ilp.solution_size(&solution);
        assert!(result.is_valid);
        // Optimal: x0=3, x1=2 => objective = 5 (3 + 2 = 5 <= 4 is false!)
        // Wait, 3+2=5 > 4, so constraint is violated. Let's check actual optimal:
        // x0=2, x1=2 => 4 <= 4 valid, obj=4
        // x0=3, x1=1 => 4 <= 4 valid, obj=4
        assert!((result.size - 4.0).abs() < 1e-9);
    }

    #[test]
    fn test_ilp_negative_lower_bounds() {
        // Variables with negative lower bounds
        // x0 in [-2, 2], x1 in [-1, 1]
        // Maximize x0 + x1 (no constraints)
        let ilp = ILP::new(
            2,
            vec![VarBounds::bounded(-2, 2), VarBounds::bounded(-1, 1)],
            vec![],
            vec![(0, 1.0), (1, 1.0)],
            ObjectiveSense::Maximize,
        );

        let solver = ILPSolver::new();
        let solution = solver.solve(&ilp).unwrap();

        let result = ilp.solution_size(&solution);
        assert!(result.is_valid);
        // Optimal: x0=2, x1=1 => objective = 3
        assert!((result.size - 3.0).abs() < 1e-9);
    }

    #[test]
    fn test_ilp_config_to_values_roundtrip() {
        // Ensure the config encoding/decoding works correctly
        let ilp = ILP::new(
            2,
            vec![VarBounds::bounded(-2, 2), VarBounds::bounded(1, 3)],
            vec![],
            vec![(0, 1.0), (1, 1.0)],
            ObjectiveSense::Maximize,
        );

        let solver = ILPSolver::new();
        let solution = solver.solve(&ilp).unwrap();

        // The solution should be valid
        let result = ilp.solution_size(&solution);
        assert!(result.is_valid);
        // Optimal: x0=2, x1=3 => objective = 5
        assert!((result.size - 5.0).abs() < 1e-9);
    }

    #[test]
    fn test_ilp_multiple_constraints() {
        // Maximize 2*x0 + 3*x1 + x2 subject to:
        //   x0 + x1 + x2 <= 2
        //   x0 + x1 >= 1
        // Binary vars
        let ilp = ILP::binary(
            3,
            vec![
                LinearConstraint::le(vec![(0, 1.0), (1, 1.0), (2, 1.0)], 2.0),
                LinearConstraint::ge(vec![(0, 1.0), (1, 1.0)], 1.0),
            ],
            vec![(0, 2.0), (1, 3.0), (2, 1.0)],
            ObjectiveSense::Maximize,
        );

        let solver = ILPSolver::new();
        let solution = solver.solve(&ilp).unwrap();

        let result = ilp.solution_size(&solution);
        assert!(result.is_valid);

        // Check against brute force
        let bf = BruteForce::new();
        let bf_solutions = bf.find_best(&ilp);
        let bf_size = ilp.solution_size(&bf_solutions[0]).size;

        assert!(
            (bf_size - result.size).abs() < 1e-9,
            "ILP should match brute force"
        );
    }

    #[test]
    fn test_ilp_unconstrained() {
        // Maximize x0 + x1, no constraints, binary vars
        let ilp = ILP::binary(
            2,
            vec![],
            vec![(0, 1.0), (1, 1.0)],
            ObjectiveSense::Maximize,
        );

        let solver = ILPSolver::new();
        let solution = solver.solve(&ilp).unwrap();

        let result = ilp.solution_size(&solution);
        assert!(result.is_valid);
        // Optimal: both = 1
        assert!((result.size - 2.0).abs() < 1e-9);
    }

    #[test]
    fn test_ilp_with_time_limit() {
        let solver = ILPSolver::with_time_limit(10.0);
        assert_eq!(solver.time_limit, Some(10.0));

        // Should still work for simple problems
        let ilp = ILP::binary(
            2,
            vec![LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0)],
            vec![(0, 1.0), (1, 1.0)],
            ObjectiveSense::Maximize,
        );

        let solution = solver.solve(&ilp);
        assert!(solution.is_some());
    }
}
