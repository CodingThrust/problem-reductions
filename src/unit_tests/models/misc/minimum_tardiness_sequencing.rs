use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::Direction;

#[test]
fn test_minimum_tardiness_sequencing_basic() {
    let problem = MinimumTardinessSequencing::new(
        5,
        vec![5, 5, 5, 3, 3],
        vec![(0, 3), (1, 3), (1, 4), (2, 4)],
    );
    assert_eq!(problem.num_tasks(), 5);
    assert_eq!(problem.deadlines(), &[5, 5, 5, 3, 3]);
    assert_eq!(problem.precedences(), &[(0, 3), (1, 3), (1, 4), (2, 4)]);
    assert_eq!(problem.num_precedences(), 4);
    assert_eq!(problem.dims(), vec![5, 4, 3, 2, 1]);
    assert_eq!(problem.direction(), Direction::Minimize);
    assert_eq!(
        <MinimumTardinessSequencing as Problem>::NAME,
        "MinimumTardinessSequencing"
    );
    assert_eq!(<MinimumTardinessSequencing as Problem>::variant(), vec![]);
}

#[test]
fn test_minimum_tardiness_sequencing_evaluate_optimal() {
    // Example from issue: 5 tasks, optimal has 1 tardy task
    let problem = MinimumTardinessSequencing::new(
        5,
        vec![5, 5, 5, 3, 3],
        vec![(0, 3), (1, 3), (1, 4), (2, 4)],
    );
    // Lehmer code [0,0,1,0,0] decodes to schedule [0,1,3,2,4]:
    // available=[0,1,2,3,4] pick idx 0 -> 0; available=[1,2,3,4] pick idx 0 -> 1;
    // available=[2,3,4] pick idx 1 -> 3; available=[2,4] pick idx 0 -> 2; available=[4] pick idx 0 -> 4.
    // sigma: task 0 at pos 0, task 1 at pos 1, task 3 at pos 2, task 2 at pos 3, task 4 at pos 4.
    // t0 finishes at 1 <= 5, t1 at 2 <= 5, t3 at 3 <= 3, t2 at 4 <= 5, t4 at 5 > 3 (tardy)
    let config = vec![0, 0, 1, 0, 0];
    assert_eq!(problem.evaluate(&config), SolutionSize::Valid(1));
}

#[test]
fn test_minimum_tardiness_sequencing_evaluate_invalid_lehmer() {
    let problem = MinimumTardinessSequencing::new(3, vec![2, 3, 1], vec![]);
    // dims = [3, 2, 1]; config [0, 2, 0] has 2 >= 2 (second dim), invalid Lehmer code
    assert_eq!(problem.evaluate(&[0, 2, 0]), SolutionSize::Invalid);
}

#[test]
fn test_minimum_tardiness_sequencing_evaluate_out_of_range() {
    let problem = MinimumTardinessSequencing::new(3, vec![2, 3, 1], vec![]);
    // dims = [3, 2, 1]; config [0, 1, 5] has 5 >= 1 (third dim), out of range
    assert_eq!(problem.evaluate(&[0, 1, 5]), SolutionSize::Invalid);
}

#[test]
fn test_minimum_tardiness_sequencing_evaluate_wrong_length() {
    let problem = MinimumTardinessSequencing::new(3, vec![2, 3, 1], vec![]);
    assert_eq!(problem.evaluate(&[0, 1]), SolutionSize::Invalid);
    assert_eq!(problem.evaluate(&[0, 1, 2, 3]), SolutionSize::Invalid);
}

#[test]
fn test_minimum_tardiness_sequencing_evaluate_precedence_violation() {
    let problem = MinimumTardinessSequencing::new(
        3,
        vec![3, 3, 3],
        vec![(0, 1)], // task 0 must precede task 1
    );
    // Lehmer [0,0,0] -> schedule [0,1,2] -> sigma [0,1,2]: sigma(0)=0 < sigma(1)=1, valid
    assert_eq!(problem.evaluate(&[0, 0, 0]), SolutionSize::Valid(0));
    // Lehmer [1,0,0] -> schedule [1,0,2] -> sigma [1,0,2]: sigma(0)=1 >= sigma(1)=0, violates
    assert_eq!(problem.evaluate(&[1, 0, 0]), SolutionSize::Invalid);
    // Lehmer [2,1,0] -> schedule [2,1,0] -> sigma [2,1,0]: sigma(0)=2 >= sigma(1)=1, violates
    assert_eq!(problem.evaluate(&[2, 1, 0]), SolutionSize::Invalid);
}

#[test]
fn test_minimum_tardiness_sequencing_evaluate_all_on_time() {
    let problem = MinimumTardinessSequencing::new(3, vec![3, 3, 3], vec![]);
    // All deadlines are 3, so any permutation of 3 tasks is on time
    // Lehmer [0,0,0] -> schedule [0,1,2]
    assert_eq!(problem.evaluate(&[0, 0, 0]), SolutionSize::Valid(0));
    // Lehmer [2,1,0] -> schedule [2,1,0]
    assert_eq!(problem.evaluate(&[2, 1, 0]), SolutionSize::Valid(0));
}

#[test]
fn test_minimum_tardiness_sequencing_evaluate_all_tardy() {
    // Deadlines are all 0 (impossible to meet since earliest finish is 1)
    // Wait: deadlines are usize and d(t)=0 means finish must be <= 0, but finish is at least 1
    // Actually, let's use deadlines that can't be met
    let problem = MinimumTardinessSequencing::new(2, vec![0, 0], vec![]);
    // Lehmer [0,0] -> schedule [0,1] -> sigma [0,1]
    // pos 0 finishes at 1 > 0 (tardy), pos 1 finishes at 2 > 0 (tardy)
    assert_eq!(problem.evaluate(&[0, 0]), SolutionSize::Valid(2));
}

#[test]
fn test_minimum_tardiness_sequencing_brute_force() {
    let problem = MinimumTardinessSequencing::new(
        5,
        vec![5, 5, 5, 3, 3],
        vec![(0, 3), (1, 3), (1, 4), (2, 4)],
    );
    let solver = BruteForce::new();
    let solution = solver.find_best(&problem).expect("should find a solution");
    let metric = problem.evaluate(&solution);
    // Optimal is 1 tardy task
    assert_eq!(metric, SolutionSize::Valid(1));
}

#[test]
fn test_minimum_tardiness_sequencing_brute_force_no_precedences() {
    // Without precedences, Moore's algorithm gives optimal
    // 3 tasks: deadlines 1, 3, 2. Best is to schedule task with deadline 1 first.
    let problem = MinimumTardinessSequencing::new(3, vec![1, 3, 2], vec![]);
    let solver = BruteForce::new();
    let solution = solver.find_best(&problem).expect("should find a solution");
    let metric = problem.evaluate(&solution);
    // All can be on time: t0 at pos 0 (finish 1 <= 1), t2 at pos 1 (finish 2 <= 2), t1 at pos 2 (finish 3 <= 3)
    assert_eq!(metric, SolutionSize::Valid(0));
}

#[test]
fn test_minimum_tardiness_sequencing_serialization() {
    let problem = MinimumTardinessSequencing::new(3, vec![2, 3, 1], vec![(0, 1)]);
    let json = serde_json::to_value(&problem).unwrap();
    let restored: MinimumTardinessSequencing = serde_json::from_value(json).unwrap();
    assert_eq!(restored.num_tasks(), problem.num_tasks());
    assert_eq!(restored.deadlines(), problem.deadlines());
    assert_eq!(restored.precedences(), problem.precedences());
}

#[test]
fn test_minimum_tardiness_sequencing_empty() {
    let problem = MinimumTardinessSequencing::new(0, vec![], vec![]);
    assert_eq!(problem.num_tasks(), 0);
    assert_eq!(problem.dims(), Vec::<usize>::new());
    assert_eq!(problem.evaluate(&[]), SolutionSize::Valid(0));
}

#[test]
fn test_minimum_tardiness_sequencing_single_task() {
    let problem = MinimumTardinessSequencing::new(1, vec![1], vec![]);
    assert_eq!(problem.dims(), vec![1]);
    // Task at position 0, finishes at 1 <= 1, not tardy
    assert_eq!(problem.evaluate(&[0]), SolutionSize::Valid(0));

    let problem_tardy = MinimumTardinessSequencing::new(1, vec![0], vec![]);
    // Task at position 0, finishes at 1 > 0, tardy
    assert_eq!(problem_tardy.evaluate(&[0]), SolutionSize::Valid(1));
}

#[test]
#[should_panic(expected = "deadlines length must equal num_tasks")]
fn test_minimum_tardiness_sequencing_mismatched_deadlines() {
    MinimumTardinessSequencing::new(3, vec![1, 2], vec![]);
}

#[test]
#[should_panic(expected = "predecessor index 5 out of range")]
fn test_minimum_tardiness_sequencing_invalid_precedence() {
    MinimumTardinessSequencing::new(3, vec![1, 2, 3], vec![(5, 0)]);
}

#[test]
fn test_minimum_tardiness_sequencing_cyclic_precedences() {
    // Cyclic precedences: 0 -> 1 -> 2 -> 0. No valid schedule exists.
    let problem = MinimumTardinessSequencing::new(
        3,
        vec![3, 3, 3],
        vec![(0, 1), (1, 2), (2, 0)],
    );
    let solver = BruteForce::new();
    assert!(solver.find_best(&problem).is_none());
}
