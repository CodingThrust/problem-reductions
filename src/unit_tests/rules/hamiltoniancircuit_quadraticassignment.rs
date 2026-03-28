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
    let reduction = ReduceTo::<QuadraticAssignment>::reduce_to(&source);

    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &reduction,
        "HamiltonianCircuit -> QuadraticAssignment",
    );
}

#[test]
fn test_hamiltoniancircuit_to_quadraticassignment_structure() {
    let source = cycle4_hc();
    let reduction = ReduceTo::<QuadraticAssignment>::reduce_to(&source);
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

    // Distance matrix: edge = 1, non-edge = omega = 5
    let dist = target.distance_matrix();
    for (k, dist_row) in dist.iter().enumerate() {
        for (l, &dist_val) in dist_row.iter().enumerate() {
            let expected = if k == l {
                0
            } else if source.graph().has_edge(k, l) {
                1
            } else {
                5 // omega = n + 1 = 5
            };
            assert_eq!(dist_val, expected, "dist[{k}][{l}] should be {expected}");
        }
    }
}

#[test]
fn test_hamiltoniancircuit_to_quadraticassignment_optimal_cost_equals_n() {
    let source = cycle4_hc();
    let reduction = ReduceTo::<QuadraticAssignment>::reduce_to(&source);
    let target = reduction.target_problem();

    // The identity permutation [0,1,2,3] is a valid HC on a 4-cycle,
    // so the QAP optimum should be exactly n = 4.
    let best = BruteForce::new()
        .find_witness(target)
        .expect("QAP should have an optimal solution");
    let value = target.evaluate(&best);
    assert_eq!(value, Min(Some(4)), "optimal QAP cost should be n=4");
}

#[test]
fn test_hamiltoniancircuit_to_quadraticassignment_nonhamiltonian_cost_gap() {
    // Star graph on 4 vertices has no Hamiltonian circuit
    let source = HamiltonianCircuit::new(SimpleGraph::star(4));
    let reduction = ReduceTo::<QuadraticAssignment>::reduce_to(&source);
    let target = reduction.target_problem();
    let n = source.num_vertices();

    let best = BruteForce::new()
        .find_witness(target)
        .expect("QAP always has a solution");
    let value = target.evaluate(&best);
    assert!(
        value.is_valid(),
        "QAP solution should have a valid objective"
    );
    assert!(
        value.unwrap() > n as i64,
        "expected QAP cost > {n} for non-Hamiltonian graph, got {:?}",
        value
    );
}

#[test]
fn test_hamiltoniancircuit_to_quadraticassignment_extract_solution() {
    let source = cycle4_hc();
    let reduction = ReduceTo::<QuadraticAssignment>::reduce_to(&source);

    // Permutation [0,1,2,3] visits 0->1->2->3->0 on cycle4
    let target_config = vec![0, 1, 2, 3];
    let extracted = reduction.extract_solution(&target_config);
    assert_eq!(extracted, vec![0, 1, 2, 3]);
    assert!(
        source.evaluate(&extracted).0,
        "extracted solution should be a valid HC"
    );
}
