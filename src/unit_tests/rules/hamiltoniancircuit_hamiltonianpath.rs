use crate::models::graph::{HamiltonianCircuit, HamiltonianPath};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::rules::ReduceTo;
use crate::rules::ReductionResult;
use crate::solvers::BruteForce;
use crate::topology::{Graph, SimpleGraph};
use crate::Problem;

fn cycle4_hc() -> HamiltonianCircuit<SimpleGraph> {
    HamiltonianCircuit::new(SimpleGraph::cycle(4))
}

#[test]
fn test_hamiltoniancircuit_to_hamiltonianpath_closed_loop() {
    let source = cycle4_hc();
    let reduction = ReduceTo::<HamiltonianPath<SimpleGraph>>::reduce_to(&source);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "HamiltonianCircuit -> HamiltonianPath",
    );
}

#[test]
fn test_hamiltoniancircuit_to_hamiltonianpath_structure() {
    let source = cycle4_hc();
    let reduction = ReduceTo::<HamiltonianPath<SimpleGraph>>::reduce_to(&source);
    let target = reduction.target_problem();

    // Original: 4 vertices, 4 edges (cycle)
    // Target: 4 + 3 = 7 vertices
    assert_eq!(target.graph().num_vertices(), 7);

    // deg(0) in cycle4 = 2, so target edges = 4 + 2 + 2 = 8
    assert_eq!(target.graph().num_edges(), 8);

    // s (=5) should only be connected to vertex 0
    let s_neighbors = target.graph().neighbors(5);
    assert_eq!(s_neighbors, vec![0]);

    // t (=6) should only be connected to v' (=4)
    let t_neighbors = target.graph().neighbors(6);
    assert_eq!(t_neighbors, vec![4]);

    // v' (=4) should be connected to neighbors of 0 in original graph plus t
    let v_prime_neighbors = target.graph().neighbors(4);
    assert!(v_prime_neighbors.contains(&1)); // neighbor of 0
    assert!(v_prime_neighbors.contains(&3)); // neighbor of 0
    assert!(v_prime_neighbors.contains(&6)); // t
}

#[test]
fn test_hamiltoniancircuit_to_hamiltonianpath_extract_solution() {
    let source = cycle4_hc();
    let reduction = ReduceTo::<HamiltonianPath<SimpleGraph>>::reduce_to(&source);

    // HP solution: s=5, 0, 1, 2, 3, v'=4, t=6
    let hp_config = vec![5, 0, 1, 2, 3, 4, 6];
    let extracted = reduction.extract_solution(&hp_config);

    assert_eq!(extracted.len(), 4);
    assert!(
        source.evaluate(&extracted).0,
        "extracted solution must be a valid HC"
    );
}

#[test]
fn test_hamiltoniancircuit_to_hamiltonianpath_extract_reversed() {
    let source = cycle4_hc();
    let reduction = ReduceTo::<HamiltonianPath<SimpleGraph>>::reduce_to(&source);

    // HP solution reversed: t=6, v'=4, 3, 2, 1, 0, s=5
    let hp_config = vec![6, 4, 3, 2, 1, 0, 5];
    let extracted = reduction.extract_solution(&hp_config);

    assert_eq!(extracted.len(), 4);
    assert!(
        source.evaluate(&extracted).0,
        "extracted reversed solution must be a valid HC"
    );
}

#[test]
fn test_hamiltoniancircuit_to_hamiltonianpath_no_circuit() {
    // Path graph 0-1-2-3: no Hamiltonian circuit (vertices 0 and 3 have degree 1)
    let source = HamiltonianCircuit::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]));
    let reduction = ReduceTo::<HamiltonianPath<SimpleGraph>>::reduce_to(&source);
    let target = reduction.target_problem();

    // The target should have no Hamiltonian path (since source has no HC)
    let solver = BruteForce::new();
    let witness = solver.find_witness(target);
    assert!(
        witness.is_none(),
        "target should have no HP when source has no HC"
    );
}

#[test]
fn test_hamiltoniancircuit_to_hamiltonianpath_triangle() {
    // Triangle graph: 3 vertices, 3 edges, has HC
    let source = HamiltonianCircuit::new(SimpleGraph::cycle(3));
    let reduction = ReduceTo::<HamiltonianPath<SimpleGraph>>::reduce_to(&source);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "HamiltonianCircuit(triangle) -> HamiltonianPath",
    );
}

#[test]
fn test_hamiltoniancircuit_to_hamiltonianpath_two_vertex_special_case_is_unsatisfiable() {
    let source = HamiltonianCircuit::new(SimpleGraph::new(2, vec![(0, 1)]));
    let reduction = ReduceTo::<HamiltonianPath<SimpleGraph>>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.graph().num_vertices(), 5);
    assert_eq!(target.graph().num_edges(), 0);

    let solver = BruteForce::new();
    assert!(
        solver.find_witness(target).is_none(),
        "2-vertex source should reduce to an unsatisfiable HamiltonianPath instance"
    );
}
