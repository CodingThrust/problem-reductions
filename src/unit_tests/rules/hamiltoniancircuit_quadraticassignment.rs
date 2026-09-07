use crate::models::algebraic::QuadraticAssignment;
use crate::models::graph::HamiltonianCircuit;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::rules::ReduceTo;
use crate::rules::ReductionResult;
use crate::solvers::BruteForce;
use crate::topology::{Graph, SimpleGraph};
use crate::types::Min;
use crate::Problem;

fn cycle4_hc() -> HamiltonianCircuit<SimpleGraph> {
    HamiltonianCircuit::new(SimpleGraph::cycle(4))
}

#[test]
fn test_hamiltoniancircuit_to_quadraticassignment_closed_loop() {
    let source = cycle4_hc();
    let reduction =
        ReduceTo::<QuadraticAssignment>::reduce_to(&source).expect("reduction should succeed");

    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &reduction,
        "HamiltonianCircuit -> QuadraticAssignment",
    );
}

#[test]
fn test_hamiltoniancircuit_to_quadraticassignment_structure() {
    let source = cycle4_hc();
    let reduction =
        ReduceTo::<QuadraticAssignment>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    assert_eq!(target.num_facilities(), 4);
    assert_eq!(target.num_locations(), 4);

    // Cost matrix: cycle adjacency on positions
    let cost = target.cost_matrix();
    for (i, cost_row) in cost.iter().enumerate() {
        for (j, &cost_val) in cost_row.iter().enumerate() {
            let expected = if j == (i + 1) % 4 { 1 } else { 0 };
            assert_eq!(cost_val, expected, "cost[{i}][{j}] should be {expected}");
        }
    }

    // Distance matrix: edges and diagonal cost zero, non-edges cost one.
    let dist = target.distance_matrix();
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
    let reduction =
        ReduceTo::<QuadraticAssignment>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    // The identity permutation [0,1,2,3] is a valid HC on a 4-cycle,
    // so the QAP optimum should be zero.
    let best = BruteForce::new()
        .solve(target)
        .unwrap()
        .expect("QAP should have an optimal solution");
    let value = target.evaluate(&best).unwrap();
    assert_eq!(value, Min(Some(0)), "optimal QAP cost should be zero");
}

#[test]
fn test_hamiltoniancircuit_to_quadraticassignment_nonhamiltonian_cost_gap() {
    // Star graph on 4 vertices has no Hamiltonian circuit
    let source = HamiltonianCircuit::new(SimpleGraph::star(4));
    let reduction =
        ReduceTo::<QuadraticAssignment>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    let best = BruteForce::new()
        .solve(target)
        .unwrap()
        .expect("QAP always has a solution");
    let value = target.evaluate(&best).unwrap();
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
    let reduction =
        ReduceTo::<QuadraticAssignment>::reduce_to(&source).expect("reduction should succeed");

    // Permutation [0,1,2,3] visits 0->1->2->3->0 on cycle4
    let target_config = vec![0, 1, 2, 3];
    let extracted = reduction.extract_solution(&target_config).unwrap();
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
    let r1 = ReduceTo::<QuadraticAssignment>::reduce_to(&hc).expect("reduction should succeed");
    let r2 =
        ReduceTo::<ILP<bool>>::reduce_to(r1.target_problem()).expect("reduction should succeed");
    let ilp_sol = ILPSolver::new()
        .solve(r2.target_problem())
        .expect("ILP should be feasible");
    let qap_sol = r2.extract_solution(&ilp_sol).unwrap();
    let hc_sol = r1.extract_solution(&qap_sol).unwrap();

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
        let reduction = ReduceTo::<QuadraticAssignment>::reduce_to(&source).unwrap();
        let target = reduction.target_problem();
        assert_eq!(target.num_facilities(), 3);
        assert_eq!(target.num_locations(), 3);
        let best = BruteForce::new().solve(target).unwrap().unwrap();
        let value = target.evaluate(&best).unwrap();
        assert_eq!(value, Min(Some(3)));
        assert!(!crate::rules::AggregateReductionResult::extract_value(&reduction, value).0);
        assert!(reduction.extract_solution(&best).is_err());
    }
}

#[test]
fn test_hamiltoniancircuit_to_quadraticassignment_rejects_invalid_certificates() {
    let reduction = ReduceTo::<QuadraticAssignment>::reduce_to(&cycle4_hc()).unwrap();
    for config in [
        vec![],
        vec![0, 1, 2],
        vec![0, 1, 2, 4],
        vec![0, 0, 1, 2],
        vec![0, 2, 1, 3],
    ] {
        assert!(reduction.extract_solution(&config).is_err(), "{config:?}");
    }
    for value in [Min(None), Min(Some(-1)), Min(Some(1))] {
        assert!(!crate::rules::AggregateReductionResult::extract_value(&reduction, value).0);
    }
    assert!(crate::rules::AggregateReductionResult::extract_value(&reduction, Min(Some(0))).0);
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
            let reduction = ReduceTo::<QuadraticAssignment>::reduce_to(&source).unwrap();
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
                        reduction.target_problem().evaluate(&order).unwrap(),
                        Min(None)
                    );
                    assert!(reduction.extract_solution(&order).is_err());
                    continue;
                }
                let missing = (0..n)
                    .filter(|&i| !source.graph().has_edge(order[i], order[(i + 1) % n]))
                    .count();
                let value = reduction.target_problem().evaluate(&order).unwrap();
                assert_eq!(value, Min(Some(i64::try_from(missing).unwrap())));
                let expected = source.evaluate(&order).unwrap().0;
                assert_eq!(
                    crate::rules::AggregateReductionResult::extract_value(&reduction, value).0,
                    expected
                );
                if expected {
                    assert_eq!(reduction.extract_solution(&order).unwrap(), order);
                } else {
                    assert!(reduction.extract_solution(&order).is_err());
                }
            }
        }
    }
}

#[test]
fn test_hamiltoniancircuit_to_quadraticassignment_registered_aggregate_path() {
    use crate::rules::{ReductionGraph, ReductionPath, ReductionStep};
    use crate::types::Or;

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
                name: QuadraticAssignment::NAME.to_string(),
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
        let chain = graph
            .reduce_aggregate_along_path(&path, &source)
            .unwrap()
            .unwrap();
        let target = chain.target_problem::<QuadraticAssignment>();
        let best = BruteForce::new().solve(target).unwrap().unwrap();
        let optimum = target.evaluate(&best).unwrap();
        assert_eq!(
            chain.extract_value_dyn(serde_json::to_value(optimum).unwrap()),
            serde_json::to_value(Or(expected)).unwrap(),
        );
        assert_eq!(
            BruteForce::new().solve(&source).unwrap().is_some(),
            expected
        );
    }
}
