use super::*;
use crate::solvers::{BruteForce, ILPSolveError, ILPSolver};
use crate::traits::Problem;
use crate::types::Min;
#[test]
fn test_minimumweightandorgraph_to_ilp_closed_loop() {
    for (n, arcs, gates, weights, expected) in [
        (
            3,
            vec![(0, 1), (0, 2), (1, 2)],
            vec![Some(true), Some(false), None],
            vec![1, 2, -4],
            Some(-1),
        ),
        (
            3,
            vec![(0, 1)],
            vec![Some(true), Some(false), None],
            vec![1],
            None,
        ),
        (
            4,
            vec![(0, 1), (2, 3), (3, 2)],
            vec![Some(true), None, Some(false), Some(false)],
            vec![1, -8, -8],
            Some(1),
        ),
        (
            3,
            vec![(0, 1), (1, 2), (2, 1)],
            vec![Some(true), Some(false), Some(false)],
            vec![1, -2, -2],
            Some(-3),
        ),
        (
            3,
            vec![(0, 1), (1, 2)],
            vec![Some(true), None, Some(false)],
            vec![1, -5],
            Some(-4),
        ),
        (
            1,
            vec![(0, 0), (0, 0)],
            vec![Some(false)],
            vec![-1, 2],
            Some(-1),
        ),
        (1, vec![], vec![Some(true)], vec![], Some(0)),
        (1, vec![], vec![Some(false)], vec![], None),
    ] {
        let source = MinimumWeightAndOrGraph::new(n, arcs, 0, gates, weights);
        let reduced = ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).unwrap();
        crate::rules::test_helpers::assert_parameter_predictions(&source, &reduced);
        let brute = BruteForce::new().solve(&source).unwrap();
        assert_eq!(
            brute.as_ref().map(|w| source.evaluate(w).unwrap()),
            expected.map(|v| Min(Some(v)))
        );
        match (expected, ILPSolver::new().solve(&source)) {
            (Some(v), Ok(w)) => assert_eq!(source.evaluate(&w).unwrap(), Min(Some(v))),
            (None, Err(ILPSolveError::Infeasible)) => {}
            other => panic!("AND/OR graph disagrees: {other:?}"),
        }
    }
}
#[test]
fn and_or_extraction_rejects_unreachable_selections_and_preserves_overflow() {
    let source = MinimumWeightAndOrGraph::new(
        3,
        vec![(0, 1), (2, 2)],
        0,
        vec![Some(true), None, Some(true)],
        vec![1, -8],
    );
    let reduced = ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).unwrap();
    assert_eq!(
        reduced
            .extract_solution(&vec![1, 0, 1, 1, 0, 1, 0])
            .unwrap(),
        vec![true, false]
    );
    for invalid in [vec![], vec![1, 1, 1, 1, 1, 1, 0], vec![2, 0, 1, 1, 0, 1, 0]] {
        assert!(reduced.extract_solution(&invalid).is_err());
    }
    for weights in [
        vec![i64::MAX, 1, -1],
        vec![i64::MIN, -1, 1],
        vec![i64::MAX, -1, 1],
    ] {
        let source = MinimumWeightAndOrGraph::new(1, vec![(0, 0); 3], 0, vec![Some(true)], weights);
        let reduced = ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).unwrap();
        let target = vec![1, 1, 1, 1, 0, 0, 0];
        match source.evaluate(&vec![true; 3]) {
            Ok(Min(Some(value))) => {
                assert_eq!(
                    reduced.target_problem().evaluate(&target).unwrap().value,
                    Some(value)
                );
                assert_eq!(reduced.extract_solution(&target).unwrap(), vec![true; 3]);
            }
            Err(crate::traits::EvaluationError::IntegerOverflow(_)) => {
                assert!(matches!(
                    reduced.target_problem().evaluate(&target),
                    Err(crate::traits::EvaluationError::IntegerOverflow(_))
                ));
                assert!(reduced.extract_solution(&target).is_err());
            }
            other => panic!("unexpected source evaluation: {other:?}"),
        }
    }
}
