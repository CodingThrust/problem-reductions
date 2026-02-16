use super::*;
use crate::solvers::ILPSolver;

/// Check if a configuration represents a valid clique in the graph.
/// A clique is valid if all selected vertices are pairwise adjacent.
fn is_valid_clique(problem: &MaximumClique<SimpleGraph, i32>, config: &[usize]) -> bool {
    let selected: Vec<usize> = config
        .iter()
        .enumerate()
        .filter(|(_, &v)| v == 1)
        .map(|(i, _)| i)
        .collect();

    // Check all pairs of selected vertices are adjacent
    for i in 0..selected.len() {
        for j in (i + 1)..selected.len() {
            if !problem.graph().has_edge(selected[i], selected[j]) {
                return false;
            }
        }
    }
    true
}

/// Compute the clique size (sum of weights of selected vertices).
fn clique_size(problem: &MaximumClique<SimpleGraph, i32>, config: &[usize]) -> i32 {
    config
        .iter()
        .enumerate()
        .filter(|(_, &v)| v == 1)
        .map(|(i, _)| problem.weights()[i])
        .sum()
}

/// Find maximum clique size by brute force enumeration.
fn brute_force_max_clique(problem: &MaximumClique<SimpleGraph, i32>) -> i32 {
    let n = problem.graph().num_vertices();
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
    let problem: MaximumClique<SimpleGraph, i32> =
        MaximumClique::new(SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]), vec![1; 3]);
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
    let problem: MaximumClique<SimpleGraph, i32> = MaximumClique::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1; 3]);
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
    let problem: MaximumClique<SimpleGraph, i32> =
        MaximumClique::new(SimpleGraph::new(3, vec![(0, 1)]), vec![5, 10, 15]);
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
    let problem: MaximumClique<SimpleGraph, i32> =
        MaximumClique::new(SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]), vec![1; 3]);
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
    let problem: MaximumClique<SimpleGraph, i32> =
        MaximumClique::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]), vec![1; 4]);
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
    let problem: MaximumClique<SimpleGraph, i32> =
        MaximumClique::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1, 100, 1]);
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
    let problem: MaximumClique<SimpleGraph, i32> = MaximumClique::new(SimpleGraph::new(4, vec![(0, 1), (2, 3)]), vec![1; 4]);
    let reduction: ReductionCliqueToILP = ReduceTo::<ILP>::reduce_to(&problem);

    // Test that extraction works correctly (1:1 mapping)
    let ilp_solution = vec![1, 1, 0, 0];
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted, vec![1, 1, 0, 0]);

    // Verify this is a valid clique (0 and 1 are adjacent)
    assert!(is_valid_clique(&problem, &extracted));
}

#[test]
fn test_ilp_structure() {
    let problem: MaximumClique<SimpleGraph, i32> =
        MaximumClique::new(SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]), vec![1; 5]);
    let reduction: ReductionCliqueToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars, 5);
    // Number of non-edges in a path of 5 vertices: C(5,2) - 4 = 10 - 4 = 6
    assert_eq!(ilp.constraints.len(), 6);
}

#[test]
fn test_empty_graph() {
    // Graph with no edges: max clique = 1 (any single vertex)
    let problem: MaximumClique<SimpleGraph, i32> = MaximumClique::new(SimpleGraph::new(3, vec![]), vec![1; 3]);
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
    let problem: MaximumClique<SimpleGraph, i32> =
        MaximumClique::new(SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]), vec![1; 4]);
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
    let problem: MaximumClique<SimpleGraph, i32> =
        MaximumClique::new(SimpleGraph::new(4, vec![(0, 2), (0, 3), (1, 2), (1, 3)]), vec![1; 4]);
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
    let problem: MaximumClique<SimpleGraph, i32> =
        MaximumClique::new(SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3)]), vec![1; 4]);
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
