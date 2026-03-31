use super::*;
use crate::models::set::ExactCoverBy3Sets;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Max;

#[test]
fn test_exactcoverby3sets_to_maximumsetpacking_closed_loop() {
    let source = ExactCoverBy3Sets::new(
        6,
        vec![[0, 1, 2], [0, 1, 3], [3, 4, 5], [2, 4, 5], [1, 3, 5]],
    );
    let reduction = ReduceTo::<MaximumSetPacking<One>>::reduce_to(&source);

    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &reduction,
        "ExactCoverBy3Sets -> MaximumSetPacking closed loop",
    );
}

#[test]
fn test_exactcoverby3sets_to_maximumsetpacking_structure() {
    let source = ExactCoverBy3Sets::new(
        6,
        vec![[0, 1, 2], [0, 1, 3], [3, 4, 5], [2, 4, 5], [1, 3, 5]],
    );
    let reduction = ReduceTo::<MaximumSetPacking<One>>::reduce_to(&source);
    let target = reduction.target_problem();

    // Same number of sets as source subsets
    assert_eq!(target.num_sets(), source.num_subsets());
    assert_eq!(target.num_sets(), 5);

    // Each set should have exactly 3 elements (converted from [usize; 3] to Vec)
    for i in 0..target.num_sets() {
        assert_eq!(target.get_set(i).unwrap().len(), 3);
    }

    // Verify specific set contents
    assert_eq!(target.sets()[0], vec![0, 1, 2]);
    assert_eq!(target.sets()[1], vec![0, 1, 3]);
    assert_eq!(target.sets()[2], vec![3, 4, 5]);
    assert_eq!(target.sets()[3], vec![2, 4, 5]);
    assert_eq!(target.sets()[4], vec![1, 3, 5]);
}

#[test]
fn test_exactcoverby3sets_to_maximumsetpacking_unsatisfiable() {
    // Universe {0,1,2,3,4,5} but subsets cannot form an exact cover:
    // all subsets share element 0
    let source = ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [0, 3, 4], [0, 4, 5]]);
    let reduction = ReduceTo::<MaximumSetPacking<One>>::reduce_to(&source);
    let target = reduction.target_problem();

    // Best packing can only select one set (since all share element 0)
    let best = BruteForce::new()
        .find_witness(target)
        .expect("Should have an optimal solution");
    assert_eq!(target.evaluate(&best), Max(Some(1)));

    // q = 2, but packing value is 1 < 2, so no exact cover exists
    let extracted = reduction.extract_solution(&best);
    assert!(!source.evaluate(&extracted));
}

#[test]
fn test_exactcoverby3sets_to_maximumsetpacking_optimal_value() {
    // Satisfiable instance: S0={0,1,2}, S1={3,4,5} form an exact cover
    let source = ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [3, 4, 5], [0, 3, 4]]);
    let reduction = ReduceTo::<MaximumSetPacking<One>>::reduce_to(&source);
    let target = reduction.target_problem();

    let best = BruteForce::new()
        .find_witness(target)
        .expect("Should have an optimal solution");
    // Maximum packing: S0 + S1 = 2 disjoint sets = q
    assert_eq!(target.evaluate(&best), Max(Some(2)));

    let extracted = reduction.extract_solution(&best);
    assert!(source.evaluate(&extracted));
}
