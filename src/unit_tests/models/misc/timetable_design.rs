use crate::models::misc::TimetableDesign;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;

fn timetable_design_flat_index(
    num_tasks: usize,
    num_periods: usize,
    craftsman: usize,
    task: usize,
    period: usize,
) -> usize {
    ((craftsman * num_tasks) + task) * num_periods + period
}

fn timetable_design_toy_problem() -> TimetableDesign {
    TimetableDesign::new(
        2,
        2,
        2,
        vec![vec![true, false], vec![true, true]],
        vec![vec![true, true], vec![false, true]],
        vec![vec![1, 0], vec![0, 1]],
    )
}

#[test]
fn test_timetable_design_creation_and_dims() {
    let problem = timetable_design_toy_problem();

    assert_eq!(problem.num_periods(), 2);
    assert_eq!(problem.num_craftsmen(), 2);
    assert_eq!(problem.num_tasks(), 2);
    assert_eq!(
        problem.craftsman_avail(),
        &[vec![true, false], vec![true, true]]
    );
    assert_eq!(problem.task_avail(), &[vec![true, true], vec![false, true]]);
    assert_eq!(problem.requirements(), &[vec![1, 0], vec![0, 1]]);
    assert_eq!(problem.dims(), vec![2; 8]);
}

#[test]
fn test_timetable_design_problem_name_and_variant() {
    assert_eq!(<TimetableDesign as Problem>::NAME, "TimetableDesign");
    assert!(<TimetableDesign as Problem>::variant().is_empty());
}

#[test]
fn test_timetable_design_evaluate_valid_config() {
    let problem = timetable_design_toy_problem();
    let config = vec![1, 0, 0, 0, 0, 0, 0, 1];

    assert!(problem.evaluate(&config));
}

#[test]
fn test_timetable_design_rejects_wrong_config_length() {
    let problem = timetable_design_toy_problem();

    assert!(!problem.evaluate(&[1, 0, 0]));
    assert!(!problem.evaluate(&[0; 9]));
}

#[test]
fn test_timetable_design_rejects_assignment_outside_availability() {
    let problem = timetable_design_toy_problem();
    let config = vec![0, 1, 0, 0, 0, 0, 0, 1];

    assert!(!problem.evaluate(&config));
}

#[test]
fn test_timetable_design_rejects_double_booked_craftsman() {
    let problem = timetable_design_toy_problem();
    let config = vec![1, 0, 0, 0, 0, 1, 0, 1];

    assert!(!problem.evaluate(&config));
}

#[test]
fn test_timetable_design_rejects_double_booked_task() {
    let problem = timetable_design_toy_problem();
    let config = vec![1, 0, 0, 0, 1, 0, 0, 1];

    assert!(!problem.evaluate(&config));
}

#[test]
fn test_timetable_design_rejects_requirement_mismatch() {
    let problem = timetable_design_toy_problem();
    let config = vec![1, 0, 0, 0, 0, 0, 0, 0];

    assert!(!problem.evaluate(&config));
}

#[test]
fn test_timetable_design_bruteforce_solver_finds_solution() {
    let problem = timetable_design_toy_problem();
    let solution = BruteForce::new().find_satisfying(&problem);

    assert!(solution.is_some());
    assert!(problem.evaluate(&solution.unwrap()));
}

#[test]
fn test_timetable_design_serialization_round_trip() {
    let problem = timetable_design_toy_problem();

    let json = serde_json::to_value(&problem).unwrap();
    let restored: TimetableDesign = serde_json::from_value(json).unwrap();

    assert_eq!(restored.num_periods(), problem.num_periods());
    assert_eq!(restored.num_craftsmen(), problem.num_craftsmen());
    assert_eq!(restored.num_tasks(), problem.num_tasks());
    assert_eq!(restored.craftsman_avail(), problem.craftsman_avail());
    assert_eq!(restored.task_avail(), problem.task_avail());
    assert_eq!(restored.requirements(), problem.requirements());
}

#[test]
fn test_timetable_design_issue_example_is_valid() {
    let problem = super::issue_example_problem();
    let config = super::issue_example_config();

    assert!(problem.evaluate(&config));
}

#[test]
fn test_timetable_design_issue_example_rejects_flipped_required_assignment() {
    let problem = super::issue_example_problem();
    let mut config = super::issue_example_config();
    let forced = timetable_design_flat_index(problem.num_tasks(), problem.num_periods(), 1, 1, 1);
    config[forced] = 0;

    assert!(!problem.evaluate(&config));
}

#[test]
fn test_timetable_design_issue_example_rejects_conflicting_assignment() {
    let problem = super::issue_example_problem();
    let mut config = super::issue_example_config();
    let conflicting =
        timetable_design_flat_index(problem.num_tasks(), problem.num_periods(), 4, 0, 0);
    config[conflicting] = 1;

    assert!(!problem.evaluate(&config));
}

#[cfg(feature = "example-db")]
#[test]
fn test_timetable_design_paper_example_is_valid() {
    let specs = super::canonical_model_example_specs();
    assert_eq!(specs.len(), 1);

    let spec = &specs[0];
    assert_eq!(spec.id, "timetable_design");
    assert_eq!(spec.optimal_config, super::issue_example_config());
    assert_eq!(
        spec.instance.serialize_json(),
        serde_json::to_value(super::issue_example_problem()).unwrap()
    );
    assert_eq!(
        spec.instance.evaluate_json(&spec.optimal_config),
        serde_json::json!(true)
    );
    assert_eq!(spec.optimal_value, serde_json::json!(true));
}
