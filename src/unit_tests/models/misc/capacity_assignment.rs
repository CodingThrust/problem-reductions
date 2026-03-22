use crate::models::misc::CapacityAssignment;
use crate::solvers::BruteForce;
use crate::traits::Problem;

fn example_problem() -> CapacityAssignment {
    CapacityAssignment::new(
        vec![1, 2, 3],
        vec![vec![1, 3, 6], vec![2, 4, 7], vec![1, 2, 5]],
        vec![vec![8, 4, 1], vec![7, 3, 1], vec![6, 3, 1]],
        10,
        12,
    )
}

#[test]
fn test_capacity_assignment_basic_properties() {
    let problem = example_problem();
    assert_eq!(problem.num_links(), 3);
    assert_eq!(problem.num_capacities(), 3);
    assert_eq!(problem.capacities(), &[1, 2, 3]);
    assert_eq!(problem.cost_budget(), 10);
    assert_eq!(problem.delay_budget(), 12);
    assert_eq!(problem.dims(), vec![3, 3, 3]);
    assert_eq!(
        <CapacityAssignment as Problem>::NAME,
        "CapacityAssignment"
    );
    assert_eq!(<CapacityAssignment as Problem>::variant(), Vec::new());
}

#[test]
fn test_capacity_assignment_evaluate_yes_and_no_examples() {
    let problem = example_problem();
    assert!(problem.evaluate(&[1, 1, 1]));
    assert!(problem.evaluate(&[0, 1, 2]));
    assert!(!problem.evaluate(&[0, 0, 0]));
    assert!(!problem.evaluate(&[2, 2, 2]));
}

#[test]
fn test_capacity_assignment_rejects_invalid_configs() {
    let problem = example_problem();
    assert!(!problem.evaluate(&[1, 1]));
    assert!(!problem.evaluate(&[1, 1, 3]));
}

#[test]
fn test_capacity_assignment_bruteforce_solution_count() {
    let problem = example_problem();
    let solver = BruteForce::new();
    let solutions = solver.find_all_satisfying(&problem);
    assert_eq!(solutions.len(), 5);
    assert!(solutions.contains(&vec![1, 1, 1]));
    assert!(solutions.contains(&vec![0, 1, 2]));
}

#[test]
fn test_capacity_assignment_serialization_round_trip() {
    let problem = example_problem();
    let json = serde_json::to_value(&problem).unwrap();
    let restored: CapacityAssignment = serde_json::from_value(json).unwrap();
    assert_eq!(restored.capacities(), problem.capacities());
    assert_eq!(restored.cost(), problem.cost());
    assert_eq!(restored.delay(), problem.delay());
    assert_eq!(restored.cost_budget(), problem.cost_budget());
    assert_eq!(restored.delay_budget(), problem.delay_budget());
}

#[test]
fn test_capacity_assignment_paper_example() {
    let problem = example_problem();
    let config = vec![1, 1, 1];
    assert!(problem.evaluate(&config));

    let solver = BruteForce::new();
    let solutions = solver.find_all_satisfying(&problem);
    assert_eq!(solutions.len(), 5);
    assert!(solutions.contains(&config));
}

#[test]
fn test_capacity_assignment_rejects_non_increasing_capacities() {
    let result = std::panic::catch_unwind(|| {
        CapacityAssignment::new(vec![1, 1], vec![vec![1, 2]], vec![vec![2, 1]], 3, 3)
    });
    assert!(result.is_err());
}

#[test]
fn test_capacity_assignment_rejects_non_monotone_delay_row() {
    let result = std::panic::catch_unwind(|| {
        CapacityAssignment::new(vec![1, 2], vec![vec![1, 2]], vec![vec![1, 2]], 3, 3)
    });
    assert!(result.is_err());
}
