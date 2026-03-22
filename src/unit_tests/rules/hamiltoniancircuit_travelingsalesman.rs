use crate::models::graph::{HamiltonianCircuit, TravelingSalesman};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::rules::ReduceTo;
use crate::rules::ReductionResult;
use crate::solvers::{BruteForce, Solver};
use crate::topology::{Graph, SimpleGraph};
use crate::types::SolutionSize;
use crate::Problem;

fn cycle4_hc() -> HamiltonianCircuit<SimpleGraph> {
    HamiltonianCircuit::new(SimpleGraph::cycle(4))
}

#[test]
fn test_hamiltoniancircuit_to_travelingsalesman_closed_loop() {
    let source = cycle4_hc();
    let reduction = ReduceTo::<TravelingSalesman<SimpleGraph, i32>>::reduce_to(&source);

    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &reduction,
        "HamiltonianCircuit -> TravelingSalesman",
    );
}

#[test]
fn test_hamiltoniancircuit_to_travelingsalesman_structure() {
    let source = cycle4_hc();
    let reduction = ReduceTo::<TravelingSalesman<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.graph().num_vertices(), 4);
    assert_eq!(target.graph().num_edges(), 6);

    for ((u, v), weight) in target.graph().edges().into_iter().zip(target.weights()) {
        let expected = if source.graph().has_edge(u, v) { 1 } else { 2 };
        assert_eq!(weight, expected, "unexpected weight on edge ({u}, {v})");
    }
}

#[test]
fn test_hamiltoniancircuit_to_travelingsalesman_nonhamiltonian_cost_gap() {
    let source = HamiltonianCircuit::new(SimpleGraph::star(4));
    let reduction = ReduceTo::<TravelingSalesman<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();
    let best = BruteForce::new()
        .find_best(target)
        .expect("complete weighted graph should always admit a tour");

    match target.evaluate(&best) {
        SolutionSize::Valid(cost) => assert!(cost > 4, "expected cost > 4, got {cost}"),
        SolutionSize::Invalid => panic!("best TSP solution evaluated as invalid"),
    }
}

#[test]
fn test_hamiltoniancircuit_to_travelingsalesman_extract_solution_cycle() {
    let source = cycle4_hc();
    let reduction = ReduceTo::<TravelingSalesman<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();
    let cycle_edges = [(0usize, 1usize), (1, 2), (2, 3), (0, 3)];
    let target_solution: Vec<usize> = target
        .graph()
        .edges()
        .into_iter()
        .map(|(u, v)| usize::from(cycle_edges.contains(&(u, v)) || cycle_edges.contains(&(v, u))))
        .collect();

    let extracted = reduction.extract_solution(&target_solution);

    assert_eq!(target.evaluate(&target_solution), SolutionSize::Valid(4));
    assert_eq!(extracted.len(), 4);
    assert!(source.evaluate(&extracted));
}
