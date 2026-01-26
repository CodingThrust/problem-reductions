//! Export Petersen graph and its grid mapping to JSON files for visualization.
//!
//! Run with: `cargo run --example export_petersen_mapping`
//!
//! Outputs:
//! - docs/paper/petersen_source.json - The original Petersen graph
//! - docs/paper/petersen_square.json - Mapping to square lattice (King's graph)
//! - docs/paper/petersen_triangular.json - Mapping to triangular lattice

use problemreductions::rules::mapping::{map_graph, map_graph_triangular};
use problemreductions::topology::Graph;
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

    // Map to square lattice (King's graph)
    let square_result = map_graph(num_vertices, &petersen_edges);
    println!(
        "Square lattice: {}x{}, {} nodes, overhead={}",
        square_result.grid_graph.size().0,
        square_result.grid_graph.size().1,
        square_result.grid_graph.num_vertices(),
        square_result.mis_overhead
    );
    write_json(&square_result, Path::new("docs/paper/petersen_square.json"));

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
        "  Square: {} nodes, MIS = {} + {} = {}",
        square_result.grid_graph.num_vertices(),
        petersen_mis,
        square_result.mis_overhead,
        petersen_mis as i32 + square_result.mis_overhead
    );
    println!(
        "  Triangular: {} nodes, MIS = {} + {} = {}",
        triangular_result.grid_graph.num_vertices(),
        petersen_mis,
        triangular_result.mis_overhead,
        petersen_mis as i32 + triangular_result.mis_overhead
    );
}
