use crate::example_db::{
    build_example_db, build_model_db, build_rule_db, find_model_example, find_rule_example,
};
use crate::export::ProblemRef;
use crate::registry::load_dyn;
use crate::rules::{registry::reduction_entries, ReductionGraph};
use std::collections::{BTreeMap, BTreeSet, HashSet};

#[test]
fn test_build_model_db_contains_curated_examples() {
    let db = build_model_db().expect("model db should build");
    assert!(!db.models.is_empty(), "model db should not be empty");
    assert!(
        db.models
            .iter()
            .any(|model| model.problem == "MaximumIndependentSet"),
        "model db should include a canonical MaximumIndependentSet example"
    );
}

#[test]
fn test_build_example_db_contains_models_and_rules() {
    let db = build_example_db().expect("example db should build");
    assert!(!db.models.is_empty(), "example db should contain models");
    assert!(!db.rules.is_empty(), "example db should contain rules");
}

#[test]
fn test_find_model_example_mis_simplegraph_i32() {
    let problem = ProblemRef {
        name: "MaximumIndependentSet".to_string(),
        variant: BTreeMap::from([
            ("graph".to_string(), "SimpleGraph".to_string()),
            ("weight".to_string(), "i32".to_string()),
        ]),
    };

    let example = find_model_example(&problem).expect("MIS example should exist");
    assert_eq!(example.problem, "MaximumIndependentSet");
    assert_eq!(example.variant, problem.variant);
    assert!(example.instance.is_object());
    assert!(
        !example.optimal_config.is_empty(),
        "canonical example should include optima"
    );
}

#[test]
fn test_find_model_example_exact_cover_by_3_sets() {
    let problem = ProblemRef {
        name: "ExactCoverBy3Sets".to_string(),
        variant: BTreeMap::new(),
    };

    let example = find_model_example(&problem).expect("X3C example should exist");
    assert_eq!(example.problem, "ExactCoverBy3Sets");
    assert_eq!(example.variant, problem.variant);
    assert!(example.instance.is_object());
    assert!(
        !example.optimal_config.is_empty(),
        "canonical example should include satisfying assignments"
    );
}

#[test]
fn test_find_model_example_staff_scheduling() {
    let problem = ProblemRef {
        name: "StaffScheduling".to_string(),
        variant: BTreeMap::new(),
    };

    let example = find_model_example(&problem).expect("StaffScheduling example should exist");
    assert_eq!(example.problem, "StaffScheduling");
    assert_eq!(example.variant, problem.variant);
    assert_eq!(example.instance["num_workers"], 4);
    assert!(example.instance["schedules"].is_array());
    assert!(
        !example.optimal_config.is_empty(),
        "canonical example should include satisfying assignments"
    );
}

#[test]
fn test_find_model_example_stacker_crane() {
    let problem = ProblemRef {
        name: "StackerCrane".to_string(),
        variant: BTreeMap::new(),
    };

    let example = find_model_example(&problem).expect("StackerCrane example should exist");
    assert_eq!(example.problem, "StackerCrane");
    assert_eq!(example.variant, problem.variant);
    assert_eq!(example.optimal_config, vec![0, 2, 1, 4, 3]);
    assert_eq!(example.instance["num_vertices"], 6);
    assert_eq!(example.instance["arcs"].as_array().unwrap().len(), 5);
}

#[test]
fn test_find_model_example_multiprocessor_scheduling() {
    let problem = ProblemRef {
        name: "MultiprocessorScheduling".to_string(),
        variant: BTreeMap::new(),
    };

    let example = find_model_example(&problem).expect("MultiprocessorScheduling example exists");
    assert_eq!(example.problem, "MultiprocessorScheduling");
    assert_eq!(example.variant, problem.variant);
    assert!(example.instance.is_object());
    assert!(
        !example.optimal_config.is_empty(),
        "canonical example should include satisfying assignments"
    );
}

#[test]
fn test_find_model_example_job_shop_scheduling() {
    let problem = ProblemRef {
        name: "JobShopScheduling".to_string(),
        variant: BTreeMap::new(),
    };

    let example = find_model_example(&problem).expect("JobShopScheduling example exists");
    assert_eq!(example.problem, "JobShopScheduling");
    assert_eq!(example.variant, problem.variant);
    assert_eq!(example.instance["num_processors"], 2);
    assert!(example.instance["jobs"].is_array());
    assert_eq!(
        example.optimal_config,
        vec![0, 0, 0, 0, 0, 0, 1, 3, 0, 1, 1, 0]
    );
}

#[test]
fn test_find_model_example_integral_flow_bundles() {
    let problem = ProblemRef {
        name: "IntegralFlowBundles".to_string(),
        variant: BTreeMap::new(),
    };

    let example = find_model_example(&problem).expect("IntegralFlowBundles example exists");
    assert_eq!(example.problem, "IntegralFlowBundles");
    assert_eq!(example.variant, problem.variant);
    assert_eq!(example.instance["graph"]["num_vertices"], 4);
    assert_eq!(example.instance["requirement"], 1);
    assert_eq!(example.optimal_config, vec![1, 0, 1, 0, 0, 0]);
}

#[test]
fn test_find_model_example_strong_connectivity_augmentation() {
    let problem = ProblemRef {
        name: "StrongConnectivityAugmentation".to_string(),
        variant: BTreeMap::from([("weight".to_string(), "i32".to_string())]),
    };

    let example = find_model_example(&problem).expect("SCA example should exist");
    assert_eq!(example.problem, "StrongConnectivityAugmentation");
    assert_eq!(example.variant, problem.variant);
    assert!(example.instance.is_object());
    assert!(
        !example.optimal_config.is_empty(),
        "canonical example should include satisfying assignments"
    );
}

#[test]
fn test_find_model_example_integral_flow_homologous_arcs() {
    let problem = ProblemRef {
        name: "IntegralFlowHomologousArcs".to_string(),
        variant: BTreeMap::new(),
    };

    let example = find_model_example(&problem).expect("IntegralFlowHomologousArcs example exists");
    assert_eq!(example.problem, "IntegralFlowHomologousArcs");
    assert_eq!(example.variant, problem.variant);
    assert_eq!(example.instance["requirement"], 2);
    assert_eq!(
        example.instance["homologous_pairs"],
        serde_json::json!([[2, 5], [4, 3]])
    );
    assert_eq!(example.optimal_config, vec![1, 1, 1, 0, 0, 1, 1, 1]);
}

#[test]
fn test_find_model_example_minimum_dummy_activities_pert() {
    let problem = ProblemRef {
        name: "MinimumDummyActivitiesPert".to_string(),
        variant: BTreeMap::new(),
    };

    let example = find_model_example(&problem).expect("MinimumDummyActivitiesPert example exists");
    assert_eq!(example.problem, "MinimumDummyActivitiesPert");
    assert_eq!(example.variant, problem.variant);
    assert!(example.instance.is_object());
    assert_eq!(example.optimal_value, serde_json::json!(2));
    assert!(
        !example.optimal_config.is_empty(),
        "canonical example should include an optimal merge selection"
    );
}

#[test]
fn test_find_rule_example_mvc_to_mis_contains_full_problem_json() {
    let source = ProblemRef {
        name: "MinimumVertexCover".to_string(),
        variant: BTreeMap::from([
            ("graph".to_string(), "SimpleGraph".to_string()),
            ("weight".to_string(), "i32".to_string()),
        ]),
    };
    let target = ProblemRef {
        name: "MaximumIndependentSet".to_string(),
        variant: BTreeMap::from([
            ("graph".to_string(), "SimpleGraph".to_string()),
            ("weight".to_string(), "i32".to_string()),
        ]),
    };

    let example = find_rule_example(&source, &target).unwrap();
    assert!(example.source.instance.get("graph").is_some());
    assert!(example.target.instance.get("graph").is_some());
}

#[test]
fn test_find_rule_example_sat_to_kcoloring_contains_full_instances() {
    let source = ProblemRef {
        name: "Satisfiability".to_string(),
        variant: BTreeMap::new(),
    };
    let target = ProblemRef {
        name: "KColoring".to_string(),
        variant: BTreeMap::from([
            ("graph".to_string(), "SimpleGraph".to_string()),
            ("k".to_string(), "K3".to_string()),
        ]),
    };

    let example = find_rule_example(&source, &target).unwrap();
    assert!(
        example.source.instance.get("clauses").is_some(),
        "SAT source should have clauses field"
    );
    assert!(
        example.target.instance.get("graph").is_some(),
        "KColoring target should have graph field"
    );
}

#[cfg(feature = "ilp-solver")]
#[test]
fn test_find_rule_example_integral_flow_bundles_to_ilp_contains_full_instances() {
    let source = ProblemRef {
        name: "IntegralFlowBundles".to_string(),
        variant: BTreeMap::new(),
    };
    let target = ProblemRef {
        name: "ILP".to_string(),
        variant: BTreeMap::from([("variable".to_string(), "i32".to_string())]),
    };

    let example = find_rule_example(&source, &target).expect("IntegralFlowBundles -> ILP exists");
    assert_eq!(example.source.problem, "IntegralFlowBundles");
    assert_eq!(example.target.problem, "ILP");
    assert!(example.source.instance.get("graph").is_some());
    assert!(!example.solutions[0].source_config.is_empty());
    assert!(!example.solutions[0].target_config.is_empty());
}

#[test]
fn test_build_rule_db_has_unique_structural_keys() {
    let db = build_rule_db().expect("rule db should build");
    let mut seen = BTreeSet::new();
    for rule in &db.rules {
        let key = (rule.source.problem_ref(), rule.target.problem_ref());
        assert!(
            seen.insert(key.clone()),
            "Duplicate rule key: {} {:?} -> {} {:?}",
            key.0.name,
            key.0.variant,
            key.1.name,
            key.1.variant
        );
    }
}

#[test]
fn test_find_rule_example_rejects_composed_path_pairs() {
    let source = ProblemRef {
        name: "MaximumIndependentSet".to_string(),
        variant: BTreeMap::from([
            ("graph".to_string(), "SimpleGraph".to_string()),
            ("weight".to_string(), "i32".to_string()),
        ]),
    };
    let target = ProblemRef {
        name: "ILP".to_string(),
        variant: BTreeMap::from([("variable".to_string(), "bool".to_string())]),
    };

    let result = find_rule_example(&source, &target);
    assert!(
        result.is_err(),
        "rule example db should only expose primitive direct reductions"
    );
}

#[test]
fn test_build_model_db_has_unique_structural_keys() {
    let db = build_model_db().expect("model db should build");
    let mut seen = BTreeSet::new();
    for model in &db.models {
        let key = model.problem_ref();
        assert!(
            seen.insert(key.clone()),
            "Duplicate model key: {} {:?}",
            key.name,
            key.variant
        );
    }
}

#[test]
fn test_rule_examples_store_single_solution_pair() {
    let db = build_rule_db().expect("rule db should build");
    for rule in &db.rules {
        assert_eq!(
            rule.solutions.len(),
            1,
            "canonical rule example should store one witness pair for {} {:?} -> {} {:?}",
            rule.source.problem,
            rule.source.variant,
            rule.target.problem,
            rule.target.variant
        );
    }
}

#[test]
fn canonical_model_example_ids_are_unique() {
    let specs = crate::models::graph::canonical_model_example_specs();
    let specs: Vec<_> = specs
        .into_iter()
        .chain(crate::models::formula::canonical_model_example_specs())
        .chain(crate::models::set::canonical_model_example_specs())
        .chain(crate::models::algebraic::canonical_model_example_specs())
        .chain(crate::models::misc::canonical_model_example_specs())
        .collect();
    let mut seen = HashSet::new();
    for spec in &specs {
        assert!(
            seen.insert(spec.id),
            "Duplicate model example id: {}",
            spec.id
        );
    }
}

#[test]
fn canonical_rule_example_ids_are_unique() {
    let specs = crate::rules::canonical_rule_example_specs();
    let mut seen = HashSet::new();
    for spec in &specs {
        assert!(
            seen.insert(spec.id),
            "Duplicate rule example id: {}",
            spec.id
        );
    }
}

#[test]
fn canonical_rule_examples_cover_exactly_authored_direct_reductions() {
    let computed = build_rule_db().expect("computed rule db should build");
    let example_keys: BTreeSet<_> = computed
        .rules
        .iter()
        .map(|rule| (rule.source.problem_ref(), rule.target.problem_ref()))
        .collect();

    let direct_reduction_keys: BTreeSet<_> = reduction_entries()
        .into_iter()
        .filter(|entry| entry.source_name != entry.target_name)
        .map(|entry| {
            (
                ProblemRef {
                    name: entry.source_name.to_string(),
                    variant: ReductionGraph::variant_to_map(&entry.source_variant()),
                },
                ProblemRef {
                    name: entry.target_name.to_string(),
                    variant: ReductionGraph::variant_to_map(&entry.target_variant()),
                },
            )
        })
        .collect();

    assert_eq!(
        example_keys, direct_reduction_keys,
        "rule example coverage should match authored direct reductions exactly"
    );
}

// ---- Error path tests for example_db ----

#[test]
fn find_rule_example_nonexistent_returns_error() {
    let source = ProblemRef {
        name: "NonExistentProblem".to_string(),
        variant: BTreeMap::new(),
    };
    let target = ProblemRef {
        name: "AlsoNonExistent".to_string(),
        variant: BTreeMap::new(),
    };
    let result = find_rule_example(&source, &target);
    assert!(result.is_err());
    let err_msg = format!("{}", result.unwrap_err());
    assert!(
        err_msg.contains("No canonical rule example"),
        "error should mention no canonical rule: {err_msg}"
    );
}

#[test]
fn find_model_example_nonexistent_returns_error() {
    let problem = ProblemRef {
        name: "NonExistentModel".to_string(),
        variant: BTreeMap::from([("graph".to_string(), "SimpleGraph".to_string())]),
    };
    let result = find_model_example(&problem);
    assert!(result.is_err());
    let err_msg = format!("{}", result.unwrap_err());
    assert!(
        err_msg.contains("No canonical model example"),
        "error should mention no canonical model: {err_msg}"
    );
}

// ---- Self-consistency tests ----

#[test]
fn model_specs_are_self_consistent() {
    let specs = crate::models::graph::canonical_model_example_specs()
        .into_iter()
        .chain(crate::models::formula::canonical_model_example_specs())
        .chain(crate::models::set::canonical_model_example_specs())
        .chain(crate::models::algebraic::canonical_model_example_specs())
        .chain(crate::models::misc::canonical_model_example_specs());

    for spec in specs {
        let actual = spec.instance.evaluate_json(&spec.optimal_config);
        assert_eq!(
            actual, spec.optimal_value,
            "Model spec '{}': evaluate(optimal_config) = {} but stored optimal_value = {}",
            spec.id, actual, spec.optimal_value
        );
    }
}

#[cfg(feature = "ilp-solver")]
#[test]
fn model_specs_are_optimal() {
    use crate::registry::find_variant_entry;
    use crate::solvers::ILPSolver;

    let ilp_solver = ILPSolver::new();

    let specs = crate::models::graph::canonical_model_example_specs()
        .into_iter()
        .chain(crate::models::formula::canonical_model_example_specs())
        .chain(crate::models::set::canonical_model_example_specs())
        .chain(crate::models::algebraic::canonical_model_example_specs())
        .chain(crate::models::misc::canonical_model_example_specs());

    for spec in specs {
        let name = spec.instance.problem_name();
        let variant = spec.instance.variant_map();
        // Try ILP (direct or via reduction), fall back to brute force for small instances
        let best_config = ilp_solver
            .solve_via_reduction(name, &variant, spec.instance.as_any())
            .or_else(|| {
                // Only brute-force if search space is small (≤ 2^20 configs)
                let dims = spec.instance.dims_dyn();
                let log_space: f64 = dims.iter().map(|&d| (d as f64).log2()).sum();
                if log_space > 20.0 {
                    return None;
                }
                let entry = find_variant_entry(name, &variant)?;
                let (config, _) = (entry.solve_witness_fn)(spec.instance.as_any())?;
                Some(config)
            });

        if let Some(best_config) = best_config {
            let best_value = spec.instance.evaluate_json(&best_config);
            assert_eq!(
                best_value, spec.optimal_value,
                "Model spec '{}': solver optimal = {} but stored optimal_value = {} \
                 (solver config: {:?}, stored config: {:?})",
                spec.id, best_value, spec.optimal_value, best_config, spec.optimal_config
            );
        } else {
            // Aggregate-only models (e.g., Sum) don't support witnesses.
            // Verify the stored config evaluates to the stored value.
            let stored_value = spec.instance.evaluate_json(&spec.optimal_config);
            assert_eq!(
                stored_value, spec.optimal_value,
                "Model spec '{}': stored config evaluates to {} but optimal_value = {} \
                 (config: {:?})",
                spec.id, stored_value, spec.optimal_value, spec.optimal_config
            );
        }
    }
}

#[test]
fn rule_specs_solution_pairs_are_consistent() {
    let graph = ReductionGraph::new();

    let db = build_rule_db().unwrap();
    for example in &db.rules {
        let label = format!(
            "{} {:?} -> {} {:?}",
            example.source.problem,
            example.source.variant,
            example.target.problem,
            example.target.variant
        );
        assert!(
            !example.solutions.is_empty(),
            "Rule {label} has no solution pairs"
        );

        // Deserialize source and target via the registry so we can evaluate configs
        let source = load_dyn(
            &example.source.problem,
            &example.source.variant,
            example.source.instance.clone(),
        )
        .unwrap_or_else(|e| panic!("Failed to load source for {label}: {e}"));
        let target = load_dyn(
            &example.target.problem,
            &example.target.variant,
            example.target.instance.clone(),
        )
        .unwrap_or_else(|e| panic!("Failed to load target for {label}: {e}"));

        // Re-run the reduction to get extract_solution for round-trip check
        let chain = graph
            .reduce_along_path(
                &graph
                    .find_cheapest_path(
                        &example.source.problem,
                        &example.source.variant,
                        &example.target.problem,
                        &example.target.variant,
                        &crate::types::ProblemSize::new(vec![]),
                        &crate::rules::MinimizeSteps,
                    )
                    .unwrap_or_else(|| panic!("No reduction path for {label}")),
                source.as_any(),
            )
            .unwrap_or_else(|| panic!("Failed to reduce along path for {label}"));

        for pair in &example.solutions {
            // Verify config lengths match problem dimensions
            assert_eq!(
                pair.source_config.len(),
                source.dims_dyn().len(),
                "Rule {label}: source_config length {} != dims length {}",
                pair.source_config.len(),
                source.dims_dyn().len()
            );
            assert_eq!(
                pair.target_config.len(),
                target.dims_dyn().len(),
                "Rule {label}: target_config length {} != dims length {}",
                pair.target_config.len(),
                target.dims_dyn().len()
            );
            // Verify configs produce feasible witness-capable evaluations.
            let source_eval = source.evaluate_dyn(&pair.source_config);
            let target_eval = target.evaluate_dyn(&pair.target_config);
            let source_val = source.evaluate_json(&pair.source_config);
            assert_ne!(
                source_eval, "Max(None)",
                "Rule {label}: source_config evaluates to Max(None)"
            );
            assert_ne!(
                source_eval, "Min(None)",
                "Rule {label}: source_config evaluates to Min(None)"
            );
            assert_ne!(
                source_eval, "Or(false)",
                "Rule {label}: source_config evaluates to Or(false)"
            );
            assert_ne!(
                target_eval, "Max(None)",
                "Rule {label}: target_config evaluates to Max(None)"
            );
            assert_ne!(
                target_eval, "Min(None)",
                "Rule {label}: target_config evaluates to Min(None)"
            );
            assert_ne!(
                target_eval, "Or(false)",
                "Rule {label}: target_config evaluates to Or(false)"
            );
            // Round-trip: extract_solution(target_config) must produce a valid
            // source config with the same evaluation value
            let extracted = chain.extract_solution(&pair.target_config);
            let extracted_val = source.evaluate_json(&extracted);
            assert_eq!(
                extracted_val, source_val,
                "Rule {label}: round-trip value mismatch: \
                 evaluate(extract_solution(target_config)) = {} but evaluate(source_config) = {} \
                 (extracted: {:?}, stored: {:?})",
                extracted_val, source_val, extracted, pair.source_config
            );
        }
    }
}

// ---- Rule lookup tests for issue #974 ----

// PR #777 rules

#[test]
fn test_find_rule_example_hamiltoniancircuit_to_hamiltonianpath() {
    let source = ProblemRef {
        name: "HamiltonianCircuit".to_string(),
        variant: BTreeMap::from([("graph".to_string(), "SimpleGraph".to_string())]),
    };
    let target = ProblemRef {
        name: "HamiltonianPath".to_string(),
        variant: BTreeMap::from([("graph".to_string(), "SimpleGraph".to_string())]),
    };
    let example = find_rule_example(&source, &target).unwrap();
    assert_eq!(example.source.problem, "HamiltonianCircuit");
    assert_eq!(example.target.problem, "HamiltonianPath");
}

#[test]
fn test_find_rule_example_kclique_to_subgraphisomorphism() {
    let source = ProblemRef {
        name: "KClique".to_string(),
        variant: BTreeMap::from([("graph".to_string(), "SimpleGraph".to_string())]),
    };
    let target = ProblemRef {
        name: "SubgraphIsomorphism".to_string(),
        variant: BTreeMap::new(),
    };
    let example = find_rule_example(&source, &target).unwrap();
    assert_eq!(example.source.problem, "KClique");
    assert_eq!(example.target.problem, "SubgraphIsomorphism");
}

#[test]
fn test_find_rule_example_partition_to_multiprocessorscheduling() {
    let source = ProblemRef {
        name: "Partition".to_string(),
        variant: BTreeMap::new(),
    };
    let target = ProblemRef {
        name: "MultiprocessorScheduling".to_string(),
        variant: BTreeMap::new(),
    };
    let example = find_rule_example(&source, &target).unwrap();
    assert_eq!(example.source.problem, "Partition");
    assert_eq!(example.target.problem, "MultiprocessorScheduling");
}

#[test]
fn test_find_rule_example_hamiltonianpath_to_isomorphicspanningtree() {
    let source = ProblemRef {
        name: "HamiltonianPath".to_string(),
        variant: BTreeMap::from([("graph".to_string(), "SimpleGraph".to_string())]),
    };
    let target = ProblemRef {
        name: "IsomorphicSpanningTree".to_string(),
        variant: BTreeMap::new(),
    };
    let example = find_rule_example(&source, &target).unwrap();
    assert_eq!(example.source.problem, "HamiltonianPath");
    assert_eq!(example.target.problem, "IsomorphicSpanningTree");
}

#[test]
fn test_find_rule_example_hamiltoniancircuit_to_bottlenecktravelingsalesman() {
    let source = ProblemRef {
        name: "HamiltonianCircuit".to_string(),
        variant: BTreeMap::from([("graph".to_string(), "SimpleGraph".to_string())]),
    };
    let target = ProblemRef {
        name: "BottleneckTravelingSalesman".to_string(),
        variant: BTreeMap::new(),
    };
    let example = find_rule_example(&source, &target).unwrap();
    assert_eq!(example.source.problem, "HamiltonianCircuit");
    assert_eq!(example.target.problem, "BottleneckTravelingSalesman");
}

#[test]
fn test_find_rule_example_kclique_to_conjunctivebooleanquery() {
    let source = ProblemRef {
        name: "KClique".to_string(),
        variant: BTreeMap::from([("graph".to_string(), "SimpleGraph".to_string())]),
    };
    let target = ProblemRef {
        name: "ConjunctiveBooleanQuery".to_string(),
        variant: BTreeMap::new(),
    };
    let example = find_rule_example(&source, &target).unwrap();
    assert_eq!(example.source.problem, "KClique");
    assert_eq!(example.target.problem, "ConjunctiveBooleanQuery");
}

#[test]
fn test_find_rule_example_exactcoverby3sets_to_staffscheduling() {
    let source = ProblemRef {
        name: "ExactCoverBy3Sets".to_string(),
        variant: BTreeMap::new(),
    };
    let target = ProblemRef {
        name: "StaffScheduling".to_string(),
        variant: BTreeMap::new(),
    };
    let example = find_rule_example(&source, &target).unwrap();
    assert_eq!(example.source.problem, "ExactCoverBy3Sets");
    assert_eq!(example.target.problem, "StaffScheduling");
}

#[test]
fn test_find_rule_example_satisfiability_to_naesatisfiability() {
    let source = ProblemRef {
        name: "Satisfiability".to_string(),
        variant: BTreeMap::new(),
    };
    let target = ProblemRef {
        name: "NAESatisfiability".to_string(),
        variant: BTreeMap::new(),
    };
    let example = find_rule_example(&source, &target).unwrap();
    assert_eq!(example.source.problem, "Satisfiability");
    assert_eq!(example.target.problem, "NAESatisfiability");
}

// PR #779 rules

#[test]
fn test_find_rule_example_ksatisfiability_to_minimumvertexcover() {
    let source = ProblemRef {
        name: "KSatisfiability".to_string(),
        variant: BTreeMap::from([("k".to_string(), "K3".to_string())]),
    };
    let target = ProblemRef {
        name: "MinimumVertexCover".to_string(),
        variant: BTreeMap::from([
            ("graph".to_string(), "SimpleGraph".to_string()),
            ("weight".to_string(), "i32".to_string()),
        ]),
    };
    let example = find_rule_example(&source, &target).unwrap();
    assert_eq!(example.source.problem, "KSatisfiability");
    assert_eq!(example.target.problem, "MinimumVertexCover");
}

#[test]
fn test_find_rule_example_partition_to_sequencingwithinintervals() {
    let source = ProblemRef {
        name: "Partition".to_string(),
        variant: BTreeMap::new(),
    };
    let target = ProblemRef {
        name: "SequencingWithinIntervals".to_string(),
        variant: BTreeMap::new(),
    };
    let example = find_rule_example(&source, &target).unwrap();
    assert_eq!(example.source.problem, "Partition");
    assert_eq!(example.target.problem, "SequencingWithinIntervals");
}

#[test]
fn test_find_rule_example_minimumvertexcover_to_minimumfeedbackarcset() {
    let source = ProblemRef {
        name: "MinimumVertexCover".to_string(),
        variant: BTreeMap::from([
            ("graph".to_string(), "SimpleGraph".to_string()),
            ("weight".to_string(), "i32".to_string()),
        ]),
    };
    let target = ProblemRef {
        name: "MinimumFeedbackArcSet".to_string(),
        variant: BTreeMap::from([("weight".to_string(), "i32".to_string())]),
    };
    let example = find_rule_example(&source, &target).unwrap();
    assert_eq!(example.source.problem, "MinimumVertexCover");
    assert_eq!(example.target.problem, "MinimumFeedbackArcSet");
}

#[test]
fn test_find_rule_example_ksatisfiability_to_kclique() {
    let source = ProblemRef {
        name: "KSatisfiability".to_string(),
        variant: BTreeMap::from([("k".to_string(), "K3".to_string())]),
    };
    let target = ProblemRef {
        name: "KClique".to_string(),
        variant: BTreeMap::from([("graph".to_string(), "SimpleGraph".to_string())]),
    };
    let example = find_rule_example(&source, &target).unwrap();
    assert_eq!(example.source.problem, "KSatisfiability");
    assert_eq!(example.target.problem, "KClique");
}

#[test]
fn test_find_rule_example_hamiltoniancircuit_to_biconnectivityaugmentation() {
    let source = ProblemRef {
        name: "HamiltonianCircuit".to_string(),
        variant: BTreeMap::from([("graph".to_string(), "SimpleGraph".to_string())]),
    };
    let target = ProblemRef {
        name: "BiconnectivityAugmentation".to_string(),
        variant: BTreeMap::from([
            ("graph".to_string(), "SimpleGraph".to_string()),
            ("weight".to_string(), "i32".to_string()),
        ]),
    };
    let example = find_rule_example(&source, &target).unwrap();
    assert_eq!(example.source.problem, "HamiltonianCircuit");
    assert_eq!(example.target.problem, "BiconnectivityAugmentation");
}

#[test]
fn test_find_rule_example_hamiltoniancircuit_to_strongconnectivityaugmentation() {
    let source = ProblemRef {
        name: "HamiltonianCircuit".to_string(),
        variant: BTreeMap::from([("graph".to_string(), "SimpleGraph".to_string())]),
    };
    let target = ProblemRef {
        name: "StrongConnectivityAugmentation".to_string(),
        variant: BTreeMap::from([("weight".to_string(), "i32".to_string())]),
    };
    let example = find_rule_example(&source, &target).unwrap();
    assert_eq!(example.source.problem, "HamiltonianCircuit");
    assert_eq!(example.target.problem, "StrongConnectivityAugmentation");
}

#[test]
fn test_find_rule_example_hamiltoniancircuit_to_stackercrane() {
    let source = ProblemRef {
        name: "HamiltonianCircuit".to_string(),
        variant: BTreeMap::from([("graph".to_string(), "SimpleGraph".to_string())]),
    };
    let target = ProblemRef {
        name: "StackerCrane".to_string(),
        variant: BTreeMap::new(),
    };
    let example = find_rule_example(&source, &target).unwrap();
    assert_eq!(example.source.problem, "HamiltonianCircuit");
    assert_eq!(example.target.problem, "StackerCrane");
}

#[test]
fn test_find_rule_example_hamiltoniancircuit_to_ruralpostman() {
    let source = ProblemRef {
        name: "HamiltonianCircuit".to_string(),
        variant: BTreeMap::from([("graph".to_string(), "SimpleGraph".to_string())]),
    };
    let target = ProblemRef {
        name: "RuralPostman".to_string(),
        variant: BTreeMap::from([
            ("graph".to_string(), "SimpleGraph".to_string()),
            ("weight".to_string(), "i32".to_string()),
        ]),
    };
    let example = find_rule_example(&source, &target).unwrap();
    assert_eq!(example.source.problem, "HamiltonianCircuit");
    assert_eq!(example.target.problem, "RuralPostman");
}

#[test]
fn test_find_rule_example_partition_to_shortestweightconstrainedpath() {
    let source = ProblemRef {
        name: "Partition".to_string(),
        variant: BTreeMap::new(),
    };
    let target = ProblemRef {
        name: "ShortestWeightConstrainedPath".to_string(),
        variant: BTreeMap::from([
            ("graph".to_string(), "SimpleGraph".to_string()),
            ("weight".to_string(), "i32".to_string()),
        ]),
    };
    let example = find_rule_example(&source, &target).unwrap();
    assert_eq!(example.source.problem, "Partition");
    assert_eq!(example.target.problem, "ShortestWeightConstrainedPath");
}

#[test]
fn test_find_rule_example_maximumindependentset_to_integralflowbundles() {
    let source = ProblemRef {
        name: "MaximumIndependentSet".to_string(),
        variant: BTreeMap::from([
            ("graph".to_string(), "SimpleGraph".to_string()),
            ("weight".to_string(), "i32".to_string()),
        ]),
    };
    let target = ProblemRef {
        name: "IntegralFlowBundles".to_string(),
        variant: BTreeMap::new(),
    };
    let example = find_rule_example(&source, &target).unwrap();
    assert_eq!(example.source.problem, "MaximumIndependentSet");
    assert_eq!(example.target.problem, "IntegralFlowBundles");
}

#[test]
fn test_find_rule_example_hamiltoniancircuit_to_quadraticassignment() {
    let source = ProblemRef {
        name: "HamiltonianCircuit".to_string(),
        variant: BTreeMap::from([("graph".to_string(), "SimpleGraph".to_string())]),
    };
    let target = ProblemRef {
        name: "QuadraticAssignment".to_string(),
        variant: BTreeMap::new(),
    };
    let example = find_rule_example(&source, &target).unwrap();
    assert_eq!(example.source.problem, "HamiltonianCircuit");
    assert_eq!(example.target.problem, "QuadraticAssignment");
}

#[test]
fn test_find_rule_example_hamiltonianpath_to_consecutiveonessubmatrix() {
    let source = ProblemRef {
        name: "HamiltonianPath".to_string(),
        variant: BTreeMap::from([("graph".to_string(), "SimpleGraph".to_string())]),
    };
    let target = ProblemRef {
        name: "ConsecutiveOnesSubmatrix".to_string(),
        variant: BTreeMap::new(),
    };
    let example = find_rule_example(&source, &target).unwrap();
    assert_eq!(example.source.problem, "HamiltonianPath");
    assert_eq!(example.target.problem, "ConsecutiveOnesSubmatrix");
}

// PR #804 rules

#[test]
fn test_find_rule_example_minimumvertexcover_to_minimumhittingset() {
    let source = ProblemRef {
        name: "MinimumVertexCover".to_string(),
        variant: BTreeMap::from([
            ("graph".to_string(), "SimpleGraph".to_string()),
            ("weight".to_string(), "One".to_string()),
        ]),
    };
    let target = ProblemRef {
        name: "MinimumHittingSet".to_string(),
        variant: BTreeMap::new(),
    };
    let example = find_rule_example(&source, &target).unwrap();
    assert_eq!(example.source.problem, "MinimumVertexCover");
    assert_eq!(example.target.problem, "MinimumHittingSet");
}

#[test]
fn test_find_rule_example_pp2_to_boundedcomponentspanningforest() {
    let source = ProblemRef {
        name: "PartitionIntoPathsOfLength2".to_string(),
        variant: BTreeMap::from([("graph".to_string(), "SimpleGraph".to_string())]),
    };
    let target = ProblemRef {
        name: "BoundedComponentSpanningForest".to_string(),
        variant: BTreeMap::from([
            ("graph".to_string(), "SimpleGraph".to_string()),
            ("weight".to_string(), "i32".to_string()),
        ]),
    };
    let example = find_rule_example(&source, &target).unwrap();
    assert_eq!(example.source.problem, "PartitionIntoPathsOfLength2");
    assert_eq!(example.target.problem, "BoundedComponentSpanningForest");
}

#[test]
fn test_find_rule_example_hamiltoniancircuit_to_longestcircuit() {
    let source = ProblemRef {
        name: "HamiltonianCircuit".to_string(),
        variant: BTreeMap::from([("graph".to_string(), "SimpleGraph".to_string())]),
    };
    let target = ProblemRef {
        name: "LongestCircuit".to_string(),
        variant: BTreeMap::from([
            ("graph".to_string(), "SimpleGraph".to_string()),
            ("weight".to_string(), "i32".to_string()),
        ]),
    };
    let example = find_rule_example(&source, &target).unwrap();
    assert_eq!(example.source.problem, "HamiltonianCircuit");
    assert_eq!(example.target.problem, "LongestCircuit");
}

#[test]
fn test_find_rule_example_partition_to_subsetsum() {
    let source = ProblemRef {
        name: "Partition".to_string(),
        variant: BTreeMap::new(),
    };
    let target = ProblemRef {
        name: "SubsetSum".to_string(),
        variant: BTreeMap::new(),
    };
    let example = find_rule_example(&source, &target).unwrap();
    assert_eq!(example.source.problem, "Partition");
    assert_eq!(example.target.problem, "SubsetSum");
}

#[test]
fn test_find_rule_example_rootedtreearrangement_to_rootedtreestorageassignment() {
    let source = ProblemRef {
        name: "RootedTreeArrangement".to_string(),
        variant: BTreeMap::from([("graph".to_string(), "SimpleGraph".to_string())]),
    };
    let target = ProblemRef {
        name: "RootedTreeStorageAssignment".to_string(),
        variant: BTreeMap::new(),
    };
    let example = find_rule_example(&source, &target).unwrap();
    assert_eq!(example.source.problem, "RootedTreeArrangement");
    assert_eq!(example.target.problem, "RootedTreeStorageAssignment");
}

#[test]
fn test_find_rule_example_subsetsum_to_capacityassignment() {
    let source = ProblemRef {
        name: "SubsetSum".to_string(),
        variant: BTreeMap::new(),
    };
    let target = ProblemRef {
        name: "CapacityAssignment".to_string(),
        variant: BTreeMap::new(),
    };
    let example = find_rule_example(&source, &target).unwrap();
    assert_eq!(example.source.problem, "SubsetSum");
    assert_eq!(example.target.problem, "CapacityAssignment");
}

#[test]
fn test_find_rule_example_longestcommonsubsequence_to_maximumindependentset() {
    let source = ProblemRef {
        name: "LongestCommonSubsequence".to_string(),
        variant: BTreeMap::new(),
    };
    let target = ProblemRef {
        name: "MaximumIndependentSet".to_string(),
        variant: BTreeMap::from([
            ("graph".to_string(), "SimpleGraph".to_string()),
            ("weight".to_string(), "One".to_string()),
        ]),
    };
    let example = find_rule_example(&source, &target).unwrap();
    assert_eq!(example.source.problem, "LongestCommonSubsequence");
    assert_eq!(example.target.problem, "MaximumIndependentSet");
}

#[test]
fn test_find_rule_example_minimumvertexcover_to_ensemblecomputation() {
    let source = ProblemRef {
        name: "MinimumVertexCover".to_string(),
        variant: BTreeMap::from([
            ("graph".to_string(), "SimpleGraph".to_string()),
            ("weight".to_string(), "One".to_string()),
        ]),
    };
    let target = ProblemRef {
        name: "EnsembleComputation".to_string(),
        variant: BTreeMap::new(),
    };
    let example = find_rule_example(&source, &target).unwrap();
    assert_eq!(example.source.problem, "MinimumVertexCover");
    assert_eq!(example.target.problem, "EnsembleComputation");
}

#[test]
fn test_find_rule_example_kclique_to_balancedcompletebipartitesubgraph() {
    let source = ProblemRef {
        name: "KClique".to_string(),
        variant: BTreeMap::from([("graph".to_string(), "SimpleGraph".to_string())]),
    };
    let target = ProblemRef {
        name: "BalancedCompleteBipartiteSubgraph".to_string(),
        variant: BTreeMap::new(),
    };
    let example = find_rule_example(&source, &target).unwrap();
    assert_eq!(example.source.problem, "KClique");
    assert_eq!(example.target.problem, "BalancedCompleteBipartiteSubgraph");
}

#[test]
fn test_find_rule_example_kcoloring_to_twodimensionalconsecutivesets() {
    let source = ProblemRef {
        name: "KColoring".to_string(),
        variant: BTreeMap::from([
            ("graph".to_string(), "SimpleGraph".to_string()),
            ("k".to_string(), "K3".to_string()),
        ]),
    };
    let target = ProblemRef {
        name: "TwoDimensionalConsecutiveSets".to_string(),
        variant: BTreeMap::new(),
    };
    let example = find_rule_example(&source, &target).unwrap();
    assert_eq!(example.source.problem, "KColoring");
    assert_eq!(example.target.problem, "TwoDimensionalConsecutiveSets");
}

#[test]
fn test_find_rule_example_paintshop_to_qubo() {
    let source = ProblemRef {
        name: "PaintShop".to_string(),
        variant: BTreeMap::new(),
    };
    let target = ProblemRef {
        name: "QUBO".to_string(),
        variant: BTreeMap::from([("weight".to_string(), "f64".to_string())]),
    };
    let example = find_rule_example(&source, &target).unwrap();
    assert_eq!(example.source.problem, "PaintShop");
    assert_eq!(example.target.problem, "QUBO");
}

// PR #972 rules

#[test]
fn test_find_rule_example_partition_to_binpacking() {
    let source = ProblemRef {
        name: "Partition".to_string(),
        variant: BTreeMap::new(),
    };
    let target = ProblemRef {
        name: "BinPacking".to_string(),
        variant: BTreeMap::from([("weight".to_string(), "i32".to_string())]),
    };
    let example = find_rule_example(&source, &target).unwrap();
    assert_eq!(example.source.problem, "Partition");
    assert_eq!(example.target.problem, "BinPacking");
}

#[test]
fn test_find_rule_example_exactcoverby3sets_to_maximumsetpacking() {
    let source = ProblemRef {
        name: "ExactCoverBy3Sets".to_string(),
        variant: BTreeMap::new(),
    };
    let target = ProblemRef {
        name: "MaximumSetPacking".to_string(),
        variant: BTreeMap::from([("weight".to_string(), "One".to_string())]),
    };
    let example = find_rule_example(&source, &target).unwrap();
    assert_eq!(example.source.problem, "ExactCoverBy3Sets");
    assert_eq!(example.target.problem, "MaximumSetPacking");
}

#[test]
fn test_find_rule_example_naesatisfiability_to_maxcut() {
    let source = ProblemRef {
        name: "NAESatisfiability".to_string(),
        variant: BTreeMap::new(),
    };
    let target = ProblemRef {
        name: "MaxCut".to_string(),
        variant: BTreeMap::from([
            ("graph".to_string(), "SimpleGraph".to_string()),
            ("weight".to_string(), "i32".to_string()),
        ]),
    };
    let example = find_rule_example(&source, &target).unwrap();
    assert_eq!(example.source.problem, "NAESatisfiability");
    assert_eq!(example.target.problem, "MaxCut");
}

#[test]
fn test_find_rule_example_threepartition_to_resourceconstrainedscheduling() {
    let source = ProblemRef {
        name: "ThreePartition".to_string(),
        variant: BTreeMap::new(),
    };
    let target = ProblemRef {
        name: "ResourceConstrainedScheduling".to_string(),
        variant: BTreeMap::new(),
    };
    let example = find_rule_example(&source, &target).unwrap();
    assert_eq!(example.source.problem, "ThreePartition");
    assert_eq!(example.target.problem, "ResourceConstrainedScheduling");
}

#[test]
fn test_find_rule_example_threepartition_to_sequencingwithreleasetimesanddeadlines() {
    let source = ProblemRef {
        name: "ThreePartition".to_string(),
        variant: BTreeMap::new(),
    };
    let target = ProblemRef {
        name: "SequencingWithReleaseTimesAndDeadlines".to_string(),
        variant: BTreeMap::new(),
    };
    let example = find_rule_example(&source, &target).unwrap();
    assert_eq!(example.source.problem, "ThreePartition");
    assert_eq!(
        example.target.problem,
        "SequencingWithReleaseTimesAndDeadlines"
    );
}

#[test]
fn test_find_rule_example_threepartition_to_sequencingtominimizeweightedtardiness() {
    let source = ProblemRef {
        name: "ThreePartition".to_string(),
        variant: BTreeMap::new(),
    };
    let target = ProblemRef {
        name: "SequencingToMinimizeWeightedTardiness".to_string(),
        variant: BTreeMap::new(),
    };
    let example = find_rule_example(&source, &target).unwrap();
    assert_eq!(example.source.problem, "ThreePartition");
    assert_eq!(
        example.target.problem,
        "SequencingToMinimizeWeightedTardiness"
    );
}

#[test]
fn test_find_rule_example_threepartition_to_flowshopscheduling() {
    let source = ProblemRef {
        name: "ThreePartition".to_string(),
        variant: BTreeMap::new(),
    };
    let target = ProblemRef {
        name: "FlowShopScheduling".to_string(),
        variant: BTreeMap::new(),
    };
    let example = find_rule_example(&source, &target).unwrap();
    assert_eq!(example.source.problem, "ThreePartition");
    assert_eq!(example.target.problem, "FlowShopScheduling");
}

#[test]
fn test_find_rule_example_threepartition_to_jobshopscheduling() {
    let source = ProblemRef {
        name: "ThreePartition".to_string(),
        variant: BTreeMap::new(),
    };
    let target = ProblemRef {
        name: "JobShopScheduling".to_string(),
        variant: BTreeMap::new(),
    };
    let example = find_rule_example(&source, &target).unwrap();
    assert_eq!(example.source.problem, "ThreePartition");
    assert_eq!(example.target.problem, "JobShopScheduling");
}

#[test]
fn test_find_rule_example_maxcut_to_minimumcutintoboundedsets() {
    let source = ProblemRef {
        name: "MaxCut".to_string(),
        variant: BTreeMap::from([
            ("graph".to_string(), "SimpleGraph".to_string()),
            ("weight".to_string(), "i32".to_string()),
        ]),
    };
    let target = ProblemRef {
        name: "MinimumCutIntoBoundedSets".to_string(),
        variant: BTreeMap::from([
            ("graph".to_string(), "SimpleGraph".to_string()),
            ("weight".to_string(), "i32".to_string()),
        ]),
    };
    let example = find_rule_example(&source, &target).unwrap();
    assert_eq!(example.source.problem, "MaxCut");
    assert_eq!(example.target.problem, "MinimumCutIntoBoundedSets");
}
