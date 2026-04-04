use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::models::graph::MaximumLeafSpanningTree;
use crate::rules::ReduceTo;
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Max;

/// Small instance: 4 vertices, 4 edges (P4 with a shortcut).
/// Vertices 0-1-2-3 plus edge 0-2.
fn small_instance() -> MaximumLeafSpanningTree<SimpleGraph> {
    MaximumLeafSpanningTree::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (0, 2)]))
}

/// Issue #897 canonical instance: 6 vertices, 9 edges.
fn canonical_instance() -> MaximumLeafSpanningTree<SimpleGraph> {
    MaximumLeafSpanningTree::new(SimpleGraph::new(
        6,
        vec![
            (0, 1),
            (0, 2),
            (0, 3),
            (1, 4),
            (2, 4),
            (2, 5),
            (3, 5),
            (4, 5),
            (1, 3),
        ],
    ))
}

#[test]
fn test_reduction_creates_expected_ilp_shape() {
    let problem = small_instance();
    let reduction: ReductionMaximumLeafSpanningTreeToILP =
        ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // n=4, m=4: num_vars = 3*4 + 4 = 16
    assert_eq!(ilp.num_vars, 16);
    assert_eq!(ilp.sense, ObjectiveSense::Maximize);
    // Objective should be z_0 + z_1 + z_2 + z_3 (indices 4..8)
    assert_eq!(ilp.objective, vec![(4, 1.0), (5, 1.0), (6, 1.0), (7, 1.0)]);
}

#[test]
fn test_maximumleafspanningtree_to_ilp_closed_loop() {
    let problem = small_instance();
    let reduction: ReductionMaximumLeafSpanningTreeToILP =
        ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();
    let best_source = bf.find_all_witnesses(&problem);
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    // All brute-force optimal solutions have the same value
    let bf_value = problem.evaluate(&best_source[0]);
    let ilp_value = problem.evaluate(&extracted);
    assert_eq!(ilp_value, bf_value);
    assert!(problem.is_valid_solution(&extracted));
}

#[test]
fn test_maximumleafspanningtree_to_ilp_canonical_closed_loop() {
    let problem = canonical_instance();
    let reduction: ReductionMaximumLeafSpanningTreeToILP =
        ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();
    let best_source = bf.find_all_witnesses(&problem);
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert_eq!(problem.evaluate(&best_source[0]), Max(Some(4)));
    assert_eq!(problem.evaluate(&extracted), Max(Some(4)));
    assert!(problem.is_valid_solution(&extracted));
}

#[test]
fn test_solution_extraction_reads_edge_selector_prefix() {
    let problem = small_instance();
    let reduction: ReductionMaximumLeafSpanningTreeToILP =
        ReduceTo::<ILP<i32>>::reduce_to(&problem);

    // 16 variables total, first 4 are edge selectors
    let mut target_solution = vec![0; 16];
    target_solution[0] = 1; // edge (0,1)
    target_solution[1] = 1; // edge (1,2)
    target_solution[2] = 1; // edge (2,3)

    assert_eq!(
        reduction.extract_solution(&target_solution),
        vec![1, 1, 1, 0]
    );
}

#[test]
fn test_reduce_and_solve_via_ilp() {
    let problem = canonical_instance();
    let reduction: ReductionMaximumLeafSpanningTreeToILP =
        ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), Max(Some(4)));
    assert!(problem.is_valid_solution(&extracted));
}

#[test]
fn test_maximumleafspanningtree_to_ilp_bf_vs_ilp() {
    let problem = canonical_instance();
    let reduction: ReductionMaximumLeafSpanningTreeToILP =
        ReduceTo::<ILP<i32>>::reduce_to(&problem);
    crate::rules::test_helpers::assert_bf_vs_ilp(&problem, &reduction);
}

#[test]
fn test_maximumleafspanningtree_to_ilp_path_graph() {
    // Path P4: 0-1-2-3, only spanning tree is the path itself => 2 leaves
    let problem = MaximumLeafSpanningTree::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]));
    let reduction: ReductionMaximumLeafSpanningTreeToILP =
        ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), Max(Some(2)));
}

#[test]
fn test_maximumleafspanningtree_to_ilp_star_graph() {
    // Star K1,3: center 0, leaves 1,2,3 => 3 leaves
    let problem = MaximumLeafSpanningTree::new(SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3)]));
    let reduction: ReductionMaximumLeafSpanningTreeToILP =
        ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), Max(Some(3)));
    assert!(problem.is_valid_solution(&extracted));
}

#[test]
fn test_maximumleafspanningtree_to_ilp_complete_graph() {
    // K4: 4 vertices, 6 edges. Star spanning tree has 3 leaves.
    let problem = MaximumLeafSpanningTree::new(SimpleGraph::new(
        4,
        vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)],
    ));
    let bf = BruteForce::new();
    let bf_solutions = bf.find_all_witnesses(&problem);
    let bf_value = problem.evaluate(&bf_solutions[0]);

    let reduction: ReductionMaximumLeafSpanningTreeToILP =
        ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    let ilp_solution = ILPSolver::new().solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert_eq!(problem.evaluate(&extracted), bf_value);
    assert_eq!(bf_value, Max(Some(3)));
}
