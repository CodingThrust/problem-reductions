//! Export Petersen graph and its grid mapping to JSON files for visualization.
//!
//! Run with: `cargo run --example export_petersen_mapping`
//!
//! Outputs:
//! - docs/paper/petersen_source.json - The original Petersen graph
//! - docs/paper/petersen_square_weighted.json - Weighted square lattice (King's subgraph)
//! - docs/paper/petersen_square_unweighted.json - Unweighted square lattice
//! - docs/paper/petersen_triangular.json - Weighted triangular lattice

use problemreductions::rules::unitdiskmapping::{map_graph, map_graph_triangular, MappingResult};
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

/// Grid mapping output for visualization.
#[derive(Serialize)]
struct GridMapping {
    grid_graph: GridGraph<i32>,
    mis_overhead: i32,
    padding: usize,
    spacing: usize,
    weighted: bool,
}

/// Generate grid mapping from copy lines with weights.
fn make_weighted_grid(result: &MappingResult, grid_type: GridType, radius: f64, triangular: bool) -> GridMapping {
    let mut all_nodes: Vec<GridNode<i32>> = Vec::new();

    // Collect all locations from each copy line with weights
    for line in &result.lines {
        let locs = if triangular {
            line.copyline_locations_triangular(result.padding, result.spacing)
        } else {
            line.copyline_locations(result.padding, result.spacing)
        };
        for (row, col, weight) in locs {
            all_nodes.push(GridNode::new(row as i32, col as i32, weight as i32));
        }
    }

    // Remove duplicates (same position), keeping max weight
    all_nodes.sort_by_key(|n| (n.row, n.col));
    let mut deduped: Vec<GridNode<i32>> = Vec::new();
    for node in all_nodes {
        if let Some(last) = deduped.last_mut() {
            if last.row == node.row && last.col == node.col {
                last.weight = last.weight.max(node.weight);
                continue;
            }
        }
        deduped.push(node);
    }

    let grid_graph = GridGraph::new(grid_type, result.grid_graph.size(), deduped, radius);

    GridMapping {
        grid_graph,
        mis_overhead: result.mis_overhead,
        padding: result.padding,
        spacing: result.spacing,
        weighted: true,
    }
}

/// Generate grid mapping from copy lines without weights (all weight=1).
fn make_unweighted_grid(result: &MappingResult, grid_type: GridType, radius: f64, triangular: bool) -> GridMapping {
    let mut all_nodes: Vec<GridNode<i32>> = Vec::new();

    // Collect all locations from each copy line, ignoring weights
    for line in &result.lines {
        let locs = if triangular {
            line.copyline_locations_triangular(result.padding, result.spacing)
        } else {
            line.copyline_locations(result.padding, result.spacing)
        };
        for (row, col, _weight) in locs {
            all_nodes.push(GridNode::new(row as i32, col as i32, 1));
        }
    }

    // Remove duplicates (same position)
    all_nodes.sort_by_key(|n| (n.row, n.col));
    all_nodes.dedup_by_key(|n| (n.row, n.col));

    let grid_graph = GridGraph::new(grid_type, result.grid_graph.size(), all_nodes, radius);

    GridMapping {
        grid_graph,
        mis_overhead: result.mis_overhead,
        padding: result.padding,
        spacing: result.spacing,
        weighted: false,
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

    // Create weighted square grid (radius 1.5 for 8-connectivity)
    let square_weighted = make_weighted_grid(&square_result, GridType::Square, 1.5, false);
    println!(
        "Square weighted: {}x{}, {} nodes, {} edges, overhead={}",
        square_weighted.grid_graph.size().0,
        square_weighted.grid_graph.size().1,
        square_weighted.grid_graph.num_vertices(),
        square_weighted.grid_graph.num_edges(),
        square_weighted.mis_overhead
    );
    write_json(&square_weighted, Path::new("docs/paper/petersen_square_weighted.json"));

    // Create unweighted square grid
    let square_unweighted = make_unweighted_grid(&square_result, GridType::Square, 1.5, false);
    println!(
        "Square unweighted: {}x{}, {} nodes, {} edges, overhead={}",
        square_unweighted.grid_graph.size().0,
        square_unweighted.grid_graph.size().1,
        square_unweighted.grid_graph.num_vertices(),
        square_unweighted.grid_graph.num_edges(),
        square_unweighted.mis_overhead
    );
    write_json(&square_unweighted, Path::new("docs/paper/petersen_square_unweighted.json"));

    // Map to triangular lattice
    let triangular_result = map_graph_triangular(num_vertices, &petersen_edges);

    // Create weighted triangular grid (radius 1.0 for triangular connectivity)
    let triangular_weighted = make_weighted_grid(&triangular_result, GridType::Triangular { offset_even_cols: false }, 1.0, true);
    println!(
        "Triangular weighted: {}x{}, {} nodes, overhead={}",
        triangular_weighted.grid_graph.size().0,
        triangular_weighted.grid_graph.size().1,
        triangular_weighted.grid_graph.num_vertices(),
        triangular_weighted.mis_overhead
    );
    write_json(&triangular_weighted, Path::new("docs/paper/petersen_triangular.json"));

    println!("\nSummary:");
    println!("  Source: Petersen graph, n={}, MIS={}", num_vertices, petersen_mis);
    println!(
        "  Square weighted: {} nodes, MIS = {} + {} = {}",
        square_weighted.grid_graph.num_vertices(),
        petersen_mis,
        square_weighted.mis_overhead,
        petersen_mis as i32 + square_weighted.mis_overhead
    );
    println!(
        "  Square unweighted: {} nodes, MIS = {} + {} = {}",
        square_unweighted.grid_graph.num_vertices(),
        petersen_mis,
        square_unweighted.mis_overhead,
        petersen_mis as i32 + square_unweighted.mis_overhead
    );
    println!(
        "  Triangular weighted: {} nodes, MIS = {} + {} = {}",
        triangular_weighted.grid_graph.num_vertices(),
        petersen_mis,
        triangular_weighted.mis_overhead,
        petersen_mis as i32 + triangular_weighted.mis_overhead
    );
}
