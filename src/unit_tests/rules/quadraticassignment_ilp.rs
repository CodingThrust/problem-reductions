use super::*;
use crate::solvers::{BruteForce, ILPSolver, Solver};
use crate::traits::Problem;

fn small_qap() -> QuadraticAssignment {
    QuadraticAssignment::new(
        vec![vec![0, 5, 2], vec![5, 0, 3], vec![2, 3, 0]],
        vec![vec![0, 4, 1], vec![4, 0, 3], vec![1, 3, 0]],
    )
}

#[test]
fn test_reduction_creates_valid_ilp() {
    let problem = small_qap();
    let reduction: ReductionQAPToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    // n=3, m=3: num_x=9, z pairs: 3*2*3*3=54, total=63
    assert_eq!(ilp.num_vars, 63);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
}

#[test]
fn test_quadraticassignment_to_ilp_closed_loop() {
    let problem = small_qap();
    // BruteForce on source to get optimal value
    let bf = BruteForce::new();
    let bf_solution = bf.find_witness(&problem).expect("brute-force optimum");
    let bf_value = problem.evaluate(&bf_solution);

    // Solve via ILP
    let reduction: ReductionQAPToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_value = problem.evaluate(&extracted);

    assert!(
        ilp_value.is_valid(),
        "Extracted solution should be a valid assignment"
    );
    assert_eq!(
        ilp_value, bf_value,
        "ILP and brute-force should agree on optimal value"
    );
}

#[test]
fn test_quadraticassignment_to_ilp_2x2() {
    let problem =
        QuadraticAssignment::new(vec![vec![0, 1], vec![1, 0]], vec![vec![0, 2], vec![2, 0]]);
    // BruteForce on source
    let bf = BruteForce::new();
    let bf_solution = bf.find_witness(&problem).expect("brute-force optimum");
    let bf_value = problem.evaluate(&bf_solution);

    // Solve via ILP
    let reduction: ReductionQAPToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
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
    let problem = small_qap();
    let reduction: ReductionQAPToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let metric = problem.evaluate(&extracted);
    assert!(metric.is_valid());
}

#[test]
fn test_quadraticassignment_to_ilp_rectangular() {
    // 2 facilities, 3 locations (more locations than facilities)
    let problem = QuadraticAssignment::new(
        vec![vec![0, 3], vec![3, 0]],
        vec![vec![0, 1, 5], vec![1, 0, 2], vec![5, 2, 0]],
    );
    // BruteForce on source
    let bf = BruteForce::new();
    let bf_solution = bf.find_witness(&problem).expect("brute-force optimum");
    let bf_value = problem.evaluate(&bf_solution);

    // Solve via ILP
    let reduction: ReductionQAPToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
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
fn test_quadraticassignment_to_ilp_bf_vs_ilp() {
    let problem = small_qap();
    let reduction: ReductionQAPToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let bf_value = BruteForce::new().solve(&problem);
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), bf_value);
}
