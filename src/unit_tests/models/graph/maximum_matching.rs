use super::*;
use crate::solvers::{BruteForce, Solver};

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
fn test_solution_size() {
    let problem =
        MaximumMatching::<SimpleGraph, i32>::new(4, vec![(0, 1, 5), (1, 2, 10), (2, 3, 3)]);

    let sol = problem.solution_size(&[1, 0, 1]);
    assert!(sol.is_valid);
    assert_eq!(sol.size, 8); // 5 + 3

    let sol = problem.solution_size(&[0, 1, 0]);
    assert!(sol.is_valid);
    assert_eq!(sol.size, 10);
}

#[test]
fn test_brute_force_path() {
    // Path 0-1-2-3 with unit weights
    let problem = MaximumMatching::<SimpleGraph, i32>::unweighted(4, vec![(0, 1), (1, 2), (2, 3)]);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    // Maximum matching has 2 edges: {0-1, 2-3}
    assert!(solutions.contains(&vec![1, 0, 1]));
    for sol in &solutions {
        assert_eq!(problem.solution_size(sol).size, 2);
    }
}

#[test]
fn test_brute_force_triangle() {
    let problem = MaximumMatching::<SimpleGraph, i32>::unweighted(3, vec![(0, 1), (1, 2), (0, 2)]);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    // Maximum matching has 1 edge (any of the 3)
    for sol in &solutions {
        assert_eq!(sol.iter().sum::<usize>(), 1);
        assert!(problem.solution_size(sol).is_valid);
    }
}

#[test]
fn test_brute_force_weighted() {
    // Prefer heavy edge even if it excludes more edges
    let problem =
        MaximumMatching::<SimpleGraph, i32>::new(4, vec![(0, 1, 100), (0, 2, 1), (1, 3, 1)]);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    // Edge 0-1 (weight 100) alone beats edges 0-2 + 1-3 (weight 2)
    assert!(solutions.contains(&vec![1, 0, 0]));
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
fn test_energy_mode() {
    let problem = MaximumMatching::<SimpleGraph, i32>::unweighted(2, vec![(0, 1)]);
    assert!(problem.energy_mode().is_maximization());
}

#[test]
fn test_empty_graph() {
    let problem = MaximumMatching::<SimpleGraph, i32>::unweighted(3, vec![]);
    let sol = problem.solution_size(&[]);
    assert!(sol.is_valid);
    assert_eq!(sol.size, 0);
}

#[test]
fn test_constraints() {
    let problem = MaximumMatching::<SimpleGraph, i32>::unweighted(3, vec![(0, 1), (1, 2)]);
    let constraints = problem.constraints();
    // Vertex 1 has degree 2, so 1 constraint
    assert_eq!(constraints.len(), 1);
}

#[test]
fn test_edges() {
    let problem = MaximumMatching::<SimpleGraph, i32>::new(3, vec![(0, 1, 5), (1, 2, 10)]);
    let edges = problem.edges();
    assert_eq!(edges.len(), 2);
}

#[test]
fn test_perfect_matching() {
    // K4: can have perfect matching (2 edges covering all 4 vertices)
    let problem = MaximumMatching::<SimpleGraph, i32>::unweighted(
        4,
        vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)],
    );
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    // Perfect matching has 2 edges
    for sol in &solutions {
        assert_eq!(problem.solution_size(sol).size, 2);
        // Check it's a valid matching using 4 vertices
        let mut used = [false; 4];
        for (idx, &sel) in sol.iter().enumerate() {
            if sel == 1 {
                if let Some((u, v)) = problem.edge_endpoints(idx) {
                    used[u] = true;
                    used[v] = true;
                }
            }
        }
        assert!(used.iter().all(|&u| u)); // All vertices matched
    }
}

#[test]
fn test_is_satisfied() {
    let problem = MaximumMatching::<SimpleGraph, i32>::unweighted(4, vec![(0, 1), (1, 2), (2, 3)]);

    assert!(problem.is_satisfied(&[1, 0, 1])); // Valid matching
    assert!(problem.is_satisfied(&[0, 1, 0])); // Valid matching
    assert!(!problem.is_satisfied(&[1, 1, 0])); // Share vertex 1
}

#[test]
fn test_objectives() {
    let problem = MaximumMatching::<SimpleGraph, i32>::new(3, vec![(0, 1, 5), (1, 2, 10)]);
    let objectives = problem.objectives();
    assert_eq!(objectives.len(), 2);
}

#[test]
fn test_set_weights() {
    let mut problem = MaximumMatching::<SimpleGraph, i32>::unweighted(3, vec![(0, 1), (1, 2)]);
    assert!(!problem.is_weighted()); // Initially uniform
    problem.set_weights(vec![1, 2]);
    assert!(problem.is_weighted());
    assert_eq!(problem.weights(), vec![1, 2]);
}

#[test]
fn test_is_weighted_empty() {
    let problem = MaximumMatching::<SimpleGraph, i32>::unweighted(2, vec![]);
    assert!(!problem.is_weighted());
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
fn test_problem_size() {
    let problem = MaximumMatching::<SimpleGraph, i32>::unweighted(5, vec![(0, 1), (1, 2), (2, 3)]);
    let size = problem.problem_size();
    assert_eq!(size.get("num_vertices"), Some(5));
    assert_eq!(size.get("num_edges"), Some(3));
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
fn test_matching_problem_v2() {
    use crate::traits::{OptimizationProblemV2, ProblemV2};
    use crate::types::Direction;

    // Path graph 0-1-2 with edges (0,1) and (1,2)
    let p = MaximumMatching::<SimpleGraph, i32>::unweighted(3, vec![(0, 1), (1, 2)]);
    assert_eq!(p.dims(), vec![2, 2]);
    // Valid matching: select edge 0 only
    assert_eq!(p.evaluate(&[1, 0]), 1);
    // Invalid matching: select both edges (vertex 1 shared)
    assert_eq!(p.evaluate(&[1, 1]), i32::MIN);
    assert_eq!(p.direction(), Direction::Maximize);
}
