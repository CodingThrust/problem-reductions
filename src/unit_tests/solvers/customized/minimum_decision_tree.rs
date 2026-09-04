use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_subset_dp_minimum_decision_tree_matches_brute_force() {
    let rows = [
        vec![false, false, true],
        vec![false, true, false],
        vec![false, true, true],
        vec![true, false, false],
        vec![true, false, true],
        vec![true, true, false],
    ];
    for first in 0..rows.len() {
        for second in (first + 1)..rows.len() {
            for third in (second + 1)..rows.len() {
                let matrix = vec![
                    rows[first].clone(),
                    rows[second].clone(),
                    rows[third].clone(),
                ];
                if (0..3).any(|a| {
                    ((a + 1)..3).any(|b| !(0..3).any(|test| matrix[test][a] != matrix[test][b]))
                }) {
                    continue;
                }
                let problem = MinimumDecisionTree::new(matrix, 3, 3);
                let expected = BruteForce::new().solve(&problem).unwrap().unwrap();
                let actual = solve(&problem).unwrap();
                assert_eq!(
                    problem.evaluate(&actual).unwrap(),
                    problem.evaluate(&expected).unwrap()
                );
            }
        }
    }
}

#[test]
fn test_subset_dp_minimum_decision_tree_handles_eight_objects() {
    let matrix = (0..3)
        .map(|bit| (0..8).map(|object| object & (1 << bit) != 0).collect())
        .collect();
    let problem = MinimumDecisionTree::new(matrix, 8, 3);
    let solution = solve(&problem).unwrap();
    assert_eq!(problem.evaluate(&solution).unwrap().0, Some(24));
}
