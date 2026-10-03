use super::*;
use crate::{
    solvers::{BruteForce, ILPSolveError, ILPSolver},
    Problem,
};

#[test]
fn indexed_triples_preserve_duplicate_items_and_impossibility() {
    for sizes in [vec![4, 5, 6, 4, 6, 5], vec![4, 4, 4, 6, 6, 6], vec![5; 6]] {
        let source = ThreePartition::new(sizes, 15);
        let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
        crate::rules::test_helpers::assert_parameter_predictions(&source, &reduction);
        let expected = BruteForce::new().solve(&source).unwrap();
        match (expected, ILPSolver::new().solve(reduction.target_problem())) {
            (Some(_), Ok(witness)) => {
                assert!(
                    source
                        .evaluate(&reduction.extract_solution(&witness).unwrap())
                        .unwrap()
                        .0
                );
                assert!(reduction.extract_solution(&vec![0; witness.len()]).is_err());
                assert!(reduction.extract_solution(&vec![1; witness.len()]).is_err());
            }
            (None, Err(ILPSolveError::Infeasible)) => {
                assert!(reduction.extract_solution(&vec![]).is_err())
            }
            other => panic!("partition disagreement: {other:?}"),
        }
        assert!(reduction
            .extract_solution(&vec![0; reduction.target_problem().num_vars() + 1])
            .is_err());
    }
}
