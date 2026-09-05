use super::*;
use crate::solvers::BruteForceProblem as _;

#[test]
fn create_specs_separate_runtime_and_fixed_color_counts() {
    let runtime = KColoring::<KN, SimpleGraph>::try_from(RuntimeKColoringCreateSpec {
        graph: vec![(0, 1)],
        num_vertices: Some(3),
        k: 4,
    })
    .unwrap();
    assert_eq!(runtime.num_vertices(), 3);
    assert_eq!(runtime.num_colors(), 4);

    let fixed = KColoring::<K3, SimpleGraph>::try_from(FixedKColoringCreateSpec {
        graph: vec![(0, 1)],
        num_vertices: None,
    })
    .unwrap();
    assert_eq!(fixed.num_colors(), 3);
}

#[test]
fn fixed_and_runtime_variants_report_num_colors_parameter() {
    let fixed = KColoring::<K3, _>::new(SimpleGraph::new(2, vec![(0, 1)]));
    let runtime = KColoring::<KN, _>::with_k(SimpleGraph::new(2, vec![(0, 1)]), 5);

    assert_eq!(Problem::parameters(&fixed).get("num_colors"), Some(3));
    assert_eq!(Problem::parameters(&runtime).get("num_colors"), Some(5));
    assert_eq!(
        <KColoring<K3, SimpleGraph> as Problem>::parameter_names(),
        <KColoring<KN, SimpleGraph> as Problem>::parameter_names()
    );
}
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::variant::{K1, K2, K3, K4};
include!("../../jl_helpers.rs");

#[test]
fn test_kcoloring_creation() {
    let problem = KColoring::<K3, _>::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]));
    assert_eq!(problem.graph().num_vertices(), 4);
    assert_eq!(problem.graph().num_edges(), 3);
    assert_eq!(problem.num_colors(), 3);
    assert_eq!(problem.dimensions(), vec![3, 3, 3, 3]);
}

#[test]
fn test_evaluate_valid() {
    use crate::traits::Problem;

    let problem = KColoring::<K3, _>::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]));

    // Valid: different colors on adjacent vertices
    assert!(problem.evaluate(&vec![0, 1, 0]).unwrap());
    assert!(problem.evaluate(&vec![0, 1, 2]).unwrap());
}

#[test]
fn test_evaluate_invalid() {
    use crate::traits::Problem;

    let problem = KColoring::<K3, _>::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]));

    // Invalid: adjacent vertices have same color
    assert!(!problem.evaluate(&vec![0, 0, 1]).unwrap());
    assert!(!problem.evaluate(&vec![0, 0, 0]).unwrap());
}

#[test]
fn test_brute_force_path() {
    use crate::traits::Problem;

    // Path graph can be 2-colored
    let problem = KColoring::<K2, _>::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]));
    let solver = BruteForce::new();

    let solutions = solver.find_all_witnesses(&problem).unwrap();
    // All solutions should be valid
    for sol in &solutions {
        assert!(problem.evaluate(sol).unwrap());
    }
}

#[test]
fn test_brute_force_triangle() {
    use crate::traits::Problem;

    // Triangle needs 3 colors
    let problem = KColoring::<K3, _>::new(SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]));
    let solver = BruteForce::new();

    let solutions = solver.find_all_witnesses(&problem).unwrap();
    for sol in &solutions {
        assert!(problem.evaluate(sol).unwrap());
        // All three vertices have different colors
        assert_ne!(sol[0], sol[1]);
        assert_ne!(sol[1], sol[2]);
        assert_ne!(sol[0], sol[2]);
    }
}

#[test]
fn test_triangle_2_colors() {
    // Triangle cannot be 2-colored
    let problem = KColoring::<K2, _>::new(SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]));
    let solver = BruteForce::new();

    let solutions = solver.find_all_witnesses(&problem).unwrap();
    // No valid solutions
    assert!(solutions.is_empty());
}

#[test]
fn test_is_valid_coloring_function() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);

    assert!(is_valid_coloring(&graph, &[0, 1, 0], 2));
    assert!(is_valid_coloring(&graph, &[0, 1, 2], 3));
    assert!(!is_valid_coloring(&graph, &[0, 0, 1], 2)); // 0-1 conflict
    assert!(!is_valid_coloring(&graph, &[0, 1, 1], 2)); // 1-2 conflict
    assert!(!is_valid_coloring(&graph, &[0, 2, 0], 2)); // Color out of range
}

#[test]
#[should_panic(expected = "coloring length must match num_vertices")]
fn test_is_valid_coloring_wrong_len() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    is_valid_coloring(&graph, &[0, 1], 2); // Wrong length
}

#[test]
fn test_empty_graph() {
    use crate::traits::Problem;

    let problem = KColoring::<K1, _>::new(SimpleGraph::new(3, vec![]));
    let solver = BruteForce::new();

    let solutions = solver.find_all_witnesses(&problem).unwrap();
    // Any coloring is valid when there are no edges
    assert!(!solutions.is_empty());
    for sol in &solutions {
        assert!(problem.evaluate(sol).unwrap());
    }
}

#[test]
fn test_complete_graph_k4() {
    use crate::traits::Problem;

    // K4 needs 4 colors
    let problem = KColoring::<K4, _>::new(SimpleGraph::new(
        4,
        vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)],
    ));
    let solver = BruteForce::new();

    let solutions = solver.find_all_witnesses(&problem).unwrap();
    for sol in &solutions {
        assert!(problem.evaluate(sol).unwrap());
    }
}

#[test]
fn test_new_from_graph() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = KColoring::<K3, _>::new(graph);
    assert_eq!(problem.graph().num_vertices(), 3);
    assert_eq!(problem.graph().num_edges(), 2);
}

#[test]
fn test_kcoloring_problem() {
    use crate::traits::Problem;

    // Triangle graph with 3 colors
    let p = KColoring::<K3, _>::new(SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]));
    assert_eq!(p.dimensions(), vec![3, 3, 3]);
    // Valid: each vertex different color
    assert!(p.evaluate(&vec![0, 1, 2]).unwrap());
    // Invalid: vertices 0 and 1 same color
    assert!(!p.evaluate(&vec![0, 0, 1]).unwrap());
}

#[test]
fn test_jl_parity_evaluation() {
    let data: serde_json::Value =
        serde_json::from_str(include_str!("../../../../tests/data/jl/coloring.json")).unwrap();
    for instance in data["instances"].as_array().unwrap() {
        let nv = instance["instance"]["num_vertices"].as_u64().unwrap() as usize;
        let edges = jl_parse_edges(&instance["instance"]);
        let num_edges = edges.len();
        let problem = KColoring::<K3, _>::new(SimpleGraph::new(nv, edges));
        for eval in instance["evaluations"].as_array().unwrap() {
            let config = jl_parse_config(&eval["config"]);
            let result = problem.evaluate(&config).unwrap().0;
            let jl_size = eval["size"].as_i64().unwrap() as usize;
            assert_eq!(
                result,
                jl_size == num_edges,
                "KColoring mismatch for config {:?}",
                config
            );
        }
        let all_sat = BruteForce::new().find_all_witnesses(&problem).unwrap();
        let jl_best = jl_parse_configs_set(&instance["best_solutions"]);
        let rust_sat: HashSet<Vec<usize>> = all_sat.into_iter().collect();
        assert_eq!(rust_sat, jl_best, "KColoring satisfying solutions mismatch");
    }
}

#[test]
fn test_is_valid_solution() {
    // Path graph: 0-1-2, 3-coloring
    let problem = KColoring::<K3, _>::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]));
    // Valid: neighbors have different colors
    assert!(problem.is_valid_solution(&[0, 1, 0]));
    // Invalid: adjacent vertices 0 and 1 have same color
    assert!(!problem.is_valid_solution(&[0, 0, 1]));
}

#[test]
fn test_parameter_getters() {
    let problem = KColoring::<K3, _>::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]));
    assert_eq!(problem.num_vertices(), 3);
    assert_eq!(problem.num_edges(), 2);
}

#[test]
fn test_kcoloring_paper_example() {
    use crate::traits::Problem;
    // Paper: house graph, k=3, proper coloring [0,1,1,0,2], chi(G)=3
    let graph = SimpleGraph::new(5, vec![(0, 1), (0, 2), (1, 3), (2, 3), (2, 4), (3, 4)]);
    let problem = KColoring::<K3, _>::new(graph);
    let config = vec![0, 1, 1, 0, 2];
    assert!(problem.evaluate(&config).unwrap());

    // Verify not 2-colorable (triangle v_2,v_3,v_4)
    let graph2 = SimpleGraph::new(5, vec![(0, 1), (0, 2), (1, 3), (2, 3), (2, 4), (3, 4)]);
    let problem2 = KColoring::<K2, _>::new(graph2);
    let solver = BruteForce::new();
    assert!(solver.solve(&problem2).unwrap().is_none());
}
