use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::rules::test_helpers::assert_bf_vs_ilp;
use crate::rules::{ReduceTo, ReductionResult};

#[test]
fn mutually_incompatible_edges_reduce_factor_search_symmetry() {
    use crate::solvers::ILPSolver;
    use crate::traits::Problem;
    let source = BMF::new(
        vec![
            vec![true, false, false],
            vec![false, true, false],
            vec![false, false, true],
        ],
        3,
    );
    let reduced = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
    // No two diagonal edges can share a factor. After naming their factors,
    // each diagonal needs at most one additional coverage bit.
    assert!(reduced.target_problem().num_vars() <= 21);
    let solution = ILPSolver::new().solve(&source).unwrap();
    assert_eq!(source.evaluate(&solution).unwrap().0, Some(6));
}

#[test]
fn test_bmf_to_ilp_structure() {
    // 2x2 identity matrix, rank 1
    let problem = BMF::new(vec![vec![true, false], vec![false, true]], 1);
    let reduction: ReductionBMFToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();
    // The first anchored edge fixes its factor; the other diagonal is impossible.
    assert_eq!(ilp.num_vars(), 4);
    assert_eq!(ilp.num_constraints(), 5);
    assert_eq!(ilp.num_nonzeros(), 4);
    assert_eq!(ilp.sense(), ObjectiveSense::Minimize);
}

#[test]
fn test_bmf_to_ilp_closed_loop() {
    // 2x2 identity, rank 2 — exact factorization exists.
    // Use ILP solver on target (fast) + brute force on source (tiny 2x2).
    let problem = BMF::new(vec![vec![true, false], vec![false, true]], 2);
    let reduction: ReductionBMFToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    assert_bf_vs_ilp(&problem, &reduction);
}

#[test]
fn test_bmf_to_ilp_bf_vs_ilp() {
    // All-ones 2x2 has an exact rank-1 factorization (boolean rank 1).
    let problem = BMF::new(vec![vec![true, true], vec![true, true]], 1);
    let reduction: ReductionBMFToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    assert_bf_vs_ilp(&problem, &reduction);
}

#[test]
fn test_bmf_to_ilp_trivial() {
    // 1x1 matrix, rank 1
    let problem = BMF::new(vec![vec![true]], 1);
    let reduction: ReductionBMFToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();
    // Both factor bits are fixed by the only edge, so coverage needs no extra bit.
    assert_eq!(ilp.num_vars(), 2);
}

#[test]
fn bmf_extraction_requires_exact_reconstruction() {
    let source = BMF::new(vec![vec![true]], 1);
    let reduced = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
    assert!(reduced
        .extract_solution(&vec![0; reduced.target_problem().num_vars()])
        .is_err());
}

#[test]
fn bmf_sparse_coverage_preserves_values_and_zero_dimensions() {
    use crate::solvers::{ILPSolveError, ILPSolver};
    use crate::traits::Problem;
    use crate::types::Min;
    for (matrix, k, expected) in [
        (vec![], 0, Some(0)),
        (vec![], 2, Some(0)),
        (vec![vec![], vec![]], 2, Some(0)),
        (vec![vec![false; 2]; 2], 0, Some(0)),
        (vec![vec![true]], 0, None),
        (vec![vec![true, false], vec![false, true]], 1, None),
        (vec![vec![true, false], vec![false, true]], 2, Some(4)),
        (vec![vec![true; 3]; 2], 2, Some(5)),
    ] {
        let source = BMF::new(matrix, k);
        let reduced = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
        crate::rules::test_helpers::assert_parameter_predictions(&source, &reduced);
        match (expected, ILPSolver::new().solve(&source)) {
            (Some(value), Ok(w)) => assert_eq!(source.evaluate(&w).unwrap(), Min(Some(value))),
            (None, Err(ILPSolveError::Infeasible)) => {}
            other => panic!("Boolean factorization disagrees: {other:?}"),
        }
    }
    let source = BMF::new(vec![vec![true]], 2);
    let reduced = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
    // The anchored first factor covers the edge; extra memberships remain valid.
    let witness = reduced.extract_solution(&vec![1, 1, 1, 1]).unwrap();
    assert_eq!(source.evaluate(&witness).unwrap(), Min(Some(4)));
    assert!(reduced.extract_solution(&vec![0, 0, 0, 0]).is_err());
    assert!(reduced.extract_solution(&vec![]).is_err());
    let huge = BMF::new(vec![vec![false]], usize::MAX);
    assert!(matches!(
        ReduceTo::<ILP<bool>>::reduce_to(&huge),
        Err(crate::rules::ReductionError::IntegerOverflow { .. })
    ));
}
