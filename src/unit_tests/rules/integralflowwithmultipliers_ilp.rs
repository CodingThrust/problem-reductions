use super::*;
use crate::models::algebraic::ILP;
use crate::rules::ReduceTo;
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::DirectedGraph;
use crate::traits::Problem;

#[test]
fn test_integralflowwithmultipliers_to_ilp_closed_loop() {
    // 4 vertices, arcs (0,1),(0,2),(1,3),(2,3), multipliers all 1, caps all 2, req 2
    let source = IntegralFlowWithMultipliers::new(
        DirectedGraph::new(4, vec![(0, 1), (0, 2), (1, 3), (2, 3)]),
        0,
        3,
        vec![1, 1, 1, 1],
        vec![2, 2, 2, 2],
        2,
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
