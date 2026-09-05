use super::*;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::solvers::BruteForce;
include!("../jl_helpers.rs");

#[test]
fn test_minimumvertexcover_to_minimumsetcovering_closed_loop() {
    // Path graph 0-1-2 with edges (0,1) and (1,2)
    // Vertex 0 covers edge 0
    // Vertex 1 covers edges 0 and 1
    // Vertex 2 covers edge 1
    let vc_problem =
        MinimumVertexCover::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1i64; 3]);
    let reduction = ReduceTo::<MinimumSetCovering<i64>>::reduce_to(&vc_problem)
        .expect("reduction should succeed");
    let sc_problem = reduction.target_problem();

    // Check the sets are constructed correctly
    assert_eq!(sc_problem.universe_size(), 2); // 2 edges
    assert_eq!(sc_problem.num_sets(), 3); // 3 vertices

    // Set 0 (vertex 0): should contain edge 0
    assert_eq!(sc_problem.get_set(0), Some(&vec![0]));
    // Set 1 (vertex 1): should contain edges 0 and 1
    assert_eq!(sc_problem.get_set(1), Some(&vec![0, 1]));
    // Set 2 (vertex 2): should contain edge 1
    assert_eq!(sc_problem.get_set(2), Some(&vec![1]));
}

#[test]
fn test_vc_to_sc_triangle() {
    // Triangle graph: 3 vertices, 3 edges
    // Edge indices: (0,1)->0, (1,2)->1, (0,2)->2
    let vc_problem = MinimumVertexCover::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![1i64; 3],
    );
    let reduction = ReduceTo::<MinimumSetCovering<i64>>::reduce_to(&vc_problem)
        .expect("reduction should succeed");
    let sc_problem = reduction.target_problem();

    assert_eq!(sc_problem.universe_size(), 3);
    assert_eq!(sc_problem.num_sets(), 3);

    // Verify each vertex covers exactly 2 edges
    for i in 0..3 {
        let set = sc_problem.get_set(i).unwrap();
        assert_eq!(set.len(), 2);
    }
}

#[test]
fn test_vc_to_sc_weighted() {
    // Weighted problem: weights should be preserved
    let vc_problem =
        MinimumVertexCover::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![10, 1, 10]);
    let reduction = ReduceTo::<MinimumSetCovering<i64>>::reduce_to(&vc_problem)
        .expect("reduction should succeed");
    let sc_problem = reduction.target_problem();

    // Weights should be preserved - access via weights_ref method on the problem
    assert_eq!(*sc_problem.weights_ref(), vec![10, 1, 10]);

    // Solve both ways
    let solver = BruteForce::new();
    let vc_solutions = solver.find_all_witnesses(&vc_problem).unwrap();
    let sc_solutions = solver.find_all_witnesses(sc_problem).unwrap();

    // Both should select vertex 1 (weight 1)
    assert_eq!(vc_solutions[0], vec![false, true, false]);
    assert_eq!(sc_solutions[0], vec![false, true, false]);
}

#[test]
fn test_vc_to_sc_empty_graph() {
    // Graph with no edges
    let vc_problem = MinimumVertexCover::new(SimpleGraph::new(3, vec![]), vec![1i64; 3]);
    let reduction = ReduceTo::<MinimumSetCovering<i64>>::reduce_to(&vc_problem)
        .expect("reduction should succeed");
    let sc_problem = reduction.target_problem();

    assert_eq!(sc_problem.universe_size(), 0);
    assert_eq!(sc_problem.num_sets(), 3);

    // All sets should be empty
    for i in 0..3 {
        assert!(sc_problem.get_set(i).unwrap().is_empty());
    }
}

#[test]
fn test_vc_to_sc_star_graph() {
    // Star graph: center vertex 0 connected to all others
    // Edges: (0,1), (0,2), (0,3)
    let vc_problem = MinimumVertexCover::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3)]),
        vec![1i64; 4],
    );
    let reduction = ReduceTo::<MinimumSetCovering<i64>>::reduce_to(&vc_problem)
        .expect("reduction should succeed");
    let sc_problem = reduction.target_problem();

    // Vertex 0 should cover all 3 edges
    assert_eq!(sc_problem.get_set(0), Some(&vec![0, 1, 2]));
    // Other vertices cover only 1 edge each
    assert_eq!(sc_problem.get_set(1), Some(&vec![0]));
    assert_eq!(sc_problem.get_set(2), Some(&vec![1]));
    assert_eq!(sc_problem.get_set(3), Some(&vec![2]));

    // Minimum cover should be just vertex 0
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&vc_problem).unwrap();
    assert_eq!(solutions[0], vec![true, false, false, false]);
}

#[test]
fn test_jl_parity_vc_to_setcovering() {
    let data: serde_json::Value = serde_json::from_str(include_str!(
        "../../../tests/data/jl/vertexcovering_to_setcovering.json"
    ))
    .unwrap();
    let vc_data: serde_json::Value =
        serde_json::from_str(include_str!("../../../tests/data/jl/vertexcovering.json")).unwrap();
    let inst = &vc_data["instances"][0]["instance"];
    let nv = inst["num_vertices"].as_u64().unwrap() as usize;
    let source = MinimumVertexCover::new(
        SimpleGraph::new(nv, jl_parse_edges(inst)),
        jl_parse_i64_vec(&inst["weights"]),
    );
    let result =
        ReduceTo::<MinimumSetCovering<i64>>::reduce_to(&source).expect("reduction should succeed");
    let solver = BruteForce::new();
    let best_source: HashSet<Vec<bool>> = solver
        .find_all_witnesses(&source)
        .unwrap()
        .into_iter()
        .collect();
    assert_optimization_round_trip_from_optimization_target(
        &source,
        &result,
        "JL parity VC->SetCovering",
    );
    for case in data["cases"].as_array().unwrap() {
        assert_eq!(best_source, jl_parse_bool_configs_set(&case["best_source"]));
    }
}

#[test]
fn test_jl_parity_rule_vc_to_setcovering() {
    let data: serde_json::Value = serde_json::from_str(include_str!(
        "../../../tests/data/jl/rule_vertexcovering_to_setcovering.json"
    ))
    .unwrap();
    let vc_data: serde_json::Value =
        serde_json::from_str(include_str!("../../../tests/data/jl/vertexcovering.json")).unwrap();
    let inst = &jl_find_instance_by_label(&vc_data, "rule_4vertex")["instance"];
    let nv = inst["num_vertices"].as_u64().unwrap() as usize;
    let source = MinimumVertexCover::new(
        SimpleGraph::new(nv, jl_parse_edges(inst)),
        jl_parse_i64_vec(&inst["weights"]),
    );
    let result =
        ReduceTo::<MinimumSetCovering<i64>>::reduce_to(&source).expect("reduction should succeed");
    let solver = BruteForce::new();
    let best_source: HashSet<Vec<bool>> = solver
        .find_all_witnesses(&source)
        .unwrap()
        .into_iter()
        .collect();
    assert_optimization_round_trip_from_optimization_target(
        &source,
        &result,
        "JL parity rule VC->SetCovering",
    );
    for case in data["cases"].as_array().unwrap() {
        assert_eq!(best_source, jl_parse_bool_configs_set(&case["best_source"]));
    }
}
