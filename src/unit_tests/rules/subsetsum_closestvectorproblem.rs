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

    let expected: serde_json::Value = serde_json::json!({"basis": [[1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1], [0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1], [0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1], [0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0, 1, -2, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, -2, 0], [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, -2]], "target": [0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1]});
    assert_eq!(serde_json::to_value(target).unwrap(), expected);
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

    for solution in [vec![1, 0, 0, 1, 0, 0, 0], vec![1, 1, 1, 0, 1, 1, 1]] {
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
fn test_subsetsum_to_closestvectorproblem_large_integers_and_unit_pivots() {
    use num_bigint::BigUint;
    let size = BigUint::from(1u32) << 70usize;
    let source = SubsetSum::new(vec![size.clone()], size);
    let result = ReduceTo::<ClosestVectorProblem<i64>>::reduce_to(&source).unwrap();
    let mut witness = vec![0; result.target_problem().num_basis_vectors()];
    witness[0] = 1;
    assert_eq!(
        result.target_problem().evaluate(&witness).unwrap(),
        Min(Some(1.0))
    );
    assert_eq!(result.extract_solution(&witness).unwrap(), vec![true]);
    assert!(result
        .target_problem()
        .basis()
        .iter()
        .flatten()
        .all(|&x| (-2..=1).contains(&x)));

    let source = SubsetSum::new(vec![1u32; 40], 20u32);
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
        let source = SubsetSum::new(sizes, target_sum);
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
            let certificate = value == Min(Some(result.target_distance));
            assert_eq!(
                crate::rules::AggregateReductionResult::extract_value(&result, value),
                Or(certificate)
            );
            match result.extract_solution(&config) {
                Ok(x) => {
                    assert!(certificate);
                    assert!(source.evaluate(&x).unwrap().0);
                    accepted = true;
                }
                Err(_) => assert!(!certificate),
            }
        }
        assert_eq!(
            accepted,
            crate::solvers::BruteForce::new()
                .solve(&source)
                .unwrap()
                .is_some()
        );
        assert!(result.extract_solution(&vec![0; dimensions + 1]).is_err());
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
