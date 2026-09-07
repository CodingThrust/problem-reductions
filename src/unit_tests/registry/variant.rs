use crate::registry::variant::{
    validate_create_inputs, validate_direct_create_inputs, validate_variant_aliases,
    variant_entries, variant_label,
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
        let inputs = inputs();
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
fn every_custom_construction_contract_rejects_unknown_and_missing_inputs() {
    for entry in inventory::iter::<crate::registry::VariantEntry>() {
        let Some(inputs) = entry.create_inputs else {
            continue;
        };
        let inputs = inputs();
        assert_eq!(
            validate_create_inputs(&inputs, &serde_json::json!({"unknown_input": null})),
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
        let result = validate_create_inputs(&inputs, &serde_json::json!({}));
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

#[test]
fn random_contract_input_names_are_unique() {
    let entries = variant_entries();
    assert!(entries.iter().any(|entry| entry.random.is_some()));

    for entry in entries {
        let Some(random) = entry.random else {
            continue;
        };
        let mut names = BTreeSet::new();
        for input in (random.inputs)() {
            assert!(
                !input.name.is_empty(),
                "{} has an empty random input",
                variant_label(entry)
            );
            assert!(
                names.insert(input.name),
                "{} declares random input `{}` more than once",
                variant_label(entry),
                input.name
            );
        }
    }
}

#[test]
fn established_random_generation_models_remain_registered() {
    let expected = "
        DecisionMinimumVertexCover MaximumIndependentSet MinimumVertexCover MaximumClique
        MinimumDominatingSet MaximalIS KClique MinimumCutIntoBoundedSets HamiltonianCircuit
        HamiltonianPath HamiltonianPathBetweenTwoVertices LongestCircuit MinimumMaximalMatching
        RootedTreeArrangement SteinerTree SteinerTreeInGraphs LengthBoundedDisjointPaths
        MaximumAchromaticNumber MaximumDomaticNumber MinimumCoveringByCliques
        MinimumIntersectionGraphBasis MaximumLeafSpanningTree GeneralizedHex
        BottleneckTravelingSalesman MaxCut MaximumMatching TravelingSalesman SpinGlass KColoring
        OptimalLinearArrangement MinimumSumMulticenter
    ";
    let registered = variant_entries()
        .into_iter()
        .filter(|entry| entry.random.is_some())
        .map(|entry| entry.name)
        .collect::<BTreeSet<_>>();

    for name in expected.split_whitespace() {
        assert!(registered.contains(name), "{name} lost random generation");
    }
}

#[test]
fn unit_variants_construct_without_unit_inputs() {
    use serde_json::json;
    let graph = json!({"num_vertices": 3, "edges": [[0,1],[1,2]]});
    for entry in crate::registry::variant_entries()
        .into_iter()
        .filter(|entry| entry.variant().iter().any(|&(_, value)| value == "One"))
    {
        let label = variant_label(entry);
        let inputs = entry.inputs();
        assert!(
            !inputs.iter().any(|input| matches!(
                input.name,
                "weights"
                    | "edge_weights"
                    | "arc_weights"
                    | "r_weights"
                    | "s_weights"
                    | "lengths"
                    | "edge_lengths"
            )),
            "{label} exposes unit inputs"
        );
        let data = match entry.name {
            "MinimumVertexCover" | "MaximumClique" | "MinimumDominatingSet" => {
                json!({"graph":graph})
            }
            "MaximumCoKPlex" => json!({"graph":graph,"k":1}),
            "MinimumFeedbackVertexSet" => json!({"graph":{"num_vertices":3,"arcs":[[0,1],[1,2]]}}),
            "MaximumSetPacking" => json!({"subsets":[[0,1],[1,2]]}),
            "SteinerTree" | "SteinerTreeInGraphs" => json!({"graph":graph,"terminals":[0,2]}),
            "MaximumIndependentSet" => match entry.variant_map()["graph"].as_str() {
                "SimpleGraph" => json!({"graph":[[0,1],[1,2]]}),
                "KingsSubgraph" => json!({"positions":[[0,0],[1,0],[2,0]]}),
                "UnitDiskGraph" => {
                    json!({"positions":[[0.0,0.0],[1.0,0.0],[2.0,0.0]],"radius":1.1})
                }
                graph => panic!("missing construction case for {graph}"),
            },
            "DecisionMaximumIndependentSet" => json!({"graph":[[0,1],[1,2]],"bound":2}),
            "DecisionMinimumDominatingSet" => json!({"graph":graph,"bound":1}),
            "MaxCut" => json!({"graph":[[0,1],[1,2]]}),
            "LongestPath" => json!({"graph":[[0,1],[1,2]],"source_vertex":0,"target_vertex":2}),
            "MinMaxMulticenter" => json!({"graph":[[0,1],[1,2]],"k":1}),
            "MixedChinesePostman" => json!({"graph":[[0,1],[1,2]],"arcs":[[2,0]]}),
            "ComparativeContainment" => {
                json!({"universe_size":3,"r_sets":[[0,1]],"s_sets":[[1,2]]})
            }
            "MinimumTardinessSequencing" => json!({"deadlines":[1,2,3]}),
            name => panic!("missing unit construction case for {name}"),
        };
        let problem =
            (entry.construct_fn)(data.clone()).unwrap_or_else(|error| panic!("{label}: {error}"));
        assert_eq!(problem.variant_map(), entry.variant_map());
        let serialized = problem.serialize_json();
        let inner = serialized.get("inner").unwrap_or(&serialized);
        let unit_fields: Vec<_> = [
            "weights",
            "edge_weights",
            "arc_weights",
            "r_weights",
            "s_weights",
            "lengths",
            "edge_lengths",
        ]
        .into_iter()
        .filter(|field| inner.get(field).is_some())
        .collect();
        assert!(
            !unit_fields.is_empty(),
            "{label} has no unit values to check"
        );
        for field in unit_fields {
            let values = inner[field].as_array().unwrap();
            assert!(!values.is_empty());
            assert!(
                values.iter().all(|value| value == &json!(1)),
                "{label}: {field}"
            );
            let mut redundant = data.clone();
            redundant[field] = inner[field].clone();
            assert!(
                matches!(
                    (entry.construct_fn)(redundant),
                    Err(ConstructionError::UnknownInputs(_))
                ),
                "{label} accepted redundant {field}"
            );
        }
    }
}

#[test]
fn unit_construction_preserves_model_validation() {
    use serde_json::json;
    let graph = json!({"num_vertices":3,"edges":[[0,1],[1,2]]});
    for (name, data) in [
        ("MaximumCoKPlex", json!({"graph":graph,"k":0})),
        ("SteinerTree", json!({"graph":graph,"terminals":[0]})),
        ("SteinerTree", json!({"graph":graph,"terminals":[0,0]})),
        ("SteinerTree", json!({"graph":graph,"terminals":[0,3]})),
        (
            "SteinerTreeInGraphs",
            json!({"graph":graph,"terminals":[3]}),
        ),
        (
            "MinimumTardinessSequencing",
            json!({"deadlines":[1,2],"precedences":[[0,2]]}),
        ),
        (
            "DecisionMaximumIndependentSet",
            json!({"graph":[[0,0]],"bound":1}),
        ),
    ] {
        let entry = crate::registry::variant_entries()
            .into_iter()
            .find(|entry| entry.name == name && entry.variant().iter().any(|&(_, v)| v == "One"))
            .unwrap();
        assert!(
            (entry.construct_fn)(data).is_err(),
            "{name} accepted invalid inputs"
        );
    }
}
