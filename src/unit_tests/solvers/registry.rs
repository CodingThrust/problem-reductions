use super::*;
use std::collections::BTreeMap;

const BOOL_VARIANT: &[(&str, &str)] = &[("variable", "bool")];
const I32_VARIANT: &[(&str, &str)] = &[("variable", "i32")];
const NO_VARIANT: &[(&str, &str)] = &[];

static DIRECT_BOOL_A: IlpPipelineRegistration = IlpPipelineRegistration {
    path: &[StaticProblemStep {
        name: "ILP",
        variant: BOOL_VARIANT,
    }],
};
static DIRECT_BOOL_B: IlpPipelineRegistration = IlpPipelineRegistration {
    path: &[StaticProblemStep {
        name: "ILP",
        variant: BOOL_VARIANT,
    }],
};
static MISSING_EDGE: IlpPipelineRegistration = IlpPipelineRegistration {
    path: &[
        StaticProblemStep {
            name: "Source",
            variant: NO_VARIANT,
        },
        StaticProblemStep {
            name: "ILP",
            variant: BOOL_VARIANT,
        },
    ],
};
static CONTINUES_AFTER_ILP: IlpPipelineRegistration = IlpPipelineRegistration {
    path: &[
        StaticProblemStep {
            name: "ILP",
            variant: BOOL_VARIANT,
        },
        StaticProblemStep {
            name: "ILP",
            variant: I32_VARIANT,
        },
    ],
};

fn source_variant() -> Vec<(&'static str, &'static str)> {
    Vec::new()
}

fn no_solution(_: &dyn std::any::Any) -> Option<Vec<usize>> {
    None
}

static NATIVE_A: NativeSolverRegistration = NativeSolverRegistration {
    source_name: "Source",
    source_variant_fn: source_variant,
    implementation: "native-a",
    solve_fn: no_solution,
};
static NATIVE_B: NativeSolverRegistration = NativeSolverRegistration {
    source_name: "Source",
    source_variant_fn: source_variant,
    implementation: "native-b",
    solve_fn: no_solution,
};

#[test]
fn solver_capability_registry_constructs_without_graph_search() {
    solver_capability_registry().expect("production solver registrations must be valid");
}

#[test]
fn exact_problem_key_has_canonical_label() {
    let key = ExactProblemKey::new(
        "MaximumIndependentSet",
        BTreeMap::from([
            ("graph".to_string(), "SimpleGraph".to_string()),
            ("weight".to_string(), "One".to_string()),
        ]),
    );
    assert_eq!(key.label(), "MaximumIndependentSet<SimpleGraph, One>");
}

#[test]
fn solver_capability_registry_duplicate_ilp_registration_is_rejected_independent_of_order() {
    let variants = BTreeSet::from([ExactProblemKey::new(
        "ILP",
        BTreeMap::from([("variable".to_string(), "bool".to_string())]),
    )]);
    for pipelines in [
        [&DIRECT_BOOL_A, &DIRECT_BOOL_B],
        [&DIRECT_BOOL_B, &DIRECT_BOOL_A],
    ] {
        let error = build_registry(&variants, std::iter::empty(), pipelines, &[]).unwrap_err();
        assert!(matches!(error, RegistryBuildError::DuplicateIlp(_)));
    }
}

#[test]
fn solver_capability_registry_duplicate_native_registration_is_rejected() {
    let variants = BTreeSet::from([ExactProblemKey::new("Source", BTreeMap::new())]);
    let error =
        build_registry(&variants, [&NATIVE_A, &NATIVE_B], std::iter::empty(), &[]).unwrap_err();
    assert!(matches!(error, RegistryBuildError::DuplicateNative(_)));
}

#[test]
fn solver_capability_registry_pipeline_with_missing_exact_edge_is_rejected() {
    let variants = BTreeSet::from([
        ExactProblemKey::new("Source", BTreeMap::new()),
        ExactProblemKey::new(
            "ILP",
            BTreeMap::from([("variable".to_string(), "bool".to_string())]),
        ),
    ]);
    let error = build_registry(&variants, std::iter::empty(), [&MISSING_EDGE], &[]).unwrap_err();
    assert!(matches!(
        error,
        RegistryBuildError::InvalidEdge { matches: 0, .. }
    ));
}

#[test]
fn solver_capability_registry_pipeline_must_stop_at_first_supported_ilp_node() {
    let variants = BTreeSet::from([
        ExactProblemKey::new(
            "ILP",
            BTreeMap::from([("variable".to_string(), "bool".to_string())]),
        ),
        ExactProblemKey::new(
            "ILP",
            BTreeMap::from([("variable".to_string(), "i32".to_string())]),
        ),
    ]);
    let error =
        build_registry(&variants, std::iter::empty(), [&CONTINUES_AFTER_ILP], &[]).unwrap_err();
    assert!(matches!(error, RegistryBuildError::ContinuesAfterIlp(_)));
}

#[test]
fn solver_capability_registry_production_registry_has_expected_exact_capability_counts() {
    let registry = solver_capability_registry().unwrap();
    assert_eq!(registry.native_entries().count(), 7);
    #[cfg(feature = "ilp-solver")]
    assert_eq!(registry.ilp_entries().count(), 151);
}

#[test]
fn solver_capability_registry_does_not_leak_across_exact_variants() {
    let registry = solver_capability_registry().unwrap();
    let key = ExactProblemKey::new(
        "MinimumCardinalityKey",
        BTreeMap::from([("unexpected".to_string(), "variant".to_string())]),
    );
    let capabilities = registry.lookup(&key);
    assert!(capabilities.native.is_none());
    assert!(capabilities.ilp.is_none());
}

#[test]
#[cfg(feature = "ilp-solver")]
fn solver_capability_registry_ignores_unrelated_reduction_edges() {
    let source = ExactProblemKey::new(
        "MaximumIndependentSet",
        BTreeMap::from([
            ("graph".to_string(), "SimpleGraph".to_string()),
            ("weight".to_string(), "One".to_string()),
        ]),
    );
    let registration = inventory::iter::<IlpPipelineRegistration>
        .into_iter()
        .find(|registration| {
            registration.path.first().map(ExactProblemKey::from_static) == Some(source.clone())
        })
        .expect("production MIS<One> pipeline must be registered");
    let path = registration
        .path
        .iter()
        .map(ExactProblemKey::from_static)
        .collect::<Vec<_>>();
    let all_reductions = reduction_entries();
    let required_reductions = all_reductions
        .iter()
        .copied()
        .filter(|entry| {
            path.windows(2)
                .any(|pair| edge_key(entry, true) == pair[0] && edge_key(entry, false) == pair[1])
        })
        .collect::<Vec<_>>();
    let unrelated = all_reductions
        .iter()
        .copied()
        .find(|entry| {
            !required_reductions
                .iter()
                .any(|required| std::ptr::eq(*required, *entry))
        })
        .expect("catalog must contain an unrelated reduction edge");
    let mut with_unrelated = required_reductions.clone();
    with_unrelated.push(unrelated);

    let variants = registered_variant_keys();
    let minimal = build_registry(
        &variants,
        std::iter::empty(),
        [registration],
        &required_reductions,
    )
    .unwrap();
    let expanded = build_registry(
        &variants,
        std::iter::empty(),
        [registration],
        &with_unrelated,
    )
    .unwrap();
    let minimal_pipeline = minimal.lookup(&source).ilp.unwrap();
    let expanded_pipeline = expanded.lookup(&source).ilp.unwrap();

    assert_eq!(minimal_pipeline.path(), expanded_pipeline.path());
    assert_eq!(
        minimal_pipeline
            .reducers
            .iter()
            .map(|reducer| *reducer as usize)
            .collect::<Vec<_>>(),
        expanded_pipeline
            .reducers
            .iter()
            .map(|reducer| *reducer as usize)
            .collect::<Vec<_>>()
    );
}
