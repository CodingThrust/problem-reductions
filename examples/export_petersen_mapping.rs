//! Export Petersen graph and its grid mapping to JSON files for visualization.
//!
//! Run with: `cargo run --example export_petersen_mapping`
//!
//! Outputs:
//! - docs/paper/petersen_source.json - The original Petersen graph
//! - docs/paper/petersen_square.json - Mapping to square lattice (King's subgraph)
//! - docs/paper/petersen_triangular.json - Mapping to triangular lattice

use problemreductions::rules::mapping::{map_graph, map_graph_triangular, MappingResult};
use problemreductions::topology::{Graph, GridGraph, GridNode, GridType};
use serde::Serialize;
use std::fs;
use std::path::Path;

/// The Petersen graph in a serializable format.
#[derive(Serialize)]
struct SourceGraph {
    name: String,
    num_vertices: usize,
    edges: Vec<(usize, usize)>,
    mis: usize,
}

/// Dense King's Subgraph output for visualization.
#[derive(Serialize)]
struct DenseKSG {
    grid_graph: GridGraph<i32>,
    mis_overhead: i32,
    padding: usize,
    spacing: usize,
}

/// Generate dense King's subgraph from copy lines.
fn make_dense_ksg(result: &MappingResult, radius: f64) -> DenseKSG {
    let mut all_nodes: Vec<GridNode<i32>> = Vec::new();

    // Collect all dense locations from each copy line
    for line in &result.lines {
        for (row, col, weight) in line.dense_locations(result.padding, result.spacing) {
            all_nodes.push(GridNode::new(row as i32, col as i32, weight as i32));
        }
    }

    // Remove duplicates (same position)
    all_nodes.sort_by_key(|n| (n.row, n.col));
    all_nodes.dedup_by_key(|n| (n.row, n.col));

    let grid_graph = GridGraph::new(
        GridType::Square,
        result.grid_graph.size(),
        all_nodes,
        radius,
    );

    DenseKSG {
        grid_graph,
        mis_overhead: result.mis_overhead,
        padding: result.padding,
        spacing: result.spacing,
    }
}

/// Write JSON to file with pretty formatting.
fn write_json<T: Serialize>(data: &T, path: &Path) {
    let json = serde_json::to_string_pretty(data).expect("Failed to serialize to JSON");
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("Failed to create output directory");
    }
    fs::write(path, json).expect("Failed to write JSON file");
    println!("Wrote: {}", path.display());
}

fn main() {
    // Petersen graph: n=10, MIS=4
    // Outer pentagon: 0-1-2-3-4-0
    // Inner star: 5-7-9-6-8-5
    // Spokes: 0-5, 1-6, 2-7, 3-8, 4-9
    let petersen_edges = vec![
        // Outer pentagon
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 4),
        (4, 0),
        // Inner star (pentagram)
        (5, 7),
        (7, 9),
        (9, 6),
        (6, 8),
        (8, 5),
        // Spokes
        (0, 5),
        (1, 6),
        (2, 7),
        (3, 8),
        (4, 9),
    ];
    let num_vertices = 10;
    let petersen_mis = 4;

    // Export source graph
    let source = SourceGraph {
        name: "Petersen".to_string(),
        num_vertices,
        edges: petersen_edges.clone(),
        mis: petersen_mis,
    };
    write_json(&source, Path::new("docs/paper/petersen_source.json"));

    // Map to square lattice (King's subgraph)
    let square_result = map_graph(num_vertices, &petersen_edges);

    // Create dense King's subgraph for visualization (radius 1.5 for 8-connectivity)
    let dense_ksg = make_dense_ksg(&square_result, 1.5);
    println!(
        "King's subgraph: {}x{}, {} dense nodes, {} edges, overhead={}",
        dense_ksg.grid_graph.size().0,
        dense_ksg.grid_graph.size().1,
        dense_ksg.grid_graph.num_vertices(),
        dense_ksg.grid_graph.num_edges(),
        dense_ksg.mis_overhead
    );
    write_json(&dense_ksg, Path::new("docs/paper/petersen_square.json"));

    // Map to triangular lattice
    let triangular_result = map_graph_triangular(num_vertices, &petersen_edges);
    println!(
        "Triangular lattice: {}x{}, {} nodes, overhead={}",
        triangular_result.grid_graph.size().0,
        triangular_result.grid_graph.size().1,
        triangular_result.grid_graph.num_vertices(),
        triangular_result.mis_overhead
    );
    write_json(
        &triangular_result,
        Path::new("docs/paper/petersen_triangular.json"),
    );

    println!("\nSummary:");
    println!("  Source: Petersen graph, n={}, MIS={}", num_vertices, petersen_mis);
    println!(
        "  King's subgraph: {} nodes, {} edges, MIS = {} + {} = {}",
        dense_ksg.grid_graph.num_vertices(),
        dense_ksg.grid_graph.num_edges(),
        petersen_mis,
        dense_ksg.mis_overhead,
        petersen_mis as i32 + dense_ksg.mis_overhead
    );
    println!(
        "  Triangular: {} nodes, MIS = {} + {} = {}",
        triangular_result.grid_graph.num_vertices(),
        petersen_mis,
        triangular_result.mis_overhead,
        petersen_mis as i32 + triangular_result.mis_overhead
    );
}
