use crate::models::graph::{BottleneckTravelingSalesman, HamiltonianCircuit};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::rules::ReduceTo;
use crate::rules::ReductionResult;
use crate::solvers::BruteForce;
use crate::solvers::SolveOutcome;
use crate::topology::{Graph, SimpleGraph};
use crate::types::Min;
use crate::Problem;

fn cycle5_hc() -> HamiltonianCircuit<SimpleGraph> {
    HamiltonianCircuit::new(SimpleGraph::cycle(5))
}

#[test]
fn test_hamiltoniancircuit_to_bottlenecktravelingsalesman_closed_loop() {
    let source = cycle5_hc();
    let reduction = ReduceTo::<BottleneckTravelingSalesman>::reduce_to(&source)
        .expect("reduction should succeed");

    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &reduction,
        "HamiltonianCircuit -> BottleneckTravelingSalesman",
    );
}

#[test]
fn test_hamiltoniancircuit_to_bottlenecktravelingsalesman_structure() {
    let source = cycle5_hc();
    let reduction = ReduceTo::<BottleneckTravelingSalesman>::reduce_to(&source)
        .expect("reduction should succeed");
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
    let reduction = ReduceTo::<BottleneckTravelingSalesman>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();
    let best = BruteForce::new()
        .solve(target)
        .unwrap()
        .expect("complete weighted graph should always admit a tour");

    let metric = target.evaluate(&best).unwrap();
    assert!(metric.is_valid(), "best BTSP solution evaluated as invalid");
    assert!(
        metric.unwrap() > 1,
        "expected bottleneck > 1 for non-Hamiltonian source"
    );
}

#[test]
fn test_hamiltoniancircuit_to_bottlenecktravelingsalesman_extract_solution_cycle() {
    let source = cycle5_hc();
    let reduction = ReduceTo::<BottleneckTravelingSalesman>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();

    // Manually select the cycle edges in the complete graph
    let cycle_edges = [(0usize, 1usize), (1, 2), (2, 3), (3, 4), (0, 4)];
    let target_solution: Vec<bool> = target
        .graph()
        .edges()
        .into_iter()
        .map(|(u, v)| cycle_edges.contains(&(u, v)) || cycle_edges.contains(&(v, u)))
        .collect();

    let extracted = reduction
        .recover_result(
            &source,
            SolveOutcome::optimal(reduction.target_problem(), target_solution.clone()).unwrap(),
        )
        .unwrap()
        .into_solution()
        .expect("qualifying target result must recover a source solution");

    // Bottleneck should be 1 (all selected edges are original cycle edges)
    assert_eq!(target.evaluate(&target_solution).unwrap(), Min(Some(1)));
    assert_eq!(extracted.len(), 5);
    assert!(source.evaluate(&extracted).unwrap().is_valid());
}

/// Edge selection of the closed tour visiting `order` in the complete target graph.
fn tour_edges(graph: &SimpleGraph, order: &[usize]) -> Vec<bool> {
    graph
        .edges()
        .into_iter()
        .map(|(u, v)| {
            (0..order.len()).any(|i| {
                let (a, b) = (order[i], order[(i + 1) % order.len()]);
                (a, b) == (u, v) || (a, b) == (v, u)
            })
        })
        .collect()
}

#[test]
fn test_hamiltoniancircuit_to_bottlenecktravelingsalesman_feasible_target_incumbents() {
    let source = cycle5_hc();
    let reduction = ReduceTo::<BottleneckTravelingSalesman>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();

    // 0-1-2-3-4 uses only source edges (bottleneck 1): the incumbent is a Hamiltonian circuit.
    let circuit = tour_edges(target.graph(), &[0, 1, 2, 3, 4]);
    assert_eq!(target.evaluate(&circuit).unwrap(), Min(Some(1)));
    let recovered = reduction
        .recover_result(&source, SolveOutcome::feasible(target, circuit).unwrap())
        .unwrap();
    let SolveOutcome::Feasible {
        solution,
        evaluation,
    } = recovered
    else {
        panic!("a Hamiltonian incumbent must stay feasible, got {recovered:?}");
    };
    assert_eq!(evaluation, crate::types::Or(true));
    assert_eq!(source.evaluate(&solution).unwrap(), crate::types::Or(true));

    // 0-2-4-1-3 is the pentagram (bottleneck 2): a valid tour that proves nothing about the source.
    let pentagram = tour_edges(target.graph(), &[0, 2, 4, 1, 3]);
    assert_eq!(target.evaluate(&pentagram).unwrap(), Min(Some(2)));
    assert_eq!(
        reduction.recover_result(&source, SolveOutcome::feasible(target, pentagram).unwrap()),
        Err(crate::rules::ExtractionError::InsufficientSolutionQuality)
    );
}
