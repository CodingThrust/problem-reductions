use super::*;
use crate::solvers::BruteForceProblem as _;
#[test]
fn create_spec_rejects_nonpositive_lengths() {
    assert!(LongestPath::try_from(LongestPathI64CreateSpec {
        graph: vec![(0, 1)],
        num_vertices: None,
        edge_lengths: vec![0],
        source_vertex: 0,
        target_vertex: 1
    })
    .is_err());
}
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::{Max, One};

fn issue_problem() -> LongestPath<SimpleGraph, i64> {
    LongestPath::new(
        SimpleGraph::new(
            7,
            vec![
                (0, 1),
                (0, 2),
                (1, 3),
                (2, 3),
                (2, 4),
                (3, 5),
                (4, 5),
                (4, 6),
                (5, 6),
                (1, 6),
            ],
        ),
        vec![3, 2, 4, 1, 5, 2, 3, 2, 4, 1],
        0,
        6,
    )
}

fn optimal_config() -> Vec<bool> {
    vec![
        true, false, true, true, true, false, true, false, true, false,
    ]
}

fn suboptimal_config() -> Vec<bool> {
    vec![
        false, true, true, false, true, true, true, false, false, true,
    ]
}

#[test]
fn test_longest_path_creation() {
    let mut problem = issue_problem();

    assert_eq!(problem.graph().num_vertices(), 7);
    assert_eq!(problem.graph().num_edges(), 10);
    assert_eq!(problem.num_vertices(), 7);
    assert_eq!(problem.num_edges(), 10);
    assert_eq!(problem.source_vertex(), 0);
    assert_eq!(problem.target_vertex(), 6);
    assert_eq!(problem.dimensions(), vec![2; 10]);
    assert_eq!(problem.edge_lengths(), &[3, 2, 4, 1, 5, 2, 3, 2, 4, 1]);
    assert!(problem.is_weighted());

    problem.set_lengths(vec![1; 10]);
    assert_eq!(problem.edge_lengths(), &[1; 10]);

    let unweighted = LongestPath::new(SimpleGraph::path(4), vec![One; 3], 0, 3);
    assert!(!unweighted.is_weighted());
}

#[test]
fn test_longest_path_evaluate_valid_and_invalid_configs() {
    let problem = issue_problem();

    assert_eq!(problem.evaluate(&optimal_config()).unwrap(), Max(Some(20)));
    assert_eq!(
        problem.evaluate(&suboptimal_config()).unwrap(),
        Max(Some(17))
    );
    assert!(problem.is_valid_solution(&optimal_config()));
    assert!(problem.is_valid_solution(&suboptimal_config()));

    assert_eq!(
        problem
            .evaluate(&vec![
                true, true, true, false, false, false, false, false, false, false
            ])
            .unwrap(),
        Max(None)
    );
    assert_eq!(
        problem
            .evaluate(&vec![
                true, false, true, false, true, false, false, false, false, true
            ])
            .unwrap(),
        Max(None)
    );
    assert_eq!(
        problem
            .evaluate(&vec![
                true, false, true, true, true, true, true, true, true, true
            ])
            .unwrap(),
        Max(None)
    );
    assert_eq!(problem.evaluate(&vec![false; 10]).unwrap(), Max(None));
    assert!(!problem.is_valid_solution(&[true, false, true]));
    assert!(crate::registry::DynProblem::evaluate_dyn(
        &problem,
        &serde_json::json!([1, 0, 1, 0, 1, 0, 1, 0, 1, 2]),
    )
    .is_err());
}

#[test]
fn test_longest_path_bruteforce_finds_issue_optimum() {
    let problem = issue_problem();
    let solver = BruteForce::new();

    let best = solver.solve(&problem).unwrap().unwrap();
    assert_eq!(best, optimal_config());
    assert_eq!(problem.evaluate(&best).unwrap(), Max(Some(20)));

    let all_best = solver.find_all_witnesses(&problem).unwrap();
    assert_eq!(all_best, vec![optimal_config()]);
}

#[test]
fn test_longest_path_serialization() {
    let problem = issue_problem();
    let json = serde_json::to_value(&problem).unwrap();
    let restored: LongestPath<SimpleGraph, i64> = serde_json::from_value(json).unwrap();

    assert_eq!(restored.num_vertices(), 7);
    assert_eq!(restored.num_edges(), 10);
    assert_eq!(restored.source_vertex(), 0);
    assert_eq!(restored.target_vertex(), 6);
    assert_eq!(restored.edge_lengths(), &[3, 2, 4, 1, 5, 2, 3, 2, 4, 1]);
    assert_eq!(restored.evaluate(&optimal_config()).unwrap(), Max(Some(20)));
}

#[test]
fn test_longest_path_source_equals_target_only_allows_empty_path() {
    let problem = LongestPath::new(SimpleGraph::path(3), vec![5, 7], 1, 1);

    assert!(problem.is_valid_solution(&[false, false]));
    assert_eq!(problem.evaluate(&vec![false, false]).unwrap(), Max(Some(0)));
    assert!(!problem.is_valid_solution(&[true, false]));
    assert_eq!(problem.evaluate(&vec![true, false]).unwrap(), Max(None));

    let best = BruteForce::new().solve(&problem).unwrap().unwrap();
    assert_eq!(best, vec![false, false]);
}

#[test]
fn test_longestpath_paper_example() {
    let problem = issue_problem();

    assert_eq!(problem.evaluate(&optimal_config()).unwrap(), Max(Some(20)));
    assert_eq!(
        problem.evaluate(&suboptimal_config()).unwrap(),
        Max(Some(17))
    );
    assert_eq!(
        problem
            .evaluate(&vec![
                true, true, true, false, false, false, false, false, false, false
            ])
            .unwrap(),
        Max(None)
    );
}

#[test]
fn test_longest_path_problem_name() {
    assert_eq!(
        <LongestPath<SimpleGraph, i64> as Problem>::NAME,
        "LongestPath"
    );
}

#[test]
#[should_panic(expected = "edge_lengths length must match num_edges")]
fn test_longest_path_rejects_wrong_edge_lengths_len() {
    LongestPath::new(SimpleGraph::path(3), vec![1], 0, 2);
}

#[test]
#[should_panic(expected = "All edge lengths must be positive (> 0)")]
fn test_longest_path_rejects_non_positive_edge_lengths() {
    LongestPath::new(SimpleGraph::path(2), vec![0], 0, 1);
}

#[test]
#[should_panic(expected = "source_vertex 3 out of bounds (graph has 3 vertices)")]
fn test_longest_path_rejects_out_of_bounds_source() {
    LongestPath::new(SimpleGraph::path(3), vec![1, 1], 3, 2);
}

#[test]
#[should_panic(expected = "target_vertex 3 out of bounds (graph has 3 vertices)")]
fn test_longest_path_rejects_out_of_bounds_target() {
    LongestPath::new(SimpleGraph::path(3), vec![1, 1], 0, 3);
}
