use super::*;
use crate::solvers::BruteForce;
include!("../jl_helpers.rs");

#[test]
fn test_weighted_reduction() {
    // Test with weighted problems
    let is_problem = MaximumIndependentSet::with_weights(3, vec![(0, 1), (1, 2)], vec![10, 20, 30]);
    let reduction = ReduceTo::<MinimumVertexCover<SimpleGraph, i32>>::reduce_to(&is_problem);
    let vc_problem = reduction.target_problem();

    // Weights should be preserved
    assert_eq!(vc_problem.weights().to_vec(), vec![10, 20, 30]);
}

#[test]
fn test_reduction_structure() {
    let is_problem =
        MaximumIndependentSet::<SimpleGraph, i32>::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]);
    let reduction = ReduceTo::<MinimumVertexCover<SimpleGraph, i32>>::reduce_to(&is_problem);
    let vc = reduction.target_problem();

    // Same number of vertices in both problems
    assert_eq!(vc.graph().num_vertices(), 5);
}

#[test]
fn test_jl_parity_is_to_vertexcovering() {
    let data: serde_json::Value = serde_json::from_str(include_str!(
        "../../../tests/data/jl/independentset_to_vertexcovering.json"
    ))
    .unwrap();
    let is_data: serde_json::Value =
        serde_json::from_str(include_str!("../../../tests/data/jl/independentset.json")).unwrap();
    let inst = &is_data["instances"][0]["instance"];
    let source = MaximumIndependentSet::<SimpleGraph, i32>::new(
        inst["num_vertices"].as_u64().unwrap() as usize,
        jl_parse_edges(inst),
    );
    let result = ReduceTo::<MinimumVertexCover<SimpleGraph, i32>>::reduce_to(&source);
    let solver = BruteForce::new();
    let best_target = solver.find_all_best(result.target_problem());
    let best_source: HashSet<Vec<usize>> = solver.find_all_best(&source).into_iter().collect();
    let extracted: HashSet<Vec<usize>> = best_target
        .iter()
        .map(|t| result.extract_solution(t))
        .collect();
    assert!(extracted.is_subset(&best_source));
    for case in data["cases"].as_array().unwrap() {
        assert_eq!(best_source, jl_parse_configs_set(&case["best_source"]));
    }
}

#[test]
fn test_jl_parity_rule_is_to_vertexcovering() {
    let data: serde_json::Value = serde_json::from_str(include_str!(
        "../../../tests/data/jl/rule2_independentset_to_vertexcovering.json"
    ))
    .unwrap();
    let is_data: serde_json::Value =
        serde_json::from_str(include_str!("../../../tests/data/jl/independentset.json")).unwrap();
    let inst = &jl_find_instance_by_label(&is_data, "doc_4vertex")["instance"];
    let source = MaximumIndependentSet::<SimpleGraph, i32>::new(
        inst["num_vertices"].as_u64().unwrap() as usize,
        jl_parse_edges(inst),
    );
    let result = ReduceTo::<MinimumVertexCover<SimpleGraph, i32>>::reduce_to(&source);
    let solver = BruteForce::new();
    let best_target = solver.find_all_best(result.target_problem());
    let best_source: HashSet<Vec<usize>> = solver.find_all_best(&source).into_iter().collect();
    let extracted: HashSet<Vec<usize>> = best_target
        .iter()
        .map(|t| result.extract_solution(t))
        .collect();
    assert!(extracted.is_subset(&best_source));
    for case in data["cases"].as_array().unwrap() {
        assert_eq!(best_source, jl_parse_configs_set(&case["best_source"]));
    }
}
