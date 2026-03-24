use super::*;
use crate::models::algebraic::ILP;
use crate::rules::ReduceTo;
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::DirectedGraph;
use crate::traits::Problem;

#[test]
fn test_integralflowhomologousarcs_to_ilp_closed_loop() {
    // 4 vertices, arcs (0,1),(0,2),(1,3),(2,3), caps all 2, req 2, pair (0,1)
    let source = IntegralFlowHomologousArcs::new(
        DirectedGraph::new(4, vec![(0, 1), (0, 2), (1, 3), (2, 3)]),
        vec![2, 2, 2, 2],
        0,
        3,
        2,
        vec![(0, 1)],
    );
    // Verify source is satisfiable via brute force
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
