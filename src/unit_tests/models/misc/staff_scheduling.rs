use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_staff_scheduling_create_spec_uses_k_input() {
    assert_eq!(StaffSchedulingCreateSpec::FIELDS[0].name, "k");
    let problem = StaffScheduling::try_from(StaffSchedulingCreateSpec {
        k: 1,
        schedules: vec![vec![true, false]],
        requirements: vec![1, 0],
        num_workers: 1,
    })
    .unwrap();
    assert_eq!(problem.shifts_per_schedule(), 1);
}

fn issue_example_problem() -> StaffScheduling {
    StaffScheduling::new(
        5,
        vec![
            vec![true, true, true, true, true, false, false],
            vec![false, true, true, true, true, true, false],
            vec![false, false, true, true, true, true, true],
            vec![true, false, false, true, true, true, true],
            vec![true, true, false, false, true, true, true],
        ],
        vec![2, 2, 2, 3, 3, 2, 1],
        4,
    )
}

#[test]
fn test_staff_scheduling_creation() {
    let problem = issue_example_problem();
    assert_eq!(problem.num_periods(), 7);
    assert_eq!(problem.shifts_per_schedule(), 5);
    assert_eq!(problem.num_schedules(), 5);
    assert_eq!(problem.requirements(), &[2, 2, 2, 3, 3, 2, 1]);
    assert_eq!(problem.num_workers(), 4);
    assert_eq!(problem.dims(), vec![5; 5]);
}

#[test]
#[should_panic(expected = "schedule 1 has 2 periods, expected 3")]
fn test_staff_scheduling_new_panics_on_schedule_length_mismatch() {
    let _ = StaffScheduling::new(
        1,
        vec![vec![true, false, false], vec![false, true]],
        vec![1, 1, 1],
        2,
    );
}

#[test]
#[should_panic(expected = "schedule 1 has 2 active periods, expected 1")]
fn test_staff_scheduling_new_panics_on_wrong_active_period_count() {
    let _ = StaffScheduling::new(
        1,
        vec![vec![true, false, false], vec![false, true, true]],
        vec![1, 1, 1],
        2,
    );
}

#[test]
fn test_staff_scheduling_evaluate_feasible_issue_example() {
    let problem = issue_example_problem();
    assert!(problem.evaluate(&[1, 1, 1, 1, 0]).unwrap());
}

#[test]
fn test_staff_scheduling_rejects_invalid_configs() {
    let problem = issue_example_problem();
    assert!(!problem.evaluate(&[1, 1, 1, 1]).unwrap());
    assert!(!problem.evaluate(&[5, 0, 0, 0, 0]).unwrap());
    assert!(!problem.evaluate(&[1, 1, 1, 1, 1]).unwrap());
    assert!(!problem.evaluate(&[0, 0, 0, 0, 4]).unwrap());
}

#[test]
fn test_staff_scheduling_bruteforce_solver_finds_solution() {
    let problem = issue_example_problem();
    let solution = BruteForce::new().find_witness(&problem).unwrap();
    assert!(solution.is_some());
    assert!(problem.evaluate(&solution.unwrap()).unwrap());
}

#[test]
fn test_staff_scheduling_bruteforce_solver_detects_unsat() {
    let problem =
        StaffScheduling::new(1, vec![vec![true, false], vec![false, true]], vec![2, 2], 1);
    assert!(BruteForce::new().find_witness(&problem).unwrap().is_none());
}

#[test]
fn test_staff_scheduling_serialization_round_trip() {
    let problem = issue_example_problem();
    let json = serde_json::to_value(&problem).unwrap();
    let restored: StaffScheduling = serde_json::from_value(json).unwrap();
    assert_eq!(
        restored.shifts_per_schedule(),
        problem.shifts_per_schedule()
    );
    assert_eq!(restored.schedules(), problem.schedules());
    assert_eq!(restored.requirements(), problem.requirements());
    assert_eq!(restored.num_workers(), problem.num_workers());
}

#[test]
fn test_staff_scheduling_paper_example() {
    let problem = issue_example_problem();
    let config = vec![1, 1, 1, 1, 0];
    assert!(problem.evaluate(&config).unwrap());

    let satisfying = BruteForce::new().find_all_witnesses(&problem).unwrap();
    assert!(satisfying.contains(&config));
}

#[test]
fn test_staff_scheduling_problem_name_and_variant() {
    assert_eq!(<StaffScheduling as Problem>::NAME, "StaffScheduling");
    assert!(<StaffScheduling as Problem>::variant().is_empty());
}
