//! Reduction from SetPacking to ILP (Integer Linear Programming).
//!
//! The Set Packing problem can be formulated as a binary ILP:
//! - Variables: One binary variable per set (0 = not selected, 1 = selected)
//! - Constraints: x_i + x_j <= 1 for each overlapping pair (i, j)
//! - Objective: Maximize the sum of weights of selected sets

use crate::models::optimization::{ILP, LinearConstraint, ObjectiveSense, VarBounds};
use crate::models::set::SetPacking;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::traits::Problem;
use crate::types::ProblemSize;

/// Result of reducing SetPacking to ILP.
///
/// This reduction creates a binary ILP where:
/// - Each set corresponds to a binary variable
/// - Overlapping pair constraints ensure at most one of each pair is selected
/// - The objective maximizes the total weight of selected sets
#[derive(Debug, Clone)]
pub struct ReductionSPToILP {
    target: ILP,
    source_size: ProblemSize,
}

impl ReductionResult for ReductionSPToILP {
    type Source = SetPacking<i32>;
    type Target = ILP;

    fn target_problem(&self) -> &ILP {
        &self.target
    }

    /// Extract solution from ILP back to SetPacking.
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

impl ReduceTo<ILP> for SetPacking<i32> {
    type Result = ReductionSPToILP;

    fn reduce_to(&self) -> Self::Result {
        let num_vars = self.num_sets();

        // All variables are binary (0 or 1)
        let bounds = vec![VarBounds::binary(); num_vars];

        // Constraints: x_i + x_j <= 1 for each overlapping pair (i, j)
        // This ensures at most one set from each overlapping pair is selected
        let constraints: Vec<LinearConstraint> = self
            .overlapping_pairs()
            .into_iter()
            .map(|(i, j)| LinearConstraint::le(vec![(i, 1.0), (j, 1.0)], 1.0))
            .collect();

        // Objective: maximize sum of w_i * x_i (weighted sum of selected sets)
        let objective: Vec<(usize, f64)> = self
            .weights_ref()
            .iter()
            .enumerate()
            .map(|(i, &w)| (i, w as f64))
            .collect();

        let target = ILP::new(
            num_vars,
            bounds,
            constraints,
            objective,
            ObjectiveSense::Maximize,
        );

        ReductionSPToILP {
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
        // Three sets with two overlapping pairs
        let problem = SetPacking::<i32>::new(vec![vec![0, 1], vec![1, 2], vec![2, 3]]);
        let reduction: ReductionSPToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        // Check ILP structure
        assert_eq!(ilp.num_vars, 3, "Should have one variable per set");
        assert_eq!(
            ilp.constraints.len(),
            2,
            "Should have one constraint per overlapping pair"
        );
        assert_eq!(ilp.sense, ObjectiveSense::Maximize, "Should maximize");

        // All variables should be binary
        for bound in &ilp.bounds {
            assert_eq!(*bound, VarBounds::binary());
        }

        // Each constraint should be x_i + x_j <= 1
        for constraint in &ilp.constraints {
            assert_eq!(constraint.terms.len(), 2);
            assert!((constraint.rhs - 1.0).abs() < 1e-9);
        }
    }

    #[test]
    fn test_reduction_weighted() {
        let problem = SetPacking::with_weights(vec![vec![0, 1], vec![2, 3]], vec![5, 10]);
        let reduction: ReductionSPToILP = ReduceTo::<ILP>::reduce_to(&problem);
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
    fn test_ilp_solution_equals_brute_force_chain() {
        // Chain: {0,1}, {1,2}, {2,3} - can select at most 2 non-adjacent sets
        let problem = SetPacking::<i32>::new(vec![vec![0, 1], vec![1, 2], vec![2, 3]]);
        let reduction: ReductionSPToILP = ReduceTo::<ILP>::reduce_to(&problem);
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
    fn test_ilp_solution_equals_brute_force_all_overlap() {
        // All sets share element 0: can only select one
        let problem = SetPacking::<i32>::new(vec![vec![0, 1], vec![0, 2], vec![0, 3]]);
        let reduction: ReductionSPToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        let bf = BruteForce::new();
        let ilp_solver = ILPSolver::new();

        let bf_solutions = bf.find_best(&problem);
        let bf_size: usize = bf_solutions[0].iter().sum();

        let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
        let extracted = reduction.extract_solution(&ilp_solution);
        let ilp_size: usize = extracted.iter().sum();

        assert_eq!(bf_size, 1);
        assert_eq!(ilp_size, 1);

        let sol_result = problem.solution_size(&extracted);
        assert!(sol_result.is_valid);
    }

    #[test]
    fn test_ilp_solution_equals_brute_force_weighted() {
        // Weighted problem: single heavy set vs multiple light sets
        // Set 0 covers all elements but has weight 5
        // Sets 1 and 2 are disjoint and together have weight 6
        let problem = SetPacking::with_weights(
            vec![vec![0, 1, 2, 3], vec![0, 1], vec![2, 3]],
            vec![5, 3, 3],
        );
        let reduction: ReductionSPToILP = ReduceTo::<ILP>::reduce_to(&problem);
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

        // Should select sets 1 and 2
        assert_eq!(extracted, vec![0, 1, 1]);
    }

    #[test]
    fn test_solution_extraction() {
        let problem = SetPacking::<i32>::new(vec![vec![0, 1], vec![2, 3], vec![4, 5], vec![6, 7]]);
        let reduction: ReductionSPToILP = ReduceTo::<ILP>::reduce_to(&problem);

        // Test that extraction works correctly (1:1 mapping)
        let ilp_solution = vec![1, 0, 1, 0];
        let extracted = reduction.extract_solution(&ilp_solution);
        assert_eq!(extracted, vec![1, 0, 1, 0]);

        // Verify this is a valid packing (sets 0 and 2 are disjoint)
        let sol_result = problem.solution_size(&extracted);
        assert!(sol_result.is_valid);
    }

    #[test]
    fn test_source_and_target_size() {
        let problem = SetPacking::<i32>::new(vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]]);
        let reduction: ReductionSPToILP = ReduceTo::<ILP>::reduce_to(&problem);

        let source_size = reduction.source_size();
        let target_size = reduction.target_size();

        assert_eq!(source_size.get("num_sets"), Some(4));

        assert_eq!(target_size.get("num_vars"), Some(4));
        // 3 overlapping pairs: (0,1), (1,2), (2,3)
        assert_eq!(target_size.get("num_constraints"), Some(3));
    }

    #[test]
    fn test_disjoint_sets() {
        // All sets are disjoint: no overlapping pairs
        let problem = SetPacking::<i32>::new(vec![vec![0], vec![1], vec![2], vec![3]]);
        let reduction: ReductionSPToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        assert_eq!(ilp.constraints.len(), 0);

        let ilp_solver = ILPSolver::new();
        let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
        let extracted = reduction.extract_solution(&ilp_solution);

        // All sets should be selected
        assert_eq!(extracted, vec![1, 1, 1, 1]);

        let sol_result = problem.solution_size(&extracted);
        assert!(sol_result.is_valid);
        assert_eq!(sol_result.size, 4);
    }

    #[test]
    fn test_empty_sets() {
        let problem = SetPacking::<i32>::new(vec![]);
        let reduction: ReductionSPToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        assert_eq!(ilp.num_vars, 0);
        assert_eq!(ilp.constraints.len(), 0);
    }

    #[test]
    fn test_solve_reduced() {
        // Test the ILPSolver::solve_reduced method
        let problem = SetPacking::<i32>::new(vec![vec![0, 1], vec![1, 2], vec![2, 3]]);

        let ilp_solver = ILPSolver::new();
        let solution = ilp_solver
            .solve_reduced(&problem)
            .expect("solve_reduced should work");

        let sol_result = problem.solution_size(&solution);
        assert!(sol_result.is_valid);
        assert_eq!(sol_result.size, 2);
    }

    #[test]
    fn test_all_sets_overlap_pairwise() {
        // All pairs overlap: can only select one set
        // Sets: {0,1}, {0,2}, {1,2} - each pair shares one element
        let problem = SetPacking::<i32>::new(vec![vec![0, 1], vec![0, 2], vec![1, 2]]);
        let reduction: ReductionSPToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        // 3 overlapping pairs
        assert_eq!(ilp.constraints.len(), 3);

        let ilp_solver = ILPSolver::new();
        let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
        let extracted = reduction.extract_solution(&ilp_solution);

        let sol_result = problem.solution_size(&extracted);
        assert!(sol_result.is_valid);
        assert_eq!(sol_result.size, 1);
    }
}
