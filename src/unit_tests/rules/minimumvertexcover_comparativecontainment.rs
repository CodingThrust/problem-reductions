use super::*;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_minimumvertexcover_to_comparativecontainment_closed_loop() {
    for bound in [1, 2] {
        let source = Decision::new(
            MinimumVertexCover::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![4i64, 2, -1]),
            bound,
        );
        let reduction = ReduceTo::<ComparativeContainment<i64>>::reduce_to(&source).unwrap();
        assert_satisfaction_round_trip_from_satisfaction_target(
            &source,
            &reduction,
            "signed vertex cover",
        );
        let witness = BruteForce::new()
            .solve(reduction.target_problem())
            .unwrap()
            .unwrap();
        assert!(
            source
                .evaluate(&reduction.extract_solution(&witness).unwrap())
                .unwrap()
                .0
        );
    }
    let source = Decision::new(
        MinimumVertexCover::new(
            SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
            vec![4i64, 2, -1],
        ),
        0,
    );
    let reduction = ReduceTo::<ComparativeContainment<i64>>::reduce_to(&source).unwrap();
    assert!(BruteForce::new()
        .solve(reduction.target_problem())
        .unwrap()
        .is_none());
}

#[test]
fn test_signed_containment_all_small_graphs_and_witnesses() {
    // Exhaust all graphs through 3 vertices, including loops, and all choices
    // of negative/zero/positive weights. Check every witness in both directions.
    for n in 0usize..=3 {
        let possible: Vec<_> = (0..n).flat_map(|u| (u..n).map(move |v| (u, v))).collect();
        for edge_mask in 0..(1usize << possible.len()) {
            let edges: Vec<_> = possible
                .iter()
                .enumerate()
                .filter_map(|(i, &e)| (edge_mask & (1 << i) != 0).then_some(e))
                .collect();
            for mut code in 0..3usize.pow(u32::try_from(n).unwrap()) {
                let weights: Vec<i64> = (0..n)
                    .map(|_| {
                        let w = [-2, 0, 3][code % 3];
                        code /= 3;
                        w
                    })
                    .collect();
                for bound in [-7, -2, 0, 1, 3, 9, i64::MAX] {
                    let source = Decision::new(
                        MinimumVertexCover::new(
                            SimpleGraph::new(n, edges.clone()),
                            weights.clone(),
                        ),
                        bound,
                    );
                    let reduction =
                        ReduceTo::<ComparativeContainment<i64>>::reduce_to(&source).unwrap();
                    let target = reduction.target_problem();
                    assert_eq!(target.universe_size(), n);
                    assert!(target.num_r_sets() <= n + 1);
                    assert!(target.num_s_sets() <= n + edges.len() + 1);
                    assert!(target
                        .r_weights()
                        .iter()
                        .chain(target.s_weights())
                        .all(|&w| w > 0));
                    for mask in 0..(1usize << n) {
                        let witness: Vec<_> = (0..n).map(|v| mask & (1 << v) != 0).collect();
                        let valid = source.evaluate(&witness).unwrap().0;
                        assert_eq!(target.evaluate(&witness).unwrap().0, valid);
                        if valid {
                            assert_eq!(reduction.extract_solution(&witness).unwrap(), witness);
                        } else {
                            assert!(reduction.extract_solution(&witness).is_err());
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn test_signed_containment_duplicate_edges_and_invalid_length() {
    let source = Decision::new(
        MinimumVertexCover::new(
            SimpleGraph::new(3, vec![(0, 0), (0, 1), (0, 1)]),
            vec![-3i64, 0, 5],
        ),
        -3,
    );
    let reduction = ReduceTo::<ComparativeContainment<i64>>::reduce_to(&source).unwrap();
    let witness = vec![true, false, false];
    assert_eq!(reduction.extract_solution(&witness).unwrap(), witness);
    for bad in [vec![], vec![true; 4]] {
        assert!(reduction.extract_solution(&bad).is_err());
    }
}

#[test]
fn test_signed_containment_numeric_domain() {
    // Distinct causes: source positive/negative totals, budget subtraction,
    // strict penalty, coefficient sign change, and total target-family sum.
    for (weights, bound, edges) in [
        (vec![i64::MAX, 1], 0, vec![]),
        (vec![i64::MIN, -1], 0, vec![]),
        (vec![1], i64::MIN, vec![]),
        (vec![i64::MAX], 0, vec![]),
        (vec![i64::MIN], 0, vec![]),
        (vec![0], i64::MIN, vec![]),
        (vec![i64::MAX / 2], 0, vec![(0, 0), (0, 0)]),
    ] {
        let source = Decision::new(
            MinimumVertexCover::new(SimpleGraph::new(weights.len(), edges), weights),
            bound,
        );
        assert!(matches!(
            ReduceTo::<ComparativeContainment<i64>>::reduce_to(&source),
            Err(crate::rules::ReductionError::IntegerOverflow { .. })
        ));
    }
    // Large values are accepted when all target arithmetic is representable.
    for (weight, bound) in [
        (i64::MAX - 1, i64::MAX),
        (-(i64::MAX - 1), 0),
        (0, i64::MIN + 1),
    ] {
        let source = Decision::new(
            MinimumVertexCover::new(SimpleGraph::new(1, vec![]), vec![weight]),
            bound,
        );
        let reduction = ReduceTo::<ComparativeContainment<i64>>::reduce_to(&source).unwrap();
        for witness in [vec![false], vec![true]] {
            assert_eq!(
                source.evaluate(&witness).unwrap().0,
                reduction.target_problem().evaluate(&witness).unwrap().0
            );
        }
    }
}
