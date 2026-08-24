use problemreductions::models::formula::{
    CNFClause, KSatisfiability, Maximum2Satisfiability, NAESatisfiability,
    OneInThreeSatisfiability, Planar3Satisfiability, QuantifiedBooleanFormulas, Quantifier,
    Satisfiability,
};
use problemreductions::models::graph::MinimumDominatingSet;
use problemreductions::models::set::MinimumSetCovering;
use problemreductions::rules::ReduceTo;
use problemreductions::topology::SimpleGraph;
use problemreductions::variant::K3;
use problemreductions::Problem;

#[test]
fn numeric_boundaries_weight_totals_use_i64() {
    let weight = i64::MAX / 2;
    let expected = i64::MAX - 1;
    let dominating = MinimumDominatingSet::new(SimpleGraph::new(2, vec![]), vec![weight, weight]);
    assert_eq!(dominating.evaluate(&[1, 1]).unwrap().0, Some(expected));

    let covering =
        MinimumSetCovering::with_weights(2, vec![vec![0], vec![1]], vec![weight, weight]);
    assert_eq!(covering.evaluate(&[1, 1]).unwrap().0, Some(expected));

    let ordinary = MinimumSetCovering::with_weights(1, vec![vec![0]], vec![7i64]);
    assert_eq!(ordinary.evaluate(&[1]).unwrap().0, Some(7));
}

#[test]
fn numeric_boundaries_all_cnf_models_reject_invalid_literals() {
    for literal in [0, i64::MIN, 2] {
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
            let error = error.to_string();
            assert!(error.contains(&literal.to_string()), "{error}");
            assert!(error.contains("1..=1"), "{error}");
        }
    }
}

#[test]
fn numeric_boundaries_sat_variable_limit_does_not_allocate() {
    let max = i64::MAX as usize;
    let formula = Satisfiability::try_new(max, vec![CNFClause::new(vec![i64::MAX])]).unwrap();
    assert_eq!(formula.num_vars(), max);

    let error = Satisfiability::try_new(max + 1, vec![]).unwrap_err();
    let error = error.to_string();
    assert!(error.contains(&(max + 1).to_string()), "{error}");
    assert!(error.contains(&i64::MAX.to_string()), "{error}");
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
    let source = Satisfiability::new(i64::MAX as usize, vec![CNFClause::new(vec![i64::MAX])]);
    let message = <Satisfiability as ReduceTo<KSatisfiability<K3>>>::reduce_to(&source)
        .unwrap_err()
        .to_string();
    assert!(
        message.contains("Satisfiability -> KSatisfiability"),
        "{message}"
    );
    assert!(
        message.contains("allocate 1 auxiliary variable"),
        "{message}"
    );
    assert!(message.contains(&i64::MAX.to_string()), "{message}");
}
