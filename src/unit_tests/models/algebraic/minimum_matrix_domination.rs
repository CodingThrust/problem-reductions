use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Min;

/// P6 adjacency matrix (6×6 symmetric, 10 ones).
fn p6_adjacency_matrix() -> Vec<Vec<bool>> {
    vec![
        vec![false, true, false, false, false, false],
        vec![true, false, true, false, false, false],
        vec![false, true, false, true, false, false],
        vec![false, false, true, false, true, false],
        vec![false, false, false, true, false, true],
        vec![false, false, false, false, true, false],
    ]
}

#[test]
fn test_minimum_matrix_domination_creation() {
    let problem = MinimumMatrixDomination::new(p6_adjacency_matrix());
    assert_eq!(problem.num_rows(), 6);
    assert_eq!(problem.num_cols(), 6);
    assert_eq!(problem.num_ones(), 10);
    assert_eq!(problem.dims(), vec![2; 10]);
    assert_eq!(
        <MinimumMatrixDomination as Problem>::NAME,
        "MinimumMatrixDomination"
    );
    assert_eq!(<MinimumMatrixDomination as Problem>::variant(), vec![]);
}

#[test]
fn test_minimum_matrix_domination_ones_enumeration() {
    let problem = MinimumMatrixDomination::new(p6_adjacency_matrix());
    let expected_ones = vec![
        (0, 1),
        (1, 0),
        (1, 2),
        (2, 1),
        (2, 3),
        (3, 2),
        (3, 4),
        (4, 3),
        (4, 5),
        (5, 4),
    ];
    assert_eq!(problem.ones(), &expected_ones);
}

#[test]
fn test_minimum_matrix_domination_evaluate_optimal() {
    let problem = MinimumMatrixDomination::new(p6_adjacency_matrix());
    // Select entries 0,1,6,7: (0,1),(1,0),(3,4),(4,3)
    // Covered rows: {0,1,3,4}, covered cols: {0,1,3,4}
    // Unselected: (1,2) row 1 covered, (2,1) col 1 covered, (2,3) col 3 covered,
    //             (3,2) row 3 covered, (4,5) row 4 covered, (5,4) col 4 covered
    let config = vec![1, 1, 0, 0, 0, 0, 1, 1, 0, 0];
    assert_eq!(problem.evaluate(&config), Min(Some(4)));
}

#[test]
fn test_minimum_matrix_domination_evaluate_infeasible() {
    let problem = MinimumMatrixDomination::new(p6_adjacency_matrix());
    // Select only entry 0: (0,1) — covers row 0, col 1
    // Entry (2,3) at index 4: row 2 not covered, col 3 not covered → infeasible
    let config = vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    assert_eq!(problem.evaluate(&config), Min(None));
}

#[test]
fn test_minimum_matrix_domination_evaluate_all_selected() {
    let problem = MinimumMatrixDomination::new(p6_adjacency_matrix());
    let config = vec![1; 10];
    assert_eq!(problem.evaluate(&config), Min(Some(10)));
}

#[test]
fn test_minimum_matrix_domination_evaluate_wrong_length() {
    let problem = MinimumMatrixDomination::new(p6_adjacency_matrix());
    assert_eq!(problem.evaluate(&[1, 0]), Min(None));
    assert_eq!(problem.evaluate(&[1; 11]), Min(None));
}

#[test]
fn test_minimum_matrix_domination_evaluate_invalid_variable() {
    let problem = MinimumMatrixDomination::new(p6_adjacency_matrix());
    let mut config = vec![0; 10];
    config[0] = 2;
    assert_eq!(problem.evaluate(&config), Min(None));
}

#[test]
fn test_minimum_matrix_domination_brute_force() {
    let problem = MinimumMatrixDomination::new(p6_adjacency_matrix());
    let solver = BruteForce::new();
    let witness = solver.find_witness(&problem).expect("should find optimal");
    let val = problem.evaluate(&witness);
    assert_eq!(val, Min(Some(4)));
}

#[test]
fn test_minimum_matrix_domination_identity_matrix() {
    // 3×3 identity: 3 ones on diagonal, no shared rows/cols
    // Every entry must be selected
    let matrix = vec![
        vec![true, false, false],
        vec![false, true, false],
        vec![false, false, true],
    ];
    let problem = MinimumMatrixDomination::new(matrix);
    assert_eq!(problem.num_ones(), 3);
    let solver = BruteForce::new();
    let witness = solver.find_witness(&problem).expect("should find optimal");
    assert_eq!(problem.evaluate(&witness), Min(Some(3)));
    assert_eq!(witness, vec![1, 1, 1]);
}

#[test]
fn test_minimum_matrix_domination_single_row() {
    // One row with multiple ones: selecting any one dominates all others
    let matrix = vec![vec![true, true, true]];
    let problem = MinimumMatrixDomination::new(matrix);
    assert_eq!(problem.num_ones(), 3);
    let solver = BruteForce::new();
    let witness = solver.find_witness(&problem).expect("should find optimal");
    assert_eq!(problem.evaluate(&witness), Min(Some(1)));
}

#[test]
fn test_minimum_matrix_domination_empty_matrix() {
    let problem = MinimumMatrixDomination::new(vec![]);
    assert_eq!(problem.num_ones(), 0);
    assert_eq!(problem.dims(), Vec::<usize>::new());
    // Empty config: vacuously valid with 0 selected
    assert_eq!(problem.evaluate(&[]), Min(Some(0)));
}

#[test]
fn test_minimum_matrix_domination_no_ones() {
    let matrix = vec![vec![false, false], vec![false, false]];
    let problem = MinimumMatrixDomination::new(matrix);
    assert_eq!(problem.num_ones(), 0);
    assert_eq!(problem.evaluate(&[]), Min(Some(0)));
}

#[test]
fn test_minimum_matrix_domination_serialization() {
    let matrix = vec![vec![true, false], vec![false, true]];
    let problem = MinimumMatrixDomination::new(matrix);
    let json = serde_json::to_value(&problem).unwrap();
    assert_eq!(
        json,
        serde_json::json!({
            "matrix": [[true, false], [false, true]],
            "ones": [[0, 0], [1, 1]],
        })
    );
    let restored: MinimumMatrixDomination = serde_json::from_value(json).unwrap();
    assert_eq!(restored.num_rows(), 2);
    assert_eq!(restored.num_cols(), 2);
    assert_eq!(restored.num_ones(), 2);
}

#[test]
fn test_minimum_matrix_domination_complexity_metadata() {
    use crate::registry::VariantEntry;

    let entry = inventory::iter::<VariantEntry>()
        .find(|entry| entry.name == "MinimumMatrixDomination")
        .expect("MinimumMatrixDomination variant entry should exist");

    assert_eq!(entry.complexity, "2^num_ones");
}

#[test]
#[should_panic(expected = "same length")]
fn test_minimum_matrix_domination_inconsistent_rows() {
    let matrix = vec![vec![true, false], vec![true]];
    MinimumMatrixDomination::new(matrix);
}
