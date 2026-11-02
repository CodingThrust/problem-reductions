use super::*;
use crate::rules::test_helpers::{
    assert_parameter_predictions, assert_satisfaction_round_trip_from_satisfaction_target,
};
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Or;

fn cycle(k: usize) -> KColoring<KN, SimpleGraph> {
    KColoring::with_k(
        SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)]),
        k,
    )
}

#[test]
fn test_kcoloring_to_satisfiability_closed_loop() {
    let source = cycle(3);
    let result = ReduceTo::<Satisfiability>::reduce_to(&source).unwrap();
    assert_satisfaction_round_trip_from_satisfaction_target(&source, &result, "three-color cycle");
    let solutions = BruteForce::new()
        .find_all_witnesses(result.target_problem())
        .unwrap();
    assert_eq!(solutions.len(), 30);
    let expected = vec![0, 1, 0, 1, 2];
    let bits = vec![
        true, false, false, false, true, false, true, false, false, false, true, false, false,
        false, true,
    ];
    assert!(result.target_problem().evaluate(&bits).unwrap().0);
    assert_eq!(result.extract_solution(&bits).unwrap(), expected);
    assert_eq!(
        (
            result.target.num_vars(),
            result.target.num_clauses(),
            result.target.num_literals()
        ),
        (15, 35, 75)
    );
    assert_parameter_predictions(&source, &result);
    assert_eq!(
        crate::rules::AggregateReductionResult::extract_value(&result, Or(true)),
        Or(true)
    );
    assert_eq!(
        crate::rules::AggregateReductionResult::extract_value(&result, Or(false)),
        Or(false)
    );
}

#[test]
fn odd_cycle_and_complete_graph_are_infeasible() {
    for source in [
        cycle(2),
        KColoring::with_k(
            SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]),
            3,
        ),
    ] {
        let result = ReduceTo::<Satisfiability>::reduce_to(&source).unwrap();
        assert!(BruteForce::new().solve(&source).unwrap().is_none());
        assert!(
            BruteForce::new()
                .solve(result.target_problem())
                .unwrap()
                .is_none()
        );
        assert_parameter_predictions(&source, &result);
    }
    let result = ReduceTo::<Satisfiability>::reduce_to(&cycle(2)).unwrap();
    assert_eq!(
        (
            result.target.num_vars(),
            result.target.num_clauses(),
            result.target.num_literals()
        ),
        (10, 20, 40)
    );
}

#[test]
fn empty_isolated_disconnected_and_excess_color_domains() {
    let cases = [
        (0, vec![], 0, 1),
        (0, vec![], usize::MAX, 1),
        (1, vec![], 0, 0),
        (3, vec![], 0, 0),
        (3, vec![], 1, 1),
        (2, vec![], 3, 9),
        (2, vec![(0, 1)], 4, 12),
        (4, vec![(0, 1), (2, 3)], 2, 4),
        (3, vec![(0, 1)], 2, 4),
        (3, vec![(0, 1), (1, 2), (2, 0)], 3, 6),
    ];
    for (n, edges, k, count) in cases {
        let source = KColoring::<KN, _>::with_k(SimpleGraph::new(n, edges), k);
        let result = ReduceTo::<Satisfiability>::reduce_to(&source).unwrap();
        let expected = BruteForce::new().find_all_witnesses(&source).unwrap();
        let actual = BruteForce::new()
            .find_all_witnesses(result.target_problem())
            .unwrap();
        assert_eq!(expected.len(), count);
        assert_eq!(actual.len(), count);
        for bits in actual {
            let colors = result.extract_solution(&bits).unwrap();
            assert!(expected.contains(&colors));
        }
        assert_parameter_predictions(&source, &result);
    }
}

#[test]
fn clause_order_and_stored_edge_orientation() {
    let source = KColoring::<KN, _>::with_k(SimpleGraph::new(3, vec![(2, 0), (1, 2)]), 2);
    let result = ReduceTo::<Satisfiability>::reduce_to(&source).unwrap();
    let actual: Vec<_> = result
        .target
        .clauses()
        .iter()
        .map(|clause| clause.literals.clone())
        .collect();
    assert_eq!(
        actual,
        vec![
            vec![1, 2],
            vec![3, 4],
            vec![5, 6],
            vec![-1, -2],
            vec![-3, -4],
            vec![-5, -6],
            vec![-5, -1],
            vec![-6, -2],
            vec![-3, -5],
            vec![-4, -6]
        ]
    );
}

#[test]
fn extraction_rejects_zero_color_blocks() {
    let result = ReduceTo::<Satisfiability>::reduce_to(&cycle(3)).unwrap();
    assert!(result.extract_solution(&vec![false; 15]).is_err());
}

#[test]
fn extraction_rejects_multiple_color_blocks() {
    let source = KColoring::<KN, _>::with_k(SimpleGraph::empty(1), 3);
    let result = ReduceTo::<Satisfiability>::reduce_to(&source).unwrap();
    assert!(result.extract_solution(&vec![true, true, false]).is_err());
}

#[test]
fn extraction_rejects_improper_one_hot_assignment() {
    let source = KColoring::<KN, _>::with_k(SimpleGraph::new(2, vec![(0, 1)]), 2);
    let result = ReduceTo::<Satisfiability>::reduce_to(&source).unwrap();
    assert!(
        result
            .extract_solution(&vec![true, false, true, false])
            .is_err()
    );
}

#[test]
fn extraction_rejects_short_and_long_assignments() {
    let result = ReduceTo::<Satisfiability>::reduce_to(&cycle(3)).unwrap();
    for len in [0, 14, 16] {
        assert!(matches!(
            result.extract_solution(&vec![true; len]),
            Err(crate::rules::ExtractionError::Evaluation(_))
        ));
    }
}

#[test]
fn source_rejects_loops_and_parallel_edges() {
    for edges in [vec![(0, 0)], vec![(0, 1), (0, 1)], vec![(0, 1), (1, 0)]] {
        assert!(SimpleGraph::try_new(2, edges.clone()).is_err());
        let data = serde_json::json!({"num_vertices": 2, "edges": edges});
        assert!(serde_json::from_value::<SimpleGraph>(data).is_err());
    }
}

#[test]
#[cfg(target_pointer_width = "64")]
fn checked_sizes_accept_limits_and_reject_overflow() {
    let max_literal = usize::try_from(i64::MAX).unwrap();
    assert_eq!(coloring_sat_size(0, 0, usize::MAX).unwrap(), (0, 0, 0));
    assert_eq!(coloring_sat_size(5, 5, 0).unwrap(), (0, 5, 0));
    assert_eq!(coloring_sat_size(5, 5, 3).unwrap(), (15, 35, 75));
    assert_eq!(
        coloring_sat_size(max_literal, 0, 1).unwrap(),
        (max_literal, max_literal, max_literal)
    );
    // With one isolated vertex the literal count is k^2; 2^32 is its first overflowing k.
    let k = 4_294_967_295;
    assert_eq!(
        coloring_sat_size(1, 0, k).unwrap(),
        (k, 9_223_372_030_412_324_866, 18_446_744_065_119_617_025)
    );
    // At k=1, total literals are n+2m. These edge counts fit the simple-graph bound.
    let n = 1usize << 33;
    let m = (usize::MAX - n) / 2;
    assert_eq!(
        coloring_sat_size(n, m, 1).unwrap(),
        (n, n + m, usize::MAX - 1)
    );
    for (n, m, k) in [
        (usize::MAX, 0, 2),
        (1, 0, max_literal + 1),
        (1, 0, 6_074_001_001),
        (3, 0, 4_000_000_000),
        (n, usize::MAX / 3 + 1, 3),
        (n, usize::MAX / 3, 3),
        (n, usize::MAX - n + 1, 1),
        (1, 0, 4_294_967_297),
        (1, 0, 4_294_967_296),
        (n, m + 1, 1),
    ] {
        assert!(
            matches!(
                coloring_sat_size(n, m, k),
                Err(crate::rules::ReductionError::IntegerOverflow { .. })
            ),
            "n={n},m={m},k={k}"
        );
    }
    let source = KColoring::<KN, _>::with_k(SimpleGraph::empty(1), usize::MAX);
    assert!(matches!(
        ReduceTo::<Satisfiability>::reduce_to(&source),
        Err(crate::rules::ReductionError::IntegerOverflow { .. })
    ));
    let source = KColoring::<KN, _>::with_k(SimpleGraph::empty(1), k);
    assert!(matches!(
        ReduceTo::<Satisfiability>::reduce_to(&source),
        Err(crate::rules::ReductionError::InvalidTarget { .. })
    ));
}

#[test]
#[cfg(target_pointer_width = "64")]
fn signed_literal_indices_are_checked_at_each_boundary() {
    let max = usize::try_from(i64::MAX).unwrap();
    assert_eq!(color_literal(0, 3, 0).unwrap(), 1);
    assert_eq!(color_literal(4, 3, 2).unwrap(), 15);
    assert_eq!(color_literal(max - 1, 1, 0).unwrap(), i64::MAX);
    for (v, k, c) in [
        (max, 1, 0),
        (usize::MAX, 2, 0),
        (usize::MAX, 1, 1),
        (usize::MAX, 1, 0),
    ] {
        assert!(matches!(
            color_literal(v, k, c),
            Err(crate::rules::ReductionError::IntegerOverflow { .. })
        ));
    }
}
