use super::*;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Min;

fn issue_yes_instance() -> ExactCoverBy3Sets {
    ExactCoverBy3Sets::new(
        6,
        vec![[0, 1, 2], [0, 3, 4], [2, 4, 5], [1, 3, 5], [0, 2, 4]],
    )
}

fn shared_zero_instance() -> ExactCoverBy3Sets {
    ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [0, 3, 4], [0, 4, 5]])
}

#[test]
fn test_exactcoverby3sets_to_minimumaxiomset_closed_loop() {
    let source = issue_yes_instance();
    let reduction = ReduceTo::<MinimumAxiomSet>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &reduction,
        "ExactCoverBy3Sets -> MinimumAxiomSet closed loop",
    );

    let optimal = BruteForce::new()
        .find_witness(target)
        .expect("expected an optimal target witness");
    assert_eq!(target.evaluate(&optimal), Min(Some(2)));
}

#[test]
fn test_exactcoverby3sets_to_minimumaxiomset_structure() {
    let source = issue_yes_instance();
    let reduction = ReduceTo::<MinimumAxiomSet>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_sentences(), 11);
    assert_eq!(target.num_true_sentences(), 11);
    assert_eq!(target.num_implications(), 20);
    assert_eq!(target.true_sentences(), &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    assert_eq!(target.implications()[0], (vec![6], 0));
    assert_eq!(target.implications()[1], (vec![6], 1));
    assert_eq!(target.implications()[2], (vec![6], 2));
    assert_eq!(target.implications()[3], (vec![0, 1, 2], 6));
    assert_eq!(target.implications()[16], (vec![10], 0));
    assert_eq!(target.implications()[17], (vec![10], 2));
    assert_eq!(target.implications()[18], (vec![10], 4));
    assert_eq!(target.implications()[19], (vec![0, 2, 4], 10));
}

#[test]
fn test_exactcoverby3sets_to_minimumaxiomset_no_instance_gap() {
    let source = shared_zero_instance();
    let reduction = ReduceTo::<MinimumAxiomSet>::reduce_to(&source);
    let target = reduction.target_problem();

    let optimal = BruteForce::new()
        .find_witness(target)
        .expect("expected an optimal target witness");
    assert_eq!(target.evaluate(&optimal), Min(Some(3)));

    let extracted = reduction.extract_solution(&optimal);
    assert!(!source.evaluate(&extracted));
}

#[test]
fn test_extract_solution_reads_only_set_sentence_axioms() {
    let source = issue_yes_instance();
    let reduction = ReduceTo::<MinimumAxiomSet>::reduce_to(&source);

    let extracted = reduction.extract_solution(&[1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 1]);
    assert_eq!(extracted, vec![0, 0, 0, 1, 1]);
}
