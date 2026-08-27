use super::*;
use crate::solvers::BruteForce;
use crate::solvers::BruteForceProblem as _;
use crate::traits::Problem;
use crate::types::Or;

fn example_problem(k: i64) -> KthLargestMTuple {
    // m=3, X_1={2,5,8}, X_2={3,6}, X_3={1,4,7}, B=12
    KthLargestMTuple::new(vec![vec![2, 5, 8], vec![3, 6], vec![1, 4, 7]], k, 12)
}

#[test]
fn test_kth_largest_m_tuple_create_spec_uses_subsets_input() {
    assert_eq!(KthLargestMTupleCreateSpec::FIELDS[0].name, "subsets");
    let problem = KthLargestMTuple::try_from(KthLargestMTupleCreateSpec {
        subsets: vec![vec![1], vec![2]],
        k: 1,
        bound: 3,
    })
    .unwrap();
    assert_eq!(problem.sets(), &[vec![1], vec![2]]);
}

#[test]
fn test_kth_largest_m_tuple_creation() {
    let p = example_problem(14);
    assert_eq!(p.sets().len(), 3);
    assert_eq!(p.sets()[0], vec![2, 5, 8]);
    assert_eq!(p.sets()[1], vec![3, 6]);
    assert_eq!(p.sets()[2], vec![1, 4, 7]);
    assert_eq!(p.k(), 14);
    assert_eq!(p.bound(), 12);
    assert_eq!(p.num_sets(), 3);
    assert_eq!(p.total_tuples(), 18);
    assert_eq!(p.dimensions(), Vec::<usize>::new());
    assert_eq!(p.num_variables(), 0);
    assert_eq!(<KthLargestMTuple as Problem>::NAME, "KthLargestMTuple");
    assert_eq!(<KthLargestMTuple as Problem>::variant(), vec![]);
}

#[test]
fn test_kth_largest_m_tuple_threshold_decision() {
    let p = example_problem(14);
    assert_eq!(
        p.evaluate(&BruteForce::new().solve(&p).unwrap().unwrap())
            .unwrap(),
        Or(true)
    );

    let above_threshold = example_problem(15);
    assert!(BruteForce::new().solve(&above_threshold).unwrap().is_none());
}

#[test]
fn test_kth_largest_m_tuple_evaluate_invalid_configs() {
    let p = example_problem(14);
    assert!(crate::registry::DynProblem::evaluate_dyn(&p, &serde_json::json!([0])).is_err());
    assert!(crate::registry::DynProblem::evaluate_dyn(&p, &serde_json::json!([2, 1, 2])).is_err());
}

#[test]
fn test_kth_largest_m_tuple_serialization_round_trip() {
    let p = example_problem(14);
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
    let p = example_problem(14);
    let solver = BruteForce::new();
    assert_eq!(
        p.evaluate(&solver.solve(&p).unwrap().unwrap()).unwrap(),
        Or(true)
    );
}

#[test]
fn test_kth_largest_m_tuple_all_qualify() {
    // Two sets each with one large element, B=1 -> all tuples qualify
    let p = KthLargestMTuple::new(vec![vec![5], vec![10]], 1, 1);
    let solver = BruteForce::new();
    assert_eq!(
        p.evaluate(&solver.solve(&p).unwrap().unwrap()).unwrap(),
        Or(true)
    );
    assert_eq!(p.total_tuples(), 1);
}

#[test]
fn test_kth_largest_m_tuple_none_qualify() {
    // B is larger than any possible sum
    let p = KthLargestMTuple::new(vec![vec![1, 2], vec![1, 2]], 1, 100);
    let solver = BruteForce::new();
    assert!(solver.solve(&p).unwrap().is_none());
}

#[test]
fn test_kth_largest_m_tuple_reports_sum_beyond_i64_max() {
    let p = KthLargestMTuple::new(vec![vec![i64::MAX], vec![1]], 1, i64::MAX);
    assert!(matches!(
        BruteForce::new().solve(&p),
        Err(crate::solvers::SolveError::Evaluation(
            crate::traits::EvaluationError::IntegerOverflow(_)
        ))
    ));
}

#[test]
fn test_kth_largest_m_tuple_many_singleton_sets_do_not_use_call_stack() {
    let p = KthLargestMTuple::new(vec![vec![1]; 10_000], 1, 10_000);
    assert_eq!(
        p.evaluate(&BruteForce::new().solve(&p).unwrap().unwrap())
            .unwrap(),
        Or(true)
    );
}

#[test]
#[should_panic(expected = "total tuple count exceeds usize")]
fn test_kth_largest_m_tuple_total_tuples_overflow_panics() {
    let p = KthLargestMTuple::new(vec![vec![1, 2]; usize::BITS as usize], 1, 1);
    p.total_tuples();
}
