use super::*;
use crate::solvers::{BruteForce, ILPSolveError, ILPSolver};
use crate::traits::Problem;

#[test]
fn test_minimumcodegenerationunlimitedregisters_to_ilp_closed_loop() {
    for (n, left, right) in [
        (0, vec![], vec![]),
        (3, vec![], vec![]),
        (3, vec![(0, 1), (1, 2)], vec![]),
        (3, vec![(0, 2), (1, 2)], vec![(0, 2), (1, 2)]),
        (
            5,
            vec![(1, 3), (2, 3), (0, 1)],
            vec![(1, 4), (2, 4), (0, 2)],
        ),
        (3, vec![(0, 1), (1, 2), (2, 0)], vec![]),
    ] {
        let source = MinimumCodeGenerationUnlimitedRegisters::new(n, left, right);
        let reduced = ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).unwrap();
        crate::rules::test_helpers::assert_parameter_predictions(&source, &reduced);
        match (
            BruteForce::new().solve(&source).unwrap(),
            ILPSolver::new().solve(&source),
        ) {
            (Some(expected), Ok(actual)) => assert_eq!(
                source.evaluate(&expected).unwrap(),
                source.evaluate(&actual).unwrap()
            ),
            (None, Err(ILPSolveError::Infeasible)) => {}
            other => panic!("code generation disagrees: {other:?}"),
        }
    }
}
#[test]
fn code_generation_extracts_tied_ranks_and_validates_target() {
    let source = MinimumCodeGenerationUnlimitedRegisters::new(4, vec![(0, 2), (1, 3)], vec![]);
    let reduced = ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).unwrap();
    // Unrelated operations may tie; nonoptimal copy bits still decode feasibly.
    assert_eq!(
        reduced.extract_solution(&vec![0, 0, 1, 1, 1]).unwrap(),
        vec![0, 1]
    );
    for invalid in [vec![], vec![0, 0, 0, 0, 0], vec![2, 0, 0, 0, 1]] {
        assert!(reduced.extract_solution(&invalid).is_err());
    }
    let reused = MinimumCodeGenerationUnlimitedRegisters::new(3, vec![(0, 2), (1, 2)], vec![]);
    let reduced = ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&reused).unwrap();
    assert!(reduced.extract_solution(&vec![0, 1, 0, 0, 1]).is_err());
    assert_eq!(
        reduced.extract_solution(&vec![0, 1, 1, 0, 1]).unwrap(),
        vec![0, 1]
    );
}
