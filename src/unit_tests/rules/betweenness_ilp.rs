use super::*;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;

#[test]
fn betweenness_closed_loop_and_counts() {
    for (n, triples) in [
        (1, vec![]),
        (3, vec![(0, 1, 2)]),
        (3, vec![(0, 1, 2); 2]),
        (3, vec![(0, 1, 2), (0, 2, 1)]),
    ] {
        let source = Betweenness::new(n, triples);
        let reduced = ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).unwrap();
        crate::rules::test_helpers::assert_parameter_predictions(&source, &reduced);
        let expected = BruteForce::new().solve(&source).unwrap();
        match (expected, ILPSolver::new().solve(&source)) {
            (Some(_), Ok(witness)) => assert!(source.evaluate(&witness).unwrap().0),
            (None, Err(crate::solvers::ILPSolveError::Infeasible)) => {}
            other => panic!("betweenness disagrees: {other:?}"),
        }
    }
}

#[test]
fn betweenness_decodes_both_directions_without_distinct_unrelated_ranks() {
    let source = Betweenness::new(4, vec![(0, 1, 2)]);
    let reduced = ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).unwrap();
    for target in [vec![0, 1, 2, 1, 1], vec![2, 1, 0, 1, 0]] {
        let witness = reduced.extract_solution(&target).unwrap();
        assert!(source.evaluate(&witness).unwrap().0);
        let mut sorted = witness;
        sorted.sort();
        assert_eq!(sorted, vec![0, 1, 2, 3]);
    }
    for invalid in [vec![], vec![0; 5], vec![0, 1, 2, 1, 0], vec![0, 1, 4, 1, 1]] {
        assert!(reduced.extract_solution(&invalid).is_err());
    }
}
