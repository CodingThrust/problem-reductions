use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_creation() {
    let problem = MinimumRegisterSufficiencyForLoops::new(6, vec![(0, 3), (2, 3), (4, 3)]);
    assert_eq!(problem.loop_length(), 6);
    assert_eq!(problem.num_variables(), 3);
    assert_eq!(problem.variables(), &[(0, 3), (2, 3), (4, 3)]);
    assert_eq!(problem.dims(), vec![3, 3, 3]);
}

#[test]
fn test_evaluate_optimal() {
    // K3 graph: all 3 vars conflict, need 3 registers
    let problem = MinimumRegisterSufficiencyForLoops::new(6, vec![(0, 3), (2, 3), (4, 3)]);
    let result = problem.evaluate(&[0, 1, 2]);
    assert_eq!(result, Min(Some(3)));
}

#[test]
fn test_evaluate_conflict() {
    // Two overlapping vars assigned same register => conflict
    let problem = MinimumRegisterSufficiencyForLoops::new(6, vec![(0, 3), (2, 3), (4, 3)]);
    let result = problem.evaluate(&[0, 0, 1]);
    // Vars 0 and 1 overlap (arcs [0,3) and [2,5)), same register 0 => invalid
    assert_eq!(result, Min(None));
}

#[test]
fn test_evaluate_non_overlapping() {
    // Two non-overlapping vars can share a register
    let problem = MinimumRegisterSufficiencyForLoops::new(6, vec![(0, 2), (3, 2)]);
    // Arcs [0,2) and [3,5) don't overlap
    let result = problem.evaluate(&[0, 0]);
    assert_eq!(result, Min(Some(1)));
}

#[test]
fn test_evaluate_all_different() {
    // Trivial assignment: all different registers
    let problem = MinimumRegisterSufficiencyForLoops::new(6, vec![(0, 3), (2, 3), (4, 3)]);
    let result = problem.evaluate(&[0, 1, 2]);
    assert_eq!(result, Min(Some(3)));
}

#[test]
fn test_evaluate_invalid_config_length() {
    let problem = MinimumRegisterSufficiencyForLoops::new(6, vec![(0, 3), (2, 3)]);
    let result = problem.evaluate(&[0]);
    assert_eq!(result, Min(None));
}

#[test]
fn test_evaluate_out_of_range_register() {
    let problem = MinimumRegisterSufficiencyForLoops::new(6, vec![(0, 3), (2, 3)]);
    let result = problem.evaluate(&[0, 5]); // 5 >= num_variables (2)
    assert_eq!(result, Min(None));
}

#[test]
fn test_solver_k3() {
    // All pairs conflict: need 3 registers
    let problem = MinimumRegisterSufficiencyForLoops::new(6, vec![(0, 3), (2, 3), (4, 3)]);
    let solver = BruteForce::new();
    let witness = solver.find_witness(&problem).unwrap();
    let value = problem.evaluate(&witness);
    assert_eq!(value, Min(Some(3)));
}

#[test]
fn test_solver_two_non_overlapping() {
    // Two non-overlapping arcs: can share 1 register
    let problem = MinimumRegisterSufficiencyForLoops::new(6, vec![(0, 2), (3, 2)]);
    let solver = BruteForce::new();
    let witness = solver.find_witness(&problem).unwrap();
    let value = problem.evaluate(&witness);
    assert_eq!(value, Min(Some(1)));
}

#[test]
fn test_solver_two_overlapping() {
    // Two overlapping arcs: need 2 registers
    let problem = MinimumRegisterSufficiencyForLoops::new(6, vec![(0, 4), (3, 4)]);
    let solver = BruteForce::new();
    let witness = solver.find_witness(&problem).unwrap();
    let value = problem.evaluate(&witness);
    assert_eq!(value, Min(Some(2)));
}

#[test]
fn test_circular_wrap_around_overlap() {
    // Arc (5, 3) on loop length 6 covers timesteps {5, 0, 1}
    // Arc (0, 3) covers timesteps {0, 1, 2}
    // They overlap at timesteps 0 and 1
    let problem = MinimumRegisterSufficiencyForLoops::new(6, vec![(5, 3), (0, 3)]);
    let result = problem.evaluate(&[0, 0]);
    assert_eq!(result, Min(None)); // conflict
    let result = problem.evaluate(&[0, 1]);
    assert_eq!(result, Min(Some(2)));
}

#[test]
fn test_single_variable() {
    let problem = MinimumRegisterSufficiencyForLoops::new(4, vec![(0, 2)]);
    let solver = BruteForce::new();
    let witness = solver.find_witness(&problem).unwrap();
    let value = problem.evaluate(&witness);
    assert_eq!(value, Min(Some(1)));
}

#[test]
fn test_serialization() {
    let problem = MinimumRegisterSufficiencyForLoops::new(6, vec![(0, 3), (2, 3), (4, 3)]);
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: MinimumRegisterSufficiencyForLoops = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.loop_length(), 6);
    assert_eq!(deserialized.num_variables(), 3);
    assert_eq!(deserialized.variables(), &[(0, 3), (2, 3), (4, 3)]);
}

#[test]
fn test_paper_example() {
    // Paper example: N=6, vars: (0,3), (2,3), (4,3) - all pairs conflict (K3)
    // Config [0,1,2] -> 3 registers -> Min(3) is optimal
    let problem = MinimumRegisterSufficiencyForLoops::new(6, vec![(0, 3), (2, 3), (4, 3)]);
    let config = vec![0, 1, 2];
    let result = problem.evaluate(&config);
    assert_eq!(result, Min(Some(3)));

    // Verify optimality with brute force
    let solver = BruteForce::new();
    let best = solver.find_witness(&problem).unwrap();
    assert_eq!(problem.evaluate(&best), Min(Some(3)));
}

#[test]
#[should_panic(expected = "loop_length must be positive")]
fn test_zero_loop_length_panics() {
    MinimumRegisterSufficiencyForLoops::new(0, vec![]);
}

#[test]
#[should_panic(expected = "duration")]
fn test_zero_duration_panics() {
    MinimumRegisterSufficiencyForLoops::new(6, vec![(0, 0)]);
}

#[test]
#[should_panic(expected = "start_time")]
fn test_invalid_start_time_panics() {
    MinimumRegisterSufficiencyForLoops::new(6, vec![(6, 2)]);
}
