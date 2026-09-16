use super::*;
use crate::models::algebraic::ClosestVectorProblem;
use crate::models::decision::Decision;
use crate::rules::ReductionResult;
use crate::solvers::SolveOutcome;
use crate::traits::EvaluationError::InvalidConfiguration;
use crate::traits::Problem;
use crate::types::OptimizationValue;

#[test]
fn test_subsetsum_to_closestvectorproblem_closed_loop() {
    let source = SubsetSum::new(vec![3u32, 7, 1, 8], 11u32);
    let reduction = ReduceTo::<Decision<ClosestVectorProblem>>::reduce_to(&source).unwrap();
    let target_solution = crate::solvers::customized::closest_vector_problem::solve(
        reduction.target_problem().inner(),
    )
    .unwrap();
    let source_solution = reduction
        .recover_result(
            &source,
            SolveOutcome::optimal(reduction.target_problem(), target_solution.clone()).unwrap(),
        )
        .unwrap()
        .into_solution()
        .expect("qualifying target result must recover a source solution");

    assert!(source.evaluate(&source_solution).unwrap().0);
    assert_eq!(
        reduction
            .target_problem()
            .inner()
            .evaluate(&target_solution)
            .unwrap()
            .0,
        Some(4)
    );
}

#[test]
fn test_subsetsum_to_closestvectorproblem_structure() {
    let source = SubsetSum::new(vec![3u32, 7, 1, 8], 11u32);
    let reduction = ReduceTo::<Decision<ClosestVectorProblem>>::reduce_to(&source).unwrap();
    let target = reduction.target_problem();

    assert_eq!(target.inner().num_basis_vectors(), 7);
    assert_eq!(target.inner().ambient_dimension(), 12);
    assert_eq!(&target.inner().target()[..8], &[0, 0, 0, 0, 1, 1, 1, 1]);
    assert_eq!(
        ClosestVectorProblem::variant(),
        vec![("coefficient", "i64")]
    );
}

#[test]
fn test_subsetsum_to_closestvectorproblem_binary_minimizers() {
    let source = SubsetSum::new(vec![3u32, 7, 1, 8], 11u32);
    let reduction = ReduceTo::<Decision<ClosestVectorProblem>>::reduce_to(&source).unwrap();
    let target = reduction.target_problem();

    for solution in [vec![1, 0, 0, 1, 0, 0, 0], vec![1, 1, 1, 0, 1, 1, 1]] {
        assert_eq!(target.inner().evaluate(&solution).unwrap().0, Some(4));
        assert!(
            source
                .evaluate(
                    &reduction
                        .recover_result(
                            &source,
                            SolveOutcome::optimal(reduction.target_problem(), solution.clone())
                                .unwrap()
                        )
                        .unwrap()
                        .into_solution()
                        .expect("qualifying target result must recover a source solution")
                )
                .unwrap()
                .0
        );
    }
}

#[test]
fn test_subsetsum_to_closestvectorproblem_unsatisfiable_instance() {
    let source = SubsetSum::new(vec![2u32, 4, 6], 5u32);
    let reduction = ReduceTo::<Decision<ClosestVectorProblem>>::reduce_to(&source).unwrap();
    let solution = crate::solvers::customized::closest_vector_problem::solve(
        reduction.target_problem().inner(),
    )
    .unwrap();
    assert!(
        reduction
            .target_problem()
            .inner()
            .evaluate(&solution)
            .unwrap()
            .unwrap()
            > i64::try_from(source.num_elements()).unwrap()
    );
}

#[test]
fn test_subsetsum_to_closestvectorproblem_binary_carries_preserve_large_inputs() {
    use num_bigint::BigUint;
    let size = BigUint::from(1u32) << 70usize;
    let source = SubsetSum::new(vec![size.clone()], size);
    let result = ReduceTo::<Decision<ClosestVectorProblem>>::reduce_to(&source).unwrap();
    let mut witness = vec![0; result.target_problem().inner().num_basis_vectors()];
    witness[0] = 1;
    assert_eq!(
        result.target_problem().inner().evaluate(&witness).unwrap(),
        crate::types::Min(Some(1))
    );
    assert_eq!(
        result
            .recover_result(
                &source,
                SolveOutcome::optimal(result.target_problem(), witness.clone()).unwrap()
            )
            .unwrap()
            .into_solution()
            .expect("qualifying target result must recover a source solution"),
        vec![true]
    );
    assert!(result
        .target_problem()
        .inner()
        .basis()
        .iter()
        .flatten()
        .all(|&x| (-2..=1).contains(&x)));

    let source = SubsetSum::new(vec![1u32; 40], 20u32);
    let result = ReduceTo::<Decision<ClosestVectorProblem>>::reduce_to(&source).unwrap();
    let mut witness = vec![0; result.target_problem().inner().num_basis_vectors()];
    witness[..20].fill(1);
    witness[40..].copy_from_slice(&[1, 2, 5, 10]);
    assert!(
        source
            .evaluate(
                &result
                    .recover_result(
                        &source,
                        SolveOutcome::optimal(result.target_problem(), witness.clone()).unwrap()
                    )
                    .unwrap()
                    .into_solution()
                    .expect("qualifying target result must recover a source solution")
            )
            .unwrap()
            .0
    );
}

#[test]
fn test_subsetsum_to_closestvectorproblem_all_small_coefficients() {
    for (sizes, target_sum) in [
        (vec![], 0u32),
        (vec![], 1),
        (vec![1], 2),
        (vec![1, 1], 2),
        (vec![3, 7, 1], 11),
        (vec![2, 4], 5),
    ] {
        let source = SubsetSum::new(sizes, target_sum);
        let result = ReduceTo::<Decision<ClosestVectorProblem>>::reduce_to(&source).unwrap();
        let target = result.target_problem();
        assert!(std::ptr::eq(
            target,
            crate::rules::ReductionResult::target_problem(&result)
        ));
        let dimensions = target.inner().num_basis_vectors();
        let mut accepted = false;
        for index in 0..4usize.pow(dimensions as u32) {
            let mut index = index;
            let config: Vec<i64> = (0..dimensions)
                .map(|_| {
                    let x = (index % 4) as i64 - 1;
                    index /= 4;
                    x
                })
                .collect();
            let value = target.inner().evaluate(&config).unwrap();
            let certificate =
                value == crate::types::Min(Some(i64::try_from(source.num_elements()).unwrap()));
            assert_eq!(
                crate::types::Or(OptimizationValue::meets_bound(
                    &(value),
                    crate::rules::ReductionResult::target_problem(&result).bound()
                )),
                crate::types::Or(certificate)
            );
            if certificate {
                let x = result
                    .recover_result(
                        &source,
                        SolveOutcome::optimal(result.target_problem(), config.clone()).unwrap(),
                    )
                    .unwrap()
                    .into_solution()
                    .expect("qualifying target result must recover a source solution");
                assert!(source.evaluate(&x).unwrap().0);
                accepted = true;
            }
        }
        assert_eq!(
            accepted,
            crate::solvers::BruteForce::new()
                .solve(&source)
                .unwrap()
                .is_some()
        );
        assert!(matches!(
            ReductionResult::target_problem(&result)
                .inner()
                .evaluate(&vec![0; dimensions + 1]),
            Err(InvalidConfiguration(_))
        ));
        assert_eq!(
            crate::types::Or(OptimizationValue::meets_bound(
                &(crate::types::Min(None)),
                crate::rules::ReductionResult::target_problem(&result).bound()
            )),
            crate::types::Or(false)
        );
    }
}

#[test]
fn test_subsetsum_to_closestvectorproblem_dimension_boundaries() {
    type R = ReductionSubsetSumToClosestVectorProblem;
    assert_eq!(R::dimensions(4, 4).unwrap(), (4, 12, 7));
    assert_eq!(R::dimensions(0, 1).unwrap(), (1, 1, 0));
    assert!(R::dimensions(0, 0).is_err());
    assert!(R::dimensions(usize::MAX, 2).is_err());
    assert!(R::dimensions(usize::MAX, 1).is_err());
    assert!(R::dimensions(usize::MAX / 2, 1).is_err());
    if usize::BITS == 64 {
        assert!(R::dimensions(1usize << 30, 1).is_err());
        assert!(R::dimensions((1usize << 30) - 1, 1).is_ok());
    }
}

#[test]
fn test_subset_sum_to_cvp_recovers_selected_items() {
    let source = SubsetSum::new(vec![3u32, 5, 7], 8u32);
    let reduction = ReduceTo::<Decision<ClosestVectorProblem>>::reduce_to(&source).unwrap();
    let target = reduction.target_problem();
    let solution =
        crate::solvers::customized::closest_vector_problem::solve(target.inner()).unwrap();
    assert_eq!(target.inner().evaluate(&solution).unwrap().0, Some(3));
    let recovered = reduction
        .recover_result(&source, SolveOutcome::optimal(target, solution).unwrap())
        .unwrap()
        .into_solution()
        .unwrap();
    assert_eq!(recovered, vec![true, true, false]);
    assert!(source.evaluate(&recovered).unwrap().0);
}
