use super::*;
use crate::models::algebraic::ILP;
use crate::models::misc::OpenShopScheduling;
use crate::solvers::ILPSolver;
use crate::traits::Problem;
use crate::types::Min;

/// 2 machines, 2 jobs: smallest non-trivial instance.
/// processing_times[j][i]: J1=[1,2], J2=[2,1].  Optimal makespan = 3.
fn small_instance() -> OpenShopScheduling {
    OpenShopScheduling::new(2, vec![vec![1, 2], vec![2, 1]])
}

/// 3 machines, 2 jobs.
fn medium_instance() -> OpenShopScheduling {
    OpenShopScheduling::new(3, vec![vec![3, 1, 2], vec![2, 3, 1]])
}

// ─── structure ───────────────────────────────────────────────────────────────

#[test]
fn test_openshopscheduling_to_ilp_structure_small() {
    let p = small_instance();
    let reduction: ReductionOSSToILP = ReduceTo::<ILP<i32>>::reduce_to(&p);
    let ilp = reduction.target_problem();

    // n=2, m=2:
    // num_pairs = 1, num_order_vars = 1*2 = 2 (x_{0,1,0}, x_{0,1,1})
    // num_start_vars = 2*2 = 4 (s_{0,0}, s_{0,1}, s_{1,0}, s_{1,1})
    // num_machine_pairs = 1, num_job_pair_vars = 2*1 = 2 (y_{0,0,1}, y_{1,0,1})
    // c_var = 1
    // Total = 2 + 4 + 2 + 1 = 9
    assert_eq!(
        ilp.num_vars, 9,
        "expected 9 variables, got {}",
        ilp.num_vars
    );
    // Constraint count: 2 bound_x + 4 s_upper + 1 c_upper + 4 machine_nooverlap
    //                 + 2 bound_y + 4 job_nooverlap + 4 makespan = 21
    assert_eq!(
        ilp.constraints.len(),
        21,
        "expected 21 constraints, got {}",
        ilp.constraints.len()
    );
    assert_eq!(
        ilp.objective,
        vec![(8, 1.0)],
        "objective should minimize C (index 8)"
    );
}

// ─── closed-loop ─────────────────────────────────────────────────────────────

#[test]
fn test_openshopscheduling_to_ilp_closed_loop_small() {
    let p = small_instance();
    let reduction: ReductionOSSToILP = ReduceTo::<ILP<i32>>::reduce_to(&p);
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be feasible");

    let extracted = reduction.extract_solution(&ilp_solution);
    let value = p.evaluate(&extracted);
    assert!(
        value.0.is_some(),
        "extracted schedule must be valid, got {value:?}"
    );
    // Optimal makespan = 3
    assert_eq!(value, Min(Some(3)), "ILP should find optimal makespan = 3");
}

#[test]
fn test_openshopscheduling_to_ilp_closed_loop_medium() {
    let p = medium_instance();
    let reduction: ReductionOSSToILP = ReduceTo::<ILP<i32>>::reduce_to(&p);
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be feasible");

    let extracted = reduction.extract_solution(&ilp_solution);
    let value = p.evaluate(&extracted);
    assert!(
        value.0.is_some(),
        "extracted schedule must be valid, got {value:?}"
    );
    // Max machine total = max(5, 4, 3) = 5; max job total = max(6, 6) = 6
    // Lower bound = 6.
    let makespan = value.0.unwrap();
    assert!(makespan >= 6, "makespan {makespan} must be ≥ lower bound 6");
}

// ─── extract_solution ────────────────────────────────────────────────────────

#[test]
fn test_openshopscheduling_to_ilp_extract_solution_respects_start_times() {
    // For small instance, if we manually craft an ILP solution, extraction should
    // order jobs on each machine by start time.
    let p = small_instance();
    let reduction: ReductionOSSToILP = ReduceTo::<ILP<i32>>::reduce_to(&p);

    // Variable layout: x_{0,1,0}=0, x_{0,1,1}=1, s_{0,0}=1, s_{0,1}=0, s_{1,0}=0, s_{1,1}=2, y_{0,0,1}=0, y_{1,0,1}=1, C=3
    // => M1: job 1 starts at 0, job 0 starts at 1 → order [1, 0]
    // => M2: job 0 starts at 0, job 1 starts at 2 → order [0, 1]
    let target_solution = vec![0, 1, 1, 0, 0, 2, 0, 1, 3];
    let extracted = reduction.extract_solution(&target_solution);
    // M1: J1 at t=0, J0 at t=1 → order [1, 0]
    // M2: J0 at t=0, J1 at t=2 → order [0, 1]
    assert_eq!(extracted[0..2], [1, 0], "M1 order should be [1, 0]");
    assert_eq!(extracted[2..4], [0, 1], "M2 order should be [0, 1]");
    let value = p.evaluate(&extracted);
    assert!(value.0.is_some(), "extracted config should be valid");
}

// ─── single job / single machine ─────────────────────────────────────────────

#[test]
fn test_openshopscheduling_to_ilp_single_job() {
    // 1 job, 2 machines: trivial, makespan = sum of processing times
    let p = OpenShopScheduling::new(2, vec![vec![3, 4]]);
    let reduction: ReductionOSSToILP = ReduceTo::<ILP<i32>>::reduce_to(&p);
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution);
    let value = p.evaluate(&extracted);
    assert!(value.0.is_some());
    assert_eq!(value, Min(Some(7)));
}

#[test]
fn test_openshopscheduling_to_ilp_single_machine() {
    // 3 jobs, 1 machine: serial schedule, makespan = sum of all processing times
    let p = OpenShopScheduling::new(1, vec![vec![2], vec![3], vec![1]]);
    let reduction: ReductionOSSToILP = ReduceTo::<ILP<i32>>::reduce_to(&p);
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution);
    let value = p.evaluate(&extracted);
    assert!(value.0.is_some());
    assert_eq!(value, Min(Some(6)));
}
