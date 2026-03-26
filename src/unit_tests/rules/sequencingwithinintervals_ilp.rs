use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::solvers::{BruteForce, ILPSolver, Solver};
use crate::traits::Problem;

fn feasible_instance() -> SequencingWithinIntervals {
    // 2 tasks: task 0 [r=0, d=3, l=2] (slots: 0 only), task 1 [r=2, d=5, l=2] (slots: 0,1)
    // Non-overlapping: task 0 at [0,2), task 1 at [2,4) or [3,5) — feasible
    SequencingWithinIntervals::new(vec![0, 2], vec![3, 5], vec![2, 2])
}

fn infeasible_instance() -> SequencingWithinIntervals {
    // 2 tasks that must overlap: both occupy [0,2) interval but can only start at offset 0
    // task 0 [r=0, d=2, l=2]: only start at offset 0
    // task 1 [r=0, d=2, l=2]: only start at offset 0
    // Both start at 0 → overlap
    SequencingWithinIntervals::new(vec![0, 0], vec![2, 2], vec![2, 2])
}

#[test]
fn test_sequencingwithinintervals_to_ilp_structure() {
    let problem = feasible_instance();
    let reduction: ReductionSWIToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // task 0 has 1 start slot (d-r-l+1=3-0-2+1=2, so 2 offsets: 0 or 1... wait)
    // dim for task 0: d[0]-r[0]-l[0]+1 = 3-0-2+1 = 2 offsets
    // dim for task 1: d[1]-r[1]-l[1]+1 = 5-2-2+1 = 2 offsets
    // total vars = 2 + 2 = 4
    assert_eq!(ilp.num_vars, 4);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
    assert!(ilp.objective.is_empty());

    // 2 one-hot constraints + overlap constraints
    // task 0 k1=0: [0,2), task 1 k2=0: [2,4) → no overlap
    // task 0 k1=0: [0,2), task 1 k2=1: [3,5) → no overlap
    // task 0 k1=1: [1,3), task 1 k2=0: [2,4) → overlap at [2,3)
    // task 0 k1=1: [1,3), task 1 k2=1: [3,5) → no overlap
    // So 1 non-overlap constraint + 2 one-hot = 3 total
    assert_eq!(ilp.constraints.len(), 3);
}

#[test]
fn test_sequencingwithinintervals_to_ilp_closed_loop() {
    let problem = feasible_instance();
    let bf = BruteForce::new();
    let bf_solution = bf
        .find_witness(&problem)
        .expect("feasible instance has a witness");
    assert!(
        problem.evaluate(&bf_solution).0,
        "brute force solution is valid"
    );

    let reduction: ReductionSWIToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
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
fn test_sequencingwithinintervals_to_ilp_infeasible() {
    let problem = infeasible_instance();
    let reduction: ReductionSWIToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    assert!(
        ILPSolver::new().solve(reduction.target_problem()).is_none(),
        "infeasible instance (forced overlap) should yield infeasible ILP"
    );
}

#[test]
fn test_sequencingwithinintervals_to_ilp_extract_solution() {
    let problem = feasible_instance();
    let reduction: ReductionSWIToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    // task 0 at offset 0, task 1 at offset 0
    // vars: x_{0,0}=1, x_{0,1}=0, x_{1,0}=1, x_{1,1}=0
    let ilp_solution = vec![1, 0, 1, 0];
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted, vec![0, 0]);
    assert!(
        problem.evaluate(&extracted).0,
        "manually constructed solution is valid"
    );
}

#[test]
fn test_sequencingwithinintervals_to_ilp_bf_vs_ilp() {
    let problem = feasible_instance();
    let reduction: ReductionSWIToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let bf_value = BruteForce::new().solve(&problem);
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), bf_value);
}
