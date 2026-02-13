use super::*;
use crate::solvers::BruteForce;
use crate::types::SolutionSize;

#[test]
fn test_maxcut_creation() {
    use crate::traits::Problem;

    let problem = MaxCut::<SimpleGraph, i32>::new(4, vec![(0, 1, 1), (1, 2, 2), (2, 3, 3)]);
    assert_eq!(problem.num_vertices(), 4);
    assert_eq!(problem.num_edges(), 3);
    assert_eq!(problem.dims(), vec![2, 2, 2, 2]);
}

#[test]
fn test_maxcut_unweighted() {
    let problem = MaxCut::<SimpleGraph, i32>::unweighted(3, vec![(0, 1), (1, 2)]);
    assert_eq!(problem.num_edges(), 2);
}

#[test]
fn test_evaluate() {
    use crate::traits::Problem;

    let problem = MaxCut::<SimpleGraph, i32>::new(3, vec![(0, 1, 1), (1, 2, 2), (0, 2, 3)]);

    // All same partition: no cut
    assert_eq!(problem.evaluate(&[0, 0, 0]), SolutionSize::Valid(0));

    // 0 vs {1,2}: cuts edges 0-1 (1) and 0-2 (3) = 4
    assert_eq!(problem.evaluate(&[0, 1, 1]), SolutionSize::Valid(4));

    // {0,2} vs {1}: cuts edges 0-1 (1) and 1-2 (2) = 3
    assert_eq!(problem.evaluate(&[0, 1, 0]), SolutionSize::Valid(3));
}

#[test]
fn test_brute_force_triangle() {
    use crate::traits::Problem;

    // Triangle with unit weights: max cut is 2
    let problem = MaxCut::<SimpleGraph, i32>::unweighted(3, vec![(0, 1), (1, 2), (0, 2)]);
    let solver = BruteForce::new();

    let solutions = solver.find_all_best(&problem);
    for sol in &solutions {
        let size = problem.evaluate(sol);
        assert_eq!(size, SolutionSize::Valid(2));
    }
}

#[test]
fn test_brute_force_path() {
    use crate::traits::Problem;

    // Path 0-1-2: max cut is 2 (partition {0,2} vs {1})
    let problem = MaxCut::<SimpleGraph, i32>::unweighted(3, vec![(0, 1), (1, 2)]);
    let solver = BruteForce::new();

    let solutions = solver.find_all_best(&problem);
    for sol in &solutions {
        let size = problem.evaluate(sol);
        assert_eq!(size, SolutionSize::Valid(2));
    }
}

#[test]
fn test_brute_force_weighted() {
    use crate::traits::Problem;

    // Edge with weight 10 should always be cut
    let problem = MaxCut::<SimpleGraph, i32>::new(3, vec![(0, 1, 10), (1, 2, 1)]);
    let solver = BruteForce::new();

    let solutions = solver.find_all_best(&problem);
    // Max is 11 (cut both edges) with partition like [0,1,0] or [1,0,1]
    for sol in &solutions {
        let size = problem.evaluate(sol);
        assert_eq!(size, SolutionSize::Valid(11));
    }
}

#[test]
fn test_cut_size_function() {
    use crate::topology::SimpleGraph;
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let weights = vec![1, 2, 3];

    // Partition {0} vs {1, 2}
    assert_eq!(cut_size(&graph, &weights, &[false, true, true]), 4); // 1 + 3

    // Partition {0, 1} vs {2}
    assert_eq!(cut_size(&graph, &weights, &[false, false, true]), 5); // 2 + 3

    // All same partition
    assert_eq!(cut_size(&graph, &weights, &[false, false, false]), 0);
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
fn test_direction() {
    use crate::traits::OptimizationProblem;
    use crate::types::Direction;

    let problem = MaxCut::<SimpleGraph, i32>::unweighted(2, vec![(0, 1)]);
    assert_eq!(problem.direction(), Direction::Maximize);
}

#[test]
fn test_empty_graph() {
    use crate::traits::Problem;

    let problem = MaxCut::<SimpleGraph, i32>::unweighted(3, vec![]);
    let solver = BruteForce::new();

    let solutions = solver.find_all_best(&problem);
    // Any partition gives cut size 0
    assert!(!solutions.is_empty());
    for sol in &solutions {
        assert_eq!(problem.evaluate(sol), SolutionSize::Valid(0));
    }
}

#[test]
fn test_single_edge() {
    use crate::traits::Problem;

    let problem = MaxCut::<SimpleGraph, i32>::new(2, vec![(0, 1, 5)]);
    let solver = BruteForce::new();

    let solutions = solver.find_all_best(&problem);
    // Putting vertices in different sets maximizes cut
    assert_eq!(solutions.len(), 2); // [0,1] and [1,0]
    for sol in &solutions {
        assert_eq!(problem.evaluate(sol), SolutionSize::Valid(5));
    }
}

#[test]
fn test_complete_graph_k4() {
    use crate::traits::Problem;

    // K4: every partition cuts exactly 4 edges (balanced) or less
    let problem = MaxCut::<SimpleGraph, i32>::unweighted(
        4,
        vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)],
    );
    let solver = BruteForce::new();

    let solutions = solver.find_all_best(&problem);
    // Max cut in K4 is 4 (2-2 partition)
    for sol in &solutions {
        assert_eq!(problem.evaluate(sol), SolutionSize::Valid(4));
    }
}

#[test]
fn test_bipartite_graph() {
    use crate::traits::Problem;

    // Complete bipartite K_{2,2}: max cut is all 4 edges
    let problem = MaxCut::<SimpleGraph, i32>::unweighted(4, vec![(0, 2), (0, 3), (1, 2), (1, 3)]);
    let solver = BruteForce::new();

    let solutions = solver.find_all_best(&problem);
    // Bipartite graph can achieve max cut = all edges
    for sol in &solutions {
        assert_eq!(problem.evaluate(sol), SolutionSize::Valid(4));
    }
}

#[test]
fn test_symmetry() {
    use crate::traits::Problem;

    // Complementary partitions should give same cut
    let problem = MaxCut::<SimpleGraph, i32>::unweighted(3, vec![(0, 1), (1, 2), (0, 2)]);

    let sol1 = problem.evaluate(&[0, 1, 1]);
    let sol2 = problem.evaluate(&[1, 0, 0]); // complement
    assert_eq!(sol1, sol2);
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
fn test_maxcut_problem() {
    use crate::traits::{OptimizationProblem, Problem};
    use crate::types::Direction;

    // Triangle with unit edge weights
    let p = MaxCut::<SimpleGraph, i32>::unweighted(3, vec![(0, 1), (1, 2), (0, 2)]);
    assert_eq!(p.dims(), vec![2, 2, 2]);
    // Partition {0} vs {1,2}: cuts edges (0,1) and (0,2), weight = 2
    assert_eq!(p.evaluate(&[1, 0, 0]), SolutionSize::Valid(2));
    // All same partition: no cut, weight = 0
    assert_eq!(p.evaluate(&[0, 0, 0]), SolutionSize::Valid(0));
    assert_eq!(p.direction(), Direction::Maximize);
}
