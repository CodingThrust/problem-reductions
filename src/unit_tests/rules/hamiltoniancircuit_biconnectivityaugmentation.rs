use crate::models::graph::{BiconnectivityAugmentation, HamiltonianCircuit};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::rules::ReduceTo;
use crate::rules::ReductionResult;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::Problem;

fn cycle4_hc() -> HamiltonianCircuit<SimpleGraph> {
    HamiltonianCircuit::new(SimpleGraph::cycle(4))
}

#[test]
fn test_hamiltoniancircuit_to_biconnectivityaugmentation_closed_loop() {
    let source = cycle4_hc();
    let reduction = ReduceTo::<BiconnectivityAugmentation<SimpleGraph, i32>>::reduce_to(&source);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "HamiltonianCircuit -> BiconnectivityAugmentation",
    );
}

#[test]
fn test_hamiltoniancircuit_to_biconnectivityaugmentation_structure() {
    let source = cycle4_hc();
    let reduction = ReduceTo::<BiconnectivityAugmentation<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    // Same number of vertices
    assert_eq!(target.num_vertices(), 4);

    // Initial graph is edgeless
    assert_eq!(target.num_edges(), 0);

    // All pairs: C(4,2) = 6 potential edges
    assert_eq!(target.num_potential_edges(), 6);

    // Budget = n = 4
    assert_eq!(*target.budget(), 4);

    // Check weights: edges in cycle have weight 1, non-edges have weight 2
    let weights = target.potential_weights();
    // (0,1) in cycle => w=1
    assert_eq!(weights[0], (0, 1, 1));
    // (0,2) not in cycle => w=2
    assert_eq!(weights[1], (0, 2, 2));
    // (0,3) in cycle => w=1
    assert_eq!(weights[2], (0, 3, 1));
    // (1,2) in cycle => w=1
    assert_eq!(weights[3], (1, 2, 1));
    // (1,3) not in cycle => w=2
    assert_eq!(weights[4], (1, 3, 2));
    // (2,3) in cycle => w=1
    assert_eq!(weights[5], (2, 3, 1));
}

#[test]
fn test_hamiltoniancircuit_to_biconnectivityaugmentation_extract_solution() {
    let source = cycle4_hc();
    let reduction = ReduceTo::<BiconnectivityAugmentation<SimpleGraph, i32>>::reduce_to(&source);

    // Select edges (0,1), (0,3), (1,2), (2,3) => config [1, 0, 1, 1, 0, 1]
    let target_config = vec![1, 0, 1, 1, 0, 1];
    let extracted = reduction.extract_solution(&target_config);

    assert_eq!(extracted.len(), 4);
    assert!(
        source.evaluate(&extracted).0,
        "extracted solution must be a valid HC"
    );
}

#[test]
fn test_hamiltoniancircuit_to_biconnectivityaugmentation_no_circuit() {
    // Path graph 0-1-2-3: no Hamiltonian circuit (endpoints have degree 1)
    let source = HamiltonianCircuit::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]));
    let reduction = ReduceTo::<BiconnectivityAugmentation<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    // The target should have no feasible augmentation
    let solver = BruteForce::new();
    let witness = solver.find_witness(target);
    assert!(
        witness.is_none(),
        "target should be infeasible when source has no HC"
    );
}

#[test]
fn test_hamiltoniancircuit_to_biconnectivityaugmentation_triangle() {
    // Triangle graph: 3 vertices, 3 edges, has HC
    let source = HamiltonianCircuit::new(SimpleGraph::cycle(3));
    let reduction = ReduceTo::<BiconnectivityAugmentation<SimpleGraph, i32>>::reduce_to(&source);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "HamiltonianCircuit(triangle) -> BiconnectivityAugmentation",
    );
}

#[test]
fn test_hamiltoniancircuit_to_biconnectivityaugmentation_complete4() {
    // Complete graph K4: has many Hamiltonian circuits
    let source = HamiltonianCircuit::new(SimpleGraph::complete(4));
    let reduction = ReduceTo::<BiconnectivityAugmentation<SimpleGraph, i32>>::reduce_to(&source);

    // All potential edges have weight 1 (K4 has all edges)
    let target = reduction.target_problem();
    for &(_, _, w) in target.potential_weights() {
        assert_eq!(w, 1, "all edges in K4 should have weight 1");
    }

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "HamiltonianCircuit(K4) -> BiconnectivityAugmentation",
    );
}
