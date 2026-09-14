use super::*;
use crate::models::algebraic::ClosestVectorProblem;
use crate::traits::Problem;

#[test]
fn test_subsetsum_to_closestvectorproblem_closed_loop() {
    let source = SubsetSum::new(vec![3u32, 7, 1, 8], 11u32).unwrap();
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
        Some(BigRational::from_integer(4.into()))
    );
}

#[test]
fn test_subsetsum_to_closestvectorproblem_structure() {
    let source = SubsetSum::new(vec![3u32, 7, 1, 8], 11u32).unwrap();
    let reduction = ReduceTo::<ClosestVectorProblem<i64>>::reduce_to(&source).unwrap();
    let target = reduction.target_problem();

    assert_eq!(target.num_basis_vectors(), 7);
    assert_eq!(target.ambient_dimension(), 12);
    assert_eq!(&target.target()[..8], &[0, 0, 0, 0, 1, 1, 1, 1]);
    assert_eq!(
        ClosestVectorProblem::<i64>::variant(),
        vec![("target", "i64")]
    );
}

#[test]
fn test_subsetsum_to_closestvectorproblem_binary_minimizers() {
    let source = SubsetSum::new(vec![3u32, 7, 1, 8], 11u32).unwrap();
    let reduction = ReduceTo::<ClosestVectorProblem<i64>>::reduce_to(&source).unwrap();
    let target = reduction.target_problem();

    for solution in [vec![1, 0, 0, 1, 0, 0, 0], vec![1, 1, 1, 0, 1, 1, 1]] {
        assert_eq!(
            target.evaluate(&solution).unwrap().0,
            Some(BigRational::from_integer(4.into()))
        );
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
    let source = SubsetSum::new(vec![2u32, 4, 6], 5u32).unwrap();
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
            > BigRational::from_integer(source.num_elements().into())
    );
}

#[test]
fn test_subsetsum_to_closestvectorproblem_binary_carries_preserve_large_inputs() {
    use num_bigint::BigUint;
    let size = BigUint::from(1u32) << 70usize;
    let source = SubsetSum::new(vec![size.clone()], size).unwrap();
    let result = ReduceTo::<ClosestVectorProblem<i64>>::reduce_to(&source).unwrap();
    let mut witness = vec![0; result.target_problem().num_basis_vectors()];
    witness[0] = 1;
    assert_eq!(
        result.target_problem().evaluate(&witness).unwrap(),
        Min(Some(BigRational::from_integer(1.into())))
    );
    assert_eq!(result.extract_solution(&witness).unwrap(), vec![true]);
    assert!(result
        .target_problem()
        .basis()
        .iter()
        .flatten()
        .all(|&x| (-2..=1).contains(&x)));

    let source = SubsetSum::new(vec![1u32; 40], 20u32).unwrap();
    let result = ReduceTo::<ClosestVectorProblem<i64>>::reduce_to(&source).unwrap();
    let mut witness = vec![0; result.target_problem().num_basis_vectors()];
    witness[..20].fill(1);
    witness[40..].copy_from_slice(&[1, 2, 5, 10]);
    assert!(
        source
            .evaluate(&result.extract_solution(&witness).unwrap())
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
        let source = SubsetSum::new(sizes, target_sum).unwrap();
        let result = ReduceTo::<ClosestVectorProblem<i64>>::reduce_to(&source).unwrap();
        let target = result.target_problem();
        assert!(std::ptr::eq(
            target,
            crate::rules::AggregateReductionResult::target_problem(&result)
        ));
        let dimensions = target.num_basis_vectors();
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
            let value = target.evaluate(&config).unwrap();
            let certificate = value
                == Min(Some(BigRational::from_integer(
                    source.num_elements().into(),
                )));
            assert_eq!(
                crate::rules::AggregateReductionResult::extract_value(&result, value),
                Or(certificate)
            );
            if certificate {
                let x = result.extract_solution(&config).unwrap();
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
        assert!(
            !matches!(crate::traits::Problem::evaluate(crate::rules::ReductionResult::target_problem(&result), &vec![0; dimensions + 1]), Ok(value) if { let value = crate::rules::AggregateReductionResult::extract_value(&result, value.clone()); value.is_valid() })
        );
        assert_eq!(
            crate::rules::AggregateReductionResult::extract_value(&result, Min(None)),
            Or(false)
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
