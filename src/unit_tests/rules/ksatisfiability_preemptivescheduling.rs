use super::*;
use crate::models::algebraic::ILP;
use crate::models::formula::CNFClause;
use crate::models::misc::{PrecedenceConstrainedScheduling, PreemptiveScheduling};
use crate::solvers::ILPSolver;
use crate::traits::Problem;
use crate::types::Min;
use crate::variant::K3;

fn yes_single_variable_instance() -> KSatisfiability<K3> {
    KSatisfiability::<K3>::new(1, vec![CNFClause::new(vec![1, 1, 1])])
}

fn no_single_variable_instance() -> KSatisfiability<K3> {
    KSatisfiability::<K3>::new(
        1,
        vec![
            CNFClause::new(vec![1, 1, 1]),
            CNFClause::new(vec![-1, -1, -1]),
        ],
    )
}

fn solve_threshold_schedule_via_ilp(
    target: &PreemptiveScheduling,
    deadline: usize,
) -> Option<Vec<Vec<bool>>> {
    let pcs = PrecedenceConstrainedScheduling::new(
        target.num_tasks(),
        target.num_processors(),
        i64::try_from(deadline).unwrap(),
        target.precedences().to_vec(),
    );
    let pcs_to_ilp = ReduceTo::<ILP<bool>>::reduce_to(&pcs).expect("reduction should succeed");
    let ilp_solution = match ILPSolver::new().solve(pcs_to_ilp.target_problem()) {
        Ok(solution) => solution,
        Err(crate::solvers::ILPSolveError::Infeasible) => return None,
        Err(error) => panic!("threshold solver failed: {error}"),
    };
    let slot_assignment = pcs_to_ilp.extract_solution(&ilp_solution).unwrap();

    let mut config = vec![vec![false; target.d_max()]; target.num_tasks()];
    for (task, &slot) in slot_assignment.iter().enumerate() {
        config[task][slot] = true;
    }
    Some(config)
}

#[test]
fn test_ksatisfiability_to_preemptivescheduling_structure() {
    let source = yes_single_variable_instance();
    let reduction =
        ReduceTo::<PreemptiveScheduling>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    assert_eq!(reduction.threshold(), 4);
    assert_eq!(target.num_processors(), 7);
    assert_eq!(target.num_tasks(), 28);
    assert_eq!(target.d_max(), 28);
    let construction = build_ullman_construction(&source);
    assert!(construction
        .filler_jobs_by_slot
        .iter()
        .all(|layer| !layer.is_empty()));
    assert!(target.lengths().iter().all(|&length| length == 1));
}

#[test]
fn test_ksatisfiability_to_preemptivescheduling_extract_solution_from_constructed_schedule() {
    let source = yes_single_variable_instance();
    let reduction =
        ReduceTo::<PreemptiveScheduling>::reduce_to(&source).expect("reduction should succeed");

    let schedule = construct_schedule_from_assignment(reduction.target_problem(), &[true], &source)
        .expect("satisfying assignment should yield a witness schedule");

    assert_eq!(
        reduction.target_problem().evaluate(&schedule).unwrap(),
        Min(Some(4))
    );

    let extracted = reduction.extract_solution(&schedule).unwrap();
    assert_eq!(extracted, vec![true]);
    assert!(source.evaluate(&extracted).unwrap().0);
}

#[test]
fn test_ksatisfiability_to_preemptivescheduling_multi_variable_round_trip() {
    let source = KSatisfiability::<K3>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, -3]),
        ],
    );
    let result =
        ReduceTo::<PreemptiveScheduling>::reduce_to(&source).expect("reduction should succeed");

    let schedule =
        construct_schedule_from_assignment(result.target_problem(), &[true, true, false], &source)
            .expect("satisfying assignment should yield a witness schedule");

    let extracted = result.extract_solution(&schedule).unwrap();
    assert_eq!(extracted, vec![true, true, false]);
    assert!(source.evaluate(&extracted).unwrap().0);
}

#[test]
fn test_ksatisfiability_to_preemptivescheduling_closed_loop() {
    let source = yes_single_variable_instance();
    let reduction =
        ReduceTo::<PreemptiveScheduling>::reduce_to(&source).expect("reduction should succeed");

    let target_solution =
        solve_threshold_schedule_via_ilp(reduction.target_problem(), reduction.threshold())
            .expect("satisfying instance should meet the threshold");

    assert_eq!(
        reduction
            .target_problem()
            .evaluate(&target_solution)
            .unwrap(),
        Min(Some(i64::try_from(reduction.threshold()).unwrap()))
    );

    let extracted = reduction.extract_solution(&target_solution).unwrap();
    assert_eq!(extracted, vec![true]);
    assert!(source.evaluate(&extracted).unwrap().0);
}

#[test]
fn test_ksatisfiability_to_preemptivescheduling_unsatisfiable_threshold_gap() {
    let source = no_single_variable_instance();
    let reduction =
        ReduceTo::<PreemptiveScheduling>::reduce_to(&source).expect("reduction should succeed");

    assert!(
        solve_threshold_schedule_via_ilp(reduction.target_problem(), reduction.threshold())
            .is_none(),
        "unsatisfiable instance should not admit a schedule by the threshold"
    );
}

#[test]
fn test_threshold_value_mapping_and_short_clauses() {
    use crate::rules::AggregateReductionResult;
    use crate::types::Or;

    for source in [
        KSatisfiability::<K3>::new(0, vec![]),
        KSatisfiability::<K3>::new(2, vec![]),
        KSatisfiability::<K3>::new_allow_less(0, vec![CNFClause::new(vec![])]),
        KSatisfiability::<K3>::new_allow_less(1, vec![CNFClause::new(vec![1])]),
        KSatisfiability::<K3>::new_allow_less(2, vec![CNFClause::new(vec![1, -2])]),
        KSatisfiability::<K3>::new_allow_less(
            1,
            vec![CNFClause::new(vec![1]), CNFClause::new(vec![-1])],
        ),
        yes_single_variable_instance(),
        no_single_variable_instance(),
    ] {
        let result = ReduceTo::<PreemptiveScheduling>::reduce_to(&source).unwrap();
        let expected = crate::solvers::BruteForce::new()
            .solve(&source)
            .unwrap()
            .is_some();
        let target = ReductionResult::target_problem(&result);
        if result.threshold() == 0 {
            let optimum = crate::solvers::BruteForce::new()
                .solve(target)
                .unwrap()
                .unwrap();
            assert!(!expected);
            assert_eq!(
                result.extract_value(target.evaluate(&optimum).unwrap()),
                Or(false)
            );
            assert!(result.extract_solution(&optimum).is_err());
            continue;
        }
        let schedule = solve_threshold_schedule_via_ilp(target, result.threshold());
        assert_eq!(schedule.is_some(), expected);
        assert_eq!(
            result.extract_value(Min(Some(result.threshold() as i64 + 1))),
            Or(false)
        );
        assert_eq!(result.extract_value(Min(None)), Or(false));
        if let Some(schedule) = schedule {
            assert!(construct_schedule_from_assignment(
                target,
                &vec![true; source.num_vars()],
                &source
            )
            .is_some());
            let value = target.evaluate(&schedule).unwrap();
            assert_eq!(result.extract_value(value), Or(true));
            assert!(
                source
                    .evaluate(&result.extract_solution(&schedule).unwrap())
                    .unwrap()
                    .0
            );
        }
    }
}

#[test]
fn test_extract_rejects_invalid_and_late_schedules() {
    let source = yes_single_variable_instance();
    let result = ReduceTo::<PreemptiveScheduling>::reduce_to(&source).unwrap();
    let mut schedule =
        construct_schedule_from_assignment(result.target_problem(), &[true], &source).unwrap();
    for task in &mut schedule {
        task.rotate_right(1);
    }
    assert_eq!(
        result.target_problem().evaluate(&schedule).unwrap(),
        Min(Some(5))
    );
    assert!(result.extract_solution(&schedule).is_err());
    assert!(result
        .extract_solution(&vec![
            vec![false; result.target_problem().d_max()];
            schedule.len()
        ])
        .is_err());
    assert!(result.extract_solution(&vec![]).is_err());
}

#[test]
fn test_registered_aggregate_mapping() {
    let entries = crate::rules::registry::reduction_entries();
    let edge = entries
        .iter()
        .find(|edge| {
            edge.source_name == "KSatisfiability"
                && (edge.source_variant_fn)() == KSatisfiability::<K3>::variant()
                && edge.target_name == "PreemptiveScheduling"
        })
        .unwrap();
    for (clauses, expected) in [(vec![], true), (vec![CNFClause::new(vec![])], false)] {
        let source = KSatisfiability::<K3>::new_allow_less(0, clauses);
        let result = (edge.reduce_aggregate_fn.unwrap())(&source).unwrap();
        assert_eq!(
            result
                .extract_value_from_solution_dyn(&vec![vec![true]])
                .unwrap(),
            serde_json::json!(expected),
        );
    }
}
