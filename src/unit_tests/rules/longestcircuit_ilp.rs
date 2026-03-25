use super::*;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;

#[test]
fn test_reduction_creates_valid_ilp() {
    // Triangle with unit lengths
    let problem = LongestCircuit::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![1, 1, 1],
    );
    let reduction: ReductionLongestCircuitToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    // m=3, n=3, commodities=2, flow=2*3*2=12, total=3+3+12=18
    assert_eq!(ilp.num_vars, 18);
    assert_eq!(ilp.sense, ObjectiveSense::Maximize);
}

#[test]
fn test_longestcircuit_to_ilp_closed_loop() {
    // Hexagon with varying edge lengths
    let problem = LongestCircuit::new(
        SimpleGraph::new(
            6,
            vec![
                (0, 1),
                (1, 2),
                (2, 3),
                (3, 4),
                (4, 5),
                (5, 0),
                (0, 3),
                (1, 4),
                (2, 5),
                (3, 5),
            ],
        ),
        vec![3, 2, 4, 1, 5, 2, 3, 2, 1, 2],
    );
    // BruteForce on source to verify feasibility
    let bf = BruteForce::new();
    let bf_solution = bf
        .find_witness(&problem)
        .expect("brute-force should find a solution");
    assert!(problem.evaluate(&bf_solution).0.is_some());

    // Solve via ILP
    let reduction: ReductionLongestCircuitToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert!(
        problem.evaluate(&extracted).0.is_some(),
        "ILP solution should be a valid circuit"
    );
}

#[test]
fn test_longestcircuit_to_ilp_triangle() {
    // Triangle: all edges length 1
    let problem = LongestCircuit::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![1, 1, 1],
    );
    let reduction: ReductionLongestCircuitToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    assert_optimization_round_trip_from_optimization_target(
        &problem,
        &reduction,
        "LongestCircuit->ILP triangle",
    );
}

#[test]
fn test_solution_extraction() {
    let problem = LongestCircuit::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (3, 0), (0, 2), (1, 3)]),
        vec![1, 1, 1, 1, 2, 2],
    );
    let reduction: ReductionLongestCircuitToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert!(problem.evaluate(&extracted).0.is_some());
}
