use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_minimum_code_generation_unlimited_registers_creation() {
    let problem = MinimumCodeGenerationUnlimitedRegisters::new(
        5,
        vec![(1, 3), (2, 3), (0, 1)],
        vec![(1, 4), (2, 4), (0, 2)],
    );
    assert_eq!(problem.num_vertices(), 5);
    assert_eq!(problem.num_leaves(), 2);
    assert_eq!(problem.num_internal(), 3);
    assert_eq!(problem.left_arcs(), &[(1, 3), (2, 3), (0, 1)]);
    assert_eq!(problem.right_arcs(), &[(1, 4), (2, 4), (0, 2)]);
    assert_eq!(problem.dims(), vec![3; 3]);
    assert_eq!(
        <MinimumCodeGenerationUnlimitedRegisters as Problem>::NAME,
        "MinimumCodeGenerationUnlimitedRegisters"
    );
    assert_eq!(
        <MinimumCodeGenerationUnlimitedRegisters as Problem>::variant(),
        vec![]
    );
}

#[test]
fn test_minimum_code_generation_unlimited_registers_evaluate_optimal() {
    // Issue #902 example: optimal is 4 instructions (3 OPs + 1 LOAD)
    // Evaluation order: v1, v2, v0
    // config[i] = position for internal vertex i
    // internal = [0,1,2], so v0->pos 2, v1->pos 0, v2->pos 1
    let problem = MinimumCodeGenerationUnlimitedRegisters::new(
        5,
        vec![(1, 3), (2, 3), (0, 1)],
        vec![(1, 4), (2, 4), (0, 2)],
    );
    let config = vec![2, 0, 1];
    assert_eq!(problem.evaluate(&config), Min(Some(4)));
    assert_eq!(problem.simulate(&config), Some(4));
}

#[test]
fn test_minimum_code_generation_unlimited_registers_evaluate_suboptimal() {
    // Order: v2, v1, v0 — v2 destroys v3 first, then v1 also needs v3 as left
    // So v1 needs a copy of v3 too, but v3 was already destroyed by v2.
    // Wait — with unlimited registers, v3 stays in its register. When v2 executes,
    // OP v2 overwrites v3's register. But if v1 hasn't run yet and also needs v3,
    // then before v2 we must copy v3.
    // Then when v1 runs, v3 is gone (overwritten by v2), but we copied it.
    // Actually: the copy goes to a new register. So v1 can use the copy.
    // But in our simulation, we track: before OP v2, left_child=v3 still needed
    // by v1 (as left operand). So we LOAD (copy) v3. Cost so far: 1 LOAD + 1 OP = 2.
    // Then OP v1: left_child=v3. But v3's register was overwritten by v2.
    // Hmm, our simulation tracks "computed" but not register aliasing.
    //
    // Actually, with unlimited registers, the LOAD creates a copy in a new register.
    // The key insight: when we count future uses, we check if the left child
    // value is still needed. If so, we add a LOAD before the OP.
    // After the OP, the left child's "register slot" now holds the new value.
    // But the copy made by LOAD is in a separate register.
    //
    // Our simulation correctly handles this: it counts LOADs needed.
    // Order v2(pos 0), v1(pos 1), v0(pos 2):
    // config: v0->2, v1->1, v2->0
    let problem = MinimumCodeGenerationUnlimitedRegisters::new(
        5,
        vec![(1, 3), (2, 3), (0, 1)],
        vec![(1, 4), (2, 4), (0, 2)],
    );
    let config = vec![2, 1, 0];
    // Step 0: OP v2, left=v3. future uses of v3 after decrement: left_uses=1 (from v1), right_uses=0.
    //   Still needed -> LOAD v3. instructions = 2 (1 LOAD + 1 OP).
    // Step 1: OP v1, left=v3_copy. future uses of v3 after decrement: 0.
    //   Not needed -> no LOAD. instructions = 3 (+ 1 OP).
    // Step 2: OP v0, left=v1. future uses of v1: 0. No LOAD. instructions = 4 (+ 1 OP).
    // Total: 4 (same as optimal for this instance)
    assert_eq!(problem.simulate(&config), Some(4));
}

#[test]
fn test_minimum_code_generation_unlimited_registers_dependency_violation() {
    // Try to evaluate v0 before its children v1, v2
    let problem = MinimumCodeGenerationUnlimitedRegisters::new(
        5,
        vec![(1, 3), (2, 3), (0, 1)],
        vec![(1, 4), (2, 4), (0, 2)],
    );
    // v0 first (pos 0) — depends on v1,v2 which haven't been computed
    let config = vec![0, 1, 2];
    assert_eq!(problem.evaluate(&config), Min(None));
}

#[test]
fn test_minimum_code_generation_unlimited_registers_invalid_permutation() {
    let problem = MinimumCodeGenerationUnlimitedRegisters::new(
        5,
        vec![(1, 3), (2, 3), (0, 1)],
        vec![(1, 4), (2, 4), (0, 2)],
    );
    // Not a permutation: position 0 used twice
    assert_eq!(problem.evaluate(&[0, 0, 1]), Min(None));
    // Wrong length
    assert_eq!(problem.evaluate(&[0, 1]), Min(None));
    // Position out of range
    assert_eq!(problem.evaluate(&[0, 1, 5]), Min(None));
}

#[test]
fn test_minimum_code_generation_unlimited_registers_solver() {
    // Issue #902 example
    let problem = MinimumCodeGenerationUnlimitedRegisters::new(
        5,
        vec![(1, 3), (2, 3), (0, 1)],
        vec![(1, 4), (2, 4), (0, 2)],
    );
    let solver = BruteForce::new();
    let result = solver.solve(&problem);
    assert_eq!(result, Min(Some(4)));
}

#[test]
fn test_minimum_code_generation_unlimited_registers_solver_witness() {
    let problem = MinimumCodeGenerationUnlimitedRegisters::new(
        5,
        vec![(1, 3), (2, 3), (0, 1)],
        vec![(1, 4), (2, 4), (0, 2)],
    );
    let solver = BruteForce::new();
    let witness = solver.find_witness(&problem).expect("should find witness");
    assert_eq!(problem.simulate(&witness), Some(4));
}

#[test]
fn test_minimum_code_generation_unlimited_registers_serialization() {
    let problem = MinimumCodeGenerationUnlimitedRegisters::new(
        5,
        vec![(1, 3), (2, 3), (0, 1)],
        vec![(1, 4), (2, 4), (0, 2)],
    );
    let json = serde_json::to_value(&problem).unwrap();
    let restored: MinimumCodeGenerationUnlimitedRegisters = serde_json::from_value(json).unwrap();
    assert_eq!(restored.num_vertices(), problem.num_vertices());
    assert_eq!(restored.left_arcs(), problem.left_arcs());
    assert_eq!(restored.right_arcs(), problem.right_arcs());
    assert_eq!(restored.num_leaves(), problem.num_leaves());
}

#[test]
fn test_minimum_code_generation_unlimited_registers_unary_ops() {
    // Simple chain: v0 = unary(v1), v1 = unary(v2)
    // Leaves: {2}, Internal: {0, 1}
    // Unary ops only have left arcs
    let problem = MinimumCodeGenerationUnlimitedRegisters::new(3, vec![(0, 1), (1, 2)], vec![]);
    // Order: v1 first, v0 second. config = [1, 0]
    let config = vec![1, 0];
    // v1: left=v2, no future uses of v2 -> no LOAD. OP v1 = 1.
    // v0: left=v1, no future uses of v1 -> no LOAD. OP v0 = 1.
    // Total = 2 (just 2 OPs, no copies needed)
    assert_eq!(problem.simulate(&config), Some(2));
    assert_eq!(problem.evaluate(&config), Min(Some(2)));
}

#[test]
fn test_minimum_code_generation_unlimited_registers_no_copy_needed() {
    // v0 = op(v1, v2), v1 and v2 are leaves
    // No shared operands, so no copies needed
    let problem = MinimumCodeGenerationUnlimitedRegisters::new(3, vec![(0, 1)], vec![(0, 2)]);
    // Only one internal vertex v0, config = [0]
    let config = vec![0];
    // OP v0: left=v1, right=v2. No future uses of v1. No LOAD. 1 OP.
    assert_eq!(problem.simulate(&config), Some(1));
}

#[test]
fn test_minimum_code_generation_unlimited_registers_paper_example() {
    // Issue #902 example
    let problem = MinimumCodeGenerationUnlimitedRegisters::new(
        5,
        vec![(1, 3), (2, 3), (0, 1)],
        vec![(1, 4), (2, 4), (0, 2)],
    );

    // Optimal order: v1, v2, v0 => config = [2, 0, 1]
    let config = vec![2, 0, 1];
    assert_eq!(problem.evaluate(&config), Min(Some(4)));

    // Verify with brute force
    let solver = BruteForce::new();
    let result = solver.solve(&problem);
    assert_eq!(result, Min(Some(4)));

    // Verify witness
    let witness = solver.find_witness(&problem).unwrap();
    assert_eq!(problem.simulate(&witness), Some(4));
}
