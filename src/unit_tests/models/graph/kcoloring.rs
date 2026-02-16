use super::*;
use crate::solvers::BruteForce;
use crate::variant::{K1, K2, K3, K4};
include!("../../jl_helpers.rs");

#[test]
fn test_kcoloring_creation() {
    use crate::traits::Problem;

    let problem = KColoring::<K3, SimpleGraph>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    assert_eq!(problem.graph().num_vertices(), 4);
    assert_eq!(problem.graph().num_edges(), 3);
    assert_eq!(problem.num_colors(), 3);
    assert_eq!(problem.dims(), vec![3, 3, 3, 3]);
}

#[test]
fn test_evaluate_valid() {
    use crate::traits::Problem;

    let problem = KColoring::<K3, SimpleGraph>::new(3, vec![(0, 1), (1, 2)]);

    // Valid: different colors on adjacent vertices
    assert!(problem.evaluate(&[0, 1, 0]));
    assert!(problem.evaluate(&[0, 1, 2]));
}

#[test]
fn test_evaluate_invalid() {
    use crate::traits::Problem;

    let problem = KColoring::<K3, SimpleGraph>::new(3, vec![(0, 1), (1, 2)]);

    // Invalid: adjacent vertices have same color
    assert!(!problem.evaluate(&[0, 0, 1]));
    assert!(!problem.evaluate(&[0, 0, 0]));
}

#[test]
fn test_brute_force_path() {
    use crate::traits::Problem;

    // Path graph can be 2-colored
    let problem = KColoring::<K2, SimpleGraph>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let solver = BruteForce::new();

    let solutions = solver.find_all_satisfying(&problem);
    // All solutions should be valid
    for sol in &solutions {
        assert!(problem.evaluate(sol));
    }
}

#[test]
fn test_brute_force_triangle() {
    use crate::traits::Problem;

    // Triangle needs 3 colors
    let problem = KColoring::<K3, SimpleGraph>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let solver = BruteForce::new();

    let solutions = solver.find_all_satisfying(&problem);
    for sol in &solutions {
        assert!(problem.evaluate(sol));
        // All three vertices have different colors
        assert_ne!(sol[0], sol[1]);
        assert_ne!(sol[1], sol[2]);
        assert_ne!(sol[0], sol[2]);
    }
}

#[test]
fn test_triangle_2_colors() {
    // Triangle cannot be 2-colored
    let problem = KColoring::<K2, SimpleGraph>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let solver = BruteForce::new();

    let solutions = solver.find_all_satisfying(&problem);
    // No valid solutions
    assert!(solutions.is_empty());
}

#[test]
fn test_is_valid_coloring_function() {
    let edges = vec![(0, 1), (1, 2)];

    assert!(is_valid_coloring(3, &edges, &[0, 1, 0], 2));
    assert!(is_valid_coloring(3, &edges, &[0, 1, 2], 3));
    assert!(!is_valid_coloring(3, &edges, &[0, 0, 1], 2)); // 0-1 conflict
    assert!(!is_valid_coloring(3, &edges, &[0, 1, 1], 2)); // 1-2 conflict
    assert!(!is_valid_coloring(3, &edges, &[0, 1], 2)); // Wrong length
    assert!(!is_valid_coloring(3, &edges, &[0, 2, 0], 2)); // Color out of range
}

#[test]
fn test_empty_graph() {
    use crate::traits::Problem;

    let problem = KColoring::<K1, SimpleGraph>::new(3, vec![]);
    let solver = BruteForce::new();

    let solutions = solver.find_all_satisfying(&problem);
    // Any coloring is valid when there are no edges
    assert!(!solutions.is_empty());
    for sol in &solutions {
        assert!(problem.evaluate(sol));
    }
}

#[test]
fn test_complete_graph_k4() {
    use crate::traits::Problem;

    // K4 needs 4 colors
    let problem =
        KColoring::<K4, SimpleGraph>::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
    let solver = BruteForce::new();

    let solutions = solver.find_all_satisfying(&problem);
    for sol in &solutions {
        assert!(problem.evaluate(sol));
    }
}

#[test]
fn test_from_graph() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = KColoring::<K3, SimpleGraph>::from_graph(graph);
    assert_eq!(problem.graph().num_vertices(), 3);
    assert_eq!(problem.graph().num_edges(), 2);
}

#[test]
fn test_kcoloring_problem() {
    use crate::traits::Problem;

    // Triangle graph with 3 colors
    let p = KColoring::<K3, SimpleGraph>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    assert_eq!(p.dims(), vec![3, 3, 3]);
    // Valid: each vertex different color
    assert!(p.evaluate(&[0, 1, 2]));
    // Invalid: vertices 0 and 1 same color
    assert!(!p.evaluate(&[0, 0, 1]));
}

#[test]
fn test_jl_parity_evaluation() {
    let data: serde_json::Value =
        serde_json::from_str(include_str!("../../../../tests/data/jl/coloring.json")).unwrap();
    for instance in data["instances"].as_array().unwrap() {
        let nv = instance["instance"]["num_vertices"].as_u64().unwrap() as usize;
        let edges = jl_parse_edges(&instance["instance"]);
        let num_edges = edges.len();
        let problem = KColoring::<K3, SimpleGraph>::new(nv, edges);
        for eval in instance["evaluations"].as_array().unwrap() {
            let config = jl_parse_config(&eval["config"]);
            let result: bool = problem.evaluate(&config);
            let jl_size = eval["size"].as_i64().unwrap() as usize;
            assert_eq!(
                result,
                jl_size == num_edges,
                "KColoring mismatch for config {:?}",
                config
            );
        }
        let all_sat = BruteForce::new().find_all_satisfying(&problem);
        let jl_best = jl_parse_configs_set(&instance["best_solutions"]);
        let rust_sat: HashSet<Vec<usize>> = all_sat.into_iter().collect();
        assert_eq!(rust_sat, jl_best, "KColoring satisfying solutions mismatch");
    }
}
