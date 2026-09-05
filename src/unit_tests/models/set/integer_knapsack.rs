use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_integer_knapsack_basic() {
    let problem = IntegerKnapsack::new(vec![3, 4, 5, 2, 7], vec![4, 5, 7, 3, 9], 15).unwrap();
    assert_eq!(problem.num_items(), 5);
    assert_eq!(problem.sizes(), &[3, 4, 5, 2, 7]);
    assert_eq!(problem.values(), &[4, 5, 7, 3, 9]);
    assert_eq!(problem.capacity(), 15);
    // dims: floor(15/3)+1=6, floor(15/4)+1=4, floor(15/5)+1=4, floor(15/2)+1=8, floor(15/7)+1=3
    assert_eq!(problem.dimensions(), vec![6, 4, 4, 8, 3]);
    assert_eq!(<IntegerKnapsack as Problem>::NAME, "IntegerKnapsack");
    assert_eq!(<IntegerKnapsack as Problem>::variant(), vec![]);
}

#[test]
fn test_integer_knapsack_evaluate_optimal() {
    let problem = IntegerKnapsack::new(vec![3, 4, 5, 2, 7], vec![4, 5, 7, 3, 9], 15).unwrap();
    // c=(0,0,1,5,0): size=0+0+5+10+0=15, value=0+0+7+15+0=22
    assert_eq!(
        problem.evaluate(&vec![0, 0, 1, 5, 0]).unwrap(),
        Max(Some(22))
    );
}

#[test]
fn test_integer_knapsack_evaluate_feasible() {
    let problem = IntegerKnapsack::new(vec![3, 4, 5, 2, 7], vec![4, 5, 7, 3, 9], 15).unwrap();
    // c=(1,0,0,6,0): size=3+0+0+12+0=15, value=4+0+0+18+0=22
    assert_eq!(
        problem.evaluate(&vec![1, 0, 0, 6, 0]).unwrap(),
        Max(Some(22))
    );
}

#[test]
fn test_integer_knapsack_evaluate_overweight() {
    let problem = IntegerKnapsack::new(vec![3, 4, 5, 2, 7], vec![4, 5, 7, 3, 9], 15).unwrap();
    // c=(5,0,0,1,0): size=15+0+0+2+0=17 > 15
    assert_eq!(problem.evaluate(&vec![5, 0, 0, 1, 0]).unwrap(), Max(None));
}

#[test]
fn test_integer_knapsack_evaluate_empty() {
    let problem = IntegerKnapsack::new(vec![3, 4, 5, 2, 7], vec![4, 5, 7, 3, 9], 15).unwrap();
    assert_eq!(
        problem.evaluate(&vec![0, 0, 0, 0, 0]).unwrap(),
        Max(Some(0))
    );
}

#[test]
fn test_integer_knapsack_evaluate_wrong_config_length() {
    let problem = IntegerKnapsack::new(vec![3, 4], vec![4, 5], 10).unwrap();
    assert!(matches!(
        problem.evaluate(&vec![1]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
    assert!(matches!(
        problem.evaluate(&vec![1, 0, 0]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
}

#[test]
fn test_integer_knapsack_evaluate_out_of_domain() {
    let problem = IntegerKnapsack::new(vec![3, 4], vec![4, 5], 10).unwrap();
    // dims = [4, 3], so config [4, 0] is out of domain for item 0
    assert!(matches!(
        problem.evaluate(&vec![4, 0]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
}

#[test]
fn test_integer_knapsack_empty_instance() {
    let problem = IntegerKnapsack::new(vec![], vec![], 10).unwrap();
    assert_eq!(problem.num_items(), 0);
    assert_eq!(problem.dimensions(), Vec::<usize>::new());
    assert_eq!(problem.evaluate(&vec![]).unwrap(), Max(Some(0)));
}

#[test]
fn test_integer_knapsack_brute_force() {
    let problem = IntegerKnapsack::new(vec![3, 4, 5, 2, 7], vec![4, 5, 7, 3, 9], 15).unwrap();
    let solver = BruteForce::new();
    let solution = solver
        .solve(&problem)
        .unwrap()
        .expect("should find a solution");
    let metric = problem.evaluate(&solution).unwrap();
    assert_eq!(metric, Max(Some(22)));
}

#[test]
fn test_integer_knapsack_serialization() {
    let problem = IntegerKnapsack::new(vec![3, 4, 5, 2, 7], vec![4, 5, 7, 3, 9], 15).unwrap();
    let json = serde_json::to_value(&problem).unwrap();
    let restored: IntegerKnapsack = serde_json::from_value(json).unwrap();
    assert_eq!(restored.sizes(), problem.sizes());
    assert_eq!(restored.values(), problem.values());
    assert_eq!(restored.capacity(), problem.capacity());
}

#[test]
fn test_integer_knapsack_zero_capacity() {
    let problem = IntegerKnapsack::new(vec![1, 2], vec![10, 20], 0).unwrap();
    assert_eq!(problem.dimensions(), vec![1, 1]); // floor(0/1)+1=1, floor(0/2)+1=1
    assert_eq!(problem.evaluate(&vec![0, 0]).unwrap(), Max(Some(0)));
    let solver = BruteForce::new();
    let solution = solver.solve(&problem).unwrap().unwrap();
    assert_eq!(problem.evaluate(&solution).unwrap(), Max(Some(0)));
}

#[cfg(target_pointer_width = "64")]
#[test]
fn test_integer_knapsack_dimension_uses_structural_range() {
    let problem = IntegerKnapsack::new(vec![1], vec![1], i64::MAX).unwrap();
    assert_eq!(problem.dimensions(), vec![1_usize << 63]);
}

#[test]
fn test_integer_knapsack_single_item() {
    // Single item size=3, value=5, capacity=7
    // Max multiplicity: floor(7/3)=2, dims=[3]
    let problem = IntegerKnapsack::new(vec![3], vec![5], 7).unwrap();
    assert_eq!(problem.dimensions(), vec![3]);
    assert_eq!(problem.evaluate(&vec![0]).unwrap(), Max(Some(0)));
    assert_eq!(problem.evaluate(&vec![1]).unwrap(), Max(Some(5)));
    assert_eq!(problem.evaluate(&vec![2]).unwrap(), Max(Some(10)));
    let solver = BruteForce::new();
    let solution = solver.solve(&problem).unwrap().unwrap();
    assert_eq!(problem.evaluate(&solution).unwrap(), Max(Some(10)));
}

#[test]
fn test_integer_knapsack_multiple_copies_better() {
    // Item 0: size=3, value=4
    // Item 1: size=5, value=6
    // Capacity=9
    // 0-1 knapsack best: {0,1} size=8, value=10
    // Integer knapsack best: 3 copies of item 0 → size=9, value=12
    let problem = IntegerKnapsack::new(vec![3, 5], vec![4, 6], 9).unwrap();
    let solver = BruteForce::new();
    let solution = solver.solve(&problem).unwrap().unwrap();
    assert_eq!(problem.evaluate(&solution).unwrap(), Max(Some(12)));
}

#[test]
fn test_integer_knapsack_mismatched_lengths() {
    assert!(IntegerKnapsack::new(vec![1, 2], vec![3], 5).is_err());
}

#[test]
fn test_integer_knapsack_rejects_zero_size() {
    assert!(IntegerKnapsack::new(vec![0, 2], vec![3, 4], 5).is_err());
}

#[test]
fn test_integer_knapsack_rejects_negative_size() {
    assert!(IntegerKnapsack::new(vec![-1, 2], vec![3, 4], 5).is_err());
}

#[test]
fn test_integer_knapsack_rejects_zero_value() {
    assert!(IntegerKnapsack::new(vec![1, 2], vec![0, 4], 5).is_err());
}

#[test]
fn test_integer_knapsack_rejects_negative_capacity() {
    assert!(IntegerKnapsack::new(vec![1, 2], vec![3, 4], -1).is_err());
}

#[test]
fn test_integer_knapsack_deserialization_rejects_invalid_fields() {
    let invalid_cases = [
        (
            serde_json::json!({
                "sizes": [0, 2],
                "values": [3, 4],
                "capacity": 5,
            }),
            "positive",
        ),
        (
            serde_json::json!({
                "sizes": [-1, 2],
                "values": [3, 4],
                "capacity": 5,
            }),
            "positive",
        ),
        (
            serde_json::json!({
                "sizes": [1, 2],
                "values": [-3, 4],
                "capacity": 5,
            }),
            "positive",
        ),
        (
            serde_json::json!({
                "sizes": [1, 2],
                "values": [3, 4],
                "capacity": -1,
            }),
            "nonnegative",
        ),
        (
            serde_json::json!({
                "sizes": [1, 2, 3],
                "values": [4, 5],
                "capacity": 10,
            }),
            "same length",
        ),
    ];

    for (invalid, expected_msg) in invalid_cases {
        let error = serde_json::from_value::<IntegerKnapsack>(invalid).unwrap_err();
        assert!(
            error.to_string().contains(expected_msg),
            "Expected error containing '{}', got: {}",
            expected_msg,
            error
        );
    }
}

#[test]
fn test_integer_knapsack_paper_example() {
    // From issue #532: 5 items, sizes=[3,4,5,2,7], values=[4,5,7,3,9], B=15
    // Optimal=22 with c=(0,0,1,5,0) or c=(1,0,0,6,0)
    let problem = IntegerKnapsack::new(vec![3, 4, 5, 2, 7], vec![4, 5, 7, 3, 9], 15).unwrap();

    // Verify both optimal solutions
    assert_eq!(
        problem.evaluate(&vec![0, 0, 1, 5, 0]).unwrap(),
        Max(Some(22))
    );
    assert_eq!(
        problem.evaluate(&vec![1, 0, 0, 6, 0]).unwrap(),
        Max(Some(22))
    );

    // Brute force confirms the optimum
    let solver = BruteForce::new();
    let solution = solver.solve(&problem).unwrap().unwrap();
    assert_eq!(problem.evaluate(&solution).unwrap(), Max(Some(22)));
}
