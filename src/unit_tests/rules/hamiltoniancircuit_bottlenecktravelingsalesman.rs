use crate::models::graph::{BottleneckTravelingSalesman, HamiltonianCircuit};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::rules::ReduceTo;
use crate::rules::ReductionResult;
use crate::solvers::BruteForce;
use crate::topology::{Graph, SimpleGraph};
use crate::types::Min;
use crate::Problem;

fn cycle5_hc() -> HamiltonianCircuit<SimpleGraph> {
    HamiltonianCircuit::new(SimpleGraph::cycle(5))
}

#[test]
fn test_hamiltoniancircuit_to_bottlenecktravelingsalesman_closed_loop() {
    let source = cycle5_hc();
    let reduction = ReduceTo::<BottleneckTravelingSalesman>::reduce_to(&source);

    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &reduction,
        "HamiltonianCircuit -> BottleneckTravelingSalesman",
    );
}

#[test]
fn test_hamiltoniancircuit_to_bottlenecktravelingsalesman_structure() {
    let source = cycle5_hc();
    let reduction = ReduceTo::<BottleneckTravelingSalesman>::reduce_to(&source);
    let target = reduction.target_problem();

    // Complete graph on 5 vertices: C(5,2) = 10 edges
    assert_eq!(target.graph().num_vertices(), 5);
    assert_eq!(target.graph().num_edges(), 10);

    // Edge weights: 1 for cycle edges, 2 for non-cycle edges
    for ((u, v), weight) in target.graph().edges().into_iter().zip(target.weights()) {
        let expected = if source.graph().has_edge(u, v) { 1 } else { 2 };
        assert_eq!(weight, expected, "unexpected weight on edge ({u}, {v})");
    }
}

#[test]
fn test_hamiltoniancircuit_to_bottlenecktravelingsalesman_nonhamiltonian_bottleneck_gap() {
    // Star graph has no Hamiltonian circuit, so optimal bottleneck must exceed 1
    let source = HamiltonianCircuit::new(SimpleGraph::star(5));
    let reduction = ReduceTo::<BottleneckTravelingSalesman>::reduce_to(&source);
    let target = reduction.target_problem();
    let best = BruteForce::new()
        .find_witness(target)
        .expect("complete weighted graph should always admit a tour");

    let metric = target.evaluate(&best);
    assert!(metric.is_valid(), "best BTSP solution evaluated as invalid");
    assert!(
        metric.unwrap() > 1,
        "expected bottleneck > 1 for non-Hamiltonian source"
    );
}

#[test]
fn test_hamiltoniancircuit_to_bottlenecktravelingsalesman_extract_solution_cycle() {
    let source = cycle5_hc();
    let reduction = ReduceTo::<BottleneckTravelingSalesman>::reduce_to(&source);
    let target = reduction.target_problem();

    // Manually select the cycle edges in the complete graph
    let cycle_edges = [(0usize, 1usize), (1, 2), (2, 3), (3, 4), (0, 4)];
    let target_solution: Vec<usize> = target
        .graph()
        .edges()
        .into_iter()
        .map(|(u, v)| usize::from(cycle_edges.contains(&(u, v)) || cycle_edges.contains(&(v, u))))
        .collect();

    let extracted = reduction.extract_solution(&target_solution);

    // Bottleneck should be 1 (all selected edges are original cycle edges)
    assert_eq!(target.evaluate(&target_solution), Min(Some(1)));
    assert_eq!(extracted.len(), 5);
    assert!(source.evaluate(&extracted).is_valid());
}
