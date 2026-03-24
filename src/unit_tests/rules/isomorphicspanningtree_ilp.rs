use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Or;

#[test]
fn test_reduction_creates_valid_ilp() {
    // K3, path tree
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let tree = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = IsomorphicSpanningTree::new(graph, tree);
    let reduction: ReductionISTToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars(), 9); // 3x3
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
    assert!(ilp.objective.is_empty());
}

#[test]
fn test_isomorphicspanningtree_to_ilp_closed_loop() {
    let graph = SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
    let tree = SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3)]);
    let problem = IsomorphicSpanningTree::new(graph, tree);
    let reduction: ReductionISTToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &problem,
        &reduction,
        "IsomorphicSpanningTree->ILP closed loop",
    );
}

#[test]
fn test_isomorphicspanningtree_to_ilp_bf_vs_ilp() {
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (0, 3)]);
    let tree = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let problem = IsomorphicSpanningTree::new(graph, tree);

    let bf = BruteForce::new();
    let bf_witness = bf.find_witness(&problem);
    assert!(
        bf_witness.is_some(),
        "BF should find a satisfying assignment"
    );

    let reduction: ReductionISTToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), Or(true));
}

#[test]
fn test_solution_extraction() {
    // K3 with path tree
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let tree = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = IsomorphicSpanningTree::new(graph, tree);
    let reduction: ReductionISTToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted.len(), 3);
    assert_eq!(problem.evaluate(&extracted), Or(true));
}
