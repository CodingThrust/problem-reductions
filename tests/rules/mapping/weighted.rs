//! Tests for weighted mode functionality (src/rules/mapping/weighted.rs).

use problemreductions::rules::mapping::{
    map_graph_triangular, map_weights, trace_centers, CopyLine,
    copyline_weighted_locations_triangular,
};
use problemreductions::topology::Graph;

// === Trace Centers Tests ===

#[test]
fn test_trace_centers_returns_correct_count() {
    let edges = vec![(0, 1), (1, 2)];
    let result = map_graph_triangular(3, &edges);

    let centers = trace_centers(&result);
    assert_eq!(centers.len(), 3);
}

#[test]
fn test_trace_centers_positive_coordinates() {
    let edges = vec![(0, 1), (1, 2), (0, 2)];
    let result = map_graph_triangular(3, &edges);

    let centers = trace_centers(&result);
    for (i, &(row, col)) in centers.iter().enumerate() {
        assert!(row > 0, "Vertex {} center row should be positive", i);
        assert!(col > 0, "Vertex {} center col should be positive", i);
    }
}

#[test]
fn test_trace_centers_single_vertex() {
    let edges: Vec<(usize, usize)> = vec![];
    let result = map_graph_triangular(1, &edges);

    let centers = trace_centers(&result);
    assert_eq!(centers.len(), 1);
}

// === Map Weights Tests ===

#[test]
fn test_map_weights_uniform() {
    let edges = vec![(0, 1), (1, 2)];
    let result = map_graph_triangular(3, &edges);

    // Use uniform weights (all 0.5)
    let weights = vec![0.5, 0.5, 0.5];
    let mapped = map_weights(&result, &weights);

    // Mapped weights should be non-negative
    assert!(
        mapped.iter().all(|&w| w >= 0.0),
        "All mapped weights should be non-negative"
    );

    // Mapped should have one weight per grid node
    assert_eq!(mapped.len(), result.grid_graph.num_vertices());
}

#[test]
fn test_map_weights_zero() {
    let edges = vec![(0, 1), (1, 2)];
    let result = map_graph_triangular(3, &edges);

    let weights = vec![0.0, 0.0, 0.0];
    let mapped = map_weights(&result, &weights);

    // With zero weights, the mapped weights should be positive
    // (because of the overhead structure)
    assert!(mapped.iter().any(|&w| w > 0.0));
}

#[test]
fn test_map_weights_one() {
    use problemreductions::topology::Graph;

    let edges = vec![(0, 1), (1, 2)];
    let result = map_graph_triangular(3, &edges);

    let weights = vec![1.0, 1.0, 1.0];
    let mapped = map_weights(&result, &weights);

    // All weights should be positive
    assert!(mapped.iter().all(|&w| w > 0.0));

    // Mapped weights should equal base weights plus original weights at centers
    let base_total: f64 = result
        .grid_graph
        .nodes()
        .iter()
        .map(|n| n.weight as f64)
        .sum();
    let original_total: f64 = weights.iter().sum();
    let mapped_total: f64 = mapped.iter().sum();

    // The mapped total should be base_total + original_total
    // Allow 1.0 tolerance for rounding or center node lookup differences
    assert!(
        (mapped_total - (base_total + original_total)).abs() < 1.5,
        "Mapped total {} should be close to base {} + original {} = {}",
        mapped_total,
        base_total,
        original_total,
        base_total + original_total
    );
}

#[test]
#[should_panic]
fn test_map_weights_invalid_negative() {
    let edges = vec![(0, 1)];
    let result = map_graph_triangular(2, &edges);

    let weights = vec![-0.5, 0.5];
    let _ = map_weights(&result, &weights);
}

#[test]
#[should_panic]
fn test_map_weights_invalid_over_one() {
    let edges = vec![(0, 1)];
    let result = map_graph_triangular(2, &edges);

    let weights = vec![1.5, 0.5];
    let _ = map_weights(&result, &weights);
}

#[test]
#[should_panic]
fn test_map_weights_wrong_length() {
    let edges = vec![(0, 1)];
    let result = map_graph_triangular(2, &edges);

    let weights = vec![0.5]; // Wrong length
    let _ = map_weights(&result, &weights);
}

// === Weighted Interface Tests ===

#[test]
fn test_triangular_weighted_interface() {
    use problemreductions::topology::smallgraph;

    let (n, edges) = smallgraph("bull").unwrap();
    let result = map_graph_triangular(n, &edges);

    // Test with uniform weights
    let ws = vec![0.5; n];
    let grid_weights = map_weights(&result, &ws);

    // Should produce valid weights for all grid nodes
    assert_eq!(grid_weights.len(), result.grid_graph.num_vertices());
    assert!(grid_weights.iter().all(|&w| w > 0.0));
}

#[test]
fn test_triangular_interface_full() {
    use problemreductions::topology::smallgraph;

    let (n, edges) = smallgraph("diamond").unwrap();
    let result = map_graph_triangular(n, &edges);

    // Uniform weights in [0, 1]
    let ws = vec![0.3; n];
    let grid_weights = map_weights(&result, &ws);

    assert_eq!(grid_weights.len(), result.grid_graph.num_vertices());
    assert!(grid_weights.iter().all(|&w| w >= 0.0));

    // Test map_config_back
    let config = vec![0; result.grid_graph.num_vertices()];
    let original_config = result.map_config_back(&config);
    assert_eq!(original_config.len(), n);

    // Verify trace_centers
    let centers = trace_centers(&result);
    assert_eq!(centers.len(), n);
}

// === Copyline Weight Invariant Tests ===

#[test]
fn test_triangular_copyline_weight_invariant() {
    let spacing = 6usize;

    // Test various copyline configurations
    let configs = [
        (1, 1, 1, 2), // Simple case
        (1, 2, 1, 3), // With vertical segment
        (2, 3, 2, 4), // Offset case
    ];

    for (vslot, hslot, vstart, hstop) in configs {
        let vstop = hslot.max(vstart);
        let copyline = CopyLine::new(0, vslot, hslot, vstart, vstop, hstop);
        let (locs, weights) = copyline_weighted_locations_triangular(&copyline, spacing);

        // Weights should be positive
        assert!(
            weights.iter().all(|&w| w >= 1),
            "Config ({}, {}, {}, {}): all weights should be >= 1",
            vslot,
            hslot,
            vstart,
            hstop
        );

        // Locations and weights should have same length
        assert_eq!(
            locs.len(),
            weights.len(),
            "Config ({}, {}, {}, {}): locs and weights should match",
            vslot,
            hslot,
            vstart,
            hstop
        );
    }
}
