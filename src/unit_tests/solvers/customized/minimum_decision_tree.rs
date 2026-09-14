use super::*;
use crate::registry::load_dyn;
use crate::solvers::{BruteForce, SolveOutcome, SolverExecution, SolverRequest};
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
                let problem = MinimumDecisionTree::new(matrix, 3, 3).unwrap();
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
    let problem = MinimumDecisionTree::new(matrix, 8, 3).unwrap();
    let loaded = load_dyn(
        MinimumDecisionTree::NAME,
        &Default::default(),
        serde_json::to_value(&problem).unwrap(),
    )
    .unwrap();
    let result = crate::solvers::solve(&loaded, SolverRequest::Default).unwrap();
    assert!(matches!(
        result.solver,
        SolverExecution::Customized {
            implementation: "subset-dp"
        }
    ));
    let SolveOutcome::Optimal {
        solution,
        evaluation,
    } = result.outcome
    else {
        panic!("the instance has a solution");
    };
    assert_eq!(evaluation, "Min(24)");
    let solution = serde_json::from_value(solution).unwrap();
    assert_eq!(problem.evaluate(&solution).unwrap().0, Some(24));
}

#[test]
fn subset_dp_reports_mask_and_table_representation_errors() {
    for n in [usize::BITS as usize, usize::BITS as usize - 1] {
        let tests = (n.ilog2() + 1) as usize;
        let matrix = (0..tests)
            .map(|bit| (0..n).map(|object| object & (1 << bit) != 0).collect())
            .collect();
        let problem = MinimumDecisionTree::new(matrix, n, tests).unwrap();
        let loaded = load_dyn(
            MinimumDecisionTree::NAME,
            &Default::default(),
            serde_json::to_value(&problem).unwrap(),
        )
        .unwrap();
        let error = crate::solvers::solve(&loaded, SolverRequest::Default).unwrap_err();
        if n == usize::BITS as usize {
            assert!(matches!(error, SolveError::IntegerOverflow(_)));
        } else {
            assert!(matches!(error, SolveError::Allocation(_)));
        }
    }
}
