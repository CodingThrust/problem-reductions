use super::*;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::size_bound::{BoundVector, SizeBound};
use crate::size_map::SizeMap;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::{One, ProblemSize};
use num_bigint::BigUint;

#[test]
fn test_maximumindependentset_to_maximumclique_closed_loop() {
    // Path graph: 0-1-2-3-4
    let source = MaximumIndependentSet::new(
        SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]),
        vec![1i32; 5],
    );
    let reduction = ReduceTo::<MaximumClique<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    // Complement of path graph should have n*(n-1)/2 - m = 10 - 4 = 6 edges
    assert_eq!(target.num_vertices(), 5);
    assert_eq!(target.num_edges(), 6);

    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "MaximumIndependentSet->MaximumClique closed loop",
    );
}

#[test]
fn exact_size_map_matches_constructed_complement() {
    let source = MaximumIndependentSet::new(
        SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]),
        vec![1i32; 5],
    );
    let size_map = SizeMap::new(
        "MaximumIndependentSet -> MaximumClique",
        [
            ("num_vertices", crate::expr::Expr::parse("num_vertices")),
            (
                "num_edges",
                crate::expr::Expr::parse("num_vertices * (num_vertices - 1) / 2 - num_edges"),
            ),
        ],
    )
    .unwrap();

    let predicted = size_map
        .evaluate(&ProblemSize::new(vec![
            ("num_vertices", source.num_vertices()),
            ("num_edges", source.num_edges()),
        ]))
        .unwrap();
    let reduction = ReduceTo::<MaximumClique<SimpleGraph, i32>>::reduce_to(&source);
    let constructed = ProblemSize::new(vec![
        ("num_vertices", reduction.target_problem().num_vertices()),
        ("num_edges", reduction.target_problem().num_edges()),
    ]);

    assert_eq!(
        predicted,
        ProblemSize::new(vec![("num_vertices", 5), ("num_edges", 6)])
    );
    assert_eq!(predicted, constructed);
}

#[test]
fn certified_size_bound_contains_constructed_complement() {
    let source = MaximumIndependentSet::new(
        SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]),
        vec![1i32; 5],
    );
    let size_bound = SizeBound::new(
        "MaximumIndependentSet -> MaximumClique",
        [
            ("num_vertices", crate::expr::Expr::parse("num_vertices")),
            ("num_edges", crate::expr::Expr::parse("num_vertices ^ 2")),
        ],
    )
    .unwrap();

    let predicted = size_bound
        .evaluate(&BoundVector::new([
            ("num_vertices", source.num_vertices()),
            ("num_edges", source.num_edges()),
        ]))
        .unwrap();
    let reduction = ReduceTo::<MaximumClique<SimpleGraph, i32>>::reduce_to(&source);

    assert_eq!(predicted.get("num_vertices"), Some(&BigUint::from(5u8)));
    assert_eq!(predicted.get("num_edges"), Some(&BigUint::from(25u8)));
    assert!(
        BigUint::from(reduction.target_problem().num_vertices())
            <= *predicted.get("num_vertices").unwrap()
    );
    assert!(
        BigUint::from(reduction.target_problem().num_edges())
            <= *predicted.get("num_edges").unwrap()
    );
}

#[test]
fn test_maximumindependentset_to_maximumclique_weighted() {
    // Triangle with weights
    let source = MaximumIndependentSet::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![10, 20, 30],
    );
    let reduction = ReduceTo::<MaximumClique<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    // Complement of K3 has 0 edges (empty graph)
    assert_eq!(target.num_vertices(), 3);
    assert_eq!(target.num_edges(), 0);
    assert_eq!(target.weights().to_vec(), vec![10, 20, 30]);

    // In empty graph, max clique is a single vertex. Best is vertex 2 (weight 30).
    let solver = BruteForce::new();
    let best = solver.find_all_witnesses(target);
    for sol in &best {
        let extracted = reduction.extract_solution(sol).unwrap();
        let metric = source.evaluate(&extracted);
        assert!(metric.is_valid());
    }
}

#[test]
fn test_maximumindependentset_to_maximumclique_empty_graph() {
    // Empty graph (no edges) - complement is complete graph
    let source = MaximumIndependentSet::new(SimpleGraph::new(4, vec![]), vec![1i32; 4]);
    let reduction = ReduceTo::<MaximumClique<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    // Complement of empty graph is K4 with 6 edges
    assert_eq!(target.num_vertices(), 4);
    assert_eq!(target.num_edges(), 6);

    // All 4 vertices form a clique in complement = all 4 are independent set in source
    let solver = BruteForce::new();
    let best_target = solver.find_all_witnesses(target);
    assert!(best_target.iter().all(|s| s.iter().sum::<usize>() == 4));
}

#[test]
fn test_maximumindependentset_to_maximumclique_one_weights_closed_loop() {
    // Unit-weight closed loop: <SimpleGraph, One> endpoint stays on One all the way.
    let source = MaximumIndependentSet::new(
        SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]),
        vec![One; 5],
    );
    let reduction = ReduceTo::<MaximumClique<SimpleGraph, One>>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_vertices(), 5);
    assert_eq!(target.num_edges(), 6);

    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "MaximumIndependentSet<One>->MaximumClique<One> closed loop",
    );
}

#[test]
fn test_maximumindependentset_to_maximumclique_complete_graph() {
    // Complete graph K4 - complement is empty graph
    let source = MaximumIndependentSet::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]),
        vec![1i32; 4],
    );
    let reduction = ReduceTo::<MaximumClique<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_edges(), 0);

    // Max clique in empty graph is single vertex, max IS in K4 is also single vertex
    let solver = BruteForce::new();
    let best = solver.find_all_witnesses(target);
    assert!(best.iter().all(|s| s.iter().sum::<usize>() == 1));
}
