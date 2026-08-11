use crate::registry::variant::{
    validate_create_inputs, validate_direct_create_inputs, validate_variant_aliases, variant_label,
};
use crate::registry::{ConstructionError, CreateInputCodec, CreateInputInfo, FieldInfo};
use std::collections::{BTreeMap, BTreeSet};

#[test]
fn variant_alias_inventory_is_valid() {
    if let Err(conflicts) = validate_variant_aliases() {
        panic!("variant alias validation failed:\n{}", conflicts.join("\n"));
    }
}

// --- validate_aliases_inner unit tests ---

use crate::registry::variant::validate_aliases_inner;

fn empty_problem_names() -> BTreeMap<String, Vec<String>> {
    BTreeMap::new()
}

const CREATE_INPUTS: &[CreateInputInfo] = &[
    CreateInputInfo {
        name: "required_value",
        type_name: "usize",
        description: "A required value",
        required: true,
        codec: CreateInputCodec::Scalar,
    },
    CreateInputInfo {
        name: "optional_value",
        type_name: "usize",
        description: "An optional value",
        required: false,
        codec: CreateInputCodec::Scalar,
    },
];

#[test]
fn construction_contract_accepts_declared_inputs() {
    let data = serde_json::json!({"required_value": 1, "optional_value": 2});
    assert_eq!(validate_create_inputs(CREATE_INPUTS, &data), Ok(()));
}

#[test]
fn construction_contract_rejects_unknown_inputs() {
    let data = serde_json::json!({"required_value": 1, "removed_value": 2});
    assert_eq!(
        validate_create_inputs(CREATE_INPUTS, &data),
        Err(ConstructionError::UnknownInputs(vec![
            "removed_value".to_string()
        ]))
    );
}

#[test]
fn construction_contract_rejects_missing_required_inputs() {
    let data = serde_json::json!({"optional_value": 2});
    assert_eq!(
        validate_create_inputs(CREATE_INPUTS, &data),
        Err(ConstructionError::MissingInputs(vec![
            "required_value".to_string()
        ]))
    );
}

#[test]
fn construction_contract_rejects_non_object_values() {
    assert_eq!(
        validate_create_inputs(CREATE_INPUTS, &serde_json::json!([])),
        Err(ConstructionError::ExpectedObject)
    );
}

#[test]
fn construction_contract_rejects_duplicate_declarations() {
    let duplicate = [CREATE_INPUTS[0], CREATE_INPUTS[0]];
    assert_eq!(
        validate_create_inputs(&duplicate, &serde_json::json!({"required_value": 1})),
        Err(ConstructionError::DuplicateInput(
            "required_value".to_string()
        ))
    );
}

#[test]
fn catalog_custom_construction_metadata_is_well_formed() {
    for entry in inventory::iter::<crate::registry::VariantEntry>() {
        let Some(inputs) = entry.create_inputs else {
            continue;
        };
        let label = variant_label(entry);
        let mut names = BTreeSet::new();
        for input in inputs {
            assert!(
                !input.name.is_empty(),
                "{label} declares an empty construction input name"
            );
            assert!(
                input
                    .name
                    .bytes()
                    .all(|byte| byte == b'_' || byte.is_ascii_lowercase() || byte.is_ascii_digit()),
                "{label} construction input `{}` must use snake_case",
                input.name
            );
            assert!(
                names.insert(input.name),
                "{label} declares construction input `{}` more than once",
                input.name
            );
            assert!(
                !input.type_name.trim().is_empty(),
                "{label} construction input `{}` has no Rust type",
                input.name
            );
            assert_eq!(
                input.description,
                input.description.trim(),
                "{label} construction input `{}` has surrounding whitespace in its description",
                input.name
            );
        }
    }
}

#[test]
fn default_custom_construction_inputs_match_catalog_schema_fields() {
    for entry in inventory::iter::<crate::registry::VariantEntry>()
        .filter(|entry| entry.is_default && entry.create_inputs.is_some())
    {
        let schema = inventory::iter::<crate::registry::ProblemSchemaEntry>()
            .find(|schema| schema.name == entry.name)
            .unwrap_or_else(|| panic!("{} has no ProblemSchemaEntry", entry.name));
        let schema_names = schema
            .fields
            .iter()
            .map(|field| field.name)
            .collect::<BTreeSet<_>>();
        let input_names = entry
            .create_inputs
            .unwrap()
            .iter()
            .map(|input| input.name)
            .collect::<BTreeSet<_>>();
        assert_eq!(
            schema_names,
            input_names,
            "default variant {} catalog fields differ from its construction inputs",
            variant_label(entry)
        );
    }
}

#[test]
fn every_custom_construction_contract_rejects_unknown_and_missing_inputs() {
    for entry in inventory::iter::<crate::registry::VariantEntry>() {
        let Some(inputs) = entry.create_inputs else {
            continue;
        };
        assert_eq!(
            validate_create_inputs(inputs, &serde_json::json!({"unknown_input": null})),
            Err(ConstructionError::UnknownInputs(vec![
                "unknown_input".to_string()
            ])),
            "{} accepted an undeclared construction input",
            variant_label(entry)
        );

        let required = inputs
            .iter()
            .filter(|input| input.required)
            .map(|input| input.name.to_string())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        let result = validate_create_inputs(inputs, &serde_json::json!({}));
        if required.is_empty() {
            assert_eq!(
                result,
                Ok(()),
                "{} rejected an empty payload",
                variant_label(entry)
            );
        } else {
            assert_eq!(
                result,
                Err(ConstructionError::MissingInputs(required)),
                "{} did not report all missing required inputs",
                variant_label(entry)
            );
        }
    }
}

#[test]
fn construction_contract_direct_fields_are_required() {
    let fields = [FieldInfo {
        name: "value",
        type_name: "usize",
        description: "Stored value",
    }];
    assert_eq!(
        validate_direct_create_inputs(&fields, &serde_json::json!({})),
        Err(ConstructionError::MissingInputs(vec!["value".to_string()]))
    );
}

#[test]
fn validate_inner_accepts_valid_aliases() {
    let entries = vec![
        ("Foo {k=K3}".to_string(), &["3FOO"][..]),
        ("Foo {k=K2}".to_string(), &["2FOO"][..]),
    ];
    assert!(validate_aliases_inner(&empty_problem_names(), &entries).is_ok());
}

#[test]
fn validate_inner_rejects_empty_alias() {
    let entries = vec![("Foo {k=K3}".to_string(), &[""][..])];
    let err = validate_aliases_inner(&empty_problem_names(), &entries).unwrap_err();
    assert_eq!(err.len(), 1);
    assert!(
        err[0].contains("empty or whitespace-only"),
        "expected empty alias error, got: {}",
        err[0]
    );
}

#[test]
fn validate_inner_rejects_whitespace_only_alias() {
    let entries = vec![("Foo".to_string(), &["  \t"][..])];
    let err = validate_aliases_inner(&empty_problem_names(), &entries).unwrap_err();
    assert!(err[0].contains("empty or whitespace-only"));
}

#[test]
fn validate_inner_rejects_collision_with_canonical_name() {
    let mut names = BTreeMap::new();
    names
        .entry("bar".to_string())
        .or_insert_with(Vec::new)
        .push("canonical problem name `Bar`".to_string());

    let entries = vec![("Foo {k=K3}".to_string(), &["BAR"][..])];
    let err = validate_aliases_inner(&names, &entries).unwrap_err();
    assert_eq!(err.len(), 1);
    assert!(err[0].contains("conflicts with canonical problem name"));
}

#[test]
fn validate_inner_rejects_collision_with_problem_level_alias() {
    let mut names = BTreeMap::new();
    names
        .entry("baz".to_string())
        .or_insert_with(Vec::new)
        .push("problem-level alias `BAZ` for `Bazinga`".to_string());

    let entries = vec![("Foo".to_string(), &["baz"][..])];
    let err = validate_aliases_inner(&names, &entries).unwrap_err();
    assert_eq!(err.len(), 1);
    assert!(err[0].contains("conflicts with problem-level alias"));
}

#[test]
fn validate_inner_rejects_duplicate_variant_aliases() {
    let entries = vec![
        ("Foo {k=K3}".to_string(), &["DUP"][..]),
        ("Bar {k=K2}".to_string(), &["dup"][..]),
    ];
    let err = validate_aliases_inner(&empty_problem_names(), &entries).unwrap_err();
    assert_eq!(err.len(), 1);
    assert!(
        err[0].contains("duplicate variant-level alias"),
        "expected duplicate error, got: {}",
        err[0]
    );
}

#[test]
fn validate_inner_reports_multiple_conflicts() {
    let entries = vec![
        ("A".to_string(), &[""][..]),
        ("B".to_string(), &["X"][..]),
        ("C".to_string(), &["x"][..]),
    ];
    let err = validate_aliases_inner(&empty_problem_names(), &entries).unwrap_err();
    assert_eq!(err.len(), 2, "expected 2 conflicts, got: {err:?}");
}

// --- variant_label unit tests ---

#[test]
fn variant_label_bare_problem() {
    // Find a VariantEntry with no variant dimensions (empty variant list).
    // QUBO is a standalone problem with no variants.
    let entry = inventory::iter::<crate::registry::VariantEntry>()
        .find(|e| e.variant().is_empty())
        .expect("expected at least one VariantEntry with empty variant");
    let label = variant_label(entry);
    assert_eq!(label, entry.name);
}

#[test]
fn variant_label_with_variant_dimensions() {
    let entry = inventory::iter::<crate::registry::VariantEntry>()
        .find(|e| e.name == "KSatisfiability" && e.aliases.contains(&"3SAT"))
        .expect("expected KSatisfiability<K3> VariantEntry");
    let label = variant_label(entry);
    assert!(
        label.contains("k=K3"),
        "expected label to include k=K3, got: {label}"
    );
}
