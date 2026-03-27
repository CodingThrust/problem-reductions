use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;

fn feasible_instance() -> SchedulingWithIndividualDeadlines {
    // 3 tasks, 2 processors, individual deadlines [2, 2, 3], precedence: 0→2
    SchedulingWithIndividualDeadlines::new(3, 2, vec![2, 2, 3], vec![(0, 2)])
}

fn infeasible_instance() -> SchedulingWithIndividualDeadlines {
    // 3 tasks, 1 processor, deadlines [1, 1, 1] → only 1 slot, can't fit 3 tasks
    SchedulingWithIndividualDeadlines::new(3, 1, vec![1, 1, 1], vec![])
}

#[test]
fn test_schedulingwithindividualdeadlines_to_ilp_structure() {
    let problem = feasible_instance();
    let reduction: ReductionSWIDToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // n=3, max_deadline=3 → 9 variables
    assert_eq!(ilp.num_vars, 9);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
    assert!(ilp.objective.is_empty());

    // 3 one-hot + 3 capacity + 1 precedence = 7 constraints
    assert_eq!(ilp.constraints.len(), 7);
}

#[test]
fn test_schedulingwithindividualdeadlines_to_ilp_closed_loop() {
    let problem = feasible_instance();
    let bf = BruteForce::new();
    let bf_solution = bf
        .find_witness(&problem)
        .expect("feasible instance has a witness");
    assert!(
        problem.evaluate(&bf_solution).0,
        "brute force solution is valid"
    );

    let reduction: ReductionSWIDToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert!(
        problem.evaluate(&extracted).0,
        "ILP extracted solution should be a valid schedule"
    );
}

#[test]
fn test_schedulingwithindividualdeadlines_to_ilp_infeasible() {
    let problem = infeasible_instance();
    let reduction: ReductionSWIDToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    assert!(
        ILPSolver::new().solve(reduction.target_problem()).is_none(),
        "infeasible instance should yield infeasible ILP"
    );
}

#[test]
fn test_schedulingwithindividualdeadlines_to_ilp_extract_solution() {
    let problem = feasible_instance();
    let reduction: ReductionSWIDToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    // task 0 at slot 0, task 1 at slot 0, task 2 at slot 1
    // max_deadline=3: x_{j,t} at j*3+t
    // x_{0,0}=1, x_{0,1}=0, x_{0,2}=0, x_{1,0}=1, x_{1,1}=0, x_{1,2}=0, x_{2,0}=0, x_{2,1}=1, x_{2,2}=0
    let ilp_solution = vec![1, 0, 0, 1, 0, 0, 0, 1, 0];
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted, vec![0, 0, 1]);
    assert!(
        problem.evaluate(&extracted).0,
        "manually constructed solution is valid"
    );
}
