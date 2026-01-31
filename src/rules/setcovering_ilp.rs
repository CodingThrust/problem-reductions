//! Reduction from SetCovering to ILP (Integer Linear Programming).
//!
//! The Set Covering problem can be formulated as a binary ILP:
//! - Variables: One binary variable per set (0 = not selected, 1 = selected)
//! - Constraints: For each element e: sum_{j: e in set_j} x_j >= 1 (element must be covered)
//! - Objective: Minimize the sum of weights of selected sets

use crate::models::optimization::{LinearConstraint, ObjectiveSense, VarBounds, ILP};
use crate::models::set::SetCovering;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::traits::{ConstraintSatisfactionProblem, Problem};
use crate::types::ProblemSize;

/// Result of reducing SetCovering to ILP.
///
/// This reduction creates a binary ILP where:
/// - Each set corresponds to a binary variable
/// - Element coverage constraints ensure each element is covered by at least one selected set
/// - The objective minimizes the total weight of selected sets
#[derive(Debug, Clone)]
pub struct ReductionSCToILP {
    target: ILP,
    source_size: ProblemSize,
}

impl ReductionResult for ReductionSCToILP {
    type Source = SetCovering<i32>;
    type Target = ILP;

    fn target_problem(&self) -> &ILP {
        &self.target
    }

    /// Extract solution from ILP back to SetCovering.
    ///
    /// Since the mapping is 1:1 (each set maps to one binary variable),
    /// the solution extraction is simply copying the configuration.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }

    fn source_size(&self) -> ProblemSize {
        self.source_size.clone()
    }

    fn target_size(&self) -> ProblemSize {
        self.target.problem_size()
    }
}

impl ReduceTo<ILP> for SetCovering<i32> {
    type Result = ReductionSCToILP;

    fn reduce_to(&self) -> Self::Result {
        let num_vars = self.num_sets();

        // All variables are binary (0 or 1)
        let bounds = vec![VarBounds::binary(); num_vars];

        // Constraints: For each element e, sum_{j: e in set_j} x_j >= 1
        // This ensures each element is covered by at least one selected set
        let constraints: Vec<LinearConstraint> = (0..self.universe_size())
            .map(|element| {
                // Find all sets containing this element
                let terms: Vec<(usize, f64)> = self
                    .sets()
                    .iter()
                    .enumerate()
                    .filter(|(_, set)| set.contains(&element))
                    .map(|(j, _)| (j, 1.0))
                    .collect();

                LinearConstraint::ge(terms, 1.0)
            })
            .collect();

        // Objective: minimize sum of w_i * x_i (weighted sum of selected sets)
        let objective: Vec<(usize, f64)> = self
            .weights()
            .iter()
            .enumerate()
            .map(|(i, &w)| (i, w as f64))
            .collect();

        let target = ILP::new(
            num_vars,
            bounds,
            constraints,
            objective,
            ObjectiveSense::Minimize,
        );

        ReductionSCToILP {
            target,
            source_size: self.problem_size(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::{BruteForce, ILPSolver, Solver};

    #[test]
    fn test_reduction_creates_valid_ilp() {
        // Universe: {0, 1, 2}, Sets: S0={0,1}, S1={1,2}
        let problem = SetCovering::<i32>::new(3, vec![vec![0, 1], vec![1, 2]]);
        let reduction: ReductionSCToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        // Check ILP structure
        assert_eq!(ilp.num_vars, 2, "Should have one variable per set");
        assert_eq!(
            ilp.constraints.len(),
            3,
            "Should have one constraint per element"
        );
        assert_eq!(ilp.sense, ObjectiveSense::Minimize, "Should minimize");

        // All variables should be binary
        for bound in &ilp.bounds {
            assert_eq!(*bound, VarBounds::binary());
        }

        // Each constraint should be sum >= 1
        for constraint in &ilp.constraints {
            assert!((constraint.rhs - 1.0).abs() < 1e-9);
        }
    }

    #[test]
    fn test_reduction_weighted() {
        let problem = SetCovering::with_weights(3, vec![vec![0, 1], vec![1, 2]], vec![5, 10]);
        let reduction: ReductionSCToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        // Check that weights are correctly transferred to objective
        let mut coeffs: Vec<f64> = vec![0.0; 2];
        for &(var, coef) in &ilp.objective {
            coeffs[var] = coef;
        }
        assert!((coeffs[0] - 5.0).abs() < 1e-9);
        assert!((coeffs[1] - 10.0).abs() < 1e-9);
    }

    #[test]
    fn test_ilp_solution_equals_brute_force_simple() {
        // Universe: {0, 1, 2}, Sets: S0={0,1}, S1={1,2}, S2={0,2}
        // Minimum cover: any 2 sets work
        let problem = SetCovering::<i32>::new(3, vec![vec![0, 1], vec![1, 2], vec![0, 2]]);
        let reduction: ReductionSCToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        let bf = BruteForce::new();
        let ilp_solver = ILPSolver::new();

        // Solve with brute force on original problem
        let bf_solutions = bf.find_best(&problem);

        // Solve via ILP reduction
        let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
        let extracted = reduction.extract_solution(&ilp_solution);

        // Both should find optimal size = 2
        let bf_size: usize = bf_solutions[0].iter().sum();
        let ilp_size: usize = extracted.iter().sum();
        assert_eq!(bf_size, 2);
        assert_eq!(ilp_size, 2);

        // Verify the ILP solution is valid for the original problem
        let sol_result = problem.solution_size(&extracted);
        assert!(sol_result.is_valid, "Extracted solution should be valid");
    }

    #[test]
    fn test_ilp_solution_equals_brute_force_weighted() {
        // Weighted problem: prefer lighter sets
        // Universe: {0,1,2}, Sets: S0={0,1,2}, S1={0,1}, S2={2}
        // Weights: [10, 3, 3]
        // Optimal: select S1 and S2 (weight 6) instead of S0 (weight 10)
        let problem =
            SetCovering::with_weights(3, vec![vec![0, 1, 2], vec![0, 1], vec![2]], vec![10, 3, 3]);
        let reduction: ReductionSCToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        let bf = BruteForce::new();
        let ilp_solver = ILPSolver::new();

        let bf_solutions = bf.find_best(&problem);
        let bf_obj = problem.solution_size(&bf_solutions[0]).size;

        let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
        let extracted = reduction.extract_solution(&ilp_solution);
        let ilp_obj = problem.solution_size(&extracted).size;

        assert_eq!(bf_obj, 6);
        assert_eq!(ilp_obj, 6);

        // Verify the solution selects S1 and S2
        assert_eq!(extracted, vec![0, 1, 1]);
    }

    #[test]
    fn test_solution_extraction() {
        let problem = SetCovering::<i32>::new(4, vec![vec![0, 1], vec![2, 3]]);
        let reduction: ReductionSCToILP = ReduceTo::<ILP>::reduce_to(&problem);

        // Test that extraction works correctly (1:1 mapping)
        let ilp_solution = vec![1, 1];
        let extracted = reduction.extract_solution(&ilp_solution);
        assert_eq!(extracted, vec![1, 1]);

        // Verify this is a valid set cover
        let sol_result = problem.solution_size(&extracted);
        assert!(sol_result.is_valid);
    }

    #[test]
    fn test_source_and_target_size() {
        let problem =
            SetCovering::<i32>::new(5, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]]);
        let reduction: ReductionSCToILP = ReduceTo::<ILP>::reduce_to(&problem);

        let source_size = reduction.source_size();
        let target_size = reduction.target_size();

        assert_eq!(source_size.get("universe_size"), Some(5));
        assert_eq!(source_size.get("num_sets"), Some(4));

        assert_eq!(target_size.get("num_vars"), Some(4));
        assert_eq!(target_size.get("num_constraints"), Some(5));
    }

    #[test]
    fn test_single_set_covers_all() {
        // Single set covers entire universe
        let problem = SetCovering::<i32>::new(3, vec![vec![0, 1, 2], vec![0], vec![1], vec![2]]);

        let ilp_solver = ILPSolver::new();
        let reduction: ReductionSCToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
        let extracted = reduction.extract_solution(&ilp_solution);

        // First set alone covers everything with weight 1
        assert_eq!(extracted, vec![1, 0, 0, 0]);

        let sol_result = problem.solution_size(&extracted);
        assert!(sol_result.is_valid);
        assert_eq!(sol_result.size, 1);
    }

    #[test]
    fn test_overlapping_sets() {
        // All sets overlap on element 1
        let problem = SetCovering::<i32>::new(3, vec![vec![0, 1], vec![1, 2]]);

        let ilp_solver = ILPSolver::new();
        let reduction: ReductionSCToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
        let extracted = reduction.extract_solution(&ilp_solution);

        // Need both sets to cover all elements
        assert_eq!(extracted, vec![1, 1]);

        let sol_result = problem.solution_size(&extracted);
        assert!(sol_result.is_valid);
        assert_eq!(sol_result.size, 2);
    }

    #[test]
    fn test_empty_universe() {
        // Empty universe is trivially covered
        let problem = SetCovering::<i32>::new(0, vec![]);
        let reduction: ReductionSCToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        assert_eq!(ilp.num_vars, 0);
        assert_eq!(ilp.constraints.len(), 0);
    }

    #[test]
    fn test_solve_reduced() {
        // Test the ILPSolver::solve_reduced method
        let problem =
            SetCovering::<i32>::new(4, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![0, 3]]);

        let ilp_solver = ILPSolver::new();
        let solution = ilp_solver
            .solve_reduced(&problem)
            .expect("solve_reduced should work");

        let sol_result = problem.solution_size(&solution);
        assert!(sol_result.is_valid);
        assert_eq!(sol_result.size, 2);
    }

    #[test]
    fn test_constraint_structure() {
        // Universe: {0, 1, 2}
        // Sets: S0={0}, S1={0,1}, S2={1,2}
        // Element 0 is in S0, S1 -> constraint: x0 + x1 >= 1
        // Element 1 is in S1, S2 -> constraint: x1 + x2 >= 1
        // Element 2 is in S2 -> constraint: x2 >= 1
        let problem = SetCovering::<i32>::new(3, vec![vec![0], vec![0, 1], vec![1, 2]]);
        let reduction: ReductionSCToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        assert_eq!(ilp.constraints.len(), 3);

        // Check constraint for element 0: should involve sets 0 and 1
        let c0 = &ilp.constraints[0];
        let vars0: Vec<usize> = c0.terms.iter().map(|&(v, _)| v).collect();
        assert!(vars0.contains(&0));
        assert!(vars0.contains(&1));
        assert!(!vars0.contains(&2));

        // Check constraint for element 1: should involve sets 1 and 2
        let c1 = &ilp.constraints[1];
        let vars1: Vec<usize> = c1.terms.iter().map(|&(v, _)| v).collect();
        assert!(!vars1.contains(&0));
        assert!(vars1.contains(&1));
        assert!(vars1.contains(&2));

        // Check constraint for element 2: should involve only set 2
        let c2 = &ilp.constraints[2];
        let vars2: Vec<usize> = c2.terms.iter().map(|&(v, _)| v).collect();
        assert!(!vars2.contains(&0));
        assert!(!vars2.contains(&1));
        assert!(vars2.contains(&2));
    }
}
