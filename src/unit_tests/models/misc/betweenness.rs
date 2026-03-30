use crate::models::misc::Betweenness;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Or;

fn example_problem() -> Betweenness {
    Betweenness::new(5, vec![(0, 1, 2), (2, 3, 4), (0, 2, 4), (1, 3, 4)])
}

#[test]
fn test_betweenness_basic() {
    let problem = example_problem();
    assert_eq!(problem.num_elements(), 5);
    assert_eq!(problem.num_triples(), 4);
    assert_eq!(
        problem.triples(),
        &[(0, 1, 2), (2, 3, 4), (0, 2, 4), (1, 3, 4)]
    );
    assert_eq!(problem.dims(), vec![5; 5]);
    assert_eq!(problem.num_variables(), 5);
    assert_eq!(<Betweenness as Problem>::NAME, "Betweenness");
    assert_eq!(<Betweenness as Problem>::variant(), vec![]);
}

#[test]
fn test_betweenness_evaluate_identity_permutation() {
    let problem = example_problem();
    // Identity permutation: element i is at position i
    assert_eq!(problem.evaluate(&[0, 1, 2, 3, 4]), Or(true));
}

#[test]
fn test_betweenness_evaluate_reverse_permutation() {
    let problem = example_problem();
    // Reverse permutation: element i is at position 4-i
    assert_eq!(problem.evaluate(&[4, 3, 2, 1, 0]), Or(true));
}

#[test]
fn test_betweenness_evaluate_invalid_permutation() {
    let problem = example_problem();
    // Not a permutation (duplicate positions)
    assert_eq!(problem.evaluate(&[0, 0, 1, 2, 3]), Or(false));
    // Position out of range
    assert_eq!(problem.evaluate(&[0, 1, 2, 3, 5]), Or(false));
    // Wrong length
    assert_eq!(problem.evaluate(&[0, 1, 2]), Or(false));
}

#[test]
fn test_betweenness_evaluate_unsatisfying_permutation() {
    let problem = example_problem();
    // Permutation [1, 0, 2, 3, 4]: triple (0,1,2) => f(0)=1, f(1)=0, f(2)=2
    // Need f(0)<f(1)<f(2) or f(2)<f(1)<f(0), i.e., 1<0<2 or 2<0<1 — neither holds
    assert_eq!(problem.evaluate(&[1, 0, 2, 3, 4]), Or(false));
}

#[test]
fn test_betweenness_solver_finds_witness() {
    let problem = example_problem();
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem).unwrap();
    assert_eq!(problem.evaluate(&solution), Or(true));
}

#[test]
fn test_betweenness_unsatisfiable_instance() {
    // Triples (0,1,2) and (1,0,2): first requires 1 between 0 and 2,
    // second requires 0 between 1 and 2. Together with (0,2,1) which
    // requires 2 between 0 and 1, these are contradictory for 3 elements.
    let problem = Betweenness::new(3, vec![(0, 1, 2), (1, 0, 2), (0, 2, 1)]);
    let solver = BruteForce::new();
    assert!(solver.find_witness(&problem).is_none());
}

#[test]
fn test_betweenness_serialization_round_trip() {
    let problem = example_problem();
    let json = serde_json::to_value(&problem).unwrap();
    assert_eq!(
        json,
        serde_json::json!({
            "num_elements": 5,
            "triples": [[0, 1, 2], [2, 3, 4], [0, 2, 4], [1, 3, 4]],
        })
    );

    let restored: Betweenness = serde_json::from_value(json).unwrap();
    assert_eq!(restored.num_elements(), problem.num_elements());
    assert_eq!(restored.triples(), problem.triples());
}

#[test]
fn test_betweenness_deserialization_rejects_invalid() {
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
        assert!(serde_json::from_value::<Betweenness>(invalid).is_err());
    }
}

#[test]
#[should_panic(expected = "at least one element")]
fn test_betweenness_zero_elements_panics() {
    Betweenness::new(0, vec![]);
}

#[test]
#[should_panic(expected = "out of range")]
fn test_betweenness_element_out_of_range_panics() {
    Betweenness::new(3, vec![(0, 1, 5)]);
}

#[test]
#[should_panic(expected = "duplicate elements")]
fn test_betweenness_duplicate_in_triple_panics() {
    Betweenness::new(3, vec![(0, 0, 1)]);
}
