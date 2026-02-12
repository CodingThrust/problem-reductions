use super::*;
use crate::polynomial::Polynomial;
use crate::rules::registry::ReductionOverhead;

#[test]
fn test_overhead_to_json_empty() {
    let overhead = ReductionOverhead::default();
    let entries = overhead_to_json(&overhead);
    assert!(entries.is_empty());
}

#[test]
fn test_overhead_to_json_single_field() {
    let overhead = ReductionOverhead::new(vec![(
        "num_vertices",
        Polynomial::var("n") + Polynomial::var("m"),
    )]);
    let entries = overhead_to_json(&overhead);
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].field, "num_vertices");
    assert_eq!(entries[0].polynomial.len(), 2);

    // Check first monomial: 1*n
    assert_eq!(entries[0].polynomial[0].coefficient, 1.0);
    assert_eq!(
        entries[0].polynomial[0].variables,
        vec![("n".to_string(), 1)]
    );

    // Check second monomial: 1*m
    assert_eq!(entries[0].polynomial[1].coefficient, 1.0);
    assert_eq!(
        entries[0].polynomial[1].variables,
        vec![("m".to_string(), 1)]
    );
}

#[test]
fn test_overhead_to_json_constant_monomial() {
    let overhead = ReductionOverhead::new(vec![("num_vars", Polynomial::constant(42.0))]);
    let entries = overhead_to_json(&overhead);
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].field, "num_vars");
    assert_eq!(entries[0].polynomial.len(), 1);
    assert_eq!(entries[0].polynomial[0].coefficient, 42.0);
    assert!(entries[0].polynomial[0].variables.is_empty());
}

#[test]
fn test_overhead_to_json_scaled_power() {
    let overhead =
        ReductionOverhead::new(vec![("num_edges", Polynomial::var_pow("n", 2).scale(3.0))]);
    let entries = overhead_to_json(&overhead);
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].polynomial.len(), 1);
    assert_eq!(entries[0].polynomial[0].coefficient, 3.0);
    assert_eq!(
        entries[0].polynomial[0].variables,
        vec![("n".to_string(), 2)]
    );
}

#[test]
fn test_overhead_to_json_multiple_fields() {
    let overhead = ReductionOverhead::new(vec![
        ("num_vertices", Polynomial::var("n")),
        ("num_edges", Polynomial::var_pow("n", 2)),
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
    let result = lookup_overhead("MaximumIndependentSet", "MinimumVertexCover");
    assert!(result.is_some());
}

#[test]
fn test_lookup_overhead_unknown_reduction() {
    let result = lookup_overhead("NonExistent", "AlsoNonExistent");
    assert!(result.is_none());
}

#[test]
fn test_lookup_overhead_or_empty_known() {
    let overhead = lookup_overhead_or_empty("MaximumIndependentSet", "MinimumVertexCover");
    assert!(!overhead.output_size.is_empty());
}

#[test]
fn test_lookup_overhead_or_empty_unknown() {
    let overhead = lookup_overhead_or_empty("NonExistent", "AlsoNonExistent");
    assert!(overhead.output_size.is_empty());
}

#[test]
fn test_write_example_creates_files() {
    use std::fs;

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

    write_example("_test_export", &data, &results);

    // Verify files exist and contain valid JSON
    let reduction_path = "docs/paper/examples/_test_export.json";
    let results_path = "docs/paper/examples/_test_export.result.json";

    let reduction_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(reduction_path).unwrap()).unwrap();
    assert_eq!(reduction_json["source"]["problem"], "TestProblem");
    assert_eq!(reduction_json["target"]["problem"], "TargetProblem");

    let results_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(results_path).unwrap()).unwrap();
    assert_eq!(
        results_json["solutions"][0]["source_config"],
        serde_json::json!([1, 0, 1])
    );

    // Clean up test files
    let _ = fs::remove_file(reduction_path);
    let _ = fs::remove_file(results_path);
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
            polynomial: vec![MonomialJson {
                coefficient: 1.0,
                variables: vec![("n".to_string(), 1)],
            }],
        }],
    };
    let json = serde_json::to_value(&data).unwrap();
    assert_eq!(json["overhead"][0]["field"], "num_vertices");
    assert_eq!(json["overhead"][0]["polynomial"][0]["coefficient"], 1.0);
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
