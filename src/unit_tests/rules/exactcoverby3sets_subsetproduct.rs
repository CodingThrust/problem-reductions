use crate::models::misc::SubsetProduct;
use crate::models::set::ExactCoverBy3Sets;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::rules::{ReduceTo, ReductionResult};

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

    assert_eq!(target.values(), &[30, 1001, 154]);
    assert_eq!(target.target(), 30030);
    assert_eq!(target.num_elements(), 3);
}

#[test]
fn test_exactcoverby3sets_to_subsetproduct_extract_solution_is_identity() {
    let source = ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [3, 4, 5], [0, 3, 4]]);
    let reduction = ReduceTo::<SubsetProduct>::reduce_to(&source);

    assert_eq!(reduction.extract_solution(&[1, 0, 1]), vec![1, 0, 1]);
}

#[test]
#[should_panic(expected = "u64")]
fn test_exactcoverby3sets_to_subsetproduct_panics_when_target_overflows_u64() {
    let source = ExactCoverBy3Sets::new(
        18,
        vec![
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [9, 10, 11],
            [12, 13, 14],
            [15, 16, 17],
        ],
    );

    let _ = ReduceTo::<SubsetProduct>::reduce_to(&source);
}
