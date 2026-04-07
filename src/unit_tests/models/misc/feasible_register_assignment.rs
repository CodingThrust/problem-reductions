use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_feasible_register_assignment_basic() {
    let problem =
        FeasibleRegisterAssignment::new(4, vec![(0, 1), (0, 2), (1, 3)], 2, vec![0, 1, 0, 0]);
    assert_eq!(problem.num_vertices(), 4);
    assert_eq!(problem.num_arcs(), 3);
    assert_eq!(problem.num_registers(), 2);
    assert_eq!(problem.num_same_register_pairs(), 3);
    assert_eq!(problem.arcs(), &[(0, 1), (0, 2), (1, 3)]);
    assert_eq!(problem.assignment(), &[0, 1, 0, 0]);
    assert_eq!(problem.dims(), vec![4; 4]);
    assert_eq!(
        <FeasibleRegisterAssignment as Problem>::NAME,
        "FeasibleRegisterAssignment"
    );
    assert_eq!(<FeasibleRegisterAssignment as Problem>::variant(), vec![]);
}

#[test]
fn test_feasible_register_assignment_evaluate_valid() {
    // 4 vertices: v0 depends on v1 and v2, v1 depends on v3
    // K=2, assignment [0, 1, 0, 0]
    // Order: v3(pos0), v1(pos1), v2(pos2), v0(pos3)
    // config[v] = position
    let problem =
        FeasibleRegisterAssignment::new(4, vec![(0, 1), (0, 2), (1, 3)], 2, vec![0, 1, 0, 0]);
    let config = vec![3, 1, 2, 0];
    assert!(problem.evaluate(&config));
}

#[test]
fn test_feasible_register_assignment_evaluate_invalid_permutation() {
    let problem =
        FeasibleRegisterAssignment::new(4, vec![(0, 1), (0, 2), (1, 3)], 2, vec![0, 1, 0, 0]);
    // Not a permutation: position 0 used twice
    assert!(!problem.evaluate(&[0, 0, 1, 2]));
    // Wrong length
    assert!(!problem.evaluate(&[0, 1, 2]));
    assert!(!problem.evaluate(&[0, 1, 2, 3, 4]));
    // Position out of range
    assert!(!problem.evaluate(&[0, 1, 2, 4]));
}

#[test]
fn test_feasible_register_assignment_evaluate_invalid_dependency() {
    // v0 depends on v1, v1 depends on v3
    let problem =
        FeasibleRegisterAssignment::new(4, vec![(0, 1), (0, 2), (1, 3)], 2, vec![0, 1, 0, 0]);
    // v0 at position 0 but v1 at position 1 -> v0 evaluated before its dependency v1
    assert!(!problem.evaluate(&[0, 1, 2, 3]));
}

#[test]
fn test_feasible_register_assignment_register_conflict() {
    // Simple case: v0 depends on v1, v2 depends on v1
    // K=2, assignment [0, 0, 0] - all use register 0
    // In any valid topological order, v1 must come first.
    // After computing v1 (reg 0), v1 is live until both v0 and v2 are computed.
    // Computing v0 or v2 next would need register 0, but v1 is still live there.
    let problem = FeasibleRegisterAssignment::new(3, vec![(0, 1), (2, 1)], 2, vec![0, 0, 0]);
    // v1 at pos 0, v0 at pos 1, v2 at pos 2
    // After computing v1 (reg 0): v1 is live (v0, v2 still uncomputed)
    // Computing v0 (reg 0): conflict! v1 is still live in reg 0
    assert!(!problem.evaluate(&[1, 0, 2]));

    // With different assignment: v1->reg 1, v0->reg 0, v2->reg 0
    let problem2 = FeasibleRegisterAssignment::new(3, vec![(0, 1), (2, 1)], 2, vec![0, 1, 0]);
    // v1 at pos 0, v0 at pos 1, v2 at pos 2
    // After computing v1 (reg 1): v1 is live
    // Computing v0 (reg 0): no conflict, v0 uses reg 0
    // After v0 is computed, v1's only remaining dependent is v2
    // Computing v2 (reg 0): v1 is still live (v2 not computed yet)... but
    // v1 is in reg 1, v2 is in reg 0 => no conflict
    assert!(problem2.evaluate(&[1, 0, 2]));
}

#[test]
fn test_feasible_register_assignment_brute_force() {
    let problem =
        FeasibleRegisterAssignment::new(4, vec![(0, 1), (0, 2), (1, 3)], 2, vec![0, 1, 0, 0]);
    let solver = BruteForce::new();
    let solution = solver
        .find_witness(&problem)
        .expect("should find a solution");
    assert!(problem.evaluate(&solution));
}

#[test]
fn test_feasible_register_assignment_brute_force_all() {
    let problem =
        FeasibleRegisterAssignment::new(4, vec![(0, 1), (0, 2), (1, 3)], 2, vec![0, 1, 0, 0]);
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem);
    assert!(!solutions.is_empty());
    for sol in &solutions {
        assert!(problem.evaluate(sol));
    }
}

#[test]
fn test_feasible_register_assignment_unsatisfiable() {
    // v0 depends on v1 and v2, v1 depends on v2
    // All assigned to register 0 with K=1
    // v2 must be computed first. v2 is live (v0 and v1 depend on it).
    // Next must be v1 (since v0 depends on v1). But v2 is in reg 0
    // and v2 has uncomputed dependent v0 (excluding v1), so v2 is live.
    // Computing v1 in reg 0 conflicts with live v2.
    let problem =
        FeasibleRegisterAssignment::new(3, vec![(0, 1), (0, 2), (1, 2)], 1, vec![0, 0, 0]);
    let solver = BruteForce::new();
    assert!(solver.find_witness(&problem).is_none());
}

#[test]
fn test_feasible_register_assignment_serialization() {
    let problem =
        FeasibleRegisterAssignment::new(4, vec![(0, 1), (0, 2), (1, 3)], 2, vec![0, 1, 0, 0]);
    let json = serde_json::to_value(&problem).unwrap();
    let restored: FeasibleRegisterAssignment = serde_json::from_value(json).unwrap();
    assert_eq!(restored.num_vertices(), problem.num_vertices());
    assert_eq!(restored.num_arcs(), problem.num_arcs());
    assert_eq!(restored.num_registers(), problem.num_registers());
    assert_eq!(restored.arcs(), problem.arcs());
    assert_eq!(restored.assignment(), problem.assignment());
}

#[test]
fn test_feasible_register_assignment_empty() {
    let problem = FeasibleRegisterAssignment::new(0, vec![], 0, vec![]);
    assert_eq!(problem.num_vertices(), 0);
    assert_eq!(problem.dims(), Vec::<usize>::new());
    assert!(problem.evaluate(&[]));
}

#[test]
fn test_feasible_register_assignment_single_vertex() {
    let problem = FeasibleRegisterAssignment::new(1, vec![], 1, vec![0]);
    assert!(problem.evaluate(&[0]));
}

#[test]
fn test_feasible_register_assignment_no_dependencies() {
    // 3 vertices, no arcs, K=2, assignment [0, 1, 0]
    // Any permutation is valid as long as no register conflict.
    // v0(reg 0) and v2(reg 0): since there are no dependencies, no vertex is
    // ever "live" (no dependents), so no conflicts can arise.
    let problem = FeasibleRegisterAssignment::new(3, vec![], 2, vec![0, 1, 0]);
    // Any order works since no vertex has dependents => nothing is ever live
    assert!(problem.evaluate(&[0, 1, 2]));
    assert!(problem.evaluate(&[2, 1, 0]));
}

#[test]
fn test_feasible_register_assignment_same_register_pair_count() {
    let problem = FeasibleRegisterAssignment::new(5, vec![], 3, vec![0, 1, 0, 2, 0]);
    assert_eq!(problem.num_same_register_pairs(), 3);
}
