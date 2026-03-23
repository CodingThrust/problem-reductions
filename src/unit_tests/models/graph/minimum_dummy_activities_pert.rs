use super::*;
use crate::solvers::BruteForce;
use crate::topology::DirectedGraph;
use crate::traits::Problem;
use crate::types::Min;

fn issue_graph() -> DirectedGraph {
    DirectedGraph::new(6, vec![(0, 2), (0, 3), (1, 3), (1, 4), (2, 5)])
}

fn issue_problem() -> MinimumDummyActivitiesPert {
    MinimumDummyActivitiesPert::new(issue_graph())
}

fn config_for_merges(
    problem: &MinimumDummyActivitiesPert,
    merges: &[(usize, usize)],
) -> Vec<usize> {
    let mut config = vec![0; problem.num_arcs()];
    let arcs = problem.graph().arcs();
    for &(u, v) in merges {
        let index = arcs
            .iter()
            .position(|&(a, b)| a == u && b == v)
            .expect("merge arc must exist in issue graph");
        config[index] = 1;
    }
    config
}

#[test]
fn test_minimum_dummy_activities_pert_creation() {
    let problem = issue_problem();
    assert_eq!(problem.num_vertices(), 6);
    assert_eq!(problem.num_arcs(), 5);
    assert_eq!(problem.dims(), vec![2; 5]);
}

#[test]
fn test_minimum_dummy_activities_pert_rejects_cyclic_input() {
    let err =
        MinimumDummyActivitiesPert::try_new(DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]))
            .unwrap_err();
    assert!(err.contains("DAG"));
}

#[test]
fn test_minimum_dummy_activities_pert_issue_example() {
    let problem = issue_problem();
    let config = config_for_merges(&problem, &[(0, 2), (1, 4), (2, 5)]);
    assert_eq!(problem.evaluate(&config), Min(Some(2)));
    assert!(problem.is_valid_solution(&config));
}

#[test]
fn test_minimum_dummy_activities_pert_rejects_spurious_reachability() {
    let problem = issue_problem();
    let config = config_for_merges(&problem, &[(0, 3), (1, 3)]);
    assert_eq!(problem.evaluate(&config), Min(None));
    assert!(!problem.is_valid_solution(&config));
}

#[test]
fn test_minimum_dummy_activities_pert_solver_finds_optimum_two() {
    let problem = issue_problem();
    let solution = BruteForce::new().find_witness(&problem).unwrap();
    assert_eq!(problem.evaluate(&solution), Min(Some(2)));
}

#[test]
fn test_minimum_dummy_activities_pert_serialization_roundtrip() {
    let problem = issue_problem();
    let json = serde_json::to_string(&problem).unwrap();
    let restored: MinimumDummyActivitiesPert = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.graph(), problem.graph());
}

#[test]
fn test_minimum_dummy_activities_pert_transitive_arc_zero_dummies() {
    // DAG with transitive arc: 0→1, 1→2, 0→2.
    // Merging 0+=1- and 1+=2- makes the 0→2 reachability transitively
    // satisfied, so the optimal dummy count is 0.
    let dag = DirectedGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let problem = MinimumDummyActivitiesPert::new(dag);
    let solution = BruteForce::new().find_witness(&problem).unwrap();
    assert_eq!(problem.evaluate(&solution), Min(Some(0)));
}

#[test]
fn test_minimum_dummy_activities_pert_paper_example() {
    let problem = issue_problem();
    let config = config_for_merges(&problem, &[(0, 2), (1, 4), (2, 5)]);
    assert_eq!(problem.evaluate(&config), Min(Some(2)));
    let solution = BruteForce::new().find_witness(&problem).unwrap();
    assert_eq!(problem.evaluate(&solution), Min(Some(2)));
}
