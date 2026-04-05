use super::*;
use crate::models::algebraic::ILP;
use crate::models::formula::CNFClause;
use crate::models::misc::{PrecedenceConstrainedScheduling, PreemptiveScheduling};
#[cfg(feature = "ilp-solver")]
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

#[cfg(feature = "ilp-solver")]
fn solve_threshold_schedule_via_ilp(
    target: &PreemptiveScheduling,
    deadline: usize,
) -> Option<Vec<usize>> {
    let pcs = PrecedenceConstrainedScheduling::new(
        target.num_tasks(),
        target.num_processors(),
        deadline,
        target.precedences().to_vec(),
    );
    let pcs_to_ilp = ReduceTo::<ILP<bool>>::reduce_to(&pcs);
    let ilp_solution = ILPSolver::new().solve(pcs_to_ilp.target_problem())?;
    let slot_assignment = pcs_to_ilp.extract_solution(&ilp_solution);

    let mut config = vec![0usize; target.num_tasks() * target.d_max()];
    for (task, &slot) in slot_assignment.iter().enumerate() {
        config[task * target.d_max() + slot] = 1;
    }
    Some(config)
}

#[test]
fn test_ksatisfiability_to_preemptivescheduling_structure() {
    let source = yes_single_variable_instance();
    let reduction = ReduceTo::<PreemptiveScheduling>::reduce_to(&source);
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
    let reduction = ReduceTo::<PreemptiveScheduling>::reduce_to(&source);

    let schedule = construct_schedule_from_assignment(reduction.target_problem(), &[1], &source)
        .expect("satisfying assignment should yield a witness schedule");

    assert_eq!(reduction.target_problem().evaluate(&schedule), Min(Some(4)));

    let extracted = reduction.extract_solution(&schedule);
    assert_eq!(extracted, vec![1]);
    assert!(source.evaluate(&extracted).0);
}

#[cfg(feature = "ilp-solver")]
#[test]
fn test_ksatisfiability_to_preemptivescheduling_closed_loop() {
    let source = yes_single_variable_instance();
    let reduction = ReduceTo::<PreemptiveScheduling>::reduce_to(&source);

    let target_solution =
        solve_threshold_schedule_via_ilp(reduction.target_problem(), reduction.threshold())
            .expect("satisfying instance should meet the threshold");

    assert_eq!(
        reduction.target_problem().evaluate(&target_solution),
        Min(Some(reduction.threshold()))
    );

    let extracted = reduction.extract_solution(&target_solution);
    assert_eq!(extracted, vec![1]);
    assert!(source.evaluate(&extracted).0);
}

#[cfg(feature = "ilp-solver")]
#[test]
fn test_ksatisfiability_to_preemptivescheduling_unsatisfiable_threshold_gap() {
    let source = no_single_variable_instance();
    let reduction = ReduceTo::<PreemptiveScheduling>::reduce_to(&source);

    assert!(
        solve_threshold_schedule_via_ilp(reduction.target_problem(), reduction.threshold())
            .is_none(),
        "unsatisfiable instance should not admit a schedule by the threshold"
    );
}
