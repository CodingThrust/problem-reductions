use super::*;
use crate::expr::Expr;

#[test]
fn registered_aggregate_mappings_share_the_witness_result() {
    use crate::models::formula::{CNFClause, Satisfiability};
    use crate::traits::Problem;

    let source = Satisfiability::new(1, vec![CNFClause::new(vec![1])]);
    let entry = reduction_entries()
        .into_iter()
        .find(|entry| {
            entry.source_name == Satisfiability::NAME && entry.target_name == "KSatisfiability"
        })
        .unwrap();
    let witness = entry.reduce_fn.unwrap()(&source).unwrap();
    let view = entry.aggregate_view_fn.unwrap()(witness.as_ref()).unwrap();
    assert!(std::ptr::eq(
        witness.target_problem_any(),
        view.target_problem_any()
    ));
}

#[test]
fn aggregate_executors_reject_wrong_source_types() {
    for mapping in inventory::iter::<AggregateMappingEntry> {
        let error = (mapping.reduce_fn)(&())
            .err()
            .expect("wrong source type must fail");
        assert!(
            matches!(error, crate::rules::ReductionError::SourceTypeMismatch {
            source_problem, target_problem, ..
        } if source_problem == mapping.source_name && target_problem == mapping.target_name)
        );
    }
}

#[test]
fn registered_executors_preserve_construction_errors() {
    use crate::models::misc::Partition;
    let source = Partition::new(vec![1_i64 << 61, 1_i64 << 61]).unwrap();
    let entry = reduction_entries()
        .into_iter()
        .find(|entry| entry.source_name == "Partition" && entry.target_name == "OpenShopScheduling")
        .unwrap();
    let witness_error = entry.reduce_fn.unwrap()(&source).err().unwrap();
    let aggregate_error = entry.reduce_aggregate_fn.unwrap()(&source).err().unwrap();
    for error in [witness_error, aggregate_error] {
        assert!(matches!(
            error,
            crate::rules::ReductionError::Construction {
                source_problem: "Partition",
                target_problem: "OpenShopScheduling",
                cause: crate::registry::ConstructionError::IntegerOverflow(_),
            }
        ));
    }
}

#[test]
fn aggregate_registration_matches_exact_endpoints_and_rejects_conflicts() {
    let mapping = AggregateMappingEntry {
        source_name: "Source",
        target_name: "Target",
        source_variant_fn: || vec![("weight", "i64"), ("graph", "SimpleGraph")],
        target_variant_fn: Vec::new,
        reduce_fn: |_| unreachable!(),
        view_fn: |_| unreachable!(),
    };
    let mut entry = entry_with(ReductionParameterDeclarations::default);
    entry.source_variant_fn = || vec![("graph", "SimpleGraph"), ("weight", "i64")];
    entry.reduce_fn = Some(|_| unreachable!());
    let mut wrong_variant = entry;
    wrong_variant.source_variant_fn = Vec::new;
    let mut entries = [wrong_variant, entry];
    attach_aggregate_mapping(&mut entries, &mapping);
    assert!(!entries[0].capabilities().aggregate);
    assert!(entries[1].capabilities().aggregate);

    let mut no_executor = entry;
    no_executor.reduce_fn = None;
    let mut turing = entry;
    turing.turing = true;
    for mut invalid in [
        vec![],
        vec![wrong_variant],
        vec![entry, entry],
        vec![no_executor],
        vec![turing],
        vec![entries[1]],
    ] {
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            attach_aggregate_mapping(&mut invalid, &mapping);
        }))
        .is_err());
    }
}

fn entry_with(declarations: fn() -> ReductionParameterDeclarations) -> ReductionEntry {
    ReductionEntry {
        source_name: "Source",
        target_name: "Target",
        source_variant_fn: Vec::new,
        target_variant_fn: Vec::new,
        parameter_declarations_fn: declarations,
        module_path: module_path!(),
        reduce_fn: None,
        reduce_aggregate_fn: None,
        aggregate_view_fn: None,
        turing: false,
    }
}

#[test]
fn one_relation_applies_to_the_whole_transform() {
    let entry = entry_with(|| ReductionParameterDeclarations {
        relation: Some(crate::parameters::ParameterRelation::Exact),
        fields: vec![("n", Expr::variable("n"))],
        unavailable: vec![],
    });
    let contract = entry.parameter_contract().unwrap();
    let transform = contract.transform().unwrap();
    assert_eq!(
        transform.relation(),
        crate::parameters::ParameterRelation::Exact
    );
    assert!(transform.get("n").is_some());
}

#[test]
fn unavailable_field_cannot_overlap_a_formula() {
    let entry = entry_with(|| ReductionParameterDeclarations {
        relation: Some(crate::parameters::ParameterRelation::Exact),
        fields: vec![("n", Expr::variable("n"))],
        unavailable: vec![UnavailableParameterField {
            field: "n",
            reason: "the construction does not expose this statistic",
        }],
    });
    assert!(matches!(
        entry.parameter_contract(),
        Err(ParameterContractError::DuplicateClassification { field, .. }) if field.as_ref() == "n"
    ));
}

#[test]
fn unavailable_field_requires_a_reason() {
    let entry = entry_with(|| ReductionParameterDeclarations {
        relation: None,
        fields: vec![],
        unavailable: vec![UnavailableParameterField {
            field: "n",
            reason: " ",
        }],
    });
    assert!(matches!(
        entry.parameter_contract(),
        Err(ParameterContractError::EmptyUnavailableReason { field, .. }) if field.as_ref() == "n"
    ));
}

#[test]
fn parameter_contract_errors_and_entry_debug_are_transparent() {
    let transform_error = crate::parameters::ParameterTransform::new(
        "bad exact",
        crate::parameters::ParameterRelation::Exact,
        [("x", Expr::variable("n")), ("x", Expr::variable("m"))],
    )
    .unwrap_err();
    assert!(ParameterContractError::from(transform_error)
        .to_string()
        .starts_with("invalid parameter transform:"));
    assert!(ParameterContractError::DuplicateClassification {
        edge: "A -> B".into(),
        field: "x".into(),
    }
    .to_string()
    .contains("classifies target field `x` more than once"));
    assert!(ParameterContractError::EmptyUnavailableReason {
        edge: "A -> B".into(),
        field: "x".into(),
    }
    .to_string()
    .contains("unavailable without a reason"));

    let entry = entry_with(ReductionParameterDeclarations::default);
    let debug = format!("{entry:?}");
    assert!(debug.contains("parameter_contract"));
    assert!(debug.contains("capabilities"));
}

#[test]
fn every_registered_contract_validates() {
    for entry in reduction_entries() {
        entry.parameter_contract().unwrap_or_else(|error| {
            panic!(
                "{} -> {} has an invalid parameter contract: {error}",
                entry.source_name, entry.target_name
            )
        });
    }
}

#[test]
fn every_registered_parameter_declaration_uses_problem_owned_parameters() {
    if let Err(errors) = validate_reduction_parameter_schemas() {
        panic!("{}", errors.join("\n"));
    }
}
