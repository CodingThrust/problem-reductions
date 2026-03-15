// Test remaining example binaries to keep them compiling and correct.
// Examples with `pub fn run()` are included directly; others are run as subprocesses.

// --- Chained reduction demo (has pub fn run()) ---

#[cfg(feature = "ilp-solver")]
#[allow(unused)]
mod chained_reduction_factoring_to_spinglass {
    include!("../../examples/chained_reduction_factoring_to_spinglass.rs");
}

#[cfg(feature = "ilp-solver")]
#[test]
fn test_chained_reduction_factoring_to_spinglass() {
    chained_reduction_factoring_to_spinglass::run();
}

// --- Subprocess tests for export utilities ---

fn run_example(name: &str) {
    let status = std::process::Command::new(env!("CARGO"))
        .args(["run", "--example", name, "--features", "ilp-highs"])
        .status()
        .unwrap_or_else(|e| panic!("Failed to run example {name}: {e}"));
    assert!(status.success(), "Example {name} failed with {status}");
}

#[test]
fn test_export_graph() {
    run_example("export_graph");
}

#[test]
fn test_export_schemas() {
    run_example("export_schemas");
}

#[test]
fn test_export_petersen_mapping() {
    run_example("export_petersen_mapping");
}

#[test]
fn test_reduction_knapsack_to_qubo() {
    let output_dir = std::env::temp_dir().join(format!(
        "pr-reduction-knapsack-to-qubo-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let status = std::process::Command::new(env!("CARGO"))
        .args(["run", "--example", "reduction_knapsack_to_qubo"])
        .env("PROBLEMREDUCTIONS_EXAMPLES_DIR", &output_dir)
        .status()
        .unwrap_or_else(|e| panic!("Failed to run example reduction_knapsack_to_qubo: {e}"));

    assert!(
        status.success(),
        "Example reduction_knapsack_to_qubo failed with {status}"
    );
    assert!(output_dir.join("knapsack_to_qubo.json").exists());
    let _ = std::fs::remove_dir_all(output_dir);
}

// Note: detect_isolated_problems and detect_unreachable_from_3sat are diagnostic
// tools that exit(1) when they find issues. They are run via `make` targets
// (topology-sanity-check), not as part of `cargo test`.

// Note: export_examples requires the `example-db` feature which is not enabled
// in standard CI test runs. It is exercised via `make examples`.
