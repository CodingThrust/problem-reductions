use std::process::Command;

fn pred() -> Command {
    Command::new(env!("CARGO_BIN_EXE_pred"))
}

#[test]
fn test_help() {
    let output = pred().arg("--help").output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Explore NP-hard problem reductions"));
}

#[test]
fn test_graph_list() {
    let output = pred().args(["graph", "list"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("MaximumIndependentSet"));
    assert!(stdout.contains("QUBO"));
}

#[test]
fn test_graph_show() {
    let output = pred().args(["graph", "show", "MIS"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("MaximumIndependentSet"));
    assert!(stdout.contains("Reduces to"));
}

#[test]
fn test_graph_show_variants() {
    let output = pred()
        .args(["graph", "show", "MIS", "--variants"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Variants"));
}

#[test]
fn test_graph_path() {
    let output = pred()
        .args(["graph", "path", "MIS", "QUBO"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Path"));
    assert!(stdout.contains("step"));
}

#[test]
fn test_graph_export() {
    let tmp = std::env::temp_dir().join("pred_test_export.json");
    let output = pred()
        .args(["graph", "export", "--output", tmp.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(tmp.exists());
    let content = std::fs::read_to_string(&tmp).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert!(json["nodes"].is_array());
    std::fs::remove_file(&tmp).ok();
}

#[test]
fn test_schema() {
    let output = pred().args(["schema", "MIS"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("MaximumIndependentSet"));
    assert!(stdout.contains("Fields"));
}

#[test]
fn test_graph_list_json() {
    let tmp = std::env::temp_dir().join("pred_test_list.json");
    let output = pred()
        .args([
            "--json",
            "--output",
            tmp.to_str().unwrap(),
            "graph",
            "list",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(tmp.exists());
    let content = std::fs::read_to_string(&tmp).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert!(json["problems"].is_array());
    std::fs::remove_file(&tmp).ok();
}

#[test]
fn test_unknown_problem() {
    let output = pred().args(["graph", "show", "NonExistent"]).output().unwrap();
    assert!(!output.status.success());
}
