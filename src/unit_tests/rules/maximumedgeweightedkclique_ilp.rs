use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::models::graph::MaximumEdgeWeightedKClique;
use crate::rules::test_helpers::{
    assert_bf_vs_ilp, assert_optimization_round_trip_from_optimization_target,
};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Max;

fn issue_instance() -> MaximumEdgeWeightedKClique<i32> {
    // 4 vertices, edges (0,1),(0,2),(1,2),(0,3),(1,3) with weights [5,4,-1,1,0], k=3.
    // Optimum induced weight is 5 + 4 + (-1) = 8 on clique {0, 1, 2}.
    MaximumEdgeWeightedKClique::<i32>::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 2), (0, 3), (1, 3)]),
        vec![5, 4, -1, 1, 0],
        3,
    )
}

#[test]
fn test_maximumedgeweightedkclique_to_ilp_closed_loop() {
    let source = issue_instance();
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source);
    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "MaximumEdgeWeightedKClique -> ILP closed loop",
    );
}

#[test]
fn test_maximumedgeweightedkclique_to_ilp_structure() {
    let source = issue_instance();
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source);
    let ilp = reduction.target_problem();

    // 4 vertex variables + 5 edge variables = 9.
    assert_eq!(ilp.num_vars, 9);
    assert_eq!(ilp.sense, ObjectiveSense::Maximize);
    // Objective is on the edge variables (indices 4..9).
    let weights: Vec<f64> = ilp.objective.iter().map(|(_, w)| *w).collect();
    assert_eq!(weights, vec![5.0, 4.0, -1.0, 1.0, 0.0]);
}

#[test]
fn test_maximumedgeweightedkclique_to_ilp_extract_solution_identity() {
    let source = issue_instance();
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source);
    let target_solution = vec![1, 1, 1, 0, 1, 1, 1, 0, 0];
    let extracted = reduction.extract_solution(&target_solution);
    assert_eq!(extracted, vec![1, 1, 1, 0]);
    assert_eq!(source.evaluate(&extracted), Max(Some(8)));
}

#[test]
fn test_maximumedgeweightedkclique_to_ilp_bf_vs_ilp() {
    let source = issue_instance();
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source);
    assert_bf_vs_ilp(&source, &reduction);
}

#[test]
fn test_maximumedgeweightedkclique_to_ilp_negative_weight_excluded_via_extra_constraint() {
    // Triangle (0,1,2) with edges all weight = -1 and k=3. Only one feasible
    // size-3 clique exists, and its weight is -3. The McCormick lower bound
    // y >= x_u + x_v - 1 ensures negative-weight y's are forced to 1 when
    // both endpoints are selected.
    let source = MaximumEdgeWeightedKClique::<i32>::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![-1, -1, -1],
        3,
    );
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source);
    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "MaximumEdgeWeightedKClique -> ILP negative-weight triangle",
    );
    assert_bf_vs_ilp(&source, &reduction);
}
