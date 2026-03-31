use crate::models::misc::Numerical3DimensionalMatching;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Or;

fn yes_problem() -> Numerical3DimensionalMatching {
    // W=[4,5], X=[4,5], Y=[5,7], B=15, m=2
    // Valid: w0↔x0,y1 (4+4+7=15) and w1↔x1,y0 (5+5+5=15)
    Numerical3DimensionalMatching::new(vec![4, 5], vec![4, 5], vec![5, 7], 15)
}

#[test]
fn test_numerical_3dm_creation() {
    let problem = yes_problem();
    assert_eq!(problem.sizes_w(), &[4, 5]);
    assert_eq!(problem.sizes_x(), &[4, 5]);
    assert_eq!(problem.sizes_y(), &[5, 7]);
    assert_eq!(problem.bound(), 15);
    assert_eq!(problem.num_groups(), 2);
    assert_eq!(problem.dims(), vec![2; 4]);
    assert_eq!(problem.num_variables(), 4);
    assert_eq!(
        <Numerical3DimensionalMatching as Problem>::NAME,
        "Numerical3DimensionalMatching"
    );
    assert_eq!(
        <Numerical3DimensionalMatching as Problem>::variant(),
        vec![]
    );
}

#[test]
fn test_numerical_3dm_evaluate_valid() {
    let problem = yes_problem();
    // config [0, 1, 1, 0]: w0↔x0,y1 (4+4+7=15), w1↔x1,y0 (5+5+5=15)
    assert_eq!(problem.evaluate(&[0, 1, 1, 0]), Or(true));
}

#[test]
fn test_numerical_3dm_evaluate_invalid_sums() {
    let problem = yes_problem();
    // config [0, 1, 0, 1]: w0↔x0,y0 (4+4+5=13≠15)
    assert_eq!(problem.evaluate(&[0, 1, 0, 1]), Or(false));
    // config [1, 0, 0, 1]: w0↔x1,y0 (4+5+5=14≠15)
    assert_eq!(problem.evaluate(&[1, 0, 0, 1]), Or(false));
}

#[test]
fn test_numerical_3dm_evaluate_invalid_permutation() {
    let problem = yes_problem();
    // Both X assignments point to 0 — not a permutation
    assert_eq!(problem.evaluate(&[0, 0, 0, 1]), Or(false));
    // Both Y assignments point to 1 — not a permutation
    assert_eq!(problem.evaluate(&[0, 1, 1, 1]), Or(false));
}

#[test]
fn test_numerical_3dm_evaluate_wrong_length() {
    let problem = yes_problem();
    assert_eq!(problem.evaluate(&[0, 1, 1]), Or(false));
    assert_eq!(problem.evaluate(&[0, 1, 1, 0, 0]), Or(false));
}

#[test]
fn test_numerical_3dm_evaluate_out_of_range() {
    let problem = yes_problem();
    // Index 2 is out of range for m=2
    assert_eq!(problem.evaluate(&[0, 2, 1, 0]), Or(false));
}

#[test]
fn test_numerical_3dm_solver_finds_witness() {
    let problem = yes_problem();
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem).unwrap();
    assert_eq!(problem.evaluate(&solution), Or(true));
}

#[test]
fn test_numerical_3dm_solver_unsatisfiable() {
    // W=[4,5], X=[4,6], Y=[5,7], B=15 — total=31≠30, invalid
    // Need a valid instance that is unsatisfiable.
    // W=[4,5], X=[4,5], Y=[4,7], B=15 — total=4+5+4+5+4+7=29≠30, invalid.
    // W=[4,6], X=[4,6], Y=[4,6], B=15 — total=4+6+4+6+4+6=30=2*15, all between 3.75 and 7.5.
    // Check: w0↔x0,y0: 4+4+4=12≠15; w0↔x0,y1: 4+4+6=14≠15; w0↔x1,y0: 4+6+4=14≠15; w0↔x1,y1: 4+6+6=16≠15
    // No valid matching exists!
    let problem = Numerical3DimensionalMatching::new(vec![4, 6], vec![4, 6], vec![4, 6], 15);
    let solver = BruteForce::new();
    assert!(solver.find_witness(&problem).is_none());
}

#[test]
fn test_numerical_3dm_serialization_round_trip() {
    let problem = yes_problem();
    let json = serde_json::to_value(&problem).unwrap();
    assert_eq!(
        json,
        serde_json::json!({
            "sizes_w": [4, 5],
            "sizes_x": [4, 5],
            "sizes_y": [5, 7],
            "bound": 15,
        })
    );

    let restored: Numerical3DimensionalMatching = serde_json::from_value(json).unwrap();
    assert_eq!(restored.sizes_w(), problem.sizes_w());
    assert_eq!(restored.sizes_x(), problem.sizes_x());
    assert_eq!(restored.sizes_y(), problem.sizes_y());
    assert_eq!(restored.bound(), problem.bound());
}

#[test]
fn test_numerical_3dm_deserialization_rejects_invalid() {
    let invalid_cases = [
        // Empty sets
        serde_json::json!({
            "sizes_w": [],
            "sizes_x": [],
            "sizes_y": [],
            "bound": 15,
        }),
        // Different set sizes
        serde_json::json!({
            "sizes_w": [4, 5],
            "sizes_x": [4],
            "sizes_y": [5, 7],
            "bound": 15,
        }),
        // Zero size
        serde_json::json!({
            "sizes_w": [0, 5],
            "sizes_x": [4, 5],
            "sizes_y": [5, 7],
            "bound": 15,
        }),
        // Size outside B/4..B/2 range
        serde_json::json!({
            "sizes_w": [3, 5],
            "sizes_x": [4, 5],
            "sizes_y": [5, 8],
            "bound": 15,
        }),
        // Wrong total sum
        serde_json::json!({
            "sizes_w": [4, 5],
            "sizes_x": [4, 5],
            "sizes_y": [5, 7],
            "bound": 14,
        }),
    ];

    for invalid in invalid_cases {
        assert!(serde_json::from_value::<Numerical3DimensionalMatching>(invalid).is_err());
    }
}

#[test]
#[should_panic(expected = "at least one element")]
fn test_numerical_3dm_empty_sets_panics() {
    Numerical3DimensionalMatching::new(vec![], vec![], vec![], 15);
}

#[test]
#[should_panic(expected = "same size")]
fn test_numerical_3dm_mismatched_sizes_panics() {
    Numerical3DimensionalMatching::new(vec![4, 5], vec![4], vec![5, 7], 15);
}

#[test]
#[should_panic(expected = "positive")]
fn test_numerical_3dm_zero_size_panics() {
    Numerical3DimensionalMatching::new(vec![0, 5], vec![4, 5], vec![5, 7], 15);
}

#[test]
#[should_panic(expected = "strictly between")]
fn test_numerical_3dm_size_outside_bounds_panics() {
    Numerical3DimensionalMatching::new(vec![3, 5], vec![4, 5], vec![5, 8], 15);
}

#[test]
#[should_panic(expected = "must equal m * bound")]
fn test_numerical_3dm_wrong_total_sum_panics() {
    Numerical3DimensionalMatching::new(vec![4, 5], vec![4, 5], vec![5, 6], 14);
}
