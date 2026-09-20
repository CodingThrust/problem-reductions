use super::*;
use crate::models::decision::Decision;
use crate::models::graph::{HamiltonianCircuit, MinimumVertexCover};
use crate::rules::ReduceTo;
use crate::solvers::BruteForce;
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;

fn decision_mvc(
    num_vertices: usize,
    edges: &[(usize, usize)],
    k: i64,
) -> Decision<MinimumVertexCover<SimpleGraph, One>> {
    Decision::new(
        MinimumVertexCover::new(
            SimpleGraph::new(num_vertices, edges.to_vec()),
            vec![One; num_vertices],
        ),
        k,
    )
}

#[test]
fn test_decisionminimumvertexcover_to_hamiltoniancircuit_structure_counts() {
    let source = decision_mvc(3, &[(0, 1), (1, 2)], 1);
    let reduction = ReduceTo::<HamiltonianCircuit<SimpleGraph>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();

    assert_eq!(target.num_vertices(), 25);
    assert_eq!(target.num_edges(), 35);
    assert_eq!(target.graph().neighbors(0).len(), 6);
}

#[test]
fn test_decisionminimumvertexcover_to_hamiltoniancircuit_closed_loop() {
    let source = decision_mvc(3, &[(0, 1), (1, 2)], 1);
    let reduction = ReduceTo::<HamiltonianCircuit<SimpleGraph>>::reduce_to(&source)
        .expect("reduction should succeed");

    let cover = vec![false, true, false];
    let target_witness = reduction.build_target_witness(&cover);

    assert!(
        reduction
            .target_problem()
            .evaluate(&target_witness)
            .unwrap()
            .0
    );

    let extracted = reduction.extract_solution(&target_witness).unwrap();
    assert_eq!(extracted, cover);
    assert!(source.evaluate(&extracted).unwrap().0);
}

#[test]
fn test_decisionminimumvertexcover_to_hamiltoniancircuit_ignores_isolated_vertices() {
    let source = decision_mvc(3, &[(0, 1)], 1);
    let reduction = ReduceTo::<HamiltonianCircuit<SimpleGraph>>::reduce_to(&source)
        .expect("reduction should succeed");

    let target_witness = reduction.build_target_witness(&[true, false, false]);
    assert!(
        reduction
            .target_problem()
            .evaluate(&target_witness)
            .unwrap()
            .0
    );

    let extracted = reduction.extract_solution(&target_witness).unwrap();
    assert_eq!(extracted.len(), 3);
    assert!(!extracted[2]);
    assert!(source.evaluate(&extracted).unwrap().0);
}

#[test]
fn test_decisionminimumvertexcover_to_hamiltoniancircuit_fixed_yes_when_k_covers_all_active_vertices(
) {
    let source = decision_mvc(3, &[(0, 1), (1, 2)], 3);
    let reduction = ReduceTo::<HamiltonianCircuit<SimpleGraph>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();

    assert_eq!(target.num_vertices(), 3);
    assert_eq!(target.num_edges(), 3);

    let witness = BruteForce::new()
        .solve(target)
        .unwrap()
        .expect("triangle should have a Hamiltonian circuit");
    let extracted = reduction.extract_solution(&witness).unwrap();
    assert!(source.evaluate(&extracted).unwrap().0);
}

#[test]
fn test_decisionminimumvertexcover_to_hamiltoniancircuit_fixed_no_when_k_zero() {
    let source = decision_mvc(2, &[(0, 1)], 0);
    let reduction = ReduceTo::<HamiltonianCircuit<SimpleGraph>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();

    assert_eq!(target.num_vertices(), 3);
    assert!(BruteForce::new().solve(target).unwrap().is_none());
}

#[test]
fn test_self_loops_consume_cover_budget() {
    for (edges, bound, cover) in [
        (vec![(0, 0), (0, 1)], 1, vec![true, false, false, false]),
        (
            vec![(0, 0), (1, 2), (2, 3)],
            2,
            vec![true, false, true, false],
        ),
    ] {
        let source = decision_mvc(4, &edges, bound);
        let result = ReduceTo::<HamiltonianCircuit<SimpleGraph>>::reduce_to(&source).unwrap();
        let witness = result.build_target_witness(&cover);
        assert!(result.target_problem().evaluate(&witness).unwrap().0);
        let extracted = result.extract_solution(&witness).unwrap();
        assert!(extracted[0]);
        assert!(source.evaluate(&extracted).unwrap().0);
        assert!(result.extract_solution(&vec![]).is_err());
    }
    for bound in [-1, 0, 1] {
        let source = decision_mvc(2, &[(0, 0), (1, 1)], bound);
        let result = ReduceTo::<HamiltonianCircuit<SimpleGraph>>::reduce_to(&source).unwrap();
        assert!(BruteForce::new().solve(&source).unwrap().is_none());
        assert!(BruteForce::new()
            .solve(result.target_problem())
            .unwrap()
            .is_none());
        assert_eq!(
            crate::rules::AggregateReductionResult::extract_value(&result, crate::types::Or(false)),
            crate::types::Or(false)
        );
        assert!(result.extract_solution(&vec![0, 1, 2]).is_err());
    }
}

#[test]
fn test_registered_aggregate_preserves_decision() {
    let entries = crate::rules::registry::reduction_entries();
    let edge = entries
        .iter()
        .find(|edge| {
            edge.source_name == "DecisionMinimumVertexCover"
                && (edge.source_variant_fn)()
                    == Decision::<MinimumVertexCover<SimpleGraph, One>>::variant()
                && edge.target_name == "HamiltonianCircuit"
        })
        .unwrap();
    for bound in [0, 1] {
        let source = decision_mvc(1, &[(0, 0)], bound);
        let result = (edge.reduce_aggregate_fn.unwrap())(&source).unwrap();
        assert_eq!(
            result
                .extract_value_from_solution_dyn(&vec![0usize, 1, 2])
                .unwrap(),
            serde_json::json!(bound == 1),
        );
    }
}
