use super::*;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;

#[test]
fn consecutive_sets_closed_loop_and_counts() {
    for subsets in [
        vec![],
        vec![vec![], vec![0]],
        vec![vec![0, 1], vec![1, 2]],
        vec![vec![0, 1]; 2],
        vec![vec![0, 1], vec![1, 2], vec![0, 2]],
    ] {
        let source = TwoDimensionalConsecutiveSets::new(3, subsets);
        let reduced = ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).unwrap();
        crate::rules::test_helpers::assert_parameter_predictions(&source, &reduced);
        let expected = BruteForce::new().solve(&source).unwrap();
        match (expected, ILPSolver::new().solve(&source)) {
            (Some(_), Ok(witness)) => assert!(source.evaluate(&witness).unwrap().0),
            (None, Err(crate::solvers::ILPSolveError::Infeasible)) => {}
            other => panic!("consecutive sets disagree: {other:?}"),
        }
    }
}

#[test]
fn consecutive_sets_preserve_shared_groups_and_unused_labels() {
    let source = TwoDimensionalConsecutiveSets::new(4, vec![vec![0, 1], vec![1, 2]]);
    let reduced = ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).unwrap();
    let witness = reduced.extract_solution(&vec![1, 2, 1, 0, 1, 0]).unwrap();
    assert_eq!(witness, vec![1, 2, 1, 0]);
    assert!(source.evaluate(&witness).unwrap().0);
    for invalid in [vec![], vec![0; 6], vec![0, 2, 3, 1, 1, 1]] {
        assert!(reduced.extract_solution(&invalid).is_err());
    }
}
