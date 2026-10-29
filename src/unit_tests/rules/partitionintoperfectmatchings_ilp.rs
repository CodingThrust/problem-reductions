use super::*;
use crate::solvers::{BruteForce, ILPSolveError, ILPSolver};
use crate::traits::Problem;
#[test]
fn test_partitionintoperfectmatchings_to_ilp_closed_loop() {
    for n in 1..=4 {
        let pairs: Vec<_> = (0..n)
            .flat_map(|u| (u + 1..n).map(move |v| (u, v)))
            .collect();
        for mask in 0..1 << pairs.len() {
            let edges = pairs
                .iter()
                .enumerate()
                .filter_map(|(i, &e)| (mask >> i & 1 == 1).then_some(e))
                .collect();
            let graph = SimpleGraph::new(n, edges);
            for k in 1..=n {
                let source = PartitionIntoPerfectMatchings::new(graph.clone(), k);
                let reduced = ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).unwrap();
                crate::rules::test_helpers::assert_parameter_predictions(&source, &reduced);
                match (
                    BruteForce::new().solve(&source).unwrap(),
                    ILPSolver::new().solve(&source),
                ) {
                    (Some(_), Ok(w)) => assert!(source.evaluate(&w).unwrap().0),
                    (None, Err(ILPSolveError::Infeasible)) => {}
                    other => panic!("matching partition disagrees: {other:?}"),
                }
            }
        }
    }
}
