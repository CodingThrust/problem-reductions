use super::*;
use crate::solvers::BruteForce;
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};
include!("../../jl_helpers.rs");

#[test]
fn test_matching_creation() {
    let problem =
        MaximumMatching::<SimpleGraph, i32>::new(4, vec![(0, 1, 1), (1, 2, 2), (2, 3, 3)]);
    assert_eq!(problem.num_vertices(), 4);
    assert_eq!(problem.num_edges(), 3);
    assert_eq!(problem.num_variables(), 3);
}

#[test]
fn test_matching_unweighted() {
    let problem = MaximumMatching::<SimpleGraph, i32>::unweighted(3, vec![(0, 1), (1, 2)]);
    assert_eq!(problem.num_edges(), 2);
}

#[test]
fn test_edge_endpoints() {
    let problem = MaximumMatching::<SimpleGraph, i32>::new(3, vec![(0, 1, 1), (1, 2, 2)]);
    assert_eq!(problem.edge_endpoints(0), Some((0, 1)));
    assert_eq!(problem.edge_endpoints(1), Some((1, 2)));
    assert_eq!(problem.edge_endpoints(2), None);
}

#[test]
fn test_is_valid_matching() {
    let problem =
        MaximumMatching::<SimpleGraph, i32>::new(4, vec![(0, 1, 1), (1, 2, 1), (2, 3, 1)]);

    // Valid: select edge 0 only
    assert!(problem.is_valid_matching(&[1, 0, 0]));

    // Valid: select edges 0 and 2 (disjoint)
    assert!(problem.is_valid_matching(&[1, 0, 1]));

    // Invalid: edges 0 and 1 share vertex 1
    assert!(!problem.is_valid_matching(&[1, 1, 0]));
}

#[test]
fn test_is_matching_function() {
    let edges = vec![(0, 1), (1, 2), (2, 3)];

    assert!(is_matching(4, &edges, &[true, false, true])); // Disjoint
    assert!(is_matching(4, &edges, &[false, true, false])); // Single edge
    assert!(!is_matching(4, &edges, &[true, true, false])); // Share vertex 1
    assert!(is_matching(4, &edges, &[false, false, false])); // Empty is valid
}

#[test]
fn test_direction() {
    let problem = MaximumMatching::<SimpleGraph, i32>::unweighted(2, vec![(0, 1)]);
    assert_eq!(problem.direction(), Direction::Maximize);
}

#[test]
fn test_empty_graph() {
    let problem = MaximumMatching::<SimpleGraph, i32>::unweighted(3, vec![]);
    // Empty matching is valid with size 0
    assert_eq!(Problem::evaluate(&problem, &[]), SolutionSize::Valid(0));
}

#[test]
fn test_edges() {
    let problem = MaximumMatching::<SimpleGraph, i32>::new(3, vec![(0, 1, 5), (1, 2, 10)]);
    let edges = problem.edges();
    assert_eq!(edges.len(), 2);
}

#[test]
fn test_empty_sets() {
    let problem = MaximumMatching::<SimpleGraph, i32>::unweighted(2, vec![]);
    // Empty matching
    assert_eq!(Problem::evaluate(&problem, &[]), SolutionSize::Valid(0));
}

#[test]
fn test_is_matching_wrong_len() {
    let edges = vec![(0, 1), (1, 2)];
    assert!(!is_matching(3, &edges, &[true])); // Wrong length
}

#[test]
fn test_is_matching_out_of_bounds() {
    let edges = vec![(0, 5)]; // Vertex 5 doesn't exist
    assert!(!is_matching(3, &edges, &[true]));
}

#[test]
fn test_from_graph() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MaximumMatching::<SimpleGraph, i32>::from_graph(graph, vec![5, 10]);
    assert_eq!(problem.num_vertices(), 3);
    assert_eq!(problem.num_edges(), 2);
    assert_eq!(problem.weights(), vec![5, 10]);
}

#[test]
fn test_from_graph_unit_weights() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MaximumMatching::<SimpleGraph, i32>::from_graph_unit_weights(graph);
    assert_eq!(problem.num_vertices(), 3);
    assert_eq!(problem.num_edges(), 2);
    assert_eq!(problem.weights(), vec![1, 1]);
}

#[test]
fn test_graph_accessor() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MaximumMatching::<SimpleGraph, i32>::from_graph_unit_weights(graph);
    assert_eq!(problem.graph().num_vertices(), 3);
    assert_eq!(problem.graph().num_edges(), 2);
}

#[test]
fn test_jl_parity_evaluation() {
    let data: serde_json::Value =
        serde_json::from_str(include_str!("../../../../tests/data/jl/matching.json")).unwrap();
    for instance in data["instances"].as_array().unwrap() {
        let nv = instance["instance"]["num_vertices"].as_u64().unwrap() as usize;
        let weighted_edges = jl_parse_weighted_edges(&instance["instance"]);
        let problem = MaximumMatching::<SimpleGraph, i32>::new(nv, weighted_edges);
        for eval in instance["evaluations"].as_array().unwrap() {
            let config = jl_parse_config(&eval["config"]);
            let result = problem.evaluate(&config);
            let jl_valid = eval["is_valid"].as_bool().unwrap();
            assert_eq!(result.is_valid(), jl_valid, "Matching validity mismatch for config {:?}", config);
            if jl_valid {
                let jl_size = eval["size"].as_i64().unwrap() as i32;
                assert_eq!(result.unwrap(), jl_size, "Matching size mismatch for config {:?}", config);
            }
        }
        let best = BruteForce::new().find_all_best(&problem);
        let jl_best = jl_parse_configs_set(&instance["best_solutions"]);
        let rust_best: HashSet<Vec<usize>> = best.into_iter().collect();
        assert_eq!(rust_best, jl_best, "Matching best solutions mismatch");
    }
}
