use super::*;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::{ObjectiveProblem, Problem};
use crate::types::ExtremumSense;
include!("../../jl_helpers.rs");

#[test]
fn test_vertex_cover_creation() {
    let problem = MinimumVertexCover::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![1i32; 4],
    );
    assert_eq!(problem.graph().num_vertices(), 4);
    assert_eq!(problem.graph().num_edges(), 3);
    assert_eq!(problem.num_variables(), 4);
}

#[test]
fn test_vertex_cover_with_weights() {
    let problem = MinimumVertexCover::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1, 2, 3]);
    assert_eq!(problem.weights().to_vec(), vec![1, 2, 3]);
}

#[test]
fn test_is_vertex_cover_function() {
    assert!(is_vertex_cover(
        &SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        &[false, true, false]
    ));
    assert!(is_vertex_cover(
        &SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        &[true, false, true]
    ));
    assert!(!is_vertex_cover(
        &SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        &[true, false, false]
    ));
    assert!(!is_vertex_cover(
        &SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        &[false, false, false]
    ));
}

#[test]
fn test_direction() {
    let problem = MinimumVertexCover::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1i32; 3]);
    assert_eq!(problem.direction(), ExtremumSense::Minimize);
}

#[test]
fn test_complement_relationship() {
    // For a graph, if S is an independent set, then V\S is a vertex cover
    use crate::models::graph::MaximumIndependentSet;

    let edges = vec![(0, 1), (1, 2), (2, 3)];
    let is_problem = MaximumIndependentSet::new(SimpleGraph::new(4, edges.clone()), vec![1i32; 4]);
    let vc_problem = MinimumVertexCover::new(SimpleGraph::new(4, edges), vec![1i32; 4]);

    let solver = BruteForce::new();

    let is_solutions = solver.find_all_witnesses(&is_problem);
    for is_sol in &is_solutions {
        // Complement should be a valid vertex cover
        let vc_config: Vec<usize> = is_sol.iter().map(|&x| 1 - x).collect();
        // Valid cover should return Valid
        assert!(Problem::evaluate(&vc_problem, &vc_config).is_valid());
    }
}

#[test]
#[should_panic(expected = "selected length must match num_vertices")]
fn test_is_vertex_cover_wrong_len() {
    // Wrong length should panic
    is_vertex_cover(&SimpleGraph::new(3, vec![(0, 1)]), &[true, false]);
}

#[test]
fn test_from_graph() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumVertexCover::new(graph, vec![1i32, 1, 1]);
    assert_eq!(problem.graph().num_vertices(), 3);
    assert_eq!(problem.graph().num_edges(), 2);
}

#[test]
fn test_from_graph_with_weights() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumVertexCover::new(graph, vec![1, 2, 3]);
    assert_eq!(problem.weights().to_vec(), vec![1, 2, 3]);
}

#[test]
fn test_graph_accessor() {
    let problem = MinimumVertexCover::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1i32; 3]);
    let graph = problem.graph();
    assert_eq!(graph.num_vertices(), 3);
    assert_eq!(graph.num_edges(), 2);
}

#[test]
fn test_has_edge() {
    let problem = MinimumVertexCover::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1i32; 3]);
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
        let problem = MinimumVertexCover::new(SimpleGraph::new(nv, edges), weights);
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
        let best = BruteForce::new().find_all_witnesses(&problem);
        let jl_best = jl_parse_configs_set(&instance["best_solutions"]);
        let rust_best: HashSet<Vec<usize>> = best.into_iter().collect();
        assert_eq!(rust_best, jl_best, "VC best solutions mismatch");
    }
}

#[test]
fn test_is_valid_solution() {
    // Path graph: 0-1-2
    let problem = MinimumVertexCover::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1i32; 3]);
    // Valid: {1} covers both edges
    assert!(problem.is_valid_solution(&[0, 1, 0]));
    // Invalid: {0} doesn't cover edge (1,2)
    assert!(!problem.is_valid_solution(&[1, 0, 0]));
}

#[test]
fn test_size_getters() {
    let problem = MinimumVertexCover::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1i32; 3]);
    assert_eq!(problem.num_vertices(), 3);
    assert_eq!(problem.num_edges(), 2);
}

#[test]
fn test_mvc_paper_example() {
    // Paper: house graph, VC = {v_0, v_3, v_4}, weight = 3
    let graph = SimpleGraph::new(5, vec![(0, 1), (0, 2), (1, 3), (2, 3), (2, 4), (3, 4)]);
    let problem = MinimumVertexCover::new(graph, vec![1i32; 5]);
    let config = vec![1, 0, 0, 1, 1]; // {v_0, v_3, v_4}
    let result = problem.evaluate(&config);
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 3);

    let solver = BruteForce::new();
    let best = solver.find_witness(&problem).unwrap();
    assert_eq!(problem.evaluate(&best).unwrap(), 3);
}
