use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_integer_knapsack_basic() {
    let problem = IntegerKnapsack::new(vec![3, 4, 5, 2, 7], vec![4, 5, 7, 3, 9], 15);
    assert_eq!(problem.num_items(), 5);
    assert_eq!(problem.sizes(), &[3, 4, 5, 2, 7]);
    assert_eq!(problem.values(), &[4, 5, 7, 3, 9]);
    assert_eq!(problem.capacity(), 15);
    // dims: floor(15/3)+1=6, floor(15/4)+1=4, floor(15/5)+1=4, floor(15/2)+1=8, floor(15/7)+1=3
    assert_eq!(problem.dims(), vec![6, 4, 4, 8, 3]);
    assert_eq!(<IntegerKnapsack as Problem>::NAME, "IntegerKnapsack");
    assert_eq!(<IntegerKnapsack as Problem>::variant(), vec![]);
}

#[test]
fn test_integer_knapsack_evaluate_optimal() {
    let problem = IntegerKnapsack::new(vec![3, 4, 5, 2, 7], vec![4, 5, 7, 3, 9], 15);
    // c=(0,0,1,5,0): size=0+0+5+10+0=15, value=0+0+7+15+0=22
    assert_eq!(problem.evaluate(&[0, 0, 1, 5, 0]), Max(Some(22)));
}

#[test]
fn test_integer_knapsack_evaluate_feasible() {
    let problem = IntegerKnapsack::new(vec![3, 4, 5, 2, 7], vec![4, 5, 7, 3, 9], 15);
    // c=(1,0,0,6,0): size=3+0+0+12+0=15, value=4+0+0+18+0=22
    assert_eq!(problem.evaluate(&[1, 0, 0, 6, 0]), Max(Some(22)));
}

#[test]
fn test_integer_knapsack_evaluate_overweight() {
    let problem = IntegerKnapsack::new(vec![3, 4, 5, 2, 7], vec![4, 5, 7, 3, 9], 15);
    // c=(5,0,0,1,0): size=15+0+0+2+0=17 > 15
    assert_eq!(problem.evaluate(&[5, 0, 0, 1, 0]), Max(None));
}

#[test]
fn test_integer_knapsack_evaluate_empty() {
    let problem = IntegerKnapsack::new(vec![3, 4, 5, 2, 7], vec![4, 5, 7, 3, 9], 15);
    assert_eq!(problem.evaluate(&[0, 0, 0, 0, 0]), Max(Some(0)));
}

#[test]
fn test_integer_knapsack_evaluate_wrong_config_length() {
    let problem = IntegerKnapsack::new(vec![3, 4], vec![4, 5], 10);
    assert_eq!(problem.evaluate(&[1]), Max(None));
    assert_eq!(problem.evaluate(&[1, 0, 0]), Max(None));
}

#[test]
fn test_integer_knapsack_evaluate_out_of_domain() {
    let problem = IntegerKnapsack::new(vec![3, 4], vec![4, 5], 10);
    // dims = [4, 3], so config [4, 0] is out of domain for item 0
    assert_eq!(problem.evaluate(&[4, 0]), Max(None));
}

#[test]
fn test_integer_knapsack_empty_instance() {
    let problem = IntegerKnapsack::new(vec![], vec![], 10);
    assert_eq!(problem.num_items(), 0);
    assert_eq!(problem.dims(), Vec::<usize>::new());
    assert_eq!(problem.evaluate(&[]), Max(Some(0)));
}

#[test]
fn test_integer_knapsack_brute_force() {
    let problem = IntegerKnapsack::new(vec![3, 4, 5, 2, 7], vec![4, 5, 7, 3, 9], 15);
    let solver = BruteForce::new();
    let solution = solver
        .find_witness(&problem)
        .expect("should find a solution");
    let metric = problem.evaluate(&solution);
    assert_eq!(metric, Max(Some(22)));
}

#[test]
fn test_integer_knapsack_serialization() {
    let problem = IntegerKnapsack::new(vec![3, 4, 5, 2, 7], vec![4, 5, 7, 3, 9], 15);
    let json = serde_json::to_value(&problem).unwrap();
    let restored: IntegerKnapsack = serde_json::from_value(json).unwrap();
    assert_eq!(restored.sizes(), problem.sizes());
    assert_eq!(restored.values(), problem.values());
    assert_eq!(restored.capacity(), problem.capacity());
}

#[test]
fn test_integer_knapsack_zero_capacity() {
    let problem = IntegerKnapsack::new(vec![1, 2], vec![10, 20], 0);
    assert_eq!(problem.dims(), vec![1, 1]); // floor(0/1)+1=1, floor(0/2)+1=1
    assert_eq!(problem.evaluate(&[0, 0]), Max(Some(0)));
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem).unwrap();
    assert_eq!(problem.evaluate(&solution), Max(Some(0)));
}

#[test]
fn test_integer_knapsack_single_item() {
    // Single item size=3, value=5, capacity=7
    // Max multiplicity: floor(7/3)=2, dims=[3]
    let problem = IntegerKnapsack::new(vec![3], vec![5], 7);
    assert_eq!(problem.dims(), vec![3]);
    assert_eq!(problem.evaluate(&[0]), Max(Some(0)));
    assert_eq!(problem.evaluate(&[1]), Max(Some(5)));
    assert_eq!(problem.evaluate(&[2]), Max(Some(10)));
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem).unwrap();
    assert_eq!(problem.evaluate(&solution), Max(Some(10)));
}

#[test]
fn test_integer_knapsack_multiple_copies_better() {
    // Item 0: size=3, value=4
    // Item 1: size=5, value=6
    // Capacity=9
    // 0-1 knapsack best: {0,1} size=8, value=10
    // Integer knapsack best: 3 copies of item 0 → size=9, value=12
    let problem = IntegerKnapsack::new(vec![3, 5], vec![4, 6], 9);
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem).unwrap();
    assert_eq!(problem.evaluate(&solution), Max(Some(12)));
}

#[test]
#[should_panic(expected = "sizes and values must have the same length")]
fn test_integer_knapsack_mismatched_lengths() {
    IntegerKnapsack::new(vec![1, 2], vec![3], 5);
}

#[test]
#[should_panic(expected = "positive")]
fn test_integer_knapsack_zero_size_panics() {
    IntegerKnapsack::new(vec![0, 2], vec![3, 4], 5);
}

#[test]
#[should_panic(expected = "positive")]
fn test_integer_knapsack_negative_size_panics() {
    IntegerKnapsack::new(vec![-1, 2], vec![3, 4], 5);
}

#[test]
#[should_panic(expected = "positive")]
fn test_integer_knapsack_zero_value_panics() {
    IntegerKnapsack::new(vec![1, 2], vec![0, 4], 5);
}

#[test]
#[should_panic(expected = "nonnegative")]
fn test_integer_knapsack_negative_capacity_panics() {
    IntegerKnapsack::new(vec![1, 2], vec![3, 4], -1);
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
    let problem = IntegerKnapsack::new(vec![3, 4, 5, 2, 7], vec![4, 5, 7, 3, 9], 15);

    // Verify both optimal solutions
    assert_eq!(problem.evaluate(&[0, 0, 1, 5, 0]), Max(Some(22)));
    assert_eq!(problem.evaluate(&[1, 0, 0, 6, 0]), Max(Some(22)));

    // Brute force confirms the optimum
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem).unwrap();
    assert_eq!(problem.evaluate(&solution), Max(Some(22)));
}
