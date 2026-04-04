use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_sequencing_to_minimize_tardy_task_weight_basic() {
    let problem = SequencingToMinimizeTardyTaskWeight::new(
        vec![3, 2, 4, 1, 2],
        vec![5, 3, 7, 2, 4],
        vec![6, 4, 10, 2, 8],
    );

    assert_eq!(problem.num_tasks(), 5);
    assert_eq!(problem.lengths(), &[3, 2, 4, 1, 2]);
    assert_eq!(problem.weights(), &[5, 3, 7, 2, 4]);
    assert_eq!(problem.deadlines(), &[6, 4, 10, 2, 8]);
    assert_eq!(problem.dims(), vec![5, 5, 5, 5, 5]);
    assert_eq!(
        <SequencingToMinimizeTardyTaskWeight as Problem>::NAME,
        "SequencingToMinimizeTardyTaskWeight"
    );
    assert_eq!(
        <SequencingToMinimizeTardyTaskWeight as Problem>::variant(),
        vec![]
    );
}

#[test]
fn test_sequencing_to_minimize_tardy_task_weight_evaluate_issue_example() {
    let problem = SequencingToMinimizeTardyTaskWeight::new(
        vec![3, 2, 4, 1, 2],
        vec![5, 3, 7, 2, 4],
        vec![6, 4, 10, 2, 8],
    );

    // Schedule [3,0,4,2,1] = t3,t0,t4,t2,t1
    // t3: completes at 1, deadline=2, on time
    // t0: completes at 1+3=4, deadline=6, on time
    // t4: completes at 4+2=6, deadline=8, on time
    // t2: completes at 6+4=10, deadline=10, on time
    // t1: completes at 10+2=12, deadline=4, TARDY weight=3
    // Total = 3
    assert_eq!(problem.evaluate(&[3, 0, 4, 2, 1]), Min(Some(3)));
}

#[test]
fn test_sequencing_to_minimize_tardy_task_weight_evaluate_all_on_time() {
    // Single task with generous deadline
    let problem = SequencingToMinimizeTardyTaskWeight::new(vec![2, 3], vec![5, 4], vec![10, 10]);
    // Both orders: no task is tardy
    assert_eq!(problem.evaluate(&[0, 1]), Min(Some(0)));
    assert_eq!(problem.evaluate(&[1, 0]), Min(Some(0)));
}

#[test]
fn test_sequencing_to_minimize_tardy_task_weight_evaluate_all_tardy() {
    // Tight deadlines: every task is tardy regardless of order (3 tasks, total length=9)
    let problem =
        SequencingToMinimizeTardyTaskWeight::new(vec![3, 3, 3], vec![1, 2, 3], vec![2, 2, 2]);
    // [0,1,2]: t0 completes 3>2 tardy(1), t1 completes 6>2 tardy(2), t2 completes 9>2 tardy(3) = 6
    assert_eq!(problem.evaluate(&[0, 1, 2]), Min(Some(6)));
}

#[test]
fn test_sequencing_to_minimize_tardy_task_weight_evaluate_invalid_config() {
    let problem =
        SequencingToMinimizeTardyTaskWeight::new(vec![2, 3, 1], vec![1, 2, 3], vec![5, 6, 7]);

    // Wrong length
    assert_eq!(problem.evaluate(&[0, 1]), Min(None));
    assert_eq!(problem.evaluate(&[0, 1, 2, 0]), Min(None));
    // Not a permutation (duplicate)
    assert_eq!(problem.evaluate(&[0, 0, 1]), Min(None));
    // Out of range
    assert_eq!(problem.evaluate(&[0, 1, 3]), Min(None));
}

#[test]
fn test_sequencing_to_minimize_tardy_task_weight_brute_force_small() {
    // 3 tasks so brute force is fast (3^3 = 27 configs)
    let problem =
        SequencingToMinimizeTardyTaskWeight::new(vec![3, 2, 1], vec![4, 2, 3], vec![4, 3, 6]);
    let solver = BruteForce::new();
    let solution = solver
        .find_witness(&problem)
        .expect("should find a solution");
    let value = problem.evaluate(&solution);
    assert!(value.is_valid());

    // Check it's truly optimal by brute-forcing all permutations
    let permutations: Vec<Vec<usize>> = vec![
        vec![0, 1, 2],
        vec![0, 2, 1],
        vec![1, 0, 2],
        vec![1, 2, 0],
        vec![2, 0, 1],
        vec![2, 1, 0],
    ];
    let best = permutations
        .iter()
        .filter_map(|perm| problem.evaluate(perm).0)
        .min()
        .unwrap();
    assert_eq!(value, Min(Some(best)));
}

#[test]
fn test_sequencing_to_minimize_tardy_task_weight_paper_example() {
    let problem = SequencingToMinimizeTardyTaskWeight::new(
        vec![3, 2, 4, 1, 2],
        vec![5, 3, 7, 2, 4],
        vec![6, 4, 10, 2, 8],
    );
    let expected_config = vec![3, 0, 4, 2, 1];
    assert_eq!(problem.evaluate(&expected_config), Min(Some(3)));

    let solver = BruteForce::new();
    let solution = solver
        .find_witness(&problem)
        .expect("should find a solution");
    assert_eq!(problem.evaluate(&solution), Min(Some(3)));
}

#[test]
fn test_sequencing_to_minimize_tardy_task_weight_serialization() {
    let problem =
        SequencingToMinimizeTardyTaskWeight::new(vec![3, 2, 1], vec![4, 2, 3], vec![4, 3, 6]);
    let json = serde_json::to_value(&problem).unwrap();
    let restored: SequencingToMinimizeTardyTaskWeight = serde_json::from_value(json).unwrap();

    assert_eq!(restored.lengths(), problem.lengths());
    assert_eq!(restored.weights(), problem.weights());
    assert_eq!(restored.deadlines(), problem.deadlines());
}

#[test]
fn test_sequencing_to_minimize_tardy_task_weight_deserialization_rejects_zero_length() {
    let err = serde_json::from_value::<SequencingToMinimizeTardyTaskWeight>(serde_json::json!({
        "lengths": [0, 1, 3],
        "weights": [1, 2, 3],
        "deadlines": [5, 5, 5],
    }))
    .unwrap_err();
    assert!(err.to_string().contains("task lengths must be positive"));
}

#[test]
fn test_sequencing_to_minimize_tardy_task_weight_deserialization_rejects_zero_weight() {
    let err = serde_json::from_value::<SequencingToMinimizeTardyTaskWeight>(serde_json::json!({
        "lengths": [1, 2, 3],
        "weights": [0, 2, 3],
        "deadlines": [5, 5, 5],
    }))
    .unwrap_err();
    assert!(err.to_string().contains("task weights must be positive"));
}

#[test]
fn test_sequencing_to_minimize_tardy_task_weight_single_task() {
    let problem = SequencingToMinimizeTardyTaskWeight::new(vec![3], vec![2], vec![5]);
    assert_eq!(problem.dims(), vec![1]);
    // completes at 3, deadline 5, on time
    assert_eq!(problem.evaluate(&[0]), Min(Some(0)));
}

#[test]
fn test_sequencing_to_minimize_tardy_task_weight_single_task_tardy() {
    let problem = SequencingToMinimizeTardyTaskWeight::new(vec![3], vec![2], vec![2]);
    // completes at 3, deadline 2, tardy, weight 2
    assert_eq!(problem.evaluate(&[0]), Min(Some(2)));
}

#[test]
fn test_sequencing_to_minimize_tardy_task_weight_empty() {
    let problem = SequencingToMinimizeTardyTaskWeight::new(vec![], vec![], vec![]);
    assert_eq!(problem.num_tasks(), 0);
    assert_eq!(problem.dims(), Vec::<usize>::new());
    assert_eq!(problem.evaluate(&[]), Min(Some(0)));
}

#[test]
#[should_panic(expected = "lengths length must equal weights length")]
fn test_sequencing_to_minimize_tardy_task_weight_mismatched_lengths_weights() {
    SequencingToMinimizeTardyTaskWeight::new(vec![2, 1], vec![3], vec![5, 5]);
}

#[test]
#[should_panic(expected = "lengths length must equal deadlines length")]
fn test_sequencing_to_minimize_tardy_task_weight_mismatched_lengths_deadlines() {
    SequencingToMinimizeTardyTaskWeight::new(vec![2, 1], vec![3, 4], vec![5]);
}

#[test]
#[should_panic(expected = "task lengths must be positive")]
fn test_sequencing_to_minimize_tardy_task_weight_zero_length() {
    SequencingToMinimizeTardyTaskWeight::new(vec![0, 1], vec![2, 3], vec![5, 5]);
}
