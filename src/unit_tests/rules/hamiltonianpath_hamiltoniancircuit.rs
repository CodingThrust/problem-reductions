use crate::models::graph::{HamiltonianCircuit, HamiltonianPath};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::rules::{ReduceTo, ReductionResult};
use crate::solvers::BruteForce;
use crate::topology::{Graph, SimpleGraph};
use crate::Problem;

fn canonical_path_example() -> HamiltonianPath<SimpleGraph> {
    HamiltonianPath::new(SimpleGraph::new(
        5,
        vec![(0, 1), (1, 2), (2, 3), (3, 4), (0, 2), (1, 3)],
    ))
}

#[test]
fn test_hamiltonianpath_to_hamiltoniancircuit_closed_loop() {
    let source = canonical_path_example();
    let reduction = ReduceTo::<HamiltonianCircuit<SimpleGraph>>::reduce_to(&source);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "HamiltonianPath -> HamiltonianCircuit",
    );

    let target = reduction.target_problem();
    assert_eq!(target.num_vertices(), 6);
    assert_eq!(target.num_edges(), 11);
    assert_eq!(target.graph().neighbors(5).len(), 5);
    for vertex in 0..5 {
        assert!(target.graph().has_edge(5, vertex));
    }
    for edge in source.graph().edges() {
        assert!(target.graph().has_edge(edge.0, edge.1));
    }
}

#[test]
fn test_extracts_rotated_and_reversed_circuits() {
    let source = canonical_path_example();
    let reduction = ReduceTo::<HamiltonianCircuit<SimpleGraph>>::reduce_to(&source);

    let rotated = vec![2, 3, 4, 5, 0, 1];
    assert!(reduction.target_problem().evaluate(&rotated).0);
    assert_eq!(reduction.extract_solution(&rotated), vec![0, 1, 2, 3, 4]);

    let reversed_and_rotated = vec![2, 1, 0, 5, 4, 3];
    assert!(reduction.target_problem().evaluate(&reversed_and_rotated).0);
    assert_eq!(
        reduction.extract_solution(&reversed_and_rotated),
        vec![4, 3, 2, 1, 0]
    );
}

#[test]
fn test_empty_and_singleton_sources_use_feasible_triangle() {
    let empty = HamiltonianPath::new(SimpleGraph::empty(0));
    let empty_reduction = ReduceTo::<HamiltonianCircuit<SimpleGraph>>::reduce_to(&empty);
    assert_eq!(
        empty_reduction.target_problem().graph(),
        &SimpleGraph::cycle(3)
    );
    assert_eq!(
        empty_reduction.extract_solution(&[0, 1, 2]),
        Vec::<usize>::new()
    );

    let singleton = HamiltonianPath::new(SimpleGraph::empty(1));
    let singleton_reduction = ReduceTo::<HamiltonianCircuit<SimpleGraph>>::reduce_to(&singleton);
    assert_eq!(
        singleton_reduction.target_problem().graph(),
        &SimpleGraph::cycle(3)
    );
    assert_eq!(singleton_reduction.extract_solution(&[2, 1, 0]), vec![0]);
}

#[test]
fn test_two_vertex_boundary_cases() {
    let isolated = HamiltonianPath::new(SimpleGraph::empty(2));
    let isolated_reduction = ReduceTo::<HamiltonianCircuit<SimpleGraph>>::reduce_to(&isolated);
    assert_eq!(isolated_reduction.target_problem().num_vertices(), 3);
    assert_eq!(isolated_reduction.target_problem().num_edges(), 2);
    assert!(BruteForce::new()
        .find_witness(isolated_reduction.target_problem())
        .is_none());

    let one_edge = HamiltonianPath::new(SimpleGraph::new(2, vec![(0, 1)]));
    let one_edge_reduction = ReduceTo::<HamiltonianCircuit<SimpleGraph>>::reduce_to(&one_edge);
    let target_solution = BruteForce::new()
        .find_witness(one_edge_reduction.target_problem())
        .expect("one source edge must extend to a target triangle");
    let extracted = one_edge_reduction.extract_solution(&target_solution);
    assert!(one_edge.evaluate(&extracted).0);
}

#[test]
fn test_star_and_disconnected_sources_remain_infeasible() {
    let star = HamiltonianPath::new(SimpleGraph::star(5));
    let star_reduction = ReduceTo::<HamiltonianCircuit<SimpleGraph>>::reduce_to(&star);
    assert!(BruteForce::new()
        .find_witness(star_reduction.target_problem())
        .is_none());

    let disconnected = HamiltonianPath::new(SimpleGraph::new(5, vec![(0, 1), (1, 2), (3, 4)]));
    let disconnected_reduction =
        ReduceTo::<HamiltonianCircuit<SimpleGraph>>::reduce_to(&disconnected);
    assert!(BruteForce::new()
        .find_witness(disconnected_reduction.target_problem())
        .is_none());
}

#[test]
fn test_self_loops_and_parallel_edges_are_copied() {
    let source = HamiltonianPath::new(SimpleGraph::new(3, vec![(0, 0), (0, 1), (0, 1), (1, 2)]));
    let reduction = ReduceTo::<HamiltonianCircuit<SimpleGraph>>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_vertices(), 4);
    assert_eq!(target.num_edges(), 7);
    assert_eq!(
        target
            .graph()
            .edges()
            .into_iter()
            .filter(|&(u, v)| (u == 0 && v == 1) || (u == 1 && v == 0))
            .count(),
        2
    );
    assert!(target.graph().has_edge(0, 0));

    let target_solution = vec![3, 0, 1, 2];
    assert!(target.evaluate(&target_solution).0);
    assert_eq!(reduction.extract_solution(&target_solution), vec![0, 1, 2]);
}
