use super::*;
use crate::models::graph::{MaximumIndependentSet, MinimumVertexCover};
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::types::{Max, Min};
use crate::Solver;

#[test]
fn test_golden_section_min() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumVertexCover::new(graph, vec![1i32; 3]);

    assert_eq!(solve_via_decision(&problem, 0, 3), Some(1));
}

#[test]
fn test_golden_section_max() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MaximumIndependentSet::new(graph, vec![1i32; 3]);

    assert_eq!(solve_via_decision(&problem, 0, 3), Some(2));
}

#[test]
fn test_golden_section_matches_brute_force() {
    let graph = SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)]);
    let problem = MinimumVertexCover::new(graph, vec![1i32; 5]);

    let brute_force_value = BruteForce::new().solve(&problem);

    assert_eq!(solve_via_decision(&problem, 0, 5), brute_force_value.size().copied());
}

#[test]
fn test_golden_section_min_returns_none_when_upper_bound_is_too_small() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumVertexCover::new(graph, vec![1i32; 3]);

    assert_eq!(solve_via_decision(&problem, 0, 0), None);
}

#[test]
fn test_golden_section_max_returns_none_when_interval_is_above_optimum() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MaximumIndependentSet::new(graph, vec![1i32; 3]);

    assert_eq!(solve_via_decision(&problem, 3, 4), None);
}

#[test]
fn test_golden_section_invalid_interval_returns_none() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let min_problem = MinimumVertexCover::new(graph.clone(), vec![1i32; 3]);
    let max_problem = MaximumIndependentSet::new(graph, vec![1i32; 3]);

    assert_eq!(solve_via_decision(&min_problem, 2, 1), None);
    assert_eq!(solve_via_decision(&max_problem, 2, 1), None);
}

#[test]
fn test_golden_section_preserves_value_direction() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let min_problem = MinimumVertexCover::new(graph.clone(), vec![1i32; 3]);
    let max_problem = MaximumIndependentSet::new(graph, vec![1i32; 3]);

    let min_value = BruteForce::new().solve(&min_problem);
    let max_value = BruteForce::new().solve(&max_problem);

    assert_eq!(min_value, Min(Some(1)));
    assert_eq!(max_value, Max(Some(2)));
    assert_eq!(solve_via_decision(&min_problem, 0, 3), Some(1));
    assert_eq!(solve_via_decision(&max_problem, 0, 3), Some(2));
}
