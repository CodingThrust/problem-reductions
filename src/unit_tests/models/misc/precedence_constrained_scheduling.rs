use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_precedence_constrained_scheduling_basic() {
    let problem = PrecedenceConstrainedScheduling::new(4, 2, 3, vec![(0, 2), (1, 3)]);
    assert_eq!(problem.num_tasks(), 4);
    assert_eq!(problem.num_processors(), 2);
    assert_eq!(problem.deadline(), 3);
    assert_eq!(problem.precedences(), &[(0, 2), (1, 3)]);
    assert_eq!(problem.dims(), vec![3; 4]);
    assert_eq!(
        <PrecedenceConstrainedScheduling as Problem>::NAME,
        "PrecedenceConstrainedScheduling"
    );
    assert_eq!(
        <PrecedenceConstrainedScheduling as Problem>::variant(),
        vec![]
    );
}

#[test]
fn test_precedence_constrained_scheduling_evaluate_valid() {
    // Issue example: 8 tasks, 3 processors, deadline 4
    // Precedences: 0<2, 0<3, 1<3, 1<4, 2<5, 3<6, 4<6, 5<7, 6<7
    let problem = PrecedenceConstrainedScheduling::new(
        8,
        3,
        4,
        vec![
            (0, 2),
            (0, 3),
            (1, 3),
            (1, 4),
            (2, 5),
            (3, 6),
            (4, 6),
            (5, 7),
            (6, 7),
        ],
    );
    // Valid schedule: slot 0: {t0, t1}, slot 1: {t2, t3, t4}, slot 2: {t5, t6}, slot 3: {t7}
    let config = vec![0, 0, 1, 1, 1, 2, 2, 3];
    assert!(problem.evaluate(&config));
}

#[test]
fn test_precedence_constrained_scheduling_evaluate_invalid_precedence() {
    // t0 < t1, but we assign both to slot 0
    let problem = PrecedenceConstrainedScheduling::new(2, 2, 3, vec![(0, 1)]);
    assert!(!problem.evaluate(&[0, 0])); // slot[1] = 0 < slot[0] + 1 = 1
}

#[test]
fn test_precedence_constrained_scheduling_evaluate_invalid_capacity() {
    // 3 tasks, 2 processors, all in slot 0
    let problem = PrecedenceConstrainedScheduling::new(3, 2, 2, vec![]);
    assert!(!problem.evaluate(&[0, 0, 0])); // 3 tasks in slot 0, capacity 2
}

#[test]
fn test_precedence_constrained_scheduling_evaluate_wrong_config_length() {
    let problem = PrecedenceConstrainedScheduling::new(3, 2, 3, vec![]);
    assert!(!problem.evaluate(&[0, 1]));
    assert!(!problem.evaluate(&[0, 1, 2, 0]));
}

#[test]
fn test_precedence_constrained_scheduling_evaluate_invalid_variable_value() {
    let problem = PrecedenceConstrainedScheduling::new(2, 2, 3, vec![]);
    assert!(!problem.evaluate(&[0, 3])); // 3 >= deadline=3
}

#[test]
fn test_precedence_constrained_scheduling_brute_force() {
    // Small instance: 3 tasks, 2 processors, deadline 2, t0 < t2
    let problem = PrecedenceConstrainedScheduling::new(3, 2, 2, vec![(0, 2)]);
    let solver = BruteForce::new();
    let solution = solver
        .find_witness(&problem)
        .expect("should find a solution");
    assert!(problem.evaluate(&solution));
}

#[test]
fn test_precedence_constrained_scheduling_brute_force_all() {
    let problem = PrecedenceConstrainedScheduling::new(3, 2, 2, vec![(0, 2)]);
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem);
    assert!(!solutions.is_empty());
    for sol in &solutions {
        assert!(problem.evaluate(sol));
    }
}

#[test]
fn test_precedence_constrained_scheduling_unsatisfiable() {
    // 3 tasks in a chain t0 < t1 < t2, but only deadline 2 (need 3 slots)
    let problem = PrecedenceConstrainedScheduling::new(3, 1, 2, vec![(0, 1), (1, 2)]);
    let solver = BruteForce::new();
    assert!(solver.find_witness(&problem).is_none());
}

#[test]
fn test_precedence_constrained_scheduling_serialization() {
    let problem = PrecedenceConstrainedScheduling::new(4, 2, 3, vec![(0, 2), (1, 3)]);
    let json = serde_json::to_value(&problem).unwrap();
    let restored: PrecedenceConstrainedScheduling = serde_json::from_value(json).unwrap();
    assert_eq!(restored.num_tasks(), problem.num_tasks());
    assert_eq!(restored.num_processors(), problem.num_processors());
    assert_eq!(restored.deadline(), problem.deadline());
    assert_eq!(restored.precedences(), problem.precedences());
}

#[test]
fn test_precedence_constrained_scheduling_empty() {
    let problem = PrecedenceConstrainedScheduling::new(0, 1, 1, vec![]);
    assert_eq!(problem.num_tasks(), 0);
    assert_eq!(problem.dims(), Vec::<usize>::new());
    assert!(problem.evaluate(&[]));
}

#[test]
fn test_precedence_constrained_scheduling_no_precedences() {
    // 4 tasks, 2 processors, deadline 2, no precedences
    let problem = PrecedenceConstrainedScheduling::new(4, 2, 2, vec![]);
    // 2 tasks per slot, 2 slots = 4 tasks
    assert!(problem.evaluate(&[0, 0, 1, 1]));
    let solver = BruteForce::new();
    let solution = solver
        .find_witness(&problem)
        .expect("should find a solution");
    assert!(problem.evaluate(&solution));
}
