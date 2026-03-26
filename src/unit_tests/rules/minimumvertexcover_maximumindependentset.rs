use super::*;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::solvers::BruteForce;
include!("../jl_helpers.rs");

#[test]
fn test_minimumvertexcover_to_maximumindependentset_closed_loop() {
    // Test with weighted problems
    let is_problem =
        MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![10, 20, 30]);
    let reduction = ReduceTo::<MinimumVertexCover<SimpleGraph, i32>>::reduce_to(&is_problem);
    let vc_problem = reduction.target_problem();

    // Weights should be preserved
    assert_eq!(vc_problem.weights().to_vec(), vec![10, 20, 30]);
}

#[test]
fn test_reduction_structure() {
    let is_problem = MaximumIndependentSet::new(
        SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]),
        vec![1i32; 5],
    );
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
    let nv = inst["num_vertices"].as_u64().unwrap() as usize;
    let source =
        MaximumIndependentSet::new(SimpleGraph::new(nv, jl_parse_edges(inst)), vec![1i32; nv]);
    let result = ReduceTo::<MinimumVertexCover<SimpleGraph, i32>>::reduce_to(&source);
    let solver = BruteForce::new();
    let best_source: HashSet<Vec<usize>> = solver.find_all_witnesses(&source).into_iter().collect();
    assert_optimization_round_trip_from_optimization_target(&source, &result, "JL parity MIS->VC");
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
    let nv = inst["num_vertices"].as_u64().unwrap() as usize;
    let source =
        MaximumIndependentSet::new(SimpleGraph::new(nv, jl_parse_edges(inst)), vec![1i32; nv]);
    let result = ReduceTo::<MinimumVertexCover<SimpleGraph, i32>>::reduce_to(&source);
    let solver = BruteForce::new();
    let best_source: HashSet<Vec<usize>> = solver.find_all_witnesses(&source).into_iter().collect();
    assert_optimization_round_trip_from_optimization_target(
        &source,
        &result,
        "JL parity rule MIS->VC",
    );
    for case in data["cases"].as_array().unwrap() {
        assert_eq!(best_source, jl_parse_configs_set(&case["best_source"]));
    }
}
