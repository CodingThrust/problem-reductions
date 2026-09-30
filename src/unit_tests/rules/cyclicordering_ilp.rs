use super::*;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;

#[test]
fn cyclic_ordering_closed_loop_and_counts() {
    for (n, triples) in [
        (1, vec![]),
        (3, vec![(0, 1, 2)]),
        (3, vec![(0, 1, 2); 2]),
        (3, vec![(0, 1, 2), (0, 2, 1)]),
    ] {
        let source = CyclicOrdering::new(n, triples);
        let reduced = ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).unwrap();
        crate::rules::test_helpers::assert_parameter_predictions(&source, &reduced);
        let expected = BruteForce::new().solve(&source).unwrap();
        match (expected, ILPSolver::new().solve(&source)) {
            (Some(_), Ok(witness)) => assert!(source.evaluate(&witness).unwrap().0),
            (None, Err(crate::solvers::ILPSolveError::Infeasible)) => {}
            other => panic!("cyclic ordering disagrees: {other:?}"),
        }
    }
}

#[test]
fn cyclic_ordering_decodes_inverse_positions_and_unrelated_ties() {
    let source = CyclicOrdering::new(4, vec![(0, 1, 2)]);
    let reduced = ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).unwrap();
    let witness = reduced
        .extract_solution(&vec![1, 2, 0, 1, 1, 0, 1])
        .unwrap();
    assert_eq!(witness, vec![1, 3, 0, 2]);
    assert!(source.evaluate(&witness).unwrap().0);
    for invalid in [vec![], vec![0; 7], vec![1, 2, 0, 1, 1, 0, 2]] {
        assert!(reduced.extract_solution(&invalid).is_err());
    }
}
