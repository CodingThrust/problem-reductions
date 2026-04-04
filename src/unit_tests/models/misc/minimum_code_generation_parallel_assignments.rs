use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_minimum_code_generation_parallel_assignments_creation() {
    let assignments = vec![(0, vec![1, 2]), (1, vec![0]), (2, vec![3]), (3, vec![1, 2])];
    let problem = MinimumCodeGenerationParallelAssignments::new(4, assignments.clone());
    assert_eq!(problem.num_variables(), 4);
    assert_eq!(problem.num_assignments(), 4);
    assert_eq!(problem.assignments(), &assignments);
    assert_eq!(problem.dims(), vec![4; 4]);
    assert_eq!(
        <MinimumCodeGenerationParallelAssignments as Problem>::NAME,
        "MinimumCodeGenerationParallelAssignments"
    );
    assert_eq!(
        <MinimumCodeGenerationParallelAssignments as Problem>::variant(),
        vec![]
    );
}

#[test]
fn test_minimum_code_generation_parallel_assignments_evaluate_optimal() {
    let assignments = vec![(0, vec![1, 2]), (1, vec![0]), (2, vec![3]), (3, vec![1, 2])];
    let problem = MinimumCodeGenerationParallelAssignments::new(4, assignments);
    // Config [0, 3, 1, 2]: A_0 at pos 0, A_1 at pos 3, A_2 at pos 1, A_3 at pos 2
    // Order: (A_0, A_2, A_3, A_1)
    // A_0 writes a(0): A_1 reads a and is later (pos 3) -> 1 backward dep
    // A_2 writes c(2): A_3 reads c and is later (pos 2) -> 1 backward dep
    // A_3 writes d(3): A_1 does not read d -> 0
    // Total: 2
    assert_eq!(problem.evaluate(&[0, 3, 1, 2]), Min(Some(2)));
}

#[test]
fn test_minimum_code_generation_parallel_assignments_evaluate_suboptimal() {
    let assignments = vec![(0, vec![1, 2]), (1, vec![0]), (2, vec![3]), (3, vec![1, 2])];
    let problem = MinimumCodeGenerationParallelAssignments::new(4, assignments);
    // Config [1, 0, 2, 3]: A_0 at pos 1, A_1 at pos 0, A_2 at pos 2, A_3 at pos 3
    // Order: (A_1, A_0, A_2, A_3)
    // A_1 writes b(1): A_0 reads b (later, pos 1) -> 1; A_3 reads b (later, pos 3) -> 1
    // A_0 writes a(0): A_1 already executed -> 0
    // A_2 writes c(2): A_3 reads c (later, pos 3) -> 1
    // Total: 3
    assert_eq!(problem.evaluate(&[1, 0, 2, 3]), Min(Some(3)));
}

#[test]
fn test_minimum_code_generation_parallel_assignments_evaluate_invalid() {
    let assignments = vec![(0, vec![1, 2]), (1, vec![0]), (2, vec![3]), (3, vec![1, 2])];
    let problem = MinimumCodeGenerationParallelAssignments::new(4, assignments);
    // Duplicate position
    assert_eq!(problem.evaluate(&[0, 0, 1, 2]), Min(None));
    // Out of range
    assert_eq!(problem.evaluate(&[0, 1, 2, 4]), Min(None));
    // Wrong length
    assert_eq!(problem.evaluate(&[0, 1, 2]), Min(None));
    assert_eq!(problem.evaluate(&[0, 1, 2, 3, 0]), Min(None));
}

#[test]
fn test_minimum_code_generation_parallel_assignments_solver() {
    let assignments = vec![(0, vec![1, 2]), (1, vec![0]), (2, vec![3]), (3, vec![1, 2])];
    let problem = MinimumCodeGenerationParallelAssignments::new(4, assignments);
    let solver = BruteForce::new();
    let solution = solver
        .find_witness(&problem)
        .expect("should find a solution");
    let value = problem.evaluate(&solution);
    assert_eq!(value, Min(Some(2)));
}

#[test]
fn test_minimum_code_generation_parallel_assignments_serialization() {
    let assignments = vec![(0, vec![1, 2]), (1, vec![0]), (2, vec![3]), (3, vec![1, 2])];
    let problem = MinimumCodeGenerationParallelAssignments::new(4, assignments.clone());
    let json = serde_json::to_value(&problem).unwrap();
    let restored: MinimumCodeGenerationParallelAssignments = serde_json::from_value(json).unwrap();
    assert_eq!(restored.num_variables(), 4);
    assert_eq!(restored.assignments(), &assignments);
}

#[test]
fn test_minimum_code_generation_parallel_assignments_no_dependencies() {
    // No assignment reads the target of another -> 0 backward deps for any ordering
    let assignments = vec![
        (0, vec![2]), // writes a, reads c
        (1, vec![3]), // writes b, reads d
    ];
    let problem = MinimumCodeGenerationParallelAssignments::new(4, assignments);
    // Neither assignment reads the target of the other
    assert_eq!(problem.evaluate(&[0, 1]), Min(Some(0)));
    assert_eq!(problem.evaluate(&[1, 0]), Min(Some(0)));
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem).unwrap();
    assert_eq!(problem.evaluate(&solution), Min(Some(0)));
}

#[test]
#[should_panic(expected = "target variable")]
fn test_minimum_code_generation_parallel_assignments_invalid_target_panics() {
    MinimumCodeGenerationParallelAssignments::new(2, vec![(2, vec![0])]);
}

#[test]
#[should_panic(expected = "read variable")]
fn test_minimum_code_generation_parallel_assignments_invalid_read_panics() {
    MinimumCodeGenerationParallelAssignments::new(2, vec![(0, vec![3])]);
}

#[cfg(feature = "example-db")]
#[test]
fn test_minimum_code_generation_parallel_assignments_canonical_example() {
    let specs = canonical_model_example_specs();
    assert_eq!(specs.len(), 1);
    let spec = &specs[0];
    assert_eq!(spec.id, "minimum_code_generation_parallel_assignments");
    assert_eq!(spec.optimal_config, vec![0, 3, 1, 2]);
    assert_eq!(spec.optimal_value, serde_json::json!(2));
}
