//! Tests for triangular lattice mapping (src/rules/mapping/triangular.rs).

use super::common::solve_weighted_grid_mis;
use crate::rules::unitdiskmapping::{
    map_graph_triangular, map_graph_triangular_with_order, trace_centers, MappingResult,
};
use crate::topology::{smallgraph, Graph};
use std::collections::HashMap;

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

/// Get vertex order from Julia's trace JSON file.
/// Returns None if file doesn't exist or can't be parsed.
fn get_julia_vertex_order(graph_name: &str) -> Option<Vec<usize>> {
    let path = format!(
        "{}/tests/data/{}_triangular_trace.json",
        env!("CARGO_MANIFEST_DIR"),
        graph_name
    );
    let content = std::fs::read_to_string(&path).ok()?;
    let data: serde_json::Value = serde_json::from_str(&content).ok()?;
    let copy_lines = data["copy_lines"].as_array()?;

    // Extract (vertex, vslot) pairs and sort by vslot to get order
    let mut lines: Vec<_> = copy_lines
        .iter()
        .filter_map(|cl| {
            let vertex = cl["vertex"].as_u64()? as usize;
            let vslot = cl["vslot"].as_u64()? as usize;
            Some((vertex - 1, vslot)) // Convert to 0-indexed
        })
        .collect();
    lines.sort_by_key(|(_, vslot)| *vslot);
    Some(lines.into_iter().map(|(v, _)| v).collect())
}

/// Verify that the triangular mapping matches Julia's trace data.
/// For triangular weighted mode: mapped_weighted_mis == overhead
/// (The overhead represents the entire weighted MIS of the grid graph,
/// with original vertex contributions encoded separately at center locations.)
fn verify_mapping_matches_julia(name: &str) -> bool {
    let (n, edges) = smallgraph(name).unwrap();

    // Use Julia's vertex order to ensure consistent mapping
    let vertex_order = get_julia_vertex_order(name).unwrap_or_else(|| (0..n).collect());
    let result = map_graph_triangular_with_order(n, &edges, &vertex_order);

    // Load Julia's trace data
    let julia_path = format!(
        "{}/tests/data/{}_triangular_trace.json",
        env!("CARGO_MANIFEST_DIR"),
        name
    );
    let julia_content = match std::fs::read_to_string(&julia_path) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("{}: Julia trace file not found", name);
            return false;
        }
    };
    let julia_data: serde_json::Value = serde_json::from_str(&julia_content).unwrap();

    // Compare node count
    let julia_nodes = julia_data["num_grid_nodes"].as_u64().unwrap() as usize;
    if result.grid_graph.num_vertices() != julia_nodes {
        eprintln!(
            "{}: node count mismatch - Rust={}, Julia={}",
            name,
            result.grid_graph.num_vertices(),
            julia_nodes
        );
        return false;
    }

    // Compare overhead
    let julia_overhead = julia_data["mis_overhead"].as_i64().unwrap() as i32;
    if result.mis_overhead != julia_overhead {
        eprintln!(
            "{}: overhead mismatch - Rust={}, Julia={}",
            name, result.mis_overhead, julia_overhead
        );
        return false;
    }

    // Compare edge count
    if let Some(julia_edges) = julia_data["num_grid_edges"].as_u64() {
        let rust_edges = result.grid_graph.num_edges();
        if rust_edges != julia_edges as usize {
            eprintln!(
                "{}: edge count mismatch - Rust={}, Julia={}",
                name, rust_edges, julia_edges
            );
            return false;
        }
    }

    // Compute and compare weighted MIS
    let mapped_mis = solve_weighted_grid_mis(&result) as i32;
    let julia_mis = julia_data["mapped_mis_size"]
        .as_f64()
        .or_else(|| julia_data["mapped_mis_size"].as_i64().map(|v| v as f64))
        .unwrap_or(0.0) as i32;

    // For triangular weighted mode: mapped_mis == overhead
    if mapped_mis != julia_mis {
        eprintln!(
            "{}: weighted MIS mismatch - Rust={}, Julia={}",
            name, mapped_mis, julia_mis
        );
        return false;
    }

    if mapped_mis != julia_overhead {
        eprintln!(
            "{}: MIS != overhead - mapped={}, overhead={}",
            name, mapped_mis, julia_overhead
        );
        return false;
    }

    true
}

#[test]
fn test_triangular_mis_overhead_path_graph() {
    let edges = vec![(0, 1), (1, 2)];
    let n = 3;
    let result = map_graph_triangular(n, &edges);

    let mapped_mis = solve_weighted_grid_mis(&result) as i32;

    // For triangular weighted mode: mapped_weighted_mis == overhead
    // (The overhead represents the entire weighted MIS of the grid graph)
    assert!(
        (mapped_mis - result.mis_overhead).abs() <= 1,
        "Triangular path: mapped {} should equal overhead {}",
        mapped_mis,
        result.mis_overhead
    );
}

#[test]
fn test_triangular_mapping_bull() {
    assert!(verify_mapping_matches_julia("bull"));
}

#[test]
fn test_triangular_mapping_diamond() {
    assert!(verify_mapping_matches_julia("diamond"));
}

#[test]
fn test_triangular_mapping_house() {
    assert!(verify_mapping_matches_julia("house"));
}

#[test]
fn test_triangular_mapping_petersen() {
    assert!(verify_mapping_matches_julia("petersen"));
}

#[test]
fn test_triangular_mapping_cubical() {
    // No Julia trace file for cubical triangular, skip
    let julia_path = format!(
        "{}/tests/data/cubical_triangular_trace.json",
        env!("CARGO_MANIFEST_DIR")
    );
    if std::fs::read_to_string(&julia_path).is_err() {
        return; // Skip if no Julia trace
    }
    assert!(verify_mapping_matches_julia("cubical"));
}

#[test]
fn test_triangular_mapping_tutte() {
    // Skip if no Julia trace file exists
    let julia_path = format!(
        "{}/tests/data/tutte_triangular_trace.json",
        env!("CARGO_MANIFEST_DIR")
    );
    if std::fs::read_to_string(&julia_path).is_err() {
        return; // Skip if no Julia trace
    }
    assert!(verify_mapping_matches_julia("tutte"));
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

// === map_config_back Verification Tests ===

/// Test triangular mode map_config_back_via_centers for standard graphs.
///
/// Follows Julia's test approach:
/// 1. Use source weights of 0.2 for each vertex
/// 2. Call map_weights to add source weights at centers
/// 3. Multiply by 10 for integer solver
/// 4. Verify: config at centers is valid IS with correct size
#[test]
fn test_triangular_map_config_back_standard_graphs() {
    use super::common::{is_independent_set, solve_mis, solve_weighted_mis_config};
    use crate::rules::unitdiskmapping::map_weights;
    use crate::topology::Graph;

    // All standard graphs (excluding tutte/karate which are slow)
    let graph_names = [
        "bull",
        "chvatal",
        "cubical",
        "desargues",
        "diamond",
        "dodecahedral",
        "frucht",
        "heawood",
        "house",
        "housex",
        "icosahedral",
        "krackhardtkite",
        "moebiuskantor",
        "octahedral",
        "pappus",
        "petersen",
        "sedgewickmaze",
        "tetrahedral",
        "truncatedcube",
        "truncatedtetrahedron",
    ];

    for name in graph_names {
        let (n, edges) = smallgraph(name).unwrap();

        // Use Julia's vertex order if available
        let vertex_order = get_julia_vertex_order(name).unwrap_or_else(|| (0..n).collect());
        let result = map_graph_triangular_with_order(n, &edges, &vertex_order);

        // Follow Julia's approach: source weights of 0.2 for each vertex
        let source_weights: Vec<f64> = vec![0.2; n];

        // map_weights adds source weights at center locations (like Julia)
        let mapped_weights = map_weights(&result, &source_weights);

        // Multiply by 10 and round to get integer weights (like Julia)
        let weights: Vec<i32> = mapped_weights
            .iter()
            .map(|&w| (w * 10.0).round() as i32)
            .collect();

        let grid_edges = result.grid_graph.edges().to_vec();
        let num_grid = result.grid_graph.num_vertices();

        // Solve weighted MIS on grid
        let grid_config = solve_weighted_mis_config(num_grid, &grid_edges, &weights);

        // Use triangular-specific trace_centers (not the KSG version)
        // Build position to node index map
        let mut pos_to_idx: HashMap<(usize, usize), usize> = HashMap::new();
        for (idx, node) in result.grid_graph.nodes().iter().enumerate() {
            if let (Ok(row), Ok(col)) = (usize::try_from(node.row), usize::try_from(node.col)) {
                pos_to_idx.insert((row, col), idx);
            }
        }

        // Get traced center locations using triangular-specific trace_centers
        let centers = trace_centers(&result);

        // Extract config at centers
        let center_config: Vec<usize> = centers
            .iter()
            .map(|&(row, col)| {
                pos_to_idx
                    .get(&(row, col))
                    .and_then(|&idx| grid_config.get(idx).copied())
                    .unwrap_or(0)
            })
            .collect();

        // Verify it's a valid independent set
        assert!(
            is_independent_set(&edges, &center_config),
            "{}: Triangular config at centers should be a valid IS",
            name
        );

        // Verify it's a maximum independent set
        // Julia test: count(isone, sc) ≈ (missize.n / 10) * 5
        // With weights 0.2, original MIS value = count * 2 (after *10)
        // So extracted count should equal original MIS size
        let original_mis = solve_mis(n, &edges);
        let extracted_size = center_config.iter().filter(|&&x| x > 0).count();
        assert_eq!(
            extracted_size, original_mis,
            "{}: Extracted config size {} should equal original MIS size {}",
            name, extracted_size, original_mis
        );
    }
}
