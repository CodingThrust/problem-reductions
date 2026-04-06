use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Min;

fn issue_problem() -> MinimumWeightAndOrGraph {
    // 7 vertices: AND at 0, OR at 1 and 2, leaves 3-6
    // Arcs: (0,1,1), (0,2,2), (1,3,3), (1,4,1), (2,5,4), (2,6,2)
    MinimumWeightAndOrGraph::new(
        7,
        vec![(0, 1), (0, 2), (1, 3), (1, 4), (2, 5), (2, 6)],
        0,
        vec![Some(true), Some(false), Some(false), None, None, None, None],
        vec![1, 2, 3, 1, 4, 2],
    )
}

#[test]
fn test_minimum_weight_and_or_graph_creation() {
    let problem = issue_problem();

    assert_eq!(problem.num_vertices(), 7);
    assert_eq!(problem.num_arcs(), 6);
    assert_eq!(problem.source(), 0);
    assert_eq!(problem.gate_types().len(), 7);
    assert_eq!(problem.arc_weights().len(), 6);
    assert_eq!(problem.num_variables(), 6);
    assert_eq!(problem.dims(), vec![2; 6]);
    assert_eq!(
        <MinimumWeightAndOrGraph as Problem>::NAME,
        "MinimumWeightAndOrGraph"
    );
    assert!(<MinimumWeightAndOrGraph as Problem>::variant().is_empty());
}

#[test]
fn test_minimum_weight_and_or_graph_evaluate_optimal() {
    let problem = issue_problem();

    // Config [1,1,0,1,0,1]: arcs 0,1,3,5 selected
    // Weights: 1+2+1+2 = 6
    assert_eq!(problem.evaluate(&[1, 1, 0, 1, 0, 1]), Min(Some(6)));
}

#[test]
fn test_minimum_weight_and_or_graph_evaluate_all_arcs() {
    let problem = issue_problem();

    // Config [1,1,1,1,1,1]: all arcs selected, also valid (AND satisfied, OR satisfied)
    // Weights: 1+2+3+1+4+2 = 13
    assert_eq!(problem.evaluate(&[1, 1, 1, 1, 1, 1]), Min(Some(13)));
}

#[test]
fn test_minimum_weight_and_or_graph_and_violated() {
    let problem = issue_problem();

    // Config [1,0,0,1,0,1]: arc 1 (0->2) not selected, but source is AND
    // AND at source requires both arcs 0 and 1
    assert_eq!(problem.evaluate(&[1, 0, 0, 1, 0, 1]), Min(None));
}

#[test]
fn test_minimum_weight_and_or_graph_or_violated() {
    let problem = issue_problem();

    // Config [1,1,0,0,0,1]: arcs 0,1,5 selected
    // OR at v1 has no selected outgoing arcs (arcs 2,3 both 0)
    assert_eq!(problem.evaluate(&[1, 1, 0, 0, 0, 1]), Min(None));
}

#[test]
fn test_minimum_weight_and_or_graph_dangling_arc() {
    let problem = issue_problem();

    // Config [0,0,1,0,0,0]: only arc 2 (1->3) selected
    // Arc 2 goes from vertex 1, but vertex 1 is not solved (no arc leads to it from source)
    // Source AND requires arcs 0,1 — they are missing, so it's invalid at the source check
    assert_eq!(problem.evaluate(&[0, 0, 1, 0, 0, 0]), Min(None));
}

#[test]
fn test_minimum_weight_and_or_graph_empty_config() {
    let problem = issue_problem();

    // No arcs selected: AND at source requires all outgoing arcs
    assert_eq!(problem.evaluate(&[0, 0, 0, 0, 0, 0]), Min(None));
}

#[test]
fn test_minimum_weight_and_or_graph_wrong_config_length() {
    let problem = issue_problem();

    assert_eq!(problem.evaluate(&[1, 1, 0]), Min(None));
}

#[test]
fn test_minimum_weight_and_or_graph_solver() {
    let problem = issue_problem();
    let solver = BruteForce::new();

    use crate::solvers::Solver;
    let optimal = solver.solve(&problem);
    assert_eq!(optimal, Min(Some(6)));

    let witness = solver.find_witness(&problem);
    assert!(witness.is_some());
    let w = witness.unwrap();
    assert_eq!(problem.evaluate(&w), Min(Some(6)));
}

#[test]
fn test_minimum_weight_and_or_graph_serialization() {
    let problem = issue_problem();
    let json = serde_json::to_string(&problem).unwrap();
    let round_trip: MinimumWeightAndOrGraph = serde_json::from_str(&json).unwrap();

    assert_eq!(round_trip.num_vertices(), 7);
    assert_eq!(round_trip.num_arcs(), 6);
    assert_eq!(round_trip.source(), 0);
    assert_eq!(round_trip.evaluate(&[1, 1, 0, 1, 0, 1]), Min(Some(6)));
}

#[test]
fn test_minimum_weight_and_or_graph_paper_example() {
    let problem = issue_problem();

    // Verify the paper example: optimal config [1,1,0,1,0,1] with value 6
    assert_eq!(problem.evaluate(&[1, 1, 0, 1, 0, 1]), Min(Some(6)));

    // Confirm optimality via brute force
    let solver = BruteForce::new();
    use crate::solvers::Solver;
    let optimal = solver.solve(&problem);
    assert_eq!(optimal, Min(Some(6)));

    // Verify there is exactly one optimal witness
    let all = solver.find_all_witnesses(&problem);
    let optimal_witnesses: Vec<_> = all
        .into_iter()
        .filter(|w| problem.evaluate(w) == Min(Some(6)))
        .collect();
    assert_eq!(optimal_witnesses.len(), 1);
    assert_eq!(optimal_witnesses[0], vec![1, 1, 0, 1, 0, 1]);
}
