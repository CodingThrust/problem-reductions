use super::*;
use crate::solvers::BruteForce;
use crate::solvers::BruteForceProblem as _;
use crate::topology::DirectedGraph;
use crate::traits::Problem;
use crate::types::Or;

/// Build the canonical YES instance from issue #1024.
///
/// `V = {0, 1, 2}` and `A = [(0, 1), (0, 1), (1, 2), (2, 0)]`, with parallel
/// arcs `a_0` and `a_1` between vertices `0` and `1`. The witness ordering
/// `(a_0, a_2, a_3, a_1)` traces the directed trail `0 -> 1 -> 2 -> 0 -> 1`.
fn canonical_instance() -> EulerianPath {
    let graph = DirectedGraph::new(3, vec![(0, 1), (0, 1), (1, 2), (2, 0)]);
    EulerianPath::new(graph)
}

#[test]
fn test_eulerian_path_creation() {
    let problem = canonical_instance();
    assert_eq!(problem.num_vertices(), 3);
    assert_eq!(problem.num_arcs(), 4);
    // m = 4 position variables, each with domain {0..3}.
    assert_eq!(problem.dimensions(), vec![4, 4, 4, 4]);
    assert_eq!(problem.num_variables(), 4);
}

#[test]
fn test_eulerian_path_evaluate_valid_witness() {
    let problem = canonical_instance();
    // a_0 -> a_2 -> a_3 -> a_1 = (0->1)(1->2)(2->0)(0->1) -- a valid Eulerian trail.
    assert_eq!(problem.evaluate(&vec![0, 2, 3, 1]).unwrap(), Or(true));
    assert!(problem.is_valid_solution(&[0, 2, 3, 1]));
}

#[test]
fn test_eulerian_path_evaluate_not_permutation() {
    let problem = canonical_instance();
    // Arc 0 reused; arc 2 is missing -> not a permutation of {0..3}.
    assert_eq!(problem.evaluate(&vec![0, 0, 3, 1]).unwrap(), Or(false));
}

#[test]
fn test_eulerian_path_evaluate_bad_trail() {
    let problem = canonical_instance();
    // [0, 3, 2, 1]: arc 0 = (0->1), arc 3 = (2->0). head(0)=1 != tail(3)=2.
    assert_eq!(problem.evaluate(&vec![0, 3, 2, 1]).unwrap(), Or(false));
}

#[test]
fn test_eulerian_path_evaluate_out_of_range() {
    let problem = canonical_instance();
    // Value 4 is outside the domain {0..3}.
    assert!(matches!(
        problem.evaluate(&vec![0, 2, 3, 4]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
}

#[test]
fn test_eulerian_path_evaluate_wrong_length() {
    let problem = canonical_instance();
    // m = 4 but length 3.
    assert!(matches!(
        problem.evaluate(&vec![0, 2, 3]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
}

#[test]
fn test_eulerian_path_brute_force_yes_instance() {
    let problem = canonical_instance();
    let solver = BruteForce::new();
    let value_solution = solver.solve(&problem).unwrap().unwrap();
    let value = problem.evaluate(&value_solution).unwrap();
    assert_eq!(value, Or(true));

    let witness = solver.solve(&problem).unwrap().expect("yes-instance");
    assert_eq!(problem.evaluate(&witness).unwrap(), Or(true));

    let all = solver.find_all_witnesses(&problem).unwrap();
    assert!(!all.is_empty(), "expected at least one Eulerian witness");
    for w in &all {
        assert_eq!(problem.evaluate(w).unwrap(), Or(true));
    }
}

#[test]
fn test_eulerian_path_no_instance() {
    // Three parallel arcs (0,1) and one return arc (1,0).
    // outdeg(0) - indeg(0) = 3 - 1 = 2, breaks the degree-balance condition,
    // so no Eulerian trail exists.
    let graph = DirectedGraph::new(2, vec![(0, 1), (0, 1), (0, 1), (1, 0)]);
    let problem = EulerianPath::new(graph);
    let solver = BruteForce::new();
    assert!(solver.solve(&problem).unwrap().is_none());
    assert!(solver.solve(&problem).unwrap().is_none());
    assert!(solver.find_all_witnesses(&problem).unwrap().is_empty());
}

#[test]
fn test_eulerian_path_empty_arcs_instance() {
    // m = 0 (only isolated vertices): dims = [] and the empty witness is valid.
    let graph = DirectedGraph::new(3, vec![]);
    let problem = EulerianPath::new(graph);
    assert_eq!(problem.dimensions(), Vec::<usize>::new());
    assert_eq!(problem.num_variables(), 0);
    assert_eq!(problem.evaluate(&vec![]).unwrap(), Or(true));

    let solver = BruteForce::new();
    assert_eq!(
        problem
            .evaluate(&solver.solve(&problem).unwrap().unwrap())
            .unwrap(),
        Or(true)
    );
    let witness = solver.solve(&problem).unwrap().expect("empty witness");
    assert!(witness.is_empty());
}

#[test]
fn test_eulerian_path_serialization_roundtrip() {
    let problem = canonical_instance();
    let json = serde_json::to_value(&problem).unwrap();
    let restored: EulerianPath = serde_json::from_value(json).unwrap();
    assert_eq!(restored.num_vertices(), problem.num_vertices());
    assert_eq!(restored.num_arcs(), problem.num_arcs());
    assert_eq!(restored.graph().arcs(), problem.graph().arcs());
}

#[test]
fn test_eulerian_path_variant_and_name() {
    assert_eq!(EulerianPath::NAME, "EulerianPath");
    // No type parameters, so the variant tuple is empty.
    assert!(EulerianPath::variant().is_empty());
}
