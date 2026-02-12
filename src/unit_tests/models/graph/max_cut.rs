use super::*;
use crate::solvers::{BruteForce, Solver};

#[test]
fn test_maxcut_creation() {
    let problem = MaxCut::<SimpleGraph, i32>::new(4, vec![(0, 1, 1), (1, 2, 2), (2, 3, 3)]);
    assert_eq!(problem.num_vertices(), 4);
    assert_eq!(problem.num_edges(), 3);
    assert_eq!(problem.num_variables(), 4);
    assert_eq!(problem.num_flavors(), 2);
}

#[test]
fn test_maxcut_unweighted() {
    let problem = MaxCut::<SimpleGraph, i32>::unweighted(3, vec![(0, 1), (1, 2)]);
    assert_eq!(problem.num_edges(), 2);
}

#[test]
fn test_solution_size() {
    let problem = MaxCut::<SimpleGraph, i32>::new(3, vec![(0, 1, 1), (1, 2, 2), (0, 2, 3)]);

    // All same partition: no cut
    let sol = problem.solution_size(&[0, 0, 0]);
    assert_eq!(sol.size, 0);
    assert!(sol.is_valid);

    // 0 vs {1,2}: cuts edges 0-1 (1) and 0-2 (3) = 4
    let sol = problem.solution_size(&[0, 1, 1]);
    assert_eq!(sol.size, 4);

    // {0,2} vs {1}: cuts edges 0-1 (1) and 1-2 (2) = 3
    let sol = problem.solution_size(&[0, 1, 0]);
    assert_eq!(sol.size, 3);
}

#[test]
fn test_brute_force_triangle() {
    // Triangle with unit weights: max cut is 2
    let problem = MaxCut::<SimpleGraph, i32>::unweighted(3, vec![(0, 1), (1, 2), (0, 2)]);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    for sol in &solutions {
        let size = problem.solution_size(sol);
        assert_eq!(size.size, 2);
    }
}

#[test]
fn test_brute_force_path() {
    // Path 0-1-2: max cut is 2 (partition {0,2} vs {1})
    let problem = MaxCut::<SimpleGraph, i32>::unweighted(3, vec![(0, 1), (1, 2)]);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    for sol in &solutions {
        let size = problem.solution_size(sol);
        assert_eq!(size.size, 2);
    }
}

#[test]
fn test_brute_force_weighted() {
    // Edge with weight 10 should always be cut
    let problem = MaxCut::<SimpleGraph, i32>::new(3, vec![(0, 1, 10), (1, 2, 1)]);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    // Max is 11 (cut both edges) with partition like [0,1,0] or [1,0,1]
    for sol in &solutions {
        let size = problem.solution_size(sol);
        assert_eq!(size.size, 11);
    }
}

#[test]
fn test_cut_size_function() {
    let edges = vec![(0, 1, 1), (1, 2, 2), (0, 2, 3)];

    // Partition {0} vs {1, 2}
    assert_eq!(cut_size(&edges, &[false, true, true]), 4); // 1 + 3

    // Partition {0, 1} vs {2}
    assert_eq!(cut_size(&edges, &[false, false, true]), 5); // 2 + 3

    // All same partition
    assert_eq!(cut_size(&edges, &[false, false, false]), 0);
}

#[test]
fn test_edge_weight() {
    let problem = MaxCut::<SimpleGraph, i32>::new(3, vec![(0, 1, 5), (1, 2, 10)]);
    assert_eq!(problem.edge_weight(0, 1), Some(&5));
    assert_eq!(problem.edge_weight(1, 2), Some(&10));
    assert_eq!(problem.edge_weight(0, 2), None);
}

#[test]
fn test_edges() {
    let problem = MaxCut::<SimpleGraph, i32>::new(3, vec![(0, 1, 1), (1, 2, 2)]);
    let edges = problem.edges();
    assert_eq!(edges.len(), 2);
}

#[test]
fn test_energy_mode() {
    let problem = MaxCut::<SimpleGraph, i32>::unweighted(2, vec![(0, 1)]);
    assert!(problem.energy_mode().is_maximization());
}

#[test]
fn test_empty_graph() {
    let problem = MaxCut::<SimpleGraph, i32>::unweighted(3, vec![]);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    // Any partition gives cut size 0
    assert!(!solutions.is_empty());
    for sol in &solutions {
        assert_eq!(problem.solution_size(sol).size, 0);
    }
}

#[test]
fn test_single_edge() {
    let problem = MaxCut::<SimpleGraph, i32>::new(2, vec![(0, 1, 5)]);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    // Putting vertices in different sets maximizes cut
    assert_eq!(solutions.len(), 2); // [0,1] and [1,0]
    for sol in &solutions {
        assert_eq!(problem.solution_size(sol).size, 5);
    }
}

#[test]
fn test_complete_graph_k4() {
    // K4: every partition cuts exactly 4 edges (balanced) or less
    let problem = MaxCut::<SimpleGraph, i32>::unweighted(
        4,
        vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)],
    );
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    // Max cut in K4 is 4 (2-2 partition)
    for sol in &solutions {
        assert_eq!(problem.solution_size(sol).size, 4);
    }
}

#[test]
fn test_bipartite_graph() {
    // Complete bipartite K_{2,2}: max cut is all 4 edges
    let problem = MaxCut::<SimpleGraph, i32>::unweighted(4, vec![(0, 2), (0, 3), (1, 2), (1, 3)]);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    // Bipartite graph can achieve max cut = all edges
    for sol in &solutions {
        assert_eq!(problem.solution_size(sol).size, 4);
    }
}

#[test]
fn test_symmetry() {
    // Complementary partitions should give same cut
    let problem = MaxCut::<SimpleGraph, i32>::unweighted(3, vec![(0, 1), (1, 2), (0, 2)]);

    let sol1 = problem.solution_size(&[0, 1, 1]);
    let sol2 = problem.solution_size(&[1, 0, 0]); // complement
    assert_eq!(sol1.size, sol2.size);
}

#[test]
fn test_from_graph() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MaxCut::<SimpleGraph, i32>::from_graph(graph, vec![5, 10]);
    assert_eq!(problem.num_vertices(), 3);
    assert_eq!(problem.num_edges(), 2);
    assert_eq!(problem.edge_weights(), vec![5, 10]);
}

#[test]
fn test_from_graph_unweighted() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MaxCut::<SimpleGraph, i32>::from_graph_unweighted(graph);
    assert_eq!(problem.num_vertices(), 3);
    assert_eq!(problem.num_edges(), 2);
    assert_eq!(problem.edge_weights(), vec![1, 1]);
}

#[test]
fn test_graph_accessor() {
    let problem = MaxCut::<SimpleGraph, i32>::unweighted(3, vec![(0, 1), (1, 2)]);
    let graph = problem.graph();
    assert_eq!(graph.num_vertices(), 3);
    assert_eq!(graph.num_edges(), 2);
}

#[test]
fn test_with_weights() {
    let problem = MaxCut::<SimpleGraph, i32>::with_weights(3, vec![(0, 1), (1, 2)], vec![7, 3]);
    assert_eq!(problem.edge_weights(), vec![7, 3]);
}

#[test]
fn test_edge_weight_by_index() {
    let problem = MaxCut::<SimpleGraph, i32>::new(3, vec![(0, 1, 5), (1, 2, 10)]);
    assert_eq!(problem.edge_weight_by_index(0), Some(&5));
    assert_eq!(problem.edge_weight_by_index(1), Some(&10));
    assert_eq!(problem.edge_weight_by_index(2), None);
}

#[test]
fn test_variant() {
    let variant = MaxCut::<SimpleGraph, i32>::variant();
    assert_eq!(variant.len(), 2);
    assert_eq!(variant[0], ("graph", "SimpleGraph"));
    assert_eq!(variant[1], ("weight", "i32"));
}

#[test]
fn test_maxcut_problem_v2() {
    use crate::traits::{OptimizationProblemV2, ProblemV2};
    use crate::types::Direction;

    // Triangle with unit edge weights
    let p = MaxCut::<SimpleGraph, i32>::unweighted(3, vec![(0, 1), (1, 2), (0, 2)]);
    assert_eq!(p.dims(), vec![2, 2, 2]);
    // Partition {0} vs {1,2}: cuts edges (0,1) and (0,2), weight = 2
    assert_eq!(p.evaluate(&[1, 0, 0]), 2);
    // All same partition: no cut, weight = 0
    assert_eq!(p.evaluate(&[0, 0, 0]), 0);
    assert_eq!(p.direction(), Direction::Maximize);
}
