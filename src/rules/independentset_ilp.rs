//! Reduction from IndependentSet to ILP (Integer Linear Programming).
//!
//! The Independent Set problem can be formulated as a binary ILP:
//! - Variables: One binary variable per vertex (0 = not selected, 1 = selected)
//! - Constraints: x_u + x_v <= 1 for each edge (u, v) - at most one endpoint can be selected
//! - Objective: Maximize the sum of weights of selected vertices

use crate::models::graph::IndependentSet;
use crate::models::optimization::{ILP, LinearConstraint, ObjectiveSense, VarBounds};
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::traits::Problem;
use crate::types::ProblemSize;

/// Result of reducing IndependentSet to ILP.
///
/// This reduction creates a binary ILP where:
/// - Each vertex corresponds to a binary variable
/// - Edge constraints ensure at most one endpoint is selected
/// - The objective maximizes the total weight of selected vertices
#[derive(Debug, Clone)]
pub struct ReductionISToILP {
    target: ILP,
    source_size: ProblemSize,
}

impl ReductionResult for ReductionISToILP {
    type Source = IndependentSet<i32>;
    type Target = ILP;

    fn target_problem(&self) -> &ILP {
        &self.target
    }

    /// Extract solution from ILP back to IndependentSet.
    ///
    /// Since the mapping is 1:1 (each vertex maps to one binary variable),
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

impl ReduceTo<ILP> for IndependentSet<i32> {
    type Result = ReductionISToILP;

    fn reduce_to(&self) -> Self::Result {
        let num_vars = self.num_vertices();

        // All variables are binary (0 or 1)
        let bounds = vec![VarBounds::binary(); num_vars];

        // Constraints: x_u + x_v <= 1 for each edge (u, v)
        // This ensures at most one endpoint of each edge is selected
        let constraints: Vec<LinearConstraint> = self
            .edges()
            .into_iter()
            .map(|(u, v)| LinearConstraint::le(vec![(u, 1.0), (v, 1.0)], 1.0))
            .collect();

        // Objective: maximize sum of w_i * x_i (weighted sum of selected vertices)
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

        ReductionISToILP {
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
        // Triangle graph: 3 vertices, 3 edges
        let problem = IndependentSet::<i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
        let reduction: ReductionISToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        // Check ILP structure
        assert_eq!(ilp.num_vars, 3, "Should have one variable per vertex");
        assert_eq!(ilp.constraints.len(), 3, "Should have one constraint per edge");
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
        let problem = IndependentSet::with_weights(3, vec![(0, 1)], vec![5, 10, 15]);
        let reduction: ReductionISToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        // Check that weights are correctly transferred to objective
        let mut coeffs: Vec<f64> = vec![0.0; 3];
        for &(var, coef) in &ilp.objective {
            coeffs[var] = coef;
        }
        assert!((coeffs[0] - 5.0).abs() < 1e-9);
        assert!((coeffs[1] - 10.0).abs() < 1e-9);
        assert!((coeffs[2] - 15.0).abs() < 1e-9);
    }

    #[test]
    fn test_ilp_solution_equals_brute_force_triangle() {
        // Triangle graph: max IS = 1 vertex
        let problem = IndependentSet::<i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
        let reduction: ReductionISToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        let bf = BruteForce::new();
        let ilp_solver = ILPSolver::new();

        // Solve with brute force on original problem
        let bf_solutions = bf.find_best(&problem);

        // Solve via ILP reduction
        let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
        let extracted = reduction.extract_solution(&ilp_solution);

        // Both should find optimal size = 1
        let bf_size: usize = bf_solutions[0].iter().sum();
        let ilp_size: usize = extracted.iter().sum();
        assert_eq!(bf_size, 1);
        assert_eq!(ilp_size, 1);

        // Verify the ILP solution is valid for the original problem
        let sol_result = problem.solution_size(&extracted);
        assert!(sol_result.is_valid, "Extracted solution should be valid");
    }

    #[test]
    fn test_ilp_solution_equals_brute_force_path() {
        // Path graph 0-1-2-3: max IS = 2 (e.g., {0, 2} or {1, 3} or {0, 3})
        let problem = IndependentSet::<i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
        let reduction: ReductionISToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        let bf = BruteForce::new();
        let ilp_solver = ILPSolver::new();

        // Solve with brute force
        let bf_solutions = bf.find_best(&problem);
        let bf_size: usize = bf_solutions[0].iter().sum();

        // Solve via ILP
        let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
        let extracted = reduction.extract_solution(&ilp_solution);
        let ilp_size: usize = extracted.iter().sum();

        assert_eq!(bf_size, 2);
        assert_eq!(ilp_size, 2);

        // Verify validity
        let sol_result = problem.solution_size(&extracted);
        assert!(sol_result.is_valid);
    }

    #[test]
    fn test_ilp_solution_equals_brute_force_weighted() {
        // Weighted problem: vertex 1 has high weight but is connected to both 0 and 2
        // 0 -- 1 -- 2
        // Weights: [1, 100, 1]
        // Max IS by weight: just vertex 1 (weight 100) beats 0+2 (weight 2)
        let problem = IndependentSet::with_weights(3, vec![(0, 1), (1, 2)], vec![1, 100, 1]);
        let reduction: ReductionISToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        let bf = BruteForce::new();
        let ilp_solver = ILPSolver::new();

        let bf_solutions = bf.find_best(&problem);
        let bf_obj = problem.solution_size(&bf_solutions[0]).size;

        let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
        let extracted = reduction.extract_solution(&ilp_solution);
        let ilp_obj = problem.solution_size(&extracted).size;

        assert_eq!(bf_obj, 100);
        assert_eq!(ilp_obj, 100);

        // Verify the solution selects vertex 1
        assert_eq!(extracted, vec![0, 1, 0]);
    }

    #[test]
    fn test_solution_extraction() {
        let problem = IndependentSet::<i32>::new(4, vec![(0, 1), (2, 3)]);
        let reduction: ReductionISToILP = ReduceTo::<ILP>::reduce_to(&problem);

        // Test that extraction works correctly (1:1 mapping)
        let ilp_solution = vec![1, 0, 0, 1];
        let extracted = reduction.extract_solution(&ilp_solution);
        assert_eq!(extracted, vec![1, 0, 0, 1]);

        // Verify this is a valid IS (0 and 3 are not adjacent)
        let sol_result = problem.solution_size(&extracted);
        assert!(sol_result.is_valid);
    }

    #[test]
    fn test_source_and_target_size() {
        let problem = IndependentSet::<i32>::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]);
        let reduction: ReductionISToILP = ReduceTo::<ILP>::reduce_to(&problem);

        let source_size = reduction.source_size();
        let target_size = reduction.target_size();

        assert_eq!(source_size.get("num_vertices"), Some(5));
        assert_eq!(source_size.get("num_edges"), Some(4));

        assert_eq!(target_size.get("num_vars"), Some(5));
        assert_eq!(target_size.get("num_constraints"), Some(4));
    }

    #[test]
    fn test_empty_graph() {
        // Graph with no edges: all vertices can be selected
        let problem = IndependentSet::<i32>::new(3, vec![]);
        let reduction: ReductionISToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        assert_eq!(ilp.constraints.len(), 0);

        let ilp_solver = ILPSolver::new();
        let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
        let extracted = reduction.extract_solution(&ilp_solution);

        // All vertices should be selected
        assert_eq!(extracted, vec![1, 1, 1]);

        let sol_result = problem.solution_size(&extracted);
        assert!(sol_result.is_valid);
        assert_eq!(sol_result.size, 3);
    }

    #[test]
    fn test_complete_graph() {
        // Complete graph K4: max IS = 1
        let problem = IndependentSet::<i32>::new(
            4,
            vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)],
        );
        let reduction: ReductionISToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        assert_eq!(ilp.constraints.len(), 6);

        let ilp_solver = ILPSolver::new();
        let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
        let extracted = reduction.extract_solution(&ilp_solution);

        let sol_result = problem.solution_size(&extracted);
        assert!(sol_result.is_valid);
        assert_eq!(sol_result.size, 1);
    }

    #[test]
    fn test_solve_reduced() {
        // Test the ILPSolver::solve_reduced method
        let problem = IndependentSet::<i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);

        let ilp_solver = ILPSolver::new();
        let solution = ilp_solver
            .solve_reduced(&problem)
            .expect("solve_reduced should work");

        let sol_result = problem.solution_size(&solution);
        assert!(sol_result.is_valid);
        assert_eq!(sol_result.size, 2);
    }

    #[test]
    fn test_bipartite_graph() {
        // Bipartite graph: 0-2, 0-3, 1-2, 1-3 (two independent sets: {0,1} and {2,3})
        // With equal weights, max IS = 2
        let problem = IndependentSet::<i32>::new(4, vec![(0, 2), (0, 3), (1, 2), (1, 3)]);
        let reduction: ReductionISToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        let ilp_solver = ILPSolver::new();
        let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
        let extracted = reduction.extract_solution(&ilp_solution);

        let sol_result = problem.solution_size(&extracted);
        assert!(sol_result.is_valid);
        assert_eq!(sol_result.size, 2);

        // Should select either {0, 1} or {2, 3}
        let sum: usize = extracted.iter().sum();
        assert_eq!(sum, 2);
    }
}
