use super::*;
use crate::rules::traits::ReductionResult;
use crate::rules::ReduceTo;
use crate::solvers::{BruteForce, SolveOutcome};
use crate::traits::Problem;
use crate::types::Or;

fn unit_cycle_cover(bound: i64) -> Decision<MinimumVertexCover<SimpleGraph, One>> {
    Decision::new(
        MinimumVertexCover::new(SimpleGraph::cycle(5), vec![One; 5]),
        bound,
    )
}

#[test]
fn test_decisionminimumvertexcover_one_to_i64_cast_closed_loop() {
    // C5 has minimum vertex cover size 3.
    let source = unit_cycle_cover(3);
    let reduction = ReduceTo::<Decision<MinimumVertexCover<SimpleGraph, i64>>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();

    assert_eq!(target.inner().graph(), source.inner().graph());
    assert_eq!(target.inner().weights(), &[1_i64; 5]);
    assert_eq!(target.bound(), &3);

    let target_solution = BruteForce::new().solve(target).unwrap().unwrap();
    let source_solution = reduction
        .recover_result(
            &source,
            SolveOutcome::optimal(target, target_solution.clone()).unwrap(),
        )
        .unwrap()
        .into_solution()
        .expect("qualifying target result must recover a source solution");

    assert_eq!(source_solution, target_solution);
    assert_eq!(source.evaluate(&source_solution).unwrap(), Or(true));
}

#[test]
fn test_decisionminimumvertexcover_one_to_i64_cast_preserves_infeasibility() {
    let source = unit_cycle_cover(2);
    let reduction = ReduceTo::<Decision<MinimumVertexCover<SimpleGraph, i64>>>::reduce_to(&source)
        .expect("reduction should succeed");

    assert_eq!(BruteForce::new().solve(&source).unwrap(), None);
    assert_eq!(
        BruteForce::new().solve(reduction.target_problem()).unwrap(),
        None
    );
    assert!(matches!(
        reduction
            .recover_result(&source, SolveOutcome::Infeasible)
            .unwrap(),
        SolveOutcome::Infeasible
    ));
}
