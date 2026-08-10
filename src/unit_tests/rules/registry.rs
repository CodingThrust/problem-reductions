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
        source_size_fn: |_| crate::types::ProblemSize::new(vec![]),
    }
}

#[test]
fn exact_field_may_have_a_separate_certified_bound() {
    let entry = entry_with(|| ReductionSizeDeclarations {
        exact: vec![("n", Expr::variable("n"))],
        bounds: vec![("n", Expr::variable("n"))],
        unavailable: vec![],
    });
    let contract = entry.size_contract().unwrap();
    assert!(contract.exact().unwrap().get("n").is_some());
    assert!(contract.bounds().unwrap().get("n").is_some());
}

#[test]
fn unavailable_field_cannot_overlap_a_formula() {
    let entry = entry_with(|| ReductionSizeDeclarations {
        exact: vec![("n", Expr::variable("n"))],
        bounds: vec![],
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
        exact: vec![],
        bounds: vec![],
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
    let exact_error = SizeMap::new(
        "bad exact",
        [("x", Expr::variable("n")), ("x", Expr::variable("m"))],
    )
    .unwrap_err();
    let bound_error =
        SizeBound::new("bad bound", [("x", Expr::try_parse("n - 1").unwrap())]).unwrap_err();
    assert!(SizeContractError::from(exact_error)
        .to_string()
        .starts_with("invalid exact size map:"));
    assert!(SizeContractError::from(bound_error)
        .to_string()
        .starts_with("invalid certified size bound:"));
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
fn every_registered_target_schema_field_is_classified() {
    let mut mismatches = Vec::new();
    for entry in reduction_entries() {
        let declared: std::collections::HashSet<_> =
            crate::registry::declared_size_fields(entry.target_name)
                .into_iter()
                .collect();
        let contract = entry.size_contract().unwrap();
        let mut classified = std::collections::HashSet::new();
        if let Some(exact) = contract.exact() {
            classified.extend(exact.expressions().map(|(field, _)| field));
        }
        if let Some(bounds) = contract.bounds() {
            classified.extend(bounds.expressions().map(|(field, _)| field));
        }
        classified.extend(contract.unavailable().iter().map(|field| field.field));
        if !declared.is_empty() && classified != declared {
            mismatches.push(format!(
                "{} -> {}: classified={classified:?}, declared={declared:?}",
                entry.source_name, entry.target_name
            ));
        }
    }
    assert!(mismatches.is_empty(), "{}", mismatches.join("\n"));
}
