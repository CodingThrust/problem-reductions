use crate::models::misc::SubsetProduct;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_subsetproduct_basic() {
    let problem = SubsetProduct::new(vec![2, 3, 5, 7], 30);
    assert_eq!(problem.num_elements(), 4);
    assert_eq!(problem.values(), &[2, 3, 5, 7]);
    assert_eq!(problem.target(), 30);
    assert_eq!(problem.dims(), vec![2; 4]);
    assert_eq!(<SubsetProduct as Problem>::NAME, "SubsetProduct");
    assert_eq!(<SubsetProduct as Problem>::variant(), vec![]);
}

#[test]
fn test_subsetproduct_evaluate_satisfying() {
    let problem = SubsetProduct::new(vec![2, 3, 5, 6], 30);
    assert!(problem.evaluate(&[1, 1, 1, 0]));
    assert!(problem.evaluate(&[0, 0, 1, 1]));
}

#[test]
fn test_subsetproduct_evaluate_unsatisfying() {
    let problem = SubsetProduct::new(vec![2, 3, 5, 7], 30);
    assert!(!problem.evaluate(&[0, 0, 0, 0]));
    assert!(!problem.evaluate(&[1, 1, 0, 0]));
    assert!(!problem.evaluate(&[1, 1, 1, 1]));
}

#[test]
fn test_subsetproduct_rejects_invalid_configs() {
    let problem = SubsetProduct::new(vec![2, 3, 5], 30);
    assert!(!problem.evaluate(&[1, 0]));
    assert!(!problem.evaluate(&[1, 0, 0, 0]));
    assert!(!problem.evaluate(&[2, 0, 0]));
}

#[test]
fn test_subsetproduct_overflow_returns_false() {
    let problem = SubsetProduct::new(vec![u64::MAX, 2], u64::MAX);
    assert!(problem.evaluate(&[1, 0]));
    assert!(!problem.evaluate(&[1, 1]));
}

#[test]
fn test_subsetproduct_bruteforce_finds_witness() {
    let problem = SubsetProduct::new(vec![2, 3, 5, 7], 35);
    let solution = BruteForce::new()
        .find_witness(&problem)
        .expect("expected a satisfying subset");
    assert!(problem.evaluate(&solution));
}

#[test]
fn test_subsetproduct_serialization() {
    let problem = SubsetProduct::new(vec![2, 3, 5], 30);
    let json = serde_json::to_value(&problem).unwrap();
    assert_eq!(
        json,
        serde_json::json!({
            "values": [2, 3, 5],
            "target": 30,
        })
    );

    let restored: SubsetProduct = serde_json::from_value(json).unwrap();
    assert_eq!(restored.values(), problem.values());
    assert_eq!(restored.target(), problem.target());
}

#[test]
#[should_panic(expected = "positive")]
fn test_subsetproduct_zero_value_panics() {
    SubsetProduct::new(vec![2, 0, 5], 10);
}
