use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::DirectedGraph;
use crate::traits::Problem;
use crate::types::Min;

fn issue_instance() -> MinimumEdgeCostFlow {
    MinimumEdgeCostFlow::new(
        DirectedGraph::new(5, vec![(0, 1), (0, 2), (0, 3), (1, 4), (2, 4), (3, 4)]),
        vec![3, 1, 2, 0, 0, 0],
        vec![2, 2, 2, 2, 2, 2],
        0,
        4,
        3,
    )
}

fn small_instance() -> MinimumEdgeCostFlow {
    // 3-vertex: s=0, t=2, two parallel paths via v1
    // Arc 0: (0,1) cap=2, price=5
    // Arc 1: (1,2) cap=2, price=3
    // R=1 → cost = 5+3 = 8
    MinimumEdgeCostFlow::new(
        DirectedGraph::new(3, vec![(0, 1), (1, 2)]),
        vec![5, 3],
        vec![2, 2],
        0,
        2,
        1,
    )
}

fn infeasible_instance() -> MinimumEdgeCostFlow {
    // Cannot route 2 units through capacity-1 arcs
    MinimumEdgeCostFlow::new(
        DirectedGraph::new(3, vec![(0, 1), (1, 2)]),
        vec![1, 1],
        vec![1, 1],
        0,
        2,
        2,
    )
}

#[test]
fn test_minimumedgecostflow_to_ilp_structure() {
    let problem = issue_instance();
    let reduction: ReductionMECFToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    // 6 arcs → 2*6 = 12 variables
    assert_eq!(ilp.num_vars(), 12);
    assert_eq!(ilp.sense(), ObjectiveSense::Minimize);

    // Zero-priced arcs are omitted by sparse objective normalization.
    assert_eq!(ilp.objective(), vec![(6, 3), (7, 1), (8, 2)]);

    // Constraints: 6 linking + 6 binary + (5-2)=3 conservation + 1 flow req = 16
    // That is 2*6 + 5 - 1 = 16
    assert_eq!(ilp.constraints().len(), 16);
}

#[test]
fn test_minimumedgecostflow_to_ilp_closed_loop() {
    let problem = issue_instance();
    let bf = BruteForce::new();
    let bf_witness = bf
        .solve(&problem)
        .unwrap()
        .expect("issue instance has optimal");
    let bf_value = problem.evaluate(&bf_witness).unwrap();
    assert_eq!(bf_value, Min(Some(3)));

    let reduction: ReductionMECFToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();

    let ilp_value = problem.evaluate(&extracted).unwrap();
    assert_eq!(ilp_value, bf_value);
}

#[test]
fn test_minimumedgecostflow_to_ilp_small_closed_loop() {
    let problem = small_instance();
    let bf = BruteForce::new();
    let bf_witness = bf
        .solve(&problem)
        .unwrap()
        .expect("small instance has optimal");
    let bf_value = problem.evaluate(&bf_witness).unwrap();
    assert_eq!(bf_value, Min(Some(8)));

    let reduction: ReductionMECFToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    assert_eq!(problem.evaluate(&extracted).unwrap(), bf_value);
}

#[test]
fn test_minimumedgecostflow_to_ilp_infeasible() {
    let problem = infeasible_instance();
    let reduction: ReductionMECFToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");
    assert!(
        ILPSolver::new().solve(reduction.target_problem()).is_err(),
        "infeasible instance should produce infeasible ILP"
    );
}

#[test]
fn test_minimumedgecostflow_to_ilp_bf_vs_ilp() {
    let problem = issue_instance();
    let reduction: ReductionMECFToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");
    crate::rules::test_helpers::assert_bf_vs_ilp(&problem, &reduction);
}

#[test]
fn test_minimumedgecostflow_to_ilp_extract_solution() {
    let problem = issue_instance();
    let reduction: ReductionMECFToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");

    // Manually construct a target solution: route 1 via v2, 2 via v3
    // f = [0, 1, 2, 0, 1, 2], y = [0, 1, 1, 0, 1, 1]
    let mut target_solution = vec![0_i64; 12];
    target_solution[1] = 1; // f on arc (0,2)
    target_solution[2] = 2; // f on arc (0,3)
    target_solution[4] = 1; // f on arc (2,4)
    target_solution[5] = 2; // f on arc (3,4)
    target_solution[7] = 1; // y on arc (0,2)
    target_solution[8] = 1; // y on arc (0,3)
    target_solution[10] = 1; // y on arc (2,4)
    target_solution[11] = 1; // y on arc (3,4)

    let extracted = reduction.extract_solution(&target_solution).unwrap();
    assert_eq!(extracted.len(), 6);
    assert_eq!(extracted, vec![0, 1, 2, 0, 1, 2]);
    assert_eq!(problem.evaluate(&extracted).unwrap(), Min(Some(3)));
}
