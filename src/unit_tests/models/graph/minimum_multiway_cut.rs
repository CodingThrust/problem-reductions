use super::*;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};

#[test]
fn test_minimummultiwaycut_creation() {
    let graph = SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (0, 4), (1, 3)]);
    let problem = MinimumMultiwayCut::new(graph, vec![0, 2, 4], vec![2, 3, 1, 2, 4, 5]);
    assert_eq!(problem.dims().len(), 6);
    assert_eq!(problem.num_vertices(), 5);
    assert_eq!(problem.num_edges(), 6);
    assert_eq!(problem.num_terminals(), 3);
}

#[test]
fn test_minimummultiwaycut_evaluate_valid() {
    // Issue example: 5 vertices, terminals {0,2,4}
    // Edges: (0,1)w=2, (1,2)w=3, (2,3)w=1, (3,4)w=2, (0,4)w=4, (1,3)w=5
    let graph = SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (0, 4), (1, 3)]);
    let problem = MinimumMultiwayCut::new(graph, vec![0, 2, 4], vec![2, 3, 1, 2, 4, 5]);

    // Optimal cut: remove edges (0,1), (3,4), (0,4) => indices 0, 3, 4
    // config: [1, 0, 0, 1, 1, 0] => weight 2 + 2 + 4 = 8
    let config = vec![1, 0, 0, 1, 1, 0];
    let result = problem.evaluate(&config);
    assert_eq!(result, SolutionSize::Valid(8));
}

#[test]
fn test_minimummultiwaycut_evaluate_invalid() {
    let graph = SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (0, 4), (1, 3)]);
    let problem = MinimumMultiwayCut::new(graph, vec![0, 2, 4], vec![2, 3, 1, 2, 4, 5]);

    // No edges cut: all terminals connected => invalid
    let config = vec![0, 0, 0, 0, 0, 0];
    let result = problem.evaluate(&config);
    assert_eq!(result, SolutionSize::Invalid);
}

#[test]
fn test_minimummultiwaycut_direction() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumMultiwayCut::new(graph, vec![0, 2], vec![1i32, 1]);
    assert_eq!(problem.direction(), Direction::Minimize);
}

#[test]
fn test_minimummultiwaycut_brute_force() {
    // Issue example: optimal cut has weight 8
    let graph = SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (0, 4), (1, 3)]);
    let problem = MinimumMultiwayCut::new(graph, vec![0, 2, 4], vec![2, 3, 1, 2, 4, 5]);

    let solver = BruteForce::new();
    let solutions = solver.find_all_best(&problem);
    assert!(!solutions.is_empty());
    for sol in &solutions {
        let val = problem.evaluate(sol);
        assert_eq!(val, SolutionSize::Valid(8));
    }
}

#[test]
fn test_minimummultiwaycut_two_terminals() {
    // k=2: classical min s-t cut. Path graph: 0-1-2, terminals {0,2}
    // Edges: (0,1)w=3, (1,2)w=5
    // Min cut: remove (0,1) with weight 3
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumMultiwayCut::new(graph, vec![0, 2], vec![3i32, 5]);

    let solver = BruteForce::new();
    let solutions = solver.find_all_best(&problem);
    for sol in &solutions {
        assert_eq!(problem.evaluate(sol), SolutionSize::Valid(3));
    }
}

#[test]
fn test_minimummultiwaycut_all_edges_cut() {
    let graph = SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (0, 4), (1, 3)]);
    let problem = MinimumMultiwayCut::new(graph, vec![0, 2, 4], vec![2, 3, 1, 2, 4, 5]);
    let config = vec![1, 1, 1, 1, 1, 1];
    let result = problem.evaluate(&config);
    assert_eq!(result, SolutionSize::Valid(2 + 3 + 1 + 2 + 4 + 5));
}

#[test]
fn test_minimummultiwaycut_already_disconnected() {
    // Terminals already in different components => empty cut is valid
    // Graph: 0-1  2-3, terminals {0, 2}
    let graph = SimpleGraph::new(4, vec![(0, 1), (2, 3)]);
    let problem = MinimumMultiwayCut::new(graph, vec![0, 2], vec![1i32, 1]);
    let config = vec![0, 0];
    let result = problem.evaluate(&config);
    assert_eq!(result, SolutionSize::Valid(0));

    let solver = BruteForce::new();
    let solutions = solver.find_all_best(&problem);
    for sol in &solutions {
        assert_eq!(problem.evaluate(sol), SolutionSize::Valid(0));
    }
}

#[test]
fn test_minimummultiwaycut_serialization() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumMultiwayCut::new(graph, vec![0, 2], vec![1i32, 2]);
    let json = serde_json::to_string(&problem).unwrap();
    let restored: MinimumMultiwayCut<SimpleGraph, i32> = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.num_vertices(), 3);
    assert_eq!(restored.num_edges(), 2);
    assert_eq!(restored.terminals(), &[0, 2]);
}

#[test]
fn test_minimummultiwaycut_name() {
    assert_eq!(
        <MinimumMultiwayCut<SimpleGraph, i32> as Problem>::NAME,
        "MinimumMultiwayCut"
    );
}

#[test]
fn test_minimummultiwaycut_short_config_no_panic() {
    let graph = SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (0, 4), (1, 3)]);
    let problem = MinimumMultiwayCut::new(graph, vec![0, 2, 4], vec![2, 3, 1, 2, 4, 5]);

    let short_config = vec![1, 0];
    let result = problem.evaluate(&short_config);
    assert_ne!(result, SolutionSize::Valid(0));

    let empty_config: Vec<usize> = vec![];
    let result = problem.evaluate(&empty_config);
    assert_ne!(result, SolutionSize::Valid(0));
}
