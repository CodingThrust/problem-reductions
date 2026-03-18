use super::*;

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
fn test_write_canonical_example_db() {
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

    let db = ExampleDb {
        models: vec![ModelExample {
            problem: "ModelProblem".to_string(),
            variant: variant_to_map(vec![("graph", "SimpleGraph")]),
            instance: serde_json::json!({"n": 5}),
            optimal_config: vec![],
            optimal_value: serde_json::json!(null),
        }],
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
            solutions: vec![],
        }],
    };
    write_example_db_to(&dir, &db);

    let examples_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(dir.join("examples.json")).unwrap()).unwrap();

    assert_eq!(
        examples_json["rules"][0]["source"]["problem"],
        "SourceProblem"
    );
    assert_eq!(examples_json["models"][0]["problem"], "ModelProblem");
    assert!(
        !dir.join("rules.json").exists(),
        "canonical export should not split rules into a separate file"
    );
    assert!(
        !dir.join("models.json").exists(),
        "canonical export should not split models into a separate file"
    );

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_write_example_db_uses_wrapped_json_contract() {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    let dir = std::env::temp_dir().join(format!(
        "problemreductions-export-db-contract-test-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&dir).unwrap();

    let db = ExampleDb {
        models: vec![ModelExample {
            problem: "ModelProblem".to_string(),
            variant: variant_to_map(vec![("graph", "SimpleGraph")]),
            instance: serde_json::json!({"n": 5}),
            optimal_config: vec![],
            optimal_value: serde_json::json!(null),
        }],
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
            solutions: vec![],
        }],
    };
    write_example_db_to(&dir, &db);

    let examples_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(dir.join("examples.json")).unwrap()).unwrap();

    assert_eq!(
        examples_json["rules"][0]["source"]["problem"],
        "SourceProblem"
    );
    assert_eq!(examples_json["models"][0]["problem"], "ModelProblem");

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_write_example_db_uses_one_line_per_example_entry() {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    let dir = std::env::temp_dir().join(format!(
        "problemreductions-export-db-lines-test-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&dir).unwrap();

    let db = ExampleDb {
        models: vec![ModelExample {
            problem: "ModelProblem".to_string(),
            variant: variant_to_map(vec![("graph", "SimpleGraph")]),
            instance: serde_json::json!({"n": 5, "edges": [[0, 1], [1, 2]]}),
            optimal_config: vec![1, 0, 1],
            optimal_value: serde_json::json!({"Valid": 2}),
        }],
        rules: vec![RuleExample {
            source: ProblemSide {
                problem: "SourceProblem".to_string(),
                variant: variant_to_map(vec![("graph", "SimpleGraph")]),
                instance: serde_json::json!({"n": 3, "edges": [[0, 1], [1, 2]]}),
            },
            target: ProblemSide {
                problem: "TargetProblem".to_string(),
                variant: variant_to_map(vec![("weight", "i32")]),
                instance: serde_json::json!({"m": 4, "weights": [1, 2, 3, 4]}),
            },
            solutions: vec![SolutionPair {
                source_config: vec![1, 0, 1],
                target_config: vec![0, 1, 1, 0],
            }],
        }],
    };
    write_example_db_to(&dir, &db);

    let text = fs::read_to_string(dir.join("examples.json")).unwrap();
    let model_line = text
        .lines()
        .find(|line| line.contains("\"problem\":\"ModelProblem\""))
        .expect("model entry should appear on a single line");
    let rule_line = text
        .lines()
        .find(|line| line.contains("\"problem\":\"SourceProblem\""))
        .expect("rule entry should appear on a single line");

    assert!(
        model_line.trim().starts_with('{')
            && model_line.trim().trim_end_matches(',').ends_with('}'),
        "model entry should be serialized as one compact JSON object line"
    );
    assert!(
        rule_line.trim().starts_with('{') && rule_line.trim().trim_end_matches(',').ends_with('}'),
        "rule entry should be serialized as one compact JSON object line"
    );

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn rule_example_serialization_omits_overhead() {
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
        solutions: vec![],
    };

    let json = serde_json::to_value(&example).unwrap();
    assert!(
        json.get("overhead").is_none(),
        "RuleExample should not duplicate reduction metadata"
    );
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

// ---- ProblemSide::from_problem / ModelExample::new ----

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
fn model_example_new() {
    let example = ModelExample::new(
        "MaximumIndependentSet",
        variant_to_map(vec![("graph", "SimpleGraph"), ("weight", "i32")]),
        serde_json::json!({"num_vertices": 3, "edges": [[0, 1], [1, 2]]}),
        vec![1, 0, 1],
        serde_json::json!({"Valid": 2}),
    );
    assert_eq!(example.problem, "MaximumIndependentSet");
    assert_eq!(example.optimal_config, vec![1, 0, 1]);
    assert_eq!(example.optimal_value, serde_json::json!({"Valid": 2}));
    assert!(example.instance.is_object());
}

#[test]
fn model_example_problem_ref() {
    let example = ModelExample {
        problem: "TestProblem".to_string(),
        variant: variant_to_map(vec![("graph", "SimpleGraph")]),
        instance: serde_json::json!({}),
        optimal_config: vec![],
        optimal_value: serde_json::json!(null),
    };
    let pref = example.problem_ref();
    assert_eq!(pref.name, "TestProblem");
    assert_eq!(pref.variant["graph"], "SimpleGraph");
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
        optimal_config: vec![1, 0, 1],
        optimal_value: serde_json::json!({"Valid": 2}),
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
