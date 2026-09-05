use super::*;
use crate::models::algebraic::ClosestVectorProblem;
use crate::traits::Problem;

#[test]
fn test_subsetsum_to_closestvectorproblem_closed_loop() {
    let source = SubsetSum::new(vec![3u32, 7, 1, 8], 11u32);
    let reduction = ReduceTo::<ClosestVectorProblem<i64>>::reduce_to(&source).unwrap();
    let target_solution =
        crate::solvers::customized::closest_vector_problem::solve(reduction.target_problem())
            .unwrap();
    let source_solution = reduction.extract_solution(&target_solution).unwrap();

    assert!(source.evaluate(&source_solution).unwrap().0);
    assert_eq!(
        reduction
            .target_problem()
            .evaluate(&target_solution)
            .unwrap()
            .0,
        Some(2.0)
    );
}

#[test]
fn test_subsetsum_to_closestvectorproblem_structure() {
    let source = SubsetSum::new(vec![3u32, 7, 1, 8], 11u32);
    let reduction = ReduceTo::<ClosestVectorProblem<i64>>::reduce_to(&source).unwrap();
    let target = reduction.target_problem();

    assert_eq!(target.basis()[0], vec![2, 0, 0, 0, 6]);
    assert_eq!(target.basis()[1], vec![0, 2, 0, 0, 14]);
    assert_eq!(target.basis()[2], vec![0, 0, 2, 0, 2]);
    assert_eq!(target.basis()[3], vec![0, 0, 0, 2, 16]);
    assert_eq!(target.target(), &[1, 1, 1, 1, 22]);
    assert_eq!(
        ClosestVectorProblem::<i64>::variant(),
        vec![("target", "i64")]
    );
}

#[test]
fn test_subsetsum_to_closestvectorproblem_binary_minimizers() {
    let source = SubsetSum::new(vec![3u32, 7, 1, 8], 11u32);
    let reduction = ReduceTo::<ClosestVectorProblem<i64>>::reduce_to(&source).unwrap();
    let target = reduction.target_problem();

    for solution in [vec![1, 0, 0, 1], vec![1, 1, 1, 0]] {
        assert_eq!(target.evaluate(&solution).unwrap().0, Some(2.0));
        assert!(
            source
                .evaluate(&reduction.extract_solution(&solution).unwrap())
                .unwrap()
                .0
        );
    }
}

#[test]
fn test_subsetsum_to_closestvectorproblem_unsatisfiable_instance() {
    let source = SubsetSum::new(vec![2u32, 4, 6], 5u32);
    let reduction = ReduceTo::<ClosestVectorProblem<i64>>::reduce_to(&source).unwrap();
    let solution =
        crate::solvers::customized::closest_vector_problem::solve(reduction.target_problem())
            .unwrap();
    assert!(
        reduction
            .target_problem()
            .evaluate(&solution)
            .unwrap()
            .unwrap()
            > (source.num_elements() as f64).sqrt()
    );
}

#[test]
fn test_subsetsum_to_closestvectorproblem_reports_target_overflow() {
    let outside_i64 = SubsetSum::new(vec![(i64::MAX as u64) + 1], 1u64);
    assert!(matches!(
        ReduceTo::<ClosestVectorProblem<i64>>::reduce_to(&outside_i64),
        Err(crate::rules::ReductionError::Construction {
            cause: crate::registry::ConstructionError::IntegerOverflow(_),
            ..
        })
    ));

    let scaling_overflow = SubsetSum::new(vec![(i64::MAX / 2 + 1) as u64], 1u64);
    assert!(matches!(
        ReduceTo::<ClosestVectorProblem<i64>>::reduce_to(&scaling_overflow),
        Err(crate::rules::ReductionError::IntegerOverflow { .. })
    ));
}
