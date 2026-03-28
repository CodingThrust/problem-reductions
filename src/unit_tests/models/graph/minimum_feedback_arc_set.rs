use super::*;
use crate::solvers::BruteForce;
use crate::topology::DirectedGraph;
use crate::traits::Problem;

#[test]
fn test_minimum_feedback_arc_set_creation() {
    // 6 vertices, 9 arcs (example from issue)
    let graph = DirectedGraph::new(
        6,
        vec![
            (0, 1),
            (1, 2),
            (2, 0),
            (1, 3),
            (3, 4),
            (4, 1),
            (2, 5),
            (5, 3),
            (3, 0),
        ],
    );
    let problem = MinimumFeedbackArcSet::new(graph, vec![1i32; 9]);
    assert_eq!(problem.num_vertices(), 6);
    assert_eq!(problem.num_arcs(), 9);
    assert_eq!(problem.dims().len(), 9);
    assert!(problem.dims().iter().all(|&d| d == 2));
}

#[test]
fn test_minimum_feedback_arc_set_evaluation_valid() {
    // Simple cycle: 0->1->2->0
    let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]);
    let problem = MinimumFeedbackArcSet::new(graph, vec![1i32; 3]);

    // Remove arc 2->0 (index 2) -> breaks the cycle
    let config = vec![0, 0, 1];
    let result = problem.evaluate(&config);
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 1);

    // Remove arc 0->1 (index 0) -> also breaks the cycle
    let config = vec![1, 0, 0];
    let result = problem.evaluate(&config);
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 1);

    // Remove all arcs -> valid (trivially acyclic), size 3
    let config = vec![1, 1, 1];
    let result = problem.evaluate(&config);
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 3);
}

#[test]
fn test_minimum_feedback_arc_set_evaluation_invalid() {
    // Simple cycle: 0->1->2->0
    let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]);
    let problem = MinimumFeedbackArcSet::new(graph, vec![1i32; 3]);

    // Remove no arcs -> cycle remains -> invalid
    let config = vec![0, 0, 0];
    let result = problem.evaluate(&config);
    assert!(!result.is_valid());
}

#[test]
fn test_minimum_feedback_arc_set_dag() {
    // Already a DAG: 0->1->2
    let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumFeedbackArcSet::new(graph, vec![1i32; 2]);

    // Remove no arcs -> already acyclic
    let config = vec![0, 0];
    let result = problem.evaluate(&config);
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn test_minimum_feedback_arc_set_solver_simple_cycle() {
    // Simple cycle: 0->1->2->0
    let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]);
    let problem = MinimumFeedbackArcSet::new(graph, vec![1i32; 3]);

    let solutions = BruteForce::new().find_all_witnesses(&problem);
    // Minimum FAS has size 1 (remove any one arc)
    for sol in &solutions {
        assert_eq!(sol.iter().sum::<usize>(), 1);
    }
    // There are 3 optimal solutions (one for each arc)
    assert_eq!(solutions.len(), 3);
}

#[test]
fn test_minimum_feedback_arc_set_solver_issue_example() {
    // Example from issue #213: 6 vertices, 9 arcs
    let graph = DirectedGraph::new(
        6,
        vec![
            (0, 1), // a0
            (1, 2), // a1
            (2, 0), // a2
            (1, 3), // a3
            (3, 4), // a4
            (4, 1), // a5
            (2, 5), // a6
            (5, 3), // a7
            (3, 0), // a8
        ],
    );
    let problem = MinimumFeedbackArcSet::new(graph, vec![1i32; 9]);

    let solution = BruteForce::new().find_witness(&problem).unwrap();
    // The optimal FAS has size 2
    let fas_size: usize = solution.iter().sum();
    assert_eq!(fas_size, 2);

    // Verify the solution is valid
    assert!(problem.is_valid_solution(&solution));
}

#[test]
fn test_minimum_feedback_arc_set_weighted() {
    // Cycle: 0->1->2->0 with weights [10, 1, 1]
    // Arc 0 (0->1) costs 10, arcs 1,2 cost 1 each
    // Optimal: remove arc 1 or arc 2 (cost 1), NOT arc 0 (cost 10)
    let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]);
    let problem = MinimumFeedbackArcSet::new(graph, vec![10i32, 1, 1]);

    let solution = BruteForce::new().find_witness(&problem).unwrap();
    let result = problem.evaluate(&solution);
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 1); // should pick a cheap arc

    // Arc 0 should NOT be selected (too expensive)
    assert_eq!(solution[0], 0);
}

#[test]
fn test_minimum_feedback_arc_set_is_valid_solution() {
    let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]);
    let problem = MinimumFeedbackArcSet::new(graph, vec![1i32; 3]);

    // Valid: remove one arc from the cycle
    assert!(problem.is_valid_solution(&[0, 0, 1]));
    // Invalid: keep all arcs (cycle remains)
    assert!(!problem.is_valid_solution(&[0, 0, 0]));
}

#[test]
fn test_minimum_feedback_arc_set_problem_name() {
    assert_eq!(
        <MinimumFeedbackArcSet<i32> as Problem>::NAME,
        "MinimumFeedbackArcSet"
    );
}

#[test]
fn test_minimum_feedback_arc_set_serialization() {
    let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]);
    let problem = MinimumFeedbackArcSet::new(graph, vec![1i32; 3]);
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: MinimumFeedbackArcSet<i32> = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.num_vertices(), 3);
    assert_eq!(deserialized.num_arcs(), 3);
}

#[test]
fn test_minimum_feedback_arc_set_two_disjoint_cycles() {
    // Two disjoint cycles: 0->1->0 and 2->3->2
    let graph = DirectedGraph::new(4, vec![(0, 1), (1, 0), (2, 3), (3, 2)]);
    let problem = MinimumFeedbackArcSet::new(graph, vec![1i32; 4]);

    let solution = BruteForce::new().find_witness(&problem).unwrap();
    // Need to remove at least one arc from each cycle -> size 2
    assert_eq!(solution.iter().sum::<usize>(), 2);
}

#[test]
fn test_minimum_feedback_arc_set_size_getters() {
    let graph = DirectedGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)]);
    let problem = MinimumFeedbackArcSet::new(graph, vec![1i32; 5]);
    assert_eq!(problem.num_vertices(), 5);
    assert_eq!(problem.num_arcs(), 5);
}

#[test]
fn test_minimum_feedback_arc_set_accessors() {
    let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]);
    let mut problem = MinimumFeedbackArcSet::new(graph, vec![1i32; 3]);

    assert!(problem.is_weighted()); // i32 type → true
    assert_eq!(problem.weights(), &[1, 1, 1]);

    problem.set_weights(vec![2, 3, 4]);
    assert_eq!(problem.weights(), &[2, 3, 4]);
}
