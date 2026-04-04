use crate::models::misc::SubsetProduct;
use crate::models::set::ExactCoverBy3Sets;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::rules::{ReduceTo, ReductionResult};
use num_bigint::BigUint;

#[test]
fn test_exactcoverby3sets_to_subsetproduct_closed_loop() {
    let source = ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [3, 4, 5], [0, 3, 4]]);
    let reduction = ReduceTo::<SubsetProduct>::reduce_to(&source);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "ExactCoverBy3Sets -> SubsetProduct closed loop",
    );
}

#[test]
fn test_exactcoverby3sets_to_subsetproduct_structure() {
    let source = ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [3, 4, 5], [0, 3, 4]]);
    let reduction = ReduceTo::<SubsetProduct>::reduce_to(&source);
    let target = reduction.target_problem();

    let expected_sizes: Vec<BigUint> = vec![30u64, 1001, 154]
        .into_iter()
        .map(BigUint::from)
        .collect();
    assert_eq!(target.sizes(), &expected_sizes[..]);
    assert_eq!(target.target(), &BigUint::from(30030u64));
    assert_eq!(target.num_elements(), 3);
}

#[test]
fn test_exactcoverby3sets_to_subsetproduct_extract_solution_is_identity() {
    let source = ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [3, 4, 5], [0, 3, 4]]);
    let reduction = ReduceTo::<SubsetProduct>::reduce_to(&source);

    assert_eq!(reduction.extract_solution(&[1, 0, 1]), vec![1, 0, 1]);
}
