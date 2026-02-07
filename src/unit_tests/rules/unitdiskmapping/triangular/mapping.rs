use super::*;
use crate::topology::Graph;

#[test]
fn test_map_weighted_basic() {
    let edges = vec![(0, 1), (1, 2)];
    let result = map_weighted(3, &edges);

    assert!(result.grid_graph.num_vertices() > 0);
    assert!(matches!(
        result.grid_graph.grid_type(),
        GridType::Triangular { .. }
    ));
}

#[test]
fn test_map_weighted_with_method() {
    let edges = vec![(0, 1), (1, 2)];
    let result = map_weighted_with_method(3, &edges, PathDecompositionMethod::MinhThiTrick);

    assert!(result.grid_graph.num_vertices() > 0);
}

#[test]
fn test_map_weighted_with_order() {
    let edges = vec![(0, 1), (1, 2)];
    let vertex_order = vec![0, 1, 2];
    let result = map_weighted_with_order(3, &edges, &vertex_order);

    assert!(result.grid_graph.num_vertices() > 0);
}

#[test]
fn test_trace_centers() {
    let edges = vec![(0, 1), (1, 2)];
    let result = map_weighted(3, &edges);

    let centers = trace_centers(&result);
    assert_eq!(centers.len(), 3);

    // Centers should be valid grid positions
    for (row, col) in &centers {
        assert!(*row > 0);
        assert!(*col > 0);
    }
}

#[test]
fn test_map_weights() {
    let edges = vec![(0, 1), (1, 2)];
    let result = map_weighted(3, &edges);

    let source_weights = vec![0.5, 0.3, 0.7];
    let grid_weights = map_weights(&result, &source_weights);

    // Should have same length as grid nodes
    assert_eq!(grid_weights.len(), result.grid_graph.num_vertices());

    // All weights should be positive
    assert!(grid_weights.iter().all(|&w| w > 0.0));
}

#[test]
fn test_weighted_ruleset() {
    let ruleset = weighted_ruleset();
    assert_eq!(ruleset.len(), 13);
}

#[test]
#[should_panic(expected = "num_vertices must be > 0")]
fn test_map_weighted_panics_on_zero_vertices() {
    let edges: Vec<(usize, usize)> = vec![];
    map_weighted(0, &edges);
}
