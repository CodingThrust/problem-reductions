use crate::models::misc::ThreePartition;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Or;

fn yes_problem() -> ThreePartition {
    ThreePartition::new(vec![4, 5, 6, 4, 6, 5], 15)
}

#[test]
fn test_three_partition_basic() {
    let problem = yes_problem();
    assert_eq!(problem.sizes(), &[4, 5, 6, 4, 6, 5]);
    assert_eq!(problem.bound(), 15);
    assert_eq!(problem.num_elements(), 6);
    assert_eq!(problem.num_groups(), 2);
    assert_eq!(problem.total_sum(), 30);
    assert_eq!(problem.dims(), vec![2; 6]);
    assert_eq!(problem.num_variables(), 6);
    assert_eq!(<ThreePartition as Problem>::NAME, "ThreePartition");
    assert_eq!(<ThreePartition as Problem>::variant(), vec![]);
}

#[test]
fn test_three_partition_evaluate_yes_instance() {
    let problem = yes_problem();
    assert_eq!(problem.evaluate(&[0, 0, 0, 1, 1, 1]), Or(true));
}

#[test]
fn test_three_partition_rejects_wrong_group_sizes_or_sums() {
    let problem = yes_problem();
    assert_eq!(problem.evaluate(&[0, 0, 1, 1, 1, 1]), Or(false));
    assert_eq!(problem.evaluate(&[0, 1, 0, 1, 0, 1]), Or(false));
}

#[test]
fn test_three_partition_rejects_invalid_configs() {
    let problem = yes_problem();
    assert_eq!(problem.evaluate(&[0, 0, 0]), Or(false));
    assert_eq!(problem.evaluate(&[0, 0, 0, 1, 1, 1, 0]), Or(false));
    assert_eq!(problem.evaluate(&[0, 0, 0, 1, 1, 2]), Or(false));
}

#[test]
fn test_three_partition_solver_finds_witness() {
    let problem = yes_problem();
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem).unwrap();
    assert_eq!(problem.evaluate(&solution), Or(true));
}

#[test]
fn test_three_partition_solver_reports_unsatisfiable_instance() {
    let problem = ThreePartition::new(vec![6, 6, 6, 6, 7, 9], 20);
    let solver = BruteForce::new();
    assert!(solver.find_witness(&problem).is_none());
}

#[test]
fn test_three_partition_paper_example() {
    let problem = yes_problem();
    let config = vec![0, 0, 0, 1, 1, 1];
    assert_eq!(problem.evaluate(&config), Or(true));

    let solver = BruteForce::new();
    let all = solver.find_all_witnesses(&problem);
    assert_eq!(all.len(), 8);
    assert!(all.iter().all(|sol| problem.evaluate(sol) == Or(true)));
}

#[test]
fn test_three_partition_serialization_round_trip() {
    let problem = yes_problem();
    let json = serde_json::to_value(&problem).unwrap();
    assert_eq!(
        json,
        serde_json::json!({
            "sizes": [4, 5, 6, 4, 6, 5],
            "bound": 15,
        })
    );

    let restored: ThreePartition = serde_json::from_value(json).unwrap();
    assert_eq!(restored.sizes(), problem.sizes());
    assert_eq!(restored.bound(), problem.bound());
}

#[test]
fn test_three_partition_deserialization_rejects_invalid_instances() {
    let invalid_cases = [
        serde_json::json!({
            "sizes": [],
            "bound": 15,
        }),
        serde_json::json!({
            "sizes": [4, 5, 6, 4, 6],
            "bound": 15,
        }),
        serde_json::json!({
            "sizes": [4, 5, 0, 4, 6, 5],
            "bound": 15,
        }),
        serde_json::json!({
            "sizes": [3, 5, 6, 4, 6, 6],
            "bound": 15,
        }),
        serde_json::json!({
            "sizes": [4, 5, 6, 4, 6, 5],
            "bound": 14,
        }),
    ];

    for invalid in invalid_cases {
        assert!(serde_json::from_value::<ThreePartition>(invalid).is_err());
    }
}

#[test]
#[should_panic(expected = "at least one element")]
fn test_three_partition_empty_sizes_panics() {
    ThreePartition::new(vec![], 15);
}

#[test]
#[should_panic(expected = "multiple of 3")]
fn test_three_partition_requires_three_m_elements() {
    ThreePartition::new(vec![4, 5, 6, 4, 6], 15);
}

#[test]
#[should_panic(expected = "positive")]
fn test_three_partition_rejects_zero_sizes() {
    ThreePartition::new(vec![4, 5, 0, 4, 6, 5], 15);
}

#[test]
#[should_panic(expected = "strictly between")]
fn test_three_partition_rejects_sizes_outside_strict_bounds() {
    ThreePartition::new(vec![3, 5, 6, 4, 6, 6], 15);
}

#[test]
#[should_panic(expected = "must equal m * bound")]
fn test_three_partition_rejects_wrong_total_sum() {
    ThreePartition::new(vec![4, 5, 6, 4, 6, 5], 14);
}
