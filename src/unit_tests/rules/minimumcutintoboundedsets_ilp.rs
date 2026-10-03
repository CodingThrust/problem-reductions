use super::*;
use crate::models::algebraic::ILP;
use crate::models::graph::MinimumCutIntoBoundedSets;
use crate::rules::test_helpers::assert_bf_vs_ilp;
use crate::rules::ReduceTo;
use crate::topology::SimpleGraph;
use crate::traits::Problem;

fn small_instance() -> MinimumCutIntoBoundedSets<SimpleGraph, i64> {
    // Path graph 0-1-2-3, unit weights, s=0, t=3, B=3
    MinimumCutIntoBoundedSets::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![1, 1, 1],
        0,
        3,
        3,
    )
}

#[test]
fn test_minimumcutintoboundedsets_to_ilp_closed_loop() {
    let source = small_instance();
    let reduction: ReductionMinCutBSToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&source).expect("reduction should succeed");
    assert_bf_vs_ilp(&source, &reduction);
}

#[test]
fn test_reduction_shape() {
    let source = small_instance();
    let reduction: ReductionMinCutBSToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&source).expect("reduction should succeed");
    let ilp = reduction.target_problem();
    // 4 vertex vars + 3 edge vars = 7
    assert_eq!(ilp.num_vars(), 7);
}

#[test]
fn test_extract_solution() {
    let source = small_instance();
    let reduction: ReductionMinCutBSToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&source).expect("reduction should succeed");
    let target_sol = vec![0, 0, 1, 1, 0, 1, 0];
    let extracted = reduction.extract_solution(&target_sol).unwrap();
    assert_eq!(extracted, vec![false, false, true, true]);
    assert!(source.evaluate(&extracted).unwrap().0.is_some());
}

#[test]
fn test_larger_instance() {
    let source = MinimumCutIntoBoundedSets::new(
        SimpleGraph::new(
            6,
            vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (0, 2), (3, 5)],
        ),
        vec![1, 2, 1, 2, 1, 2, 1],
        0,
        5,
        4,
    );
    let reduction: ReductionMinCutBSToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&source).expect("reduction should succeed");
    assert_bf_vs_ilp(&source, &reduction);
}

#[test]
fn test_minimumcutintoboundedsets_to_ilp_bf_vs_ilp() {
    let source = small_instance();
    let reduction: ReductionMinCutBSToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&source).expect("reduction should succeed");
    crate::rules::test_helpers::assert_bf_vs_ilp(&source, &reduction);
}

#[test]
fn test_partition_bound_normalization_preserves_the_optimum() {
    for bound in [0, 1, 2, 1000] {
        let source = MinimumCutIntoBoundedSets::new(
            SimpleGraph::new(2, vec![(0, 1)]),
            vec![3_i64],
            0,
            1,
            bound,
        );
        let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
        assert!(reduction.target_problem().max_constraint_magnitude_bits() <= 2);
        let solution = crate::solvers::ILPSolver::new().solve(reduction.target_problem());
        assert_eq!(solution.is_ok(), bound >= 1);
        if let Ok(solution) = solution {
            let recovered = reduction.extract_solution(&solution).unwrap();
            assert_eq!(source.evaluate(&recovered).unwrap().0, Some(3));
        } else {
            assert_eq!(
                solution.unwrap_err(),
                crate::solvers::ILPSolveError::Infeasible
            );
        }
    }
}
