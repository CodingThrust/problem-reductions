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
fn test_hamiltoniancircuit_to_longestcircuit_closed_loop() {
    let source = cycle4_hc();
    let reduction = ReduceTo::<LongestCircuit<SimpleGraph, i32>>::reduce_to(&source);

    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &reduction,
        "HamiltonianCircuit -> LongestCircuit",
    );
}

#[test]
fn test_hamiltoniancircuit_to_longestcircuit_structure() {
    let source = cycle4_hc();
    let reduction = ReduceTo::<LongestCircuit<SimpleGraph, i32>>::reduce_to(&source);
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
    let reduction = ReduceTo::<LongestCircuit<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    let solver = BruteForce::new();
    let witness = solver.find_witness(target);

    match witness {
        Some(sol) => {
            let value = target.evaluate(&sol);
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
    let reduction = ReduceTo::<LongestCircuit<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    // All edges selected forms a Hamiltonian circuit on the cycle graph
    let target_solution = vec![1, 1, 1, 1];
    let extracted = reduction.extract_solution(&target_solution);

    assert_eq!(target.evaluate(&target_solution), Max(Some(4)));
    assert_eq!(extracted.len(), 4);
    assert!(source.evaluate(&extracted));
}
