use super::*;
use crate::models::algebraic::ILP;
use crate::models::graph::BicliqueCover;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::rules::ReduceTo;
use crate::topology::BipartiteGraph;
use crate::traits::Problem;

fn small_instance() -> BicliqueCover {
    // L={0,1}, R={0,1,2}, edges: (0,0),(0,1),(1,1),(1,2), k=2
    BicliqueCover::new(
        BipartiteGraph::new(2, 3, vec![(0, 0), (0, 1), (1, 1), (1, 2)]),
        2,
    )
}

#[test]
fn test_bicliquecover_to_ilp_closed_loop() {
    let source = small_instance();
    let reduction: ReductionBicliqueCoverToILP = ReduceTo::<ILP<bool>>::reduce_to(&source);
    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "BicliqueCover -> ILP round trip",
    );
}

#[test]
fn test_reduction_shape() {
    let source = small_instance();
    let reduction: ReductionBicliqueCoverToILP = ReduceTo::<ILP<bool>>::reduce_to(&source);
    let ilp = reduction.target_problem();
    // n=5, k=2, left=2, right=3
    // x vars: 5*2=10, z vars: 2*3*2=12, total=22
    assert_eq!(ilp.num_vars, 22);
}

#[test]
fn test_ilp_solution_is_valid_cover() {
    let source = small_instance();
    let reduction: ReductionBicliqueCoverToILP = ReduceTo::<ILP<bool>>::reduce_to(&source);
    let ilp = reduction.target_problem();
    let solver = crate::solvers::ILPSolver::new();
    let ilp_sol = solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_sol);
    let value = source.evaluate(&extracted);
    assert!(
        value.0.is_some(),
        "extracted solution should be a valid cover"
    );
}

#[test]
fn test_single_edge() {
    // Single edge needs 1 biclique
    let source = BicliqueCover::new(BipartiteGraph::new(1, 1, vec![(0, 0)]), 1);
    let reduction: ReductionBicliqueCoverToILP = ReduceTo::<ILP<bool>>::reduce_to(&source);
    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "single edge biclique cover",
    );
}
