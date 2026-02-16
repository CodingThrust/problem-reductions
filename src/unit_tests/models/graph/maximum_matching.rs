use super::*;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};
include!("../../jl_helpers.rs");

#[test]
fn test_matching_creation() {
    let problem = MaximumMatching::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![1, 2, 3],
    );
    assert_eq!(problem.graph().num_vertices(), 4);
    assert_eq!(problem.graph().num_edges(), 3);
    assert_eq!(problem.num_variables(), 3);
}

#[test]
fn test_matching_unit_weights() {
    let problem =
        MaximumMatching::<_, i32>::unit_weights(SimpleGraph::new(3, vec![(0, 1), (1, 2)]));
    assert_eq!(problem.graph().num_edges(), 2);
}

#[test]
fn test_edge_endpoints() {
    let problem = MaximumMatching::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1, 2]);
    assert_eq!(problem.edge_endpoints(0), Some((0, 1)));
    assert_eq!(problem.edge_endpoints(1), Some((1, 2)));
    assert_eq!(problem.edge_endpoints(2), None);
}

#[test]
fn test_is_valid_matching() {
    let problem = MaximumMatching::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![1, 1, 1],
    );

    // Valid: select edge 0 only
    assert!(problem.is_valid_matching(&[1, 0, 0]));

    // Valid: select edges 0 and 2 (disjoint)
    assert!(problem.is_valid_matching(&[1, 0, 1]));

    // Invalid: edges 0 and 1 share vertex 1
    assert!(!problem.is_valid_matching(&[1, 1, 0]));
}

#[test]
fn test_is_matching_function() {
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);

    assert!(is_matching(&graph, &[true, false, true])); // Disjoint
    assert!(is_matching(&graph, &[false, true, false])); // Single edge
    assert!(!is_matching(&graph, &[true, true, false])); // Share vertex 1
    assert!(is_matching(&graph, &[false, false, false])); // Empty is valid
}

#[test]
fn test_direction() {
    let problem = MaximumMatching::<_, i32>::unit_weights(SimpleGraph::new(2, vec![(0, 1)]));
    assert_eq!(problem.direction(), Direction::Maximize);
}

#[test]
fn test_empty_graph() {
    let problem = MaximumMatching::<_, i32>::unit_weights(SimpleGraph::new(3, vec![]));
    // Empty matching is valid with size 0
    assert_eq!(Problem::evaluate(&problem, &[]), SolutionSize::Valid(0));
}

#[test]
fn test_edges() {
    let problem = MaximumMatching::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![5, 10]);
    let edges = problem.edges();
    assert_eq!(edges.len(), 2);
}

#[test]
fn test_empty_sets() {
    let problem = MaximumMatching::<_, i32>::unit_weights(SimpleGraph::new(2, vec![]));
    // Empty matching
    assert_eq!(Problem::evaluate(&problem, &[]), SolutionSize::Valid(0));
}

#[test]
#[should_panic(expected = "selected length must match num_edges")]
fn test_is_matching_wrong_len() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    is_matching(&graph, &[true]); // Wrong length
}

#[test]
fn test_new() {
    let problem = MaximumMatching::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![5, 10]);
    assert_eq!(problem.graph().num_vertices(), 3);
    assert_eq!(problem.graph().num_edges(), 2);
    assert_eq!(problem.weights(), vec![5, 10]);
}

#[test]
fn test_unit_weights() {
    let problem =
        MaximumMatching::<_, i32>::unit_weights(SimpleGraph::new(3, vec![(0, 1), (1, 2)]));
    assert_eq!(problem.graph().num_vertices(), 3);
    assert_eq!(problem.graph().num_edges(), 2);
    assert_eq!(problem.weights(), vec![1, 1]);
}

#[test]
fn test_graph_accessor() {
    let problem =
        MaximumMatching::<_, i32>::unit_weights(SimpleGraph::new(3, vec![(0, 1), (1, 2)]));
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
        let edges: Vec<(usize, usize)> = weighted_edges.iter().map(|&(u, v, _)| (u, v)).collect();
        let weights: Vec<i32> = weighted_edges.into_iter().map(|(_, _, w)| w).collect();
        let problem = MaximumMatching::new(SimpleGraph::new(nv, edges), weights);
        for eval in instance["evaluations"].as_array().unwrap() {
            let config = jl_parse_config(&eval["config"]);
            let result = problem.evaluate(&config);
            let jl_valid = eval["is_valid"].as_bool().unwrap();
            assert_eq!(
                result.is_valid(),
                jl_valid,
                "Matching validity mismatch for config {:?}",
                config
            );
            if jl_valid {
                let jl_size = eval["size"].as_i64().unwrap() as i32;
                assert_eq!(
                    result.unwrap(),
                    jl_size,
                    "Matching size mismatch for config {:?}",
                    config
                );
            }
        }
        let best = BruteForce::new().find_all_best(&problem);
        let jl_best = jl_parse_configs_set(&instance["best_solutions"]);
        let rust_best: HashSet<Vec<usize>> = best.into_iter().collect();
        assert_eq!(rust_best, jl_best, "Matching best solutions mismatch");
    }
}
