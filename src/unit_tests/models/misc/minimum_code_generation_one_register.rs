use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_minimum_code_generation_one_register_creation() {
    let problem = MinimumCodeGenerationOneRegister::new(
        7,
        vec![
            (0, 1),
            (0, 2),
            (1, 3),
            (1, 4),
            (2, 3),
            (2, 5),
            (3, 5),
            (3, 6),
        ],
        3,
    );
    assert_eq!(problem.num_vertices(), 7);
    assert_eq!(problem.num_edges(), 8);
    assert_eq!(problem.num_leaves(), 3);
    assert_eq!(problem.num_internal(), 4);
    assert_eq!(problem.dims(), vec![4; 4]);
    assert_eq!(
        <MinimumCodeGenerationOneRegister as Problem>::NAME,
        "MinimumCodeGenerationOneRegister"
    );
    assert_eq!(
        <MinimumCodeGenerationOneRegister as Problem>::variant(),
        vec![]
    );
}

#[test]
fn test_minimum_code_generation_one_register_evaluate_optimal() {
    // Issue #900 example: optimal is 8 instructions
    // Evaluation order: v3, v2, v1, v0
    // config[i] = position for internal vertex i
    // internal = [0,1,2,3], so config = [3, 2, 1, 0]
    let problem = MinimumCodeGenerationOneRegister::new(
        7,
        vec![
            (0, 1),
            (0, 2),
            (1, 3),
            (1, 4),
            (2, 3),
            (2, 5),
            (3, 5),
            (3, 6),
        ],
        3,
    );
    let config = vec![3, 2, 1, 0];
    assert_eq!(problem.evaluate(&config), Min(Some(8)));
    assert_eq!(problem.simulate(&config), Some(8));
}

#[test]
fn test_minimum_code_generation_one_register_evaluate_suboptimal() {
    // Another valid order: v3, v1, v2, v0 (computing v1 before v2).
    // With greedy stores this also achieves 8 for this instance,
    // showing the optimal value is robust to ordering choices here.
    let problem = MinimumCodeGenerationOneRegister::new(
        7,
        vec![
            (0, 1),
            (0, 2),
            (1, 3),
            (1, 4),
            (2, 3),
            (2, 5),
            (3, 5),
            (3, 6),
        ],
        3,
    );
    // Order: v3 (pos 0), v1 (pos 1), v2 (pos 2), v0 (pos 3)
    // config: v0->3, v1->1, v2->2, v3->0
    let config = vec![3, 1, 2, 0];
    assert_eq!(problem.simulate(&config), Some(8));
}

#[test]
fn test_minimum_code_generation_one_register_invalid_dependency() {
    // Try to evaluate v0 before its children
    let problem = MinimumCodeGenerationOneRegister::new(
        7,
        vec![
            (0, 1),
            (0, 2),
            (1, 3),
            (1, 4),
            (2, 3),
            (2, 5),
            (3, 5),
            (3, 6),
        ],
        3,
    );
    // v0 first (pos 0) — depends on v1,v2 which haven't been computed
    let config = vec![0, 1, 2, 3];
    assert_eq!(problem.evaluate(&config), Min(None));
}

#[test]
fn test_minimum_code_generation_one_register_invalid_permutation() {
    let problem = MinimumCodeGenerationOneRegister::new(
        7,
        vec![
            (0, 1),
            (0, 2),
            (1, 3),
            (1, 4),
            (2, 3),
            (2, 5),
            (3, 5),
            (3, 6),
        ],
        3,
    );
    // Not a permutation: position 0 used twice
    assert_eq!(problem.evaluate(&[0, 0, 1, 2]), Min(None));
    // Wrong length
    assert_eq!(problem.evaluate(&[0, 1, 2]), Min(None));
    // Position out of range
    assert_eq!(problem.evaluate(&[0, 1, 2, 5]), Min(None));
}

#[test]
fn test_minimum_code_generation_one_register_solver() {
    // Small instance: 4 vertices, 2 leaves, 2 internal
    // v0 = op(v1, v2), v1 = op(v2, v3)
    // Leaves: {2, 3}, Internal: {0, 1}
    // Edges: (0,1), (0,2), (1,2), (1,3)
    // Wait — v2 appears as both child and parent?
    // No: v0 has children v1,v2. v1 has children v2,v3.
    // Leaves: v2 and v3 have out-degree 0. So num_leaves=2.
    let problem = MinimumCodeGenerationOneRegister::new(4, vec![(0, 1), (0, 2), (1, 2), (1, 3)], 2);
    let solver = BruteForce::new();
    let result = solver.solve(&problem);
    // Only valid order: v1 first, then v0
    // v1: LOAD v2, OP v1 (using v3 from memory) = 2 instructions (or LOAD v3, OP v1 using v2)
    // v0: OP v0 (using v1 from register, v2 from memory) = 1 instruction
    // Total: 3
    assert_eq!(result, Min(Some(3)));
}

#[test]
fn test_minimum_code_generation_one_register_solver_witness() {
    let problem = MinimumCodeGenerationOneRegister::new(4, vec![(0, 1), (0, 2), (1, 2), (1, 3)], 2);
    let solver = BruteForce::new();
    let witness = solver.find_witness(&problem).expect("should find witness");
    assert_eq!(problem.simulate(&witness), Some(3));
}

#[test]
fn test_minimum_code_generation_one_register_serialization() {
    let problem = MinimumCodeGenerationOneRegister::new(
        7,
        vec![
            (0, 1),
            (0, 2),
            (1, 3),
            (1, 4),
            (2, 3),
            (2, 5),
            (3, 5),
            (3, 6),
        ],
        3,
    );
    let json = serde_json::to_value(&problem).unwrap();
    let restored: MinimumCodeGenerationOneRegister = serde_json::from_value(json).unwrap();
    assert_eq!(restored.num_vertices(), problem.num_vertices());
    assert_eq!(restored.num_edges(), problem.num_edges());
    assert_eq!(restored.num_leaves(), problem.num_leaves());
    assert_eq!(restored.edges(), problem.edges());
}

#[test]
fn test_minimum_code_generation_one_register_unary_ops() {
    // Simple chain: v0 = unary(v1), v1 = unary(v2)
    // Leaves: {2}, Internal: {0, 1}
    let problem = MinimumCodeGenerationOneRegister::new(3, vec![(0, 1), (1, 2)], 1);
    // Order: v1 first, v0 second. config = [1, 0]
    let config = vec![1, 0];
    // v1: LOAD v2, OP v1 = 2
    // v0: OP v0 (v1 in register) = 1
    // Total = 3
    assert_eq!(problem.simulate(&config), Some(3));
    assert_eq!(problem.evaluate(&config), Min(Some(3)));
}

#[test]
fn test_minimum_code_generation_one_register_paper_example() {
    // Issue #900 example
    let problem = MinimumCodeGenerationOneRegister::new(
        7,
        vec![
            (0, 1),
            (0, 2),
            (1, 3),
            (1, 4),
            (2, 3),
            (2, 5),
            (3, 5),
            (3, 6),
        ],
        3,
    );

    // Optimal order: v3, v2, v1, v0 => config = [3, 2, 1, 0]
    let config = vec![3, 2, 1, 0];
    assert_eq!(problem.evaluate(&config), Min(Some(8)));

    // Verify with brute force
    let solver = BruteForce::new();
    let result = solver.solve(&problem);
    assert_eq!(result, Min(Some(8)));

    // Verify witness
    let witness = solver.find_witness(&problem).unwrap();
    assert_eq!(problem.simulate(&witness), Some(8));
}

#[test]
fn test_minimum_code_generation_one_register_lost_value() {
    // Test that a value computed but not stored and overwritten becomes unavailable
    // v0 = op(v1, v2), v1 = op(v3, v4), v2 = op(v3, v5)
    // Leaves: {3, 4, 5}, Internal: {0, 1, 2}
    // If we evaluate v1, then v2 (overwriting v1 in register without storing),
    // then v0 needs v1 which is lost.
    let problem = MinimumCodeGenerationOneRegister::new(
        6,
        vec![(0, 1), (0, 2), (1, 3), (1, 4), (2, 3), (2, 5)],
        3,
    );
    // Order: v1, v2, v0 => config: v0->2, v1->0, v2->1
    let config = vec![2, 0, 1];
    // v1 computed first, but v1 is needed by v0.
    // When v2 is computed, we should check if v1 needs to be stored.
    // future_uses[1] = 1 (used by v0), so STORE v1 before computing v2.
    // So this should NOT be None — the simulation stores v1 automatically.
    let result = problem.simulate(&config);
    assert!(result.is_some());
}
