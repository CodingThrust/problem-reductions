use super::*;
use crate::models::algebraic::ILP;
use crate::rules::ReduceTo;
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::MixedGraph;
use crate::traits::Problem;

#[test]
fn test_mixedchinesepostman_to_ilp_closed_loop() {
    // 3 vertices, 1 directed arc, 2 undirected edges, bound 4
    let source = MixedChinesePostman::new(
        MixedGraph::new(3, vec![(0, 1)], vec![(1, 2), (2, 0)]),
        vec![1],
        vec![1, 1],
        4,
    );
    let direct = BruteForce::new()
        .find_witness(&source)
        .expect("source instance should be satisfiable");
    assert!(source.evaluate(&direct));

    let reduction = ReduceTo::<ILP<i32>>::reduce_to(&source);
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert!(source.evaluate(&extracted));
}
