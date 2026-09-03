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
