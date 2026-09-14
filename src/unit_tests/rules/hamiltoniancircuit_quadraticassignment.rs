use crate::models::algebraic::QuadraticAssignment;
use crate::models::decision::Decision;
use crate::models::graph::HamiltonianCircuit;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::rules::ReduceTo;
use crate::rules::ReductionResult;
use crate::solvers::BruteForce;
use crate::solvers::SolveOutcome;
use crate::topology::{Graph, SimpleGraph};
use crate::types::OptimizationValue;
use crate::Problem;

fn cycle4_hc() -> HamiltonianCircuit<SimpleGraph> {
    HamiltonianCircuit::new(SimpleGraph::cycle(4))
}

#[test]
fn test_hamiltoniancircuit_to_quadraticassignment_closed_loop() {
    let source = cycle4_hc();
    let reduction = ReduceTo::<Decision<QuadraticAssignment>>::reduce_to(&source)
        .expect("reduction should succeed");

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "HamiltonianCircuit -> QuadraticAssignment",
    );
}

#[test]
fn test_hamiltoniancircuit_to_quadraticassignment_structure() {
    let source = cycle4_hc();
    let reduction = ReduceTo::<Decision<QuadraticAssignment>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();

    assert_eq!(target.inner().num_facilities(), 4);
    assert_eq!(target.inner().num_locations(), 4);

    // Cost matrix: cycle adjacency on positions
    let cost = target.inner().cost_matrix();
    for (i, cost_row) in cost.iter().enumerate() {
        for (j, &cost_val) in cost_row.iter().enumerate() {
            let expected = if j == (i + 1) % 4 { 1 } else { 0 };
            assert_eq!(cost_val, expected, "cost[{i}][{j}] should be {expected}");
        }
    }

    // Distance matrix: edges and diagonal cost zero, non-edges cost one.
    let dist = target.inner().distance_matrix();
    for (k, dist_row) in dist.iter().enumerate() {
        for (l, &dist_val) in dist_row.iter().enumerate() {
            let expected = i64::from(k != l && !source.graph().has_edge(k, l));
            assert_eq!(dist_val, expected, "dist[{k}][{l}] should be {expected}");
        }
    }
}

#[test]
fn test_hamiltoniancircuit_to_quadraticassignment_optimal_cost_is_zero() {
    let source = cycle4_hc();
    let reduction = ReduceTo::<Decision<QuadraticAssignment>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();

    // The identity permutation [0,1,2,3] is a valid HC on a 4-cycle,
    // so the QAP optimum should be zero.
    let best = BruteForce::new()
        .solve(target)
        .unwrap()
        .expect("QAP should have an optimal solution");
    let value = target.inner().evaluate(&best).unwrap();
    assert_eq!(
        value,
        crate::types::Min(Some(0)),
        "optimal QAP cost should be zero"
    );
}

#[test]
fn test_hamiltoniancircuit_to_quadraticassignment_nonhamiltonian_cost_gap() {
    // Star graph on 4 vertices has no Hamiltonian circuit
    let source = HamiltonianCircuit::new(SimpleGraph::star(4));
    let reduction = ReduceTo::<Decision<QuadraticAssignment>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();
    assert!(BruteForce::new().solve(target).unwrap().is_none());

    let best = BruteForce::new()
        .solve(target.inner())
        .unwrap()
        .expect("QAP always has a solution");
    let value = target.inner().evaluate(&best).unwrap();
    assert!(
        value.is_valid(),
        "QAP solution should have a valid objective"
    );
    assert!(
        value.unwrap() > 0,
        "expected positive QAP cost for non-Hamiltonian graph, got {:?}",
        value
    );
}

#[test]
fn test_hamiltoniancircuit_to_quadraticassignment_extract_solution() {
    let source = cycle4_hc();
    let reduction = ReduceTo::<Decision<QuadraticAssignment>>::reduce_to(&source)
        .expect("reduction should succeed");

    // Permutation [0,1,2,3] visits 0->1->2->3->0 on cycle4
    let target_config = vec![0, 1, 2, 3];
    let extracted = reduction
        .recover_result(
            &source,
            SolveOutcome::optimal(reduction.target_problem(), target_config.clone()).unwrap(),
        )
        .unwrap()
        .into_solution()
        .expect("qualifying target result must recover a source solution");
    assert_eq!(extracted, vec![0, 1, 2, 3]);
    assert!(
        source.evaluate(&extracted).unwrap().0,
        "extracted solution should be a valid HC"
    );
}

#[test]
fn test_prism_graph_hc_via_qap_ilp_roundtrip() {
    use crate::models::algebraic::ILP;
    use crate::solvers::ILPSolver;

    // Prism graph: 6 vertices, 9 edges — the instance from #780.
    let edges = vec![
        (0, 1),
        (1, 2),
        (2, 0),
        (3, 4),
        (4, 5),
        (5, 3),
        (0, 3),
        (1, 4),
        (2, 5),
    ];
    let hc = HamiltonianCircuit::new(SimpleGraph::new(6, edges));

    // HC → QAP → ILP → solve → extract back
    let r1 = ReduceTo::<Decision<QuadraticAssignment>>::reduce_to(&hc)
        .expect("reduction should succeed");
    let r2 = ReduceTo::<ILP<bool>>::reduce_to(r1.target_problem().inner())
        .expect("reduction should succeed");
    let ilp_sol = ILPSolver::new()
        .solve(r2.target_problem())
        .expect("ILP should be feasible");
    let qap_sol = r2
        .recover_result(
            r1.target_problem().inner(),
            SolveOutcome::optimal(r2.target_problem(), ilp_sol).unwrap(),
        )
        .map(|outcome| outcome.into_solution().unwrap())
        .unwrap();
    let hc_sol = r1
        .recover_result(
            &hc,
            SolveOutcome::optimal(r1.target_problem(), qap_sol.clone()).unwrap(),
        )
        .unwrap()
        .into_solution()
        .expect("qualifying target result must recover a source solution");

    assert!(
        hc.evaluate(&hc_sol).unwrap().0,
        "prism graph HC via QAP→ILP should produce a valid Hamiltonian circuit, got {:?}",
        hc_sol
    );
}

#[test]
fn test_hamiltoniancircuit_to_quadraticassignment_small_graphs_are_no() {
    for (n, edges) in [
        (0, vec![]),
        (1, vec![]),
        (1, vec![(0, 0)]),
        (2, vec![]),
        (2, vec![(0, 1)]),
        (2, vec![(0, 0), (0, 1), (0, 1), (1, 1)]),
    ] {
        let source = HamiltonianCircuit::new(SimpleGraph::new(n, edges));
        assert!(!source.evaluate(&(0..n).collect()).unwrap().0);
        let reduction = ReduceTo::<Decision<QuadraticAssignment>>::reduce_to(&source).unwrap();
        let target = reduction.target_problem();
        assert!(BruteForce::new().solve(target).unwrap().is_none());
        assert_eq!(target.inner().num_facilities(), 3);
        assert_eq!(target.inner().num_locations(), 3);
        let best = BruteForce::new().solve(target.inner()).unwrap().unwrap();
        let value = target.inner().evaluate(&best).unwrap();
        assert_eq!(value, crate::types::Min(Some(3)));
        assert!(
            !crate::types::Or(OptimizationValue::meets_bound(
                &(value),
                crate::rules::ReductionResult::target_problem(&reduction).bound()
            ))
            .0
        );
        assert!(!ReductionResult::target_problem(&reduction)
            .evaluate(&best)
            .unwrap()
            .is_valid());
    }
}

#[test]
fn test_hamiltoniancircuit_to_quadraticassignment_rejects_invalid_certificates() {
    let reduction = ReduceTo::<Decision<QuadraticAssignment>>::reduce_to(&cycle4_hc()).unwrap();
    for config in [
        vec![],
        vec![0, 1, 2],
        vec![0, 1, 2, 4],
        vec![0, 0, 1, 2],
        vec![0, 2, 1, 3],
    ] {
        assert!(
            !matches!(reduction.target_problem().inner().evaluate(&config),
            Ok(value) if crate::types::Or(OptimizationValue::meets_bound(&(value), crate::rules::ReductionResult::target_problem(&reduction).bound())).0)
        );
    }
    for value in [crate::types::Min(None), crate::types::Min(Some(1))] {
        assert!(
            !crate::types::Or(OptimizationValue::meets_bound(
                &(value),
                crate::rules::ReductionResult::target_problem(&reduction).bound()
            ))
            .0
        );
    }
    assert!(
        crate::types::Or(OptimizationValue::meets_bound(
            &(crate::types::Min(Some(0))),
            crate::rules::ReductionResult::target_problem(&reduction).bound()
        ))
        .0
    );
}

#[test]
fn test_hamiltoniancircuit_to_quadraticassignment_all_small_graphs_and_orders() {
    for n in 3usize..=4 {
        let pairs: Vec<_> = (0..n)
            .flat_map(|u| (u + 1..n).map(move |v| (u, v)))
            .collect();
        for mask in 0..(1usize << pairs.len()) {
            let mut edges: Vec<_> = pairs
                .iter()
                .enumerate()
                .filter(|(i, _)| mask & (1 << i) != 0)
                .map(|(_, &e)| e)
                .collect();
            // Native loops and repeated edges must preserve the same equivalence.
            edges.extend((0..n).map(|v| (v, v)));
            edges.extend(edges.clone());
            let source = HamiltonianCircuit::new(SimpleGraph::new(n, edges));
            let reduction = ReduceTo::<Decision<QuadraticAssignment>>::reduce_to(&source).unwrap();
            for mut encoded in 0..n.pow(u32::try_from(n).unwrap()) {
                let order: Vec<_> = (0..n)
                    .map(|_| {
                        let vertex = encoded % n;
                        encoded /= n;
                        vertex
                    })
                    .collect();
                if order
                    .iter()
                    .enumerate()
                    .any(|(i, v)| order[..i].contains(v))
                {
                    assert_eq!(
                        reduction.target_problem().inner().evaluate(&order).unwrap(),
                        crate::types::Min(None)
                    );
                    continue;
                }
                let missing = (0..n)
                    .filter(|&i| !source.graph().has_edge(order[i], order[(i + 1) % n]))
                    .count();
                let value = reduction.target_problem().inner().evaluate(&order).unwrap();
                assert_eq!(
                    value,
                    crate::types::Min(Some(i64::try_from(missing).unwrap()))
                );
                let expected = source.evaluate(&order).unwrap().0;
                assert_eq!(
                    crate::types::Or(OptimizationValue::meets_bound(
                        &(value),
                        crate::rules::ReductionResult::target_problem(&reduction).bound()
                    ))
                    .0,
                    expected
                );
                if expected {
                    assert_eq!(
                        reduction
                            .recover_result(
                                &source,
                                SolveOutcome::optimal(reduction.target_problem(), order.clone())
                                    .unwrap()
                            )
                            .unwrap()
                            .into_solution()
                            .expect("qualifying target result must recover a source solution"),
                        order
                    );
                } else {
                    assert!(!ReductionResult::target_problem(&reduction)
                        .evaluate(&order)
                        .unwrap()
                        .is_valid());
                }
            }
        }
    }
}

#[test]
fn test_hamiltoniancircuit_to_quadraticassignment_registered_aggregate_path() {
    use crate::rules::{ReductionGraph, ReductionPath, ReductionStep};

    let graph = ReductionGraph::new();
    let path = ReductionPath {
        steps: vec![
            ReductionStep {
                name: HamiltonianCircuit::<SimpleGraph>::NAME.to_string(),
                variant: HamiltonianCircuit::<SimpleGraph>::variant()
                    .into_iter()
                    .map(|(key, value)| (key.to_string(), value.to_string()))
                    .collect(),
            },
            ReductionStep {
                name: Decision::<QuadraticAssignment>::NAME.to_string(),
                variant: Default::default(),
            },
        ],
    };
    for (source, expected) in [
        (cycle4_hc(), true),
        (HamiltonianCircuit::new(SimpleGraph::star(4)), false),
        (
            HamiltonianCircuit::new(SimpleGraph::new(2, vec![(0, 1)])),
            false,
        ),
    ] {
        let chain = graph.reduce_along_path(&path, &source).unwrap().unwrap();
        let target = chain.target_problem::<Decision<QuadraticAssignment>>();
        let best = BruteForce::new().solve(target.inner()).unwrap().unwrap();
        let optimum = target.evaluate(&best).unwrap();
        let outcome = if optimum.0 {
            SolveOutcome::optimal(target, best).unwrap()
        } else {
            SolveOutcome::Infeasible
        };
        let recovered = chain
            .recover_result::<HamiltonianCircuit<SimpleGraph>, Decision<QuadraticAssignment>>(
                &source, outcome,
            )
            .unwrap();
        assert_eq!(recovered.solution().is_some(), expected);
        if let Some(witness) = recovered.solution() {
            assert_eq!(source.evaluate(witness).unwrap(), crate::types::Or(true));
        }
        assert_eq!(
            BruteForce::new().solve(&source).unwrap().is_some(),
            expected
        );
    }
}
