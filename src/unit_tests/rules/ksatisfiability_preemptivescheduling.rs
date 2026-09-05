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
    let ilp_solution = ILPSolver::new().solve(pcs_to_ilp.target_problem()).ok()?;
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
    assert_eq!(target.num_processors(), 6);
    assert_eq!(target.num_tasks(), 24);
    assert_eq!(target.d_max(), 24);
    assert_eq!(target.num_precedences(), 49);
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
