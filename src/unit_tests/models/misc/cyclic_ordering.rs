use crate::models::misc::CyclicOrdering;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Or;

fn example_problem() -> CyclicOrdering {
    CyclicOrdering::new(5, vec![(0, 1, 2), (2, 3, 0), (1, 3, 4)])
}

#[test]
fn test_cyclic_ordering_basic() {
    let problem = example_problem();
    assert_eq!(problem.num_elements(), 5);
    assert_eq!(problem.num_triples(), 3);
    assert_eq!(problem.triples(), &[(0, 1, 2), (2, 3, 0), (1, 3, 4)]);
    assert_eq!(problem.dims(), vec![5; 5]);
    assert_eq!(problem.num_variables(), 5);
    assert_eq!(<CyclicOrdering as Problem>::NAME, "CyclicOrdering");
    assert_eq!(<CyclicOrdering as Problem>::variant(), vec![]);
}

#[test]
fn test_cyclic_ordering_evaluate_satisfying() {
    let problem = example_problem();
    // config = [1,3,4,0,2]: f(0)=1, f(1)=3, f(2)=4, f(3)=0, f(4)=2
    // (0,1,2): 1<3<4 ✓  (2,3,0): 0<1<4 (cyclic) ✓  (1,3,4): 0<2<3 (cyclic) ✓
    assert_eq!(problem.evaluate(&[1, 3, 4, 0, 2]), Or(true));
}

#[test]
fn test_cyclic_ordering_evaluate_unsatisfying() {
    let problem = example_problem();
    // Identity permutation [0,1,2,3,4]:
    // (0,1,2): 0<1<2 ✓  (2,3,0): f(2)=2, f(3)=3, f(0)=0 → need
    // (2<3<0) or (3<0<2) or (0<2<3). 0<2<3 ✓
    // (1,3,4): f(1)=1, f(3)=3, f(4)=4 → 1<3<4 ✓
    // Actually identity works! Let me pick one that doesn't.
    // [0,2,1,3,4]:
    // (0,1,2): f(0)=0, f(1)=2, f(2)=1 → (0<2<1)? no. (2<1<0)? no. (1<0<2)? no. → fails
    assert_eq!(problem.evaluate(&[0, 2, 1, 3, 4]), Or(false));
}

#[test]
fn test_cyclic_ordering_evaluate_invalid_permutation() {
    let problem = example_problem();
    // Not a permutation (duplicate positions)
    assert_eq!(problem.evaluate(&[0, 0, 1, 2, 3]), Or(false));
    // Position out of range
    assert_eq!(problem.evaluate(&[0, 1, 2, 3, 5]), Or(false));
    // Wrong length
    assert_eq!(problem.evaluate(&[0, 1, 2]), Or(false));
}

#[test]
fn test_cyclic_ordering_solver_finds_witness() {
    let problem = example_problem();
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem).unwrap();
    assert_eq!(problem.evaluate(&solution), Or(true));
}

#[test]
fn test_cyclic_ordering_unsatisfiable_instance() {
    // With 3 elements, triples (0,1,2) and (0,2,1):
    // (0,1,2) requires cyclic order a<b<c or b<c<a or c<a<b
    // (0,2,1) requires cyclic order a<c<b or c<b<a or b<a<c
    // These are opposite cyclic orientations, so unsatisfiable.
    let problem = CyclicOrdering::new(3, vec![(0, 1, 2), (0, 2, 1)]);
    let solver = BruteForce::new();
    assert!(solver.find_witness(&problem).is_none());
}

#[test]
fn test_cyclic_ordering_serialization_round_trip() {
    let problem = example_problem();
    let json = serde_json::to_value(&problem).unwrap();
    assert_eq!(
        json,
        serde_json::json!({
            "num_elements": 5,
            "triples": [[0, 1, 2], [2, 3, 0], [1, 3, 4]],
        })
    );

    let restored: CyclicOrdering = serde_json::from_value(json).unwrap();
    assert_eq!(restored.num_elements(), problem.num_elements());
    assert_eq!(restored.triples(), problem.triples());
}

#[test]
fn test_cyclic_ordering_deserialization_rejects_invalid() {
    let invalid_cases = [
        // Zero elements
        serde_json::json!({
            "num_elements": 0,
            "triples": [],
        }),
        // Element out of range
        serde_json::json!({
            "num_elements": 3,
            "triples": [[0, 1, 5]],
        }),
        // Duplicate elements in triple
        serde_json::json!({
            "num_elements": 3,
            "triples": [[0, 0, 1]],
        }),
    ];

    for invalid in invalid_cases {
        assert!(serde_json::from_value::<CyclicOrdering>(invalid).is_err());
    }
}

#[test]
#[should_panic(expected = "at least one element")]
fn test_cyclic_ordering_zero_elements_panics() {
    CyclicOrdering::new(0, vec![]);
}

#[test]
#[should_panic(expected = "out of range")]
fn test_cyclic_ordering_element_out_of_range_panics() {
    CyclicOrdering::new(3, vec![(0, 1, 5)]);
}

#[test]
#[should_panic(expected = "duplicate elements")]
fn test_cyclic_ordering_duplicate_in_triple_panics() {
    CyclicOrdering::new(3, vec![(0, 0, 1)]);
}
