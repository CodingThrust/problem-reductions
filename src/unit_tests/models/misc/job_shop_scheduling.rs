use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Or;

fn issue_example() -> JobShopScheduling {
    JobShopScheduling::new(
        2,
        vec![
            vec![(0, 3), (1, 4)],
            vec![(1, 2), (0, 3), (1, 2)],
            vec![(0, 4), (1, 3)],
            vec![(1, 5), (0, 2)],
            vec![(0, 2), (1, 3), (0, 1)],
        ],
        20,
    )
}

fn small_two_job_instance() -> JobShopScheduling {
    JobShopScheduling::new(2, vec![vec![(0, 1), (1, 1)], vec![(1, 1), (0, 1)]], 2)
}

#[test]
fn test_job_shop_scheduling_creation_and_dims() {
    let problem = issue_example();
    assert_eq!(problem.num_processors(), 2);
    assert_eq!(problem.num_jobs(), 5);
    assert_eq!(problem.num_tasks(), 12);
    assert_eq!(problem.dims(), vec![6, 5, 4, 3, 2, 1, 6, 5, 4, 3, 2, 1]);
}

#[test]
fn test_job_shop_scheduling_evaluate_issue_example() {
    let problem = issue_example();
    let config = vec![0, 0, 0, 0, 0, 0, 1, 3, 0, 1, 1, 0];
    assert_eq!(problem.evaluate(&config), Or(true));
}

#[test]
fn test_job_shop_scheduling_paper_example_schedule() {
    let problem = issue_example();
    let config = vec![0, 0, 0, 0, 0, 0, 1, 3, 0, 1, 1, 0];
    let start_times = problem.schedule_from_config(&config).unwrap();
    assert_eq!(start_times, vec![0, 7, 0, 3, 17, 6, 11, 2, 10, 12, 14, 17]);

    let makespan = start_times
        .iter()
        .zip(
            problem
                .jobs()
                .iter()
                .flat_map(|job| job.iter().map(|(_, length)| *length)),
        )
        .map(|(&start, length)| start + length)
        .max()
        .unwrap();
    assert_eq!(makespan, 19);
}

#[test]
fn test_job_shop_scheduling_rejects_cyclic_machine_orders() {
    let problem = small_two_job_instance();
    let config = vec![1, 0, 0, 0];
    assert_eq!(problem.evaluate(&config), Or(false));
}

#[test]
fn test_job_shop_scheduling_invalid_config_and_serialization() {
    let problem = small_two_job_instance();
    assert_eq!(problem.evaluate(&[2, 0, 0, 0]), Or(false));
    assert_eq!(problem.evaluate(&[0, 0, 0]), Or(false));

    let json = serde_json::to_value(&problem).unwrap();
    let restored: JobShopScheduling = serde_json::from_value(json).unwrap();
    assert_eq!(restored.num_processors(), problem.num_processors());
    assert_eq!(restored.jobs(), problem.jobs());
    assert_eq!(restored.deadline(), problem.deadline());
}

#[test]
fn test_job_shop_scheduling_problem_name_and_variant() {
    assert_eq!(<JobShopScheduling as Problem>::NAME, "JobShopScheduling");
    assert!(<JobShopScheduling as Problem>::variant().is_empty());
}

#[test]
fn test_job_shop_scheduling_brute_force_solver_small_instance() {
    let problem = small_two_job_instance();
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem);
    assert!(solution.is_some());
    assert_eq!(problem.evaluate(&solution.unwrap()), Or(true));
}
