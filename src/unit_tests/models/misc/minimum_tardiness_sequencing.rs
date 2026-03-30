use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::One;

// ===== Unit-length variant (W = One) =====

#[test]
fn test_minimum_tardiness_sequencing_basic() {
    let problem = MinimumTardinessSequencing::<One>::new(
        5,
        vec![5, 5, 5, 3, 3],
        vec![(0, 3), (1, 3), (1, 4), (2, 4)],
    );
    assert_eq!(problem.num_tasks(), 5);
    assert_eq!(problem.deadlines(), &[5, 5, 5, 3, 3]);
    assert_eq!(problem.precedences(), &[(0, 3), (1, 3), (1, 4), (2, 4)]);
    assert_eq!(problem.num_precedences(), 4);
    assert_eq!(problem.dims(), vec![5, 4, 3, 2, 1]);
    assert_eq!(
        <MinimumTardinessSequencing<One> as Problem>::NAME,
        "MinimumTardinessSequencing"
    );
}

#[test]
fn test_minimum_tardiness_sequencing_evaluate_optimal() {
    let problem = MinimumTardinessSequencing::<One>::new(
        5,
        vec![5, 5, 5, 3, 3],
        vec![(0, 3), (1, 3), (1, 4), (2, 4)],
    );
    let config = vec![0, 0, 1, 0, 0];
    assert_eq!(problem.evaluate(&config), Min(Some(1)));
}

#[test]
fn test_minimum_tardiness_sequencing_evaluate_invalid_lehmer() {
    let problem = MinimumTardinessSequencing::<One>::new(3, vec![2, 3, 1], vec![]);
    assert_eq!(problem.evaluate(&[0, 2, 0]), Min(None));
}

#[test]
fn test_minimum_tardiness_sequencing_evaluate_out_of_range() {
    let problem = MinimumTardinessSequencing::<One>::new(3, vec![2, 3, 1], vec![]);
    assert_eq!(problem.evaluate(&[0, 1, 5]), Min(None));
}

#[test]
fn test_minimum_tardiness_sequencing_evaluate_wrong_length() {
    let problem = MinimumTardinessSequencing::<One>::new(3, vec![2, 3, 1], vec![]);
    assert_eq!(problem.evaluate(&[0, 1]), Min(None));
    assert_eq!(problem.evaluate(&[0, 1, 2, 3]), Min(None));
}

#[test]
fn test_minimum_tardiness_sequencing_evaluate_precedence_violation() {
    let problem = MinimumTardinessSequencing::<One>::new(3, vec![3, 3, 3], vec![(0, 1)]);
    assert_eq!(problem.evaluate(&[0, 0, 0]), Min(Some(0)));
    assert_eq!(problem.evaluate(&[1, 0, 0]), Min(None));
    assert_eq!(problem.evaluate(&[2, 1, 0]), Min(None));
}

#[test]
fn test_minimum_tardiness_sequencing_evaluate_all_on_time() {
    let problem = MinimumTardinessSequencing::<One>::new(3, vec![3, 3, 3], vec![]);
    assert_eq!(problem.evaluate(&[0, 0, 0]), Min(Some(0)));
    assert_eq!(problem.evaluate(&[2, 1, 0]), Min(Some(0)));
}

#[test]
fn test_minimum_tardiness_sequencing_evaluate_all_tardy() {
    let problem = MinimumTardinessSequencing::<One>::new(2, vec![0, 0], vec![]);
    assert_eq!(problem.evaluate(&[0, 0]), Min(Some(2)));
}

#[test]
fn test_minimum_tardiness_sequencing_brute_force() {
    let problem = MinimumTardinessSequencing::<One>::new(
        5,
        vec![5, 5, 5, 3, 3],
        vec![(0, 3), (1, 3), (1, 4), (2, 4)],
    );
    let solver = BruteForce::new();
    let solution = solver
        .find_witness(&problem)
        .expect("should find a solution");
    let metric = problem.evaluate(&solution);
    assert_eq!(metric, Min(Some(1)));
}

#[test]
fn test_minimum_tardiness_sequencing_brute_force_no_precedences() {
    let problem = MinimumTardinessSequencing::<One>::new(3, vec![1, 3, 2], vec![]);
    let solver = BruteForce::new();
    let solution = solver
        .find_witness(&problem)
        .expect("should find a solution");
    let metric = problem.evaluate(&solution);
    assert_eq!(metric, Min(Some(0)));
}

#[test]
fn test_minimum_tardiness_sequencing_serialization() {
    let problem = MinimumTardinessSequencing::<One>::new(3, vec![2, 3, 1], vec![(0, 1)]);
    let json = serde_json::to_value(&problem).unwrap();
    let restored: MinimumTardinessSequencing<One> = serde_json::from_value(json).unwrap();
    assert_eq!(restored.num_tasks(), problem.num_tasks());
    assert_eq!(restored.deadlines(), problem.deadlines());
    assert_eq!(restored.precedences(), problem.precedences());
}

#[test]
fn test_minimum_tardiness_sequencing_empty() {
    let problem = MinimumTardinessSequencing::<One>::new(0, vec![], vec![]);
    assert_eq!(problem.num_tasks(), 0);
    assert_eq!(problem.dims(), Vec::<usize>::new());
    assert_eq!(problem.evaluate(&[]), Min(Some(0)));
}

#[test]
fn test_minimum_tardiness_sequencing_single_task() {
    let problem = MinimumTardinessSequencing::<One>::new(1, vec![1], vec![]);
    assert_eq!(problem.dims(), vec![1]);
    assert_eq!(problem.evaluate(&[0]), Min(Some(0)));

    let problem_tardy = MinimumTardinessSequencing::<One>::new(1, vec![0], vec![]);
    assert_eq!(problem_tardy.evaluate(&[0]), Min(Some(1)));
}

#[test]
#[should_panic(expected = "deadlines length must equal num_tasks")]
fn test_minimum_tardiness_sequencing_mismatched_deadlines() {
    MinimumTardinessSequencing::<One>::new(3, vec![1, 2], vec![]);
}

#[test]
#[should_panic(expected = "predecessor index 5 out of range")]
fn test_minimum_tardiness_sequencing_invalid_precedence() {
    MinimumTardinessSequencing::<One>::new(3, vec![1, 2, 3], vec![(5, 0)]);
}

#[test]
fn test_minimum_tardiness_sequencing_cyclic_precedences() {
    let problem =
        MinimumTardinessSequencing::<One>::new(3, vec![3, 3, 3], vec![(0, 1), (1, 2), (2, 0)]);
    let solver = BruteForce::new();
    assert!(solver.find_witness(&problem).is_none());
}

// ===== Arbitrary-length variant (W = i32) =====

#[test]
fn test_minimum_tardiness_sequencing_weighted_basic() {
    let problem = MinimumTardinessSequencing::<i32>::with_lengths(
        vec![3, 2, 2, 1, 2],
        vec![4, 3, 8, 3, 6],
        vec![(0, 2), (1, 3)],
    );
    assert_eq!(problem.num_tasks(), 5);
    assert_eq!(problem.lengths(), &[3, 2, 2, 1, 2]);
    assert_eq!(problem.deadlines(), &[4, 3, 8, 3, 6]);
    assert_eq!(problem.num_precedences(), 2);
}

#[test]
fn test_minimum_tardiness_sequencing_weighted_evaluate() {
    // Issue example: 5 tasks, lengths [3,2,2,1,2], deadlines [4,3,8,3,6], prec (0→2, 1→3)
    // Schedule: t0,t4,t2,t1,t3
    // Lehmer [0,3,1,0,0] -> schedule [0,4,2,1,3]
    let problem = MinimumTardinessSequencing::<i32>::with_lengths(
        vec![3, 2, 2, 1, 2],
        vec![4, 3, 8, 3, 6],
        vec![(0, 2), (1, 3)],
    );
    // t0(l=3): finish=3, deadline=4 → on time
    // t4(l=2): finish=5, deadline=6 → on time
    // t2(l=2): finish=7, deadline=8 → on time
    // t1(l=2): finish=9, deadline=3 → tardy
    // t3(l=1): finish=10, deadline=3 → tardy
    assert_eq!(problem.evaluate(&[0, 3, 1, 0, 0]), Min(Some(2)));
}

#[test]
fn test_minimum_tardiness_sequencing_weighted_brute_force() {
    let problem = MinimumTardinessSequencing::<i32>::with_lengths(
        vec![3, 2, 2, 1, 2],
        vec![4, 3, 8, 3, 6],
        vec![(0, 2), (1, 3)],
    );
    let solver = BruteForce::new();
    let solution = solver
        .find_witness(&problem)
        .expect("should find a solution");
    let metric = problem.evaluate(&solution);
    assert_eq!(metric, Min(Some(2)));
}

#[test]
fn test_minimum_tardiness_sequencing_weighted_serialization() {
    let problem =
        MinimumTardinessSequencing::<i32>::with_lengths(vec![3, 2, 2], vec![4, 3, 8], vec![(0, 1)]);
    let json = serde_json::to_value(&problem).unwrap();
    let restored: MinimumTardinessSequencing<i32> = serde_json::from_value(json).unwrap();
    assert_eq!(restored.num_tasks(), problem.num_tasks());
    assert_eq!(restored.lengths(), problem.lengths());
    assert_eq!(restored.deadlines(), problem.deadlines());
}

#[test]
fn test_minimum_tardiness_sequencing_weighted_different_lengths() {
    // 3 tasks: lengths [1,5,1], deadlines [2,6,3]
    // Schedule [0,2,1]: t0(l=1,fin=1≤2✓), t2(l=1,fin=2≤3✓), t1(l=5,fin=7>6✗) → 1 tardy
    // Schedule [0,1,2]: t0(l=1,fin=1≤2✓), t1(l=5,fin=6≤6✓), t2(l=1,fin=7>3✗) → 1 tardy
    // Schedule [1,0,2]: t1(l=5,fin=5≤6✓), t0(l=1,fin=6>2✗), t2(l=1,fin=7>3✗) → 2 tardy
    let problem =
        MinimumTardinessSequencing::<i32>::with_lengths(vec![1, 5, 1], vec![2, 6, 3], vec![]);
    let solver = BruteForce::new();
    let solution = solver
        .find_witness(&problem)
        .expect("should find a solution");
    assert_eq!(problem.evaluate(&solution), Min(Some(1)));
}

#[test]
#[should_panic(expected = "all task lengths must be positive")]
fn test_minimum_tardiness_sequencing_weighted_zero_length() {
    MinimumTardinessSequencing::<i32>::with_lengths(vec![1, 0, 2], vec![3, 3, 3], vec![]);
}

#[test]
fn test_minimum_tardiness_sequencing_paper_example() {
    // Issue example (unit-length): 4 tasks, deadlines [2,3,1,4], prec (0→2)
    // Lehmer [0,0,0,0] = schedule [0,1,2,3]
    // t0: finish=1≤2✓, t1: finish=2≤3✓, t2: finish=3>1✗, t3: finish=4≤4✓ → 1 tardy
    let problem = MinimumTardinessSequencing::<One>::new(4, vec![2, 3, 1, 4], vec![(0, 2)]);
    assert_eq!(problem.evaluate(&[0, 0, 0, 0]), Min(Some(1)));
}
