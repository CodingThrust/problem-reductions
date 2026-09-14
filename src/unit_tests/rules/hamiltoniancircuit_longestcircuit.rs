use crate::models::decision::Decision;
use crate::models::graph::{HamiltonianCircuit, LongestCircuit};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::rules::ReduceTo;
use crate::rules::ReductionResult;
use crate::solvers::BruteForce;
use crate::solvers::SolveOutcome;
use crate::topology::{Graph, SimpleGraph};
use crate::traits::EvaluationError::InvalidConfiguration;
use crate::types::OptimizationValue;
use crate::Problem;

fn cycle4_hc() -> HamiltonianCircuit<SimpleGraph> {
    HamiltonianCircuit::new(SimpleGraph::cycle(4))
}

#[test]
fn test_hamiltoniancircuit_aggregate_requires_a_spanning_cycle() {
    let reduction =
        ReduceTo::<Decision<LongestCircuit<SimpleGraph, i64>>>::reduce_to(&cycle4_hc()).unwrap();
    for (value, expected) in [
        (crate::types::Max(None), false),
        (crate::types::Max(Some(3)), false),
        (crate::types::Max(Some(4)), true),
    ] {
        assert_eq!(
            crate::types::Or(OptimizationValue::meets_bound(
                &(value),
                crate::rules::ReductionResult::target_problem(&reduction).bound()
            )),
            crate::types::Or(expected),
        );
    }
    let short_cycle = HamiltonianCircuit::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (0, 2)]));
    let reduction =
        ReduceTo::<Decision<LongestCircuit<SimpleGraph, i64>>>::reduce_to(&short_cycle).unwrap();
    assert!(!ReductionResult::target_problem(&reduction)
        .evaluate(&vec![true; 3])
        .unwrap()
        .is_valid());
}

#[test]
fn test_hamiltoniancircuit_to_longestcircuit_closed_loop() {
    let source = cycle4_hc();
    let reduction = ReduceTo::<Decision<LongestCircuit<SimpleGraph, i64>>>::reduce_to(&source)
        .expect("reduction should succeed");

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "HamiltonianCircuit -> LongestCircuit",
    );
}

#[test]
fn test_hamiltoniancircuit_to_longestcircuit_structure() {
    let source = cycle4_hc();
    let reduction = ReduceTo::<Decision<LongestCircuit<SimpleGraph, i64>>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();

    // Same graph structure
    assert_eq!(target.inner().graph().num_vertices(), 4);
    assert_eq!(target.inner().graph().num_edges(), 4);

    // All unit weights
    assert!(target.inner().edge_lengths().iter().all(|&w| w == 1));
}

#[test]
fn test_hamiltoniancircuit_to_longestcircuit_nonhamiltonian() {
    // Star graph on 4 vertices: no Hamiltonian circuit
    let source = HamiltonianCircuit::new(SimpleGraph::star(4));
    let reduction = ReduceTo::<Decision<LongestCircuit<SimpleGraph, i64>>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();

    let solver = BruteForce::new();
    let witness = solver.solve(target).unwrap();

    assert!(witness.is_none());
}

#[test]
fn test_hamiltoniancircuit_to_longestcircuit_extract_solution() {
    let source = cycle4_hc();
    let reduction = ReduceTo::<Decision<LongestCircuit<SimpleGraph, i64>>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();

    // All edges selected forms a Hamiltonian circuit on the cycle graph
    let target_solution = vec![true, true, true, true];
    let extracted = reduction
        .recover_result(
            &source,
            SolveOutcome::optimal(reduction.target_problem(), target_solution.clone()).unwrap(),
        )
        .unwrap()
        .into_solution()
        .expect("qualifying target result must recover a source solution");

    assert_eq!(
        target.inner().evaluate(&target_solution).unwrap(),
        crate::types::Max(Some(4))
    );
    assert_eq!(extracted.len(), 4);
    assert!(source.evaluate(&extracted).unwrap());
}

#[test]
fn test_hamiltoniancircuit_extraction_matches_all_small_target_configurations() {
    // Enumerate every simple graph and every edge selection, including graphs
    // with no circuit, short circuits, and malformed selections of valid length.
    for n in 0..=4 {
        let possible: Vec<_> = (0..n)
            .flat_map(|u| ((u + 1)..n).map(move |v| (u, v)))
            .collect();
        for graph_mask in 0usize..(1 << possible.len()) {
            let edges: Vec<_> = possible
                .iter()
                .enumerate()
                .filter_map(|(i, &edge)| ((graph_mask >> i) & 1 == 1).then_some(edge))
                .collect();
            let source = HamiltonianCircuit::new(SimpleGraph::new(n, edges));
            let reduction =
                ReduceTo::<Decision<LongestCircuit<SimpleGraph, i64>>>::reduce_to(&source).unwrap();
            let target = crate::rules::ReductionResult::target_problem(&reduction);
            for mask in 0usize..(1 << target.inner().num_edges()) {
                let config: Vec<_> = (0..target.inner().num_edges())
                    .map(|i| (mask >> i) & 1 == 1)
                    .collect();
                let value = target.inner().evaluate(&config).unwrap();
                let certifies = value.0 == Some(n as i64);
                if certifies {
                    let order = reduction
                        .recover_result(
                            &source,
                            SolveOutcome::optimal(reduction.target_problem(), config.clone())
                                .unwrap(),
                        )
                        .unwrap()
                        .into_solution()
                        .expect("qualifying target result must recover a source solution");
                    assert!(source.evaluate(&order).unwrap().0);
                }
            }
            assert!(matches!(
                ReductionResult::target_problem(&reduction)
                    .inner()
                    .evaluate(&vec![false; target.inner().num_edges() + 1]),
                Err(InvalidConfiguration(_))
            ));
        }
    }
}
