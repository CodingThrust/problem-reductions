use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::topology::SimpleGraph;
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};

fn issue_example() -> MinimumGraphBandwidth<SimpleGraph> {
    MinimumGraphBandwidth::new(SimpleGraph::new(
        6,
        vec![(0, 1), (1, 2), (0, 3), (1, 4), (2, 5), (3, 4), (4, 5)],
    ))
}

fn path_p4() -> MinimumGraphBandwidth<SimpleGraph> {
    MinimumGraphBandwidth::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]))
}

fn clique_k4() -> MinimumGraphBandwidth<SimpleGraph> {
    MinimumGraphBandwidth::new(SimpleGraph::new(
        4,
        vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)],
    ))
}

fn star_k13() -> MinimumGraphBandwidth<SimpleGraph> {
    MinimumGraphBandwidth::new(SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3)]))
}

#[test]
fn test_minimumgraphbandwidth_creation() {
    let problem = issue_example();
    assert_eq!(problem.graph().num_vertices(), 6);
    assert_eq!(problem.graph().num_edges(), 7);
    assert_eq!(problem.num_vertices(), 6);
    assert_eq!(problem.num_edges(), 7);
    assert_eq!(problem.dims(), vec![6; 6]);
    assert_eq!(problem.num_variables(), 6);
}

#[test]
fn test_minimumgraphbandwidth_direction() {
    assert_eq!(issue_example().direction(), Direction::Minimize);
}

#[test]
fn test_minimumgraphbandwidth_invalid_permutation() {
    let problem = issue_example();

    assert_eq!(problem.evaluate(&[0, 0, 1, 2, 3, 4]), SolutionSize::Invalid);
    assert_eq!(problem.max_edge_span(&[0, 0, 1, 2, 3, 4]), None);

    assert_eq!(problem.evaluate(&[0, 1, 2, 3, 4, 6]), SolutionSize::Invalid);
    assert_eq!(problem.max_edge_span(&[0, 1, 2, 3, 4, 6]), None);

    assert_eq!(problem.evaluate(&[0, 1, 2]), SolutionSize::Invalid);
    assert_eq!(problem.max_edge_span(&[0, 1, 2]), None);
}

#[test]
fn test_minimumgraphbandwidth_issue_example() {
    let problem = issue_example();
    let column_major = vec![0, 2, 4, 1, 3, 5];
    let row_major = vec![0, 1, 2, 3, 4, 5];

    assert_eq!(problem.max_edge_span(&column_major), Some(2));
    assert_eq!(problem.evaluate(&column_major), SolutionSize::Valid(2));

    assert_eq!(problem.max_edge_span(&row_major), Some(3));
    assert_eq!(problem.evaluate(&row_major), SolutionSize::Valid(3));
}

#[test]
fn test_minimumgraphbandwidth_closed_form_graphs() {
    let path = path_p4();
    let path_solution = BruteForce::new().find_best(&path).unwrap();
    assert_eq!(path.evaluate(&path_solution), SolutionSize::Valid(1));

    let clique = clique_k4();
    let clique_solution = BruteForce::new().find_best(&clique).unwrap();
    assert_eq!(clique.evaluate(&clique_solution), SolutionSize::Valid(3));

    let star = star_k13();
    let star_solution = BruteForce::new().find_best(&star).unwrap();
    assert_eq!(star.evaluate(&star_solution), SolutionSize::Valid(2));
}

#[test]
fn test_minimumgraphbandwidth_solver() {
    let problem = issue_example();
    let solver = BruteForce::new();

    let best = solver.find_best(&problem).unwrap();
    assert_eq!(problem.evaluate(&best), SolutionSize::Valid(2));

    let all_best = solver.find_all_best(&problem);
    assert!(!all_best.is_empty());
    for config in &all_best {
        assert_eq!(problem.evaluate(config), SolutionSize::Valid(2));
    }
}

#[test]
fn test_minimumgraphbandwidth_serialization() {
    let problem = issue_example();
    let json = serde_json::to_string(&problem).unwrap();
    let restored: MinimumGraphBandwidth<SimpleGraph> = serde_json::from_str(&json).unwrap();

    assert_eq!(restored.graph().num_vertices(), 6);
    assert_eq!(restored.graph().num_edges(), 7);
    assert_eq!(
        restored.evaluate(&[0, 2, 4, 1, 3, 5]),
        SolutionSize::Valid(2)
    );
}

#[test]
fn test_minimumgraphbandwidth_problem_name() {
    assert_eq!(
        <MinimumGraphBandwidth<SimpleGraph> as Problem>::NAME,
        "MinimumGraphBandwidth"
    );
}
