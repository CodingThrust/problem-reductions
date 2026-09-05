use super::*;
use crate::models::algebraic::ILP;
use crate::rules::test_helpers::assert_bf_vs_ilp;
use crate::rules::ReduceTo;
use crate::solvers::ILPSolver;
use crate::topology::SimpleGraph;
use crate::traits::Problem;

#[test]
fn test_disjointconnectingpaths_to_ilp_closed_loop() {
    // 6 vertices, two vertex-disjoint paths available:
    // Path (0,2): 0 - 1 - 2 (interior vertex 1, not a terminal)
    // Path (3,5): 3 - 4 - 5 (interior vertex 4, not a terminal)
    let source = DisjointConnectingPaths::new(
        SimpleGraph::new(6, vec![(0, 1), (1, 2), (3, 4), (4, 5)]),
        vec![(0, 2), (3, 5)],
    );
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).expect("reduction should succeed");
    assert_bf_vs_ilp(&source, &reduction);
}

#[test]
fn test_disjointconnectingpaths_to_ilp_bf_vs_ilp() {
    let source = DisjointConnectingPaths::new(
        SimpleGraph::new(6, vec![(0, 1), (1, 2), (3, 4), (4, 5)]),
        vec![(0, 2), (3, 5)],
    );
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).expect("reduction should succeed");
    crate::rules::test_helpers::assert_bf_vs_ilp(&source, &reduction);
}

#[test]
fn test_disjointconnectingpaths_to_ilp_forbids_using_another_pairs_terminal() {
    let source = DisjointConnectingPaths::new(
        SimpleGraph::new(5, vec![(0, 1), (0, 2), (0, 3), (2, 4), (3, 4)]),
        vec![(0, 1), (2, 3)],
    );
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
    let mut colliding_flow = vec![0; 20];
    for index in [0, 13, 14] {
        colliding_flow[index] = 1;
    }
    assert!(!reduction
        .target_problem()
        .is_feasible(&colliding_flow)
        .unwrap());
    let target_solution = ILPSolver::new().solve(reduction.target_problem()).unwrap();
    let extracted = reduction.extract_solution(&target_solution).unwrap();
    assert!(source.evaluate(&extracted).unwrap().0);
}

#[test]
fn test_disjointconnectingpaths_to_ilp_discards_disconnected_circulation() {
    let source = DisjointConnectingPaths::new(
        SimpleGraph::new(7, vec![(0, 1), (2, 3), (4, 5), (4, 6), (5, 6)]),
        vec![(0, 1), (2, 3)],
    );
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
    let mut target_solution = vec![0; 20];
    for index in [0, 4, 7, 8, 12] {
        target_solution[index] = 1;
    }
    assert!(reduction
        .target_problem()
        .is_feasible(&target_solution)
        .unwrap());
    let extracted = reduction.extract_solution(&target_solution).unwrap();
    assert_eq!(extracted, vec![true, true, false, false, false]);
    assert!(source.evaluate(&extracted).unwrap().0);
}
