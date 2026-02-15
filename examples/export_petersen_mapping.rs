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
//! - `docs/paper/static/petersen_source.json` - The original Petersen graph
//! - `docs/paper/static/petersen_square_weighted.json` - Weighted King's subgraph
//! - `docs/paper/static/petersen_square_unweighted.json` - Unweighted King's subgraph
//! - `docs/paper/static/petersen_triangular.json` - Weighted triangular lattice

use problemreductions::rules::unitdiskmapping::{ksg, triangular, MappingResult};
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

/// Grid mapping visualization data (flat format for Typst rendering).
#[derive(Serialize)]
struct GridVisualization {
    nodes: Vec<NodeData>,
    edges: Vec<(usize, usize)>,
    mis_overhead: i32,
    padding: usize,
    spacing: usize,
    weighted: bool,
}

#[derive(Serialize)]
struct NodeData {
    row: i32,
    col: i32,
    weight: i32,
}

impl GridVisualization {
    fn from_result<T>(result: &MappingResult<T>, weighted: bool) -> Self {
        let nodes: Vec<NodeData> = result
            .positions
            .iter()
            .zip(result.node_weights.iter())
            .map(|(&(row, col), &weight)| NodeData { row, col, weight })
            .collect();
        let edges = result.edges();
        GridVisualization {
            nodes,
            edges,
            mis_overhead: result.mis_overhead,
            padding: result.padding,
            spacing: result.spacing,
            weighted,
        }
    }
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
    write_json(&source, Path::new("docs/paper/static/petersen_source.json"));

    println!("\n=== Mapping to Grid Graphs ===\n");

    // Map to weighted King's subgraph (square lattice)
    println!("1. King's Subgraph (Weighted)");
    let square_weighted_result = ksg::map_weighted(num_vertices, &petersen_edges);
    let square_weighted_viz = GridVisualization::from_result(&square_weighted_result, true);
    println!(
        "   Vertices: {}, Edges: {}",
        square_weighted_viz.nodes.len(),
        square_weighted_viz.edges.len()
    );
    println!(
        "   MIS overhead Δ: {}",
        square_weighted_result.mis_overhead
    );
    println!(
        "   MIS(grid) = MIS(source) + Δ = {} + {} = {}",
        petersen_mis,
        square_weighted_result.mis_overhead,
        petersen_mis as i32 + square_weighted_result.mis_overhead
    );
    write_json(
        &square_weighted_viz,
        Path::new("docs/paper/static/petersen_square_weighted.json"),
    );

    // Map to unweighted King's subgraph (square lattice)
    println!("\n2. King's Subgraph (Unweighted)");
    let square_unweighted_result = ksg::map_unweighted(num_vertices, &petersen_edges);
    let square_unweighted_viz = GridVisualization::from_result(&square_unweighted_result, false);
    println!(
        "   Vertices: {}, Edges: {}",
        square_unweighted_viz.nodes.len(),
        square_unweighted_viz.edges.len()
    );
    println!(
        "   MIS overhead Δ: {}",
        square_unweighted_result.mis_overhead
    );
    println!(
        "   MIS(grid) = MIS(source) + Δ = {} + {} = {}",
        petersen_mis,
        square_unweighted_result.mis_overhead,
        petersen_mis as i32 + square_unweighted_result.mis_overhead
    );
    write_json(
        &square_unweighted_viz,
        Path::new("docs/paper/static/petersen_square_unweighted.json"),
    );

    // Map to weighted triangular lattice
    println!("\n3. Triangular Lattice (Weighted)");
    let triangular_result = triangular::map_weighted(num_vertices, &petersen_edges);
    let triangular_viz = GridVisualization::from_result(&triangular_result, true);
    println!(
        "   Vertices: {}, Edges: {}",
        triangular_viz.nodes.len(),
        triangular_viz.edges.len()
    );
    println!("   MIS overhead Δ: {}", triangular_result.mis_overhead);
    println!(
        "   MIS(grid) = MIS(source) + Δ = {} + {} = {}",
        petersen_mis,
        triangular_result.mis_overhead,
        petersen_mis as i32 + triangular_result.mis_overhead
    );
    write_json(
        &triangular_viz,
        Path::new("docs/paper/static/petersen_triangular.json"),
    );

    println!("\n=== Summary ===\n");
    println!("Source: Petersen graph (10 vertices, MIS = 4)");
    println!();
    println!(
        "King's subgraph (weighted):   {} vertices, MIS = {} (overhead Δ = {})",
        square_weighted_viz.nodes.len(),
        petersen_mis as i32 + square_weighted_result.mis_overhead,
        square_weighted_result.mis_overhead
    );
    println!(
        "King's subgraph (unweighted): {} vertices, MIS = {} (overhead Δ = {})",
        square_unweighted_viz.nodes.len(),
        petersen_mis as i32 + square_unweighted_result.mis_overhead,
        square_unweighted_result.mis_overhead
    );
    println!(
        "Triangular lattice (weighted): {} vertices, MIS = {} (overhead Δ = {})",
        triangular_viz.nodes.len(),
        petersen_mis as i32 + triangular_result.mis_overhead,
        triangular_result.mis_overhead
    );

    println!("\n✓ Unit disk mapping demonstrated successfully");
    println!("  JSON files exported for paper visualization");
}
