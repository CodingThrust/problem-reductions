//! Reduction from Matching to ILP (Integer Linear Programming).
//!
//! The Maximum Matching problem can be formulated as a binary ILP:
//! - Variables: One binary variable per edge (0 = not selected, 1 = selected)
//! - Constraints: For each vertex v, sum of incident edge variables <= 1
//!   (at most one incident edge can be selected)
//! - Objective: Maximize the sum of weights of selected edges

use crate::models::graph::Matching;
use crate::models::optimization::{ILP, LinearConstraint, ObjectiveSense, VarBounds};
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::traits::{ConstraintSatisfactionProblem, Problem};
use crate::types::ProblemSize;

/// Result of reducing Matching to ILP.
///
/// This reduction creates a binary ILP where:
/// - Each edge corresponds to a binary variable
/// - Vertex constraints ensure at most one incident edge is selected per vertex
/// - The objective maximizes the total weight of selected edges
#[derive(Debug, Clone)]
pub struct ReductionMatchingToILP {
    target: ILP,
    source_size: ProblemSize,
}

impl ReductionResult for ReductionMatchingToILP {
    type Source = Matching<i32>;
    type Target = ILP;

    fn target_problem(&self) -> &ILP {
        &self.target
    }

    /// Extract solution from ILP back to Matching.
    ///
    /// Since the mapping is 1:1 (each edge maps to one binary variable),
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

impl ReduceTo<ILP> for Matching<i32> {
    type Result = ReductionMatchingToILP;

    fn reduce_to(&self) -> Self::Result {
        let num_vars = self.num_variables(); // Number of edges

        // All variables are binary (0 or 1)
        let bounds = vec![VarBounds::binary(); num_vars];

        // Constraints: For each vertex v, sum of incident edge variables <= 1
        // This ensures at most one incident edge is selected per vertex
        let v2e = self.vertex_to_edges();
        let constraints: Vec<LinearConstraint> = v2e
            .into_iter()
            .filter(|(_, edges)| !edges.is_empty())
            .map(|(_, edges)| {
                let terms: Vec<(usize, f64)> = edges.into_iter().map(|e| (e, 1.0)).collect();
                LinearConstraint::le(terms, 1.0)
            })
            .collect();

        // Objective: maximize sum of w_e * x_e (weighted sum of selected edges)
        let weights = self.weights();
        let objective: Vec<(usize, f64)> = weights
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

        ReductionMatchingToILP {
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
        let problem = Matching::<i32>::unweighted(3, vec![(0, 1), (1, 2), (0, 2)]);
        let reduction: ReductionMatchingToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        // Check ILP structure
        assert_eq!(ilp.num_vars, 3, "Should have one variable per edge");
        // Each vertex has degree 2, so 3 constraints (one per vertex)
        assert_eq!(ilp.constraints.len(), 3, "Should have one constraint per vertex");
        assert_eq!(ilp.sense, ObjectiveSense::Maximize, "Should maximize");

        // All variables should be binary
        for bound in &ilp.bounds {
            assert_eq!(*bound, VarBounds::binary());
        }

        // Each constraint should be sum of incident edge vars <= 1
        for constraint in &ilp.constraints {
            assert!((constraint.rhs - 1.0).abs() < 1e-9);
        }
    }

    #[test]
    fn test_reduction_weighted() {
        let problem = Matching::new(3, vec![(0, 1, 5), (1, 2, 10)]);
        let reduction: ReductionMatchingToILP = ReduceTo::<ILP>::reduce_to(&problem);
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
    fn test_ilp_solution_equals_brute_force_triangle() {
        // Triangle graph: max matching = 1 edge
        let problem = Matching::<i32>::unweighted(3, vec![(0, 1), (1, 2), (0, 2)]);
        let reduction: ReductionMatchingToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        let bf = BruteForce::new();
        let ilp_solver = ILPSolver::new();

        // Solve with brute force on original problem
        let bf_solutions = bf.find_best(&problem);

        // Solve via ILP reduction
        let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
        let extracted = reduction.extract_solution(&ilp_solution);

        // Both should find optimal size = 1 (one edge)
        let bf_size = problem.solution_size(&bf_solutions[0]).size;
        let ilp_size = problem.solution_size(&extracted).size;
        assert_eq!(bf_size, 1);
        assert_eq!(ilp_size, 1);

        // Verify the ILP solution is valid for the original problem
        let sol_result = problem.solution_size(&extracted);
        assert!(sol_result.is_valid, "Extracted solution should be valid");
    }

    #[test]
    fn test_ilp_solution_equals_brute_force_path() {
        // Path graph 0-1-2-3: max matching = 2 (edges {0-1, 2-3})
        let problem = Matching::<i32>::unweighted(4, vec![(0, 1), (1, 2), (2, 3)]);
        let reduction: ReductionMatchingToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        let bf = BruteForce::new();
        let ilp_solver = ILPSolver::new();

        // Solve with brute force
        let bf_solutions = bf.find_best(&problem);
        let bf_size = problem.solution_size(&bf_solutions[0]).size;

        // Solve via ILP
        let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
        let extracted = reduction.extract_solution(&ilp_solution);
        let ilp_size = problem.solution_size(&extracted).size;

        assert_eq!(bf_size, 2);
        assert_eq!(ilp_size, 2);

        // Verify validity
        let sol_result = problem.solution_size(&extracted);
        assert!(sol_result.is_valid);
    }

    #[test]
    fn test_ilp_solution_equals_brute_force_weighted() {
        // Weighted matching: edge 0-1 has high weight
        // 0 -- 1 -- 2
        // Weights: [100, 1]
        // Max matching by weight: just edge 0-1 (weight 100) beats edge 1-2 (weight 1)
        let problem = Matching::new(3, vec![(0, 1, 100), (1, 2, 1)]);
        let reduction: ReductionMatchingToILP = ReduceTo::<ILP>::reduce_to(&problem);
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

        // Verify the solution selects edge 0 (0-1)
        assert_eq!(extracted, vec![1, 0]);
    }

    #[test]
    fn test_solution_extraction() {
        let problem = Matching::<i32>::unweighted(4, vec![(0, 1), (2, 3)]);
        let reduction: ReductionMatchingToILP = ReduceTo::<ILP>::reduce_to(&problem);

        // Test that extraction works correctly (1:1 mapping)
        let ilp_solution = vec![1, 1];
        let extracted = reduction.extract_solution(&ilp_solution);
        assert_eq!(extracted, vec![1, 1]);

        // Verify this is a valid matching (edges 0-1 and 2-3 are disjoint)
        let sol_result = problem.solution_size(&extracted);
        assert!(sol_result.is_valid);
    }

    #[test]
    fn test_source_and_target_size() {
        let problem = Matching::<i32>::unweighted(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]);
        let reduction: ReductionMatchingToILP = ReduceTo::<ILP>::reduce_to(&problem);

        let source_size = reduction.source_size();
        let target_size = reduction.target_size();

        assert_eq!(source_size.get("num_vertices"), Some(5));
        assert_eq!(source_size.get("num_edges"), Some(4));

        assert_eq!(target_size.get("num_vars"), Some(4));
        // Constraints: one per vertex with degree >= 1
        // Vertices 0,1,2,3,4 have degrees 1,2,2,2,1 respectively
        assert_eq!(target_size.get("num_constraints"), Some(5));
    }

    #[test]
    fn test_empty_graph() {
        // Graph with no edges: empty matching
        let problem = Matching::<i32>::unweighted(3, vec![]);
        let reduction: ReductionMatchingToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        assert_eq!(ilp.num_vars, 0);
        assert_eq!(ilp.constraints.len(), 0);

        let sol_result = problem.solution_size(&[]);
        assert!(sol_result.is_valid);
        assert_eq!(sol_result.size, 0);
    }

    #[test]
    fn test_k4_perfect_matching() {
        // Complete graph K4: can have perfect matching (2 edges covering all 4 vertices)
        let problem = Matching::<i32>::unweighted(
            4,
            vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)],
        );
        let reduction: ReductionMatchingToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        // 6 edges, 4 vertices with constraints
        assert_eq!(ilp.num_vars, 6);
        assert_eq!(ilp.constraints.len(), 4);

        let ilp_solver = ILPSolver::new();
        let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
        let extracted = reduction.extract_solution(&ilp_solution);

        let sol_result = problem.solution_size(&extracted);
        assert!(sol_result.is_valid);
        assert_eq!(sol_result.size, 2); // Perfect matching has 2 edges

        // Verify all vertices are matched
        let sum: usize = extracted.iter().sum();
        assert_eq!(sum, 2);
    }

    #[test]
    fn test_star_graph() {
        // Star graph with center vertex 0 connected to 1, 2, 3
        // Max matching = 1 (only one edge can be selected)
        let problem = Matching::<i32>::unweighted(4, vec![(0, 1), (0, 2), (0, 3)]);
        let reduction: ReductionMatchingToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        let ilp_solver = ILPSolver::new();
        let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
        let extracted = reduction.extract_solution(&ilp_solution);

        let sol_result = problem.solution_size(&extracted);
        assert!(sol_result.is_valid);
        assert_eq!(sol_result.size, 1);
    }

    #[test]
    fn test_bipartite_graph() {
        // Bipartite graph: {0,1} and {2,3} with all cross edges
        // Max matching = 2 (one perfect matching)
        let problem = Matching::<i32>::unweighted(4, vec![(0, 2), (0, 3), (1, 2), (1, 3)]);
        let reduction: ReductionMatchingToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        let ilp_solver = ILPSolver::new();
        let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
        let extracted = reduction.extract_solution(&ilp_solution);

        let sol_result = problem.solution_size(&extracted);
        assert!(sol_result.is_valid);
        assert_eq!(sol_result.size, 2);
    }

    #[test]
    fn test_solve_reduced() {
        // Test the ILPSolver::solve_reduced method
        let problem = Matching::<i32>::unweighted(4, vec![(0, 1), (1, 2), (2, 3)]);

        let ilp_solver = ILPSolver::new();
        let solution = ilp_solver
            .solve_reduced(&problem)
            .expect("solve_reduced should work");

        let sol_result = problem.solution_size(&solution);
        assert!(sol_result.is_valid);
        assert_eq!(sol_result.size, 2);
    }
}
