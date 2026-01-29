//! Tests for triangular lattice mapping (src/rules/mapping/triangular.rs).

use super::common::{solve_mis, solve_weighted_grid_mis, solve_weighted_mis};
use problemreductions::rules::mapping::{
    map_graph_triangular, map_graph_triangular_with_order, trace_centers, MappingResult,
};
use problemreductions::topology::{smallgraph, Graph};

// === Basic Triangular Mapping Tests ===

#[test]
fn test_triangular_path_graph() {
    let edges = vec![(0, 1), (1, 2)];
    let result = map_graph_triangular(3, &edges);

    assert!(result.grid_graph.num_vertices() > 0);
    assert!(result.mis_overhead >= 0);
    assert_eq!(result.lines.len(), 3);
}

#[test]
fn test_triangular_complete_k4() {
    let edges = vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];
    let result = map_graph_triangular(4, &edges);

    assert!(result.grid_graph.num_vertices() > 4);
    assert_eq!(result.lines.len(), 4);
}

#[test]
fn test_triangular_single_vertex() {
    let edges: Vec<(usize, usize)> = vec![];
    let result = map_graph_triangular(1, &edges);

    assert_eq!(result.lines.len(), 1);
    assert!(result.grid_graph.num_vertices() > 0);
}

#[test]
fn test_triangular_empty_graph() {
    let edges: Vec<(usize, usize)> = vec![];
    let result = map_graph_triangular(3, &edges);

    assert!(result.grid_graph.num_vertices() > 0);
    assert_eq!(result.lines.len(), 3);
}

#[test]
fn test_triangular_with_custom_order() {
    let edges = vec![(0, 1), (1, 2)];
    let order = vec![2, 1, 0];
    let result = map_graph_triangular_with_order(3, &edges, &order);

    assert!(result.grid_graph.num_vertices() > 0);
    assert_eq!(result.lines.len(), 3);
}

#[test]
fn test_triangular_star_graph() {
    let edges = vec![(0, 1), (0, 2), (0, 3)];
    let result = map_graph_triangular(4, &edges);

    assert!(result.grid_graph.num_vertices() > 4);
    assert_eq!(result.lines.len(), 4);
}

#[test]
#[should_panic]
fn test_triangular_zero_vertices_panics() {
    let edges: Vec<(usize, usize)> = vec![];
    let _ = map_graph_triangular(0, &edges);
}

#[test]
fn test_triangular_offset_setting() {
    let edges = vec![(0, 1)];
    let result = map_graph_triangular(2, &edges);

    // Triangular mode uses spacing=6, padding=2
    assert_eq!(result.spacing, 6);
    assert_eq!(result.padding, 2);
}

#[test]
fn test_triangular_mapping_result_serialization() {
    let edges = vec![(0, 1), (1, 2)];
    let result = map_graph_triangular(3, &edges);

    let json = serde_json::to_string(&result).unwrap();
    let deserialized: MappingResult = serde_json::from_str(&json).unwrap();

    assert_eq!(result.mis_overhead, deserialized.mis_overhead);
    assert_eq!(result.lines.len(), deserialized.lines.len());
}

// === Standard Graphs Triangular ===

#[test]
fn test_map_standard_graphs_triangular() {
    let graph_names = ["bull", "petersen", "cubical", "house", "diamond"];

    for name in graph_names {
        let (n, edges) = smallgraph(name).unwrap();
        let result = map_graph_triangular(n, &edges);

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

// === MIS Overhead Verification ===

fn verify_mis_overhead(name: &str) -> bool {
    let (n, edges) = smallgraph(name).unwrap();
    let result = map_graph_triangular(n, &edges);

    // Calculate mapped weighted MIS using grid weights directly (without map_weights)
    let grid_edges = result.grid_graph.edges().to_vec();
    let grid_weights: Vec<i32> = (0..result.grid_graph.num_vertices())
        .map(|i| result.grid_graph.weight(i).copied().unwrap_or(1))
        .collect();
    let mapped_weighted_mis =
        solve_weighted_mis(result.grid_graph.num_vertices(), &grid_edges, &grid_weights);

    // When using grid weights directly (without map_weights), the relationship is:
    // mapped_mis == overhead
    // (map_weights would add original vertex weights at center locations)
    let diff = (mapped_weighted_mis - result.mis_overhead).abs();

    if diff > 1 {
        eprintln!(
            "{}: FAIL - overhead={}, mapped={}, diff={}",
            name, result.mis_overhead, mapped_weighted_mis, diff
        );
        false
    } else {
        true
    }
}

#[test]
fn test_triangular_mis_overhead_path_graph() {
    let edges = vec![(0, 1), (1, 2)];
    let n = 3;
    let result = map_graph_triangular(n, &edges);

    let original_mis = solve_mis(n, &edges) as i32;
    let mapped_mis = solve_weighted_grid_mis(&result) as i32;

    let expected = original_mis + result.mis_overhead;

    assert!(
        (mapped_mis - expected).abs() <= 1,
        "Triangular path: mapped {} should equal original {} + overhead {} = {}",
        mapped_mis,
        original_mis,
        result.mis_overhead,
        expected
    );
}

#[test]
fn test_triangular_mis_overhead_bull() {
    assert!(verify_mis_overhead("bull"));
}

#[test]
fn test_triangular_mis_overhead_diamond() {
    assert!(verify_mis_overhead("diamond"));
}

#[test]
fn test_triangular_mis_overhead_house() {
    assert!(verify_mis_overhead("house"));
}

#[test]
fn test_triangular_mis_overhead_petersen() {
    assert!(verify_mis_overhead("petersen"));
}

#[test]
fn test_triangular_mis_overhead_cubical() {
    assert!(verify_mis_overhead("cubical"));
}

#[test]
#[ignore] // Tutte is large, slow
fn test_triangular_mis_overhead_tutte() {
    assert!(verify_mis_overhead("tutte"));
}

// === Trace Centers Tests ===

#[test]
fn test_trace_centers_single_vertex() {
    let edges: Vec<(usize, usize)> = vec![];
    let result = map_graph_triangular(1, &edges);

    let centers = trace_centers(&result);
    assert_eq!(centers.len(), 1);
}

#[test]
fn test_trace_centers_path_graph() {
    let edges = vec![(0, 1), (1, 2)];
    let result = map_graph_triangular(3, &edges);

    let centers = trace_centers(&result);
    assert_eq!(centers.len(), 3);

    // Each center should be at a valid grid position
    for (i, &(row, col)) in centers.iter().enumerate() {
        assert!(row > 0, "Vertex {} center row should be positive", i);
        assert!(col > 0, "Vertex {} center col should be positive", i);
    }
}

#[test]
fn test_trace_centers_triangle() {
    let edges = vec![(0, 1), (1, 2), (0, 2)];
    let result = map_graph_triangular(3, &edges);

    let centers = trace_centers(&result);
    assert_eq!(centers.len(), 3);
}
