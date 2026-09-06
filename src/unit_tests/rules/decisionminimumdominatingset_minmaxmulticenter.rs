use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

fn decision_mds(
    n: usize,
    edges: &[(usize, usize)],
    bound: i64,
) -> Decision<MinimumDominatingSet<SimpleGraph, One>> {
    Decision::new(
        MinimumDominatingSet::new(SimpleGraph::new(n, edges.to_vec()), vec![One; n]),
        bound,
    )
}

#[test]
fn test_decisionminimumdominatingset_to_minmaxmulticenter_closed_loop() {
    let source = decision_mds(3, &[(0, 1), (1, 2)], 1);
    let reduction = ReduceTo::<MinMaxMulticenter<SimpleGraph, One>>::reduce_to(&source).unwrap();
    let target = reduction.target_problem();
    assert_eq!(target.num_vertices(), 5);
    assert_eq!(target.num_edges(), 2);
    assert_eq!(target.k(), 3);
    for witness in BruteForce::new().find_all_witnesses(target).unwrap() {
        assert!(witness[3] && witness[4]);
        assert!(
            source
                .evaluate(&reduction.extract_solution(&witness).unwrap())
                .unwrap()
                .0
        );
    }
    let source = decision_mds(4, &[(0, 1), (1, 2), (2, 3)], 1);
    let reduction = ReduceTo::<MinMaxMulticenter<SimpleGraph, One>>::reduce_to(&source).unwrap();
    let witness = BruteForce::new()
        .solve(reduction.target_problem())
        .unwrap()
        .unwrap();
    let optimum = reduction.target_problem().evaluate(&witness).unwrap();
    assert_eq!(optimum, Min(Some(2)));
    assert_eq!(
        crate::rules::AggregateReductionResult::extract_value(&reduction, optimum),
        Or(false)
    );
    assert!(reduction.extract_solution(&witness).is_err());
}

#[test]
fn test_multicenter_all_small_graphs_bounds_and_placements() {
    for n in 0usize..=3 {
        let possible: Vec<_> = (0..n).flat_map(|u| (u..n).map(move |v| (u, v))).collect();
        for edge_mask in 0..(1usize << possible.len()) {
            let edges: Vec<_> = possible
                .iter()
                .enumerate()
                .filter_map(|(i, &e)| (edge_mask & (1 << i) != 0).then_some(e))
                .collect();
            let n_i64 = i64::try_from(n).unwrap();
            for bound in [i64::MIN, -1, 0, 1, n_i64, n_i64 + 1, i64::MAX] {
                let source = decision_mds(n, &edges, bound);
                let reduction =
                    ReduceTo::<MinMaxMulticenter<SimpleGraph, One>>::reduce_to(&source).unwrap();
                let target = reduction.target_problem();
                assert_eq!(target.graph().edges(), edges);
                assert_eq!(target.num_vertices(), n + 2);
                assert_eq!(target.vertex_weights(), vec![One; n + 2]);
                assert_eq!(target.edge_lengths(), vec![One; edges.len()]);
                assert!((1..=n + 2).contains(&target.k()));
                let mut source_yes = false;
                for mask in 0..(1usize << n) {
                    let mut witness: Vec<_> = (0..n).map(|v| mask & (1 << v) != 0).collect();
                    if source.evaluate(&witness).unwrap().0 {
                        source_yes = true;
                        let mut count = witness.iter().filter(|&&b| b).count();
                        for bit in &mut witness {
                            if !*bit && count < target.k() - 2 {
                                *bit = true;
                                count += 1;
                            }
                        }
                        witness.extend([true, true]);
                        assert!(target.evaluate(&witness).unwrap().0.is_some_and(|r| r <= 1));
                    }
                }
                let mut optimum: Option<i64> = None;
                for mask in 0..(1usize << (n + 2)) {
                    let witness: Vec<_> = (0..n + 2).map(|v| mask & (1 << v) != 0).collect();
                    let radius = target.evaluate(&witness).unwrap().0;
                    if let Some(r) = radius {
                        optimum = Some(optimum.map_or(r, |old| old.min(r)));
                    }
                    if radius.is_some_and(|r| r <= 1) {
                        assert!(
                            source
                                .evaluate(&reduction.extract_solution(&witness).unwrap())
                                .unwrap()
                                .0
                        );
                    } else {
                        assert!(reduction.extract_solution(&witness).is_err());
                    }
                }
                assert_eq!(
                    crate::rules::AggregateReductionResult::extract_value(&reduction, Min(optimum)),
                    Or(source_yes)
                );
            }
        }
    }
}

#[test]
fn test_multicenter_duplicate_edges_and_malformed_witness() {
    let source = decision_mds(3, &[(0, 0), (0, 1), (0, 1)], 2);
    let reduction = ReduceTo::<MinMaxMulticenter<SimpleGraph, One>>::reduce_to(&source).unwrap();
    let witness = vec![true, false, true, true, true];
    assert_eq!(
        reduction.extract_solution(&witness).unwrap(),
        vec![true, false, true]
    );
    for bad in [vec![], vec![true; 6]] {
        assert!(reduction.extract_solution(&bad).is_err());
    }
    assert_eq!(
        crate::rules::AggregateReductionResult::extract_value(&reduction, Min(None)),
        Or(false)
    );
}

#[test]
fn test_multicenter_parameter_boundaries() {
    assert_eq!(multicenter_parameters(0, i64::MIN).unwrap(), (2, 1));
    assert_eq!(multicenter_parameters(0, i64::MAX).unwrap(), (2, 2));
    assert_eq!(multicenter_parameters(3, 0).unwrap(), (5, 2));
    assert_eq!(multicenter_parameters(3, i64::MAX).unwrap(), (5, 5));
    for n in [usize::MAX, usize::MAX - 1] {
        assert!(matches!(
            multicenter_parameters(n, 0),
            Err(crate::rules::ReductionError::IntegerOverflow { .. })
        ));
    }
    #[cfg(target_pointer_width = "64")]
    {
        let max = usize::try_from(i64::MAX).unwrap();
        assert!(multicenter_parameters(max + 1, 0).is_err());
        assert!(multicenter_parameters(max, i64::MAX).is_err());
        assert!(multicenter_parameters(max - 1, i64::MAX).is_err());
        assert_eq!(
            multicenter_parameters(max - 2, i64::MAX).unwrap(),
            (max, max)
        );
        assert_eq!(multicenter_parameters(max, i64::MIN).unwrap(), (max + 2, 1));
    }
}
