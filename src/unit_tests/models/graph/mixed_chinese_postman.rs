use super::*;
use crate::solvers::BruteForce;
use crate::topology::MixedGraph;
use crate::traits::Problem;
use crate::types::Min;

fn sample_instance() -> MixedChinesePostman<i32> {
    MixedChinesePostman::new(
        MixedGraph::new(
            5,
            vec![(0, 1), (1, 2), (2, 3), (3, 0)],
            vec![(0, 2), (1, 3), (0, 4), (4, 2)],
        ),
        vec![2, 3, 1, 4],
        vec![2, 3, 1, 2],
    )
}

fn disconnected_instance() -> MixedChinesePostman<i32> {
    MixedChinesePostman::new(
        MixedGraph::new(
            6,
            vec![(0, 1), (1, 0), (2, 3)],
            vec![(0, 2), (1, 3), (3, 4), (4, 5), (5, 2)],
        ),
        vec![1, 1, 1],
        vec![1, 1, 5, 5, 5],
    )
}

#[test]
fn test_mixed_chinese_postman_creation_and_accessors() {
    let problem = sample_instance();

    assert_eq!(problem.num_vertices(), 5);
    assert_eq!(problem.num_arcs(), 4);
    assert_eq!(problem.num_edges(), 4);
    assert_eq!(problem.dims(), vec![2, 2, 2, 2]);
    assert_eq!(problem.arc_weights(), &[2, 3, 1, 4]);
    assert_eq!(problem.edge_weights(), &[2, 3, 1, 2]);
}

#[test]
fn test_mixed_chinese_postman_evaluate_optimal() {
    let problem = sample_instance();

    // Reverse (0,2) and (1,3), keep (0,4) and (4,2) forward.
    assert_eq!(problem.evaluate(&[1, 1, 0, 0]), Min(Some(21)));
}

#[test]
fn test_mixed_chinese_postman_evaluate_connected_instance() {
    let problem = disconnected_instance();

    // The available graph is strongly connected, so valid orientations
    // should return Some(cost).
    let val = problem.evaluate(&[0, 0, 0, 0, 0]);
    assert!(val.0.is_some());
}

#[test]
fn test_mixed_chinese_postman_single_edge_walk() {
    // V={0,1}, A=∅, E={{0,1}}, weight=1.
    // Walk 0→1→0: base cost 1, needs to balance so total cost is 2.
    let problem =
        MixedChinesePostman::new(MixedGraph::new(2, vec![], vec![(0, 1)]), vec![], vec![1]);

    assert_eq!(problem.evaluate(&[0]), Min(Some(2)));
    assert_eq!(problem.evaluate(&[1]), Min(Some(2)));

    let solver = BruteForce::new();
    assert!(solver.find_witness(&problem).is_some());
}

#[test]
fn test_mixed_chinese_postman_rejects_disconnected_graph() {
    // Two disconnected components {0,1} and {2,3}: no closed walk can cover all edges.
    let problem = MixedChinesePostman::new(
        MixedGraph::new(4, vec![], vec![(0, 1), (2, 3)]),
        vec![],
        vec![1, 1],
    );

    assert_eq!(problem.evaluate(&[0, 0]), Min(None));
    assert_eq!(problem.evaluate(&[0, 1]), Min(None));
    assert_eq!(problem.evaluate(&[1, 0]), Min(None));
    assert_eq!(problem.evaluate(&[1, 1]), Min(None));
}

#[test]
fn test_mixed_chinese_postman_rejects_wrong_config_length() {
    let problem = sample_instance();

    assert_eq!(problem.evaluate(&[]), Min(None));
    assert_eq!(problem.evaluate(&[1, 1, 0]), Min(None));
    assert_eq!(problem.evaluate(&[1, 1, 0, 0, 1]), Min(None));
}

#[test]
fn test_mixed_chinese_postman_solver_finds_optimal() {
    let problem = sample_instance();
    let solver = BruteForce::new();

    let solution = solver
        .find_witness(&problem)
        .expect("expected an optimal orientation");
    assert!(problem.is_valid_solution(&solution));
    // The optimal cost should be 21.
    assert_eq!(problem.evaluate(&solution), Min(Some(21)));
}

#[test]
fn test_mixed_chinese_postman_serialization_roundtrip() {
    let problem = sample_instance();

    let json = serde_json::to_string(&problem).unwrap();
    let restored: MixedChinesePostman<i32> = serde_json::from_str(&json).unwrap();

    assert_eq!(restored.num_vertices(), 5);
    assert_eq!(restored.num_arcs(), 4);
    assert_eq!(restored.num_edges(), 4);
    assert_eq!(restored.arc_weights(), &[2, 3, 1, 4]);
    assert_eq!(restored.edge_weights(), &[2, 3, 1, 2]);
}

#[test]
fn test_mixed_chinese_postman_problem_name() {
    assert_eq!(
        <MixedChinesePostman<i32> as Problem>::NAME,
        "MixedChinesePostman"
    );
}
