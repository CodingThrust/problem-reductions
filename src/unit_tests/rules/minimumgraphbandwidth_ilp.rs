use super::*;
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;

#[test]
fn test_reduction_creates_valid_ilp() {
    // Star S4: 4 vertices, 3 edges
    let problem = MinimumGraphBandwidth::new(SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3)]));
    let reduction: ReductionMGBToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    // num_x=16, pos_v=4, B=1, total=21
    assert_eq!(ilp.num_vars, 21);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
}

#[test]
fn test_minimumgraphbandwidth_to_ilp_closed_loop() {
    // Star S4
    let problem = MinimumGraphBandwidth::new(SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3)]));

    // BruteForce on source to verify feasibility
    let bf = BruteForce::new();
    let bf_solution = bf
        .find_witness(&problem)
        .expect("brute-force should find a solution");
    assert!(problem.evaluate(&bf_solution).0.is_some());

    // Solve via ILP
    let reduction: ReductionMGBToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_value = problem.evaluate(&extracted);
    assert!(
        ilp_value.0.is_some(),
        "ILP solution should produce a valid arrangement"
    );

    // BF and ILP should agree on optimal value
    let bf_value = problem.evaluate(&bf_solution);
    assert_eq!(
        ilp_value, bf_value,
        "ILP and BF should find same optimal bandwidth"
    );
}

#[test]
fn test_minimumgraphbandwidth_to_ilp_path() {
    // Path P4: 0-1-2-3 (optimal bandwidth = 1)
    let problem = MinimumGraphBandwidth::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]));

    let reduction: ReductionMGBToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let value = problem.evaluate(&extracted);
    assert_eq!(
        value,
        crate::types::Min(Some(1)),
        "path P4 optimal bandwidth = 1"
    );
}

#[test]
fn test_minimumgraphbandwidth_to_ilp_bf_vs_ilp() {
    // Star S4
    let problem = MinimumGraphBandwidth::new(SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3)]));
    let reduction: ReductionMGBToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    crate::rules::test_helpers::assert_bf_vs_ilp(&problem, &reduction);
}

#[test]
fn test_minimumgraphbandwidth_to_ilp_cycle() {
    // Cycle C4: 0-1-2-3-0 (optimal bandwidth = 2)
    let problem =
        MinimumGraphBandwidth::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (3, 0)]));
    let reduction: ReductionMGBToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    crate::rules::test_helpers::assert_bf_vs_ilp(&problem, &reduction);
}
