//! Export the reduction graph to a JSON file.
//!
//! Run with: `cargo run --example export_graph [output_path]`

use problemreductions::rules::ReductionGraph;
use std::path::PathBuf;

fn main() {
    let graph = ReductionGraph::new();

    // Print statistics
    println!("Reduction Graph Statistics:");
    println!("  Problem types: {}", graph.num_types());
    println!("  Reductions: {}", graph.num_reductions());

    // Export to JSON (single source for both mdBook and paper)
    let output_path = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("docs/src/reductions/reduction_graph.json"));

    // Create parent directories if needed
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent).expect("Failed to create output directory");
    }

    graph
        .to_json_file(output_path)
        .expect("Failed to write JSON file");

    println!("\nExported to: {}", output_path.display());

    // Also print the JSON to stdout
    println!("\nJSON content:");
    println!("{}", graph.to_json_string().unwrap());
}
