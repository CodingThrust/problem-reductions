use super::*;
use crate::expr::Expr;

fn entry_with(declarations: fn() -> ReductionSizeDeclarations) -> ReductionEntry {
    ReductionEntry {
        source_name: "Source",
        target_name: "Target",
        source_variant_fn: Vec::new,
        target_variant_fn: Vec::new,
        size_declarations_fn: declarations,
        module_path: module_path!(),
        reduce_fn: None,
        reduce_aggregate_fn: None,
        turing: false,
    }
}

#[test]
fn one_relation_applies_to_the_whole_transform() {
    let entry = entry_with(|| ReductionSizeDeclarations {
        relation: Some(crate::size::SizeRelation::Exact),
        fields: vec![("n", Expr::variable("n"))],
        unavailable: vec![],
    });
    let contract = entry.size_contract().unwrap();
    let transform = contract.transform().unwrap();
    assert_eq!(transform.relation(), crate::size::SizeRelation::Exact);
    assert!(transform.get("n").is_some());
}

#[test]
fn unavailable_field_cannot_overlap_a_formula() {
    let entry = entry_with(|| ReductionSizeDeclarations {
        relation: Some(crate::size::SizeRelation::Exact),
        fields: vec![("n", Expr::variable("n"))],
        unavailable: vec![UnavailableSizeField {
            field: "n",
            reason: "the construction does not expose this statistic",
        }],
    });
    assert!(matches!(
        entry.size_contract(),
        Err(SizeContractError::DuplicateClassification { field, .. }) if field.as_ref() == "n"
    ));
}

#[test]
fn unavailable_field_requires_a_reason() {
    let entry = entry_with(|| ReductionSizeDeclarations {
        relation: None,
        fields: vec![],
        unavailable: vec![UnavailableSizeField {
            field: "n",
            reason: " ",
        }],
    });
    assert!(matches!(
        entry.size_contract(),
        Err(SizeContractError::EmptyUnavailableReason { field, .. }) if field.as_ref() == "n"
    ));
}

#[test]
fn size_contract_errors_and_entry_debug_are_transparent() {
    let transform_error = crate::size::SizeTransform::new(
        "bad exact",
        crate::size::SizeRelation::Exact,
        [("x", Expr::variable("n")), ("x", Expr::variable("m"))],
    )
    .unwrap_err();
    assert!(SizeContractError::from(transform_error)
        .to_string()
        .starts_with("invalid size transform:"));
    assert!(SizeContractError::DuplicateClassification {
        edge: "A -> B".into(),
        field: "x".into(),
    }
    .to_string()
    .contains("classifies target field `x` more than once"));
    assert!(SizeContractError::EmptyUnavailableReason {
        edge: "A -> B".into(),
        field: "x".into(),
    }
    .to_string()
    .contains("unavailable without a reason"));

    let entry = entry_with(ReductionSizeDeclarations::default);
    let debug = format!("{entry:?}");
    assert!(debug.contains("size_contract"));
    assert!(debug.contains("capabilities"));
}

#[test]
fn every_registered_contract_validates() {
    for entry in reduction_entries() {
        entry.size_contract().unwrap_or_else(|error| {
            panic!(
                "{} -> {} has an invalid size contract: {error}",
                entry.source_name, entry.target_name
            )
        });
    }
}

#[test]
fn every_registered_size_declaration_uses_problem_owned_parameters() {
    if let Err(errors) = validate_reduction_size_schemas() {
        panic!("{}", errors.join("\n"));
    }
}
