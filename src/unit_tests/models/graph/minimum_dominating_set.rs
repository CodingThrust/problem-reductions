use super::*;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::{OptimizationProblem, Problem};
use crate::types::Direction;
include!("../../jl_helpers.rs");

#[test]
fn test_dominating_set_creation() {
    let problem = MinimumDominatingSet::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![1i32; 4],
    );
    assert_eq!(problem.graph().num_vertices(), 4);
    assert_eq!(problem.graph().num_edges(), 3);
}

#[test]
fn test_dominating_set_with_weights() {
    let problem = MinimumDominatingSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1, 2, 3]);
    assert_eq!(problem.weights(), &[1, 2, 3]);
}

#[test]
fn test_neighbors() {
    let problem = MinimumDominatingSet::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 2)]),
        vec![1i32; 4],
    );
    let nbrs = problem.neighbors(0);
    assert!(nbrs.contains(&1));
    assert!(nbrs.contains(&2));
    assert!(!nbrs.contains(&3));
}

#[test]
fn test_closed_neighborhood() {
    let problem =
        MinimumDominatingSet::new(SimpleGraph::new(4, vec![(0, 1), (0, 2)]), vec![1i32; 4]);
    let cn = problem.closed_neighborhood(0);
    assert!(cn.contains(&0));
    assert!(cn.contains(&1));
    assert!(cn.contains(&2));
    assert!(!cn.contains(&3));
}

#[test]
fn test_is_dominating_set_function() {
    let graph = SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3)]);

    // Center dominates all
    assert!(is_dominating_set(&graph, &[true, false, false, false]));
    // All leaves dominate (leaf dominates center which dominates others)
    assert!(is_dominating_set(&graph, &[false, true, true, true]));
    // Single leaf doesn't dominate other leaves
    assert!(!is_dominating_set(&graph, &[false, true, false, false]));
    // Empty doesn't dominate
    assert!(!is_dominating_set(&graph, &[false, false, false, false]));
}

#[test]
fn test_direction() {
    let problem = MinimumDominatingSet::new(SimpleGraph::new(2, vec![(0, 1)]), vec![1i32; 2]);
    assert_eq!(problem.direction(), Direction::Minimize);
}

#[test]
fn test_isolated_vertex() {
    // Isolated vertex must be in dominating set
    let problem = MinimumDominatingSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1i32; 3]);
    let solver = BruteForce::new();

    let solutions = solver.find_all_best(&problem);
    // Vertex 2 is isolated, must be selected
    for sol in &solutions {
        assert_eq!(sol[2], 1);
        // Verify it's a valid dominating set
        assert!(Problem::evaluate(&problem, sol).is_valid());
    }
}

#[test]
#[should_panic(expected = "selected length must match num_vertices")]
fn test_is_dominating_set_wrong_len() {
    is_dominating_set(&SimpleGraph::new(3, vec![(0, 1)]), &[true, false]);
}

#[test]
fn test_from_graph() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumDominatingSet::new(graph, vec![1, 2, 3]);
    assert_eq!(problem.graph().num_vertices(), 3);
    assert_eq!(problem.weights(), &[1, 2, 3]);
}

#[test]
fn test_graph_accessor() {
    let problem = MinimumDominatingSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1i32; 3]);
    assert_eq!(problem.graph().num_vertices(), 3);
    assert_eq!(problem.graph().num_edges(), 1);
}

#[test]
fn test_weights() {
    let problem = MinimumDominatingSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![5, 10, 15]);
    assert_eq!(problem.weights(), &[5, 10, 15]);
}

#[test]
fn test_edges() {
    let problem =
        MinimumDominatingSet::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1i32; 3]);
    let edges = problem.graph().edges();
    assert_eq!(edges.len(), 2);
}

#[test]
fn test_has_edge() {
    let problem =
        MinimumDominatingSet::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1i32; 3]);
    assert!(problem.graph().has_edge(0, 1));
    assert!(problem.graph().has_edge(1, 0)); // Undirected
    assert!(problem.graph().has_edge(1, 2));
    assert!(!problem.graph().has_edge(0, 2));
}

#[test]
fn test_jl_parity_evaluation() {
    let data: serde_json::Value =
        serde_json::from_str(include_str!("../../../../tests/data/jl/dominatingset.json")).unwrap();
    for instance in data["instances"].as_array().unwrap() {
        let nv = instance["instance"]["num_vertices"].as_u64().unwrap() as usize;
        let edges = jl_parse_edges(&instance["instance"]);
        let problem = MinimumDominatingSet::new(SimpleGraph::new(nv, edges), vec![1i32; nv]);
        for eval in instance["evaluations"].as_array().unwrap() {
            let config = jl_parse_config(&eval["config"]);
            let result = problem.evaluate(&config);
            let jl_valid = eval["is_valid"].as_bool().unwrap();
            assert_eq!(
                result.is_valid(),
                jl_valid,
                "DS validity mismatch for config {:?}",
                config
            );
            if jl_valid {
                let jl_size = eval["size"].as_i64().unwrap() as i32;
                assert_eq!(
                    result.unwrap(),
                    jl_size,
                    "DS size mismatch for config {:?}",
                    config
                );
            }
        }
        let best = BruteForce::new().find_all_best(&problem);
        let jl_best = jl_parse_configs_set(&instance["best_solutions"]);
        let rust_best: HashSet<Vec<usize>> = best.into_iter().collect();
        assert_eq!(rust_best, jl_best, "DS best solutions mismatch");
    }
}

#[test]
fn test_is_valid_solution() {
    // Path graph: 0-1-2
    let problem = MinimumDominatingSet::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1i32; 3]);
    // Valid: {1} dominates all vertices (0 and 2 are neighbors of 1)
    assert!(problem.is_valid_solution(&[0, 1, 0]));
    // Invalid: {0} doesn't dominate vertex 2
    assert!(!problem.is_valid_solution(&[1, 0, 0]));
}
