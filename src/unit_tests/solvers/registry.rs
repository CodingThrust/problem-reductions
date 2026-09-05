use super::*;
use std::collections::BTreeMap;

const BOOL_VARIANT: &[(&str, &str)] = &[("variable", "bool"), ("coefficient", "i64")];
const FLOAT_BOOL_VARIANT: &[(&str, &str)] = &[("variable", "bool"), ("coefficient", "f64")];
const FLOAT_I64_VARIANT: &[(&str, &str)] = &[("variable", "i64"), ("coefficient", "f64")];
const NO_VARIANT: &[(&str, &str)] = &[];

#[test]
fn generic_decision_ilp_respects_maximization_bounds() {
    use crate::models::decision::Decision;
    use crate::models::graph::MaximumIndependentSet;
    use crate::solvers::BruteForce;
    use crate::topology::SimpleGraph;

    // Exercise the same generic decision edge without adding a production solver registration.
    static PIPELINE: IlpPipelineRegistration = IlpPipelineRegistration {
        path: &[
            StaticProblemStep {
                name: "DecisionMaximumIndependentSet",
                variant: &[("graph", "SimpleGraph"), ("weight", "i64")],
            },
            StaticProblemStep {
                name: "MaximumIndependentSet",
                variant: &[("graph", "SimpleGraph"), ("weight", "i64")],
            },
            StaticProblemStep {
                name: "MaximumSetPacking",
                variant: &[("weight", "i64")],
            },
            StaticProblemStep {
                name: "ILP",
                variant: BOOL_VARIANT,
            },
            StaticProblemStep {
                name: "ILP",
                variant: FLOAT_BOOL_VARIANT,
            },
        ],
    };
    let registry = build_registry(
        &registered_variant_keys(),
        inventory::iter::<CustomizedSolverRegistration>(),
        inventory::iter::<IlpPipelineRegistration>().chain([&PIPELINE]),
        inventory::iter::<crate::solvers::BruteForceRegistration>(),
        &reduction_entries(),
    )
    .unwrap();
    let source = ExactProblemKey::from_static(&PIPELINE.path[0]);
    let pipeline = registry.lookup(&source).ilp.unwrap();
    let inner = MaximumIndependentSet::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![1i64; 3],
    );
    for bound in [0, 1, 2] {
        let decision = Decision::new(inner.clone(), bound);
        let result = pipeline
            .solve(&decision, &crate::solvers::ILPSolver::new())
            .unwrap();
        assert_eq!(result.is_some(), bound <= 1);
        assert_eq!(
            result.is_some(),
            BruteForce::new().solve(&decision).unwrap().is_some()
        );
        if let Some(solution) = result {
            let solution: Vec<bool> = serde_json::from_value(solution).unwrap();
            assert_eq!(
                crate::traits::Problem::evaluate(&decision, &solution).unwrap(),
                crate::types::Or(true)
            );
        }
    }
}

#[test]
fn generic_decision_ilp_skips_no_witness_but_preserves_extraction_errors() {
    use crate::models::decision::Decision;
    use crate::models::graph::MinimumVertexCover;
    use crate::rules::{ExtractionError, ReductionResult};
    use crate::solvers::{ILPSolveError, ILPSolver};
    use crate::topology::SimpleGraph;
    use crate::traits::Problem;

    type Inner = MinimumVertexCover<SimpleGraph, i64>;
    struct BrokenExtractor(Inner);
    impl ReductionResult for BrokenExtractor {
        type Source = Decision<Inner>;
        type Target = Inner;

        fn target_problem(&self) -> &Inner {
            &self.0
        }

        fn extract_solution(&self, _: &Vec<bool>) -> crate::rules::ExtractionResult<Vec<bool>> {
            Err(ExtractionError::invalid("broken witness decoder"))
        }
    }

    let source = ExactProblemKey::new(
        Decision::<Inner>::NAME,
        Decision::<Inner>::variant()
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect(),
    );
    let registry = solver_capability_registry().unwrap();
    let original = registry.lookup(&source).ilp.unwrap();
    let mut pipeline = CompiledIlpPipeline {
        path: original.path.clone(),
        reducers: original.reducers.clone(),
    };
    pipeline.reducers[0].0 = |source| {
        let source = source.downcast_ref::<Decision<Inner>>().unwrap();
        Ok(Box::new(BrokenExtractor(source.inner().clone())))
    };
    let inner = Inner::new(SimpleGraph::new(2, vec![(0, 1)]), vec![1i64; 2]);
    assert_eq!(
        pipeline
            .solve(&Decision::new(inner.clone(), 0), &ILPSolver::new())
            .unwrap(),
        None
    );
    assert!(matches!(
        pipeline.solve(&Decision::new(inner, 1), &ILPSolver::new()),
        Err(ILPSolveError::Extraction(ExtractionError::Reduction { message, .. }))
            if message == "broken witness decoder"
    ));
}

static DIRECT_BOOL_A: IlpPipelineRegistration = IlpPipelineRegistration {
    path: &[StaticProblemStep {
        name: "ILP",
        variant: FLOAT_BOOL_VARIANT,
    }],
};
static DIRECT_BOOL_B: IlpPipelineRegistration = IlpPipelineRegistration {
    path: &[StaticProblemStep {
        name: "ILP",
        variant: FLOAT_BOOL_VARIANT,
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
            variant: FLOAT_BOOL_VARIANT,
        },
    ],
};
static CONTINUES_AFTER_ILP: IlpPipelineRegistration = IlpPipelineRegistration {
    path: &[
        StaticProblemStep {
            name: "ILP",
            variant: FLOAT_BOOL_VARIANT,
        },
        StaticProblemStep {
            name: "ILP",
            variant: FLOAT_I64_VARIANT,
        },
    ],
};
static EMPTY_PIPELINE: IlpPipelineRegistration = IlpPipelineRegistration { path: &[] };
static UNSUPPORTED_TARGET: IlpPipelineRegistration = IlpPipelineRegistration {
    path: &[StaticProblemStep {
        name: "Source",
        variant: NO_VARIANT,
    }],
};

fn source_variant() -> Vec<(&'static str, &'static str)> {
    Vec::new()
}

fn no_solution(
    _: &dyn std::any::Any,
) -> Result<Option<serde_json::Value>, crate::solvers::SolveError> {
    Ok(None)
}

static CUSTOMIZED_A: CustomizedSolverRegistration = CustomizedSolverRegistration {
    source_name: "Source",
    source_variant_fn: source_variant,
    implementation: "customized-a",
    solve_fn: no_solution,
};
static CUSTOMIZED_B: CustomizedSolverRegistration = CustomizedSolverRegistration {
    source_name: "Source",
    source_variant_fn: source_variant,
    implementation: "customized-b",
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
        BTreeMap::from([
            ("variable".to_string(), "bool".to_string()),
            ("coefficient".to_string(), "f64".to_string()),
        ]),
    )]);
    for pipelines in [
        [&DIRECT_BOOL_A, &DIRECT_BOOL_B],
        [&DIRECT_BOOL_B, &DIRECT_BOOL_A],
    ] {
        let error = build_registry(
            &variants,
            std::iter::empty(),
            pipelines,
            std::iter::empty(),
            &[],
        )
        .unwrap_err();
        assert!(matches!(error, RegistryBuildError::DuplicateIlp(_)));
    }
}

#[test]
fn solver_capability_registry_duplicate_customized_registration_is_rejected() {
    let variants = BTreeSet::from([ExactProblemKey::new("Source", BTreeMap::new())]);
    let error = build_registry(
        &variants,
        [&CUSTOMIZED_A, &CUSTOMIZED_B],
        std::iter::empty(),
        std::iter::empty(),
        &[],
    )
    .unwrap_err();
    assert!(matches!(error, RegistryBuildError::DuplicateCustomized(_)));
}

#[test]
fn solver_capability_registry_unknown_customized_variant_is_rejected() {
    let error = build_registry(
        &BTreeSet::new(),
        [&CUSTOMIZED_A],
        std::iter::empty(),
        std::iter::empty(),
        &[],
    )
    .unwrap_err();
    assert!(matches!(error, RegistryBuildError::UnknownVariant(label) if label == "Source"));
}

#[test]
fn solver_capability_registry_unknown_pipeline_variant_is_rejected() {
    let variants = BTreeSet::from([ExactProblemKey::new(
        "ILP",
        BTreeMap::from([
            ("variable".to_string(), "bool".to_string()),
            ("coefficient".to_string(), "i64".to_string()),
        ]),
    )]);
    let error = build_registry(
        &variants,
        std::iter::empty(),
        [&MISSING_EDGE],
        std::iter::empty(),
        &[],
    )
    .unwrap_err();
    assert!(matches!(error, RegistryBuildError::UnknownVariant(label) if label == "Source"));
}

#[test]
fn solver_capability_registry_empty_pipeline_is_rejected() {
    let error = build_registry(
        &BTreeSet::new(),
        std::iter::empty(),
        [&EMPTY_PIPELINE],
        std::iter::empty(),
        &[],
    )
    .unwrap_err();
    assert!(matches!(error, RegistryBuildError::EmptyPipeline));
}

#[test]
fn solver_capability_registry_unsupported_pipeline_target_is_rejected() {
    let variants = BTreeSet::from([ExactProblemKey::new("Source", BTreeMap::new())]);
    let error = build_registry(
        &variants,
        std::iter::empty(),
        [&UNSUPPORTED_TARGET],
        std::iter::empty(),
        &[],
    )
    .unwrap_err();
    assert!(matches!(error, RegistryBuildError::UnsupportedTarget(label) if label == "Source"));
}

#[test]
fn solver_capability_registry_pipeline_with_missing_exact_edge_is_rejected() {
    let variants = BTreeSet::from([
        ExactProblemKey::new("Source", BTreeMap::new()),
        ExactProblemKey::new(
            "ILP",
            BTreeMap::from([
                ("variable".to_string(), "bool".to_string()),
                ("coefficient".to_string(), "f64".to_string()),
            ]),
        ),
    ]);
    let error = build_registry(
        &variants,
        std::iter::empty(),
        [&MISSING_EDGE],
        std::iter::empty(),
        &[],
    )
    .unwrap_err();
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
            BTreeMap::from([
                ("variable".to_string(), "bool".to_string()),
                ("coefficient".to_string(), "f64".to_string()),
            ]),
        ),
        ExactProblemKey::new(
            "ILP",
            BTreeMap::from([
                ("variable".to_string(), "i64".to_string()),
                ("coefficient".to_string(), "f64".to_string()),
            ]),
        ),
    ]);
    let error = build_registry(
        &variants,
        std::iter::empty(),
        [&CONTINUES_AFTER_ILP],
        std::iter::empty(),
        &[],
    )
    .unwrap_err();
    assert!(matches!(error, RegistryBuildError::ContinuesAfterIlp(_)));
}

#[test]
fn solver_capability_registry_rejects_variant_without_solver() {
    let variants = BTreeSet::from([ExactProblemKey::new("Source", BTreeMap::new())]);
    let error = build_registry(
        &variants,
        std::iter::empty(),
        std::iter::empty(),
        std::iter::empty(),
        &[],
    )
    .unwrap_err();
    assert!(matches!(
        error,
        RegistryBuildError::MissingSolverCapability(label) if label == "Source"
    ));
}

#[test]
fn solver_capability_registry_exposes_representative_capability_classes() {
    let key = |name: &str, variant: &[(&str, &str)]| {
        ExactProblemKey::new(
            name,
            variant
                .iter()
                .map(|&(key, value)| (key.to_string(), value.to_string()))
                .collect(),
        )
    };

    let customized_only = solver_capabilities(&key("TimetableDesign", &[])).unwrap();
    assert_eq!(
        customized_only.customized.unwrap().implementation,
        "timetable-required-assignments"
    );
    assert!(customized_only.ilp.is_none());

    let direct_ilp = solver_capabilities(&key(
        "MaximumClique",
        &[("graph", "SimpleGraph"), ("weight", "i64")],
    ))
    .unwrap();
    assert!(direct_ilp.customized.is_none());
    assert_eq!(
        direct_ilp.ilp.unwrap().path_labels(),
        [
            "MaximumClique<SimpleGraph, i64>",
            "ILP<i64, bool>",
            "ILP<f64, bool>"
        ]
    );

    let multihop_ilp = solver_capabilities(&key(
        "MaximumIndependentSet",
        &[("graph", "SimpleGraph"), ("weight", "One")],
    ))
    .unwrap();
    assert!(multihop_ilp.ilp.unwrap().path_labels().len() > 2);

    let both =
        solver_capabilities(&key("RootedTreeArrangement", &[("graph", "SimpleGraph")])).unwrap();
    assert!(both.customized.is_some());
    assert!(both.ilp.is_some());

    let brute_force_only = solver_capabilities(&key(
        "MaxCut",
        &[("graph", "SimpleGraph"), ("weight", "i64")],
    ))
    .unwrap();
    assert!(brute_force_only.customized.is_none());
    assert!(brute_force_only.ilp.is_none());

    let ilp_itself =
        solver_capabilities(&key("ILP", &[("variable", "bool"), ("coefficient", "i64")])).unwrap();
    assert_eq!(
        ilp_itself.ilp.unwrap().path_labels(),
        ["ILP<i64, bool>", "ILP<f64, bool>"]
    );
}

#[test]
fn solver_capability_registry_does_not_leak_across_exact_variants() {
    let registry = solver_capability_registry().unwrap();
    let key = ExactProblemKey::new(
        "MinimumCardinalityKey",
        BTreeMap::from([("unexpected".to_string(), "variant".to_string())]),
    );
    let capabilities = registry.lookup(&key);
    assert!(capabilities.customized.is_none());
    assert!(capabilities.ilp.is_none());
}

#[test]
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

    let variants: BTreeSet<_> = path.iter().cloned().collect();
    let pipelines = inventory::iter::<IlpPipelineRegistration>
        .into_iter()
        .filter(|entry| {
            entry
                .path
                .first()
                .map(ExactProblemKey::from_static)
                .is_some_and(|source| variants.contains(&source))
        })
        .collect::<Vec<_>>();
    let brute_force = inventory::iter::<crate::solvers::BruteForceRegistration>
        .into_iter()
        .filter(|entry| {
            variants.contains(&ExactProblemKey::new(
                entry.source_name,
                crate::export::variant_to_map((entry.source_variant_fn)()),
            ))
        })
        .collect::<Vec<_>>();
    let minimal = build_registry(
        &variants,
        std::iter::empty(),
        pipelines.iter().copied(),
        brute_force.iter().copied(),
        &required_reductions,
    )
    .unwrap();
    let expanded = build_registry(
        &variants,
        std::iter::empty(),
        pipelines.iter().copied(),
        brute_force.iter().copied(),
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
            .map(|(reducer, aggregate)| (
                *reducer as usize,
                aggregate.map(|reduce| reduce as usize)
            ))
            .collect::<Vec<_>>(),
        expanded_pipeline
            .reducers
            .iter()
            .map(|(reducer, aggregate)| (
                *reducer as usize,
                aggregate.map(|reduce| reduce as usize)
            ))
            .collect::<Vec<_>>()
    );
}

#[test]
fn solver_capability_registry_ambiguous_exact_edge_is_rejected() {
    let registration = inventory::iter::<IlpPipelineRegistration>
        .into_iter()
        .find(|registration| registration.path.len() == 2)
        .expect("production catalog must contain a direct ILP pipeline");
    let path = registration
        .path
        .iter()
        .map(ExactProblemKey::from_static)
        .collect::<Vec<_>>();
    let reduction = reduction_entries()
        .into_iter()
        .find(|entry| {
            entry.capabilities().witness
                && entry.reduce_fn.is_some()
                && edge_key(entry, true) == path[0]
                && edge_key(entry, false) == path[1]
        })
        .expect("direct pipeline must have one witness reduction");
    let error = build_registry(
        &registered_variant_keys(),
        std::iter::empty(),
        [registration],
        std::iter::empty(),
        &[reduction, reduction],
    )
    .unwrap_err();

    assert!(matches!(
        error,
        RegistryBuildError::InvalidEdge { matches: 2, .. }
    ));
}
