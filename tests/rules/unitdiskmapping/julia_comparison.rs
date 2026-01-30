//! Tests comparing Rust mapping output with Julia's UnitDiskMapping.jl traces.
//!
//! Note: Julia uses 1-indexed coordinates and vertices, Rust uses 0-indexed.
//! This test converts Julia's 1-indexed coordinates to 0-indexed for comparison.

use problemreductions::rules::unitdiskmapping::map_graph;
use serde::Deserialize;
use std::collections::HashSet;
use std::fs;

#[derive(Debug, Deserialize)]
struct JuliaTrace {
    graph_name: String,
    num_vertices: usize,
    num_edges: usize,
    edges: Vec<(usize, usize)>,
    grid_size: (usize, usize),
    num_grid_nodes: usize,
    num_grid_nodes_before_simplifiers: usize,
    mis_overhead: i32,
    original_mis_size: f64,
    mapped_mis_size: f64,
    padding: usize,
    grid_nodes: Vec<GridNode>,
    copy_lines: Vec<CopyLineInfo>,
}

#[derive(Debug, Deserialize)]
struct GridNode {
    row: i32,
    col: i32,
    weight: i32,
}

#[derive(Debug, Deserialize)]
struct CopyLineInfo {
    vertex: usize,
    vslot: usize,
    hslot: usize,
    vstart: usize,
    vstop: usize,
    hstop: usize,
    locations: Vec<Location>,
}

#[derive(Debug, Deserialize)]
struct Location {
    row: i32,
    col: i32,
}

fn load_julia_trace(name: &str) -> JuliaTrace {
    let path = format!("tests/julia/{}_unweighted_trace.json", name);
    let content = fs::read_to_string(&path).expect(&format!("Failed to read {}", path));
    serde_json::from_str(&content).expect(&format!("Failed to parse {}", path))
}

/// Get edges from Julia trace (converted from 1-indexed to 0-indexed)
fn get_graph_edges(julia: &JuliaTrace) -> Vec<(usize, usize)> {
    julia.edges
        .iter()
        .map(|(u, v)| (u - 1, v - 1))  // Convert from 1-indexed to 0-indexed
        .collect()
}


/// Compare Rust and Julia mapping results for a given graph.
/// Julia uses 1-indexed coordinates, Rust uses 0-indexed.
fn compare_mapping(name: &str) {
    let julia = load_julia_trace(name);
    let edges = get_graph_edges(&julia);
    let num_vertices = julia.num_vertices;

    // Run Rust mapping
    let rust_result = map_graph(num_vertices, &edges);

    // Collect Rust grid nodes from copyline_locations (0-indexed)
    let mut rust_nodes: HashSet<(i32, i32)> = HashSet::new();
    for line in &rust_result.lines {
        for (row, col, _weight) in line.copyline_locations(rust_result.padding, rust_result.spacing) {
            rust_nodes.insert((row as i32, col as i32));
        }
    }

    // Collect Julia grid nodes (both use same 1-indexed coordinate system)
    let julia_nodes: HashSet<(i32, i32)> = julia.grid_nodes
        .iter()
        .map(|n| (n.row, n.col))
        .collect();

    // Also collect from copy_line locations (more accurate source)
    let julia_copyline_nodes: HashSet<(i32, i32)> = julia.copy_lines
        .iter()
        .flat_map(|cl| cl.locations.iter().map(|loc| (loc.row, loc.col)))
        .collect();

    println!("\n=== {} ===", name);
    println!("Julia: {} vertices, {} edges", julia.num_vertices, julia.num_edges);
    println!("Rust:  {} vertices, {} edges", num_vertices, edges.len());

    println!("\nGrid size:");
    println!("  Julia: {:?}", julia.grid_size);
    println!("  Rust:  {:?}", rust_result.grid_graph.size());

    println!("\nGrid nodes:");
    println!("  Julia (grid_nodes): {}", julia.num_grid_nodes);
    println!("  Julia (from copylines): {}", julia_copyline_nodes.len());
    println!("  Rust:  {}", rust_nodes.len());

    println!("\nMIS overhead:");
    println!("  Julia: {}", julia.mis_overhead);
    println!("  Rust:  {}", rust_result.mis_overhead);

    // Compare using Julia's copyline locations (more reliable)
    let only_in_julia: Vec<_> = julia_copyline_nodes.difference(&rust_nodes).collect();
    let only_in_rust: Vec<_> = rust_nodes.difference(&julia_copyline_nodes).collect();

    if !only_in_julia.is_empty() {
        println!("\nNodes only in Julia ({}):", only_in_julia.len());
        let mut sorted: Vec<_> = only_in_julia.iter().copied().collect();
        sorted.sort();
        for &(r, c) in sorted.iter().take(10) {
            println!("  ({}, {})", r, c);
        }
        if sorted.len() > 10 {
            println!("  ... and {} more", sorted.len() - 10);
        }
    }

    if !only_in_rust.is_empty() {
        println!("\nNodes only in Rust ({}):", only_in_rust.len());
        let mut sorted: Vec<_> = only_in_rust.iter().copied().collect();
        sorted.sort();
        for &(r, c) in sorted.iter().take(10) {
            println!("  ({}, {})", r, c);
        }
        if sorted.len() > 10 {
            println!("  ... and {} more", sorted.len() - 10);
        }
    }

    // Compare copy lines (adjusting for 1-indexed vertex in Julia)
    println!("\nCopy lines comparison:");
    for julia_line in &julia.copy_lines {
        // Julia vertex is 1-indexed, convert to 0-indexed
        let julia_vertex_0idx = julia_line.vertex - 1;
        let rust_line = rust_result.lines.iter().find(|l| l.vertex == julia_vertex_0idx);
        if let Some(rl) = rust_line {
            let matches = rl.vslot == julia_line.vslot
                && rl.hslot == julia_line.hslot
                && rl.vstart == julia_line.vstart
                && rl.vstop == julia_line.vstop
                && rl.hstop == julia_line.hstop;
            if !matches {
                println!("  v{} (Julia v{}) MISMATCH:", julia_vertex_0idx, julia_line.vertex);
                println!("    Julia: vslot={}, hslot={}, vstart={}, vstop={}, hstop={}",
                    julia_line.vslot, julia_line.hslot, julia_line.vstart, julia_line.vstop, julia_line.hstop);
                println!("    Rust:  vslot={}, hslot={}, vstart={}, vstop={}, hstop={}",
                    rl.vslot, rl.hslot, rl.vstart, rl.vstop, rl.hstop);
            } else {
                println!("  v{} OK", julia_vertex_0idx);
            }
        } else {
            println!("  v{} (Julia v{}) missing in Rust!", julia_vertex_0idx, julia_line.vertex);
        }
    }

    // Assertions
    assert_eq!(julia.grid_size, rust_result.grid_graph.size(),
        "{}: Grid size mismatch", name);
    assert_eq!(julia.mis_overhead, rust_result.mis_overhead,
        "{}: MIS overhead mismatch", name);
    assert_eq!(julia_copyline_nodes.len(), rust_nodes.len(),
        "{}: Grid node count mismatch (Julia={}, Rust={})", name, julia_copyline_nodes.len(), rust_nodes.len());
    assert!(only_in_julia.is_empty() && only_in_rust.is_empty(),
        "{}: Grid node positions don't match", name);
}

#[test]
fn test_julia_comparison_bull() {
    compare_mapping("bull");
}

#[test]
fn test_julia_comparison_diamond() {
    compare_mapping("diamond");
}

#[test]
fn test_julia_comparison_house() {
    compare_mapping("house");
}

#[test]
fn test_julia_comparison_petersen() {
    compare_mapping("petersen");
}
