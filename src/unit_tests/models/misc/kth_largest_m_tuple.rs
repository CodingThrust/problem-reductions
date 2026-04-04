use crate::models::misc::KthLargestMTuple;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;
use crate::types::Sum;

fn example_problem() -> KthLargestMTuple {
    // m=3, X_1={2,5,8}, X_2={3,6}, X_3={1,4,7}, B=12, K=14
    KthLargestMTuple::new(vec![vec![2, 5, 8], vec![3, 6], vec![1, 4, 7]], 14, 12)
}

#[test]
fn test_kth_largest_m_tuple_creation() {
    let p = example_problem();
    assert_eq!(p.sets().len(), 3);
    assert_eq!(p.sets()[0], vec![2, 5, 8]);
    assert_eq!(p.sets()[1], vec![3, 6]);
    assert_eq!(p.sets()[2], vec![1, 4, 7]);
    assert_eq!(p.k(), 14);
    assert_eq!(p.bound(), 12);
    assert_eq!(p.num_sets(), 3);
    assert_eq!(p.total_tuples(), 18);
    assert_eq!(p.dims(), vec![3, 2, 3]);
    assert_eq!(p.num_variables(), 3);
    assert_eq!(<KthLargestMTuple as Problem>::NAME, "KthLargestMTuple");
    assert_eq!(<KthLargestMTuple as Problem>::variant(), vec![]);
}

#[test]
fn test_kth_largest_m_tuple_evaluate_qualifying_tuple() {
    let p = example_problem();
    // (8,6,7) = sum 21 >= 12 -> Sum(1)
    assert_eq!(p.evaluate(&[2, 1, 2]), Sum(1));
    // (5,6,4) = sum 15 >= 12 -> Sum(1)
    assert_eq!(p.evaluate(&[1, 1, 1]), Sum(1));
}

#[test]
fn test_kth_largest_m_tuple_evaluate_non_qualifying_tuple() {
    let p = example_problem();
    // (2,3,1) = sum 6 < 12 -> Sum(0)
    assert_eq!(p.evaluate(&[0, 0, 0]), Sum(0));
    // (2,3,4) = sum 9 < 12 -> Sum(0)
    assert_eq!(p.evaluate(&[0, 0, 1]), Sum(0));
}

#[test]
fn test_kth_largest_m_tuple_evaluate_invalid_configs() {
    let p = example_problem();
    // Wrong length
    assert_eq!(p.evaluate(&[0, 0]), Sum(0));
    assert_eq!(p.evaluate(&[0, 0, 0, 0]), Sum(0));
    // Out of range
    assert_eq!(p.evaluate(&[3, 0, 0]), Sum(0));
    assert_eq!(p.evaluate(&[0, 2, 0]), Sum(0));
    assert_eq!(p.evaluate(&[0, 0, 3]), Sum(0));
}

#[test]
fn test_kth_largest_m_tuple_solver() {
    let p = example_problem();
    let solver = BruteForce::new();
    let value = solver.solve(&p);
    // 14 of 18 tuples qualify (sum >= 12)
    assert_eq!(value, Sum(14));
}

#[test]
fn test_kth_largest_m_tuple_boundary_example() {
    // K=14 and count=14, so the answer is YES (count >= K)
    let p = example_problem();
    let solver = BruteForce::new();
    let count = solver.solve(&p);
    assert_eq!(count, Sum(14));
    assert!(count.0 >= p.k());
}

#[test]
fn test_kth_largest_m_tuple_serialization_round_trip() {
    let p = example_problem();
    let json = serde_json::to_value(&p).unwrap();
    assert_eq!(
        json,
        serde_json::json!({
            "sets": [[2, 5, 8], [3, 6], [1, 4, 7]],
            "k": 14,
            "bound": 12,
        })
    );

    let restored: KthLargestMTuple = serde_json::from_value(json).unwrap();
    assert_eq!(restored.sets(), p.sets());
    assert_eq!(restored.k(), p.k());
    assert_eq!(restored.bound(), p.bound());
}

#[test]
fn test_kth_largest_m_tuple_deserialization_rejects_invalid() {
    let invalid_cases = [
        // Empty sets
        serde_json::json!({ "sets": [], "k": 1, "bound": 5 }),
        // A set is empty
        serde_json::json!({ "sets": [[1, 2], []], "k": 1, "bound": 3 }),
        // Zero size
        serde_json::json!({ "sets": [[0, 2]], "k": 1, "bound": 1 }),
        // K=0
        serde_json::json!({ "sets": [[1, 2]], "k": 0, "bound": 1 }),
        // Bound=0
        serde_json::json!({ "sets": [[1, 2]], "k": 1, "bound": 0 }),
    ];

    for invalid in invalid_cases {
        assert!(serde_json::from_value::<KthLargestMTuple>(invalid).is_err());
    }
}

#[test]
#[should_panic(expected = "at least one set")]
fn test_kth_largest_m_tuple_empty_sets_panics() {
    KthLargestMTuple::new(vec![], 1, 5);
}

#[test]
#[should_panic(expected = "non-empty")]
fn test_kth_largest_m_tuple_empty_inner_set_panics() {
    KthLargestMTuple::new(vec![vec![1, 2], vec![]], 1, 3);
}

#[test]
#[should_panic(expected = "positive")]
fn test_kth_largest_m_tuple_zero_size_panics() {
    KthLargestMTuple::new(vec![vec![0, 2]], 1, 1);
}

#[test]
fn test_kth_largest_m_tuple_paper_example() {
    // Issue example: m=3, X_1={2,5,8}, X_2={3,6}, X_3={1,4,7}, B=12, K=14
    // 14 of 18 tuples have sum >= 12 -> YES (boundary case: count == K)
    let p = example_problem();
    let solver = BruteForce::new();
    let count = solver.solve(&p);
    assert_eq!(count, Sum(14));

    // Verify a specific qualifying tuple: (8,6,7), sum=21
    assert_eq!(p.evaluate(&[2, 1, 2]), Sum(1));

    // Verify a specific non-qualifying tuple: (2,3,1), sum=6
    assert_eq!(p.evaluate(&[0, 0, 0]), Sum(0));
}

#[test]
fn test_kth_largest_m_tuple_all_qualify() {
    // Two sets each with one large element, B=1 -> all tuples qualify
    let p = KthLargestMTuple::new(vec![vec![5], vec![10]], 1, 1);
    let solver = BruteForce::new();
    assert_eq!(solver.solve(&p), Sum(1));
    assert_eq!(p.total_tuples(), 1);
}

#[test]
fn test_kth_largest_m_tuple_none_qualify() {
    // B is larger than any possible sum
    let p = KthLargestMTuple::new(vec![vec![1, 2], vec![1, 2]], 1, 100);
    let solver = BruteForce::new();
    assert_eq!(solver.solve(&p), Sum(0));
}
