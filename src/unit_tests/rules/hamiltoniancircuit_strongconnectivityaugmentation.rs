use crate::models::graph::{HamiltonianCircuit, StrongConnectivityAugmentation};
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
fn test_hamiltoniancircuit_to_strongconnectivityaugmentation_closed_loop() {
    let source = cycle4_hc();
    let reduction = ReduceTo::<StrongConnectivityAugmentation<i32>>::reduce_to(&source);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "HamiltonianCircuit -> StrongConnectivityAugmentation",
    );
}

#[test]
fn test_hamiltoniancircuit_to_strongconnectivityaugmentation_structure() {
    let source = cycle4_hc();
    let reduction = ReduceTo::<StrongConnectivityAugmentation<i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    // Arc-less digraph on 4 vertices
    assert_eq!(target.num_vertices(), 4);
    assert_eq!(target.num_arcs(), 0);

    // n*(n-1) = 12 candidate arcs
    assert_eq!(target.num_potential_arcs(), 12);

    // Budget = n = 4
    assert_eq!(*target.bound(), 4);

    // Weight-1 arcs correspond to edges in the source graph
    let mut weight1_count = 0;
    for &(u, v, w) in target.candidate_arcs() {
        if source.graph().has_edge(u, v) {
            assert_eq!(w, 1, "arc ({u}, {v}) should have weight 1");
            weight1_count += 1;
        } else {
            assert_eq!(w, 2, "arc ({u}, {v}) should have weight 2");
        }
    }
    // Cycle on 4 vertices has 4 edges => 8 directed weight-1 arcs
    assert_eq!(weight1_count, 8);
}

#[test]
fn test_hamiltoniancircuit_to_strongconnectivityaugmentation_nonhamiltonian() {
    // Star graph on 4 vertices (center=0, leaves=1,2,3) has no Hamiltonian circuit.
    let source = HamiltonianCircuit::new(SimpleGraph::star(4));
    let reduction = ReduceTo::<StrongConnectivityAugmentation<i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    // With budget n=4, the only way to get strong connectivity at cost 4
    // is to use 4 weight-1 arcs. But star graph has 3 edges => 6 weight-1 arcs,
    // and no Hamiltonian circuit exists, so no feasible solution should exist.
    let witness = BruteForce::new().find_witness(target);
    assert!(
        witness.is_none(),
        "non-Hamiltonian source should yield infeasible SCA"
    );
}

#[test]
fn test_hamiltoniancircuit_to_strongconnectivityaugmentation_extract_solution() {
    let source = cycle4_hc();
    let reduction = ReduceTo::<StrongConnectivityAugmentation<i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    // Manually build the target config for directed cycle 0->1->2->3->0.
    let n = 4;
    let mut target_config = vec![0usize; n * (n - 1)];
    let cycle_arcs = [(0, 1), (1, 2), (2, 3), (3, 0)];
    for (u, v) in cycle_arcs {
        let idx = u * (n - 1) + if v > u { v - 1 } else { v };
        target_config[idx] = 1;
    }

    assert!(target.is_valid_solution(&target_config));

    let extracted = reduction.extract_solution(&target_config);
    assert_eq!(extracted.len(), 4);
    assert!(
        source.evaluate(&extracted).is_valid(),
        "extracted solution must be a valid Hamiltonian circuit"
    );
}
