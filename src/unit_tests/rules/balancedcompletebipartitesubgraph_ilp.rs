use super::*;
use crate::models::algebraic::ILP;
use crate::models::graph::BalancedCompleteBipartiteSubgraph;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::rules::ReduceTo;
use crate::solvers::{BruteForce, ILPSolver, Solver};
use crate::topology::BipartiteGraph;
use crate::traits::Problem;

fn small_instance() -> BalancedCompleteBipartiteSubgraph {
    // L={0,1,2}, R={0,1,2}
    // Edges: (0,0),(0,1),(1,0),(1,1),(2,1),(2,2)
    // K_{2,2} subgraph: L={0,1}, R={0,1}
    BalancedCompleteBipartiteSubgraph::new(
        BipartiteGraph::new(3, 3, vec![(0, 0), (0, 1), (1, 0), (1, 1), (2, 1), (2, 2)]),
        2,
    )
}

#[test]
fn test_balancedcompletebipartitesubgraph_to_ilp_closed_loop() {
    let source = small_instance();
    let reduction: ReductionBCBSToILP = ReduceTo::<ILP<bool>>::reduce_to(&source);
    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "BCBS -> ILP round trip",
    );
}

#[test]
fn test_reduction_shape() {
    let source = small_instance();
    let reduction: ReductionBCBSToILP = ReduceTo::<ILP<bool>>::reduce_to(&source);
    let ilp = reduction.target_problem();
    // 6 variables (3 left + 3 right)
    assert_eq!(ilp.num_vars, 6);
}

#[test]
fn test_infeasible_instance() {
    // No K_{3,3}: not all edges present
    let source = BalancedCompleteBipartiteSubgraph::new(
        BipartiteGraph::new(3, 3, vec![(0, 0), (0, 1), (1, 0), (1, 1)]),
        3,
    );
    let reduction: ReductionBCBSToILP = ReduceTo::<ILP<bool>>::reduce_to(&source);
    let ilp = reduction.target_problem();
    let solver = crate::solvers::ILPSolver::new();
    assert!(solver.solve(ilp).is_none());
}

#[test]
fn test_extract_solution_identity() {
    let source = small_instance();
    let reduction: ReductionBCBSToILP = ReduceTo::<ILP<bool>>::reduce_to(&source);
    let target_sol = vec![1, 1, 0, 1, 1, 0];
    let extracted = reduction.extract_solution(&target_sol);
    assert_eq!(extracted, vec![1, 1, 0, 1, 1, 0]);
    assert!(source.evaluate(&extracted).0);
}

#[test]
fn test_balancedcompletebipartitesubgraph_to_ilp_bf_vs_ilp() {
    let source = small_instance();
    let reduction: ReductionBCBSToILP = ReduceTo::<ILP<bool>>::reduce_to(&source);
    let bf_value = BruteForce::new().solve(&source);
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(source.evaluate(&extracted), bf_value);
}
