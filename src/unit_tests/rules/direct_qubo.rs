use crate::models::algebraic::{QuadraticAssignment, QUBO};
use crate::models::decision::Decision;
use crate::models::graph::OptimalLinearArrangement;
use crate::models::misc::{
    MaximumLikelihoodRanking, MinimumTardinessSequencing,
    SequencingToMinimizeWeightedCompletionTime, SequencingWithinIntervals, SumOfSquaresPartition,
    TimetableDesign,
};
use crate::rules::registry::reduction_entries;
use crate::rules::traits::DynReductionResult;
use crate::topology::SimpleGraph;
use crate::types::One;
use crate::types::{Min, Or};
use crate::{BruteForce, Problem};

fn direct<S: Problem + 'static, T: Problem>(source: &S) -> Box<dyn DynReductionResult> {
    let entry = reduction_entries()
        .into_iter()
        .find(|entry| {
            entry.source_name == S::NAME
                && entry.source_variant() == S::variant()
                && entry.target_name == T::NAME
                && entry.target_variant() == T::variant()
        })
        .expect("a direct QUBO reduction must be available");
    entry.reduce_fn.unwrap()(source).unwrap()
}

fn optimum<S: Problem<Value = Min<i64>> + 'static>(source: S, expected: i64, bits: usize)
where
    S::Solution: 'static,
{
    let source_witness = BruteForce::new().solve(&source).unwrap().unwrap();
    assert_eq!(
        source.evaluate(&source_witness).unwrap(),
        Min(Some(expected))
    );
    let reduction = direct::<S, QUBO<i64>>(&source);
    let target = reduction
        .target_problem_any()
        .downcast_ref::<QUBO<i64>>()
        .unwrap();
    assert_eq!(target.num_vars(), bits);
    let entry = reduction_entries()
        .into_iter()
        .find(|entry| {
            entry.source_name == S::NAME
                && entry.source_variant() == S::variant()
                && entry.target_name == "QUBO"
                && entry.target_variant() == QUBO::<i64>::variant()
        })
        .unwrap();
    let aggregate = entry.reduce_aggregate_fn.unwrap()(&source).unwrap();
    assert_eq!(
        aggregate
            .extract_value_dyn(serde_json::to_value(Min::<i64>(None)).unwrap())
            .unwrap(),
        serde_json::to_value(Min::<i64>(None)).unwrap()
    );
    // Every bitstring either certifies a valid source objective or is rejected.
    for mask in 0..1usize << bits {
        let witness: Vec<bool> = (0..bits).map(|i| mask & (1 << i) != 0).collect();
        let mapped: Min<i64> =
            serde_json::from_value(aggregate.extract_value_from_solution_dyn(&witness).unwrap())
                .unwrap();
        match reduction.extract_solution_dyn(&witness) {
            Ok(extracted) => {
                assert!(mapped.0.is_some());
                assert_eq!(
                    source
                        .evaluate(extracted.downcast_ref::<S::Solution>().unwrap())
                        .unwrap(),
                    mapped
                );
            }
            Err(_) => assert_eq!(mapped, Min(None)),
        }
    }
    for witness in BruteForce::new().find_all_witnesses(target).unwrap() {
        let extracted = reduction.extract_solution_dyn(&witness).unwrap();
        assert_eq!(
            source
                .evaluate(extracted.downcast_ref::<S::Solution>().unwrap())
                .unwrap(),
            Min(Some(expected))
        );
    }
    assert!(reduction
        .extract_solution_dyn(&vec![false; bits + 1])
        .is_err());
}

fn feasible<S: Problem<Value = Or> + 'static>(source: S, expected: bool, bits: usize)
where
    S::Solution: 'static,
{
    assert_eq!(
        BruteForce::new().solve(&source).unwrap().is_some(),
        expected
    );
    let reduction = direct::<S, Decision<QUBO<i64>>>(&source);
    let target = reduction
        .target_problem_any()
        .downcast_ref::<Decision<QUBO<i64>>>()
        .unwrap();
    assert_eq!(target.inner().num_vars(), bits);
    let witnesses = BruteForce::new().find_all_witnesses(target).unwrap();
    assert_eq!(!witnesses.is_empty(), expected);
    for mask in 0..1usize << bits {
        let witness: Vec<bool> = (0..bits).map(|i| mask & (1 << i) != 0).collect();
        assert_eq!(
            reduction.extract_solution_dyn(&witness).is_ok(),
            target.evaluate(&witness).unwrap().0
        );
    }
    for witness in witnesses {
        let extracted = reduction.extract_solution_dyn(&witness).unwrap();
        assert_eq!(
            source
                .evaluate(extracted.downcast_ref::<S::Solution>().unwrap())
                .unwrap(),
            Or(true)
        );
    }
    assert!(reduction
        .extract_solution_dyn(&vec![false; bits + 1])
        .is_err());
}

#[test]
fn test_weighted_completion_to_qubo_closed_loop() {
    // Short high-weight tasks first: completion costs 3 + 4 + 3.
    optimum(
        SequencingToMinimizeWeightedCompletionTime::new(vec![1; 3], vec![1, 2, 3], vec![]),
        10,
        3,
    );
    // Five jobs exercise overlapping triangle penalties: 5 + 8 + 9 + 8 + 5.
    optimum(
        SequencingToMinimizeWeightedCompletionTime::new(vec![1; 5], vec![1, 2, 3, 4, 5], vec![]),
        35,
        10,
    );
    optimum(
        SequencingToMinimizeWeightedCompletionTime::new(
            vec![1; 3],
            vec![1, 2, 3],
            vec![(0, 1), (1, 2)],
        ),
        14,
        3,
    );
}

#[test]
fn test_ranking_to_qubo_closed_loop() {
    optimum(
        MaximumLikelihoodRanking::new(vec![vec![0, 1, 2], vec![3, 0, 1], vec![2, 3, 0]]),
        4,
        3,
    );
}

#[test]
fn test_unit_tardiness_to_qubo_closed_loop() {
    optimum(
        MinimumTardinessSequencing::<One>::new(3, vec![1, 2, 3], vec![(0, 1)]),
        0,
        9,
    );
    optimum(
        MinimumTardinessSequencing::<One>::new(3, vec![0, 0, 0], vec![]),
        3,
        9,
    );
}

#[test]
fn test_quadratic_assignment_to_qubo_closed_loop() {
    optimum(
        QuadraticAssignment::new(
            vec![vec![0, 1, 1], vec![1, 0, 1], vec![1, 1, 0]],
            vec![vec![0, 1, 1], vec![1, 0, 1], vec![1, 1, 0]],
        ),
        6,
        9,
    );
    // Signed asymmetric rectangular input; diagonals of the cost matrix are ignored.
    optimum(
        QuadraticAssignment::new(
            vec![vec![99, -2], vec![3, 99]],
            vec![vec![7, 4, 1], vec![-1, 7, 2], vec![3, -2, 7]],
        ),
        -11,
        6,
    );
}

#[test]
fn test_sum_of_squares_to_qubo_closed_loop() {
    optimum(SumOfSquaresPartition::new(vec![1, 2, 3], 2), 18, 6);
    optimum(SumOfSquaresPartition::new(vec![1, 1, 1], 1), 9, 3);
}

#[test]
fn test_linear_arrangement_to_qubo_closed_loop() {
    optimum(
        OptimalLinearArrangement::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)])),
        2,
        9,
    );
}

#[test]
fn test_interval_sequencing_to_qubo_closed_loop() {
    feasible(
        SequencingWithinIntervals::new(vec![0, 1, 2], vec![1, 2, 3], vec![1; 3]).unwrap(),
        true,
        3,
    );
    feasible(
        SequencingWithinIntervals::new(vec![0; 3], vec![1; 3], vec![1; 3]).unwrap(),
        false,
        3,
    );
    feasible(
        SequencingWithinIntervals::new(vec![0], vec![0], vec![1]).unwrap(),
        false,
        0,
    );
}

#[test]
fn test_interval_sequencing_rejects_unrepresentable_allocations() {
    use crate::rules::{ReduceTo, ReductionError};

    let max_slots = usize::try_from(isize::MAX).unwrap() / std::mem::size_of::<(usize, i64, i64)>();
    let mut counts = vec![i64::try_from(max_slots + 1).unwrap()];
    if usize::try_from(i64::MAX).is_ok() {
        counts.push(i64::MAX);
    }
    for count in counts {
        let source = SequencingWithinIntervals::new(vec![0], vec![count - 1], vec![0]).unwrap();
        assert_eq!(source.evaluate(&vec![0]).unwrap(), Or(true));
        assert!(matches!(
            <_ as ReduceTo<Decision<QUBO<i64>>>>::reduce_to(&source),
            Err(ReductionError::IntegerOverflow { .. })
        ));
    }

    // Each task's slots fit individually, but their combined tuple array does not.
    let deadline = i64::try_from(max_slots / 2).unwrap();
    let source = SequencingWithinIntervals::new(vec![0; 2], vec![deadline; 2], vec![0; 2]).unwrap();
    assert_eq!(source.evaluate(&vec![0; 2]).unwrap(), Or(true));
    assert!(matches!(
        <_ as ReduceTo<Decision<QUBO<i64>>>>::reduce_to(&source),
        Err(ReductionError::IntegerOverflow { .. })
    ));
}

#[test]
fn test_interval_sequencing_allows_representable_time_boundaries() {
    feasible(
        SequencingWithinIntervals::new(vec![], vec![], vec![]).unwrap(),
        true,
        0,
    );
    feasible(
        SequencingWithinIntervals::new(vec![i64::MAX - 2], vec![i64::MAX], vec![0]).unwrap(),
        true,
        3,
    );
    feasible(
        SequencingWithinIntervals::new(
            vec![i64::MAX - 2, i64::MAX - 1, i64::MAX],
            vec![i64::MAX - 1, i64::MAX, i64::MAX],
            vec![1, 1, 0],
        )
        .unwrap(),
        true,
        3,
    );
}

#[test]
fn test_timetable_to_qubo_closed_loop() {
    feasible(
        TimetableDesign::new(
            3,
            3,
            1,
            vec![
                vec![true, false, false],
                vec![false, true, false],
                vec![false, false, true],
            ],
            vec![vec![true; 3]],
            vec![vec![1]; 3],
        ),
        true,
        3,
    );
    feasible(
        TimetableDesign::new(
            3,
            3,
            1,
            vec![vec![true, false, false]; 3],
            vec![vec![true; 3]],
            vec![vec![1]; 3],
        ),
        false,
        3,
    );
}

#[test]
fn test_direct_qubo_empty_instances() {
    optimum(
        SequencingToMinimizeWeightedCompletionTime::new(vec![], vec![], vec![]),
        0,
        0,
    );
    optimum(MaximumLikelihoodRanking::new(vec![]), 0, 0);
    optimum(
        MinimumTardinessSequencing::<One>::new(0, vec![], vec![]),
        0,
        0,
    );
    optimum(QuadraticAssignment::new(vec![], vec![]), 0, 0);
    optimum(
        OptimalLinearArrangement::new(SimpleGraph::new(0, vec![])),
        0,
        0,
    );
    feasible(
        SequencingWithinIntervals::new(vec![], vec![], vec![]).unwrap(),
        true,
        0,
    );
    feasible(
        TimetableDesign::new(0, 0, 0, vec![], vec![], vec![]),
        true,
        0,
    );
}

#[test]
fn test_direct_qubo_signed_and_degenerate_inputs() {
    // The two orders cost -5 and -1 with signed processing time and weights.
    optimum(
        SequencingToMinimizeWeightedCompletionTime::new(vec![-1, 2], vec![3, -2], vec![]),
        -5,
        1,
    );
    optimum(
        MaximumLikelihoodRanking::new(vec![vec![0, -4], vec![2, 0]]),
        -4,
        1,
    );
    optimum(
        OptimalLinearArrangement::new(SimpleGraph::new(3, vec![])),
        0,
        9,
    );
    feasible(
        SequencingWithinIntervals::new(vec![0, 0], vec![1, 1], vec![0, 1]).unwrap(),
        true,
        3,
    );
    feasible(
        TimetableDesign::new(
            2,
            1,
            1,
            vec![vec![true; 2]],
            vec![vec![true; 2]],
            vec![vec![2]],
        ),
        true,
        2,
    );
    for required in [-1, i64::MIN, 3, i64::MAX] {
        feasible(
            TimetableDesign::new(
                2,
                1,
                1,
                vec![vec![true; 2]],
                vec![vec![true; 2]],
                vec![vec![required]],
            ),
            false,
            if required > 0 { 2 } else { 0 },
        );
    }
}

fn infeasible_objective<S: Problem<Value = Min<i64>> + 'static>(source: S) {
    assert!(BruteForce::new().solve(&source).unwrap().is_none());
    let reduction = direct::<S, QUBO<i64>>(&source);
    let target = reduction
        .target_problem_any()
        .downcast_ref::<QUBO<i64>>()
        .unwrap();
    let entry = reduction_entries()
        .into_iter()
        .find(|e| {
            e.source_name == S::NAME
                && e.source_variant() == S::variant()
                && e.target_name == "QUBO"
                && e.target_variant() == QUBO::<i64>::variant()
        })
        .unwrap();
    let aggregate = entry.reduce_aggregate_fn.unwrap()(&source).unwrap();
    for witness in BruteForce::new().find_all_witnesses(target).unwrap() {
        assert!(reduction.extract_solution_dyn(&witness).is_err());
        assert_eq!(
            aggregate.extract_value_from_solution_dyn(&witness).unwrap(),
            serde_json::to_value(Min::<i64>(None)).unwrap()
        );
    }
}

#[test]
fn test_direct_qubo_cyclic_precedence_recovers_infeasibility() {
    for edges in [
        vec![(0, 0)],
        vec![(0, 1), (1, 2), (2, 0)],
        vec![(1, 0), (0, 1)],
    ] {
        infeasible_objective(SequencingToMinimizeWeightedCompletionTime::new(
            vec![1; 3],
            vec![1, 2, 3],
            edges.clone(),
        ));
        infeasible_objective(MinimumTardinessSequencing::<One>::new(
            3,
            vec![1, 2, 3],
            edges,
        ));
    }
    optimum(
        SequencingToMinimizeWeightedCompletionTime::new(
            vec![1; 3],
            vec![1, 2, 3],
            vec![(2, 1), (2, 1), (1, 0)],
        ),
        10,
        3,
    );
}

#[test]
fn test_direct_qubo_rejects_unrepresentable_coefficients() {
    use crate::rules::{ReduceTo, ReductionError};
    fn overflow<S: ReduceTo<QUBO<i64>>>(source: S) {
        assert!(matches!(
            source.reduce_to(),
            Err(ReductionError::IntegerOverflow { .. })
        ));
    }
    overflow(SequencingToMinimizeWeightedCompletionTime::new(
        vec![i64::MAX],
        vec![2],
        vec![],
    ));
    overflow(MaximumLikelihoodRanking::new(vec![
        vec![0, i64::MIN],
        vec![i64::MAX, 0],
    ]));
    overflow(QuadraticAssignment::new(
        vec![vec![0, i64::MAX], vec![0, 0]],
        vec![vec![0, 2], vec![2, 0]],
    ));
    overflow(SumOfSquaresPartition::new(vec![i64::MAX], 1));
}

#[test]
fn test_weighted_completion_rejects_overflow_hidden_by_zero_weights() {
    use crate::rules::{ReduceTo, ReductionError};
    for lengths in [vec![i64::MAX, 1], vec![i64::MIN, -1], vec![i64::MAX, 1, -1]] {
        let n = lengths.len();
        let source = SequencingToMinimizeWeightedCompletionTime::new(lengths, vec![0; n], vec![]);
        assert!(source.evaluate(&(0..n).collect()).is_err());
        assert!(matches!(
            <_ as ReduceTo<QUBO<i64>>>::reduce_to(&source),
            Err(ReductionError::IntegerOverflow { .. })
        ));
    }
}

#[test]
fn test_quadratic_assignment_rejects_overflow_hidden_by_cancellation() {
    use crate::rules::{ReduceTo, ReductionError};
    let a = 6_000_000_000_000_000_000;
    let source = QuadraticAssignment::new(
        vec![vec![0, a, a], vec![-a, 0, a], vec![-a, -a, 0]],
        vec![vec![1; 3]; 3],
    );
    // The mathematical sum is zero, but the first two directed costs overflow.
    assert!(source.evaluate(&vec![0, 1, 2]).is_err());
    assert!(matches!(
        <_ as ReduceTo<QUBO<i64>>>::reduce_to(&source),
        Err(ReductionError::IntegerOverflow { .. })
    ));
}

#[test]
fn test_weighted_completion_rejects_overflow_hidden_by_expansion() {
    use crate::rules::{ReduceTo, ReductionError};
    for (lengths, weights, scale, order) in [
        // Task 2 completes at time 10: its weighted cost overflows.
        (
            vec![-6, 2, 8],
            vec![5, -2, -7],
            i64::MAX / 60,
            vec![1, 2, 0],
        ),
        // Each product fits, but the task-index partial sum -129 * scale does not.
        (
            vec![9, 4, -6],
            vec![-9, -3, 3],
            i64::MAX / 120,
            vec![1, 0, 2],
        ),
    ] {
        let source = SequencingToMinimizeWeightedCompletionTime::new(
            lengths,
            weights.into_iter().map(|w| w * scale).collect(),
            vec![],
        );
        assert!(source.evaluate(&order).is_err());
        assert!(matches!(
            <_ as ReduceTo<QUBO<i64>>>::reduce_to(&source),
            Err(ReductionError::IntegerOverflow { .. })
        ));
    }
}
