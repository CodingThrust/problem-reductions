use super::*;
use crate::solvers::BruteForce;
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

fn satisfying_config() -> Vec<usize> {
    vec![1, 0, 1, 0, 0, 0]
}

#[test]
fn test_integral_flow_bundles_creation_and_getters() {
    let problem = yes_instance();
    assert_eq!(problem.num_vertices(), 4);
    assert_eq!(problem.num_arcs(), 6);
    assert_eq!(problem.num_bundles(), 3);
    assert_eq!(problem.source(), 0);
    assert_eq!(problem.sink(), 3);
    assert_eq!(problem.requirement(), 1);
    assert_eq!(problem.bundle_capacities(), &[1, 1, 1]);
    assert_eq!(problem.graph().arcs().len(), 6);
}

#[test]
fn test_integral_flow_bundles_dims_use_tight_arc_bounds() {
    let problem = yes_instance();
    assert_eq!(problem.dims(), vec![2, 2, 2, 2, 2, 2]);
}

#[test]
fn test_integral_flow_bundles_evaluate_yes_and_no_examples() {
    let yes = yes_instance();
    let no = no_instance();
    let config = satisfying_config();
    assert!(yes.evaluate(&config));
    assert!(!no.evaluate(&config));
    assert!(yes.is_valid_solution(&config));
}

#[test]
fn test_integral_flow_bundles_rejects_bad_bundle_sum_or_conservation() {
    let problem = yes_instance();

    let mut bundle_violation = satisfying_config();
    bundle_violation[1] = 1;
    assert!(!problem.evaluate(&bundle_violation));

    let conservation_violation = vec![1, 0, 0, 0, 0, 0];
    assert!(!problem.evaluate(&conservation_violation));
}

#[test]
fn test_integral_flow_bundles_solver_and_paper_example() {
    let problem = yes_instance();
    let solver = BruteForce::new();
    let all = solver.find_all_satisfying(&problem);
    assert!(!all.is_empty());
    assert!(all.contains(&satisfying_config()));
    assert!(problem.evaluate(&satisfying_config()));
}

#[test]
fn test_integral_flow_bundles_serialization() {
    let problem = yes_instance();
    let json = serde_json::to_string(&problem).unwrap();
    let roundtrip: IntegralFlowBundles = serde_json::from_str(&json).unwrap();
    assert_eq!(roundtrip.num_vertices(), 4);
    assert_eq!(roundtrip.num_arcs(), 6);
    assert_eq!(roundtrip.num_bundles(), 3);
    assert_eq!(roundtrip.requirement(), 1);
}

#[test]
fn test_integral_flow_bundles_problem_name() {
    assert_eq!(<IntegralFlowBundles as Problem>::NAME, "IntegralFlowBundles");
}
