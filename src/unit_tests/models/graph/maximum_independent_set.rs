use super::*;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::{OptimizationProblem, Problem};
use crate::types::Direction;
include!("../../jl_helpers.rs");

#[test]
fn test_independent_set_creation() {
    let problem = MaximumIndependentSet::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]), vec![1i32; 4]);
    assert_eq!(problem.graph().num_vertices(), 4);
    assert_eq!(problem.graph().num_edges(), 3);
    assert_eq!(problem.dims().len(), 4);
}

#[test]
fn test_independent_set_with_weights() {
    let problem =
        MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1, 2, 3]);
    assert_eq!(problem.weights().to_vec(), vec![1, 2, 3]);
    assert!(problem.is_weighted());
}

#[test]
fn test_independent_set_unweighted() {
    // i32 type is always considered weighted, even with uniform values
    let problem = MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1i32; 3]);
    assert!(problem.is_weighted());
}

#[test]
fn test_has_edge() {
    let problem = MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1i32; 3]);
    assert!(problem.graph().has_edge(0, 1));
    assert!(problem.graph().has_edge(1, 0)); // Undirected
    assert!(problem.graph().has_edge(1, 2));
    assert!(!problem.graph().has_edge(0, 2));
}

#[test]
fn test_is_independent_set_function() {
    assert!(is_independent_set(
        &SimpleGraph::new(3, vec![(0, 1)]),
        &[true, false, true]
    ));
    assert!(is_independent_set(
        &SimpleGraph::new(3, vec![(0, 1)]),
        &[false, true, true]
    ));
    assert!(!is_independent_set(
        &SimpleGraph::new(3, vec![(0, 1)]),
        &[true, true, false]
    ));
    assert!(is_independent_set(
        &SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        &[true, false, true]
    ));
    assert!(!is_independent_set(
        &SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        &[false, true, true]
    ));
}

#[test]
fn test_direction() {
    let problem = MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1i32; 3]);
    assert_eq!(problem.direction(), Direction::Maximize);
}

#[test]
fn test_edges() {
    let problem = MaximumIndependentSet::new(SimpleGraph::new(4, vec![(0, 1), (2, 3)]), vec![1i32; 4]);
    let edges = problem.graph().edges();
    assert_eq!(edges.len(), 2);
    assert!(edges.contains(&(0, 1)) || edges.contains(&(1, 0)));
    assert!(edges.contains(&(2, 3)) || edges.contains(&(3, 2)));
}

#[test]
fn test_with_custom_weights() {
    let problem =
        MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![5, 10, 15]);
    assert_eq!(problem.weights().to_vec(), vec![5, 10, 15]);
}

#[test]
fn test_from_graph() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem =
        MaximumIndependentSet::new(graph.clone(), vec![1, 2, 3]);
    assert_eq!(problem.graph().num_vertices(), 3);
    assert_eq!(problem.weights().to_vec(), vec![1, 2, 3]);
}

#[test]
fn test_from_graph_with_unit_weights() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MaximumIndependentSet::new(graph, vec![1i32; 3]);
    assert_eq!(problem.graph().num_vertices(), 3);
    assert_eq!(problem.weights().to_vec(), vec![1, 1, 1]);
}

#[test]
fn test_graph_accessor() {
    let problem = MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1i32; 3]);
    let graph = problem.graph();
    assert_eq!(graph.num_vertices(), 3);
    assert_eq!(graph.num_edges(), 1);
}

#[test]
fn test_weights() {
    let problem =
        MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![5, 10, 15]);
    assert_eq!(problem.weights(), &[5, 10, 15]);
}

#[test]
fn test_problem_name() {
    assert_eq!(
        <MaximumIndependentSet<SimpleGraph, i32> as Problem>::NAME,
        "MaximumIndependentSet"
    );
}

#[test]
fn test_jl_parity_evaluation() {
    let data: serde_json::Value = serde_json::from_str(include_str!(
        "../../../../tests/data/jl/independentset.json"
    ))
    .unwrap();
    for instance in data["instances"].as_array().unwrap() {
        let nv = instance["instance"]["num_vertices"].as_u64().unwrap() as usize;
        let edges = jl_parse_edges(&instance["instance"]);
        let weights = jl_parse_i32_vec(&instance["instance"]["weights"]);
        let problem = MaximumIndependentSet::new(SimpleGraph::new(nv, edges), weights);
        for eval in instance["evaluations"].as_array().unwrap() {
            let config = jl_parse_config(&eval["config"]);
            let result = problem.evaluate(&config);
            let jl_valid = eval["is_valid"].as_bool().unwrap();
            assert_eq!(
                result.is_valid(),
                jl_valid,
                "IS validity mismatch for config {:?}",
                config
            );
            if jl_valid {
                let jl_size = eval["size"].as_i64().unwrap() as i32;
                assert_eq!(
                    result.unwrap(),
                    jl_size,
                    "IS size mismatch for config {:?}",
                    config
                );
            }
        }
        let best = BruteForce::new().find_all_best(&problem);
        let jl_best = jl_parse_configs_set(&instance["best_solutions"]);
        let rust_best: HashSet<Vec<usize>> = best.into_iter().collect();
        assert_eq!(rust_best, jl_best, "IS best solutions mismatch");
    }
}
