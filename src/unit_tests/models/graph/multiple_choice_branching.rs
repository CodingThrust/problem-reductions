use super::*;
use crate::solvers::BruteForce;
use crate::topology::DirectedGraph;
use crate::traits::Problem;
use serde_json;

fn yes_instance() -> MultipleChoiceBranching<i32> {
    MultipleChoiceBranching::new(
        DirectedGraph::new(
            6,
            vec![
                (0, 1),
                (0, 2),
                (1, 3),
                (2, 3),
                (1, 4),
                (3, 5),
                (4, 5),
                (2, 4),
            ],
        ),
        vec![3, 2, 4, 1, 2, 3, 1, 3],
        vec![vec![0, 1], vec![2, 3], vec![4, 7], vec![5, 6]],
        10,
    )
}

fn no_instance() -> MultipleChoiceBranching<i32> {
    MultipleChoiceBranching::new(
        DirectedGraph::new(3, vec![(0, 1), (1, 2)]),
        vec![2, 2],
        vec![vec![0], vec![1]],
        5,
    )
}

#[test]
fn test_multiple_choice_branching_creation_and_accessors() {
    let mut problem = yes_instance();

    assert_eq!(problem.num_vertices(), 6);
    assert_eq!(problem.num_arcs(), 8);
    assert_eq!(problem.num_partition_groups(), 4);
    assert_eq!(problem.dims(), vec![2; 8]);
    assert_eq!(problem.graph().arcs().len(), 8);
    assert_eq!(problem.weights(), &[3, 2, 4, 1, 2, 3, 1, 3]);
    assert_eq!(
        problem.partition(),
        &[vec![0, 1], vec![2, 3], vec![4, 7], vec![5, 6]]
    );
    assert_eq!(problem.threshold(), &10);
    assert!(problem.is_weighted());

    problem.set_weights(vec![1; 8]);
    assert_eq!(problem.weights(), &[1, 1, 1, 1, 1, 1, 1, 1]);
}

#[test]
fn test_multiple_choice_branching_rejects_weight_length_mismatch() {
    let result = std::panic::catch_unwind(|| {
        MultipleChoiceBranching::new(
            DirectedGraph::new(2, vec![(0, 1)]),
            vec![1, 2],
            vec![vec![0]],
            1,
        )
    });
    assert!(result.is_err());
}

#[test]
fn test_multiple_choice_branching_partition_validation_out_of_range() {
    let result = std::panic::catch_unwind(|| {
        MultipleChoiceBranching::new(
            DirectedGraph::new(2, vec![(0, 1)]),
            vec![1],
            vec![vec![1]],
            1,
        )
    });
    assert!(result.is_err());
}

#[test]
fn test_multiple_choice_branching_partition_validation_overlap() {
    let result = std::panic::catch_unwind(|| {
        MultipleChoiceBranching::new(
            DirectedGraph::new(3, vec![(0, 1), (1, 2)]),
            vec![1, 1],
            vec![vec![0, 1], vec![1]],
            1,
        )
    });
    assert!(result.is_err());
}

#[test]
fn test_multiple_choice_branching_partition_validation_missing_arc() {
    let result = std::panic::catch_unwind(|| {
        MultipleChoiceBranching::new(
            DirectedGraph::new(3, vec![(0, 1), (1, 2)]),
            vec![1, 1],
            vec![vec![0]],
            1,
        )
    });
    assert!(result.is_err());
}

#[test]
fn test_multiple_choice_branching_evaluate_yes_instance() {
    let problem = yes_instance();
    assert!(problem.evaluate(&[1, 0, 1, 0, 0, 1, 0, 1]));
    assert!(problem.is_valid_solution(&[1, 0, 1, 0, 0, 1, 0, 1]));
}

#[test]
fn test_multiple_choice_branching_rejects_partition_violation() {
    let problem = yes_instance();
    assert!(!problem.evaluate(&[1, 1, 0, 0, 0, 0, 0, 0]));
}

#[test]
fn test_multiple_choice_branching_rejects_wrong_config_length() {
    let problem = yes_instance();
    assert!(!problem.evaluate(&[1, 0, 1]));
}

#[test]
fn test_multiple_choice_branching_rejects_non_binary_config_value() {
    let problem = yes_instance();
    assert!(!problem.evaluate(&[2, 0, 1, 0, 0, 1, 0, 1]));
}

#[test]
fn test_multiple_choice_branching_rejects_indegree_violation() {
    let problem = MultipleChoiceBranching::new(
        DirectedGraph::new(3, vec![(0, 2), (1, 2)]),
        vec![2, 2],
        vec![vec![0], vec![1]],
        1,
    );
    assert!(!problem.evaluate(&[1, 1]));
}

#[test]
fn test_multiple_choice_branching_rejects_cycle_violation() {
    let problem = MultipleChoiceBranching::new(
        DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]),
        vec![1, 1, 1],
        vec![vec![0], vec![1], vec![2]],
        1,
    );
    assert!(!problem.evaluate(&[1, 1, 1]));
}

#[test]
fn test_multiple_choice_branching_rejects_threshold_violation() {
    let problem = yes_instance();
    assert!(!problem.evaluate(&[1, 0, 1, 0, 0, 0, 0, 0]));
}

#[test]
fn test_multiple_choice_branching_solver_issue_examples() {
    let yes_problem = yes_instance();
    let solver = BruteForce::new();

    let solution = solver.find_witness(&yes_problem);
    assert!(solution.is_some());
    assert!(yes_problem.evaluate(&solution.unwrap()));

    let all_solutions = solver.find_all_witnesses(&yes_problem);
    assert!(!all_solutions.is_empty());
    assert!(all_solutions.contains(&vec![1, 0, 1, 0, 0, 1, 0, 1]));
    for config in &all_solutions {
        assert!(yes_problem.evaluate(config));
    }

    let no_problem = no_instance();
    assert!(solver.find_witness(&no_problem).is_none());
}

#[test]
fn test_multiple_choice_branching_paper_example() {
    let problem = yes_instance();
    let config = vec![1, 0, 1, 0, 0, 1, 0, 1];

    assert!(problem.evaluate(&config));

    let all_solutions = BruteForce::new().find_all_witnesses(&problem);
    assert_eq!(all_solutions.len(), 11);
    assert!(all_solutions.contains(&config));
}

#[test]
fn test_multiple_choice_branching_serialization() {
    let problem = yes_instance();
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: MultipleChoiceBranching<i32> = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.num_vertices(), 6);
    assert_eq!(deserialized.num_arcs(), 8);
    assert_eq!(deserialized.threshold(), &10);
}

#[test]
fn test_multiple_choice_branching_deserialize_rejects_weight_length_mismatch() {
    let json = r#"{
        "graph": {"num_vertices": 2, "arcs": [[0, 1]]},
        "weights": [1, 2],
        "partition": [[0]],
        "threshold": 1
    }"#;
    let result: Result<MultipleChoiceBranching<i32>, _> = serde_json::from_str(json);
    let err = result.unwrap_err().to_string();
    assert!(err.contains("weights length must match"), "got: {err}");
}

#[test]
fn test_multiple_choice_branching_deserialize_rejects_invalid_partition() {
    let json = r#"{
        "graph": {"num_vertices": 2, "arcs": [[0, 1]]},
        "weights": [1],
        "partition": [[1]],
        "threshold": 1
    }"#;
    let result: Result<MultipleChoiceBranching<i32>, _> = serde_json::from_str(json);
    let err = result.unwrap_err().to_string();
    assert!(err.contains("partition"), "got: {err}");
}

#[test]
fn test_multiple_choice_branching_set_weights_rejects_wrong_length() {
    let result = std::panic::catch_unwind(|| {
        let mut problem = MultipleChoiceBranching::new(
            DirectedGraph::new(3, vec![(0, 1), (1, 2)]),
            vec![1, 1],
            vec![vec![0], vec![1]],
            1,
        );
        problem.set_weights(vec![1, 2, 3]);
    });
    assert!(result.is_err());
}

#[test]
fn test_multiple_choice_branching_num_variables() {
    let problem = yes_instance();
    assert_eq!(problem.num_variables(), 8);
}
