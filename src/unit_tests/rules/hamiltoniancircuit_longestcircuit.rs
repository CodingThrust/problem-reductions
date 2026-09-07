use crate::models::graph::{HamiltonianCircuit, LongestCircuit};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::rules::ReduceTo;
use crate::rules::ReductionResult;
use crate::solvers::BruteForce;
use crate::topology::{Graph, SimpleGraph};
use crate::types::Max;
use crate::Problem;

fn cycle4_hc() -> HamiltonianCircuit<SimpleGraph> {
    HamiltonianCircuit::new(SimpleGraph::cycle(4))
}

#[test]
fn test_hamiltoniancircuit_aggregate_requires_a_spanning_cycle() {
    let reduction = ReduceTo::<LongestCircuit<SimpleGraph, i64>>::reduce_to(&cycle4_hc()).unwrap();
    for (value, expected) in [
        (Max(None), false),
        (Max(Some(3)), false),
        (Max(Some(4)), true),
    ] {
        assert_eq!(
            crate::rules::AggregateReductionResult::extract_value(&reduction, value),
            crate::types::Or(expected),
        );
    }
    let short_cycle = HamiltonianCircuit::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (0, 2)]));
    let reduction = ReduceTo::<LongestCircuit<SimpleGraph, i64>>::reduce_to(&short_cycle).unwrap();
    assert!(reduction.extract_solution(&vec![true; 3]).is_err());
}

#[test]
fn test_hamiltoniancircuit_to_longestcircuit_closed_loop() {
    let source = cycle4_hc();
    let reduction = ReduceTo::<LongestCircuit<SimpleGraph, i64>>::reduce_to(&source)
        .expect("reduction should succeed");

    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &reduction,
        "HamiltonianCircuit -> LongestCircuit",
    );
}

#[test]
fn test_hamiltoniancircuit_to_longestcircuit_structure() {
    let source = cycle4_hc();
    let reduction = ReduceTo::<LongestCircuit<SimpleGraph, i64>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();

    // Same graph structure
    assert_eq!(target.graph().num_vertices(), 4);
    assert_eq!(target.graph().num_edges(), 4);

    // All unit weights
    assert!(target.edge_lengths().iter().all(|&w| w == 1));
}

#[test]
fn test_hamiltoniancircuit_to_longestcircuit_nonhamiltonian() {
    // Star graph on 4 vertices: no Hamiltonian circuit
    let source = HamiltonianCircuit::new(SimpleGraph::star(4));
    let reduction = ReduceTo::<LongestCircuit<SimpleGraph, i64>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();

    let solver = BruteForce::new();
    let witness = solver.solve(target).unwrap();

    match witness {
        Some(sol) => {
            let value = target.evaluate(&sol).unwrap();
            // Optimal circuit length must be strictly less than n=4
            assert!(
                value.unwrap() < 4,
                "star graph should not have a circuit of length 4"
            );
        }
        None => {
            // No circuit at all in a star graph — also acceptable
        }
    }
}

#[test]
fn test_hamiltoniancircuit_to_longestcircuit_extract_solution() {
    let source = cycle4_hc();
    let reduction = ReduceTo::<LongestCircuit<SimpleGraph, i64>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();

    // All edges selected forms a Hamiltonian circuit on the cycle graph
    let target_solution = vec![true, true, true, true];
    let extracted = reduction.extract_solution(&target_solution).unwrap();

    assert_eq!(target.evaluate(&target_solution).unwrap(), Max(Some(4)));
    assert_eq!(extracted.len(), 4);
    assert!(source.evaluate(&extracted).unwrap());
}

#[test]
fn test_hamiltoniancircuit_extraction_matches_all_small_target_configurations() {
    // Enumerate every simple graph and every edge selection, including graphs
    // with no circuit, short circuits, and malformed selections of valid length.
    for n in 0..=4 {
        let possible: Vec<_> = (0..n)
            .flat_map(|u| ((u + 1)..n).map(move |v| (u, v)))
            .collect();
        for graph_mask in 0usize..(1 << possible.len()) {
            let edges: Vec<_> = possible
                .iter()
                .enumerate()
                .filter_map(|(i, &edge)| ((graph_mask >> i) & 1 == 1).then_some(edge))
                .collect();
            let source = HamiltonianCircuit::new(SimpleGraph::new(n, edges));
            let reduction =
                ReduceTo::<LongestCircuit<SimpleGraph, i64>>::reduce_to(&source).unwrap();
            let target = crate::rules::AggregateReductionResult::target_problem(&reduction);
            for mask in 0usize..(1 << target.num_edges()) {
                let config: Vec<_> = (0..target.num_edges())
                    .map(|i| (mask >> i) & 1 == 1)
                    .collect();
                let value = target.evaluate(&config).unwrap();
                let certifies = value.0 == Some(n as i64);
                let extracted = reduction.extract_solution(&config);
                assert_eq!(
                    extracted.is_ok(),
                    certifies,
                    "n={n}, graph={graph_mask}, config={mask}"
                );
                if let Ok(order) = extracted {
                    assert!(source.evaluate(&order).unwrap().0);
                }
            }
            assert!(reduction
                .extract_solution(&vec![false; target.num_edges() + 1])
                .is_err());
        }
    }
}
