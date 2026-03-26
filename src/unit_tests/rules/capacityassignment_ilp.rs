use super::*;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_reduction_creates_valid_ilp() {
    // 2 links, 2 capacity levels
    let problem = CapacityAssignment::new(
        vec![1, 2],
        vec![vec![1, 3], vec![2, 4]],
        vec![vec![8, 4], vec![7, 3]],
        12,
    );
    let reduction: ReductionCAToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // num_vars = 2 links * 2 capacities = 4
    assert_eq!(
        ilp.num_vars, 4,
        "Should have 4 variables (2 links * 2 capacities)"
    );

    // num_constraints = 2 assignment + 1 delay budget = 3
    assert_eq!(
        ilp.constraints.len(),
        3,
        "Should have 3 constraints (2 assignment + 1 delay)"
    );
    assert_eq!(ilp.sense, ObjectiveSense::Minimize, "Should minimize cost");
    // Objective should have cost coefficients
    assert!(
        !ilp.objective.is_empty(),
        "Objective should have cost terms"
    );
}

#[test]
fn test_capacityassignment_to_ilp_closed_loop() {
    // 3 links, 3 capacity levels
    let problem = CapacityAssignment::new(
        vec![1, 2, 3],
        vec![vec![1, 3, 6], vec![2, 4, 7], vec![1, 2, 5]],
        vec![vec![8, 4, 1], vec![7, 3, 1], vec![6, 3, 1]],
        12,
    );

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    let bf_witness = bf
        .find_witness(&problem)
        .expect("BF should find a solution");
    let bf_value = problem.evaluate(&bf_witness);
    assert_eq!(bf_value, Min(Some(9)));

    let reduction: ReductionCAToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_value = problem.evaluate(&extracted);
    assert_eq!(
        ilp_value, bf_value,
        "ILP and BF should agree on optimal value"
    );
}

#[test]
fn test_solution_extraction() {
    // 2 links, 3 capacity levels
    let problem = CapacityAssignment::new(
        vec![1, 2, 3],
        vec![vec![1, 3, 6], vec![2, 4, 7]],
        vec![vec![8, 4, 1], vec![7, 3, 1]],
        10,
    );
    let reduction: ReductionCAToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    // link 0 → cap 1, link 1 → cap 0
    // x_{0,0}=0, x_{0,1}=1, x_{0,2}=0, x_{1,0}=1, x_{1,1}=0, x_{1,2}=0
    let ilp_solution = vec![0, 1, 0, 1, 0, 0];
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted, vec![1, 0]);
    // Verify extraction works (evaluation may or may not be feasible)
    let _ = problem.evaluate(&extracted);
}

#[test]
fn test_capacityassignment_to_ilp_trivial() {
    // 1 link, 1 capacity level — trivially feasible
    let problem = CapacityAssignment::new(vec![1], vec![vec![0]], vec![vec![0]], 100);
    let reduction: ReductionCAToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // num_vars = 1, num_constraints = 1 + 1 = 2
    assert_eq!(ilp.num_vars, 1);
    assert_eq!(ilp.constraints.len(), 2);

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert!(problem.evaluate(&extracted).0.is_some());
}

#[test]
fn test_capacityassignment_to_ilp_bf_vs_ilp() {
    let problem = CapacityAssignment::new(
        vec![1, 2, 3],
        vec![vec![1, 3, 6], vec![2, 4, 7], vec![1, 2, 5]],
        vec![vec![8, 4, 1], vec![7, 3, 1], vec![6, 3, 1]],
        12,
    );
    let reduction: ReductionCAToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    crate::rules::test_helpers::assert_bf_vs_ilp(&problem, &reduction);
}
