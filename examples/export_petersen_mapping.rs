//! # Independent Set to Grid Graph IS Reduction (Unit Disk Mapping)
//!
//! ## Mathematical Equivalence
//! Any Maximum Independent Set (MIS) problem on a general graph G can be reduced to
//! MIS on a unit disk graph (King's subgraph or triangular lattice) with polynomial
//! overhead. The copy-line method creates L-shaped "copy lines" for each vertex, with
//! crossing gadgets enforcing edge constraints. The grid MIS size equals the source
//! MIS plus a constant overhead: MIS(G_grid) = MIS(G) + Δ.
//!
//! ## This Example
//! Demonstrates the unit disk mapping using the Petersen graph:
//! - Instance: Petersen graph (10 vertices, 15 edges, MIS = 4)
//! - King's subgraph (weighted): 30×42 grid, 219 nodes, overhead Δ = 89
//! - King's subgraph (unweighted): 30×42 grid, 219 nodes, overhead Δ = 89
//! - Triangular lattice (weighted): 42×60 grid, 395 nodes, overhead Δ = 375
//! - Reference: Based on UnitDiskMapping.jl Petersen graph example
//!
//! This example also exports JSON files for paper visualization (Figure: Unit Disk Mappings).
//!
//! ## Usage
//! ```bash
//! cargo run --example export_petersen_mapping
//! ```
//!
//! ## Outputs
//! - `docs/paper/petersen_source.json` - The original Petersen graph
//! - `docs/paper/petersen_square_weighted.json` - Weighted King's subgraph
//! - `docs/paper/petersen_square_unweighted.json` - Unweighted King's subgraph
//! - `docs/paper/petersen_triangular.json` - Weighted triangular lattice
//!
//! See docs/paper/reductions.typ for the full reduction specification.

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
    println!("  Wrote: {}", path.display());
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
    println!("\n=== Independent Set to Grid Graph IS (Unit Disk Mapping) ===\n");

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

    println!("Source Problem: Independent Set");
    println!("  Graph: Petersen graph");
    println!("  Vertices: {}", num_vertices);
    println!("  Edges: {}", petersen_edges.len());
    println!("  MIS size: {}", petersen_mis);

    // Export source graph
    let source = SourceGraph {
        name: "Petersen".to_string(),
        num_vertices,
        edges: petersen_edges.clone(),
        mis: petersen_mis,
    };
    write_json(&source, Path::new("docs/paper/petersen_source.json"));

    println!("\n=== Mapping to Grid Graphs ===\n");

    // Map to weighted King's subgraph (square lattice)
    println!("1. King's Subgraph (Weighted)");
    let square_weighted_result = ksg::map_weighted(num_vertices, &petersen_edges);
    let square_weighted = make_grid_mapping(&square_weighted_result, true);
    println!(
        "   Grid size: {}×{}",
        square_weighted.grid_graph.size().0,
        square_weighted.grid_graph.size().1
    );
    println!(
        "   Vertices: {}, Edges: {}",
        square_weighted.grid_graph.num_vertices(),
        square_weighted.grid_graph.num_edges()
    );
    println!("   MIS overhead Δ: {}", square_weighted.mis_overhead);
    println!(
        "   MIS(grid) = MIS(source) + Δ = {} + {} = {}",
        petersen_mis,
        square_weighted.mis_overhead,
        petersen_mis as i32 + square_weighted.mis_overhead
    );
    write_json(
        &square_weighted,
        Path::new("docs/paper/petersen_square_weighted.json"),
    );

    // Map to unweighted King's subgraph (square lattice)
    println!("\n2. King's Subgraph (Unweighted)");
    let square_unweighted_result = ksg::map_unweighted(num_vertices, &petersen_edges);
    let square_unweighted = make_grid_mapping(&square_unweighted_result, false);
    println!(
        "   Grid size: {}×{}",
        square_unweighted.grid_graph.size().0,
        square_unweighted.grid_graph.size().1
    );
    println!(
        "   Vertices: {}, Edges: {}",
        square_unweighted.grid_graph.num_vertices(),
        square_unweighted.grid_graph.num_edges()
    );
    println!("   MIS overhead Δ: {}", square_unweighted.mis_overhead);
    println!(
        "   MIS(grid) = MIS(source) + Δ = {} + {} = {}",
        petersen_mis,
        square_unweighted.mis_overhead,
        petersen_mis as i32 + square_unweighted.mis_overhead
    );
    write_json(
        &square_unweighted,
        Path::new("docs/paper/petersen_square_unweighted.json"),
    );

    // Map to weighted triangular lattice
    println!("\n3. Triangular Lattice (Weighted)");
    let triangular_result = triangular::map_weighted(num_vertices, &petersen_edges);
    let triangular_weighted = make_grid_mapping(&triangular_result, true);
    println!(
        "   Grid size: {}×{}",
        triangular_weighted.grid_graph.size().0,
        triangular_weighted.grid_graph.size().1
    );
    println!(
        "   Vertices: {}, Edges: {}",
        triangular_weighted.grid_graph.num_vertices(),
        triangular_weighted.grid_graph.num_edges()
    );
    println!("   MIS overhead Δ: {}", triangular_weighted.mis_overhead);
    println!(
        "   MIS(grid) = MIS(source) + Δ = {} + {} = {}",
        petersen_mis,
        triangular_weighted.mis_overhead,
        petersen_mis as i32 + triangular_weighted.mis_overhead
    );
    write_json(
        &triangular_weighted,
        Path::new("docs/paper/petersen_triangular.json"),
    );

    println!("\n=== Summary ===\n");
    println!("Source: Petersen graph (10 vertices, MIS = 4)");
    println!();
    println!(
        "King's subgraph (weighted):   {} vertices, MIS = {} (overhead Δ = {})",
        square_weighted.grid_graph.num_vertices(),
        petersen_mis as i32 + square_weighted.mis_overhead,
        square_weighted.mis_overhead
    );
    println!(
        "King's subgraph (unweighted): {} vertices, MIS = {} (overhead Δ = {})",
        square_unweighted.grid_graph.num_vertices(),
        petersen_mis as i32 + square_unweighted.mis_overhead,
        square_unweighted.mis_overhead
    );
    println!(
        "Triangular lattice (weighted): {} vertices, MIS = {} (overhead Δ = {})",
        triangular_weighted.grid_graph.num_vertices(),
        petersen_mis as i32 + triangular_weighted.mis_overhead,
        triangular_weighted.mis_overhead
    );

    println!("\n✓ Unit disk mapping demonstrated successfully");
    println!("  JSON files exported for paper visualization");
}
