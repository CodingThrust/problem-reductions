use super::*;
use crate::models::algebraic::ILP;
use crate::rules::ReduceTo;
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;

#[test]
fn test_ruralpostman_to_ilp_closed_loop() {
    // Triangle: 3 vertices, 3 edges, require edge 0, bound 3
    let source = RuralPostman::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![1, 1, 1],
        vec![0],
        3,
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
