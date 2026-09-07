use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_breadth_first_search_ensemble_computation_matches_brute_force() {
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
            let actual = solve(&problem);
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
fn test_breadth_first_search_ensemble_computation_reuses_intermediate_sets() {
    let problem = EnsembleComputation::new(
        6,
        vec![vec![0, 1], vec![0, 1, 2, 3], vec![0, 1, 2, 3, 4, 5]],
        5,
    );
    let solution = solve(&problem).unwrap();
    assert_eq!(problem.evaluate(&solution).unwrap().0, Some(5));
}
