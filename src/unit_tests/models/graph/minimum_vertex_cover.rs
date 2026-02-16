use super::*;
use crate::solvers::BruteForce;
use crate::traits::{OptimizationProblem, Problem};
use crate::types::Direction;
include!("../../jl_helpers.rs");

#[test]
fn test_vertex_cover_creation() {
    let problem = MinimumVertexCover::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    assert_eq!(problem.graph().num_vertices(), 4);
    assert_eq!(problem.graph().num_edges(), 3);
    assert_eq!(problem.num_variables(), 4);
}

#[test]
fn test_vertex_cover_with_weights() {
    let problem =
        MinimumVertexCover::<SimpleGraph, i32>::with_weights(3, vec![(0, 1)], vec![1, 2, 3]);
    assert_eq!(problem.weights().to_vec(), vec![1, 2, 3]);
}

#[test]
fn test_is_vertex_cover_function() {
    assert!(is_vertex_cover(3, &[(0, 1), (1, 2)], &[false, true, false]));
    assert!(is_vertex_cover(3, &[(0, 1), (1, 2)], &[true, false, true]));
    assert!(!is_vertex_cover(
        3,
        &[(0, 1), (1, 2)],
        &[true, false, false]
    ));
    assert!(!is_vertex_cover(
        3,
        &[(0, 1), (1, 2)],
        &[false, false, false]
    ));
}

#[test]
fn test_direction() {
    let problem = MinimumVertexCover::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
    assert_eq!(problem.direction(), Direction::Minimize);
}

#[test]
fn test_complement_relationship() {
    // For a graph, if S is an independent set, then V\S is a vertex cover
    use crate::models::graph::MaximumIndependentSet;

    let edges = vec![(0, 1), (1, 2), (2, 3)];
    let is_problem = MaximumIndependentSet::new(SimpleGraph::new(4, edges.clone()), vec![1i32; 4]);
    let vc_problem = MinimumVertexCover::<SimpleGraph, i32>::new(4, edges);

    let solver = BruteForce::new();

    let is_solutions = solver.find_all_best(&is_problem);
    for is_sol in &is_solutions {
        // Complement should be a valid vertex cover
        let vc_config: Vec<usize> = is_sol.iter().map(|&x| 1 - x).collect();
        // Valid cover should return Valid
        assert!(Problem::evaluate(&vc_problem, &vc_config).is_valid());
    }
}

#[test]
fn test_is_vertex_cover_wrong_len() {
    // Wrong length should return false
    assert!(!is_vertex_cover(3, &[(0, 1)], &[true, false]));
}

#[test]
fn test_from_graph() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumVertexCover::<SimpleGraph, i32>::from_graph(graph, vec![1, 1, 1]);
    assert_eq!(problem.graph().num_vertices(), 3);
    assert_eq!(problem.graph().num_edges(), 2);
}

#[test]
fn test_from_graph_with_weights() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumVertexCover::<SimpleGraph, i32>::from_graph(graph, vec![1, 2, 3]);
    assert_eq!(problem.weights().to_vec(), vec![1, 2, 3]);
}

#[test]
fn test_graph_accessor() {
    let problem = MinimumVertexCover::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
    let graph = problem.graph();
    assert_eq!(graph.num_vertices(), 3);
    assert_eq!(graph.num_edges(), 2);
}

#[test]
fn test_has_edge() {
    let problem = MinimumVertexCover::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
    assert!(problem.graph().has_edge(0, 1));
    assert!(problem.graph().has_edge(1, 0)); // Undirected
    assert!(problem.graph().has_edge(1, 2));
    assert!(!problem.graph().has_edge(0, 2));
}

#[test]
fn test_jl_parity_evaluation() {
    let data: serde_json::Value = serde_json::from_str(include_str!(
        "../../../../tests/data/jl/vertexcovering.json"
    ))
    .unwrap();
    for instance in data["instances"].as_array().unwrap() {
        let nv = instance["instance"]["num_vertices"].as_u64().unwrap() as usize;
        let edges = jl_parse_edges(&instance["instance"]);
        let weights = jl_parse_i32_vec(&instance["instance"]["weights"]);
        let problem = if weights.iter().all(|&w| w == 1) {
            MinimumVertexCover::<SimpleGraph, i32>::new(nv, edges)
        } else {
            MinimumVertexCover::with_weights(nv, edges, weights)
        };
        for eval in instance["evaluations"].as_array().unwrap() {
            let config = jl_parse_config(&eval["config"]);
            let result = problem.evaluate(&config);
            let jl_valid = eval["is_valid"].as_bool().unwrap();
            assert_eq!(
                result.is_valid(),
                jl_valid,
                "VC validity mismatch for config {:?}",
                config
            );
            if jl_valid {
                let jl_size = eval["size"].as_i64().unwrap() as i32;
                assert_eq!(
                    result.unwrap(),
                    jl_size,
                    "VC size mismatch for config {:?}",
                    config
                );
            }
        }
        let best = BruteForce::new().find_all_best(&problem);
        let jl_best = jl_parse_configs_set(&instance["best_solutions"]);
        let rust_best: HashSet<Vec<usize>> = best.into_iter().collect();
        assert_eq!(rust_best, jl_best, "VC best solutions mismatch");
    }
}
