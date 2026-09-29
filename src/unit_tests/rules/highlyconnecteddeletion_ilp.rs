use super::*;
use crate::models::algebraic::QUBO;
use crate::rules::test_helpers::assert_bf_vs_ilp;
use crate::rules::{ReductionGraph, ReductionPath, ReductionStep};
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Min;

fn triangle_with_leaf() -> HighlyConnectedDeletion<SimpleGraph> {
    HighlyConnectedDeletion::new(SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 2), (2, 3)]))
}

#[test]
fn polynomial_encoding_preserves_small_graph_optima_and_witnesses() {
    let entry = crate::rules::registry::reduction_entries()
        .into_iter()
        .find(|entry| {
            entry.source_name == HighlyConnectedDeletion::<SimpleGraph>::NAME
                && entry.target_name == ILP::<bool>::NAME
        })
        .unwrap();
    let contract = entry.parameter_contract().unwrap();
    let transform = contract.transform().unwrap();
    for n in 0..=4 {
        let pairs: Vec<_> = (0..n)
            .flat_map(|u| (u + 1..n).map(move |v| (u, v)))
            .collect();
        for graph_mask in 0..1_usize << pairs.len() {
            let edges = pairs
                .iter()
                .enumerate()
                .filter_map(|(i, &edge)| (graph_mask & (1 << i) != 0).then_some(edge))
                .collect();
            let source = HighlyConnectedDeletion::new(SimpleGraph::new(n, edges));
            let reduction = source.reduce_to().unwrap();
            let target = reduction.target_problem();
            let predicted = transform.evaluate(&source.parameters()).unwrap();
            for (field, actual) in target.parameters().iter() {
                assert!(predicted.get(field).expect("complete polynomial contract") >= actual);
            }
            let reference = BruteForce::new().solve(&source).unwrap().unwrap();
            let expected = source.evaluate(&reference).unwrap();
            let mut best_deleted = None;
            for mask in 0..1_usize << target.num_vars() {
                let solution: Vec<i64> = (0..target.num_vars())
                    .map(|i| ((mask >> i) & 1) as i64)
                    .collect();
                if target.is_feasible(&solution).unwrap() {
                    let recovered = reduction.extract_solution(&solution).unwrap();
                    let deleted = source
                        .evaluate(&recovered)
                        .unwrap()
                        .0
                        .expect("decoded witness must be feasible");
                    // For a loop-free graph the ILP objective counts every kept edge.
                    assert_eq!(
                        target.evaluate_objective(&solution).unwrap() + deleted,
                        source.num_edges() as i64
                    );
                    best_deleted =
                        Some(best_deleted.map_or(deleted, |best: i64| best.min(deleted)));
                }
            }
            assert_eq!(Min(best_deleted), expected, "n={n}, graph={graph_mask}");
        }
    }
}

#[test]
fn polynomial_encoding_handles_more_than_a_word_of_vertices() {
    let source = HighlyConnectedDeletion::new(SimpleGraph::new(64, vec![]));
    let reduction = source.reduce_to().unwrap();
    let target = reduction.target_problem();
    assert!(target.num_vars() <= 64 * 64);
    assert!(target.num_constraints() <= 64_usize.pow(3) + 2 * 64);
    let recovered = reduction
        .extract_solution(&vec![0; target.num_vars()])
        .unwrap();
    assert_eq!(source.evaluate(&recovered).unwrap(), Min(Some(0)));
}

#[test]
fn pair_encoding_decodes_clusters_and_rejects_invalid_assignments() {
    let source = triangle_with_leaf();
    let reduction = source.reduce_to().unwrap();
    // Pair variables: 01, 02, 03, 12, 13, 23; then four non-singleton flags.
    let valid = vec![1, 1, 0, 1, 0, 0, 1, 1, 1, 0];
    assert_eq!(
        reduction.extract_solution(&valid).unwrap(),
        vec![false, false, false, true]
    );
    for invalid in [
        vec![0; 9],
        vec![2; 10],
        vec![-1; 10],
        vec![1; 10], // The leaf prevents the whole graph from being highly connected.
        vec![1, 0, 0, 1, 0, 0, 1, 1, 1, 0], // Non-transitive membership.
    ] {
        assert!(reduction.extract_solution(&invalid).is_err());
    }
}

#[test]
fn polynomial_encoding_preserves_parallel_edge_costs_and_self_loops() {
    let source = HighlyConnectedDeletion::new(SimpleGraph::new(
        4,
        vec![
            (0, 0),
            (0, 1),
            (0, 1),
            (0, 2),
            (1, 2),
            (2, 3),
            (2, 3),
            (3, 3),
        ],
    ));
    let reduction = source.reduce_to().unwrap();
    let solution = ILPSolver::new().solve(reduction.target_problem()).unwrap();
    let recovered = reduction.extract_solution(&solution).unwrap();
    assert_eq!(
        recovered,
        vec![false, false, false, false, false, true, true, false]
    );
    assert_eq!(source.evaluate(&recovered).unwrap(), Min(Some(2)));
}

#[test]
fn test_highlyconnecteddeletion_to_ilp_closed_loop() {
    let source = HighlyConnectedDeletion::new(SimpleGraph::new(
        6,
        vec![(0, 1), (0, 2), (1, 2), (3, 4), (3, 5), (4, 5), (2, 3)],
    ));
    let reduction = source.reduce_to().unwrap();
    assert_bf_vs_ilp(&source, &reduction);
}

#[test]
fn polynomial_size_predictions_and_recovery_work_through_qubo() {
    let source = triangle_with_leaf();
    let graph = ReductionGraph::new();
    let path = ReductionPath {
        steps: [
            (
                HighlyConnectedDeletion::<SimpleGraph>::NAME,
                HighlyConnectedDeletion::<SimpleGraph>::variant(),
            ),
            (ILP::<bool>::NAME, ILP::<bool>::variant()),
            (QUBO::<i64>::NAME, QUBO::<i64>::variant()),
        ]
        .into_iter()
        .map(|(name, variant)| ReductionStep {
            name: name.into(),
            variant: ReductionGraph::variant_to_map(&variant),
        })
        .collect(),
    };
    let predicted = graph
        .compose_path_parameter_transform(&path)
        .unwrap()
        .unwrap()
        .evaluate(&source.parameters())
        .unwrap();
    let chain = graph.reduce_along_path(&path, &source).unwrap().unwrap();
    let qubo = chain.target_problem::<QUBO<i64>>();
    for (field, actual) in qubo.parameters().iter() {
        assert!(predicted.get(field).expect("composed polynomial bound") >= actual);
    }
    let solution = ILPSolver::new().solve(qubo).unwrap();
    let recovered: Vec<bool> = chain.extract_solution(&solution).unwrap();
    assert_eq!(source.evaluate(&recovered).unwrap(), Min(Some(1)));
}
