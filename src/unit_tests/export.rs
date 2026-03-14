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
fn test_write_example_creates_files() {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    let data = ReductionData {
        source: ProblemSide {
            problem: "TestProblem".to_string(),
            variant: variant_to_map(vec![("graph", "SimpleGraph")]),
            instance: serde_json::json!({"num_vertices": 3}),
        },
        target: ProblemSide {
            problem: "TargetProblem".to_string(),
            variant: variant_to_map(vec![]),
            instance: serde_json::json!({"num_vars": 5}),
        },
        overhead: vec![],
    };

    let results = ResultData {
        solutions: vec![SolutionPair {
            source_config: vec![1, 0, 1],
            target_config: vec![1, 0, 1, 0, 0],
        }],
    };

    let dir = std::env::temp_dir().join(format!(
        "problemreductions-export-test-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::env::set_var(EXAMPLES_DIR_ENV, &dir);
    write_example("_test_export", &data, &results);

    // Verify files exist and contain valid JSON
    let reduction_path = dir.join("_test_export.json");

    let reduction_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&reduction_path).unwrap()).unwrap();
    assert_eq!(reduction_json["source"]["problem"], "TestProblem");
    assert_eq!(reduction_json["target"]["problem"], "TargetProblem");
    assert_eq!(
        reduction_json["solutions"][0]["source_config"],
        serde_json::json!([1, 0, 1])
    );

    // Clean up test files
    let _ = fs::remove_dir_all(&dir);
    std::env::remove_var(EXAMPLES_DIR_ENV);
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

#[test]
fn test_reduction_data_serialization() {
    let data = ReductionData {
        source: ProblemSide {
            problem: "IS".to_string(),
            variant: variant_to_map(vec![]),
            instance: serde_json::json!({"n": 3}),
        },
        target: ProblemSide {
            problem: "VC".to_string(),
            variant: variant_to_map(vec![]),
            instance: serde_json::json!({"n": 3}),
        },
        overhead: vec![OverheadEntry {
            field: "num_vertices".to_string(),
            expr: Expr::Var("n"),
            formula: "n".to_string(),
        }],
    };
    let json = serde_json::to_value(&data).unwrap();
    assert_eq!(json["overhead"][0]["field"], "num_vertices");
    assert_eq!(json["overhead"][0]["formula"], "n");
}

#[test]
fn test_result_data_serialization() {
    let results = ResultData {
        solutions: vec![
            SolutionPair {
                source_config: vec![1, 0],
                target_config: vec![0, 1],
            },
            SolutionPair {
                source_config: vec![0, 1],
                target_config: vec![1, 0],
            },
        ],
    };
    let json = serde_json::to_value(&results).unwrap();
    assert_eq!(json["solutions"].as_array().unwrap().len(), 2);
    assert_eq!(
        json["solutions"][0]["source_config"],
        serde_json::json!([1, 0])
    );
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
