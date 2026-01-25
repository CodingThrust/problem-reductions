//! Reduction from Clique to ILP (Integer Linear Programming).
//!
//! The Clique problem can be formulated as a binary ILP:
//! - Variables: One binary variable per vertex (0 = not selected, 1 = selected)
//! - Constraints: x_u + x_v <= 1 for each NON-EDGE (u, v) - if two vertices are not adjacent,
//!   at most one can be in the clique
//! - Objective: Maximize the sum of weights of selected vertices

use crate::models::graph::CliqueT;
use crate::models::optimization::{ILP, LinearConstraint, ObjectiveSense, VarBounds};
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;
use crate::traits::{ConstraintSatisfactionProblem, Problem};
use crate::types::ProblemSize;

/// Result of reducing Clique to ILP.
///
/// This reduction creates a binary ILP where:
/// - Each vertex corresponds to a binary variable
/// - Non-edge constraints ensure at most one endpoint of each non-edge is selected
/// - The objective maximizes the total weight of selected vertices
#[derive(Debug, Clone)]
pub struct ReductionCliqueToILP {
    target: ILP,
    source_size: ProblemSize,
}

impl ReductionResult for ReductionCliqueToILP {
    type Source = CliqueT<SimpleGraph, i32>;
    type Target = ILP;

    fn target_problem(&self) -> &ILP {
        &self.target
    }

    /// Extract solution from ILP back to Clique.
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

impl ReduceTo<ILP> for CliqueT<SimpleGraph, i32> {
    type Result = ReductionCliqueToILP;

    fn reduce_to(&self) -> Self::Result {
        let num_vars = self.num_vertices();

        // All variables are binary (0 or 1)
        let bounds = vec![VarBounds::binary(); num_vars];

        // Constraints: x_u + x_v <= 1 for each NON-EDGE (u, v)
        // This ensures at most one vertex of each non-edge is selected (i.e., if both
        // are selected, they must be adjacent, forming a clique)
        let mut constraints: Vec<LinearConstraint> = Vec::new();
        for u in 0..num_vars {
            for v in (u + 1)..num_vars {
                if !self.has_edge(u, v) {
                    constraints.push(LinearConstraint::le(vec![(u, 1.0), (v, 1.0)], 1.0));
                }
            }
        }

        // Objective: maximize sum of w_i * x_i (weighted sum of selected vertices)
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
            ObjectiveSense::Maximize,
        );

        ReductionCliqueToILP {
            target,
            source_size: self.problem_size(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::ILPSolver;

    /// Check if a configuration represents a valid clique in the graph.
    /// A clique is valid if all selected vertices are pairwise adjacent.
    fn is_valid_clique(problem: &CliqueT<SimpleGraph, i32>, config: &[usize]) -> bool {
        let selected: Vec<usize> = config
            .iter()
            .enumerate()
            .filter(|(_, &v)| v == 1)
            .map(|(i, _)| i)
            .collect();

        // Check all pairs of selected vertices are adjacent
        for i in 0..selected.len() {
            for j in (i + 1)..selected.len() {
                if !problem.has_edge(selected[i], selected[j]) {
                    return false;
                }
            }
        }
        true
    }

    /// Compute the clique size (sum of weights of selected vertices).
    fn clique_size(problem: &CliqueT<SimpleGraph, i32>, config: &[usize]) -> i32 {
        let weights = problem.weights();
        config
            .iter()
            .enumerate()
            .filter(|(_, &v)| v == 1)
            .map(|(i, _)| weights[i])
            .sum()
    }

    /// Find maximum clique size by brute force enumeration.
    fn brute_force_max_clique(problem: &CliqueT<SimpleGraph, i32>) -> i32 {
        let n = problem.num_vertices();
        let mut max_size = 0;
        for mask in 0..(1 << n) {
            let config: Vec<usize> = (0..n).map(|i| (mask >> i) & 1).collect();
            if is_valid_clique(problem, &config) {
                let size = clique_size(problem, &config);
                if size > max_size {
                    max_size = size;
                }
            }
        }
        max_size
    }

    #[test]
    fn test_reduction_creates_valid_ilp() {
        // Triangle graph: 3 vertices, 3 edges (complete graph K3)
        // All pairs are adjacent, so no constraints should be added
        let problem: CliqueT<SimpleGraph, i32> =
            CliqueT::new(3, vec![(0, 1), (1, 2), (0, 2)]);
        let reduction: ReductionCliqueToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        // Check ILP structure
        assert_eq!(ilp.num_vars, 3, "Should have one variable per vertex");
        assert_eq!(
            ilp.constraints.len(),
            0,
            "Complete graph has no non-edges, so no constraints"
        );
        assert_eq!(ilp.sense, ObjectiveSense::Maximize, "Should maximize");

        // All variables should be binary
        for bound in &ilp.bounds {
            assert_eq!(*bound, VarBounds::binary());
        }
    }

    #[test]
    fn test_reduction_with_non_edges() {
        // Path graph 0-1-2: edges (0,1) and (1,2), non-edge (0,2)
        let problem: CliqueT<SimpleGraph, i32> = CliqueT::new(3, vec![(0, 1), (1, 2)]);
        let reduction: ReductionCliqueToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        // Should have 1 constraint for non-edge (0, 2)
        assert_eq!(ilp.constraints.len(), 1);

        // The constraint should be x_0 + x_2 <= 1
        let constraint = &ilp.constraints[0];
        assert_eq!(constraint.terms.len(), 2);
        assert!((constraint.rhs - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_reduction_weighted() {
        let problem: CliqueT<SimpleGraph, i32> =
            CliqueT::with_weights(3, vec![(0, 1)], vec![5, 10, 15]);
        let reduction: ReductionCliqueToILP = ReduceTo::<ILP>::reduce_to(&problem);
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
        // Triangle graph (K3): max clique = 3 vertices
        let problem: CliqueT<SimpleGraph, i32> =
            CliqueT::new(3, vec![(0, 1), (1, 2), (0, 2)]);
        let reduction: ReductionCliqueToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        let ilp_solver = ILPSolver::new();

        // Solve with brute force for clique
        let bf_size = brute_force_max_clique(&problem);

        // Solve via ILP reduction
        let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
        let extracted = reduction.extract_solution(&ilp_solution);

        // Both should find optimal size = 3 (all vertices form a clique)
        let ilp_size = clique_size(&problem, &extracted);
        assert_eq!(bf_size, 3);
        assert_eq!(ilp_size, 3);

        // Verify the ILP solution is a valid clique
        assert!(
            is_valid_clique(&problem, &extracted),
            "Extracted solution should be a valid clique"
        );
    }

    #[test]
    fn test_ilp_solution_equals_brute_force_path() {
        // Path graph 0-1-2-3: max clique = 2 (any adjacent pair)
        let problem: CliqueT<SimpleGraph, i32> =
            CliqueT::new(4, vec![(0, 1), (1, 2), (2, 3)]);
        let reduction: ReductionCliqueToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        let ilp_solver = ILPSolver::new();

        // Solve with brute force for clique
        let bf_size = brute_force_max_clique(&problem);

        // Solve via ILP
        let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
        let extracted = reduction.extract_solution(&ilp_solution);
        let ilp_size = clique_size(&problem, &extracted);

        assert_eq!(bf_size, 2);
        assert_eq!(ilp_size, 2);

        // Verify validity
        assert!(is_valid_clique(&problem, &extracted));
    }

    #[test]
    fn test_ilp_solution_equals_brute_force_weighted() {
        // Triangle with one missing edge: 0-1, 1-2, but no 0-2
        // Weights: [1, 100, 1]
        // Max clique by weight: {0, 1} (weight 101) or {1, 2} (weight 101), or just {1} (weight 100)
        // Since 0-1 and 1-2 are edges, both {0,1} and {1,2} are valid cliques
        let problem: CliqueT<SimpleGraph, i32> =
            CliqueT::with_weights(3, vec![(0, 1), (1, 2)], vec![1, 100, 1]);
        let reduction: ReductionCliqueToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        let ilp_solver = ILPSolver::new();

        let bf_obj = brute_force_max_clique(&problem);

        let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
        let extracted = reduction.extract_solution(&ilp_solution);
        let ilp_obj = clique_size(&problem, &extracted);

        assert_eq!(bf_obj, 101);
        assert_eq!(ilp_obj, 101);

        // Verify the solution is a valid clique
        assert!(is_valid_clique(&problem, &extracted));
    }

    #[test]
    fn test_solution_extraction() {
        let problem: CliqueT<SimpleGraph, i32> =
            CliqueT::new(4, vec![(0, 1), (2, 3)]);
        let reduction: ReductionCliqueToILP = ReduceTo::<ILP>::reduce_to(&problem);

        // Test that extraction works correctly (1:1 mapping)
        let ilp_solution = vec![1, 1, 0, 0];
        let extracted = reduction.extract_solution(&ilp_solution);
        assert_eq!(extracted, vec![1, 1, 0, 0]);

        // Verify this is a valid clique (0 and 1 are adjacent)
        assert!(is_valid_clique(&problem, &extracted));
    }

    #[test]
    fn test_source_and_target_size() {
        let problem: CliqueT<SimpleGraph, i32> =
            CliqueT::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]);
        let reduction: ReductionCliqueToILP = ReduceTo::<ILP>::reduce_to(&problem);

        let source_size = reduction.source_size();
        let target_size = reduction.target_size();

        assert_eq!(source_size.get("num_vertices"), Some(5));
        assert_eq!(source_size.get("num_edges"), Some(4));

        assert_eq!(target_size.get("num_vars"), Some(5));
        // Number of non-edges in a path of 5 vertices: C(5,2) - 4 = 10 - 4 = 6
        assert_eq!(target_size.get("num_constraints"), Some(6));
    }

    #[test]
    fn test_empty_graph() {
        // Graph with no edges: max clique = 1 (any single vertex)
        let problem: CliqueT<SimpleGraph, i32> = CliqueT::new(3, vec![]);
        let reduction: ReductionCliqueToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        // All pairs are non-edges, so 3 constraints
        assert_eq!(ilp.constraints.len(), 3);

        let ilp_solver = ILPSolver::new();
        let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
        let extracted = reduction.extract_solution(&ilp_solution);

        // Only one vertex should be selected
        assert_eq!(extracted.iter().sum::<usize>(), 1);

        assert!(is_valid_clique(&problem, &extracted));
        assert_eq!(clique_size(&problem, &extracted), 1);
    }

    #[test]
    fn test_complete_graph() {
        // Complete graph K4: max clique = 4 (all vertices)
        let problem: CliqueT<SimpleGraph, i32> = CliqueT::new(
            4,
            vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)],
        );
        let reduction: ReductionCliqueToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        // No non-edges, so no constraints
        assert_eq!(ilp.constraints.len(), 0);

        let ilp_solver = ILPSolver::new();
        let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
        let extracted = reduction.extract_solution(&ilp_solution);

        // All vertices should be selected
        assert_eq!(extracted, vec![1, 1, 1, 1]);

        assert!(is_valid_clique(&problem, &extracted));
        assert_eq!(clique_size(&problem, &extracted), 4);
    }

    #[test]
    fn test_bipartite_graph() {
        // Bipartite graph: 0-2, 0-3, 1-2, 1-3 (two independent sets: {0,1} and {2,3})
        // Max clique = 2 (any edge, e.g., {0, 2})
        let problem: CliqueT<SimpleGraph, i32> =
            CliqueT::new(4, vec![(0, 2), (0, 3), (1, 2), (1, 3)]);
        let reduction: ReductionCliqueToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        let ilp_solver = ILPSolver::new();
        let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
        let extracted = reduction.extract_solution(&ilp_solution);

        assert!(is_valid_clique(&problem, &extracted));
        assert_eq!(clique_size(&problem, &extracted), 2);

        // Should select an adjacent pair
        let sum: usize = extracted.iter().sum();
        assert_eq!(sum, 2);
    }

    #[test]
    fn test_star_graph() {
        // Star graph: center 0 connected to 1, 2, 3
        // Max clique = 2 (center + any leaf)
        let problem: CliqueT<SimpleGraph, i32> =
            CliqueT::new(4, vec![(0, 1), (0, 2), (0, 3)]);
        let reduction: ReductionCliqueToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        // Non-edges: (1,2), (1,3), (2,3) = 3 constraints
        assert_eq!(ilp.constraints.len(), 3);

        let ilp_solver = ILPSolver::new();
        let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
        let extracted = reduction.extract_solution(&ilp_solution);

        assert!(is_valid_clique(&problem, &extracted));
        assert_eq!(clique_size(&problem, &extracted), 2);
    }
}
