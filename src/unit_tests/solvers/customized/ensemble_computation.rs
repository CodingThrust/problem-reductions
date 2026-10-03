use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn large_single_required_set_uses_an_optimal_union_chain() {
    let required: Vec<_> = (0..64).collect();
    for budget in [63, 70] {
        // Duplicate requirements and input ordering must not defeat the shortcut.
        let reversed: Vec<_> = required.iter().rev().copied().collect();
        let problem = EnsembleComputation::new(64, vec![required.clone(), reversed], budget);
        let solution = solve(&problem).unwrap().unwrap();
        assert_eq!(solution.len(), 2 * budget);
        assert_eq!(problem.evaluate(&solution).unwrap().0, Some(63));
    }
}

#[test]
fn compact_fallback_matches_brute_force_on_feasible_and_infeasible_programs() {
    for sets in [
        vec![vec![0, 1], vec![1, 2]],
        vec![vec![0, 1], vec![0, 1, 2]],
    ] {
        for budget in 1..=3 {
            let problem = EnsembleComputation::new(3, sets.clone(), budget);
            let expected = BruteForce::new().solve(&problem).unwrap();
            // Zero construction budget exercises the production fallback on
            // an instance small enough for independent exhaustive verification.
            let actual = solve_with_union_limit(&problem, 0).unwrap();
            assert_eq!(
                actual.as_ref().map(|s| problem.evaluate(s).unwrap()),
                expected.as_ref().map(|s| problem.evaluate(s).unwrap())
            );
        }
    }
}

#[test]
fn empty_requirements_and_unrepresentable_program_lengths() {
    let empty = EnsembleComputation::new(0, vec![], 2);
    let solution = solve(&empty).unwrap().unwrap();
    assert_eq!(solution, vec![0; 4]);
    assert_eq!(empty.evaluate(&solution).unwrap().0, Some(0));
    let too_long = EnsembleComputation::new(2, vec![vec![0, 1]], usize::MAX);
    assert!(matches!(
        solve(&too_long),
        Err(SolveError::IntegerOverflow(_))
    ));
    let fallback_error = EnsembleComputation::new(3, vec![vec![0, 1], vec![1, 2]], usize::MAX);
    assert!(matches!(
        solve_with_union_limit(&fallback_error, 0),
        Err(SolveError::IlpSolve { .. })
    ));
}

#[test]
fn large_required_sets_reject_insufficient_union_budgets() {
    let required: Vec<_> = (0..64).collect();
    for budget in [1, 62] {
        let problem = EnsembleComputation::new(64, vec![required.clone()], budget);
        assert_eq!(solve(&problem).unwrap(), None);
    }
}

#[test]
fn required_set_union_budget_includes_the_boundary() {
    for budget in [2, 3, 4] {
        let problem = EnsembleComputation::new(4, vec![vec![0, 1, 2, 3]], budget);
        let solution = solve(&problem).unwrap();
        assert_eq!(solution.is_some(), budget >= 3);
        if let Some(solution) = solution {
            assert_eq!(problem.evaluate(&solution).unwrap().0, Some(3));
        }
    }
}

#[test]
fn disjoint_required_sets_need_independent_union_operations() {
    // Each triple needs two unions, and disjoint triples cannot share a union.
    for budget in [3, 4] {
        let problem = EnsembleComputation::new(6, vec![vec![0, 1, 2], vec![3, 4, 5]], budget);
        let actual = solve(&problem).unwrap();
        assert_eq!(actual.is_some(), budget == 4);
        if let Some(solution) = actual {
            assert_eq!(problem.evaluate(&solution).unwrap().0, Some(4));
        }
    }
}

#[test]
fn test_useful_union_ensemble_computation_matches_brute_force() {
    let subsets = [
        vec![],
        vec![0],
        vec![1],
        vec![2],
        vec![0, 1],
        vec![0, 2],
        vec![1, 2],
        vec![0, 1, 2],
    ];
    for first in &subsets {
        for second in &subsets {
            let problem = EnsembleComputation::new(3, vec![first.clone(), second.clone()], 2);
            let expected = BruteForce::new().solve(&problem).unwrap();
            let actual = solve(&problem).unwrap();
            assert_eq!(
                actual
                    .as_ref()
                    .map(|solution| problem.evaluate(solution).unwrap()),
                expected
                    .as_ref()
                    .map(|solution| problem.evaluate(solution).unwrap())
            );
        }
    }
}

#[test]
fn test_useful_union_ensemble_computation_reuses_intermediate_sets() {
    let problem = EnsembleComputation::new(
        6,
        vec![vec![0, 1], vec![0, 1, 2, 3], vec![0, 1, 2, 3, 4, 5]],
        5,
    );
    let solution = solve(&problem).unwrap().unwrap();
    assert_eq!(problem.evaluate(&solution).unwrap().0, Some(5));
}
