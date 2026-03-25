use crate::models::misc::CapacityAssignment;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Min;

fn example_problem() -> CapacityAssignment {
    CapacityAssignment::new(
        vec![1, 2, 3],
        vec![vec![1, 3, 6], vec![2, 4, 7], vec![1, 2, 5]],
        vec![vec![8, 4, 1], vec![7, 3, 1], vec![6, 3, 1]],
        12,
    )
}

#[test]
fn test_capacity_assignment_basic_properties() {
    let problem = example_problem();
    assert_eq!(problem.num_links(), 3);
    assert_eq!(problem.num_capacities(), 3);
    assert_eq!(problem.capacities(), &[1, 2, 3]);
    assert_eq!(problem.delay_budget(), 12);
    assert_eq!(problem.dims(), vec![3, 3, 3]);
    assert_eq!(<CapacityAssignment as Problem>::NAME, "CapacityAssignment");
    assert_eq!(<CapacityAssignment as Problem>::variant(), Vec::new());
}

#[test]
fn test_capacity_assignment_evaluate_feasible_and_infeasible() {
    let problem = example_problem();
    // [1,1,1]: cost=3+4+2=9, delay=4+3+3=10 ≤ 12 → Min(Some(9))
    assert_eq!(problem.evaluate(&[1, 1, 1]), Min(Some(9)));
    // [0,1,2]: cost=1+4+5=10, delay=8+3+1=12 ≤ 12 → Min(Some(10))
    assert_eq!(problem.evaluate(&[0, 1, 2]), Min(Some(10)));
    // [0,0,0]: cost=1+2+1=4, delay=8+7+6=21 > 12 → Min(None)
    assert_eq!(problem.evaluate(&[0, 0, 0]), Min(None));
    // [2,2,2]: cost=6+7+5=18, delay=1+1+1=3 ≤ 12 → Min(Some(18))
    assert_eq!(problem.evaluate(&[2, 2, 2]), Min(Some(18)));
}

#[test]
fn test_capacity_assignment_rejects_invalid_configs() {
    let problem = example_problem();
    assert_eq!(problem.evaluate(&[1, 1]), Min(None));
    assert_eq!(problem.evaluate(&[1, 1, 3]), Min(None));
}

#[test]
fn test_capacity_assignment_bruteforce_optimal() {
    let problem = example_problem();
    let solver = BruteForce::new();
    let witness = solver.find_witness(&problem).expect("should find witness");
    // Optimal cost is 9 at [1,1,1]
    assert_eq!(problem.evaluate(&witness), Min(Some(9)));
    assert_eq!(witness, vec![1, 1, 1]);
}

#[test]
fn test_capacity_assignment_serialization_round_trip() {
    let problem = example_problem();
    let json = serde_json::to_value(&problem).unwrap();
    let restored: CapacityAssignment = serde_json::from_value(json).unwrap();
    assert_eq!(restored.capacities(), problem.capacities());
    assert_eq!(restored.cost(), problem.cost());
    assert_eq!(restored.delay(), problem.delay());
    assert_eq!(restored.delay_budget(), problem.delay_budget());
}

#[test]
fn test_capacity_assignment_paper_example() {
    let problem = example_problem();
    let config = vec![1, 1, 1];
    assert_eq!(problem.evaluate(&config), Min(Some(9)));

    let solver = BruteForce::new();
    let witness = solver.find_witness(&problem).expect("should find optimal");
    assert_eq!(problem.evaluate(&witness), Min(Some(9)));
}

#[test]
fn test_capacity_assignment_rejects_non_increasing_capacities() {
    let result = std::panic::catch_unwind(|| {
        CapacityAssignment::new(vec![1, 1], vec![vec![1, 2]], vec![vec![2, 1]], 3)
    });
    assert!(result.is_err());
}

#[test]
fn test_capacity_assignment_rejects_non_monotone_delay_row() {
    let result = std::panic::catch_unwind(|| {
        CapacityAssignment::new(vec![1, 2], vec![vec![1, 2]], vec![vec![1, 2]], 3)
    });
    assert!(result.is_err());
}
