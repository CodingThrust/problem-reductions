use super::*;
use crate::models::misc::{Knapsack, SubsetSum};
use crate::rules::test_helpers::{
    assert_satisfaction_round_trip_from_optimization_target, solve_optimization_problem,
};
use crate::traits::Problem;
use num_bigint::BigUint;

#[test]
fn test_subsetsum_to_knapsack_closed_loop() {
    let source = SubsetSum::new(vec![3u32, 7, 1, 8, 4, 12, 5], 15u32);
    let reduction = ReduceTo::<Knapsack>::reduce_to(&source);

    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &reduction,
        "SubsetSum->Knapsack closed loop",
    );
}

#[test]
fn test_subsetsum_to_knapsack_target_structure() {
    let source = SubsetSum::new(vec![3u32, 7, 1, 8, 4, 12, 5], 15u32);
    let reduction = ReduceTo::<Knapsack>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.weights(), &[3, 7, 1, 8, 4, 12, 5]);
    assert_eq!(target.values(), &[3, 7, 1, 8, 4, 12, 5]);
    assert_eq!(target.capacity(), 15);
    assert_eq!(target.num_items(), source.num_elements());
}

#[test]
fn test_subsetsum_to_knapsack_unsat_extracts_non_solution() {
    let source = SubsetSum::new(vec![2u32, 4, 6], 5u32);
    let reduction = ReduceTo::<Knapsack>::reduce_to(&source);

    let target_solution = solve_optimization_problem(reduction.target_problem())
        .expect("SubsetSum->Knapsack: optimization target should always have an optimum");
    let extracted = reduction.extract_solution(&target_solution);

    assert!(!source.evaluate(&extracted));
}

#[test]
#[should_panic(
    expected = "SubsetSum -> Knapsack reduction requires all sizes and target to fit in i64"
)]
fn test_subsetsum_to_knapsack_panics_on_i64_overflow() {
    let too_large = BigUint::from(i64::MAX as u64) + BigUint::from(1u32);
    let source = SubsetSum::new_unchecked(vec![too_large.clone()], too_large);

    let _ = ReduceTo::<Knapsack>::reduce_to(&source);
}

#[cfg(feature = "example-db")]
#[test]
fn test_subsetsum_to_knapsack_canonical_example_spec() {
    let spec = canonical_rule_example_specs()
        .into_iter()
        .find(|spec| spec.id == "subsetsum_to_knapsack")
        .expect("missing canonical SubsetSum -> Knapsack example spec");
    let example = (spec.build)();

    assert_eq!(example.source.problem, "SubsetSum");
    assert_eq!(example.target.problem, "Knapsack");
    assert_eq!(
        example.solutions,
        vec![crate::export::SolutionPair {
            source_config: vec![1, 0, 0, 0, 0, 1, 0],
            target_config: vec![1, 0, 0, 0, 0, 1, 0],
        }]
    );
}
