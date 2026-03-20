use super::*;
use crate::models::algebraic::{ClosestVectorProblem, VarBounds};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;
use crate::types::SolutionSize;
use std::collections::HashSet;

#[test]
fn test_subsetsum_to_closestvectorproblem_closed_loop() {
    let source = SubsetSum::new(vec![3u32, 7, 1, 8], 11u32);
    let reduction = ReduceTo::<ClosestVectorProblem<i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_basis_vectors(), 4);
    assert_eq!(target.ambient_dimension(), 5);
    assert_eq!(target.bounds(), &[VarBounds::binary(); 4]);

    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &reduction,
        "SubsetSum -> ClosestVectorProblem closed loop",
    );
}

#[test]
fn test_subsetsum_to_closestvectorproblem_structure() {
    let source = SubsetSum::new(vec![3u32, 7, 1, 8], 11u32);
    let reduction = ReduceTo::<ClosestVectorProblem<i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.basis()[0], vec![1, 0, 0, 0, 3]);
    assert_eq!(target.basis()[1], vec![0, 1, 0, 0, 7]);
    assert_eq!(target.basis()[2], vec![0, 0, 1, 0, 1]);
    assert_eq!(target.basis()[3], vec![0, 0, 0, 1, 8]);
    assert_eq!(target.target(), &[0.5, 0.5, 0.5, 0.5, 11.0]);
}

#[test]
fn test_subsetsum_to_closestvectorproblem_issue_example_minimizers() {
    let source = SubsetSum::new(vec![3u32, 7, 1, 8], 11u32);
    let reduction = ReduceTo::<ClosestVectorProblem<i32>>::reduce_to(&source);
    let target = reduction.target_problem();
    let solutions: HashSet<Vec<usize>> = BruteForce::new()
        .find_all_best(target)
        .into_iter()
        .collect();

    let expected: HashSet<Vec<usize>> = [vec![1, 0, 0, 1], vec![1, 1, 1, 0]].into_iter().collect();
    assert_eq!(solutions, expected);

    for solution in &solutions {
        assert_eq!(target.evaluate(solution), SolutionSize::Valid(1.0));
    }
}

#[test]
fn test_subsetsum_to_closestvectorproblem_unsatisfiable_instance() {
    let source = SubsetSum::new(vec![2u32, 4, 6], 5u32);
    let reduction = ReduceTo::<ClosestVectorProblem<i32>>::reduce_to(&source);
    let target = reduction.target_problem();
    let best = BruteForce::new()
        .find_best(target)
        .expect("unsatisfiable instance should still have a best CVP assignment");

    match target.evaluate(&best) {
        SolutionSize::Valid(value) => assert!(value > (source.num_elements() as f64).sqrt() / 2.0),
        SolutionSize::Invalid => panic!("CVP solution should be valid"),
    }
}

#[test]
#[should_panic(
    expected = "SubsetSum -> ClosestVectorProblem requires all sizes and target to fit in i32"
)]
fn test_subsetsum_to_closestvectorproblem_panics_on_large_coefficients() {
    let source = SubsetSum::new(vec![(i32::MAX as u64) + 1], 1u64);
    let _ = ReduceTo::<ClosestVectorProblem<i32>>::reduce_to(&source);
}
