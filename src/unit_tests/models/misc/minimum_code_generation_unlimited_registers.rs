use super::*;
use crate::solvers::BruteForce;
use crate::solvers::BruteForceProblem as _;
use crate::traits::Problem;

#[test]
fn test_minimum_code_generation_unlimited_registers_validates_persisted_input() {
    let valid = serde_json::json!({"num_vertices":3,"left_arcs":[[0,1]],"right_arcs":[[0,2]]});
    let restored: MinimumCodeGenerationUnlimitedRegisters =
        serde_json::from_value(valid.clone()).unwrap();
    assert_eq!(serde_json::to_value(restored).unwrap(), valid);
    for (field, value) in [
        ("left_arcs", serde_json::json!([[0, 3]])),
        ("right_arcs", serde_json::json!([[3, 0]])),
        ("left_arcs", serde_json::json!([[0, 0]])),
        ("right_arcs", serde_json::json!([[0, 0]])),
        ("left_arcs", serde_json::json!([[0, 1], [0, 2]])),
        ("left_arcs", serde_json::json!([])),
        ("right_arcs", serde_json::json!([[1, 2]])),
    ] {
        let mut invalid = valid.clone();
        invalid[field] = value;
        assert!(
            serde_json::from_value::<MinimumCodeGenerationUnlimitedRegisters>(invalid).is_err(),
            "{field}"
        );
    }
}
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
    assert_eq!(problem.dimensions(), vec![3, 2, 1]);
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
    // Example: optimal is 4 instructions (3 OPs + 1 LOAD)
    // Evaluation order: v1, v2, v0
    // config[i] = position for internal vertex i
    // internal = [0,1,2], so v0->pos 2, v1->pos 0, v2->pos 1
    let problem = MinimumCodeGenerationUnlimitedRegisters::new(
        5,
        vec![(1, 3), (2, 3), (0, 1)],
        vec![(1, 4), (2, 4), (0, 2)],
    );
    let config = vec![2, 0, 1];
    assert_eq!(problem.evaluate(&config).unwrap(), Min(Some(4)));
    assert_eq!(problem.simulate(&config).unwrap(), Some(4));
}

#[test]
fn test_minimum_code_generation_unlimited_registers_evaluate_suboptimal() {
    // Order v2, v1, v0 needs one LOAD to preserve v3 before v2 overwrites it,
    // since v1 still needs v3. Together with three OPs, the cost is 4.
    let problem = MinimumCodeGenerationUnlimitedRegisters::new(
        5,
        vec![(1, 3), (2, 3), (0, 1)],
        vec![(1, 4), (2, 4), (0, 2)],
    );
    let config = vec![2, 1, 0];
    assert_eq!(problem.simulate(&config).unwrap(), Some(4));
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
    assert_eq!(problem.evaluate(&config).unwrap(), Min(None));
}

#[test]
fn test_minimum_code_generation_unlimited_registers_invalid_permutation() {
    let problem = MinimumCodeGenerationUnlimitedRegisters::new(
        5,
        vec![(1, 3), (2, 3), (0, 1)],
        vec![(1, 4), (2, 4), (0, 2)],
    );
    // Not a permutation: position 0 used twice
    assert_eq!(problem.evaluate(&vec![0, 0, 1]).unwrap(), Min(None));
    // Wrong length
    assert!(matches!(
        problem.evaluate(&vec![0, 1]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
    // Position out of range
    assert!(matches!(
        problem.evaluate(&vec![0, 1, 5]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
}

#[test]
fn test_minimum_code_generation_unlimited_registers_solver() {
    let problem = MinimumCodeGenerationUnlimitedRegisters::new(
        5,
        vec![(1, 3), (2, 3), (0, 1)],
        vec![(1, 4), (2, 4), (0, 2)],
    );
    let solver = BruteForce::new();
    let result_solution = solver.solve(&problem).unwrap().unwrap();
    let result = problem.evaluate(&result_solution).unwrap();
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
    let witness = solver
        .solve(&problem)
        .unwrap()
        .expect("should find witness");
    assert_eq!(problem.simulate(&witness).unwrap(), Some(4));
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
    assert_eq!(problem.simulate(&config).unwrap(), Some(2));
    assert_eq!(problem.evaluate(&config).unwrap(), Min(Some(2)));
}

#[test]
fn test_minimum_code_generation_unlimited_registers_no_copy_needed() {
    // v0 = op(v1, v2), v1 and v2 are leaves
    // No shared operands, so no copies needed
    let problem = MinimumCodeGenerationUnlimitedRegisters::new(3, vec![(0, 1)], vec![(0, 2)]);
    // Only one internal vertex v0, config = [0]
    let config = vec![0];
    // OP v0: left=v1, right=v2. No future uses of v1. No LOAD. 1 OP.
    assert_eq!(problem.simulate(&config).unwrap(), Some(1));
}

#[test]
fn test_minimum_code_generation_unlimited_registers_paper_example() {
    let problem = MinimumCodeGenerationUnlimitedRegisters::new(
        5,
        vec![(1, 3), (2, 3), (0, 1)],
        vec![(1, 4), (2, 4), (0, 2)],
    );

    // Optimal order: v1, v2, v0 => config = [2, 0, 1]
    let config = vec![2, 0, 1];
    assert_eq!(problem.evaluate(&config).unwrap(), Min(Some(4)));

    let solver = BruteForce::new();
    let result_solution = solver.solve(&problem).unwrap().unwrap();
    let result = problem.evaluate(&result_solution).unwrap();
    assert_eq!(result, Min(Some(4)));

    let witness = solver.solve(&problem).unwrap().unwrap();
    assert_eq!(problem.simulate(&witness).unwrap(), Some(4));
}
