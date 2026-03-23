use super::*;
use crate::solvers::BruteForce;
use crate::topology::DirectedGraph;
use crate::traits::Problem;

fn yes_instance() -> PathConstrainedNetworkFlow {
    let graph = DirectedGraph::new(
        8,
        vec![
            (0, 1),
            (0, 2),
            (1, 3),
            (1, 4),
            (2, 4),
            (3, 5),
            (4, 5),
            (4, 6),
            (5, 7),
            (6, 7),
        ],
    );

    PathConstrainedNetworkFlow::new(
        graph,
        vec![2, 1, 1, 1, 1, 1, 1, 1, 2, 1],
        0,
        7,
        vec![
            vec![0, 2, 5, 8],
            vec![0, 3, 6, 8],
            vec![0, 3, 7, 9],
            vec![1, 4, 6, 8],
            vec![1, 4, 7, 9],
        ],
        3,
    )
}

fn no_instance() -> PathConstrainedNetworkFlow {
    let mut problem = yes_instance();
    problem.set_requirement(4);
    problem
}

#[test]
fn test_path_constrained_network_flow_creation() {
    let problem = yes_instance();
    assert_eq!(problem.num_vertices(), 8);
    assert_eq!(problem.num_arcs(), 10);
    assert_eq!(problem.num_paths(), 5);
    assert_eq!(problem.max_capacity(), 2);
    assert_eq!(problem.requirement(), 3);
    assert_eq!(problem.source(), 0);
    assert_eq!(problem.sink(), 7);
    assert_eq!(problem.graph().num_vertices(), 8);
    assert_eq!(problem.capacities().len(), 10);
    assert_eq!(problem.paths().len(), 5);
}

#[test]
fn test_path_constrained_network_flow_dims_use_path_bottlenecks() {
    let problem = yes_instance();
    assert_eq!(problem.dims(), vec![2, 2, 2, 2, 2]);
}

#[test]
fn test_path_constrained_network_flow_evaluation_satisfying() {
    let problem = yes_instance();
    assert!(problem.evaluate(&[1, 1, 0, 0, 1]));
    assert!(problem.evaluate(&[1, 0, 1, 1, 0]));
}

#[test]
fn test_path_constrained_network_flow_evaluation_unsatisfying() {
    let problem = yes_instance();
    assert!(!problem.evaluate(&[1, 1, 0, 0, 0]));
    assert!(!problem.evaluate(&[1, 1, 1, 0, 0]));
    assert!(!problem.evaluate(&[1, 1, 0, 0]));
}

#[test]
fn test_path_constrained_network_flow_solver_yes_and_no() {
    let yes = yes_instance();
    let no = no_instance();
    let solver = BruteForce::new();

    let satisfying = solver.find_all_witnesses(&yes);
    assert_eq!(satisfying.len(), 2);
    assert!(satisfying.iter().all(|config| yes.evaluate(config)));

    assert!(solver.find_witness(&no).is_none());
}

#[test]
fn test_path_constrained_network_flow_serialization() {
    let problem = yes_instance();
    let json = serde_json::to_string(&problem).unwrap();
    let restored: PathConstrainedNetworkFlow = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.num_vertices(), 8);
    assert_eq!(restored.num_arcs(), 10);
    assert_eq!(restored.num_paths(), 5);
    assert_eq!(restored.requirement(), 3);
}

#[test]
fn test_path_constrained_network_flow_rejects_non_contiguous_path() {
    let graph = DirectedGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let result = std::panic::catch_unwind(|| {
        PathConstrainedNetworkFlow::new(graph, vec![1, 1, 1], 0, 3, vec![vec![0, 2]], 1)
    });
    assert!(result.is_err());
}

#[test]
fn test_path_constrained_network_flow_rejects_empty_path() {
    let graph = DirectedGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let result = std::panic::catch_unwind(|| {
        PathConstrainedNetworkFlow::new(graph, vec![1, 1, 1], 0, 3, vec![vec![]], 1)
    });
    assert!(result.is_err());
}

#[test]
fn test_path_constrained_network_flow_rejects_path_not_ending_at_sink() {
    let graph = DirectedGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let result = std::panic::catch_unwind(|| {
        PathConstrainedNetworkFlow::new(graph, vec![1, 1, 1], 0, 3, vec![vec![0, 1]], 1)
    });
    assert!(result.is_err());
}

#[test]
fn test_path_constrained_network_flow_rejects_path_with_repeated_vertex() {
    // Graph: 0->1, 1->2, 2->1, 1->3 (arcs 0,1,2,3)
    let graph = DirectedGraph::new(4, vec![(0, 1), (1, 2), (2, 1), (1, 3)]);
    let result = std::panic::catch_unwind(|| {
        // Path [0, 1, 2, 3]: 0->1->2->1->3 revisits vertex 1
        PathConstrainedNetworkFlow::new(graph, vec![1, 1, 1, 1], 0, 3, vec![vec![0, 1, 2, 3]], 1)
    });
    assert!(result.is_err());
}

#[test]
fn test_path_constrained_network_flow_paper_example() {
    let problem = yes_instance();
    let solver = BruteForce::new();
    let config = vec![1, 1, 0, 0, 1];

    assert!(problem.evaluate(&config));

    let all = solver.find_all_witnesses(&problem);
    assert_eq!(all.len(), 2);
    assert!(all.contains(&config));
}
