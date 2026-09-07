use super::*;
use crate::models::algebraic::{Comparison, ObjectiveSense, ILP};
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::DirectedGraph;
use crate::traits::Problem;

fn yes_instance() -> IntegralFlowBundles {
    IntegralFlowBundles::new(
        DirectedGraph::new(4, vec![(0, 1), (0, 2), (1, 3), (2, 3), (1, 2), (2, 1)]),
        0,
        3,
        vec![vec![0, 1], vec![2, 5], vec![3, 4]],
        vec![1, 1, 1],
        1,
    )
}

fn no_instance() -> IntegralFlowBundles {
    IntegralFlowBundles::new(
        DirectedGraph::new(4, vec![(0, 1), (0, 2), (1, 3), (2, 3), (1, 2), (2, 1)]),
        0,
        3,
        vec![vec![0, 1], vec![2, 5], vec![3, 4]],
        vec![1, 1, 1],
        2,
    )
}

fn satisfying_config() -> Vec<i64> {
    vec![1, 0, 1, 0, 0, 0]
}

#[test]
fn test_integral_flow_bundles_to_ilp_structure() {
    let problem = yes_instance();
    let reduction: ReductionIFBToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars(), 6);
    assert_eq!(ilp.constraints().len(), 6);
    assert_eq!(ilp.sense(), ObjectiveSense::Minimize);
    assert!(ilp.objective().is_empty());
    assert_eq!(
        ilp.constraints()
            .iter()
            .filter(|constraint| constraint.comparison() == Comparison::Le)
            .count(),
        3
    );
    assert_eq!(
        ilp.constraints()
            .iter()
            .filter(|constraint| constraint.comparison() == Comparison::Eq)
            .count(),
        2
    );
    assert_eq!(
        ilp.constraints()
            .iter()
            .filter(|constraint| constraint.comparison() == Comparison::Ge)
            .count(),
        1
    );
}

#[test]
fn test_integral_flow_bundles_to_ilp_closed_loop() {
    let problem = yes_instance();
    let direct = BruteForce::new()
        .solve(&problem)
        .unwrap()
        .expect("source instance should be satisfiable");
    assert!(problem.evaluate(&direct).unwrap());

    let reduction: ReductionIFBToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();

    assert!(problem.evaluate(&extracted).unwrap());
}

#[test]
fn test_integral_flow_bundles_to_ilp_extract_solution_is_identity() {
    let problem = yes_instance();
    let reduction: ReductionIFBToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");
    assert_eq!(
        reduction.extract_solution(&satisfying_config()).unwrap(),
        vec![1, 0, 1, 0, 0, 0]
    );
}

#[test]
fn test_integral_flow_bundles_to_ilp_unsat_instance_is_infeasible() {
    let problem = no_instance();
    let reduction: ReductionIFBToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");
    assert!(ILPSolver::new().solve(reduction.target_problem()).is_err());
}

#[test]
fn test_integral_flow_bundles_to_ilp_sink_requirement_constraint() {
    let problem = yes_instance();
    let reduction: ReductionIFBToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    let sink_constraint = ilp
        .constraints()
        .iter()
        .find(|constraint| constraint.comparison() == Comparison::Ge)
        .expect("expected one sink inflow lower bound");
    assert_eq!(sink_constraint.rhs(), 1);
    assert_eq!(sink_constraint.terms(), vec![(2, 1), (3, 1)]);
}

#[test]
fn test_integralflowbundles_to_ilp_bf_vs_ilp() {
    let problem = yes_instance();
    let reduction: ReductionIFBToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");
    crate::rules::test_helpers::assert_bf_vs_ilp(&problem, &reduction);
}
