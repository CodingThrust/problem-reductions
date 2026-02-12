use super::*;
use crate::solvers::{BruteForce, Solver};

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
fn test_solution_size_valid() {
    // Star graph: center dominates all
    let problem = MinimumDominatingSet::<SimpleGraph, i32>::new(4, vec![(0, 1), (0, 2), (0, 3)]);

    // Select center
    let sol = problem.solution_size(&[1, 0, 0, 0]);
    assert!(sol.is_valid);
    assert_eq!(sol.size, 1);

    // Select all leaves
    let sol = problem.solution_size(&[0, 1, 1, 1]);
    assert!(sol.is_valid);
    assert_eq!(sol.size, 3);
}

#[test]
fn test_solution_size_invalid() {
    let problem = MinimumDominatingSet::<SimpleGraph, i32>::new(4, vec![(0, 1), (2, 3)]);

    // Select none
    let sol = problem.solution_size(&[0, 0, 0, 0]);
    assert!(!sol.is_valid);

    // Select only vertex 0 (doesn't dominate 2, 3)
    let sol = problem.solution_size(&[1, 0, 0, 0]);
    assert!(!sol.is_valid);
}

#[test]
fn test_brute_force_star() {
    // Star graph: minimum dominating set is the center
    let problem = MinimumDominatingSet::<SimpleGraph, i32>::new(4, vec![(0, 1), (0, 2), (0, 3)]);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    assert!(solutions.contains(&vec![1, 0, 0, 0]));
    for sol in &solutions {
        assert_eq!(problem.solution_size(sol).size, 1);
    }
}

#[test]
fn test_brute_force_path() {
    // Path 0-1-2-3-4: need to dominate all 5 vertices
    let problem =
        MinimumDominatingSet::<SimpleGraph, i32>::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    // Minimum is 2 (e.g., vertices 1 and 3)
    for sol in &solutions {
        assert_eq!(problem.solution_size(sol).size, 2);
        assert!(problem.solution_size(sol).is_valid);
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

    let solutions = solver.find_best(&problem);
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
fn test_constraints() {
    let problem = MinimumDominatingSet::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
    let constraints = problem.constraints();
    assert_eq!(constraints.len(), 3); // One per vertex
}

#[test]
fn test_energy_mode() {
    let problem = MinimumDominatingSet::<SimpleGraph, i32>::new(2, vec![(0, 1)]);
    assert!(problem.energy_mode().is_minimization());
}

#[test]
fn test_isolated_vertex() {
    // Isolated vertex must be in dominating set
    let problem = MinimumDominatingSet::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    // Vertex 2 is isolated, must be selected
    for sol in &solutions {
        assert_eq!(sol[2], 1);
        assert!(problem.solution_size(sol).is_valid);
    }
}

#[test]
fn test_is_satisfied() {
    let problem = MinimumDominatingSet::<SimpleGraph, i32>::new(4, vec![(0, 1), (0, 2), (0, 3)]);

    assert!(problem.is_satisfied(&[1, 0, 0, 0])); // Center dominates all
    assert!(problem.is_satisfied(&[0, 1, 1, 1])); // Leaves dominate
    assert!(!problem.is_satisfied(&[0, 1, 0, 0])); // Missing 2 and 3
}

#[test]
fn test_objectives() {
    let problem =
        MinimumDominatingSet::<SimpleGraph, i32>::with_weights(3, vec![(0, 1)], vec![5, 10, 15]);
    let objectives = problem.objectives();
    assert_eq!(objectives.len(), 3);
}

#[test]
fn test_set_weights() {
    let mut problem = MinimumDominatingSet::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
    assert!(!problem.is_weighted()); // Initially uniform
    problem.set_weights(vec![1, 2, 3]);
    assert!(problem.is_weighted());
    assert_eq!(problem.weights(), vec![1, 2, 3]);
}

#[test]
fn test_is_weighted_empty() {
    let problem = MinimumDominatingSet::<SimpleGraph, i32>::with_weights(0, vec![], vec![]);
    assert!(!problem.is_weighted());
}

#[test]
fn test_is_dominating_set_wrong_len() {
    assert!(!is_dominating_set(3, &[(0, 1)], &[true, false]));
}

#[test]
fn test_problem_size() {
    let problem = MinimumDominatingSet::<SimpleGraph, i32>::new(5, vec![(0, 1), (1, 2), (2, 3)]);
    let size = problem.problem_size();
    assert_eq!(size.get("num_vertices"), Some(5));
    assert_eq!(size.get("num_edges"), Some(3));
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
fn test_variant() {
    let variant = MinimumDominatingSet::<SimpleGraph, i32>::variant();
    assert_eq!(variant.len(), 2);
    assert_eq!(variant[0], ("graph", "SimpleGraph"));
    assert_eq!(variant[1], ("weight", "i32"));
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
    use crate::traits::{OptimizationProblemV2, ProblemV2};
    use crate::types::Direction;

    // Path graph 0-1-2
    let p = MinimumDominatingSet::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
    assert_eq!(p.dims(), vec![2, 2, 2]);
    // Valid DS: select vertex 1 (dominates all)
    assert_eq!(p.evaluate(&[0, 1, 0]), 1);
    // Invalid DS: select no vertices
    assert_eq!(p.evaluate(&[0, 0, 0]), i32::MAX);
    assert_eq!(p.direction(), Direction::Minimize);
}
