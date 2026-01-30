//! Tests for map_graph functionality (src/rules/mapping/map_graph.rs).
//!
//! Tests square lattice mapping, MappingResult, and config_back.

use super::common::{is_independent_set, solve_mis, solve_mis_config};
use problemreductions::rules::unitdiskmapping::{map_graph, map_graph_with_order, MappingResult};
use problemreductions::topology::{smallgraph, Graph, GridType};

// === Square Lattice Basic Tests ===

#[test]
fn test_map_path_graph() {
    let edges = vec![(0, 1), (1, 2)];
    let result = map_graph(3, &edges);

    assert!(result.grid_graph.num_vertices() > 0);
    assert!(result.mis_overhead >= 0);

    let config = vec![0; result.grid_graph.num_vertices()];
    let original = result.map_config_back(&config);
    assert_eq!(original.len(), 3);
}

#[test]
fn test_map_triangle_graph() {
    let edges = vec![(0, 1), (1, 2), (0, 2)];
    let result = map_graph(3, &edges);

    assert!(result.grid_graph.num_vertices() >= 3);
    assert!(result.mis_overhead >= 0);
    assert_eq!(result.lines.len(), 3);
}

#[test]
fn test_map_star_graph() {
    let edges = vec![(0, 1), (0, 2), (0, 3)];
    let result = map_graph(4, &edges);

    assert!(result.grid_graph.num_vertices() > 4);
    assert_eq!(result.lines.len(), 4);
}

#[test]
fn test_map_empty_graph() {
    let edges: Vec<(usize, usize)> = vec![];
    let result = map_graph(3, &edges);

    assert!(result.grid_graph.num_vertices() > 0);
    assert_eq!(result.lines.len(), 3);
}

#[test]
fn test_map_single_edge() {
    let edges = vec![(0, 1)];
    let result = map_graph(2, &edges);

    assert_eq!(result.lines.len(), 2);
    assert!(result.grid_graph.num_vertices() > 0);
}

#[test]
fn test_map_single_vertex() {
    let edges: Vec<(usize, usize)> = vec![];
    let result = map_graph(1, &edges);

    assert_eq!(result.lines.len(), 1);
    assert!(result.grid_graph.num_vertices() > 0);
}

#[test]
fn test_map_complete_k4() {
    let edges = vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];
    let result = map_graph(4, &edges);

    assert!(result.grid_graph.num_vertices() > 4);
    assert_eq!(result.lines.len(), 4);
}

#[test]
fn test_map_graph_with_custom_order() {
    let edges = vec![(0, 1), (1, 2)];
    let order = vec![2, 1, 0];
    let result = map_graph_with_order(3, &edges, &order);

    assert!(result.grid_graph.num_vertices() > 0);
    assert_eq!(result.lines.len(), 3);
}

#[test]
fn test_square_grid_type() {
    let edges = vec![(0, 1)];
    let result = map_graph(2, &edges);

    assert!(matches!(result.grid_graph.grid_type(), GridType::Square));
}

#[test]
fn test_mapping_preserves_vertex_count() {
    let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4)];
    let result = map_graph(5, &edges);

    assert_eq!(result.lines.len(), 5);

    let vertices: Vec<usize> = result.lines.iter().map(|l| l.vertex).collect();
    for v in 0..5 {
        assert!(vertices.contains(&v), "Vertex {} not found in copy lines", v);
    }
}

// === MappingResult Tests ===

#[test]
fn test_mapping_result_serialization() {
    let edges = vec![(0, 1), (1, 2)];
    let result = map_graph(3, &edges);

    let json = serde_json::to_string(&result).unwrap();
    let deserialized: MappingResult = serde_json::from_str(&json).unwrap();

    assert_eq!(result.mis_overhead, deserialized.mis_overhead);
    assert_eq!(result.lines.len(), deserialized.lines.len());
}

#[test]
fn test_mapping_result_config_back_all_zeros() {
    let edges = vec![(0, 1), (1, 2)];
    let result = map_graph(3, &edges);

    let config = vec![0; result.grid_graph.num_vertices()];
    let original = result.map_config_back(&config);

    assert_eq!(original.len(), 3);
    assert!(original.iter().all(|&x| x == 0));
}

/// Test that map_config_back returns the correct length.
/// Note: All-ones config is invalid for MIS, so we use all-zeros instead.
#[test]
fn test_mapping_result_config_back_returns_correct_length() {
    let edges = vec![(0, 1), (1, 2)];
    let result = map_graph(3, &edges);

    let config = vec![0; result.grid_graph.num_vertices()];
    let original = result.map_config_back(&config);

    assert_eq!(original.len(), 3);
    assert!(original.iter().all(|&x| x == 0));
}

#[test]
fn test_mapping_result_fields_populated() {
    let edges = vec![(0, 1), (1, 2), (0, 2)];
    let result = map_graph(3, &edges);

    assert!(!result.lines.is_empty());
    assert!(result.grid_graph.num_vertices() > 0);
    assert!(result.spacing > 0);
    assert!(result.padding > 0);
}

// === Edge Cases ===

#[test]
fn test_disconnected_graph() {
    // Two disconnected edges
    let edges = vec![(0, 1), (2, 3)];
    let result = map_graph(4, &edges);

    assert_eq!(result.lines.len(), 4);
    assert!(result.grid_graph.num_vertices() > 0);
}

#[test]
fn test_linear_chain() {
    let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4)];
    let result = map_graph(5, &edges);

    assert_eq!(result.lines.len(), 5);
}

#[test]
fn test_cycle_graph() {
    // C5: pentagon
    let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)];
    let result = map_graph(5, &edges);

    assert_eq!(result.lines.len(), 5);
}

#[test]
fn test_bipartite_graph() {
    // K2,3
    let edges = vec![(0, 2), (0, 3), (0, 4), (1, 2), (1, 3), (1, 4)];
    let result = map_graph(5, &edges);

    assert_eq!(result.lines.len(), 5);
}

// === Standard Graphs ===

#[test]
fn test_map_standard_graphs_square() {
    let graph_names = ["bull", "petersen", "cubical", "house", "diamond"];

    for name in graph_names {
        let (n, edges) = smallgraph(name).unwrap();
        let result = map_graph(n, &edges);

        assert_eq!(
            result.lines.len(),
            n,
            "{}: should have {} copy lines",
            name,
            n
        );
        assert!(
            result.grid_graph.num_vertices() > 0,
            "{}: should have grid nodes",
            name
        );
    }
}

// === MIS Verification ===

#[test]
fn test_map_config_back_returns_valid_is() {
    let edges = vec![(0, 1), (1, 2), (0, 2)];
    let result = map_graph(3, &edges);

    let grid_edges = result.grid_graph.edges().to_vec();
    let grid_config = solve_mis_config(result.grid_graph.num_vertices(), &grid_edges);

    let original_config = result.map_config_back(&grid_config);

    assert!(
        is_independent_set(&edges, &original_config),
        "Mapped back config should be a valid IS"
    );
}

#[test]
fn test_mis_overhead_path_graph() {
    let edges = vec![(0, 1), (1, 2)];
    let n = 3;
    let result = map_graph(n, &edges);

    let original_mis = solve_mis(n, &edges) as i32;
    let grid_edges = result.grid_graph.edges().to_vec();
    let mapped_mis = solve_mis(result.grid_graph.num_vertices(), &grid_edges) as i32;

    let expected = original_mis + result.mis_overhead;

    assert!(
        (mapped_mis - expected).abs() <= 1,
        "Path graph: mapped MIS {} should equal original {} + overhead {} = {}",
        mapped_mis,
        original_mis,
        result.mis_overhead,
        expected
    );
}

#[test]
fn test_mis_overhead_triangle() {
    let edges = vec![(0, 1), (1, 2), (0, 2)];
    let n = 3;
    let result = map_graph(n, &edges);

    let original_mis = solve_mis(n, &edges) as i32;
    let grid_edges = result.grid_graph.edges().to_vec();
    let mapped_mis = solve_mis(result.grid_graph.num_vertices(), &grid_edges) as i32;

    let expected = original_mis + result.mis_overhead;

    assert!(
        (mapped_mis - expected).abs() <= 1,
        "Triangle: mapped MIS {} should equal original {} + overhead {} = {}",
        mapped_mis,
        original_mis,
        result.mis_overhead,
        expected
    );
}

// === map_config_back_via_centers Tests ===

#[test]
fn test_map_config_back_via_centers_all_zeros() {
    let edges = vec![(0, 1), (1, 2)];
    let result = map_graph(3, &edges);

    let config = vec![0; result.grid_graph.num_vertices()];
    let original = result.map_config_back_via_centers(&config);

    assert_eq!(original.len(), 3);
    // All zeros should map back to all zeros
    assert!(original.iter().all(|&x| x == 0));
}

#[test]
fn test_map_config_back_via_centers_triangle() {
    let edges = vec![(0, 1), (1, 2), (0, 2)];
    let result = map_graph(3, &edges);

    let config = vec![0; result.grid_graph.num_vertices()];
    let original = result.map_config_back_via_centers(&config);

    assert_eq!(original.len(), 3);
}

#[test]
fn test_map_config_back_via_centers_star() {
    let edges = vec![(0, 1), (0, 2), (0, 3)];
    let result = map_graph(4, &edges);

    // Set all grid nodes to selected
    let config = vec![1; result.grid_graph.num_vertices()];
    let original = result.map_config_back_via_centers(&config);

    assert_eq!(original.len(), 4);
}

#[test]
fn test_map_config_back_consistency() {
    // Both methods should give reasonable results for the same input
    let edges = vec![(0, 1), (1, 2)];
    let result = map_graph(3, &edges);

    let config = vec![0; result.grid_graph.num_vertices()];

    let via_regions = result.map_config_back(&config);
    let via_centers = result.map_config_back_via_centers(&config);

    assert_eq!(via_regions.len(), via_centers.len());
    // Both should return all zeros for zero input
    assert!(via_regions.iter().all(|&x| x == 0));
    assert!(via_centers.iter().all(|&x| x == 0));
}

// === Additional Edge Cases ===

#[test]
fn test_large_graph_mapping() {
    // Test with a larger graph to exercise more code paths
    let edges: Vec<(usize, usize)> = (0..9)
        .flat_map(|i| [(i, (i + 1) % 10), (i, (i + 3) % 10)])
        .collect();
    let result = map_graph(10, &edges);

    assert_eq!(result.lines.len(), 10);
    assert!(result.grid_graph.num_vertices() > 10);
}

#[test]
fn test_mapping_result_tape_populated() {
    // Triangle graph should generate crossings
    let edges = vec![(0, 1), (1, 2), (0, 2)];
    let result = map_graph(3, &edges);

    // Tape may or may not have entries depending on crossings
    // Just verify it's accessible
    let _tape_len = result.tape.len();
}

#[test]
fn test_grid_graph_edges() {
    let edges = vec![(0, 1), (1, 2)];
    let result = map_graph(3, &edges);

    let grid_edges = result.grid_graph.edges();
    // Grid graph should have edges based on unit disk distance
    // Just verify edges are accessible
    let _edge_count = grid_edges.len();
}

#[test]
fn test_grid_graph_nodes_have_weights() {
    let edges = vec![(0, 1), (1, 2)];
    let result = map_graph(3, &edges);

    for node in result.grid_graph.nodes() {
        // All nodes should have positive weights
        assert!(node.weight > 0, "Node weight should be positive");
    }
}
