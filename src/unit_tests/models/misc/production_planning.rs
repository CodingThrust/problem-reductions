use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Or;

fn issue_example_problem() -> ProductionPlanning {
    ProductionPlanning::new(
        6,
        vec![5, 3, 7, 2, 8, 5],
        vec![12, 12, 12, 12, 12, 12],
        vec![10, 10, 10, 10, 10, 10],
        vec![1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1],
        80,
    )
}

fn tiny_solver_problem() -> ProductionPlanning {
    ProductionPlanning::new(
        3,
        vec![1, 1, 1],
        vec![2, 1, 1],
        vec![1, 1, 1],
        vec![1, 1, 1],
        vec![0, 0, 0],
        5,
    )
}

#[test]
fn test_production_planning_creation() {
    let problem = issue_example_problem();
    assert_eq!(problem.num_periods(), 6);
    assert_eq!(problem.demands(), &[5, 3, 7, 2, 8, 5]);
    assert_eq!(problem.capacities(), &[12, 12, 12, 12, 12, 12]);
    assert_eq!(problem.setup_costs(), &[10, 10, 10, 10, 10, 10]);
    assert_eq!(problem.production_costs(), &[1, 1, 1, 1, 1, 1]);
    assert_eq!(problem.inventory_costs(), &[1, 1, 1, 1, 1, 1]);
    assert_eq!(problem.cost_bound(), 80);
    assert_eq!(problem.max_capacity(), 12);
    assert_eq!(problem.dims(), vec![13; 6]);
    assert_eq!(<ProductionPlanning as Problem>::NAME, "ProductionPlanning");
    assert_eq!(<ProductionPlanning as Problem>::variant(), vec![]);
}

#[test]
fn test_production_planning_evaluate_issue_example() {
    let problem = issue_example_problem();
    assert_eq!(problem.evaluate(&[8, 0, 10, 0, 12, 0]), Or(true));
}

#[test]
fn test_production_planning_rejects_capacity_overflow() {
    let problem = issue_example_problem();
    assert_eq!(problem.evaluate(&[13, 0, 10, 0, 12, 0]), Or(false));
}

#[test]
fn test_production_planning_rejects_negative_inventory_prefix() {
    let problem = issue_example_problem();
    assert_eq!(problem.evaluate(&[4, 4, 4, 4, 4, 4]), Or(false));
}

#[test]
fn test_production_planning_rejects_budget_overflow() {
    let problem = issue_example_problem();
    assert_eq!(problem.evaluate(&[8, 0, 10, 0, 12, 1]), Or(false));
}

#[test]
fn test_production_planning_rejects_wrong_config_length() {
    let problem = issue_example_problem();
    assert_eq!(problem.evaluate(&[8, 0, 10, 0, 12]), Or(false));
}

#[test]
fn test_production_planning_bruteforce_finds_satisfying_solution() {
    let problem = tiny_solver_problem();
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem);
    assert!(solution.is_some());
    assert_eq!(problem.evaluate(&solution.unwrap()), Or(true));
}

#[test]
fn test_production_planning_paper_example() {
    let problem = issue_example_problem();
    let plan = vec![8, 0, 10, 0, 12, 0];
    let solver = BruteForce::new();

    assert_eq!(problem.evaluate(&plan), Or(true));

    let witness = solver.find_witness(&problem);
    assert!(witness.is_some());
    assert_eq!(problem.evaluate(&witness.unwrap()), Or(true));
}

#[test]
fn test_production_planning_serialization() {
    let problem = issue_example_problem();
    let json = serde_json::to_value(&problem).unwrap();
    let restored: ProductionPlanning = serde_json::from_value(json).unwrap();
    assert_eq!(restored.num_periods(), problem.num_periods());
    assert_eq!(restored.demands(), problem.demands());
    assert_eq!(restored.capacities(), problem.capacities());
    assert_eq!(restored.setup_costs(), problem.setup_costs());
    assert_eq!(restored.production_costs(), problem.production_costs());
    assert_eq!(restored.inventory_costs(), problem.inventory_costs());
    assert_eq!(restored.cost_bound(), problem.cost_bound());
}

#[test]
#[should_panic(expected = "all per-period vectors must have length num_periods")]
fn test_production_planning_rejects_length_mismatch() {
    ProductionPlanning::new(
        2,
        vec![1],
        vec![1, 1],
        vec![1, 1],
        vec![1, 1],
        vec![1, 1],
        3,
    );
}

#[test]
#[should_panic(expected = "capacities must fit in usize for dims()")]
fn test_production_planning_rejects_capacity_too_large_for_dims() {
    ProductionPlanning::new(1, vec![0], vec![u64::MAX], vec![0], vec![0], vec![0], 0);
}

#[test]
#[should_panic(expected = "num_periods must be positive")]
fn test_production_planning_rejects_zero_periods() {
    ProductionPlanning::new(0, vec![], vec![], vec![], vec![], vec![], 0);
}
