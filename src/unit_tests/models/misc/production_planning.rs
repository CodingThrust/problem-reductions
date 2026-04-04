use crate::models::misc::ProductionPlanning;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Or;

fn issue_example() -> ProductionPlanning {
    ProductionPlanning::new(
        vec![5, 3, 7, 2, 8, 5],
        vec![12, 12, 12, 12, 12, 12],
        vec![10, 10, 10, 10, 10, 10],
        vec![1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1],
        80,
    )
}

#[test]
fn test_production_planning_creation() {
    let problem = issue_example();

    assert_eq!(problem.num_periods(), 6);
    assert_eq!(problem.demands(), &[5, 3, 7, 2, 8, 5]);
    assert_eq!(problem.capacities(), &[12, 12, 12, 12, 12, 12]);
    assert_eq!(problem.setup_costs(), &[10, 10, 10, 10, 10, 10]);
    assert_eq!(problem.production_costs(), &[1, 1, 1, 1, 1, 1]);
    assert_eq!(problem.inventory_costs(), &[1, 1, 1, 1, 1, 1]);
    assert_eq!(problem.bound(), 80);
    assert_eq!(problem.dims(), vec![13; 6]);
    assert_eq!(<ProductionPlanning as Problem>::NAME, "ProductionPlanning");
    assert_eq!(<ProductionPlanning as Problem>::variant(), vec![]);
}

#[test]
fn test_production_planning_evaluate_feasible_plan() {
    let problem = issue_example();

    assert_eq!(problem.evaluate(&[8, 0, 10, 0, 12, 0]), Or(true));
}

#[test]
fn test_production_planning_rejects_invalid_plans() {
    let problem = issue_example();

    assert_eq!(problem.evaluate(&[13, 0, 10, 0, 12, 0]), Or(false));
    assert_eq!(problem.evaluate(&[5, 0, 0, 0, 0, 0]), Or(false));
    assert_eq!(problem.evaluate(&[12, 0, 12, 0, 6, 0]), Or(false));
    assert_eq!(problem.evaluate(&[8, 0, 10, 0, 12]), Or(false));
}

#[test]
fn test_production_planning_bruteforce_solver() {
    let problem = ProductionPlanning::new(
        vec![1, 1],
        vec![2, 0],
        vec![1, 0],
        vec![0, 0],
        vec![0, 0],
        1,
    );
    let solver = BruteForce::new();

    assert_eq!(solver.find_all_witnesses(&problem), vec![vec![2, 0]]);
    assert_eq!(solver.find_witness(&problem), Some(vec![2, 0]));
}

#[test]
fn test_production_planning_serialization() {
    let problem = issue_example();
    let json = serde_json::to_value(&problem).unwrap();
    let restored: ProductionPlanning = serde_json::from_value(json).unwrap();

    assert_eq!(restored.demands(), problem.demands());
    assert_eq!(restored.capacities(), problem.capacities());
    assert_eq!(restored.setup_costs(), problem.setup_costs());
    assert_eq!(restored.production_costs(), problem.production_costs());
    assert_eq!(restored.inventory_costs(), problem.inventory_costs());
    assert_eq!(restored.bound(), problem.bound());
}
