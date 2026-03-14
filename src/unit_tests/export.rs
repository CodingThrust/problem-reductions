use super::*;
use crate::expr::Expr;
use crate::rules::registry::ReductionOverhead;

#[test]
fn test_overhead_to_json_empty() {
    let overhead = ReductionOverhead::default();
    let entries = overhead_to_json(&overhead);
    assert!(entries.is_empty());
}

#[test]
fn test_overhead_to_json_single_field() {
    let overhead = ReductionOverhead::new(vec![("num_vertices", Expr::Var("n") + Expr::Var("m"))]);
    let entries = overhead_to_json(&overhead);
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].field, "num_vertices");
    assert_eq!(entries[0].formula, "n + m");
}

#[test]
fn test_overhead_to_json_constant() {
    let overhead = ReductionOverhead::new(vec![("num_vars", Expr::Const(42.0))]);
    let entries = overhead_to_json(&overhead);
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].field, "num_vars");
    assert_eq!(entries[0].formula, "42");
}

#[test]
fn test_overhead_to_json_scaled_power() {
    let overhead = ReductionOverhead::new(vec![(
        "num_edges",
        Expr::Const(3.0) * Expr::pow(Expr::Var("n"), Expr::Const(2.0)),
    )]);
    let entries = overhead_to_json(&overhead);
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].formula, "3 * n^2");
}

#[test]
fn test_overhead_to_json_multiple_fields() {
    let overhead = ReductionOverhead::new(vec![
        ("num_vertices", Expr::Var("n")),
        ("num_edges", Expr::pow(Expr::Var("n"), Expr::Const(2.0))),
    ]);
    let entries = overhead_to_json(&overhead);
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].field, "num_vertices");
    assert_eq!(entries[1].field, "num_edges");
}

#[test]
fn test_variant_to_map_empty() {
    let map = variant_to_map(vec![]);
    assert!(map.is_empty());
}

#[test]
fn test_variant_to_map_single() {
    let map = variant_to_map(vec![("graph", "SimpleGraph")]);
    assert_eq!(map.len(), 1);
    assert_eq!(map["graph"], "SimpleGraph");
}

#[test]
fn test_variant_to_map_multiple() {
    let map = variant_to_map(vec![("graph", "SimpleGraph"), ("weight", "i32")]);
    assert_eq!(map.len(), 2);
    assert_eq!(map["graph"], "SimpleGraph");
    assert_eq!(map["weight"], "i32");
}

#[test]
fn test_lookup_overhead_known_reduction() {
    // IS -> VC is a known registered reduction
    let source_variant = variant_to_map(vec![("graph", "SimpleGraph"), ("weight", "i32")]);
    let target_variant = variant_to_map(vec![("graph", "SimpleGraph"), ("weight", "i32")]);
    let result = lookup_overhead(
        "MaximumIndependentSet",
        &source_variant,
        "MinimumVertexCover",
        &target_variant,
    );
    assert!(result.is_some());
}

#[test]
fn test_lookup_overhead_unknown_reduction() {
    let empty = variant_to_map(vec![]);
    let result = lookup_overhead("NonExistent", &empty, "AlsoNonExistent", &empty);
    assert!(result.is_none());
}

#[test]
fn test_write_canonical_example_dbs() {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    let dir = std::env::temp_dir().join(format!(
        "problemreductions-export-db-test-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&dir).unwrap();

    let rule_db = RuleDb {
        version: EXAMPLE_DB_VERSION,
        rules: vec![RuleExample {
            source: ProblemSide {
                problem: "SourceProblem".to_string(),
                variant: variant_to_map(vec![("graph", "SimpleGraph")]),
                instance: serde_json::json!({"n": 3}),
            },
            target: ProblemSide {
                problem: "TargetProblem".to_string(),
                variant: variant_to_map(vec![("weight", "i32")]),
                instance: serde_json::json!({"m": 4}),
            },
            overhead: vec![],
            solutions: vec![],
        }],
    };
    let model_db = ModelDb {
        version: EXAMPLE_DB_VERSION,
        models: vec![ModelExample {
            problem: "ModelProblem".to_string(),
            variant: variant_to_map(vec![("graph", "SimpleGraph")]),
            instance: serde_json::json!({"n": 5}),
            samples: vec![],
            optimal: vec![],
        }],
    };

    write_rule_db_to(&dir, &rule_db);
    write_model_db_to(&dir, &model_db);

    let rules_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(dir.join("rules.json")).unwrap()).unwrap();
    let models_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(dir.join("models.json")).unwrap()).unwrap();

    assert_eq!(rules_json["version"], EXAMPLE_DB_VERSION);
    assert_eq!(rules_json["rules"][0]["source"]["problem"], "SourceProblem");
    assert_eq!(models_json["version"], EXAMPLE_DB_VERSION);
    assert_eq!(models_json["models"][0]["problem"], "ModelProblem");

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_problem_side_serialization() {
    let side = ProblemSide {
        problem: "MaximumIndependentSet".to_string(),
        variant: variant_to_map(vec![("graph", "SimpleGraph"), ("weight", "i32")]),
        instance: serde_json::json!({"num_vertices": 4, "edges": [[0, 1], [1, 2]]}),
    };
    let json = serde_json::to_value(&side).unwrap();
    assert_eq!(json["problem"], "MaximumIndependentSet");
    assert!(json["variant"]["graph"] == "SimpleGraph");
    assert!(json["instance"]["num_vertices"] == 4);
}

// ---- variant_to_map normalization ----

#[test]
fn export_variant_to_map_normalizes_empty_graph() {
    // When a variant has an empty graph value, variant_to_map should normalize
    // it to "SimpleGraph" for consistency with the reduction graph convention.
    let map = variant_to_map(vec![("graph", ""), ("weight", "i32")]);
    assert_eq!(
        map["graph"], "SimpleGraph",
        "variant_to_map should normalize empty graph to SimpleGraph"
    );
    assert_eq!(map["weight"], "i32");
}

#[test]
fn export_variant_to_map_preserves_explicit_graph() {
    let map = variant_to_map(vec![("graph", "PlanarGraph"), ("weight", "f64")]);
    assert_eq!(map["graph"], "PlanarGraph");
    assert_eq!(map["weight"], "f64");
}

// ---- ProblemSide::from_problem / ModelExample::from_problem ----

#[test]
fn problem_side_from_typed_problem() {
    use crate::models::graph::MaximumIndependentSet;
    use crate::topology::SimpleGraph;

    let g = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let mis = MaximumIndependentSet::new(g, vec![1, 1, 1]);
    let side = ProblemSide::from_problem(&mis);
    assert_eq!(side.problem, "MaximumIndependentSet");
    assert_eq!(side.variant["graph"], "SimpleGraph");
    assert!(side.instance.is_object());
}

#[test]
fn model_example_from_typed_problem() {
    use crate::models::graph::MaximumIndependentSet;
    use crate::topology::SimpleGraph;

    let g = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let mis = MaximumIndependentSet::new(g, vec![1, 1, 1]);
    let sample = SampleEval {
        config: vec![1, 0, 1],
        metric: serde_json::json!("Valid(2)"),
    };
    let example = ModelExample::from_problem(&mis, vec![sample.clone()], vec![sample]);
    assert_eq!(example.problem, "MaximumIndependentSet");
    assert!(!example.samples.is_empty());
    assert!(!example.optimal.is_empty());
    assert!(example.instance.is_object());
}

#[test]
fn model_example_problem_ref() {
    let example = ModelExample {
        problem: "TestProblem".to_string(),
        variant: variant_to_map(vec![("graph", "SimpleGraph")]),
        instance: serde_json::json!({}),
        samples: vec![],
        optimal: vec![],
    };
    let pref = example.problem_ref();
    assert_eq!(pref.name, "TestProblem");
    assert_eq!(pref.variant["graph"], "SimpleGraph");
}

#[test]
fn default_expr_returns_zero() {
    let expr = default_expr();
    assert_eq!(expr, Expr::Const(0.0));
}

#[test]
fn examples_output_dir_fallback() {
    // Without PROBLEMREDUCTIONS_EXAMPLES_DIR set, should fallback
    let dir = examples_output_dir();
    let expected = std::path::PathBuf::from("docs/paper/examples/generated");
    // Clean env first to ensure deterministic result
    if std::env::var_os(EXAMPLES_DIR_ENV).is_none() {
        assert_eq!(dir, expected);
    }
}

#[test]
fn examples_output_dir_env_override() {
    // Temporarily set the env var and check it's respected
    let key = EXAMPLES_DIR_ENV;
    let old = std::env::var_os(key);
    std::env::set_var(key, "/tmp/custom_examples");
    let dir = examples_output_dir();
    assert_eq!(dir, std::path::PathBuf::from("/tmp/custom_examples"));
    // Restore
    match old {
        Some(v) => std::env::set_var(key, v),
        None => std::env::remove_var(key),
    }
}

#[test]
fn write_rule_example_to_creates_json_file() {
    use std::fs;
    let dir = std::env::temp_dir().join(format!(
        "pr-export-rule-example-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let example = RuleExample {
        source: ProblemSide {
            problem: "A".to_string(),
            variant: variant_to_map(vec![]),
            instance: serde_json::json!({"x": 1}),
        },
        target: ProblemSide {
            problem: "B".to_string(),
            variant: variant_to_map(vec![]),
            instance: serde_json::json!({"y": 2}),
        },
        overhead: vec![],
        solutions: vec![],
    };
    write_rule_example_to(&dir, "test_rule", &example);
    let path = dir.join("test_rule.json");
    assert!(path.exists());
    let content: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
    assert_eq!(content["source"]["problem"], "A");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn write_model_example_to_creates_json_file() {
    use std::fs;
    let dir = std::env::temp_dir().join(format!(
        "pr-export-model-example-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let example = ModelExample {
        problem: "TestModel".to_string(),
        variant: variant_to_map(vec![("graph", "SimpleGraph")]),
        instance: serde_json::json!({"n": 3}),
        samples: vec![SampleEval {
            config: vec![1, 0, 1],
            metric: serde_json::json!("Valid(2)"),
        }],
        optimal: vec![],
    };
    write_model_example_to(&dir, "test_model", &example);
    let path = dir.join("test_model.json");
    assert!(path.exists());
    let content: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
    assert_eq!(content["problem"], "TestModel");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn lookup_overhead_rejects_target_variant_mismatch() {
    let source = variant_to_map(vec![("graph", "SimpleGraph"), ("weight", "i32")]);
    // MIS<SG,i32> -> QUBO<f64> exists, but not MIS<SG,i32> -> QUBO<i32>
    let wrong_target = variant_to_map(vec![("weight", "i32")]);
    let result = lookup_overhead("MaximumIndependentSet", &source, "QUBO", &wrong_target);
    assert!(result.is_none(), "Should reject wrong target variant");
}
