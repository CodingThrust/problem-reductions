use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;

fn two_block_matrix() -> Vec<Vec<bool>> {
    vec![
        vec![true, true, false, false],
        vec![true, true, false, false],
        vec![false, false, true, true],
        vec![false, false, true, true],
    ]
}

fn issue_matrix() -> Vec<Vec<bool>> {
    // 6x6 matrix from the issue description
    vec![
        vec![true, true, true, false, false, false],
        vec![true, true, true, false, false, false],
        vec![false, false, true, true, true, false],
        vec![false, false, true, true, true, false],
        vec![false, false, false, false, true, true],
        vec![false, false, false, false, true, true],
    ]
}

#[test]
fn test_rectilinear_picture_compression_basic() {
    let problem = RectilinearPictureCompression::new(two_block_matrix(), 2);
    assert_eq!(problem.num_rows(), 4);
    assert_eq!(problem.num_cols(), 4);
    assert_eq!(problem.bound(), 2);
    assert_eq!(
        <RectilinearPictureCompression as Problem>::NAME,
        "RectilinearPictureCompression"
    );
    assert_eq!(
        <RectilinearPictureCompression as Problem>::variant(),
        vec![]
    );
}

#[test]
fn test_rectilinear_picture_compression_maximal_rectangles_two_blocks() {
    let problem = RectilinearPictureCompression::new(two_block_matrix(), 2);
    let rects = problem.maximal_rectangles();
    // Two disjoint 2x2 blocks: (0,0,1,1) and (2,2,3,3)
    assert_eq!(rects, vec![(0, 0, 1, 1), (2, 2, 3, 3)]);
}

#[test]
fn test_rectilinear_picture_compression_dims() {
    let problem = RectilinearPictureCompression::new(two_block_matrix(), 2);
    // 2 maximal rectangles -> 2 binary variables
    assert_eq!(problem.dims(), vec![2, 2]);
}

#[test]
fn test_rectilinear_picture_compression_evaluate_satisfying() {
    let problem = RectilinearPictureCompression::new(two_block_matrix(), 2);
    // Select both maximal rectangles
    assert!(problem.evaluate(&[1, 1]));
}

#[test]
fn test_rectilinear_picture_compression_evaluate_unsatisfying_not_all_covered() {
    let problem = RectilinearPictureCompression::new(two_block_matrix(), 2);
    // Select only first rectangle - second block uncovered
    assert!(!problem.evaluate(&[1, 0]));
    // Select only second rectangle - first block uncovered
    assert!(!problem.evaluate(&[0, 1]));
    // Select none
    assert!(!problem.evaluate(&[0, 0]));
}

#[test]
fn test_rectilinear_picture_compression_evaluate_bound_exceeded() {
    let problem = RectilinearPictureCompression::new(two_block_matrix(), 1);
    // Both selected but bound is 1
    assert!(!problem.evaluate(&[1, 1]));
}

#[test]
fn test_rectilinear_picture_compression_evaluate_wrong_config_length() {
    let problem = RectilinearPictureCompression::new(two_block_matrix(), 2);
    assert!(!problem.evaluate(&[1]));
    assert!(!problem.evaluate(&[1, 1, 0]));
}

#[test]
fn test_rectilinear_picture_compression_evaluate_invalid_variable_value() {
    let problem = RectilinearPictureCompression::new(two_block_matrix(), 2);
    assert!(!problem.evaluate(&[2, 0]));
}

#[test]
fn test_rectilinear_picture_compression_issue_matrix_satisfiable() {
    let problem = RectilinearPictureCompression::new(issue_matrix(), 3);
    let solver = BruteForce::new();
    let solution = solver.find_satisfying(&problem);
    assert!(solution.is_some());
    let sol = solution.unwrap();
    assert!(problem.evaluate(&sol));
}

#[test]
fn test_rectilinear_picture_compression_issue_matrix_unsatisfiable() {
    let problem = RectilinearPictureCompression::new(issue_matrix(), 2);
    let solver = BruteForce::new();
    let solution = solver.find_satisfying(&problem);
    assert!(solution.is_none());
}

#[test]
fn test_rectilinear_picture_compression_brute_force() {
    let problem = RectilinearPictureCompression::new(two_block_matrix(), 2);
    let solver = BruteForce::new();
    let solution = solver
        .find_satisfying(&problem)
        .expect("should find a solution");
    assert!(problem.evaluate(&solution));
}

#[test]
fn test_rectilinear_picture_compression_brute_force_all() {
    let problem = RectilinearPictureCompression::new(two_block_matrix(), 2);
    let solver = BruteForce::new();
    let solutions = solver.find_all_satisfying(&problem);
    // Two disjoint 2x2 blocks with K=2: exactly one satisfying config [1,1].
    assert_eq!(solutions.len(), 1);
    for sol in &solutions {
        assert!(problem.evaluate(sol));
    }
}

#[test]
fn test_rectilinear_picture_compression_serialization() {
    let problem = RectilinearPictureCompression::new(two_block_matrix(), 2);
    let json = serde_json::to_value(&problem).unwrap();
    assert_eq!(
        json,
        serde_json::json!({
            "matrix": [
                [true, true, false, false],
                [true, true, false, false],
                [false, false, true, true],
                [false, false, true, true],
            ],
            "bound": 2,
        })
    );
    let restored: RectilinearPictureCompression = serde_json::from_value(json).unwrap();
    assert_eq!(restored.num_rows(), problem.num_rows());
    assert_eq!(restored.num_cols(), problem.num_cols());
    assert_eq!(restored.bound(), problem.bound());
    assert_eq!(restored.matrix(), problem.matrix());
}

#[test]
fn test_rectilinear_picture_compression_single_cell() {
    // Single 1-entry matrix
    let matrix = vec![vec![true]];
    let problem = RectilinearPictureCompression::new(matrix, 1);
    let rects = problem.maximal_rectangles();
    assert_eq!(rects, vec![(0, 0, 0, 0)]);
    assert_eq!(problem.dims(), vec![2]);
    assert!(problem.evaluate(&[1]));
    assert!(!problem.evaluate(&[0]));
}

#[test]
fn test_rectilinear_picture_compression_all_zeros() {
    // Matrix with no 1-entries: no maximal rectangles, always satisfiable
    let matrix = vec![vec![false, false], vec![false, false]];
    let problem = RectilinearPictureCompression::new(matrix, 0);
    let rects = problem.maximal_rectangles();
    assert!(rects.is_empty());
    assert_eq!(problem.dims(), Vec::<usize>::new());
    // Empty config satisfies (no 1-entries to cover)
    assert!(problem.evaluate(&[]));
}

#[test]
fn test_rectilinear_picture_compression_full_matrix() {
    // 2x2 all-ones matrix: one maximal rectangle covers everything
    let matrix = vec![vec![true, true], vec![true, true]];
    let problem = RectilinearPictureCompression::new(matrix, 1);
    let rects = problem.maximal_rectangles();
    assert_eq!(rects, vec![(0, 0, 1, 1)]);
    assert!(problem.evaluate(&[1]));
    assert!(!problem.evaluate(&[0]));
}

#[test]
fn test_rectilinear_picture_compression_overlapping_rectangles() {
    // L-shaped region: requires multiple rectangles, some may overlap
    let matrix = vec![vec![true, true], vec![true, false]];
    let problem = RectilinearPictureCompression::new(matrix, 2);
    let rects = problem.maximal_rectangles();
    // Maximal rectangles: (0,0,1,0) vertical bar, (0,0,0,1) horizontal bar
    assert!(rects.contains(&(0, 0, 1, 0)));
    assert!(rects.contains(&(0, 0, 0, 1)));
    let solver = BruteForce::new();
    let solution = solver.find_satisfying(&problem).unwrap();
    assert!(problem.evaluate(&solution));
}

#[test]
fn test_rectilinear_picture_compression_matrix_getter() {
    let matrix = two_block_matrix();
    let problem = RectilinearPictureCompression::new(matrix.clone(), 2);
    assert_eq!(problem.matrix(), &matrix);
}

#[test]
#[should_panic(expected = "empty")]
fn test_rectilinear_picture_compression_empty_matrix_panics() {
    RectilinearPictureCompression::new(vec![], 1);
}

#[test]
#[should_panic(expected = "column")]
fn test_rectilinear_picture_compression_empty_row_panics() {
    RectilinearPictureCompression::new(vec![vec![]], 1);
}

#[test]
#[should_panic(expected = "same length")]
fn test_rectilinear_picture_compression_inconsistent_rows_panics() {
    RectilinearPictureCompression::new(vec![vec![true, false], vec![true]], 1);
}
