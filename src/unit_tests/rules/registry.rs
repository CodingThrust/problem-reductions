use super::*;
use crate::expr::Expr;

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
