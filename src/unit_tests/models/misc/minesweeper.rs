use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;

#[test]
fn test_minesweeper_creation() {
    let problem = Minesweeper::new(
        3,
        3,
        vec![(1, 1, 1)],
        vec![
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 0),
            (1, 2),
            (2, 0),
            (2, 1),
            (2, 2),
        ],
    );
    assert_eq!(problem.rows(), 3);
    assert_eq!(problem.cols(), 3);
    assert_eq!(problem.num_unrevealed(), 8);
    assert_eq!(problem.num_variables(), 8);
}

#[test]
fn test_minesweeper_evaluate_satisfiable() {
    let problem = Minesweeper::new(
        3,
        3,
        vec![(1, 1, 1)],
        vec![
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 0),
            (1, 2),
            (2, 0),
            (2, 1),
            (2, 2),
        ],
    );
    // Mine at (0,0) only => config[0]=1, rest=0
    assert!(problem.evaluate(&[1, 0, 0, 0, 0, 0, 0, 0]));
    // No mines => count would be 0, not 1
    assert!(!problem.evaluate(&[0, 0, 0, 0, 0, 0, 0, 0]));
    // Two mines adjacent => count would be 2, not 1
    assert!(!problem.evaluate(&[1, 1, 0, 0, 0, 0, 0, 0]));
}

#[test]
fn test_minesweeper_evaluate_unsatisfiable() {
    // Grid:
    //   1 ? 1
    //   ? 0 ?
    //   1 ? 1
    let problem = Minesweeper::new(
        3,
        3,
        vec![(0, 0, 1), (0, 2, 1), (1, 1, 0), (2, 0, 1), (2, 2, 1)],
        vec![(0, 1), (1, 0), (1, 2), (2, 1)],
    );
    // (1,1)=0 forces all unrevealed neighbors to 0
    // But (0,0)=1 needs at least 1 mine among its unrevealed neighbors
    assert!(!problem.evaluate(&[0, 0, 0, 0]));
    assert!(!problem.evaluate(&[1, 0, 0, 0]));
    assert!(!problem.evaluate(&[0, 1, 0, 0]));
    assert!(!problem.evaluate(&[0, 0, 1, 0]));
    assert!(!problem.evaluate(&[0, 0, 0, 1]));
}

#[test]
fn test_minesweeper_classic_pattern() {
    // Grid:
    //   1 ? ?
    //   1 2 ?
    //   0 1 ?
    let problem = Minesweeper::new(
        3,
        3,
        vec![(0, 0, 1), (1, 0, 1), (1, 1, 2), (2, 0, 0), (2, 1, 1)],
        vec![(0, 1), (0, 2), (1, 2), (2, 2)],
    );
    // Solution: mines at (0,1) and (1,2)
    assert!(problem.evaluate(&[1, 0, 1, 0]));
    // Wrong: mines at (0,1) and (0,2)
    assert!(!problem.evaluate(&[1, 1, 0, 0]));
}

#[test]
fn test_minesweeper_serialization() {
    let problem = Minesweeper::new(
        3,
        3,
        vec![(1, 1, 1)],
        vec![
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 0),
            (1, 2),
            (2, 0),
            (2, 1),
            (2, 2),
        ],
    );
    let json = serde_json::to_value(&problem).unwrap();
    let restored: Minesweeper = serde_json::from_value(json).unwrap();
    assert_eq!(restored.rows(), problem.rows());
    assert_eq!(restored.cols(), problem.cols());
    assert_eq!(restored.num_unrevealed(), problem.num_unrevealed());
}

#[test]
fn test_minesweeper_solver() {
    let problem = Minesweeper::new(
        3,
        3,
        vec![(1, 1, 1)],
        vec![
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 0),
            (1, 2),
            (2, 0),
            (2, 1),
            (2, 2),
        ],
    );
    let solver = BruteForce::new();
    let solution = solver.find_satisfying(&problem);
    assert!(solution.is_some());
    let sol = solution.unwrap();
    assert!(problem.evaluate(&sol));
    // Exactly one mine among 8 unrevealed cells
    assert_eq!(sol.iter().sum::<usize>(), 1);
}

#[test]
fn test_minesweeper_variant() {
    let v = <Minesweeper as Problem>::variant();
    assert!(v.is_empty());
}

#[test]
fn test_minesweeper_solver_unsatisfiable() {
    let problem = Minesweeper::new(
        3,
        3,
        vec![(0, 0, 1), (0, 2, 1), (1, 1, 0), (2, 0, 1), (2, 2, 1)],
        vec![(0, 1), (1, 0), (1, 2), (2, 1)],
    );
    let solver = BruteForce::new();
    let solution = solver.find_satisfying(&problem);
    assert!(solution.is_none());
}
