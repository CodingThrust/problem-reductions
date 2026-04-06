use super::*;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_minimum_metric_dimension_creation() {
    let graph = SimpleGraph::new(5, vec![(0, 1), (0, 2), (1, 3), (2, 3), (2, 4), (3, 4)]);
    let problem = MinimumMetricDimension::new(graph);
    assert_eq!(problem.graph().num_vertices(), 5);
    assert_eq!(problem.graph().num_edges(), 6);
    assert_eq!(problem.num_variables(), 5);
    assert_eq!(problem.dims(), vec![2; 5]);
}

#[test]
fn test_minimum_metric_dimension_evaluate_optimal() {
    // House graph: selecting vertices 0 and 1 forms a resolving set of size 2
    let graph = SimpleGraph::new(5, vec![(0, 1), (0, 2), (1, 3), (2, 3), (2, 4), (3, 4)]);
    let problem = MinimumMetricDimension::new(graph);
    let config = vec![1, 1, 0, 0, 0]; // select v0, v1
    let result = problem.evaluate(&config);
    assert!(result.is_valid());
    assert_eq!(result, Min(Some(2)));
}

#[test]
fn test_minimum_metric_dimension_evaluate_non_resolving() {
    // House graph: selecting only v2 should not resolve all pairs
    let graph = SimpleGraph::new(5, vec![(0, 1), (0, 2), (1, 3), (2, 3), (2, 4), (3, 4)]);
    let problem = MinimumMetricDimension::new(graph);
    // v2 alone: d(0,2)=1, d(1,2)=2, d(3,2)=1, d(4,2)=1
    // vertices 0 and 3 both have distance 1 to v2 -> not resolving
    let config = vec![0, 0, 1, 0, 0];
    let result = problem.evaluate(&config);
    assert_eq!(result, Min(None));
}

#[test]
fn test_minimum_metric_dimension_evaluate_empty_selection() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumMetricDimension::new(graph);
    let config = vec![0, 0, 0];
    let result = problem.evaluate(&config);
    assert_eq!(result, Min(None));
}

#[test]
fn test_minimum_metric_dimension_evaluate_all_selected() {
    // Selecting all vertices is always resolving (trivially)
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumMetricDimension::new(graph);
    let config = vec![1, 1, 1];
    let result = problem.evaluate(&config);
    assert!(result.is_valid());
    assert_eq!(result, Min(Some(3)));
}

#[test]
fn test_minimum_metric_dimension_solver() {
    // House graph: minimum resolving set has size 2
    let graph = SimpleGraph::new(5, vec![(0, 1), (0, 2), (1, 3), (2, 3), (2, 4), (3, 4)]);
    let problem = MinimumMetricDimension::new(graph);
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem).unwrap();
    let value = problem.evaluate(&solution);
    assert!(value.is_valid());
    assert_eq!(value, Min(Some(2)));
}

#[test]
fn test_minimum_metric_dimension_path_graph() {
    // Path graph P3: 0-1-2
    // Metric dimension of a path is 1 (either endpoint resolves)
    // d(0,0)=0, d(1,0)=1, d(2,0)=2 -> all distinct -> {0} resolves
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumMetricDimension::new(graph);
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem).unwrap();
    let value = problem.evaluate(&solution);
    assert_eq!(value, Min(Some(1)));
}

#[test]
fn test_minimum_metric_dimension_complete_graph() {
    // K4: metric dimension of K_n is n-1 (all distances are 1, so any pair
    // at distance 1 from each other needs a resolving vertex that is one of them)
    let graph = SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
    let problem = MinimumMetricDimension::new(graph);
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem).unwrap();
    let value = problem.evaluate(&solution);
    assert_eq!(value, Min(Some(3)));
}

#[test]
fn test_minimum_metric_dimension_serialization() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumMetricDimension::new(graph);
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: MinimumMetricDimension<SimpleGraph> = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.num_vertices(), 3);
    assert_eq!(deserialized.num_edges(), 2);

    // Verify evaluation is preserved
    let config = vec![1, 0, 0];
    assert_eq!(problem.evaluate(&config), deserialized.evaluate(&config));
}

#[test]
fn test_minimum_metric_dimension_size_getters() {
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let problem = MinimumMetricDimension::new(graph);
    assert_eq!(problem.num_vertices(), 4);
    assert_eq!(problem.num_edges(), 3);
}

#[test]
fn test_minimum_metric_dimension_cycle() {
    // C5: metric dimension of a cycle C_n with n >= 3 is 2
    let graph = SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)]);
    let problem = MinimumMetricDimension::new(graph);
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem).unwrap();
    let value = problem.evaluate(&solution);
    assert_eq!(value, Min(Some(2)));
}
