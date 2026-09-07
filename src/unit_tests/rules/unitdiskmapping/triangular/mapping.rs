use super::*;
use crate::rules::unitdiskmapping::triangular::{map_weights, trace_centers};

#[test]
fn test_map_weighted_basic() {
    let edges = vec![(0, 1), (1, 2)];
    let result = map_weighted(3, &edges).unwrap();
    assert!(!result.positions.is_empty());
    assert!(matches!(result.kind, GridKind::Triangular));
}

#[test]
fn test_map_weighted_with_method() {
    let edges = vec![(0, 1), (1, 2)];
    let result =
        map_weighted_with_method(3, &edges, PathDecompositionMethod::MinhThiTrick).unwrap();
    assert!(!result.positions.is_empty());
}

#[test]
fn test_map_weighted_with_order() {
    let edges = vec![(0, 1), (1, 2)];
    let vertex_order = vec![0, 1, 2];
    let result = map_weighted_with_order(3, &edges, &vertex_order).unwrap();
    assert!(!result.positions.is_empty());
}

#[test]
fn test_trace_centers() {
    let edges = vec![(0, 1), (1, 2)];
    let result = map_weighted(3, &edges).unwrap();
    let centers = trace_centers(&result).unwrap();
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
    let result = map_weighted(3, &edges).unwrap();
    let source_weights = vec![0.5, 0.3, 0.7];
    let grid_weights = map_weights(&result, &source_weights).unwrap();

    // Should have same length as grid nodes
    assert_eq!(grid_weights.len(), result.positions.len());

    // All weights should be positive
    assert!(grid_weights.iter().all(|&w| w > 0.0));
}

#[test]
fn test_map_config_back_rejects_wrong_length() {
    let result = map_weighted(2, &[(0, 1)]).unwrap();

    assert!(matches!(
        map_config_back(&result, &[]),
        Err(crate::rules::ExtractionError::InvalidTargetSolution(_))
    ));
}

#[test]
fn test_map_weighted_rejects_zero_vertices() {
    let edges: Vec<(usize, usize)> = vec![];
    assert!(matches!(
        map_weighted(0, &edges),
        Err(crate::rules::ReductionError::InvalidTarget { .. })
    ));
}
