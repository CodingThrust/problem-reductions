use super::*;
use crate::solvers::BruteForce;
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};
include!("../../jl_helpers.rs");

#[test]
fn test_vertex_cover_creation() {
    let problem = MinimumVertexCover::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    assert_eq!(problem.num_vertices(), 4);
    assert_eq!(problem.num_edges(), 3);
    assert_eq!(problem.num_variables(), 4);
}

#[test]
fn test_vertex_cover_with_weights() {
    let problem =
        MinimumVertexCover::<SimpleGraph, i32>::with_weights(3, vec![(0, 1)], vec![1, 2, 3]);
    assert_eq!(problem.weights(), vec![1, 2, 3]);
}

#[test]
fn test_evaluate_valid() {
    let problem = MinimumVertexCover::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);

    // Valid: select vertex 1 (covers both edges)
    assert_eq!(
        Problem::evaluate(&problem, &[0, 1, 0]),
        SolutionSize::Valid(1)
    );

    // Valid: select all vertices
    assert_eq!(
        Problem::evaluate(&problem, &[1, 1, 1]),
        SolutionSize::Valid(3)
    );
}

#[test]
fn test_evaluate_invalid() {
    let problem = MinimumVertexCover::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);

    // Invalid: no vertex selected - returns Invalid for minimization
    assert_eq!(
        Problem::evaluate(&problem, &[0, 0, 0]),
        SolutionSize::Invalid
    );

    // Invalid: only vertex 0 selected (edge 1-2 not covered)
    assert_eq!(
        Problem::evaluate(&problem, &[1, 0, 0]),
        SolutionSize::Invalid
    );
}

#[test]
fn test_brute_force_path() {
    // Path graph 0-1-2: minimum vertex cover is {1}
    let problem = MinimumVertexCover::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
    let solver = BruteForce::new();

    let solutions = solver.find_all_best(&problem);
    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions[0], vec![0, 1, 0]);
}

#[test]
fn test_brute_force_triangle() {
    // Triangle: minimum vertex cover has size 2
    let problem = MinimumVertexCover::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let solver = BruteForce::new();

    let solutions = solver.find_all_best(&problem);
    // There are 3 minimum covers of size 2
    assert_eq!(solutions.len(), 3);
    for sol in &solutions {
        assert_eq!(sol.iter().sum::<usize>(), 2);
        // Verify it's a valid cover by checking evaluate returns Valid
        assert!(Problem::evaluate(&problem, sol).is_valid());
    }
}

#[test]
fn test_brute_force_weighted() {
    // Weighted: prefer selecting low-weight vertices
    let problem = MinimumVertexCover::<SimpleGraph, i32>::with_weights(
        3,
        vec![(0, 1), (1, 2)],
        vec![100, 1, 100],
    );
    let solver = BruteForce::new();

    let solutions = solver.find_all_best(&problem);
    assert_eq!(solutions.len(), 1);
    // Should select vertex 1 (weight 1) instead of 0 and 2 (total 200)
    assert_eq!(solutions[0], vec![0, 1, 0]);
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
fn test_empty_graph() {
    let problem = MinimumVertexCover::<SimpleGraph, i32>::new(3, vec![]);
    let solver = BruteForce::new();

    let solutions = solver.find_all_best(&problem);
    // No edges means empty cover is valid and optimal
    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions[0], vec![0, 0, 0]);
}

#[test]
fn test_single_edge() {
    let problem = MinimumVertexCover::<SimpleGraph, i32>::new(2, vec![(0, 1)]);
    let solver = BruteForce::new();

    let solutions = solver.find_all_best(&problem);
    // Either vertex covers the single edge
    assert_eq!(solutions.len(), 2);
}

#[test]
fn test_complement_relationship() {
    // For a graph, if S is an independent set, then V\S is a vertex cover
    use crate::models::graph::MaximumIndependentSet;

    let edges = vec![(0, 1), (1, 2), (2, 3)];
    let is_problem = MaximumIndependentSet::<SimpleGraph, i32>::new(4, edges.clone());
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
    let problem = MinimumVertexCover::<SimpleGraph, i32>::from_graph_unit_weights(graph);
    assert_eq!(problem.num_vertices(), 3);
    assert_eq!(problem.num_edges(), 2);
}

#[test]
fn test_from_graph_with_weights() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumVertexCover::<SimpleGraph, i32>::from_graph(graph, vec![1, 2, 3]);
    assert_eq!(problem.weights(), vec![1, 2, 3]);
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
    assert!(problem.has_edge(0, 1));
    assert!(problem.has_edge(1, 0)); // Undirected
    assert!(problem.has_edge(1, 2));
    assert!(!problem.has_edge(0, 2));
}

#[test]
fn test_mvc_problem_v2() {
    let p = MinimumVertexCover::<SimpleGraph, i32>::with_weights(
        3,
        vec![(0, 1), (1, 2), (0, 2)],
        vec![1, 1, 1],
    );
    assert_eq!(p.dims(), vec![2, 2, 2]);
    // Valid VC: select all vertices
    assert_eq!(Problem::evaluate(&p, &[1, 1, 1]), SolutionSize::Valid(3));
    // Valid VC: select vertices 0 and 1 (covers all edges in triangle)
    assert_eq!(Problem::evaluate(&p, &[1, 1, 0]), SolutionSize::Valid(2));
    // Invalid VC: select only vertex 0 (edge (1,2) not covered)
    assert_eq!(Problem::evaluate(&p, &[1, 0, 0]), SolutionSize::Invalid);
    assert_eq!(p.direction(), Direction::Minimize);
}

#[test]
fn test_jl_parity_evaluation() {
    let data: serde_json::Value =
        serde_json::from_str(include_str!("../../../../tests/data/jl/vertexcovering.json")).unwrap();
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
            assert_eq!(result.is_valid(), jl_valid, "VC validity mismatch for config {:?}", config);
            if jl_valid {
                let jl_size = eval["size"].as_i64().unwrap() as i32;
                assert_eq!(result.unwrap(), jl_size, "VC size mismatch for config {:?}", config);
            }
        }
        let best = BruteForce::new().find_all_best(&problem);
        let jl_best = jl_parse_configs_set(&instance["best_solutions"]);
        let rust_best: HashSet<Vec<usize>> = best.into_iter().collect();
        assert_eq!(rust_best, jl_best, "VC best solutions mismatch");
    }
}
