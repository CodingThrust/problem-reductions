use super::*;
use crate::solvers::BruteForce;
include!("../../jl_helpers.rs");

#[test]
fn test_maximal_is_creation() {
    let problem = MaximalIS::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    assert_eq!(problem.num_vertices(), 4);
    assert_eq!(problem.num_edges(), 3);
}

#[test]
fn test_maximal_is_with_weights() {
    let problem = MaximalIS::<SimpleGraph, i32>::with_weights(3, vec![(0, 1)], vec![1, 2, 3]);
    assert_eq!(problem.weights(), vec![1, 2, 3]);
    assert!(problem.is_weighted());
}

#[test]
fn test_maximal_is_from_graph() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MaximalIS::<SimpleGraph, i32>::from_graph(graph, vec![1, 2, 3]);
    assert_eq!(problem.num_vertices(), 3);
    assert_eq!(problem.weights(), vec![1, 2, 3]);
}

#[test]
fn test_maximal_is_from_graph_unit_weights() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MaximalIS::<SimpleGraph, i32>::from_graph_unit_weights(graph);
    assert_eq!(problem.num_vertices(), 3);
    assert_eq!(problem.weights(), vec![1, 1, 1]);
}

#[test]
fn test_is_independent() {
    let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);

    assert!(problem.is_independent(&[1, 0, 1]));
    assert!(problem.is_independent(&[0, 1, 0]));
    assert!(!problem.is_independent(&[1, 1, 0]));
}

#[test]
fn test_is_maximal() {
    let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);

    // {0, 2} is maximal (cannot add 1)
    assert!(problem.is_maximal(&[1, 0, 1]));

    // {1} is maximal (cannot add 0 or 2)
    assert!(problem.is_maximal(&[0, 1, 0]));

    // {0} is not maximal (can add 2)
    assert!(!problem.is_maximal(&[1, 0, 0]));

    // {} is not maximal (can add any vertex)
    assert!(!problem.is_maximal(&[0, 0, 0]));
}

#[test]
fn test_is_maximal_independent_set_function() {
    let edges = vec![(0, 1), (1, 2)];

    assert!(is_maximal_independent_set(3, &edges, &[true, false, true]));
    assert!(is_maximal_independent_set(3, &edges, &[false, true, false]));
    assert!(!is_maximal_independent_set(
        3,
        &edges,
        &[true, false, false]
    )); // Can add 2
    assert!(!is_maximal_independent_set(3, &edges, &[true, true, false])); // Not independent
}

#[test]
fn test_direction() {
    use crate::traits::OptimizationProblem;
    use crate::types::Direction;

    let problem = MaximalIS::<SimpleGraph, i32>::new(2, vec![(0, 1)]);
    assert_eq!(problem.direction(), Direction::Maximize);
}

#[test]
fn test_weights() {
    let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
    let weights = problem.weights();
    assert_eq!(weights, vec![1, 1, 1]); // Unit weights
}

#[test]
fn test_set_weights() {
    let mut problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
    problem.set_weights(vec![1, 2, 3]);
    assert_eq!(problem.weights(), vec![1, 2, 3]);
}

#[test]
fn test_is_weighted() {
    let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
    assert!(!problem.is_weighted()); // Initially uniform
}

#[test]
fn test_is_weighted_empty() {
    let problem = MaximalIS::<SimpleGraph, i32>::with_weights(0, vec![], vec![]);
    assert!(!problem.is_weighted());
}

#[test]
fn test_is_maximal_independent_set_wrong_len() {
    assert!(!is_maximal_independent_set(3, &[(0, 1)], &[true, false]));
}

#[test]
fn test_graph_ref() {
    let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
    let graph = problem.graph();
    assert_eq!(graph.num_vertices(), 3);
    assert_eq!(graph.num_edges(), 2);
}

#[test]
fn test_edges() {
    let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
    let edges = problem.edges();
    assert_eq!(edges.len(), 2);
}

#[test]
fn test_has_edge() {
    let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
    assert!(problem.has_edge(0, 1));
    assert!(problem.has_edge(1, 0)); // Undirected
    assert!(problem.has_edge(1, 2));
    assert!(!problem.has_edge(0, 2));
}

#[test]
fn test_weights_ref() {
    let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
    assert_eq!(problem.weights_ref(), &vec![1, 1, 1]);
}

#[test]
fn test_jl_parity_evaluation() {
    let data: serde_json::Value =
        serde_json::from_str(include_str!("../../../../tests/data/jl/maximalis.json")).unwrap();
    for instance in data["instances"].as_array().unwrap() {
        let nv = instance["instance"]["num_vertices"].as_u64().unwrap() as usize;
        let edges = jl_parse_edges(&instance["instance"]);
        let problem = MaximalIS::<SimpleGraph, i32>::new(nv, edges);
        for eval in instance["evaluations"].as_array().unwrap() {
            let config = jl_parse_config(&eval["config"]);
            let result = problem.evaluate(&config);
            let jl_valid = eval["is_valid"].as_bool().unwrap();
            assert_eq!(
                result.is_valid(),
                jl_valid,
                "MaximalIS validity mismatch for config {:?}",
                config
            );
            if jl_valid {
                let jl_size = eval["size"].as_i64().unwrap() as i32;
                assert_eq!(
                    result.unwrap(),
                    jl_size,
                    "MaximalIS size mismatch for config {:?}",
                    config
                );
            }
        }
        let best = BruteForce::new().find_all_best(&problem);
        let jl_best = jl_parse_configs_set(&instance["best_solutions"]);
        let rust_best: HashSet<Vec<usize>> = best.into_iter().collect();
        assert_eq!(rust_best, jl_best, "MaximalIS best solutions mismatch");
    }
}
