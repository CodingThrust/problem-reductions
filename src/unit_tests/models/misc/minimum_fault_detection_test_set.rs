use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Min;

fn issue_problem() -> MinimumFaultDetectionTestSet {
    // 7 vertices, inputs={0,1}, outputs={5,6}
    // Arcs: (0,2),(0,3),(1,3),(1,4),(2,5),(3,5),(3,6),(4,6)
    MinimumFaultDetectionTestSet::new(
        7,
        vec![
            (0, 2),
            (0, 3),
            (1, 3),
            (1, 4),
            (2, 5),
            (3, 5),
            (3, 6),
            (4, 6),
        ],
        vec![0, 1],
        vec![5, 6],
    )
}

#[test]
fn test_minimum_fault_detection_test_set_creation() {
    let problem = issue_problem();

    assert_eq!(problem.num_vertices(), 7);
    assert_eq!(problem.num_arcs(), 8);
    assert_eq!(problem.inputs(), &[0, 1]);
    assert_eq!(problem.outputs(), &[5, 6]);
    assert_eq!(problem.num_inputs(), 2);
    assert_eq!(problem.num_outputs(), 2);
    // 2 inputs * 2 outputs = 4 pairs
    assert_eq!(problem.num_variables(), 4);
    assert_eq!(problem.dims(), vec![2; 4]);
    assert_eq!(
        <MinimumFaultDetectionTestSet as Problem>::NAME,
        "MinimumFaultDetectionTestSet"
    );
    assert!(<MinimumFaultDetectionTestSet as Problem>::variant().is_empty());
}

#[test]
fn test_minimum_fault_detection_test_set_evaluate_optimal() {
    let problem = issue_problem();

    // Config [1,0,0,1]: select pairs (0,5) and (1,6)
    // (0,5) covers {0,2,3,5}, (1,6) covers {1,3,4,6}
    // Union = {0,1,2,3,4,5,6} = all 7 vertices -> Min(2)
    assert_eq!(problem.evaluate(&[1, 0, 0, 1]), Min(Some(2)));
}

#[test]
fn test_minimum_fault_detection_test_set_evaluate_insufficient() {
    let problem = issue_problem();

    // Config [1,0,0,0]: select only pair (0,5)
    // (0,5) covers {0,2,3,5} -> missing {1,4,6} -> Min(None)
    assert_eq!(problem.evaluate(&[1, 0, 0, 0]), Min(None));

    // Config [0,0,0,1]: select only pair (1,6)
    // (1,6) covers {1,3,4,6} -> missing {0,2,5} -> Min(None)
    assert_eq!(problem.evaluate(&[0, 0, 0, 1]), Min(None));
}

#[test]
fn test_minimum_fault_detection_test_set_evaluate_all_pairs() {
    let problem = issue_problem();

    // Config [1,1,1,1]: select all 4 pairs
    // Union covers all vertices -> Min(4)
    assert_eq!(problem.evaluate(&[1, 1, 1, 1]), Min(Some(4)));
}

#[test]
fn test_minimum_fault_detection_test_set_evaluate_no_selection() {
    let problem = issue_problem();

    // No pairs selected -> nothing covered -> Min(None)
    assert_eq!(problem.evaluate(&[0, 0, 0, 0]), Min(None));
}

#[test]
fn test_minimum_fault_detection_test_set_wrong_config_length() {
    let problem = issue_problem();

    assert_eq!(problem.evaluate(&[1, 0]), Min(None));
}

#[test]
fn test_minimum_fault_detection_test_set_solver() {
    let problem = issue_problem();
    let solver = BruteForce::new();

    use crate::solvers::Solver;
    let optimal = solver.solve(&problem);
    assert_eq!(optimal, Min(Some(2)));

    let witness = solver.find_witness(&problem);
    assert!(witness.is_some());
    let w = witness.unwrap();
    assert_eq!(problem.evaluate(&w), Min(Some(2)));
}

#[test]
fn test_minimum_fault_detection_test_set_serialization() {
    let problem = issue_problem();
    let json = serde_json::to_string(&problem).unwrap();
    let round_trip: MinimumFaultDetectionTestSet = serde_json::from_str(&json).unwrap();

    assert_eq!(round_trip.num_vertices(), 7);
    assert_eq!(round_trip.num_arcs(), 8);
    assert_eq!(round_trip.inputs(), &[0, 1]);
    assert_eq!(round_trip.outputs(), &[5, 6]);
    assert_eq!(round_trip.evaluate(&[1, 0, 0, 1]), Min(Some(2)));
}

#[test]
fn test_minimum_fault_detection_test_set_paper_example() {
    let problem = issue_problem();

    // Verify the paper example: optimal config [1,0,0,1] with value 2
    assert_eq!(problem.evaluate(&[1, 0, 0, 1]), Min(Some(2)));

    // Confirm optimality via brute force
    let solver = BruteForce::new();
    use crate::solvers::Solver;
    let optimal = solver.solve(&problem);
    assert_eq!(optimal, Min(Some(2)));

    // Verify there is exactly one optimal witness
    let all = solver.find_all_witnesses(&problem);
    let optimal_witnesses: Vec<_> = all
        .into_iter()
        .filter(|w| problem.evaluate(w) == Min(Some(2)))
        .collect();
    assert_eq!(optimal_witnesses.len(), 1);
    assert_eq!(optimal_witnesses[0], vec![1, 0, 0, 1]);
}
