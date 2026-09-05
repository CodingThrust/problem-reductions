use crate::models::decision::Decision;
use crate::models::graph::{MinimumDominatingSet, MinimumSumMulticenter};
use crate::rules::{ReduceTo, ReductionResult};
use crate::solvers::BruteForce;
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::{Min, One, Or};

fn decision_mds(
    num_vertices: usize,
    edges: &[(usize, usize)],
    k: i64,
) -> Decision<MinimumDominatingSet<SimpleGraph, One>> {
    Decision::new(
        MinimumDominatingSet::new(
            SimpleGraph::new(num_vertices, edges.to_vec()),
            vec![One; num_vertices],
        ),
        k,
    )
}

#[test]
fn test_decisionminimumdominatingset_to_minimumsummulticenter_structure() {
    let source = decision_mds(
        6,
        &[(0, 1), (0, 2), (1, 3), (2, 3), (3, 4), (3, 5), (4, 5)],
        2,
    );
    let reduction = ReduceTo::<MinimumSumMulticenter<SimpleGraph, i64>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();
    assert_eq!(
        crate::rules::AggregateReductionResult::target_problem(&reduction).k(),
        target.k()
    );

    assert_eq!(
        target.graph().num_vertices(),
        source.inner().graph().num_vertices() + 1
    );
    assert_eq!(target.graph().edges(), source.inner().graph().edges());
    assert_eq!(target.vertex_weights(), vec![1i64; 7].as_slice());
    assert_eq!(target.edge_lengths(), vec![1i64; 7].as_slice());
    assert_eq!(target.k(), 3);
}

#[test]
fn test_decisionminimumdominatingset_to_minimumsummulticenter_closed_loop_yes_instance() {
    let source = decision_mds(
        6,
        &[(0, 1), (0, 2), (1, 3), (2, 3), (3, 4), (3, 5), (4, 5)],
        2,
    );
    let reduction = ReduceTo::<MinimumSumMulticenter<SimpleGraph, i64>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();

    let target_solutions = BruteForce::new().find_all_witnesses(target).unwrap();
    assert!(
        !target_solutions.is_empty(),
        "target should have optimal K-center placements"
    );

    for target_solution in target_solutions {
        assert_eq!(target.evaluate(&target_solution).unwrap().unwrap(), 4);
        let extracted = reduction.extract_solution(&target_solution).unwrap();
        assert_eq!(extracted, target_solution[..6]);
        assert_eq!(source.evaluate(&extracted).unwrap(), Or(true));
    }
}

#[test]
fn test_decisionminimumdominatingset_to_minimumsummulticenter_closed_loop_no_instance() {
    let source = decision_mds(
        6,
        &[(0, 1), (0, 2), (1, 3), (2, 3), (3, 4), (3, 5), (4, 5)],
        1,
    );
    let reduction = ReduceTo::<MinimumSumMulticenter<SimpleGraph, i64>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();

    let target_solutions = BruteForce::new().find_all_witnesses(target).unwrap();
    assert!(
        !target_solutions.is_empty(),
        "target should still have optimal K-center placements"
    );

    let threshold = i64::try_from(source.inner().graph().num_vertices()).unwrap()
        - i64::try_from(source.k()).unwrap();
    for target_solution in target_solutions {
        let target_value = target.evaluate(&target_solution).unwrap().unwrap();
        assert_eq!(target_value, 6);
        assert!(target_value > threshold);

        assert_eq!(
            crate::rules::AggregateReductionResult::extract_value(
                &reduction,
                Min(Some(target_value))
            ),
            Or(false)
        );
        assert!(reduction.extract_solution(&target_solution).is_err());
    }
}

// Enumerate every graph on at most four vertices and EVERY target placement,
// including infeasible/nonoptimal ones. Check both directions independently of
// the ILP pipeline and verify that extraction never accepts a false witness.
#[test]
fn test_decisionminimumdominatingset_to_minimumsummulticenter_all_small_graphs() {
    for n in 0..=4 {
        let possible_edges: Vec<_> = (0..n)
            .flat_map(|u| (u + 1..n).map(move |v| (u, v)))
            .collect();
        for graph_mask in 0usize..1 << possible_edges.len() {
            let edges: Vec<_> = possible_edges
                .iter()
                .enumerate()
                .filter(|(i, _)| graph_mask & (1 << i) != 0)
                .map(|(_, &edge)| edge)
                .collect();
            let bounds = [i64::MIN, -1, i64::MAX]
                .into_iter()
                .chain(0..=i64::try_from(n).unwrap() + 1);
            for bound in bounds {
                let source = decision_mds(n, &edges, bound);
                let reduction =
                    ReduceTo::<MinimumSumMulticenter<SimpleGraph, i64>>::reduce_to(&source)
                        .unwrap();
                let target = reduction.target_problem();
                assert!(target.num_vertices() <= n + 2);
                assert_eq!(target.num_edges(), edges.len());
                let source_yes = BruteForce::new().solve(&source).unwrap().is_some();
                let mut optimum = None;
                for mask in 0usize..1 << target.num_vertices() {
                    let placement: Vec<_> = (0..target.num_vertices())
                        .map(|i| mask & (1 << i) != 0)
                        .collect();
                    let value = target.evaluate(&placement).unwrap();
                    if let Some(cost) = value.0 {
                        optimum = Some(optimum.map_or(cost, |previous: i64| previous.min(cost)));
                    }
                    let accepted =
                        crate::rules::AggregateReductionResult::extract_value(&reduction, value).0;
                    match reduction.extract_solution(&placement) {
                        Ok(witness) => {
                            assert!(accepted);
                            assert_eq!(source.evaluate(&witness).unwrap(), Or(true));
                        }
                        Err(_) => assert!(!accepted),
                    }
                }
                assert_eq!(
                    crate::rules::AggregateReductionResult::extract_value(&reduction, Min(optimum)),
                    Or(source_yes),
                    "n={n}, edges={edges:?}, K={bound}"
                );
                assert!(reduction
                    .extract_solution(&vec![false; target.num_vertices() + 1])
                    .is_err());
            }
        }
    }
}

#[test]
fn test_multicenter_parameter_numeric_boundaries() {
    use super::multicenter_parameters;
    use crate::rules::ReductionError;

    // Graph allocation is irrelevant to arithmetic domain validation.
    for bound in [-1, 0, i64::MAX] {
        assert!(matches!(
            multicenter_parameters(usize::MAX, bound),
            Err(ReductionError::IntegerOverflow { .. })
        ));
    }
    assert!(multicenter_parameters(usize::MAX - 1, -1).is_err());
    if let Ok(n) = usize::try_from(i64::MAX) {
        assert_eq!(multicenter_parameters(n, 0).unwrap(), (n + 1, 1, i64::MAX));
        assert_eq!(
            multicenter_parameters(n, i64::MAX).unwrap(),
            (n + 1, n + 1, 0)
        );
        assert!(matches!(
            multicenter_parameters(n + 1, 0),
            Err(ReductionError::IntegerOverflow { .. })
        ));
    }
}
