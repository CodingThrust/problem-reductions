use super::*;
use crate::registry::VariantEntry;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;

fn issue_example_matrix() -> Vec<Vec<bool>> {
    vec![
        vec![true, false, false, true],
        vec![false, true, false, false],
        vec![false, false, true, false],
        vec![true, false, false, false],
    ]
}

#[test]
fn test_sparse_matrix_compression_basic() {
    let problem = SparseMatrixCompression::new(issue_example_matrix(), 2);

    assert_eq!(problem.matrix(), issue_example_matrix().as_slice());
    assert_eq!(problem.num_rows(), 4);
    assert_eq!(problem.num_cols(), 4);
    assert_eq!(problem.bound_k(), 2);
    assert_eq!(problem.storage_len(), 6);
    assert_eq!(problem.dims(), vec![2; 4]);
    assert_eq!(
        <SparseMatrixCompression as Problem>::NAME,
        "SparseMatrixCompression"
    );
    assert_eq!(<SparseMatrixCompression as Problem>::variant(), vec![]);
}

#[test]
fn test_sparse_matrix_compression_issue_example_is_satisfying() {
    let problem = SparseMatrixCompression::new(issue_example_matrix(), 2);

    assert!(problem.evaluate(&[1, 1, 1, 0]));
    assert_eq!(
        problem
            .storage_vector(&[1, 1, 1, 0])
            .expect("issue example should produce an overlay"),
        vec![4, 1, 2, 3, 1, 0]
    );
}

#[test]
fn test_sparse_matrix_compression_issue_unsatisfying_examples() {
    let problem = SparseMatrixCompression::new(issue_example_matrix(), 2);

    assert!(!problem.evaluate(&[0, 0, 0, 0]));
    assert!(!problem.evaluate(&[0, 1, 1, 1]));
    assert!(!problem.evaluate(&[1, 1, 1, 1]));
}

#[test]
fn test_sparse_matrix_compression_rejects_bad_configs() {
    let problem = SparseMatrixCompression::new(issue_example_matrix(), 2);

    assert!(!problem.evaluate(&[1, 1, 1]));
    assert!(!problem.evaluate(&[1, 1, 1, 0, 0]));
    assert!(!problem.evaluate(&[2, 1, 1, 0]));
    assert!(problem.storage_vector(&[2, 1, 1, 0]).is_none());
}

#[test]
fn test_sparse_matrix_compression_bruteforce_finds_unique_solution() {
    let problem = SparseMatrixCompression::new(issue_example_matrix(), 2);
    let solver = BruteForce::new();

    let solution = solver
        .find_satisfying(&problem)
        .expect("issue example should be satisfiable");
    assert_eq!(solution, vec![1, 1, 1, 0]);

    let all = solver.find_all_satisfying(&problem);
    assert_eq!(all, vec![vec![1, 1, 1, 0]]);
}

#[test]
fn test_sparse_matrix_compression_serialization() {
    let problem = SparseMatrixCompression::new(issue_example_matrix(), 2);

    let json = serde_json::to_value(&problem).unwrap();
    assert_eq!(
        json,
        serde_json::json!({
            "matrix": [
                [true, false, false, true],
                [false, true, false, false],
                [false, false, true, false],
                [true, false, false, false],
            ],
            "bound_k": 2,
        })
    );

    let restored: SparseMatrixCompression = serde_json::from_value(json).unwrap();
    assert_eq!(restored.num_rows(), 4);
    assert_eq!(restored.num_cols(), 4);
    assert_eq!(restored.bound_k(), 2);
}

#[test]
fn test_sparse_matrix_compression_complexity_metadata_matches_evaluator() {
    let entry = inventory::iter::<VariantEntry>()
        .into_iter()
        .find(|entry| entry.name == "SparseMatrixCompression")
        .expect("SparseMatrixCompression variant entry should exist");

    assert_eq!(
        entry.complexity,
        "(bound_k ^ num_rows) * num_rows * num_cols"
    );
}

#[test]
#[should_panic(expected = "bound_k")]
fn test_sparse_matrix_compression_rejects_zero_bound() {
    let _ = SparseMatrixCompression::new(issue_example_matrix(), 0);
}

#[test]
#[should_panic(expected = "same length")]
fn test_sparse_matrix_compression_rejects_ragged_matrix() {
    let _ = SparseMatrixCompression::new(vec![vec![true, false], vec![true]], 2);
}
