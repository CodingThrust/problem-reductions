use crate::models::graph::HamiltonianCircuit;
use crate::models::misc::StackerCrane;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::rules::ReduceTo;
use crate::rules::ReductionResult;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::types::Min;
use crate::Problem;

fn cycle4_hc() -> HamiltonianCircuit<SimpleGraph> {
    HamiltonianCircuit::new(SimpleGraph::cycle(4))
}

#[test]
fn test_hamiltoniancircuit_to_stackercrane_closed_loop() {
    let source = cycle4_hc();
    let reduction = ReduceTo::<StackerCrane>::reduce_to(&source).expect("reduction should succeed");

    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &reduction,
        "HamiltonianCircuit -> StackerCrane",
    );
}

#[test]
fn test_hamiltoniancircuit_to_stackercrane_structure() {
    let source = cycle4_hc();
    let reduction = ReduceTo::<StackerCrane>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    // 4 vertices -> 8 target vertices (2 per original vertex)
    assert_eq!(target.num_vertices(), 8);
    // 4 arcs (one per original vertex)
    assert_eq!(target.num_arcs(), 4);
    // 4 original edges -> 8 undirected connector edges
    assert_eq!(target.num_edges(), 8);

    // All arcs have length 1
    for &len in target.arc_lengths() {
        assert_eq!(len, 1);
    }
    // All connector edges have length 1
    for &len in target.edge_lengths() {
        assert_eq!(len, 1);
    }
}

#[test]
fn test_hamiltoniancircuit_to_stackercrane_optimal_cost() {
    // A 4-cycle has a Hamiltonian circuit; optimal StackerCrane cost = 2n = 8.
    let source = cycle4_hc();
    let reduction = ReduceTo::<StackerCrane>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    let witness = BruteForce::new()
        .solve(target)
        .unwrap()
        .expect("target should have a solution");
    let cost = target.evaluate(&witness).unwrap();
    assert_eq!(cost, Min(Some(8)));
}

#[test]
fn test_hamiltoniancircuit_to_stackercrane_non_hamiltonian() {
    // Star graph on 4 vertices: no Hamiltonian circuit.
    // The optimal StackerCrane cost should exceed 2n = 8.
    let source = HamiltonianCircuit::new(SimpleGraph::star(4));
    let reduction = ReduceTo::<StackerCrane>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    let witness = BruteForce::new().solve(target).unwrap();
    match witness {
        Some(w) => {
            let cost = target.evaluate(&w).unwrap();
            assert!(
                cost.0.unwrap() > 8,
                "non-Hamiltonian graph should have cost > 2n"
            );
        }
        None => {
            // Disconnected split graph has no feasible walk — also correct.
        }
    }
}

#[test]
fn test_hamiltoniancircuit_to_stackercrane_extract_solution() {
    let source = cycle4_hc();
    let reduction = ReduceTo::<StackerCrane>::reduce_to(&source).expect("reduction should succeed");

    // The identity permutation [0, 1, 2, 3] traverses arcs in order,
    // corresponding to vertex order 0, 1, 2, 3 in the original graph.
    let target_config = vec![0, 1, 2, 3];
    let extracted = reduction.extract_solution(&target_config).unwrap();
    assert_eq!(extracted, vec![0, 1, 2, 3]);
    assert!(
        source.evaluate(&extracted).unwrap().0,
        "extracted solution should be a valid HC"
    );
}

#[test]
fn test_hamiltoniancircuit_to_stackercrane_prism_graph() {
    // Regression test for #789: prism graph (6 vertices, 9 edges) has a
    // Hamiltonian circuit, but with zero-cost connectors the ILP could find
    // an optimal SC permutation that doesn't correspond to a valid HC.
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
    let source = HamiltonianCircuit::new(SimpleGraph::new(6, edges));
    let reduction = ReduceTo::<StackerCrane>::reduce_to(&source).expect("reduction should succeed");

    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &reduction,
        "HamiltonianCircuit -> StackerCrane (prism graph)",
    );
}

#[test]
fn test_stackercrane_certificate_for_all_small_configurations() {
    for n in 0..=4 {
        let possible: Vec<_> = (0..n)
            .flat_map(|u| ((u + 1)..n).map(move |v| (u, v)))
            .collect();
        for mask in 0usize..(1 << possible.len()) {
            let edges = possible
                .iter()
                .enumerate()
                .filter_map(|(i, &e)| ((mask >> i) & 1 == 1).then_some(e))
                .collect();
            let source = HamiltonianCircuit::new(SimpleGraph::new(n, edges));
            let reduction = ReduceTo::<StackerCrane>::reduce_to(&source).unwrap();
            let target = crate::rules::AggregateReductionResult::target_problem(&reduction);
            // All coordinate configurations, including repeated arc indices.
            for mut code in 0..n.pow(n as u32) {
                let config: Vec<_> = (0..n)
                    .map(|_| {
                        let i = code % n;
                        code /= n;
                        i
                    })
                    .collect();
                let expected = source.evaluate(&config).unwrap().0;
                let value = target.evaluate(&config).unwrap();
                assert_eq!(
                    crate::rules::AggregateReductionResult::extract_value(&reduction, value).0,
                    expected
                );
                let decoded = reduction.extract_solution(&config);
                assert_eq!(
                    decoded.is_ok(),
                    expected,
                    "n={n}, mask={mask}, config={config:?}"
                );
                if let Ok(order) = decoded {
                    assert!(source.evaluate(&order).unwrap().0);
                }
            }
            assert!(reduction.extract_solution(&vec![0; n + 1]).is_err());
            if n > 0 {
                assert!(reduction.extract_solution(&vec![n; n]).is_err());
            }
        }
    }
}

#[test]
fn test_stackercrane_checked_dimensions_and_cost_bound() {
    use super::split_graph_dimensions;
    assert_eq!(split_graph_dimensions(0, 0).unwrap(), (0, 0));
    assert_eq!(split_graph_dimensions(4, 4).unwrap(), (8, 8));
    for (n, m) in [(usize::MAX, 0), (0, usize::MAX), (usize::MAX / 2, 0)] {
        assert!(matches!(
            split_graph_dimensions(n, m),
            Err(crate::rules::ReductionError::IntegerOverflow { .. })
        ));
    }
    #[cfg(target_pointer_width = "64")]
    {
        assert!(split_graph_dimensions(2_147_483_647, 0).is_ok());
        assert!(matches!(
            split_graph_dimensions(2_147_483_648, 0),
            Err(crate::rules::ReductionError::IntegerOverflow { .. })
        ));
    }
}
