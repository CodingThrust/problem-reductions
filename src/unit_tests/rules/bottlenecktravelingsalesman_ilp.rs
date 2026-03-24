use super::*;
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;

fn k4_btsp() -> BottleneckTravelingSalesman {
    BottleneckTravelingSalesman::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]),
        vec![1, 3, 2, 4, 2, 1],
    )
}

#[test]
fn test_reduction_creates_valid_ilp() {
    let problem = k4_btsp();
    let reduction: ReductionBTSPToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    // n=4, m=6: num_x=16, num_z=2*6*4=48, b=1, total=65
    assert_eq!(ilp.num_vars, 65);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
}

#[test]
fn test_bottlenecktravelingsalesman_to_ilp_closed_loop() {
    let problem = k4_btsp();
    let bf = BruteForce::new();
    let bf_solution = bf.find_witness(&problem).expect("brute-force optimum");
    let bf_value = problem.evaluate(&bf_solution);

    let reduction: ReductionBTSPToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_value = problem.evaluate(&extracted);

    assert!(
        ilp_value.is_valid(),
        "Extracted solution should be a valid Hamiltonian cycle"
    );
    assert_eq!(
        ilp_value, bf_value,
        "ILP and brute-force should agree on optimal value"
    );
}

#[test]
fn test_bottlenecktravelingsalesman_to_ilp_c4() {
    // C4 with varying weights: bottleneck = max weight in the only cycle
    let problem = BottleneckTravelingSalesman::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (3, 0)]),
        vec![1, 2, 3, 4],
    );
    let bf = BruteForce::new();
    let bf_solution = bf.find_witness(&problem).expect("brute-force optimum");
    let bf_value = problem.evaluate(&bf_solution);

    let reduction: ReductionBTSPToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_value = problem.evaluate(&extracted);

    assert!(ilp_value.is_valid());
    assert_eq!(ilp_value, bf_value);
}

#[test]
fn test_solution_extraction() {
    let problem = k4_btsp();
    let reduction: ReductionBTSPToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let metric = problem.evaluate(&extracted);
    assert!(metric.is_valid());
}

#[test]
fn test_no_hamiltonian_cycle_infeasible() {
    // Path graph: no Hamiltonian cycle
    let problem = BottleneckTravelingSalesman::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![1, 1, 1],
    );
    let reduction: ReductionBTSPToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let result = ilp_solver.solve(reduction.target_problem());
    assert!(
        result.is_none(),
        "Path graph should have no Hamiltonian cycle"
    );
}
