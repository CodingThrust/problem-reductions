use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::models::misc::SchedulingToMinimizeWeightedCompletionTime;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_reduction_creates_valid_ilp_structure() {
    // 3 tasks, 2 processors
    let problem = SchedulingToMinimizeWeightedCompletionTime::new(vec![1, 2, 3], vec![4, 2, 1], 2);
    let reduction: ReductionSMWCTToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // n=3, m=2: x vars = 3*2=6, C vars = 3, y vars = 3*2/2=3, total=12
    assert_eq!(ilp.num_vars, 12);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);

    // Objective should reference C_t variables with weights
    // C vars are at indices 6, 7, 8
    assert!(ilp
        .objective
        .iter()
        .any(|&(idx, coeff)| idx == 6 && coeff == 4.0));
    assert!(ilp
        .objective
        .iter()
        .any(|&(idx, coeff)| idx == 7 && coeff == 2.0));
    assert!(ilp
        .objective
        .iter()
        .any(|&(idx, coeff)| idx == 8 && coeff == 1.0));
}

#[test]
fn test_solution_extraction() {
    let problem = SchedulingToMinimizeWeightedCompletionTime::new(vec![1, 2], vec![3, 1], 2);
    let reduction: ReductionSMWCTToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);

    // Build a manual ILP solution:
    // x_{0,0}=1, x_{0,1}=0, x_{1,0}=0, x_{1,1}=1 => task 0 on P0, task 1 on P1
    // C_0=1, C_1=2, y_{0,1}=1
    let num_vars = reduction.target_problem().num_vars;
    let mut sol = vec![0; num_vars];
    // x vars: indices 0..4
    sol[0] = 1; // x_{0,0} = 1
    sol[1] = 0; // x_{0,1} = 0
    sol[2] = 0; // x_{1,0} = 0
    sol[3] = 1; // x_{1,1} = 1
                // C vars: indices 4, 5
    sol[4] = 1; // C_0 = 1
    sol[5] = 2; // C_1 = 2
                // y vars: index 6
    sol[6] = 1; // y_{0,1} = 1

    let extracted = reduction.extract_solution(&sol);
    assert_eq!(extracted, vec![0, 1]);
    // Each on separate processor: C(0)=1, C(1)=2, WCT = 1*3 + 2*1 = 5
    assert_eq!(problem.evaluate(&extracted), Min(Some(5)));
}

#[test]
fn test_ilp_matches_bruteforce_small() {
    // 3 tasks, 2 processors
    let problem = SchedulingToMinimizeWeightedCompletionTime::new(vec![1, 2, 3], vec![4, 2, 1], 2);

    let bf = BruteForce::new();
    let bf_witness = bf
        .find_witness(&problem)
        .expect("BF should find a solution");
    let bf_value = problem.evaluate(&bf_witness);

    let reduction: ReductionSMWCTToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    let ilp_solution = ILPSolver::new().solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_value = problem.evaluate(&extracted);

    assert_eq!(ilp_value, bf_value);
}

#[test]
fn test_issue_example_closed_loop() {
    // Issue #505 example: 5 tasks, 2 processors, optimal = 47
    let problem = SchedulingToMinimizeWeightedCompletionTime::new(
        vec![1, 2, 3, 4, 5],
        vec![6, 4, 3, 2, 1],
        2,
    );

    let reduction: ReductionSMWCTToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    let ilp_solution = ILPSolver::new().solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert_eq!(problem.evaluate(&extracted), Min(Some(47)));
}

#[test]
fn test_single_task_single_processor() {
    let problem = SchedulingToMinimizeWeightedCompletionTime::new(vec![5], vec![3], 1);
    let reduction: ReductionSMWCTToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let ilp_solution = ILPSolver::new().solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), Min(Some(15)));
}

#[test]
fn test_equal_tasks_multiple_processors() {
    // 4 equal tasks, 2 processors
    let problem =
        SchedulingToMinimizeWeightedCompletionTime::new(vec![1, 1, 1, 1], vec![1, 1, 1, 1], 2);

    let bf = BruteForce::new();
    let bf_witness = bf
        .find_witness(&problem)
        .expect("BF should find a solution");
    let bf_value = problem.evaluate(&bf_witness);

    let reduction: ReductionSMWCTToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    let ilp_solution = ILPSolver::new().solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_value = problem.evaluate(&extracted);

    assert_eq!(ilp_value, bf_value);
}
