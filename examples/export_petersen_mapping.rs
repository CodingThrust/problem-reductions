//! Export Petersen graph and its grid mapping to JSON files for visualization.
//!
//! Run with: `cargo run --example export_petersen_mapping`
//!
//! Outputs:
//! - docs/paper/petersen_source.json - The original Petersen graph
//! - docs/paper/petersen_square_weighted.json - Weighted square lattice (King's subgraph)
//! - docs/paper/petersen_square_unweighted.json - Unweighted square lattice
//! - docs/paper/petersen_triangular.json - Weighted triangular lattice

use problemreductions::rules::unitdiskmapping::{ksg, triangular};
use problemreductions::topology::{Graph, GridGraph};
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

/// Write JSON to file with pretty formatting.
fn write_json<T: Serialize>(data: &T, path: &Path) {
    let json = serde_json::to_string_pretty(data).expect("Failed to serialize to JSON");
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("Failed to create output directory");
    }
    fs::write(path, json).expect("Failed to write JSON file");
    println!("Wrote: {}", path.display());
}

/// Create a GridMapping from a MappingResult by using the actual grid_graph.
fn make_grid_mapping<T>(result: &ksg::MappingResult<T>, weighted: bool) -> GridMapping {
    GridMapping {
        grid_graph: result.grid_graph.clone(),
        mis_overhead: result.mis_overhead,
        padding: result.padding,
        spacing: result.spacing,
        weighted,
    }
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

    // Map to weighted King's subgraph (square lattice)
    let square_weighted_result = ksg::map_weighted(num_vertices, &petersen_edges);
    let square_weighted = make_grid_mapping(&square_weighted_result, true);
    println!(
        "Square weighted: {}x{}, {} nodes, {} edges, overhead={}",
        square_weighted.grid_graph.size().0,
        square_weighted.grid_graph.size().1,
        square_weighted.grid_graph.num_vertices(),
        square_weighted.grid_graph.num_edges(),
        square_weighted.mis_overhead
    );
    write_json(
        &square_weighted,
        Path::new("docs/paper/petersen_square_weighted.json"),
    );

    // Map to unweighted King's subgraph (square lattice)
    let square_unweighted_result = ksg::map_unweighted(num_vertices, &petersen_edges);
    let square_unweighted = make_grid_mapping(&square_unweighted_result, false);
    println!(
        "Square unweighted: {}x{}, {} nodes, {} edges, overhead={}",
        square_unweighted.grid_graph.size().0,
        square_unweighted.grid_graph.size().1,
        square_unweighted.grid_graph.num_vertices(),
        square_unweighted.grid_graph.num_edges(),
        square_unweighted.mis_overhead
    );
    write_json(
        &square_unweighted,
        Path::new("docs/paper/petersen_square_unweighted.json"),
    );

    // Map to weighted triangular lattice
    let triangular_result = triangular::map_weighted(num_vertices, &petersen_edges);
    let triangular_weighted = make_grid_mapping(&triangular_result, true);
    println!(
        "Triangular weighted: {}x{}, {} nodes, {} edges, overhead={}",
        triangular_weighted.grid_graph.size().0,
        triangular_weighted.grid_graph.size().1,
        triangular_weighted.grid_graph.num_vertices(),
        triangular_weighted.grid_graph.num_edges(),
        triangular_weighted.mis_overhead
    );
    write_json(
        &triangular_weighted,
        Path::new("docs/paper/petersen_triangular.json"),
    );

    println!("\nSummary:");
    println!(
        "  Source: Petersen graph, n={}, MIS={}",
        num_vertices, petersen_mis
    );
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
