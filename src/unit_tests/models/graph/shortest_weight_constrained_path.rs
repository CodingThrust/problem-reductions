use super::*;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Min;

fn issue_problem() -> ShortestWeightConstrainedPath<SimpleGraph, i32> {
    ShortestWeightConstrainedPath::new(
        SimpleGraph::new(
            6,
            vec![
                (0, 1),
                (0, 2),
                (1, 3),
                (2, 3),
                (2, 4),
                (3, 5),
                (4, 5),
                (1, 4),
            ],
        ),
        vec![2, 4, 3, 1, 5, 4, 2, 6],
        vec![5, 1, 2, 3, 2, 3, 1, 1],
        0,
        5,
        8,
    )
}

#[test]
fn test_shortest_weight_constrained_path_creation() {
    let problem = issue_problem();
    assert_eq!(problem.num_vertices(), 6);
    assert_eq!(problem.num_edges(), 8);
    assert_eq!(problem.source_vertex(), 0);
    assert_eq!(problem.target_vertex(), 5);
    assert_eq!(*problem.weight_bound(), 8);
    assert_eq!(problem.dims(), vec![2; 8]);
    assert!(problem.is_weighted());
}

#[test]
fn test_shortest_weight_constrained_path_evaluation() {
    let problem = issue_problem();

    // Path 0-2-3-5: length=4+1+4=9, weight=1+3+3=7<=8
    assert_eq!(problem.evaluate(&[0, 1, 0, 1, 0, 1, 0, 0]), Min(Some(9)));
    // Path 0-1-4-5: length=2+6+2=10, weight=5+1+1=7<=8
    assert_eq!(problem.evaluate(&[1, 0, 0, 0, 0, 0, 1, 1]), Min(Some(10)));
    // Invalid: not a simple path
    assert_eq!(problem.evaluate(&[0, 1, 0, 1, 1, 1, 0, 0]), Min(None));
    // Path 0-1-3-2-4-5 is not simple s-t path structure in this encoding
    assert_eq!(problem.evaluate(&[1, 0, 0, 1, 0, 0, 1, 0]), Min(None));
    // Path 0-1-3-5: weight=5+2+3=10>8
    assert_eq!(problem.evaluate(&[1, 0, 1, 0, 0, 1, 0, 0]), Min(None));
    // Path 0-2-4-5: length=4+5+2=11, weight=1+2+1=4<=8
    assert_eq!(problem.evaluate(&[0, 1, 0, 0, 1, 0, 1, 0]), Min(Some(11)));
}

#[test]
fn test_shortest_weight_constrained_path_accessors() {
    let mut problem = issue_problem();
    problem.set_lengths(vec![1, 1, 1, 1, 1, 1, 1, 1]);
    problem.set_weights(vec![2, 2, 2, 2, 2, 2, 2, 2]);
    assert_eq!(problem.edge_lengths(), &[1, 1, 1, 1, 1, 1, 1, 1]);
    assert_eq!(problem.edge_weights(), &[2, 2, 2, 2, 2, 2, 2, 2]);
}

#[test]
fn test_shortest_weight_constrained_path_bruteforce() {
    let problem = issue_problem();
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem);
    assert!(solution.is_some());
    let config = solution.unwrap();
    // The witness should be the minimum-length feasible path (length 9)
    assert_eq!(problem.evaluate(&config), Min(Some(9)));

    let all = solver.find_all_witnesses(&problem);
    // All witnesses share the optimal value
    for c in &all {
        assert_eq!(problem.evaluate(c), Min(Some(9)));
    }
}

#[test]
fn test_shortest_weight_constrained_path_no_solution() {
    // weight_bound=3: no path has total weight <= 3
    let problem = ShortestWeightConstrainedPath::new(
        SimpleGraph::new(
            6,
            vec![
                (0, 1),
                (0, 2),
                (1, 3),
                (2, 3),
                (2, 4),
                (3, 5),
                (4, 5),
                (1, 4),
            ],
        ),
        vec![2, 4, 3, 1, 5, 4, 2, 6],
        vec![5, 1, 2, 3, 2, 3, 1, 1],
        0,
        5,
        3,
    );
    let solver = BruteForce::new();
    assert!(solver.find_witness(&problem).is_none());
}

#[test]
fn test_shortest_weight_constrained_path_serialization() {
    let problem = issue_problem();
    let json = serde_json::to_value(&problem).unwrap();
    let restored: ShortestWeightConstrainedPath<SimpleGraph, i32> =
        serde_json::from_value(json).unwrap();
    assert_eq!(restored.num_vertices(), 6);
    assert_eq!(restored.num_edges(), 8);
    assert_eq!(restored.source_vertex(), 0);
    assert_eq!(restored.target_vertex(), 5);
    assert_eq!(*restored.weight_bound(), 8);
}

#[test]
fn test_shortest_weight_constrained_path_problem_name() {
    assert_eq!(
        <ShortestWeightConstrainedPath<SimpleGraph, i32> as Problem>::NAME,
        "ShortestWeightConstrainedPath"
    );
}

#[test]
fn test_shortestweightconstrainedpath_paper_example() {
    let problem = issue_problem();
    assert_eq!(problem.evaluate(&[0, 1, 0, 1, 0, 1, 0, 0]), Min(Some(9)));

    let all = BruteForce::new().find_all_witnesses(&problem);
    // Only 1 witness at optimal value 9 (path 0-2-3-5)
    assert_eq!(all.len(), 1);
}

#[test]
fn test_shortest_weight_constrained_path_rejects_invalid_configs() {
    let problem = issue_problem();

    assert_eq!(problem.is_valid_solution(&[0, 1]), None);
    assert_eq!(problem.is_valid_solution(&[0, 1, 0, 1, 0, 1, 0, 2]), None);
    assert_eq!(problem.is_valid_solution(&[0, 0, 0, 0, 0, 0, 0, 0]), None);
}

#[test]
fn test_shortest_weight_constrained_path_source_equals_target_allows_only_empty_path() {
    let problem = ShortestWeightConstrainedPath::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        vec![3, 4],
        vec![2, 5],
        1,
        1,
        1,
    );

    assert_eq!(problem.is_valid_solution(&[0, 0]), Some(0));
    assert_eq!(problem.is_valid_solution(&[1, 0]), None);
}

#[test]
fn test_shortest_weight_constrained_path_exceeds_weight_bound() {
    // Path 0-1 with weight 5 > weight_bound 3
    let problem = ShortestWeightConstrainedPath::new(
        SimpleGraph::new(2, vec![(0, 1)]),
        vec![1],
        vec![5],
        0,
        1,
        3,
    );
    // Valid path but weight 5 > 3
    assert_eq!(problem.is_valid_solution(&[1]), None);
    assert_eq!(problem.evaluate(&[1]), Min(None));
}

#[test]
fn test_shortest_weight_constrained_path_rejects_disconnected_selected_edges() {
    let problem = ShortestWeightConstrainedPath::new(
        SimpleGraph::new(6, vec![(0, 1), (1, 2), (3, 4), (4, 5), (5, 3)]),
        vec![1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1],
        0,
        2,
        10,
    );

    assert_eq!(problem.is_valid_solution(&[1, 1, 1, 1, 1]), None);
}

#[test]
#[should_panic(expected = "All edge lengths must be positive (> 0)")]
fn test_shortest_weight_constrained_path_rejects_non_positive_edge_lengths() {
    ShortestWeightConstrainedPath::new(
        SimpleGraph::new(2, vec![(0, 1)]),
        vec![0],
        vec![1],
        0,
        1,
        1,
    );
}

#[test]
#[should_panic(expected = "weight_bound must be positive (> 0)")]
fn test_shortest_weight_constrained_path_rejects_non_positive_bounds() {
    ShortestWeightConstrainedPath::new(
        SimpleGraph::new(2, vec![(0, 1)]),
        vec![1],
        vec![1],
        0,
        1,
        0,
    );
}
