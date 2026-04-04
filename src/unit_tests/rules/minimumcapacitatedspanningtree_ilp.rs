use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::models::graph::MinimumCapacitatedSpanningTree;
use crate::rules::ReduceTo;
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Min;

/// Small instance: 4 vertices, 5 edges.
fn small_instance() -> MinimumCapacitatedSpanningTree<SimpleGraph, i32> {
    MinimumCapacitatedSpanningTree::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 2), (1, 3), (2, 3)]),
        vec![2, 3, 1, 1, 2], // edge weights
        0,                   // root
        vec![0, 1, 1, 1],    // requirements
        2,                   // capacity
    )
}

/// Canonical instance from issue #901: 5 vertices, 8 edges.
fn canonical_instance() -> MinimumCapacitatedSpanningTree<SimpleGraph, i32> {
    MinimumCapacitatedSpanningTree::new(
        SimpleGraph::new(
            5,
            vec![
                (0, 1),
                (0, 2),
                (0, 3),
                (1, 2),
                (1, 4),
                (2, 3),
                (2, 4),
                (3, 4),
            ],
        ),
        vec![2, 1, 4, 3, 1, 2, 3, 1],
        0,
        vec![0, 1, 1, 1, 1],
        3,
    )
}

#[test]
fn test_reduction_creates_expected_ilp_shape() {
    let problem = small_instance();
    let reduction: ReductionMinimumCapacitatedSpanningTreeToILP =
        ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // m=5: num_vars = 3*5 = 15
    assert_eq!(ilp.num_vars, 15);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
}

#[test]
fn test_minimumcapacitatedspanningtree_to_ilp_closed_loop() {
    let problem = small_instance();
    let reduction: ReductionMinimumCapacitatedSpanningTreeToILP =
        ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();
    let best_source = bf.find_all_witnesses(&problem);
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    let bf_value = problem.evaluate(&best_source[0]);
    let ilp_value = problem.evaluate(&extracted);
    assert_eq!(ilp_value, bf_value);
    assert!(problem.is_valid_solution(&extracted));
}

#[test]
fn test_minimumcapacitatedspanningtree_to_ilp_canonical_closed_loop() {
    let problem = canonical_instance();
    let reduction: ReductionMinimumCapacitatedSpanningTreeToILP =
        ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();
    let best_source = bf.find_all_witnesses(&problem);
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert_eq!(problem.evaluate(&best_source[0]), Min(Some(5)));
    assert_eq!(problem.evaluate(&extracted), Min(Some(5)));
    assert!(problem.is_valid_solution(&extracted));
}

#[test]
fn test_solution_extraction_reads_edge_selector_prefix() {
    let problem = small_instance();
    let reduction: ReductionMinimumCapacitatedSpanningTreeToILP =
        ReduceTo::<ILP<i32>>::reduce_to(&problem);

    // 15 variables total, first 5 are edge selectors
    let mut target_solution = vec![0; 15];
    target_solution[0] = 1; // edge (0,1)
    target_solution[1] = 1; // edge (0,2)
    target_solution[3] = 1; // edge (1,3)

    assert_eq!(
        reduction.extract_solution(&target_solution),
        vec![1, 1, 0, 1, 0]
    );
}

#[test]
fn test_minimumcapacitatedspanningtree_to_ilp_bf_vs_ilp() {
    let problem = canonical_instance();
    let reduction: ReductionMinimumCapacitatedSpanningTreeToILP =
        ReduceTo::<ILP<i32>>::reduce_to(&problem);
    crate::rules::test_helpers::assert_bf_vs_ilp(&problem, &reduction);
}

#[test]
fn test_minimumcapacitatedspanningtree_to_ilp_star_tree() {
    // Star from root 0: all edges directly from root.
    // With capacity >= max single requirement, star is always valid.
    let problem = MinimumCapacitatedSpanningTree::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3)]),
        vec![1, 1, 1],
        0,
        vec![0, 1, 1, 1],
        1, // capacity = 1 forces star tree
    );
    let reduction: ReductionMinimumCapacitatedSpanningTreeToILP =
        ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    let ilp_solution = ILPSolver::new().solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), Min(Some(3)));
    assert!(problem.is_valid_solution(&extracted));
}

#[test]
fn test_minimumcapacitatedspanningtree_to_ilp_path_graph() {
    // Path 0-1-2-3, root=0, capacity=3, requirements=[0,1,1,1]
    // Only spanning tree is the path: subtree(1)={1,2,3}->req=3<=3 OK
    let problem = MinimumCapacitatedSpanningTree::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![2, 3, 1],
        0,
        vec![0, 1, 1, 1],
        3,
    );
    let reduction: ReductionMinimumCapacitatedSpanningTreeToILP =
        ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    let ilp_solution = ILPSolver::new().solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), Min(Some(6)));
    assert!(problem.is_valid_solution(&extracted));
}
