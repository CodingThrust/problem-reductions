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
fn test_list() {
    let output = pred().args(["list"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("MaximumIndependentSet"));
    assert!(stdout.contains("QUBO"));
}

#[test]
fn test_show() {
    let output = pred().args(["show", "MIS"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("MaximumIndependentSet"));
    assert!(stdout.contains("Reduces to"));
}

#[test]
fn test_show_variants() {
    let output = pred().args(["show", "MIS"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Variants"));
}

#[test]
fn test_path() {
    let output = pred().args(["path", "MIS", "QUBO"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Path"));
    assert!(stdout.contains("step"));
}

#[test]
fn test_path_save() {
    let tmp = std::env::temp_dir().join("pred_test_path.json");
    let output = pred()
        .args(["path", "MIS", "QUBO", "-o", tmp.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(tmp.exists());
    let content = std::fs::read_to_string(&tmp).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert!(json["path"].is_array());
    std::fs::remove_file(&tmp).ok();
}

#[test]
fn test_path_all() {
    let output = pred()
        .args(["path", "MIS", "QUBO", "--all"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Found"));
    assert!(stdout.contains("paths from"));
}

#[test]
fn test_path_all_save() {
    let dir = std::env::temp_dir().join("pred_test_all_paths");
    let _ = std::fs::remove_dir_all(&dir);
    let output = pred()
        .args([
            "path",
            "MIS",
            "QUBO",
            "--all",
            "-o",
            dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(dir.is_dir());
    let entries: Vec<_> = std::fs::read_dir(&dir).unwrap().collect();
    assert!(entries.len() > 1, "expected multiple path files");

    // Verify first file is valid JSON
    let first = dir.join("path_1.json");
    let content = std::fs::read_to_string(&first).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert!(json["path"].is_array());

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn test_export() {
    let tmp = std::env::temp_dir().join("pred_test_export.json");
    let output = pred()
        .args(["export-graph", tmp.to_str().unwrap()])
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
fn test_show_includes_fields() {
    let output = pred().args(["show", "MIS"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Fields"));
    assert!(stdout.contains("graph"));
    assert!(stdout.contains("weights"));
}

#[test]
fn test_list_json() {
    let tmp = std::env::temp_dir().join("pred_test_list.json");
    let output = pred()
        .args(["--output", tmp.to_str().unwrap(), "list"])
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
    let output = pred().args(["show", "NonExistent"]).output().unwrap();
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

#[test]
fn test_reduce_via_path() {
    // 1. Create problem
    let problem_file = std::env::temp_dir().join("pred_test_reduce_via_in.json");
    let create_out = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "MIS",
            "--edges",
            "0-1,1-2,2-3",
        ])
        .output()
        .unwrap();
    assert!(create_out.status.success());

    // 2. Generate path file
    let path_file = std::env::temp_dir().join("pred_test_reduce_via_path.json");
    let path_out = pred()
        .args([
            "path",
            "MIS",
            "QUBO",
            "-o",
            path_file.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(path_out.status.success());

    // 3. Reduce via path file
    let output_file = std::env::temp_dir().join("pred_test_reduce_via_out.json");
    let reduce_out = pred()
        .args([
            "-o",
            output_file.to_str().unwrap(),
            "reduce",
            problem_file.to_str().unwrap(),
            "--to",
            "QUBO",
            "--via",
            path_file.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(
        reduce_out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&reduce_out.stderr)
    );
    assert!(output_file.exists());

    let content = std::fs::read_to_string(&output_file).unwrap();
    let bundle: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert_eq!(bundle["source"]["type"], "MaximumIndependentSet");
    assert_eq!(bundle["target"]["type"], "QUBO");

    std::fs::remove_file(&problem_file).ok();
    std::fs::remove_file(&path_file).ok();
    std::fs::remove_file(&output_file).ok();
}

#[test]
fn test_create_mis() {
    let output_file = std::env::temp_dir().join("pred_test_create_mis.json");
    let output = pred()
        .args([
            "-o",
            output_file.to_str().unwrap(),
            "create",
            "MIS",
            "--edges",
            "0-1,1-2,2-3",
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
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert_eq!(json["type"], "MaximumIndependentSet");
    assert!(json["data"].is_object());

    std::fs::remove_file(&output_file).ok();
}

#[test]
fn test_create_then_evaluate() {
    // Create a problem
    let problem_file = std::env::temp_dir().join("pred_test_create_eval.json");
    let create_output = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "MIS",
            "--edges",
            "0-1,1-2,2-3",
            "--weights",
            "1,1,1,1",
        ])
        .output()
        .unwrap();
    assert!(
        create_output.status.success(),
        "create stderr: {}",
        String::from_utf8_lossy(&create_output.stderr)
    );

    // Evaluate with the created problem
    let eval_output = pred()
        .args([
            "evaluate",
            problem_file.to_str().unwrap(),
            "--config",
            "1,0,1,0",
        ])
        .output()
        .unwrap();
    assert!(
        eval_output.status.success(),
        "evaluate stderr: {}",
        String::from_utf8_lossy(&eval_output.stderr)
    );
    let stdout = String::from_utf8(eval_output.stdout).unwrap();
    assert!(stdout.contains("Valid"));

    std::fs::remove_file(&problem_file).ok();
}

#[test]
fn test_create_sat() {
    let output_file = std::env::temp_dir().join("pred_test_create_sat.json");
    let output = pred()
        .args([
            "-o",
            output_file.to_str().unwrap(),
            "create",
            "SAT",
            "--num-vars",
            "3",
            "--clauses",
            "1,2;-1,3",
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
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert_eq!(json["type"], "Satisfiability");

    std::fs::remove_file(&output_file).ok();
}

#[test]
fn test_create_qubo() {
    let output_file = std::env::temp_dir().join("pred_test_create_qubo.json");
    let output = pred()
        .args([
            "-o",
            output_file.to_str().unwrap(),
            "create",
            "QUBO",
            "--matrix",
            "1,0.5;0.5,2",
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
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert_eq!(json["type"], "QUBO");

    std::fs::remove_file(&output_file).ok();
}
