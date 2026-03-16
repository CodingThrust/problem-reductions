use crate::example_db::{
    build_example_db, build_model_db, build_rule_db, compute_model_db, compute_rule_db,
    find_model_example, find_rule_example,
};
use crate::export::ProblemRef;
use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP, QUBO};
use crate::models::graph::{MaximumMatching, SpinGlass};
use crate::registry::load_dyn;
use crate::rules::{registry::reduction_entries, ReductionGraph};
use crate::topology::SimpleGraph;
use serde_json::Value;
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
        !example.optimal.is_empty(),
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
        !example.optimal.is_empty(),
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
        !example.optimal.is_empty(),
        "canonical example should include satisfying assignments"
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
fn test_build_rule_db_nonempty() {
    let db = build_rule_db().expect("rule db should build");
    assert!(!db.rules.is_empty(), "rule db should not be empty");
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
fn test_computed_rule_examples_store_single_solution_pair() {
    let db = compute_rule_db().expect("computed rule db should build");
    for rule in &db.rules {
        assert_eq!(
            rule.solutions.len(),
            1,
            "computed canonical rule example should store one witness pair for {} {:?} -> {} {:?}",
            rule.source.problem,
            rule.source.variant,
            rule.target.problem,
            rule.target.variant
        );
    }
}

#[test]
fn test_build_model_db_nonempty() {
    let db = build_model_db().expect("model db should build");
    assert!(!db.models.is_empty(), "model db should not be empty");
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
    let computed = compute_rule_db().expect("computed rule db should build");
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

fn problem_json_key(value: &Value) -> String {
    serde_json::to_string(value).expect("json value should serialize")
}

fn edge_key(edge: &Value) -> (u64, u64, String) {
    let values = edge
        .as_array()
        .expect("graph edge should serialize as a JSON array");
    let u = values.first().and_then(Value::as_u64).unwrap_or(u64::MAX);
    let v = values.get(1).and_then(Value::as_u64).unwrap_or(u64::MAX);
    (u, v, problem_json_key(edge))
}

fn term_key(term: &Value) -> (u64, String) {
    let values = term
        .as_array()
        .expect("ILP term should serialize as a JSON array");
    let variable = values.first().and_then(Value::as_u64).unwrap_or(u64::MAX);
    (variable, problem_json_key(term))
}

fn graph_edges_mut(object: &mut serde_json::Map<String, Value>) -> Option<&mut Vec<Value>> {
    let graph = object.get_mut("graph")?.as_object_mut()?;
    if graph.contains_key("inner") {
        return graph
            .get_mut("inner")?
            .as_object_mut()?
            .get_mut("edges")?
            .as_array_mut();
    }
    graph.get_mut("edges")?.as_array_mut()
}

fn reorder_array(values: &mut Vec<Value>, old_indices: &[usize]) {
    let reordered: Vec<Value> = old_indices.iter().map(|&idx| values[idx].clone()).collect();
    *values = reordered;
}

fn edge_aligned_fields(problem_name: &str) -> &'static [&'static str] {
    match problem_name {
        "MaximumMatching" | "MaxCut" | "TravelingSalesman" => &["edge_weights"],
        "SpinGlass" => &["couplings"],
        _ => &[],
    }
}

fn normalize_graph_instance(problem_name: &str, instance: &mut Value) {
    let Some(object) = instance.as_object_mut() else {
        return;
    };
    let edge_order = {
        let Some(edges) = graph_edges_mut(object) else {
            return;
        };
        let mut indexed_edges: Vec<(usize, Value)> = edges.iter().cloned().enumerate().collect();
        indexed_edges.sort_by_key(|(_, edge)| edge_key(edge));
        *edges = indexed_edges.iter().map(|(_, edge)| edge.clone()).collect();
        indexed_edges
            .into_iter()
            .map(|(old_index, _)| old_index)
            .collect::<Vec<_>>()
    };

    for field in edge_aligned_fields(problem_name) {
        if let Some(values) = object.get_mut(*field).and_then(Value::as_array_mut) {
            assert_eq!(
                values.len(),
                edge_order.len(),
                "{problem_name}.{field} should stay aligned with graph edges",
            );
            reorder_array(values, &edge_order);
        }
    }
}

fn normalize_ilp_instance(instance: &mut Value) {
    let Some(object) = instance.as_object_mut() else {
        return;
    };

    if let Some(objective) = object.get_mut("objective").and_then(Value::as_array_mut) {
        objective.sort_by_key(term_key);
    }

    if let Some(constraints) = object.get_mut("constraints").and_then(Value::as_array_mut) {
        for constraint in constraints.iter_mut() {
            if let Some(terms) = constraint.get_mut("terms").and_then(Value::as_array_mut) {
                terms.sort_by_key(term_key);
            }
        }
        constraints.sort_by_key(problem_json_key);
    }
}

fn normalize_problem_instance(problem: &ProblemRef, instance: &Value) -> Value {
    let loaded =
        load_dyn(&problem.name, &problem.variant, instance.clone()).unwrap_or_else(|err| {
            panic!(
                "fixture instance should deserialize for {} {:?}: {}",
                problem.name, problem.variant, err
            )
        });
    let mut normalized = loaded.serialize_json();
    normalize_graph_instance(&problem.name, &mut normalized);
    if problem.name == "ILP" {
        normalize_ilp_instance(&mut normalized);
    }
    normalized
}

fn numbers_semantically_equal(left: &serde_json::Number, right: &serde_json::Number) -> bool {
    match (left.as_i64(), right.as_i64(), left.as_u64(), right.as_u64()) {
        (Some(a), Some(b), _, _) => a == b,
        (_, _, Some(a), Some(b)) => a == b,
        _ => {
            let Some(left) = left.as_f64() else {
                return false;
            };
            let Some(right) = right.as_f64() else {
                return false;
            };
            let scale = left.abs().max(right.abs()).max(1.0);
            (left - right).abs() <= 1e-12 * scale
        }
    }
}

fn json_semantically_equal(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Null, Value::Null) => true,
        (Value::Bool(a), Value::Bool(b)) => a == b,
        (Value::Number(a), Value::Number(b)) => numbers_semantically_equal(a, b),
        (Value::String(a), Value::String(b)) => a == b,
        (Value::Array(a), Value::Array(b)) => {
            a.len() == b.len()
                && a.iter()
                    .zip(b.iter())
                    .all(|(left, right)| json_semantically_equal(left, right))
        }
        (Value::Object(a), Value::Object(b)) => {
            a.len() == b.len()
                && a.iter().all(|(key, left_value)| {
                    b.get(key)
                        .map(|right_value| json_semantically_equal(left_value, right_value))
                        .unwrap_or(false)
                })
        }
        _ => false,
    }
}

#[test]
fn normalize_problem_instance_treats_reordered_ilp_as_equal() {
    let problem = ProblemRef {
        name: "ILP".to_string(),
        variant: BTreeMap::from([("variable".to_string(), "bool".to_string())]),
    };
    let canonical = ILP::<bool>::new(
        3,
        vec![
            LinearConstraint::le(vec![(0, 1.0), (2, 1.0)], 1.0),
            LinearConstraint::ge(vec![(1, 2.0), (0, 1.0)], 2.0),
        ],
        vec![(2, 3.0), (0, 1.0)],
        ObjectiveSense::Maximize,
    );
    let canonical = serde_json::to_value(&canonical).expect("ILP should serialize");

    let reordered = ILP::<bool>::new(
        3,
        vec![
            LinearConstraint::ge(vec![(0, 1.0), (1, 2.0)], 2.0),
            LinearConstraint::le(vec![(2, 1.0), (0, 1.0)], 1.0),
        ],
        vec![(0, 1.0), (2, 3.0)],
        ObjectiveSense::Maximize,
    );
    let reordered = serde_json::to_value(&reordered).expect("ILP should serialize");

    assert_eq!(
        normalize_problem_instance(&problem, &canonical),
        normalize_problem_instance(&problem, &reordered)
    );
}

#[test]
fn json_semantically_equal_treats_tiny_float_roundoff_as_equal() {
    let problem = ProblemRef {
        name: "QUBO".to_string(),
        variant: BTreeMap::from([("weight".to_string(), "f64".to_string())]),
    };
    let canonical = QUBO::from_matrix(vec![vec![0.2, -1.5], vec![0.0, 1.0]]);
    let canonical = normalize_problem_instance(
        &problem,
        &serde_json::to_value(&canonical).expect("QUBO should serialize"),
    );

    let noisy = QUBO::from_matrix(vec![vec![0.20000000000000018, -1.5], vec![0.0, 1.0]]);
    let noisy = normalize_problem_instance(
        &problem,
        &serde_json::to_value(&noisy).expect("QUBO should serialize"),
    );

    assert!(
        json_semantically_equal(&canonical, &noisy),
        "tiny float noise should not count as a fixture mismatch"
    );
}

#[test]
fn normalize_problem_instance_treats_reordered_graph_edges_as_equal() {
    let problem = ProblemRef {
        name: "MaximumMatching".to_string(),
        variant: BTreeMap::from([
            ("graph".to_string(), "SimpleGraph".to_string()),
            ("weight".to_string(), "i32".to_string()),
        ]),
    };
    let canonical = MaximumMatching::<_, i32>::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![5, 7, 11],
    );
    let canonical = serde_json::to_value(&canonical).expect("matching should serialize");

    let reordered = MaximumMatching::<_, i32>::new(
        SimpleGraph::new(4, vec![(2, 3), (0, 1), (1, 2)]),
        vec![11, 5, 7],
    );
    let reordered = serde_json::to_value(&reordered).expect("matching should serialize");

    assert_eq!(
        normalize_problem_instance(&problem, &canonical),
        normalize_problem_instance(&problem, &reordered)
    );
}

#[test]
fn normalize_problem_instance_treats_reordered_spin_glass_interactions_as_equal() {
    let problem = ProblemRef {
        name: "SpinGlass".to_string(),
        variant: BTreeMap::from([
            ("graph".to_string(), "SimpleGraph".to_string()),
            ("weight".to_string(), "i32".to_string()),
        ]),
    };
    let canonical = SpinGlass::<SimpleGraph, i32>::new(
        3,
        vec![((0, 1), 5), ((1, 2), -2), ((0, 2), 9)],
        vec![1, 0, -1],
    );
    let canonical = serde_json::to_value(&canonical).expect("spin glass should serialize");

    let reordered = SpinGlass::<SimpleGraph, i32>::new(
        3,
        vec![((0, 2), 9), ((0, 1), 5), ((1, 2), -2)],
        vec![1, 0, -1],
    );
    let reordered = serde_json::to_value(&reordered).expect("spin glass should serialize");

    assert_eq!(
        normalize_problem_instance(&problem, &canonical),
        normalize_problem_instance(&problem, &reordered)
    );
}

// ---- Fixture verification tests ----
// These verify that stored fixtures are structurally and semantically
// consistent with freshly computed results. Rule fixture ordering can vary,
// so compare keyed content instead of relying on positional equality.

#[test]
fn verify_model_fixtures_match_computed() {
    let loaded = build_model_db().expect("fixture should load");
    let computed = compute_model_db().expect("compute should succeed");
    assert_eq!(
        loaded.models.len(),
        computed.models.len(),
        "fixture and computed model counts differ — regenerate fixtures"
    );
    for (loaded_model, computed_model) in loaded.models.iter().zip(computed.models.iter()) {
        assert_eq!(
            loaded_model.problem, computed_model.problem,
            "model fixture mismatch for {} {:?} — problem name drifted",
            loaded_model.problem, loaded_model.variant
        );
        assert_eq!(
            loaded_model.variant, computed_model.variant,
            "model fixture mismatch for {} {:?} — variant drifted",
            loaded_model.problem, loaded_model.variant
        );
        let loaded_instance =
            normalize_problem_instance(&loaded_model.problem_ref(), &loaded_model.instance);
        let computed_instance =
            normalize_problem_instance(&computed_model.problem_ref(), &computed_model.instance);
        assert!(
            json_semantically_equal(&loaded_instance, &computed_instance),
            "model fixture instance mismatch for {} {:?} — regenerate fixtures with: \
             cargo run --release --example regenerate_fixtures --features \"ilp-highs example-db\"",
            loaded_model.problem,
            loaded_model.variant
        );
        assert_eq!(
            loaded_model.samples, computed_model.samples,
            "model fixture sample evaluations mismatch for {} {:?} — regenerate fixtures with: \
             cargo run --release --example regenerate_fixtures --features \"ilp-highs example-db\"",
            loaded_model.problem, loaded_model.variant
        );
        assert_eq!(
            loaded_model.optimal, computed_model.optimal,
            "model fixture optima mismatch for {} {:?} — regenerate fixtures with: \
             cargo run --release --example regenerate_fixtures --features \"ilp-highs example-db\"",
            loaded_model.problem, loaded_model.variant
        );
    }
}

#[test]
fn verify_rule_fixtures_match_computed() {
    let loaded = build_rule_db().expect("fixture should load");
    let computed = compute_rule_db().expect("computed rule db should build");
    assert_eq!(
        loaded.rules.len(),
        computed.rules.len(),
        "fixture and computed rule counts differ — regenerate fixtures"
    );
    let loaded_keys: BTreeSet<_> = loaded
        .rules
        .iter()
        .map(|r| (r.source.problem_ref(), r.target.problem_ref()))
        .collect();
    let computed_keys: BTreeSet<_> = computed
        .rules
        .iter()
        .map(|r| (r.source.problem_ref(), r.target.problem_ref()))
        .collect();
    assert_eq!(
        loaded_keys, computed_keys,
        "fixture and computed rule sets differ — regenerate fixtures"
    );
    let loaded_by_key: BTreeMap<_, _> = loaded
        .rules
        .iter()
        .map(|rule| ((rule.source.problem_ref(), rule.target.problem_ref()), rule))
        .collect();
    let computed_by_key: BTreeMap<_, _> = computed
        .rules
        .iter()
        .map(|rule| ((rule.source.problem_ref(), rule.target.problem_ref()), rule))
        .collect();

    for key in loaded_keys {
        let loaded_rule = loaded_by_key
            .get(&key)
            .expect("loaded fixture key should exist");
        let computed_rule = computed_by_key
            .get(&key)
            .expect("computed fixture key should exist");

        let loaded_source = normalize_problem_instance(
            &loaded_rule.source.problem_ref(),
            &loaded_rule.source.instance,
        );
        let computed_source = normalize_problem_instance(
            &computed_rule.source.problem_ref(),
            &computed_rule.source.instance,
        );
        assert!(
            json_semantically_equal(&loaded_source, &computed_source),
            "source instance mismatch for {} -> {} — regenerate fixtures",
            loaded_rule.source.problem,
            loaded_rule.target.problem
        );
        let loaded_target = normalize_problem_instance(
            &loaded_rule.target.problem_ref(),
            &loaded_rule.target.instance,
        );
        let computed_target = normalize_problem_instance(
            &computed_rule.target.problem_ref(),
            &computed_rule.target.instance,
        );
        assert!(
            json_semantically_equal(&loaded_target, &computed_target),
            "target instance mismatch for {} -> {} — regenerate fixtures",
            loaded_rule.source.problem,
            loaded_rule.target.problem
        );
        // Solution witnesses may differ across platforms (ILP solver
        // nondeterminism), so compare energy (objective value) rather than
        // exact configs — both must be optimal.
        assert_eq!(
            loaded_rule.solutions.len(),
            computed_rule.solutions.len(),
            "solution count mismatch for {} -> {} — regenerate fixtures",
            loaded_rule.source.problem,
            loaded_rule.target.problem
        );
        let label = format!(
            "{} -> {}",
            loaded_rule.source.problem, loaded_rule.target.problem
        );
        for (loaded_pair, computed_pair) in loaded_rule
            .solutions
            .iter()
            .zip(computed_rule.solutions.iter())
        {
            let loaded_target_problem = load_dyn(
                &loaded_rule.target.problem,
                &loaded_rule.target.variant,
                loaded_rule.target.instance.clone(),
            )
            .unwrap_or_else(|e| panic!("{label}: load target: {e}"));
            let loaded_energy = loaded_target_problem.evaluate_dyn(&loaded_pair.target_config);
            let computed_energy = loaded_target_problem.evaluate_dyn(&computed_pair.target_config);
            assert_eq!(
                loaded_energy, computed_energy,
                "{label}: target energy mismatch — regenerate fixtures"
            );
        }
    }
}
