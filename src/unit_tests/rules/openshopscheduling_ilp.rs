use super::*;
use crate::models::algebraic::Bounded;
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

#[test]
fn test_decision_openshopscheduling_to_ilp_preserves_makespan_threshold() {
    let inner = small_instance();
    let optimization = ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&inner).unwrap();
    let solver = ILPSolver::new();
    let optimal = solver.solve(optimization.target_problem()).unwrap();
    for bound in [-1, 2, 3, 4] {
        let source = Decision::new(inner.clone(), bound);
        let reduction = ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).unwrap();
        let target = reduction.target_problem();
        assert!(target.objective().is_empty());
        assert!(target.num_vars() < optimization.target_problem().num_vars());
        crate::rules::test_helpers::assert_parameter_predictions(&source, &reduction);
        let result = solver.solve(&source);
        if bound < 3 {
            assert!(matches!(
                result,
                Err(crate::solvers::ILPSolveError::Infeasible)
            ));
            assert!(reduction.extract_solution(&optimal).is_err());
        } else {
            assert_eq!(
                source.evaluate(&result.unwrap()).unwrap(),
                crate::types::Or(true)
            );
            assert!(reduction
                .extract_solution(&vec![0; target.num_vars()])
                .is_err());
        }
    }
    assert_eq!(
        inner.evaluate(&solver.solve(&inner).unwrap()).unwrap(),
        Min(Some(3))
    );
}

/// 3 machines, 2 jobs.
fn medium_instance() -> OpenShopScheduling {
    OpenShopScheduling::new(3, vec![vec![3, 1, 2], vec![2, 3, 1]])
}

// ─── structure ───────────────────────────────────────────────────────────────

#[test]
fn test_openshopscheduling_to_ilp_structure_small() {
    let p = small_instance();
    let reduction: ReductionOSSToILP =
        ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&p).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    // n=2, m=2:
    // num_pairs = 1, num_order_vars = 1*2 = 2 (x_{0,1,0}, x_{0,1,1})
    // num_start_vars = 2*2 = 4 (s_{0,0}, s_{0,1}, s_{1,0}, s_{1,1})
    // num_machine_pairs = 1, num_job_pair_vars = 2*1 = 2 (y_{0,0,1}, y_{1,0,1})
    // c_var = 1
    // Total = 2 + 4 + 2 + 1 = 9
    assert_eq!(
        ilp.num_vars(),
        9,
        "expected 9 variables, got {}",
        ilp.num_vars()
    );
    // Four disjunction pairs need eight rows; four makespan rows remain.
    assert_eq!(ilp.num_constraints(), 12);
    assert_eq!(
        ilp.objective(),
        vec![(8, 1)],
        "objective should minimize C (index 8)"
    );
}

// ─── closed-loop ─────────────────────────────────────────────────────────────

#[test]
fn test_openshopscheduling_to_ilp_closed_loop_small() {
    let p = small_instance();
    let reduction: ReductionOSSToILP =
        ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&p).expect("reduction should succeed");
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be feasible");

    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    let value = p.evaluate(&extracted).unwrap();
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
    let reduction: ReductionOSSToILP =
        ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&p).expect("reduction should succeed");
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be feasible");

    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    let value = p.evaluate(&extracted).unwrap();
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
    let p = small_instance();
    let reduction: ReductionOSSToILP =
        ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&p).expect("reduction should succeed");
    let target_solution = vec![1, 0, 0, 1, 1, 0, 1, 0, 3];
    let extracted = reduction.extract_solution(&target_solution).unwrap();
    assert_eq!(extracted, vec![0, 1, 1, 0]);
    assert_eq!(p.evaluate(&extracted).unwrap(), Min(Some(3)));
}

// ─── single job / single machine ─────────────────────────────────────────────

#[test]
fn test_openshopscheduling_to_ilp_single_job() {
    // 1 job, 2 machines: trivial, makespan = sum of processing times
    let p = OpenShopScheduling::new(2, vec![vec![3, 4]]);
    let reduction: ReductionOSSToILP =
        ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&p).expect("reduction should succeed");
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    let value = p.evaluate(&extracted).unwrap();
    assert!(value.0.is_some());
    assert_eq!(value, Min(Some(7)));
}

#[test]
fn test_openshopscheduling_to_ilp_single_machine() {
    // 3 jobs, 1 machine: serial schedule, makespan = sum of all processing times
    let p = OpenShopScheduling::new(1, vec![vec![2], vec![3], vec![1]]);
    let reduction: ReductionOSSToILP =
        ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&p).expect("reduction should succeed");
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    let value = p.evaluate(&extracted).unwrap();
    assert!(value.0.is_some());
    assert_eq!(value, Min(Some(6)));
}

#[test]
fn test_decision_makespan_threshold_normalization() {
    for bound in [i64::MIN, -1, 0, 1, i64::MAX] {
        let source = crate::models::Decision::new(OpenShopScheduling::new(1, vec![vec![1]]), bound);
        let reduction = ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).unwrap();
        assert_eq!(
            reduction.target_problem().max_constraint_magnitude_bits(),
            1
        );
        match ILPSolver::new().solve(reduction.target_problem()) {
            Ok(solution) => {
                assert!(bound >= 1);
                assert!(
                    source
                        .evaluate(&reduction.extract_solution(&solution).unwrap())
                        .unwrap()
                        .0
                );
            }
            Err(error) => {
                assert!(bound < 1);
                assert_eq!(error, crate::solvers::ILPSolveError::Infeasible);
            }
        }
    }
}

#[test]
fn decision_open_shop_uses_the_bound_without_a_makespan_variable() {
    let source = Decision::new(OpenShopScheduling::new(2, vec![vec![1, 1], vec![1, 1]]), 2);
    let reduced = ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).unwrap();
    assert_eq!(reduced.target_problem().num_vars(), 8);
    assert_eq!(reduced.target_problem().num_constraints(), 9);
    assert_eq!(reduced.target_problem().num_nonzeros(), 26);
    let solution = ILPSolver::new().solve(&source).unwrap();
    assert_eq!(source.evaluate(&solution).unwrap(), crate::types::Or(true));
}

#[test]
fn symmetry_preserves_zero_duration_and_asymmetric_open_shops() {
    use crate::solvers::{BruteForce, ILPSolveError};
    for times in [
        vec![],
        vec![vec![]],
        vec![vec![0, 0], vec![0, 0]],
        vec![vec![0, 1], vec![1, 0]],
        vec![vec![1, 1], vec![1, 1]],
    ] {
        let machines = times.first().map_or(0, Vec::len);
        let source = OpenShopScheduling::new(machines, times);
        let reduction = ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).unwrap();
        crate::rules::test_helpers::assert_parameter_predictions(&source, &reduction);
        let expected = BruteForce::new().solve(&source).unwrap().unwrap();
        let actual = ILPSolver::new().solve(&source).unwrap();
        assert_eq!(source.evaluate(&actual), source.evaluate(&expected));
        for bound in [-1, 0, 1, 2] {
            let decision = Decision::new(source.clone(), bound);
            let reduced = ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&decision).unwrap();
            crate::rules::test_helpers::assert_parameter_predictions(&decision, &reduced);
            match ILPSolver::new().solve(&decision) {
                Ok(witness) => assert!(decision.evaluate(&witness).unwrap().0),
                Err(ILPSolveError::Infeasible) => assert!(!decision.evaluate(&expected).unwrap().0),
                other => panic!("unexpected solve: {other:?}"),
            }
        }
    }
}

#[test]
fn open_shop_reports_unrepresentable_constraint_arithmetic() {
    let source = OpenShopScheduling::new(1, vec![vec![i64::MAX / 2], vec![i64::MAX / 2]]);
    assert!(matches!(
        ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source),
        Err(crate::rules::ReductionError::IntegerOverflow { .. })
    ));
}
