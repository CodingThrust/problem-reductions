use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

fn k4_problem() -> OptimumCommunicationSpanningTree {
    let edge_weights = vec![
        vec![0, 1, 3, 2],
        vec![1, 0, 2, 4],
        vec![3, 2, 0, 1],
        vec![2, 4, 1, 0],
    ];
    let requirements = vec![
        vec![0, 2, 1, 3],
        vec![2, 0, 1, 1],
        vec![1, 1, 0, 2],
        vec![3, 1, 2, 0],
    ];
    OptimumCommunicationSpanningTree::new(edge_weights, requirements)
}

#[test]
fn test_ocst_creation() {
    let problem = k4_problem();
    assert_eq!(problem.num_vertices(), 4);
    assert_eq!(problem.num_edges(), 6);
    assert_eq!(problem.dims(), vec![2; 6]);
    assert_eq!(
        <OptimumCommunicationSpanningTree as Problem>::NAME,
        "OptimumCommunicationSpanningTree"
    );
    assert_eq!(
        <OptimumCommunicationSpanningTree as Problem>::variant(),
        vec![]
    );
}

#[test]
fn test_ocst_edges() {
    let problem = k4_problem();
    let edges = problem.edges();
    assert_eq!(edges, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
}

#[test]
fn test_ocst_edge_index() {
    let n = 4;
    assert_eq!(OptimumCommunicationSpanningTree::edge_index(0, 1, n), 0);
    assert_eq!(OptimumCommunicationSpanningTree::edge_index(0, 2, n), 1);
    assert_eq!(OptimumCommunicationSpanningTree::edge_index(0, 3, n), 2);
    assert_eq!(OptimumCommunicationSpanningTree::edge_index(1, 2, n), 3);
    assert_eq!(OptimumCommunicationSpanningTree::edge_index(1, 3, n), 4);
    assert_eq!(OptimumCommunicationSpanningTree::edge_index(2, 3, n), 5);
}

#[test]
fn test_ocst_evaluate_optimal() {
    let problem = k4_problem();
    // Optimal tree: {(0,1), (0,3), (2,3)} -> indices 0, 2, 5
    // config = [1, 0, 1, 0, 0, 1]
    // Path costs:
    //   W(0,1) = 1, W(0,2) = 2+1 = 3, W(0,3) = 2
    //   W(1,2) = 1+2+1 = 4, W(1,3) = 1+2 = 3, W(2,3) = 1
    // Total = 1*2 + 3*1 + 2*3 + 4*1 + 3*1 + 1*2 = 2+3+6+4+3+2 = 20
    assert_eq!(problem.evaluate(&[1, 0, 1, 0, 0, 1]), Min(Some(20)));
}

#[test]
fn test_ocst_evaluate_suboptimal() {
    let problem = k4_problem();
    // Suboptimal tree: {(0,1), (1,2), (2,3)} -> indices 0, 3, 5
    // config = [1, 0, 0, 1, 0, 1]
    // Path costs:
    //   W(0,1) = 1, W(0,2) = 1+2 = 3, W(0,3) = 1+2+1 = 4
    //   W(1,2) = 2, W(1,3) = 2+1 = 3, W(2,3) = 1
    // Total = 1*2 + 3*1 + 4*3 + 2*1 + 3*1 + 1*2 = 2+3+12+2+3+2 = 24
    assert_eq!(problem.evaluate(&[1, 0, 0, 1, 0, 1]), Min(Some(24)));
}

#[test]
fn test_ocst_evaluate_invalid() {
    let problem = k4_problem();
    // Wrong number of edges
    assert_eq!(problem.evaluate(&[1, 0, 1]), Min(None));
    // Too many edges (not a tree)
    assert_eq!(problem.evaluate(&[1, 1, 1, 1, 0, 1]), Min(None));
    // Not connected (two separate edges)
    assert_eq!(problem.evaluate(&[1, 0, 0, 0, 0, 1]), Min(None));
    // Value > 1
    assert_eq!(problem.evaluate(&[2, 0, 1, 0, 0, 0]), Min(None));
}

#[test]
fn test_ocst_solver() {
    let problem = k4_problem();
    let solver = BruteForce::new();
    let solution = solver
        .find_witness(&problem)
        .expect("should find a solution");
    let value = problem.evaluate(&solution);
    assert_eq!(value, Min(Some(20)));
}

#[test]
fn test_ocst_serialization() {
    let problem = k4_problem();
    let json = serde_json::to_value(&problem).unwrap();
    let restored: OptimumCommunicationSpanningTree = serde_json::from_value(json).unwrap();
    assert_eq!(restored.num_vertices(), 4);
    assert_eq!(restored.edge_weights(), problem.edge_weights());
    assert_eq!(restored.requirements(), problem.requirements());
}

#[test]
fn test_ocst_k3_equal_requirements() {
    // K3 with all requirements equal to 1
    // edge_weights: w(0,1)=1, w(0,2)=2, w(1,2)=3
    let edge_weights = vec![vec![0, 1, 2], vec![1, 0, 3], vec![2, 3, 0]];
    let requirements = vec![vec![0, 1, 1], vec![1, 0, 1], vec![1, 1, 0]];
    let problem = OptimumCommunicationSpanningTree::new(edge_weights, requirements);
    assert_eq!(problem.num_vertices(), 3);
    assert_eq!(problem.num_edges(), 3);

    // Tree {(0,1), (0,2)}: W(0,1)=1, W(0,2)=2, W(1,2)=1+2=3, cost = 1+2+3 = 6
    assert_eq!(problem.evaluate(&[1, 1, 0]), Min(Some(6)));
    // Tree {(0,1), (1,2)}: W(0,1)=1, W(0,2)=1+3=4, W(1,2)=3, cost = 1+4+3 = 8
    assert_eq!(problem.evaluate(&[1, 0, 1]), Min(Some(8)));
    // Tree {(0,2), (1,2)}: W(0,1)=2+3=5, W(0,2)=2, W(1,2)=3, cost = 5+2+3 = 10
    assert_eq!(problem.evaluate(&[0, 1, 1]), Min(Some(10)));

    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem).unwrap();
    assert_eq!(problem.evaluate(&solution), Min(Some(6)));
}

#[test]
#[should_panic(expected = "must have at least 2 vertices")]
fn test_ocst_single_vertex_panics() {
    OptimumCommunicationSpanningTree::new(vec![vec![0]], vec![vec![0]]);
}

#[test]
#[should_panic(expected = "edge_weights must be symmetric")]
fn test_ocst_asymmetric_weights_panics() {
    OptimumCommunicationSpanningTree::new(
        vec![vec![0, 1], vec![2, 0]],
        vec![vec![0, 1], vec![1, 0]],
    );
}

#[test]
#[should_panic(expected = "requirements must be symmetric")]
fn test_ocst_asymmetric_requirements_panics() {
    OptimumCommunicationSpanningTree::new(
        vec![vec![0, 1], vec![1, 0]],
        vec![vec![0, 1], vec![2, 0]],
    );
}

#[cfg(feature = "example-db")]
#[test]
fn test_ocst_canonical_example() {
    let specs = canonical_model_example_specs();
    assert_eq!(specs.len(), 1);
    let spec = &specs[0];
    assert_eq!(spec.id, "optimum_communication_spanning_tree");
    assert_eq!(spec.optimal_config, vec![1, 0, 1, 0, 0, 1]);
    assert_eq!(spec.optimal_value, serde_json::json!(20));
}
