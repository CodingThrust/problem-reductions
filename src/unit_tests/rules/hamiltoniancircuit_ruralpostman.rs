use crate::models::graph::{HamiltonianCircuit, RuralPostman};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::rules::ReduceTo;
use crate::rules::ReductionResult;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::types::Min;
use crate::Problem;

fn triangle_hc() -> HamiltonianCircuit<SimpleGraph> {
    HamiltonianCircuit::new(SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]))
}

fn cycle4_hc() -> HamiltonianCircuit<SimpleGraph> {
    HamiltonianCircuit::new(SimpleGraph::cycle(4))
}

#[test]
fn test_hamiltoniancircuit_to_ruralpostman_closed_loop() {
    let source = triangle_hc();
    let reduction = ReduceTo::<RuralPostman<SimpleGraph, i32>>::reduce_to(&source);

    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &reduction,
        "HamiltonianCircuit -> RuralPostman (triangle)",
    );
}

#[test]
fn test_hamiltoniancircuit_to_ruralpostman_closed_loop_cycle4() {
    let source = cycle4_hc();
    let reduction = ReduceTo::<RuralPostman<SimpleGraph, i32>>::reduce_to(&source);

    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &reduction,
        "HamiltonianCircuit -> RuralPostman (cycle4)",
    );
}

#[test]
fn test_hamiltoniancircuit_to_ruralpostman_structure() {
    let source = triangle_hc();
    let reduction = ReduceTo::<RuralPostman<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    // 3 vertices -> 6 vertices
    assert_eq!(target.num_vertices(), 6);
    // 3 required edges + 2*3 connectivity edges = 9
    assert_eq!(target.num_edges(), 9);
    // 3 required edges (one per vertex)
    assert_eq!(target.num_required_edges(), 3);

    // All edges have weight 1
    let weights = target.edge_lengths();
    for (i, &w) in weights.iter().enumerate() {
        assert_eq!(w, 1, "edge {i} should have weight 1");
    }
}

#[test]
fn test_hamiltoniancircuit_to_ruralpostman_structure_cycle4() {
    let source = cycle4_hc();
    let reduction = ReduceTo::<RuralPostman<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    // 4 vertices -> 8 vertices
    assert_eq!(target.num_vertices(), 8);
    // 4 required edges + 2*4 connectivity edges = 12
    assert_eq!(target.num_edges(), 12);
    // 4 required edges
    assert_eq!(target.num_required_edges(), 4);
}

#[test]
fn test_hamiltoniancircuit_to_ruralpostman_optimal_cost() {
    // Triangle has a Hamiltonian circuit, so optimal RPP cost should be 2n = 6
    let source = triangle_hc();
    let reduction = ReduceTo::<RuralPostman<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();
    let best = BruteForce::new()
        .find_witness(target)
        .expect("should find a solution");

    let metric = target.evaluate(&best);
    assert_eq!(metric, Min(Some(6)), "optimal cost should be 2n=6");
}

#[test]
fn test_hamiltoniancircuit_to_ruralpostman_nonhamiltonian_cost_gap() {
    // Star graph with 4 vertices has no Hamiltonian circuit
    let source = HamiltonianCircuit::new(SimpleGraph::star(4));
    let n = source.num_vertices();
    assert_eq!(n, 4);
    let reduction = ReduceTo::<RuralPostman<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    // Verify source has no Hamiltonian circuit
    let source_witness = BruteForce::new().find_witness(&source);
    assert!(source_witness.is_none(), "star graph should have no HC");

    // The RPP optimal cost should exceed 2n = 8
    let best = BruteForce::new().find_witness(target);
    if let Some(config) = best {
        let metric = target.evaluate(&config);
        assert!(
            metric.is_valid(),
            "best RPP solution should be a valid circuit"
        );
        let two_n = 2 * n as i32;
        assert!(
            metric.unwrap() > two_n,
            "non-Hamiltonian source should give RPP cost > 2n={two_n}, got {}",
            metric.unwrap()
        );
    }
}

#[test]
fn test_hamiltoniancircuit_to_ruralpostman_extract_solution() {
    let source = triangle_hc();
    let reduction = ReduceTo::<RuralPostman<SimpleGraph, i32>>::reduce_to(&source);

    let target = reduction.target_problem();
    let best = BruteForce::new()
        .find_witness(target)
        .expect("should find a solution");

    let extracted = reduction.extract_solution(&best);
    assert_eq!(
        extracted.len(),
        3,
        "extracted solution should have 3 vertices"
    );
    assert!(
        source.evaluate(&extracted).0,
        "extracted solution should be a valid Hamiltonian circuit"
    );
}
