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
        .args(["graph", "show", "MIS"])
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
        .args(["graph", "export", tmp.to_str().unwrap()])
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

#[test]
fn test_evaluate() {
    let problem_json = r#"{
        "type": "MaximumIndependentSet",
        "variant": {"graph": "SimpleGraph", "weight": "i32"},
        "data": {
            "graph": {"inner": {"nodes": [null, null, null, null], "node_holes": [], "edge_property": "undirected", "edges": [[0,1,null],[1,2,null],[2,3,null]]}},
            "weights": [1, 1, 1, 1]
        }
    }"#;
    let tmp = std::env::temp_dir().join("pred_test_evaluate.json");
    std::fs::write(&tmp, problem_json).unwrap();

    let output = pred()
        .args(["evaluate", tmp.to_str().unwrap(), "--config", "1,0,1,0"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Valid"));
    std::fs::remove_file(&tmp).ok();
}

#[test]
fn test_evaluate_sat() {
    let problem_json = r#"{
        "type": "Satisfiability",
        "data": {
            "num_vars": 3,
            "clauses": [{"literals": [1, 2]}]
        }
    }"#;
    let tmp = std::env::temp_dir().join("pred_test_eval_sat.json");
    std::fs::write(&tmp, problem_json).unwrap();

    let output = pred()
        .args(["evaluate", tmp.to_str().unwrap(), "--config", "1,1,0"])
        .output()
        .unwrap();
    assert!(output.status.success());
    std::fs::remove_file(&tmp).ok();
}

#[test]
fn test_reduce() {
    let problem_json = r#"{
        "type": "MIS",
        "variant": {"graph": "SimpleGraph", "weight": "i32"},
        "data": {
            "graph": {"inner": {"nodes": [null, null, null, null], "node_holes": [], "edge_property": "undirected", "edges": [[0,1,null],[1,2,null],[2,3,null]]}},
            "weights": [1, 1, 1, 1]
        }
    }"#;
    let input = std::env::temp_dir().join("pred_test_reduce_in.json");
    let output_file = std::env::temp_dir().join("pred_test_reduce_out.json");
    std::fs::write(&input, problem_json).unwrap();

    let output = pred()
        .args([
            "--json",
            "-o",
            output_file.to_str().unwrap(),
            "reduce",
            input.to_str().unwrap(),
            "--to",
            "QUBO",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output_file.exists());

    let content = std::fs::read_to_string(&output_file).unwrap();
    let bundle: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert_eq!(bundle["source"]["type"], "MaximumIndependentSet");
    assert_eq!(bundle["target"]["type"], "QUBO");
    assert!(bundle["path"].is_array());

    std::fs::remove_file(&input).ok();
    std::fs::remove_file(&output_file).ok();
}
