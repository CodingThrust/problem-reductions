use super::*;
use crate::solvers::BruteForceProblem as _;

#[test]
fn create_spec_rejects_invalid_terminals() {
    assert_eq!(MinimumMultiwayCutCreateSpec::FIELDS[1].name, "terminals");
    let result = MinimumMultiwayCut::try_from(MinimumMultiwayCutCreateSpec {
        graph: SimpleGraph::new(2, vec![(0, 1)]),
        terminals: vec![0, 0],
        edge_weights: vec![1],
    });
    assert!(result.is_err());
}
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_minimummultiwaycut_creation() {
    let graph = SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (0, 4), (1, 3)]);
    let problem = MinimumMultiwayCut::new(graph, vec![0, 2, 4], vec![2, 3, 1, 2, 4, 5]);
    assert_eq!(problem.dimensions().len(), 6);
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
    let config = vec![true, false, false, true, true, false];
    let result = problem.evaluate(&config).unwrap();
    assert_eq!(result, Min(Some(8)));
}

#[test]
fn test_minimummultiwaycut_evaluate_invalid() {
    let graph = SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (0, 4), (1, 3)]);
    let problem = MinimumMultiwayCut::new(graph, vec![0, 2, 4], vec![2, 3, 1, 2, 4, 5]);

    // No edges cut: all terminals connected => invalid
    let config = vec![false, false, false, false, false, false];
    let result = problem.evaluate(&config).unwrap();
    assert_eq!(result, Min(None));
}

#[test]
fn test_minimummultiwaycut_brute_force() {
    // Issue example: optimal cut has weight 8
    let graph = SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (0, 4), (1, 3)]);
    let problem = MinimumMultiwayCut::new(graph, vec![0, 2, 4], vec![2, 3, 1, 2, 4, 5]);

    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem).unwrap();
    assert!(!solutions.is_empty());
    for sol in &solutions {
        let val = problem.evaluate(sol).unwrap();
        assert_eq!(val, Min(Some(8)));
    }
    // Verify the claimed optimal cut [1,0,0,1,1,0] is among solutions
    let claimed_optimal = vec![true, false, false, true, true, false];
    assert!(
        solutions.contains(&claimed_optimal),
        "expected optimal config {:?} not found in brute-force solutions",
        claimed_optimal
    );
}

#[test]
fn test_minimummultiwaycut_two_terminals() {
    // k=2: classical min s-t cut. Path graph: 0-1-2, terminals {0,2}
    // Edges: (0,1)w=3, (1,2)w=5
    // Min cut: remove (0,1) with weight 3
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumMultiwayCut::new(graph, vec![0, 2], vec![3i64, 5]);

    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem).unwrap();
    for sol in &solutions {
        assert_eq!(problem.evaluate(sol).unwrap(), Min(Some(3)));
    }
}

#[test]
fn test_minimummultiwaycut_all_edges_cut() {
    let graph = SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (0, 4), (1, 3)]);
    let problem = MinimumMultiwayCut::new(graph, vec![0, 2, 4], vec![2, 3, 1, 2, 4, 5]);
    let config = vec![true, true, true, true, true, true];
    let result = problem.evaluate(&config).unwrap();
    assert_eq!(result, Min(Some(2 + 3 + 1 + 2 + 4 + 5)));
}

#[test]
fn test_minimummultiwaycut_already_disconnected() {
    // Terminals already in different components => empty cut is valid
    // Graph: 0-1  2-3, terminals {0, 2}
    let graph = SimpleGraph::new(4, vec![(0, 1), (2, 3)]);
    let problem = MinimumMultiwayCut::new(graph, vec![0, 2], vec![1i64, 1]);
    let config = vec![false, false];
    let result = problem.evaluate(&config).unwrap();
    assert_eq!(result, Min(Some(0)));

    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem).unwrap();
    for sol in &solutions {
        assert_eq!(problem.evaluate(sol).unwrap(), Min(Some(0)));
    }
}

#[test]
fn test_minimummultiwaycut_serialization() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumMultiwayCut::new(graph, vec![0, 2], vec![1i64, 2]);
    let json = serde_json::to_string(&problem).unwrap();
    let restored: MinimumMultiwayCut<SimpleGraph, i64> = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.num_vertices(), 3);
    assert_eq!(restored.num_edges(), 2);
    assert_eq!(restored.terminals(), &[0, 2]);
}

#[test]
fn test_minimummultiwaycut_name() {
    assert_eq!(
        <MinimumMultiwayCut<SimpleGraph, i64> as Problem>::NAME,
        "MinimumMultiwayCut"
    );
}

#[test]
#[should_panic(expected = "edge_weights length must match num_edges")]
fn test_minimummultiwaycut_panic_wrong_weights_len() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    MinimumMultiwayCut::new(graph, vec![0, 2], vec![1i64]);
}

#[test]
#[should_panic(expected = "need at least 2 terminals")]
fn test_minimummultiwaycut_panic_too_few_terminals() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    MinimumMultiwayCut::new(graph, vec![0], vec![1i64, 1]);
}

#[test]
#[should_panic(expected = "duplicate terminal indices")]
fn test_minimummultiwaycut_panic_duplicate_terminals() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    MinimumMultiwayCut::new(graph, vec![0, 0], vec![1i64, 1]);
}

#[test]
#[should_panic(expected = "terminal index out of bounds")]
fn test_minimummultiwaycut_panic_terminal_out_of_bounds() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    MinimumMultiwayCut::new(graph, vec![0, 10], vec![1i64, 1]);
}

#[test]
fn test_minimummultiwaycut_getters() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumMultiwayCut::new(graph, vec![0, 2], vec![3i64, 5]);
    assert_eq!(problem.graph().num_vertices(), 3);
    assert_eq!(problem.edge_weights(), &[3, 5]);
}

#[test]
fn test_minimummultiwaycut_rejects_wrong_config_lengths() {
    let graph = SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (0, 4), (1, 3)]);
    let problem = MinimumMultiwayCut::new(graph, vec![0, 2, 4], vec![2, 3, 1, 2, 4, 5]);

    let short_config = vec![true, false];
    assert!(matches!(
        problem.evaluate(&short_config),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));

    let empty_config: Vec<bool> = vec![];
    assert!(matches!(
        problem.evaluate(&empty_config),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
}
