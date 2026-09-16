use super::*;
use crate::models::algebraic::ILP;
use crate::models::formula::CNFClause;
use crate::models::misc::{PrecedenceConstrainedScheduling, PreemptiveScheduling};
use crate::solvers::ILPSolver;
use crate::solvers::SolveOutcome;
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
        Err(error) => panic!("ILP execution failed: {error}"),
    };
    let slot_assignment = pcs_to_ilp
        .recover_result(
            &pcs,
            SolveOutcome::optimal(pcs_to_ilp.target_problem(), ilp_solution.clone()).unwrap(),
        )
        .unwrap()
        .into_solution()
        .expect("qualifying target result must recover a source solution");

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
    assert_eq!(target.num_precedences(), 69);
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

    let extracted = reduction
        .recover_result(
            &source,
            SolveOutcome::optimal(reduction.target_problem(), schedule.clone()).unwrap(),
        )
        .unwrap()
        .into_solution()
        .expect("qualifying target result must recover a source solution");
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

    let extracted = result
        .recover_result(
            &source,
            SolveOutcome::optimal(result.target_problem(), schedule.clone()).unwrap(),
        )
        .unwrap()
        .into_solution()
        .expect("qualifying target result must recover a source solution");
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

    let extracted = reduction
        .recover_result(
            &source,
            SolveOutcome::optimal(reduction.target_problem(), target_solution.clone()).unwrap(),
        )
        .unwrap()
        .into_solution()
        .expect("qualifying target result must recover a source solution");
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
fn filler_clock_excludes_every_unsatisfying_slot_zero_assignment() {
    use crate::models::algebraic::{LinearConstraint, ObjectiveSense};
    let source = KSatisfiability::<K3>::new(3, vec![CNFClause::new(vec![1, 2, 3])]);
    let reduction = ReduceTo::<PreemptiveScheduling>::reduce_to(&source).unwrap();
    let target = reduction.target_problem();
    let construction = build_ullman_construction(&source);
    assert!(construction
        .filler_jobs_by_slot
        .iter()
        .all(|layer| !layer.is_empty()));
    assert_eq!(
        target.num_tasks(),
        target.num_processors() * reduction.threshold()
    );
    let pcs = PrecedenceConstrainedScheduling::new(
        target.num_tasks(),
        target.num_processors(),
        reduction.threshold() as i64,
        target.precedences().to_vec(),
    );
    let pcs_to_ilp = ReduceTo::<ILP<bool>>::reduce_to(&pcs).unwrap();
    let base = pcs_to_ilp.target_problem();
    // Fix the extracted assignment, not one particular schedule. This searches
    // ALL threshold schedules with that assignment, including noncanonical ones.
    for bits in 0..8 {
        let assignment: Vec<_> = (0..3).map(|i| bits & (1 << i) != 0).collect();
        let mut constraints = base.constraints().to_vec();
        for (&job, &value) in reduction.positive_start_jobs.iter().zip(&assignment) {
            constraints.push(LinearConstraint::eq(
                vec![(job * reduction.threshold(), 1)],
                i64::from(value),
            ));
        }
        let constrained = ILP::<bool>::new(
            base.num_vars(),
            constraints,
            vec![],
            ObjectiveSense::Minimize,
        )
        .unwrap();
        match ILPSolver::new().solve(&constrained) {
            Ok(witness) => {
                assert!(source.evaluate(&assignment).unwrap().0);
                let slots = pcs_to_ilp
                    .recover_result(&pcs, SolveOutcome::optimal(base, witness).unwrap())
                    .unwrap()
                    .into_solution()
                    .unwrap();
                let mut schedule = vec![vec![false; target.d_max()]; target.num_tasks()];
                for (job, slot) in slots.into_iter().enumerate() {
                    schedule[job][slot] = true;
                }
                for (slot, layer) in construction.filler_jobs_by_slot.iter().enumerate() {
                    assert!(layer.iter().all(|&job| schedule[job][slot]));
                }
                let recovered = reduction
                    .recover_result(
                        &source,
                        SolveOutcome::optimal(target, schedule.clone()).unwrap(),
                    )
                    .unwrap()
                    .into_solution()
                    .unwrap();
                assert_eq!(recovered, assignment);
                assert!(reduction
                    .recover_result(&source, SolveOutcome::feasible(target, schedule).unwrap())
                    .unwrap()
                    .into_solution()
                    .is_some());
            }
            Err(crate::solvers::ILPSolveError::Infeasible) => {
                assert!(!source.evaluate(&assignment).unwrap().0)
            }
            Err(error) => panic!("ILP execution failed: {error}"),
        }
    }
}

#[test]
fn short_and_empty_clauses_preserve_truth_and_recovery_status() {
    for (n, clauses) in [
        (0, vec![]),
        (2, vec![]),
        (0, vec![vec![]]),
        (2, vec![vec![]]),
        (1, vec![vec![1]]),
        (1, vec![vec![1], vec![-1]]),
        (2, vec![vec![1, -2]]),
    ] {
        let source = KSatisfiability::<K3>::new_allow_less(
            n,
            clauses.into_iter().map(CNFClause::new).collect(),
        );
        let reduction = ReduceTo::<PreemptiveScheduling>::reduce_to(&source).unwrap();
        let expected = crate::solvers::BruteForce::new().solve(&source).unwrap();
        let schedule = if reduction.threshold() == 0 {
            None
        } else {
            solve_threshold_schedule_via_ilp(reduction.target_problem(), reduction.threshold())
        };
        assert_eq!(schedule.is_some(), expected.is_some());
        if let Some(schedule) = schedule {
            assert!(reduction
                .recover_result(
                    &source,
                    SolveOutcome::optimal(reduction.target_problem(), schedule).unwrap()
                )
                .unwrap()
                .into_solution()
                .is_some());
            let witness = construct_schedule_from_assignment(
                reduction.target_problem(),
                &expected.unwrap(),
                &source,
            )
            .unwrap();
            assert!(reduction
                .target_problem()
                .evaluate(&witness)
                .unwrap()
                .0
                .is_some());
        }
        if reduction.threshold() == 0 {
            let schedule = vec![vec![true]];
            assert!(matches!(
                reduction
                    .recover_result(
                        &source,
                        SolveOutcome::optimal(reduction.target_problem(), schedule.clone())
                            .unwrap()
                    )
                    .unwrap(),
                SolveOutcome::Infeasible
            ));
            assert!(matches!(
                reduction.recover_result(
                    &source,
                    SolveOutcome::feasible(reduction.target_problem(), schedule).unwrap()
                ),
                Err(crate::rules::ExtractionError::InsufficientSolutionQuality)
            ));
        }
    }
}
