// Test example behavior directly without spawning nested Cargo builds.

use std::path::PathBuf;

// --- Chained reduction demo (has pub fn run()) ---

#[allow(unused)]
mod chained_reduction_factoring_to_spinglass {
    include!("../../examples/chained_reduction_factoring_to_spinglass.rs");
}

#[test]
fn test_chained_reduction_factoring_to_spinglass() {
    chained_reduction_factoring_to_spinglass::run().unwrap();
}

#[allow(dead_code)]
#[path = "../../examples/export_graph.rs"]
mod export_graph;

#[allow(dead_code)]
#[path = "../../examples/export_schemas.rs"]
mod export_schemas;

#[allow(dead_code)]
#[path = "../../examples/export_petersen_mapping.rs"]
mod export_petersen_mapping;

fn temp_output_dir(name: &str) -> PathBuf {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Clock must be after UNIX_EPOCH")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "problemreductions_{name}_{}_{}",
        std::process::id(),
        timestamp
    ))
}

#[test]
fn test_export_graph() {
    let output_dir = temp_output_dir("export_graph");
    let output_path = output_dir.join("reduction_graph.json");
    export_graph::run(&output_path);
    assert!(output_path.is_file());
    std::fs::remove_dir_all(output_dir).unwrap();
}

#[test]
fn test_export_schemas() {
    let output_dir = temp_output_dir("export_schemas");
    let output_path = output_dir.join("problem_schemas.json");
    export_schemas::run(&output_path);
    assert!(output_path.is_file());
    std::fs::remove_dir_all(output_dir).unwrap();
}

#[test]
fn test_export_petersen_mapping() {
    let output_dir = temp_output_dir("export_petersen_mapping");
    export_petersen_mapping::run(&output_dir);
    for filename in [
        "petersen_source.json",
        "petersen_square_weighted.json",
        "petersen_square_unweighted.json",
        "petersen_triangular.json",
    ] {
        assert!(output_dir.join(filename).is_file());
    }
    std::fs::remove_dir_all(output_dir).unwrap();
}

// Note: detect_isolated_problems and detect_unreachable_from_3sat are diagnostic
// tools that exit(1) when they find issues. They are run via `make` targets
// (topology-sanity-check), not as part of `cargo test`.

// Note: export_examples is exercised by `make paper` with the example-db feature.
