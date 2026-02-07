use super::*;

#[test]
fn test_graph_test_case() {
    let case = GraphTestCase::new(3, vec![(0, 1), (1, 2)], vec![1, 0, 1], 2);
    assert_eq!(case.num_vertices, 3);
    assert_eq!(case.edges.len(), 2);
    assert!(case.weights.is_none());
    assert!(case.optimal_size.is_none());
}

#[test]
fn test_graph_test_case_with_weights() {
    let case = GraphTestCase::with_weights(3, vec![(0, 1)], vec![1, 2, 3], vec![0, 0, 1], 3);
    assert!(case.weights.is_some());
    assert_eq!(case.weights.as_ref().unwrap(), &vec![1, 2, 3]);
}

#[test]
fn test_graph_test_case_with_optimal() {
    let case = GraphTestCase::new(3, vec![(0, 1)], vec![0, 0, 1], 1).with_optimal(2);
    assert_eq!(case.optimal_size, Some(2));
}

#[test]
fn test_sat_test_case_satisfiable() {
    let case = SatTestCase::satisfiable(2, vec![vec![1, 2], vec![-1]], vec![0, 1]);
    assert!(case.is_satisfiable);
    assert!(case.satisfying_assignment.is_some());
}

#[test]
fn test_sat_test_case_unsatisfiable() {
    let case = SatTestCase::unsatisfiable(1, vec![vec![1], vec![-1]]);
    assert!(!case.is_satisfiable);
    assert!(case.satisfying_assignment.is_none());
}
