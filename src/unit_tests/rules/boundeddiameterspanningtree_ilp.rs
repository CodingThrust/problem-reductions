use super::*;
use crate::solvers::{BruteForce, ILPSolveError, ILPSolver};
use crate::traits::Problem;
#[test]
fn test_boundeddiameterspanningtree_to_ilp_closed_loop() {
    for (n, edges, weights) in [
        (0, vec![], vec![]),
        (1, vec![(0, 0)], vec![3]),
        (3, vec![(0, 1)], vec![1]),
        (4, vec![(0, 1), (1, 2), (2, 3)], vec![1, 2, 1]),
        (
            4,
            vec![(0, 1), (0, 2), (0, 3), (1, 2), (2, 3)],
            vec![3, 1, 2, 1, 1],
        ),
        (3, vec![(0, 0), (0, 1), (1, 0), (1, 2)], vec![1, 4, 1, 1]),
    ] {
        for diameter in 1..=5 {
            for budget in [1, 2, 4, 8] {
                let source = BoundedDiameterSpanningTree::new(
                    SimpleGraph::new(n, edges.clone()),
                    weights.clone(),
                    budget,
                    diameter,
                );
                let reduced = ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).unwrap();
                crate::rules::test_helpers::assert_parameter_predictions(&source, &reduced);
                match (
                    BruteForce::new().solve(&source).unwrap(),
                    ILPSolver::new().solve(&source),
                ) {
                    (Some(_), Ok(w)) => assert!(source.evaluate(&w).unwrap().0),
                    (None, Err(ILPSolveError::Infeasible)) => {}
                    other => panic!("bounded tree disagrees: {other:?}"),
                }
            }
        }
    }
}
#[test]
fn bounded_tree_extracts_center_edge_and_rejects_invalid_certificates() {
    let source = BoundedDiameterSpanningTree::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![1; 3],
        3,
        3,
    );
    let reduced = ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).unwrap();
    let target = vec![1, 1, 1, 0, 1, 0, 0, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 0];
    assert_eq!(reduced.extract_solution(&target).unwrap(), vec![true; 3]);
    for invalid in [
        vec![],
        vec![0; 20],
        {
            let mut x = target.clone();
            x[18] = 0;
            x
        },
        {
            let mut x = target;
            x[13] = 2;
            x
        },
    ] {
        assert!(reduced.extract_solution(&invalid).is_err());
    }
    let huge = BoundedDiameterSpanningTree::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![i64::MAX, 1, 1],
        2,
        2,
    );
    assert!(huge.evaluate(&vec![false, true, true]).unwrap().0);
    assert!(matches!(
        ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&huge),
        Err(crate::rules::ReductionError::IntegerOverflow { .. })
    ));
}
