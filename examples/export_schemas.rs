//! Export problem schemas to a JSON file.
//!
//! Run with: `cargo run --example export_schemas [output_path]`

use problemreductions::registry::{collect_schemas, ProblemSchemaEntry};
use std::path::{Path, PathBuf};

pub fn run(output_path: &Path) {
    let schemas = collect_schemas();
    println!("Collected {} problem schemas", schemas.len());

    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent).expect("Failed to create output directory");
    }

    // Source links are documentation metadata, not part of the public schema type.
    let schemas: Vec<_> = schemas
        .into_iter()
        .map(|schema| {
            let entry = inventory::iter::<ProblemSchemaEntry>
                .into_iter()
                .find(|entry| entry.name == schema.name)
                .expect("Collected schema must have a registered entry");
            let mut value = serde_json::to_value(schema).expect("Failed to serialize schema");
            value["module_path"] = serde_json::json!(entry.module_path);
            value
        })
        .collect();
    let json = serde_json::to_string_pretty(&schemas).expect("Failed to serialize");
    std::fs::write(output_path, &json).expect("Failed to write file");
    println!("Exported to: {}", output_path.display());
}

fn main() {
    let output_path = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("docs/src/reductions/problem_schemas.json"));
    run(&output_path);
}
