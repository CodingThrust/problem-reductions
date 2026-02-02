//! Reduction from Coloring to ILP (Integer Linear Programming).
//!
//! The Graph K-Coloring problem can be formulated as a binary ILP:
//! - Variables: x_{v,c} for each vertex v and color c (binary, 1 if vertex v has color c)
//! - Constraints:
//!   1. Each vertex has exactly one color: sum_c x_{v,c} = 1 for each vertex v
//!   2. Adjacent vertices have different colors: x_{u,c} + x_{v,c} <= 1 for each edge (u,v) and color c
//! - Objective: None (feasibility problem, minimize 0)

use crate::models::graph::Coloring;
use crate::models::optimization::{ILP, LinearConstraint, ObjectiveSense, VarBounds};
use crate::polynomial::{Monomial, Polynomial};
use crate::rules::registry::{ReductionEntry, ReductionOverhead};
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::traits::Problem;
use crate::types::ProblemSize;

// Register reduction in the inventory for automatic discovery
inventory::submit! {
    ReductionEntry {
        source_name: "Coloring",
        target_name: "ILP",
        source_variant: &[("graph", "SimpleGraph"), ("weight", "Unweighted")],
        target_variant: &[("graph", ""), ("weight", "Unweighted")],
        overhead_fn: || ReductionOverhead::new(vec![
            // num_vars = num_vertices * num_colors
            ("num_vars", Polynomial {
                terms: vec![Monomial {
                    coefficient: 1.0,
                    variables: vec![("num_vertices", 1), ("num_colors", 1)],
                }]
            }),
            // num_constraints = num_vertices + num_edges * num_colors
            ("num_constraints", Polynomial {
                terms: vec![
                    Monomial::var("num_vertices"),
                    Monomial {
                        coefficient: 1.0,
                        variables: vec![("num_edges", 1), ("num_colors", 1)],
                    },
                ]
            }),
        ]),
    }
}

/// Result of reducing Coloring to ILP.
///
/// This reduction creates a binary ILP where:
/// - Each (vertex, color) pair corresponds to a binary variable
/// - Constraints ensure each vertex has exactly one color
/// - Constraints ensure adjacent vertices have different colors
#[derive(Debug, Clone)]
pub struct ReductionColoringToILP {
    target: ILP,
    source_size: ProblemSize,
    num_vertices: usize,
    num_colors: usize,
}

impl ReductionColoringToILP {
    /// Get the variable index for vertex v with color c.
    fn var_index(&self, vertex: usize, color: usize) -> usize {
        vertex * self.num_colors + color
    }
}

impl ReductionResult for ReductionColoringToILP {
    type Source = Coloring;
    type Target = ILP;

    fn target_problem(&self) -> &ILP {
        &self.target
    }

    /// Extract solution from ILP back to Coloring.
    ///
    /// The ILP solution has num_vertices * num_colors binary variables.
    /// For each vertex, we find which color has value 1.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        (0..self.num_vertices)
            .map(|v| {
                (0..self.num_colors)
                    .find(|&c| {
                        let var_idx = self.var_index(v, c);
                        var_idx < target_solution.len() && target_solution[var_idx] == 1
                    })
                    .unwrap_or(0)
            })
            .collect()
    }

    fn source_size(&self) -> ProblemSize {
        self.source_size.clone()
    }

    fn target_size(&self) -> ProblemSize {
        self.target.problem_size()
    }
}

impl ReduceTo<ILP> for Coloring {
    type Result = ReductionColoringToILP;

    fn reduce_to(&self) -> Self::Result {
        let num_vertices = self.num_vertices();
        let num_colors = self.num_colors();
        let num_vars = num_vertices * num_colors;

        // Helper function to get variable index
        let var_index = |v: usize, c: usize| -> usize { v * num_colors + c };

        // All variables are binary (0 or 1)
        let bounds = vec![VarBounds::binary(); num_vars];

        let mut constraints = Vec::new();

        // Constraint 1: Each vertex has exactly one color
        // sum_c x_{v,c} = 1 for each vertex v
        for v in 0..num_vertices {
            let terms: Vec<(usize, f64)> = (0..num_colors).map(|c| (var_index(v, c), 1.0)).collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // Constraint 2: Adjacent vertices have different colors
        // x_{u,c} + x_{v,c} <= 1 for each edge (u,v) and each color c
        for (u, v) in self.edges() {
            for c in 0..num_colors {
                constraints.push(LinearConstraint::le(
                    vec![(var_index(u, c), 1.0), (var_index(v, c), 1.0)],
                    1.0,
                ));
            }
        }

        // Objective: minimize 0 (feasibility problem)
        // We use an empty objective
        let objective: Vec<(usize, f64)> = vec![];

        let target = ILP::new(
            num_vars,
            bounds,
            constraints,
            objective,
            ObjectiveSense::Minimize,
        );

        ReductionColoringToILP {
            target,
            source_size: self.problem_size(),
            num_vertices,
            num_colors,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::{BruteForce, ILPSolver, Solver};

    #[test]
    fn test_reduction_creates_valid_ilp() {
        // Triangle graph with 3 colors
        let problem = Coloring::new(3, 3, vec![(0, 1), (1, 2), (0, 2)]);
        let reduction: ReductionColoringToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        // Check ILP structure
        // num_vars = 3 vertices * 3 colors = 9
        assert_eq!(ilp.num_vars, 9, "Should have 9 variables (3 vertices * 3 colors)");

        // num_constraints = 3 (one per vertex for "exactly one color")
        //                 + 3 edges * 3 colors = 9 (edge constraints)
        //                 = 12 total
        assert_eq!(
            ilp.constraints.len(),
            12,
            "Should have 12 constraints (3 vertex + 9 edge)"
        );

        assert_eq!(ilp.sense, ObjectiveSense::Minimize, "Should minimize");

        // All variables should be binary
        for bound in &ilp.bounds {
            assert_eq!(*bound, VarBounds::binary());
        }
    }

    #[test]
    fn test_reduction_path_graph() {
        // Path graph 0-1-2 with 2 colors (2-colorable)
        let problem = Coloring::new(3, 2, vec![(0, 1), (1, 2)]);
        let reduction: ReductionColoringToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        // num_vars = 3 * 2 = 6
        assert_eq!(ilp.num_vars, 6);

        // constraints = 3 (vertex) + 2 edges * 2 colors = 7
        assert_eq!(ilp.constraints.len(), 7);
    }

    #[test]
    fn test_ilp_solution_equals_brute_force_triangle() {
        // Triangle needs 3 colors
        let problem = Coloring::new(3, 3, vec![(0, 1), (1, 2), (0, 2)]);
        let reduction: ReductionColoringToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        let bf = BruteForce::new();
        let ilp_solver = ILPSolver::new();

        // Solve with brute force on original problem
        let bf_solutions = bf.find_best(&problem);
        assert!(!bf_solutions.is_empty(), "Brute force should find solutions");

        // Solve via ILP reduction
        let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
        let extracted = reduction.extract_solution(&ilp_solution);

        // Verify the extracted solution is valid for the original problem
        let sol_result = problem.solution_size(&extracted);
        assert!(sol_result.is_valid, "Extracted solution should be valid");

        // All three vertices should have different colors
        assert_ne!(extracted[0], extracted[1]);
        assert_ne!(extracted[1], extracted[2]);
        assert_ne!(extracted[0], extracted[2]);
    }

    #[test]
    fn test_ilp_solution_equals_brute_force_path() {
        // Path graph 0-1-2-3 with 2 colors
        let problem = Coloring::new(4, 2, vec![(0, 1), (1, 2), (2, 3)]);
        let reduction: ReductionColoringToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        let ilp_solver = ILPSolver::new();

        // Solve via ILP
        let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
        let extracted = reduction.extract_solution(&ilp_solution);

        // Verify validity
        let sol_result = problem.solution_size(&extracted);
        assert!(sol_result.is_valid, "Extracted solution should be valid");

        // Check adjacent vertices have different colors
        assert_ne!(extracted[0], extracted[1]);
        assert_ne!(extracted[1], extracted[2]);
        assert_ne!(extracted[2], extracted[3]);
    }

    #[test]
    fn test_ilp_infeasible_triangle_2_colors() {
        // Triangle cannot be 2-colored
        let problem = Coloring::new(3, 2, vec![(0, 1), (1, 2), (0, 2)]);
        let reduction: ReductionColoringToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        let ilp_solver = ILPSolver::new();

        // ILP should be infeasible
        let result = ilp_solver.solve(ilp);
        assert!(result.is_none(), "Triangle with 2 colors should be infeasible");
    }

    #[test]
    fn test_solution_extraction() {
        let problem = Coloring::new(3, 3, vec![(0, 1)]);
        let reduction: ReductionColoringToILP = ReduceTo::<ILP>::reduce_to(&problem);

        // ILP solution where:
        // vertex 0 has color 1 (x_{0,1} = 1)
        // vertex 1 has color 2 (x_{1,2} = 1)
        // vertex 2 has color 0 (x_{2,0} = 1)
        // Variables are indexed as: v0c0, v0c1, v0c2, v1c0, v1c1, v1c2, v2c0, v2c1, v2c2
        let ilp_solution = vec![0, 1, 0, 0, 0, 1, 1, 0, 0];
        let extracted = reduction.extract_solution(&ilp_solution);

        assert_eq!(extracted, vec![1, 2, 0]);

        // Verify this is a valid coloring (vertex 0 and 1 have different colors)
        let sol_result = problem.solution_size(&extracted);
        assert!(sol_result.is_valid);
    }

    #[test]
    fn test_source_and_target_size() {
        let problem = Coloring::new(5, 3, vec![(0, 1), (1, 2), (2, 3), (3, 4)]);
        let reduction: ReductionColoringToILP = ReduceTo::<ILP>::reduce_to(&problem);

        let source_size = reduction.source_size();
        let target_size = reduction.target_size();

        assert_eq!(source_size.get("num_vertices"), Some(5));
        assert_eq!(source_size.get("num_edges"), Some(4));
        assert_eq!(source_size.get("num_colors"), Some(3));

        assert_eq!(target_size.get("num_vars"), Some(15)); // 5 * 3
        // constraints = 5 (vertex) + 4 * 3 (edge) = 17
        assert_eq!(target_size.get("num_constraints"), Some(17));
    }

    #[test]
    fn test_empty_graph() {
        // Graph with no edges: any coloring is valid
        let problem = Coloring::new(3, 1, vec![]);
        let reduction: ReductionColoringToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        // Should only have vertex constraints (each vertex = one color)
        assert_eq!(ilp.constraints.len(), 3);

        let ilp_solver = ILPSolver::new();
        let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
        let extracted = reduction.extract_solution(&ilp_solution);

        let sol_result = problem.solution_size(&extracted);
        assert!(sol_result.is_valid);
    }

    #[test]
    fn test_complete_graph_k4() {
        // K4 needs 4 colors
        let problem = Coloring::new(4, 4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
        let reduction: ReductionColoringToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        let ilp_solver = ILPSolver::new();
        let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
        let extracted = reduction.extract_solution(&ilp_solution);

        let sol_result = problem.solution_size(&extracted);
        assert!(sol_result.is_valid);

        // All vertices should have different colors
        let mut colors: Vec<usize> = extracted.clone();
        colors.sort();
        colors.dedup();
        assert_eq!(colors.len(), 4);
    }

    #[test]
    fn test_complete_graph_k4_with_3_colors_infeasible() {
        // K4 cannot be 3-colored
        let problem = Coloring::new(4, 3, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
        let reduction: ReductionColoringToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        let ilp_solver = ILPSolver::new();
        let result = ilp_solver.solve(ilp);
        assert!(result.is_none(), "K4 with 3 colors should be infeasible");
    }

    #[test]
    fn test_bipartite_graph() {
        // Complete bipartite K_{2,2}: 0-2, 0-3, 1-2, 1-3
        // This is 2-colorable
        let problem = Coloring::new(4, 2, vec![(0, 2), (0, 3), (1, 2), (1, 3)]);
        let reduction: ReductionColoringToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        let ilp_solver = ILPSolver::new();
        let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
        let extracted = reduction.extract_solution(&ilp_solution);

        let sol_result = problem.solution_size(&extracted);
        assert!(sol_result.is_valid);

        // Vertices 0,1 should have same color, vertices 2,3 should have same color
        // And different from 0,1
        assert_eq!(extracted[0], extracted[1]);
        assert_eq!(extracted[2], extracted[3]);
        assert_ne!(extracted[0], extracted[2]);
    }

    #[test]
    fn test_solve_reduced() {
        // Test the ILPSolver::solve_reduced method
        let problem = Coloring::new(4, 2, vec![(0, 1), (1, 2), (2, 3)]);

        let ilp_solver = ILPSolver::new();
        let solution = ilp_solver
            .solve_reduced(&problem)
            .expect("solve_reduced should work");

        let sol_result = problem.solution_size(&solution);
        assert!(sol_result.is_valid);
    }

    #[test]
    fn test_single_vertex() {
        // Single vertex graph: always 1-colorable
        let problem = Coloring::new(1, 1, vec![]);
        let reduction: ReductionColoringToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        assert_eq!(ilp.num_vars, 1);
        assert_eq!(ilp.constraints.len(), 1); // Just the "exactly one color" constraint

        let ilp_solver = ILPSolver::new();
        let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
        let extracted = reduction.extract_solution(&ilp_solution);

        assert_eq!(extracted, vec![0]);
    }

    #[test]
    fn test_single_edge() {
        // Single edge: needs 2 colors
        let problem = Coloring::new(2, 2, vec![(0, 1)]);
        let reduction: ReductionColoringToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        let ilp_solver = ILPSolver::new();
        let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
        let extracted = reduction.extract_solution(&ilp_solution);

        let sol_result = problem.solution_size(&extracted);
        assert!(sol_result.is_valid);
        assert_ne!(extracted[0], extracted[1]);
    }
}
