use super::*;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::solvers::BruteForce;
use crate::traits::Problem;
include!("../jl_helpers.rs");

#[test]
fn test_weighted_independent_set_to_vertex_cover_round_trip() {
    let is_problem =
        MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![10, 20, 30]);
    let reduction = ReduceTo::<MinimumVertexCover<SimpleGraph, i64>>::reduce_to(&is_problem)
        .expect("reduction should succeed");
    let solutions = BruteForce::new()
        .find_all_witnesses(reduction.target_problem())
        .unwrap();
    assert_eq!(solutions, vec![vec![false, true, false]]);
    let extracted = reduction.extract_solution(&solutions[0]).unwrap();
    assert_eq!(extracted, vec![true, false, true]);
    assert_eq!(
        is_problem.evaluate(&extracted).unwrap(),
        crate::types::Max(Some(40))
    );
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
        MaximumIndependentSet::new(SimpleGraph::new(nv, jl_parse_edges(inst)), vec![1i64; nv]);
    let result = ReduceTo::<MinimumVertexCover<SimpleGraph, i64>>::reduce_to(&source)
        .expect("reduction should succeed");
    let solver = BruteForce::new();
    let best_source: HashSet<Vec<bool>> = solver
        .find_all_witnesses(&source)
        .unwrap()
        .into_iter()
        .collect();
    assert_optimization_round_trip_from_optimization_target(&source, &result, "JL parity MIS->VC");
    for case in data["cases"].as_array().unwrap() {
        assert_eq!(best_source, jl_parse_bool_configs_set(&case["best_source"]));
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
        MaximumIndependentSet::new(SimpleGraph::new(nv, jl_parse_edges(inst)), vec![1i64; nv]);
    let result = ReduceTo::<MinimumVertexCover<SimpleGraph, i64>>::reduce_to(&source)
        .expect("reduction should succeed");
    let solver = BruteForce::new();
    let best_source: HashSet<Vec<bool>> = solver
        .find_all_witnesses(&source)
        .unwrap()
        .into_iter()
        .collect();
    assert_optimization_round_trip_from_optimization_target(
        &source,
        &result,
        "JL parity rule MIS->VC",
    );
    for case in data["cases"].as_array().unwrap() {
        assert_eq!(best_source, jl_parse_bool_configs_set(&case["best_source"]));
    }
}
