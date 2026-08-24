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
    weights: &[i64],
    k: i64,
) -> Decision<MinimumVertexCover<SimpleGraph, i64>> {
    Decision::new(
        MinimumVertexCover::new(
            SimpleGraph::new(num_vertices, edges.to_vec()),
            weights.to_vec(),
        ),
        k,
    )
}

#[test]
fn test_decisionminimumvertexcover_to_hamiltoniancircuit_structure_counts() {
    let source = decision_mvc(3, &[(0, 1), (1, 2)], &[1, 1, 1], 1);
    let reduction = ReduceTo::<HamiltonianCircuit<SimpleGraph>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();

    assert_eq!(target.num_vertices(), 25);
    assert_eq!(target.num_edges(), 35);
    assert_eq!(target.graph().neighbors(0).len(), 6);
}

#[test]
fn test_decisionminimumvertexcover_to_hamiltoniancircuit_closed_loop() {
    let source = decision_mvc(3, &[(0, 1), (1, 2)], &[1, 1, 1], 1);
    let reduction = ReduceTo::<HamiltonianCircuit<SimpleGraph>>::reduce_to(&source)
        .expect("reduction should succeed");

    let cover = vec![0, 1, 0];
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
    let source = decision_mvc(3, &[(0, 1)], &[1, 1, 1], 1);
    let reduction = ReduceTo::<HamiltonianCircuit<SimpleGraph>>::reduce_to(&source)
        .expect("reduction should succeed");

    let target_witness = reduction.build_target_witness(&[1, 0, 0]);
    assert!(
        reduction
            .target_problem()
            .evaluate(&target_witness)
            .unwrap()
            .0
    );

    let extracted = reduction.extract_solution(&target_witness).unwrap();
    assert_eq!(extracted.len(), 3);
    assert_eq!(extracted[2], 0);
    assert!(source.evaluate(&extracted).unwrap().0);
}

#[test]
fn test_decisionminimumvertexcover_to_hamiltoniancircuit_fixed_yes_when_k_covers_all_active_vertices(
) {
    let source = decision_mvc(3, &[(0, 1), (1, 2)], &[1, 1, 1], 3);
    let reduction = ReduceTo::<HamiltonianCircuit<SimpleGraph>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();

    assert_eq!(target.num_vertices(), 3);
    assert_eq!(target.num_edges(), 3);

    let witness = BruteForce::new()
        .find_witness(target)
        .unwrap()
        .expect("triangle should have a Hamiltonian circuit");
    let extracted = reduction.extract_solution(&witness).unwrap();
    assert!(source.evaluate(&extracted).unwrap().0);
}

#[test]
fn test_decisionminimumvertexcover_to_hamiltoniancircuit_fixed_no_when_k_zero() {
    let source = decision_mvc(2, &[(0, 1)], &[1, 1], 0);
    let reduction = ReduceTo::<HamiltonianCircuit<SimpleGraph>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();

    assert_eq!(target.num_vertices(), 3);
    assert!(BruteForce::new().find_witness(target).unwrap().is_none());
}

#[test]
fn test_decisionminimumvertexcover_to_hamiltoniancircuit_rejects_non_unit_weights() {
    let source = decision_mvc(2, &[(0, 1)], &[2, 1], 1);
    let error = ReduceTo::<HamiltonianCircuit<SimpleGraph>>::reduce_to(&source).unwrap_err();
    assert!(matches!(
        error,
        crate::rules::ReductionError::InvalidTarget { .. }
    ));
}
