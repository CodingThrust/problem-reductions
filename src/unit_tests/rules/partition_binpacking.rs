use super::*;
use crate::models::misc::{BinPacking, Partition};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_partition_to_binpacking_closed_loop() {
    let source = Partition::new(vec![3, 1, 1, 2, 2, 1]);
    let reduction = ReduceTo::<BinPacking<i32>>::reduce_to(&source);

    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &reduction,
        "Partition -> BinPacking closed loop",
    );
}

#[test]
fn test_partition_to_binpacking_structure() {
    let source = Partition::new(vec![3, 1, 1, 2, 2, 1]);
    let reduction = ReduceTo::<BinPacking<i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.sizes(), &[3, 1, 1, 2, 2, 1]);
    assert_eq!(*target.capacity(), 5); // total_sum = 10, capacity = 5
    assert_eq!(target.num_items(), source.num_elements());
}

#[test]
fn test_partition_to_binpacking_odd_total_is_not_satisfying() {
    // Sizes [2, 4, 5], total = 11 (odd), capacity = 5
    // No balanced partition possible; BinPacking needs >= 3 bins
    let source = Partition::new(vec![2, 4, 5]);
    let reduction = ReduceTo::<BinPacking<i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    let best = BruteForce::new()
        .find_witness(target)
        .expect("BinPacking target should always have an optimal solution");

    // With capacity 5, items [2,4,5]: bin 0 gets [5], bin 1 gets [2,4]=6 > 5,
    // so optimal needs 3 bins
    let value = target.evaluate(&best);
    assert_eq!(value, Min(Some(3)));

    let extracted = reduction.extract_solution(&best);
    assert!(!source.evaluate(&extracted));
}

#[test]
#[should_panic(
    expected = "Partition -> BinPacking requires all sizes and total_sum / 2 to fit in i32"
)]
fn test_partition_to_binpacking_panics_on_large_coefficients() {
    let source = Partition::new(vec![(i32::MAX as u64) + 1]);
    let _ = ReduceTo::<BinPacking<i32>>::reduce_to(&source);
}
