use super::*;
use crate::solvers::BruteForce;
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};
include!("../../jl_helpers.rs");

#[test]
fn test_dominating_set_creation() {
    let problem = MinimumDominatingSet::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    assert_eq!(problem.num_vertices(), 4);
    assert_eq!(problem.num_edges(), 3);
}

#[test]
fn test_dominating_set_with_weights() {
    let problem =
        MinimumDominatingSet::<SimpleGraph, i32>::with_weights(3, vec![(0, 1)], vec![1, 2, 3]);
    assert_eq!(problem.weights(), vec![1, 2, 3]);
}

#[test]
fn test_neighbors() {
    let problem = MinimumDominatingSet::<SimpleGraph, i32>::new(4, vec![(0, 1), (0, 2), (1, 2)]);
    let nbrs = problem.neighbors(0);
    assert!(nbrs.contains(&1));
    assert!(nbrs.contains(&2));
    assert!(!nbrs.contains(&3));
}

#[test]
fn test_closed_neighborhood() {
    let problem = MinimumDominatingSet::<SimpleGraph, i32>::new(4, vec![(0, 1), (0, 2)]);
    let cn = problem.closed_neighborhood(0);
    assert!(cn.contains(&0));
    assert!(cn.contains(&1));
    assert!(cn.contains(&2));
    assert!(!cn.contains(&3));
}

#[test]
fn test_evaluate_valid() {
    // Star graph: center dominates all
    let problem = MinimumDominatingSet::<SimpleGraph, i32>::new(4, vec![(0, 1), (0, 2), (0, 3)]);

    // Select center
    assert_eq!(
        Problem::evaluate(&problem, &[1, 0, 0, 0]),
        SolutionSize::Valid(1)
    );

    // Select all leaves
    assert_eq!(
        Problem::evaluate(&problem, &[0, 1, 1, 1]),
        SolutionSize::Valid(3)
    );
}

#[test]
fn test_evaluate_invalid() {
    let problem = MinimumDominatingSet::<SimpleGraph, i32>::new(4, vec![(0, 1), (2, 3)]);

    // Select none - returns Invalid for minimization
    assert_eq!(
        Problem::evaluate(&problem, &[0, 0, 0, 0]),
        SolutionSize::Invalid
    );

    // Select only vertex 0 (doesn't dominate 2, 3)
    assert_eq!(
        Problem::evaluate(&problem, &[1, 0, 0, 0]),
        SolutionSize::Invalid
    );
}

#[test]
fn test_brute_force_star() {
    // Star graph: minimum dominating set is the center
    let problem = MinimumDominatingSet::<SimpleGraph, i32>::new(4, vec![(0, 1), (0, 2), (0, 3)]);
    let solver = BruteForce::new();

    let solutions = solver.find_all_best(&problem);
    assert!(solutions.contains(&vec![1, 0, 0, 0]));
    for sol in &solutions {
        assert_eq!(Problem::evaluate(&problem, sol), SolutionSize::Valid(1));
    }
}

#[test]
fn test_brute_force_path() {
    // Path 0-1-2-3-4: need to dominate all 5 vertices
    let problem =
        MinimumDominatingSet::<SimpleGraph, i32>::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]);
    let solver = BruteForce::new();

    let solutions = solver.find_all_best(&problem);
    // Minimum is 2 (e.g., vertices 1 and 3)
    for sol in &solutions {
        assert_eq!(Problem::evaluate(&problem, sol), SolutionSize::Valid(2));
        // Verify it's a valid dominating set
        assert!(Problem::evaluate(&problem, sol).is_valid());
    }
}

#[test]
fn test_brute_force_weighted() {
    // Star with heavy center
    let problem = MinimumDominatingSet::<SimpleGraph, i32>::with_weights(
        4,
        vec![(0, 1), (0, 2), (0, 3)],
        vec![100, 1, 1, 1],
    );
    let solver = BruteForce::new();

    let solutions = solver.find_all_best(&problem);
    // Prefer selecting all leaves (3) over center (100)
    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions[0], vec![0, 1, 1, 1]);
}

#[test]
fn test_is_dominating_set_function() {
    let edges = vec![(0, 1), (0, 2), (0, 3)];

    // Center dominates all
    assert!(is_dominating_set(4, &edges, &[true, false, false, false]));
    // All leaves dominate (leaf dominates center which dominates others)
    assert!(is_dominating_set(4, &edges, &[false, true, true, true]));
    // Single leaf doesn't dominate other leaves
    assert!(!is_dominating_set(4, &edges, &[false, true, false, false]));
    // Empty doesn't dominate
    assert!(!is_dominating_set(4, &edges, &[false, false, false, false]));
}

#[test]
fn test_direction() {
    let problem = MinimumDominatingSet::<SimpleGraph, i32>::new(2, vec![(0, 1)]);
    assert_eq!(problem.direction(), Direction::Minimize);
}

#[test]
fn test_isolated_vertex() {
    // Isolated vertex must be in dominating set
    let problem = MinimumDominatingSet::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
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
fn test_is_dominating_set_wrong_len() {
    assert!(!is_dominating_set(3, &[(0, 1)], &[true, false]));
}

#[test]
fn test_from_graph() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem =
        MinimumDominatingSet::<SimpleGraph, i32>::from_graph(graph.clone(), vec![1, 2, 3]);
    assert_eq!(problem.num_vertices(), 3);
    assert_eq!(problem.weights(), vec![1, 2, 3]);

    let problem2 = MinimumDominatingSet::<SimpleGraph, i32>::from_graph_unit_weights(graph);
    assert_eq!(problem2.num_vertices(), 3);
    assert_eq!(problem2.weights(), vec![1, 1, 1]);
}

#[test]
fn test_graph_accessor() {
    let problem = MinimumDominatingSet::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
    let graph = problem.graph();
    assert_eq!(graph.num_vertices(), 3);
    assert_eq!(graph.num_edges(), 1);
}

#[test]
fn test_weights_ref() {
    let problem =
        MinimumDominatingSet::<SimpleGraph, i32>::with_weights(3, vec![(0, 1)], vec![5, 10, 15]);
    assert_eq!(problem.weights_ref(), &vec![5, 10, 15]);
}

#[test]
fn test_edges() {
    let problem = MinimumDominatingSet::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
    let edges = problem.edges();
    assert_eq!(edges.len(), 2);
}

#[test]
fn test_has_edge() {
    let problem = MinimumDominatingSet::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
    assert!(problem.has_edge(0, 1));
    assert!(problem.has_edge(1, 0)); // Undirected
    assert!(problem.has_edge(1, 2));
    assert!(!problem.has_edge(0, 2));
}

#[test]
fn test_mds_problem_v2() {
    // Path graph 0-1-2
    let p = MinimumDominatingSet::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
    assert_eq!(p.dims(), vec![2, 2, 2]);
    // Valid DS: select vertex 1 (dominates all)
    assert_eq!(Problem::evaluate(&p, &[0, 1, 0]), SolutionSize::Valid(1));
    // Invalid DS: select no vertices
    assert_eq!(Problem::evaluate(&p, &[0, 0, 0]), SolutionSize::Invalid);
    assert_eq!(p.direction(), Direction::Minimize);
}

#[test]
fn test_jl_parity_evaluation() {
    let data: serde_json::Value =
        serde_json::from_str(include_str!("../../../../tests/data/jl/dominatingset.json")).unwrap();
    for instance in data["instances"].as_array().unwrap() {
        let nv = instance["instance"]["num_vertices"].as_u64().unwrap() as usize;
        let edges = jl_parse_edges(&instance["instance"]);
        let problem = MinimumDominatingSet::<SimpleGraph, i32>::new(nv, edges);
        for eval in instance["evaluations"].as_array().unwrap() {
            let config = jl_parse_config(&eval["config"]);
            let result = problem.evaluate(&config);
            let jl_valid = eval["is_valid"].as_bool().unwrap();
            assert_eq!(result.is_valid(), jl_valid, "DS validity mismatch for config {:?}", config);
            if jl_valid {
                let jl_size = eval["size"].as_i64().unwrap() as i32;
                assert_eq!(result.unwrap(), jl_size, "DS size mismatch for config {:?}", config);
            }
        }
        let best = BruteForce::new().find_all_best(&problem);
        let jl_best = jl_parse_configs_set(&instance["best_solutions"]);
        let rust_best: HashSet<Vec<usize>> = best.into_iter().collect();
        assert_eq!(rust_best, jl_best, "DS best solutions mismatch");
    }
}
