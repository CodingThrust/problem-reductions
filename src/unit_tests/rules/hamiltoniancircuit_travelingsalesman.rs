use crate::models::graph::{HamiltonianCircuit, TravelingSalesman};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::rules::ReduceTo;
use crate::rules::ReductionResult;
use crate::solvers::BruteForce;
use crate::solvers::SolveOutcome;
use crate::topology::{Graph, SimpleGraph};
use crate::types::Min;
use crate::Problem;

fn cycle4_hc() -> HamiltonianCircuit<SimpleGraph> {
    HamiltonianCircuit::new(SimpleGraph::cycle(4))
}

#[test]
fn test_hamiltoniancircuit_to_travelingsalesman_closed_loop() {
    let source = cycle4_hc();
    let reduction = ReduceTo::<TravelingSalesman<SimpleGraph, i64>>::reduce_to(&source)
        .expect("reduction should succeed");

    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &reduction,
        "HamiltonianCircuit -> TravelingSalesman",
    );
}

#[test]
fn test_hamiltoniancircuit_to_travelingsalesman_structure() {
    let source = cycle4_hc();
    let reduction = ReduceTo::<TravelingSalesman<SimpleGraph, i64>>::reduce_to(&source)
        .expect("reduction should succeed");
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
    let reduction = ReduceTo::<TravelingSalesman<SimpleGraph, i64>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();
    let best = BruteForce::new()
        .solve(target)
        .unwrap()
        .expect("complete weighted graph should always admit a tour");

    let metric = target.evaluate(&best).unwrap();
    assert!(metric.is_valid(), "best TSP solution evaluated as invalid");
    assert!(metric.unwrap() > 4, "expected cost > 4");
}

#[test]
fn test_hamiltoniancircuit_to_travelingsalesman_extract_solution_cycle() {
    let source = cycle4_hc();
    let reduction = ReduceTo::<TravelingSalesman<SimpleGraph, i64>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();
    let cycle_edges = [(0usize, 1usize), (1, 2), (2, 3), (0, 3)];
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

    assert_eq!(target.evaluate(&target_solution).unwrap(), Min(Some(4)));
    assert_eq!(extracted.len(), 4);
    assert!(source.evaluate(&extracted).unwrap());
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
fn test_hamiltoniancircuit_to_travelingsalesman_feasible_target_incumbents() {
    let source = cycle4_hc();
    let reduction = ReduceTo::<TravelingSalesman<SimpleGraph, i64>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();

    // 0-1-2-3 uses only source edges (cost 4): the incumbent already is a Hamiltonian circuit.
    let circuit = tour_edges(target.graph(), &[0, 1, 2, 3]);
    assert_eq!(target.evaluate(&circuit).unwrap(), Min(Some(4)));
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

    // 0-1-3-2 uses the two diagonals (cost 6): a valid tour that proves nothing about the source.
    let detour = tour_edges(target.graph(), &[0, 1, 3, 2]);
    assert_eq!(target.evaluate(&detour).unwrap(), Min(Some(6)));
    assert_eq!(
        reduction.recover_result(&source, SolveOutcome::feasible(target, detour).unwrap()),
        Err(crate::rules::ExtractionError::InsufficientSolutionQuality)
    );
}
