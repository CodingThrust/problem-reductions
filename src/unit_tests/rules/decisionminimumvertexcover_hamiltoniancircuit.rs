use super::*;
use crate::models::decision::Decision;
use crate::models::graph::{HamiltonianCircuit, MinimumVertexCover};
use crate::rules::ReduceTo;
use crate::solvers::BruteForce;
use crate::solvers::SolveOutcome;
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

    let extracted = reduction
        .recover_result(
            &source,
            SolveOutcome::optimal(reduction.target_problem(), target_witness.clone()).unwrap(),
        )
        .unwrap()
        .into_solution()
        .expect("qualifying target result must recover a source solution");
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

    let extracted = reduction
        .recover_result(
            &source,
            SolveOutcome::optimal(reduction.target_problem(), target_witness.clone()).unwrap(),
        )
        .unwrap()
        .into_solution()
        .expect("qualifying target result must recover a source solution");
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
    let extracted = reduction
        .recover_result(
            &source,
            SolveOutcome::optimal(reduction.target_problem(), witness.clone()).unwrap(),
        )
        .unwrap()
        .into_solution()
        .expect("qualifying target result must recover a source solution");
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
fn hamiltonian_edge_registers_only_unit_weight_vertex_cover() {
    let sources = inventory::iter::<crate::rules::ReductionEntry>
        .into_iter()
        .filter(|entry| {
            entry.source_name == "DecisionMinimumVertexCover"
                && entry.target_name == "HamiltonianCircuit"
        })
        .map(|entry| (entry.source_variant_fn)())
        .collect::<Vec<_>>();
    assert_eq!(
        sources,
        vec![Decision::<MinimumVertexCover<SimpleGraph, One>>::variant()]
    );
}

#[test]
fn unit_cover_bound_handles_negative_and_empty_graphs() {
    for (bound, expected) in [(-1, false), (0, true), (i64::MAX, true)] {
        let source = decision_mvc(2, &[], bound);
        let reduction = ReduceTo::<HamiltonianCircuit<SimpleGraph>>::reduce_to(&source).unwrap();
        let witness = BruteForce::new().solve(reduction.target_problem()).unwrap();
        assert_eq!(witness.is_some(), expected);
        if let Some(witness) = witness {
            let cover = reduction
                .recover_result(
                    &source,
                    SolveOutcome::optimal(reduction.target_problem(), witness.clone()).unwrap(),
                )
                .unwrap()
                .into_solution()
                .expect("qualifying target result must recover a source solution");
            assert!(source.evaluate(&cover).unwrap().0);
        }
    }
}

#[test]
fn loops_force_vertices_before_the_loopless_construction() {
    for (n, edges, bound, cover) in [
        (2, vec![(0, 0), (0, 1)], 1, vec![true, false]),
        (
            4,
            vec![(0, 0), (0, 1), (1, 2), (2, 3)],
            2,
            vec![true, false, true, false],
        ),
    ] {
        let source = decision_mvc(n, &edges, bound);
        let reduction = ReduceTo::<HamiltonianCircuit<SimpleGraph>>::reduce_to(&source).unwrap();
        let witness = reduction.build_target_witness(&cover);
        let recovered = reduction
            .recover_result(
                &source,
                SolveOutcome::optimal(reduction.target_problem(), witness).unwrap(),
            )
            .unwrap()
            .into_solution()
            .unwrap();
        assert!(recovered[0]);
        assert!(source.evaluate(&recovered).unwrap().0);
    }
    for (edges, bound) in [(vec![(0, 0), (1, 1)], 1), (vec![(0, 0), (1, 2)], 1)] {
        let source = decision_mvc(3, &edges, bound);
        let reduction = ReduceTo::<HamiltonianCircuit<SimpleGraph>>::reduce_to(&source).unwrap();
        assert!(BruteForce::new()
            .solve(reduction.target_problem())
            .unwrap()
            .is_none());
    }
}
