use super::direct_qubo_tests::{feasible, optimum};
use crate::models::graph::MinimumFeedbackArcSet;
use crate::models::misc::*;
use crate::models::set::{ExactCoverBy3Sets, ThreeDimensionalMatching};
use crate::topology::MultiDigraph;

#[test]
fn test_individual_deadlines_to_qubo_closed_loop() {
    feasible(
        SchedulingWithIndividualDeadlines::new(3, 2, vec![1, 1, 2], vec![(0, 2)]),
        true,
        6,
    );
    feasible(
        SchedulingWithIndividualDeadlines::new(2, 1, vec![1, 1], vec![]),
        false,
        3,
    );
}
#[test]
fn test_precedence_scheduling_to_qubo_closed_loop() {
    feasible(
        PrecedenceConstrainedScheduling::new(3, 2, 2, vec![(0, 2)]),
        true,
        10,
    );
    feasible(
        PrecedenceConstrainedScheduling::new(2, 2, 2, vec![(0, 1), (1, 0)]),
        false,
        4,
    );
}
#[test]
fn test_cyclic_ordering_to_qubo_closed_loop() {
    feasible(CyclicOrdering::new(3, vec![(0, 1, 2)]), true, 3);
    feasible(CyclicOrdering::new(3, vec![(0, 1, 2), (0, 2, 1)]), false, 3);
}
#[test]
fn test_grouping_swaps_to_qubo_closed_loop() {
    feasible(GroupingBySwapping::new(3, vec![0, 1, 2, 0], 2), true, 3);
    feasible(GroupingBySwapping::new(3, vec![0, 1, 2, 0], 0), false, 3);
}
#[test]
fn test_feedback_arc_set_to_qubo_closed_loop() {
    optimum(
        MinimumFeedbackArcSet::new(
            MultiDigraph::new(3, vec![(0, 1), (1, 2), (2, 0)]),
            vec![-2i64, 3, 1],
        ),
        -2,
        3,
    );
}
#[test]
fn test_partition_to_qubo_closed_loop() {
    feasible(Partition::new(vec![1, 2, 3]).unwrap(), true, 3);
    feasible(Partition::new(vec![1, 1, 1]).unwrap(), false, 3);
}
#[test]
fn test_subset_sum_to_qubo_closed_loop() {
    feasible(SubsetSum::new(vec![1u32, 2, 4], 3u32), true, 3);
    feasible(SubsetSum::new(vec![1u32, 2, 4], 8u32), false, 3);
}
#[test]
fn test_exact_cover_to_qubo_closed_loop() {
    feasible(
        ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [3, 4, 5], [0, 3, 4]]),
        true,
        3,
    );
    feasible(
        ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [0, 1, 3], [0, 1, 4]]),
        false,
        3,
    );
}
#[test]
fn test_three_partition_to_qubo_closed_loop() {
    feasible(ThreePartition::new(vec![2, 2, 2], 6), true, 1);
    feasible(ThreePartition::new(vec![4, 4, 4, 6, 6, 6], 15), false, 0);
    feasible(ThreePartition::new(vec![2; 6], 6), true, 2);
    feasible(
        ThreePartition::new(vec![6, 4, 5, 6, 4, 5, 6, 4, 5], 15),
        true,
        3,
    );
    feasible(
        ThreePartition::new(vec![1_000_000_000; 3], 3_000_000_000),
        true,
        1,
    );
}
#[test]
fn test_three_dimensional_matching_to_qubo_closed_loop() {
    feasible(
        ThreeDimensionalMatching::new(3, vec![(0, 1, 2), (1, 0, 1), (2, 2, 0)]),
        true,
        3,
    );
    feasible(
        ThreeDimensionalMatching::new(3, vec![(0, 0, 0), (1, 1, 1), (1, 2, 2)]),
        false,
        3,
    );
}
#[test]
fn test_stacker_crane_to_qubo_closed_loop() {
    optimum(
        StackerCrane::new(3, vec![(0, 1), (1, 2), (2, 0)], vec![], vec![1; 3], vec![]),
        3,
        9,
    );
}
#[test]
fn test_superstring_to_qubo_closed_loop() {
    optimum(
        ShortestCommonSuperstring::new(
            2,
            vec![vec![0, 1], vec![1, 0], vec![], vec![0, 1], vec![0]],
        ),
        3,
        4,
    );
}

use super::direct_qubo_tests::direct;
use crate::models::{algebraic::QUBO, decision::Decision};
use crate::rules::{AggregateReductionResult, ReduceTo, ReductionResult};
use crate::types::{Min, Or};
use crate::{BruteForce, Problem};

#[test]
fn test_deadline_capacity_and_compressed_horizon_boundaries() {
    feasible(
        SchedulingWithIndividualDeadlines::new(0, 0, vec![], vec![]),
        true,
        0,
    );
    feasible(
        SchedulingWithIndividualDeadlines::new(1, 0, vec![1], vec![]),
        false,
        1,
    );
    feasible(
        SchedulingWithIndividualDeadlines::new(1, 1, vec![0], vec![]),
        false,
        0,
    );
    feasible(
        SchedulingWithIndividualDeadlines::new(1, usize::MAX, vec![i64::MAX], vec![]),
        true,
        1,
    );
    feasible(
        SchedulingWithIndividualDeadlines::new(2, 2, vec![2, 2], vec![(0, 0)]),
        false,
        4,
    );
    feasible(
        PrecedenceConstrainedScheduling::new(0, 0, 0, vec![]),
        true,
        0,
    );
    feasible(
        PrecedenceConstrainedScheduling::new(2, 2, i64::MAX, vec![(0, 1), (0, 1)]),
        true,
        4,
    );
}

#[test]
fn test_ordering_empty_alphabet_unused_symbols_and_padding() {
    feasible(CyclicOrdering::new(1, vec![]), true, 0);
    feasible(
        CyclicOrdering::new(3, vec![(2, 0, 1), (1, 2, 0), (2, 0, 1)]),
        true,
        3,
    );
    feasible(GroupingBySwapping::new(0, vec![], 0), true, 0);
    feasible(GroupingBySwapping::new(1, vec![0, 0], 3), true, 0);
    feasible(GroupingBySwapping::new(4, vec![0, 2, 0], 3), true, 6);
}

#[test]
fn test_feedback_signed_parallel_arcs_and_loops() {
    optimum(
        MinimumFeedbackArcSet::new(
            MultiDigraph::new(2, vec![(0, 1), (0, 1), (1, 0), (1, 1)]),
            vec![-3i64, 2, 4, -1],
        ),
        -2,
        1,
    );
    optimum(
        MinimumFeedbackArcSet::new(MultiDigraph::new(1, vec![(0, 0), (0, 0)]), vec![4i64, -2]),
        2,
        0,
    );
    optimum(
        MinimumFeedbackArcSet::new(MultiDigraph::new(0, vec![]), Vec::<i64>::new()),
        0,
        0,
    );
}

#[test]
fn test_selection_empty_duplicate_and_uncovered_elements() {
    feasible(Partition::new(vec![1]).unwrap(), false, 1);
    feasible(SubsetSum::new(Vec::<u32>::new(), 0u32), true, 0);
    feasible(SubsetSum::new(Vec::<u32>::new(), 1u32), false, 0);
    feasible(ExactCoverBy3Sets::new(0, vec![]), true, 0);
    feasible(ExactCoverBy3Sets::new(3, vec![]), false, 0);
    feasible(
        ExactCoverBy3Sets::new(3, vec![[0, 1, 2], [0, 1, 2]]),
        true,
        2,
    );
    feasible(ThreeDimensionalMatching::new(0, vec![]), true, 0);
    feasible(ThreeDimensionalMatching::new(1, vec![]), false, 0);
    feasible(
        ThreeDimensionalMatching::new(1, vec![(0, 0, 0), (0, 0, 0)]),
        true,
        2,
    );
    assert!(ThreePartition::try_new(vec![], 1).is_err());
}

fn infeasible_optimum<S>(source: S)
where
    S: Problem<Value = Min<i64>> + ReduceTo<QUBO<i64>> + 'static,
    S::Solution: 'static,
    <S as ReduceTo<QUBO<i64>>>::Result: AggregateReductionResult<Source = S, Target = QUBO<i64>>,
{
    assert!(BruteForce::new().solve(&source).unwrap().is_none());
    let reduction = <S as ReduceTo<QUBO<i64>>>::reduce_to(&source).unwrap();
    let target = ReductionResult::target_problem(&reduction);
    for bits in BruteForce::new().find_all_witnesses(target).unwrap() {
        assert_eq!(
            reduction.extract_value(target.evaluate(&bits).unwrap()),
            Min(None)
        );
        assert!(reduction.extract_solution(&bits).is_err());
    }
}

#[test]
fn test_stacker_disconnected_empty_and_single_arc() {
    infeasible_optimum(StackerCrane::new(
        6,
        vec![(0, 1), (2, 3), (4, 5)],
        vec![],
        vec![1; 3],
        vec![],
    ));
    optimum(StackerCrane::new(0, vec![], vec![], vec![], vec![]), 0, 0);
    optimum(
        StackerCrane::new(2, vec![(0, 1)], vec![(0, 1)], vec![2], vec![3]),
        5,
        1,
    );
    optimum(
        StackerCrane::new(1, vec![(0, 0)], vec![], vec![2], vec![]),
        2,
        1,
    );
}

#[test]
fn test_superstring_empty_contained_and_invalid_alphabet() {
    optimum(
        ShortestCommonSuperstring::new(0, vec![vec![], vec![]]),
        0,
        0,
    );
    optimum(
        ShortestCommonSuperstring::new(2, vec![vec![], vec![0], vec![0, 1], vec![0, 1]]),
        2,
        1,
    );
    optimum(
        ShortestCommonSuperstring::new(2, vec![vec![0], vec![1]]),
        2,
        4,
    );
    infeasible_optimum(ShortestCommonSuperstring::new(
        2,
        vec![vec![0], vec![1], vec![2]],
    ));
}

fn overflow<S, T>(source: S)
where
    S: Problem + ReduceTo<T>,
    T: Problem,
{
    assert!(matches!(
        <S as ReduceTo<T>>::reduce_to(&source),
        Err(crate::rules::ReductionError::IntegerOverflow { .. })
    ));
}

#[test]
fn test_source_and_target_arithmetic_boundaries() {
    overflow::<_, Decision<QUBO<i64>>>(Partition::new(vec![i64::MAX]).unwrap());
    overflow::<_, Decision<QUBO<i64>>>(SubsetSum::new(
        vec![num_bigint::BigUint::from(1u32) << 130],
        0u32,
    ));
    overflow::<_, Decision<QUBO<i64>>>(SubsetSum::new(
        vec![1u32],
        num_bigint::BigUint::from(1u32) << 130,
    ));
    // Opposing parallel costs cancel in the polynomial but source removal prefixes overflow.
    overflow::<_, QUBO<i64>>(MinimumFeedbackArcSet::new(
        MultiDigraph::new(2, vec![(0, 1), (0, 1), (1, 0), (1, 0)]),
        vec![i64::MAX, 1, -i64::MAX, -1],
    ));
    overflow::<_, QUBO<i64>>(MinimumFeedbackArcSet::new(
        MultiDigraph::new(1, vec![(0, 0), (0, 0)]),
        vec![i64::MIN, -1],
    ));
    overflow::<_, QUBO<i64>>(StackerCrane::new(
        1,
        vec![(0, 0), (0, 0)],
        vec![],
        vec![i64::MAX, 1],
        vec![],
    ));
    overflow::<_, QUBO<i64>>(StackerCrane::new(
        1,
        vec![(0, 0)],
        vec![],
        vec![i64::MAX],
        vec![],
    ));
    // Expand with BigInt: s^2 and 2Ts cancel even though each exceeds i64.
    feasible(SubsetSum::new(vec![1u64 << 32], 1u64 << 31), false, 1);
}

#[test]
fn test_aggregate_identity_and_registered_parameter_bounds() {
    let source = ThreeDimensionalMatching::new(2, vec![(0, 0, 0), (1, 1, 1), (0, 1, 0)]);
    let reduction =
        <ThreeDimensionalMatching as ReduceTo<Decision<QUBO<i64>>>>::reduce_to(&source).unwrap();
    crate::rules::test_helpers::assert_parameter_predictions(&source, &reduction);
    assert_eq!(reduction.extract_value(Or(false)), Or(false));
    assert_eq!(reduction.extract_value(Or(true)), Or(true));
    crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "perfect matching",
    );
    let source = MinimumFeedbackArcSet::new(
        MultiDigraph::new(3, vec![(0, 1), (1, 2), (2, 0)]),
        vec![1i64, 2, 3],
    );
    let reduction =
        <MinimumFeedbackArcSet<i64> as ReduceTo<QUBO<i64>>>::reduce_to(&source).unwrap();
    crate::rules::test_helpers::assert_parameter_predictions(&source, &reduction);
    crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "directed cycle",
    );
    let _ = direct::<_, QUBO<i64>>(&source);
}
