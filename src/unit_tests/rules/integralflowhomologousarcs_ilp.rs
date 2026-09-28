use super::*;
use crate::models::algebraic::Bounded;
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
        .solve(&source)
        .unwrap()
        .expect("source instance should be satisfiable");
    assert!(source.evaluate(&direct).unwrap());

    let reduction =
        ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).expect("reduction should succeed");
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();

    assert!(source.evaluate(&extracted).unwrap());
}

#[test]
fn test_integralflowhomologousarcs_to_ilp_bf_vs_ilp() {
    let source = IntegralFlowHomologousArcs::new(
        DirectedGraph::new(4, vec![(0, 1), (0, 2), (1, 3), (2, 3)]),
        vec![2, 2, 2, 2],
        0,
        3,
        2,
        vec![(0, 1)],
    );
    let reduction =
        ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).expect("reduction should succeed");
    crate::rules::test_helpers::assert_bf_vs_ilp(&source, &reduction);
}

#[test]
fn test_flow_requirement_normalization_preserves_feasibility() {
    for requirement in [i64::MIN, 0, 1, 2, i64::MAX] {
        let source = IntegralFlowHomologousArcs::new(
            DirectedGraph::new(2, vec![(0, 1)]),
            vec![1],
            0,
            1,
            requirement,
            vec![],
        );
        let reduction = ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).unwrap();
        assert!(reduction.target_problem().max_constraint_magnitude_bits() <= 2);
        match ILPSolver::new().solve(reduction.target_problem()) {
            Ok(solution) => {
                assert!(requirement <= 1);
                let recovered = reduction.extract_solution(&solution).unwrap();
                assert!(source.evaluate(&recovered).unwrap());
            }
            Err(error) => {
                assert!(requirement > 1);
                assert_eq!(error, crate::solvers::ILPSolveError::Infeasible);
            }
        }
    }
}
