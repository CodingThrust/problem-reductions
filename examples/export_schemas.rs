//! Export problem schemas to a JSON file.
//!
//! Run with: `cargo run --example export_schemas`

use problemreductions::registry::collect_schemas;
use std::path::Path;

fn main() {
    let schemas = collect_schemas();
    println!("Collected {} problem schemas", schemas.len());

    let output_path = Path::new("docs/paper/problem_schemas.json");
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent).expect("Failed to create output directory");
    }

    let json = serde_json::to_string(&schemas).expect("Failed to serialize");
    std::fs::write(output_path, &json).expect("Failed to write file");
    println!("Exported to: {}", output_path.display());
}
