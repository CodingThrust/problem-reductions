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
    let reduction = ReduceTo::<BiconnectivityAugmentation<SimpleGraph, i64>>::reduce_to(&source)
        .expect("reduction should succeed");

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "HamiltonianCircuit -> BiconnectivityAugmentation",
    );
}

#[test]
fn test_hamiltoniancircuit_to_biconnectivityaugmentation_structure() {
    let source = cycle4_hc();
    let reduction = ReduceTo::<BiconnectivityAugmentation<SimpleGraph, i64>>::reduce_to(&source)
        .expect("reduction should succeed");
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
    let reduction = ReduceTo::<BiconnectivityAugmentation<SimpleGraph, i64>>::reduce_to(&source)
        .expect("reduction should succeed");

    // Select edges (0,1), (0,3), (1,2), (2,3) => config [1, 0, 1, 1, 0, 1]
    let target_config = vec![true, false, true, true, false, true];
    let extracted = reduction.extract_solution(&target_config).unwrap();

    assert_eq!(extracted.len(), 4);
    assert!(
        source.evaluate(&extracted).unwrap().0,
        "extracted solution must be a valid HC"
    );
}

#[test]
fn test_hamiltoniancircuit_to_biconnectivityaugmentation_no_circuit() {
    // Path graph 0-1-2-3: no Hamiltonian circuit (endpoints have degree 1)
    let source = HamiltonianCircuit::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]));
    let reduction = ReduceTo::<BiconnectivityAugmentation<SimpleGraph, i64>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();

    // The target should have no feasible augmentation
    let solver = BruteForce::new();
    let witness = solver.solve(target).unwrap();
    assert!(
        witness.is_none(),
        "target should be infeasible when source has no HC"
    );
}

#[test]
fn test_hamiltoniancircuit_to_biconnectivityaugmentation_triangle() {
    // Triangle graph: 3 vertices, 3 edges, has HC
    let source = HamiltonianCircuit::new(SimpleGraph::cycle(3));
    let reduction = ReduceTo::<BiconnectivityAugmentation<SimpleGraph, i64>>::reduce_to(&source)
        .expect("reduction should succeed");

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
    let reduction = ReduceTo::<BiconnectivityAugmentation<SimpleGraph, i64>>::reduce_to(&source)
        .expect("reduction should succeed");

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

#[test]
fn test_hamiltoniancircuit_to_biconnectivityaugmentation_small_graphs() {
    for n in 0..3 {
        let edges = if n == 2 {
            vec![(0, 1), (0, 1), (1, 1)]
        } else {
            vec![]
        };
        let source = HamiltonianCircuit::new(SimpleGraph::new(n, edges));
        let reduction =
            ReduceTo::<BiconnectivityAugmentation<SimpleGraph, i64>>::reduce_to(&source).unwrap();
        let target = reduction.target_problem();
        assert_eq!(target.num_vertices(), 3);
        assert_eq!(target.num_edges(), 0);
        assert_eq!(target.num_potential_edges(), 0);
        assert_eq!(*target.budget(), 0);
        assert!(!target.evaluate(&vec![]).unwrap().0);
        assert!(reduction.extract_solution(&vec![]).is_err());
        assert!(BruteForce::new().solve(&source).unwrap().is_none());
        assert!(BruteForce::new().solve(target).unwrap().is_none());
    }
}

#[test]
fn test_hamiltoniancircuit_to_biconnectivityaugmentation_all_graphs_and_certificates() {
    for n in 3..=4 {
        let pairs: Vec<_> = (0..n)
            .flat_map(|u| (u + 1..n).map(move |v| (u, v)))
            .collect();
        for edge_mask in 0..1usize << pairs.len() {
            let mut edges: Vec<_> = pairs
                .iter()
                .enumerate()
                .filter(|(i, _)| edge_mask & (1 << i) != 0)
                .map(|(_, &e)| e)
                .collect();
            // Native SimpleGraph inputs can contain loops and repeated edges.
            edges.extend(edges.clone());
            edges.extend((0..n).map(|v| (v, v)));
            let source = HamiltonianCircuit::new(SimpleGraph::new(n, edges));
            let reduction =
                ReduceTo::<BiconnectivityAugmentation<SimpleGraph, i64>>::reduce_to(&source)
                    .unwrap();
            let mut target_yes = false;
            for mask in 0..1usize << pairs.len() {
                let config: Vec<_> = (0..pairs.len()).map(|i| mask & (1 << i) != 0).collect();
                let feasible = reduction.target_problem().evaluate(&config).unwrap().0;
                let extracted = reduction.extract_solution(&config);
                assert_eq!(extracted.is_ok(), feasible);
                if let Ok(circuit) = extracted {
                    assert!(source.evaluate(&circuit).unwrap().0);
                    target_yes = true;
                }
            }
            assert_eq!(
                target_yes,
                BruteForce::new().solve(&source).unwrap().is_some()
            );
        }
    }
}

#[test]
fn test_hamiltoniancircuit_to_biconnectivityaugmentation_rejects_infeasible_certificates() {
    let source = HamiltonianCircuit::new(SimpleGraph::empty(3));
    let reduction =
        ReduceTo::<BiconnectivityAugmentation<SimpleGraph, i64>>::reduce_to(&source).unwrap();
    // A spanning cycle made only of non-edges exceeds the budget and is not a source cycle.
    assert!(reduction.extract_solution(&vec![true; 3]).is_err());
    assert!(reduction.extract_solution(&vec![false; 3]).is_err());
    assert!(reduction.extract_solution(&vec![true; 2]).is_err());
    assert!(reduction.extract_solution(&vec![true; 4]).is_err());
    let source = HamiltonianCircuit::new(SimpleGraph::complete(6));
    let reduction =
        ReduceTo::<BiconnectivityAugmentation<SimpleGraph, i64>>::reduce_to(&source).unwrap();
    // Two disjoint triangles meet the budget and degree constraints but are disconnected.
    let config = reduction
        .target_problem()
        .potential_weights()
        .iter()
        .map(|&(u, v, _)| (u < 3) == (v < 3))
        .collect();
    assert!(reduction.extract_solution(&config).is_err());
}
