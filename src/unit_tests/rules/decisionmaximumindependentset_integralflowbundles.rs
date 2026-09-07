use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Or;

fn source(
    n: usize,
    edges: Vec<(usize, usize)>,
    bound: i64,
) -> Decision<MaximumIndependentSet<SimpleGraph, One>> {
    Decision::new(
        MaximumIndependentSet::new(SimpleGraph::new(n, edges), vec![One; n]),
        bound,
    )
}

#[test]
fn test_decisionmaximumindependentset_to_integralflowbundles_closed_loop() {
    for bound in [2, 3] {
        let source = source(3, vec![(0, 1), (1, 2)], bound);
        let reduction = ReduceTo::<IntegralFlowBundles>::reduce_to(&source).unwrap();
        let witness = BruteForce::new().solve(reduction.target_problem()).unwrap();
        assert_eq!(witness.is_some(), bound == 2);
        if let Some(witness) = witness {
            assert_eq!(
                source.evaluate(&reduction.extract_solution(&witness).unwrap()),
                Ok(Or(true))
            );
        }
    }
}

#[test]
fn test_decision_ifb_all_small_graphs_thresholds_and_binary_flows() {
    // Conservation and the path bundles force every feasible arc flow to be
    // binary. Enumerate all binary vectors, including those violating conservation.
    for n in 0..=3 {
        let pairs: Vec<_> = (0..n).flat_map(|u| (u..n).map(move |v| (u, v))).collect();
        for mask in 0usize..1 << pairs.len() {
            let edges: Vec<_> = pairs
                .iter()
                .enumerate()
                .filter_map(|(i, &e)| (mask >> i & 1 == 1).then_some(e))
                .collect();
            for bound in -1..=n as i64 + 1 {
                let source = source(n, edges.clone(), bound);
                let reduction = ReduceTo::<IntegralFlowBundles>::reduce_to(&source).unwrap();
                let target = reduction.target_problem();
                assert_eq!(
                    (
                        target.num_vertices(),
                        target.num_arcs(),
                        target.num_bundles()
                    ),
                    (n + 3, 2 * n + 2, edges.len() + n + 1)
                );
                let mut source_exists = false;
                for mask in 0usize..1 << n {
                    let selected: Vec<_> = (0..n).map(|i| mask >> i & 1 == 1).collect();
                    if source.evaluate(&selected).unwrap().0 {
                        source_exists = true;
                        let mut flow: Vec<_> =
                            selected.iter().flat_map(|&x| [usize::from(x); 2]).collect();
                        flow.extend([1, 1]);
                        assert_eq!(target.evaluate(&flow), Ok(Or(true)));
                    }
                }
                let mut target_exists = false;
                for mask in 0usize..1 << target.num_arcs() {
                    let flow = (0..target.num_arcs()).map(|i| mask >> i & 1).collect();
                    if target.evaluate(&flow).unwrap().0 {
                        target_exists = true;
                        assert_eq!(
                            source.evaluate(&reduction.extract_solution(&flow).unwrap()),
                            Ok(Or(true))
                        );
                    }
                }
                assert_eq!(
                    source_exists, target_exists,
                    "n={n}, edges={edges:?}, bound={bound}"
                );
            }
        }
    }
}

#[test]
fn test_decision_ifb_loops_parallel_edges_and_invalid_witnesses() {
    let source = source(3, vec![(0, 0), (0, 1), (0, 1)], 2);
    let reduction = ReduceTo::<IntegralFlowBundles>::reduce_to(&source).unwrap();
    let valid = vec![0, 0, 1, 1, 1, 1, 1, 1];
    assert_eq!(
        reduction.extract_solution(&valid).unwrap(),
        vec![false, true, true]
    );
    for flow in [
        vec![],
        vec![usize::MAX; 8],
        vec![0; 8],
        vec![0, 0, 1, 0, 1, 1, 1, 1], // conservation
        vec![1, 1, 0, 0, 1, 1, 1, 1], // self-loop
        vec![0, 0, 2, 2, 1, 1, 1, 1], // path capacity
    ] {
        assert!(reduction.extract_solution(&flow).is_err());
    }
}

#[test]
fn test_decision_ifb_dimensions_and_threshold_boundaries() {
    assert_eq!(flow_dimensions(0, 0), Ok((3, 2, 1)));
    let n = usize::MAX / 2 - 1;
    assert_eq!(
        flow_dimensions(n, usize::MAX - n - 1),
        Ok((n + 3, usize::MAX - 1, usize::MAX))
    );
    for (n, m) in [
        (usize::MAX, 0),
        (usize::MAX - 2, 0),
        (usize::MAX / 2, 0),
        (1, usize::MAX),
        (1, usize::MAX - 1),
    ] {
        assert!(matches!(
            flow_dimensions(n, m),
            Err(crate::rules::ReductionError::IntegerOverflow { .. })
        ));
    }
    for (n, bound, expected) in [
        (0, i64::MIN, 1),
        (0, 0, 1),
        (0, 1, 2),
        (0, i64::MAX, 2),
        (3, 2, 3),
        (3, 3, 4),
        (3, 4, 5),
        (3, i64::MAX, 5),
    ] {
        assert_eq!(flow_requirement(n, bound), Ok(expected));
    }
}

#[test]
#[cfg(target_pointer_width = "64")]
fn test_decision_ifb_i64_count_boundaries() {
    assert_eq!(
        flow_requirement((i64::MAX - 2) as usize, i64::MAX),
        Ok(i64::MAX)
    );
    for n in [(i64::MAX - 1) as usize, usize::MAX] {
        assert!(matches!(
            flow_requirement(n, 0),
            Err(crate::rules::ReductionError::IntegerOverflow { .. })
        ));
    }
}

#[test]
fn test_decision_ifb_registration_replaces_optimization_edge() {
    let entries: Vec<_> = crate::rules::registry::reduction_entries()
        .into_iter()
        .filter(|e| e.target_name == "IntegralFlowBundles")
        .collect();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].source_name, "DecisionMaximumIndependentSet");
    assert_eq!(
        (entries[0].source_variant_fn)(),
        vec![("graph", "SimpleGraph"), ("weight", "One")]
    );
    // The dynamic executor must reject the old weighted source type, even though
    // it shares the same decision problem name with the registered unit variant.
    let weighted = Decision::new(
        MaximumIndependentSet::new(SimpleGraph::new(0, vec![]), Vec::<i64>::new()),
        0,
    );
    assert!(matches!(
        (entries[0].reduce_fn.unwrap())(&weighted),
        Err(crate::rules::ReductionError::SourceTypeMismatch { .. })
    ));
}
