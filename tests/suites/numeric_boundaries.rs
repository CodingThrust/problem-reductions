use problemreductions::models::formula::{
    CNFClause, KSatisfiability, Maximum2Satisfiability, NAESatisfiability,
    OneInThreeSatisfiability, Planar3Satisfiability, QuantifiedBooleanFormulas, Quantifier,
    Satisfiability,
};
use problemreductions::models::graph::MinimumDominatingSet;
use problemreductions::models::set::MinimumSetCovering;
use problemreductions::rules::{ReduceTo, ReductionResult};
use problemreductions::topology::SimpleGraph;
use problemreductions::variant::K3;
use problemreductions::Problem;

#[test]
fn numeric_boundaries_weight_totals_use_i64() {
    let dominating =
        MinimumDominatingSet::new(SimpleGraph::new(2, vec![]), vec![i32::MAX, i32::MAX]);
    assert_eq!(dominating.evaluate(&[1, 1]).0, Some(4_294_967_294_i64));

    let covering =
        MinimumSetCovering::with_weights(2, vec![vec![0], vec![1]], vec![i32::MAX, i32::MAX]);
    assert_eq!(covering.evaluate(&[1, 1]).0, Some(4_294_967_294_i64));

    let ordinary = MinimumSetCovering::with_weights(1, vec![vec![0]], vec![7i32]);
    assert_eq!(ordinary.evaluate(&[1]).0, Some(7_i64));
}

#[test]
fn numeric_boundaries_all_cnf_models_reject_invalid_literals() {
    for literal in [0, i32::MIN, 2] {
        let errors = [
            Satisfiability::try_new(1, vec![CNFClause::new(vec![literal])]).unwrap_err(),
            KSatisfiability::<K3>::try_new(1, vec![CNFClause::new(vec![literal, 1, 1])])
                .unwrap_err(),
            NAESatisfiability::try_new(1, vec![CNFClause::new(vec![literal, 1])]).unwrap_err(),
            Maximum2Satisfiability::try_new(1, vec![CNFClause::new(vec![literal, 1])]).unwrap_err(),
            OneInThreeSatisfiability::try_new(1, vec![CNFClause::new(vec![literal, 1, 1])])
                .unwrap_err(),
            Planar3Satisfiability::try_new(1, vec![CNFClause::new(vec![literal, 1, 1])])
                .unwrap_err(),
            QuantifiedBooleanFormulas::try_new(
                1,
                vec![Quantifier::Exists],
                vec![CNFClause::new(vec![literal])],
            )
            .unwrap_err(),
        ];

        for error in errors {
            assert!(error.contains(&literal.to_string()), "{error}");
            assert!(error.contains("1..=1"), "{error}");
        }
    }
}

#[test]
fn numeric_boundaries_sat_variable_limit_does_not_allocate() {
    let max = i32::MAX as usize;
    let formula = Satisfiability::try_new(max, vec![CNFClause::new(vec![i32::MAX])]).unwrap();
    assert_eq!(formula.num_vars(), max);

    let error = Satisfiability::try_new(max + 1, vec![]).unwrap_err();
    assert!(error.contains(&(max + 1).to_string()), "{error}");
    assert!(error.contains(&i32::MAX.to_string()), "{error}");
}

#[test]
fn numeric_boundaries_serde_uses_cnf_validation() {
    let error =
        serde_json::from_str::<Satisfiability>(r#"{"num_vars":1,"clauses":[{"literals":[0]}]}"#)
            .unwrap_err()
            .to_string();
    assert!(error.contains("invalid literal 0"), "{error}");
    assert!(error.contains("1..=1"), "{error}");
}

#[test]
fn numeric_boundaries_sat_reduction_rejects_exhausted_variable_ids() {
    let source = Satisfiability::new(i32::MAX as usize, vec![CNFClause::new(vec![i32::MAX])]);
    let panic = std::panic::catch_unwind(|| {
        let _ =
            <Satisfiability as ReduceTo<KSatisfiability<K3>>>::reduce_to(&source).target_problem();
    })
    .unwrap_err();
    let message = panic_message(panic);
    assert!(
        message.contains("Satisfiability -> KSatisfiability"),
        "{message}"
    );
    assert!(
        message.contains("allocate 1 auxiliary variable"),
        "{message}"
    );
    assert!(message.contains(&i32::MAX.to_string()), "{message}");
}

fn panic_message(panic: Box<dyn std::any::Any + Send>) -> String {
    if let Some(message) = panic.downcast_ref::<String>() {
        return message.clone();
    }
    panic
        .downcast_ref::<&str>()
        .expect("panic payload must be a string")
        .to_string()
}
