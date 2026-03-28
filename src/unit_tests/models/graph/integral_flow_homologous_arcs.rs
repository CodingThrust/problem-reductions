use super::*;
use crate::solvers::BruteForce;
use crate::topology::DirectedGraph;
use crate::traits::Problem;

fn yes_instance() -> IntegralFlowHomologousArcs {
    let graph = DirectedGraph::new(
        6,
        vec![
            (0, 1),
            (0, 2),
            (1, 3),
            (2, 3),
            (1, 4),
            (2, 4),
            (3, 5),
            (4, 5),
        ],
    );
    IntegralFlowHomologousArcs::new(graph, vec![1; 8], 0, 5, 2, vec![(2, 5), (4, 3)])
}

fn no_instance() -> IntegralFlowHomologousArcs {
    let graph = DirectedGraph::new(4, vec![(0, 1), (0, 2), (1, 2), (2, 3)]);
    IntegralFlowHomologousArcs::new(graph, vec![1; 4], 0, 3, 1, vec![(0, 1)])
}

#[test]
fn test_integral_flow_homologous_arcs_creation() {
    let problem = yes_instance();
    assert_eq!(problem.num_vertices(), 6);
    assert_eq!(problem.num_arcs(), 8);
    assert_eq!(problem.source(), 0);
    assert_eq!(problem.sink(), 5);
    assert_eq!(problem.requirement(), 2);
    assert_eq!(problem.max_capacity(), 1);
    assert_eq!(problem.homologous_pairs(), &[(2, 5), (4, 3)]);
    assert_eq!(problem.dims(), vec![2; 8]);
}

#[test]
fn test_integral_flow_homologous_arcs_evaluate_yes_instance() {
    let problem = yes_instance();
    let config = vec![1, 1, 1, 0, 0, 1, 1, 1];
    assert!(problem.evaluate(&config));
}

#[test]
fn test_integral_flow_homologous_arcs_evaluate_no_instance() {
    let problem = no_instance();
    assert!(!problem.evaluate(&[0, 0, 0, 0]));
}

#[test]
fn test_integral_flow_homologous_arcs_rejects_homologous_violation() {
    let problem = yes_instance();
    let config = vec![1, 1, 1, 0, 0, 0, 1, 1];
    assert!(!problem.evaluate(&config));
}

#[test]
fn test_integral_flow_homologous_arcs_rejects_capacity_violation() {
    let problem = yes_instance();
    let config = vec![2, 0, 0, 0, 0, 0, 0, 0];
    assert!(!problem.evaluate(&config));
}

#[test]
fn test_integral_flow_homologous_arcs_rejects_conservation_violation() {
    let problem = yes_instance();
    let config = vec![1, 0, 0, 0, 0, 0, 0, 0];
    assert!(!problem.evaluate(&config));
}

#[test]
fn test_integral_flow_homologous_arcs_wrong_config_length_is_invalid() {
    let problem = yes_instance();
    assert!(!problem.evaluate(&[0; 7]));
    assert!(!problem.evaluate(&[0; 9]));
}

#[test]
fn test_integral_flow_homologous_arcs_solver_yes() {
    let problem = yes_instance();
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem);
    assert!(solution.is_some());
    assert!(problem.evaluate(&solution.unwrap()));
}

#[test]
fn test_integral_flow_homologous_arcs_solver_no() {
    let problem = no_instance();
    let solver = BruteForce::new();
    assert!(solver.find_witness(&problem).is_none());
}

#[test]
fn test_integral_flow_homologous_arcs_serialization() {
    let problem = yes_instance();
    let json = serde_json::to_string(&problem).unwrap();
    let restored: IntegralFlowHomologousArcs = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.num_vertices(), 6);
    assert_eq!(restored.num_arcs(), 8);
    assert_eq!(restored.requirement(), 2);
    assert_eq!(restored.homologous_pairs(), &[(2, 5), (4, 3)]);
}

#[test]
fn test_integral_flow_homologous_arcs_problem_name() {
    assert_eq!(
        <IntegralFlowHomologousArcs as Problem>::NAME,
        "IntegralFlowHomologousArcs"
    );
}

#[test]
fn test_integral_flow_homologous_arcs_non_unit_capacity() {
    // s=0 -> 1 -> 2=t, with capacities [3, 3], homologous pair (0,1) so both arcs carry
    // equal flow. R=2 is satisfiable: f=[2,2].
    let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = IntegralFlowHomologousArcs::new(graph, vec![3, 3], 0, 2, 2, vec![(0, 1)]);
    assert_eq!(problem.dims(), vec![4, 4]);
    assert_eq!(problem.max_capacity(), 3);
    assert!(problem.evaluate(&[2, 2]));
    assert!(problem.evaluate(&[3, 3]));
    assert!(!problem.evaluate(&[2, 3])); // homologous violation
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem);
    assert_eq!(solutions.len(), 2); // [2,2] and [3,3]
}

#[test]
fn test_integral_flow_homologous_arcs_paper_example() {
    let problem = yes_instance();
    let solver = BruteForce::new();
    let config = vec![1, 1, 1, 0, 0, 1, 1, 1];

    assert!(problem.evaluate(&config));

    let solutions = solver.find_all_witnesses(&problem);
    assert!(!solutions.is_empty());
    assert!(solutions
        .iter()
        .all(|solution| problem.evaluate(solution).0));
}
