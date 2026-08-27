use super::*;
use crate::models::graph::{MaximumIndependentSet, MinimumVertexCover};
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::types::{Max, Min};

#[test]
fn test_decision_search_min() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumVertexCover::new(graph, vec![1i64; 3]);

    assert_eq!(solve_via_decision(&problem, 0, 3).unwrap(), Some(1));
}

#[test]
fn test_decision_search_max() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MaximumIndependentSet::new(graph, vec![1i64; 3]);

    assert_eq!(solve_via_decision(&problem, 0, 3).unwrap(), Some(2));
}

#[test]
fn test_decision_search_matches_brute_force() {
    let graph = SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)]);
    let problem = MinimumVertexCover::new(graph, vec![1i64; 5]);

    let solution = BruteForce::new().solve(&problem).unwrap().unwrap();
    let brute_force_value = problem.evaluate(&solution).unwrap();

    assert_eq!(
        solve_via_decision(&problem, 0, 5).unwrap(),
        brute_force_value.size().copied()
    );
}

#[test]
fn test_decision_search_min_returns_none_when_upper_bound_is_too_small() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumVertexCover::new(graph, vec![1i64; 3]);

    assert_eq!(solve_via_decision(&problem, 0, 0).unwrap(), None);
}

#[test]
fn test_decision_search_max_returns_none_when_interval_is_above_optimum() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MaximumIndependentSet::new(graph, vec![1i64; 3]);

    assert_eq!(solve_via_decision(&problem, 3, 4).unwrap(), None);
}

#[test]
fn test_decision_search_invalid_interval_returns_none() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let min_problem = MinimumVertexCover::new(graph.clone(), vec![1i64; 3]);
    let max_problem = MaximumIndependentSet::new(graph, vec![1i64; 3]);

    assert_eq!(solve_via_decision(&min_problem, 2, 1).unwrap(), None);
    assert_eq!(solve_via_decision(&max_problem, 2, 1).unwrap(), None);
}

#[test]
fn test_decision_search_preserves_value_direction() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let min_problem = MinimumVertexCover::new(graph.clone(), vec![1i64; 3]);
    let max_problem = MaximumIndependentSet::new(graph, vec![1i64; 3]);

    let min_solution = BruteForce::new().solve(&min_problem).unwrap().unwrap();
    let max_solution = BruteForce::new().solve(&max_problem).unwrap().unwrap();
    let min_value = min_problem.evaluate(&min_solution).unwrap();
    let max_value = max_problem.evaluate(&max_solution).unwrap();

    assert_eq!(min_value, Min(Some(1)));
    assert_eq!(max_value, Max(Some(2)));
    assert_eq!(solve_via_decision(&min_problem, 0, 3).unwrap(), Some(1));
    assert_eq!(solve_via_decision(&max_problem, 0, 3).unwrap(), Some(2));
}
