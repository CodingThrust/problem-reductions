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
fn test_list_includes_undirected_two_commodity_integral_flow() {
    let output = pred().args(["list"]).output().unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("UndirectedTwoCommodityIntegralFlow"));
}

#[test]
fn test_list_rules() {
    let output = pred().args(["list", "--rules"]).output().unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Registered reduction rules:"));
    assert!(stdout.contains("Source"));
    assert!(stdout.contains("Target"));
    assert!(stdout.contains("Overhead"));
    // Should contain a known reduction
    assert!(
        stdout.contains("MaximumIndependentSet"),
        "should list MIS reductions"
    );
}

#[test]
fn test_list_rules_json() {
    let output = pred().args(["list", "--rules", "--json"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert!(json["num_rules"].as_u64().unwrap() > 0);
    let rules = json["rules"].as_array().unwrap();
    assert!(!rules.is_empty());
    assert!(rules[0]["source"].is_string());
    assert!(rules[0]["target"].is_string());
    assert!(rules[0]["overhead"].is_string());
}

#[test]
fn test_show() {
    let output = pred().args(["show", "MIS"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("MaximumIndependentSet"));
    assert!(stdout.contains("Outgoing reductions"));
}

#[test]
fn test_show_undirected_two_commodity_integral_flow() {
    let output = pred()
        .args(["show", "UndirectedTwoCommodityIntegralFlow"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("UndirectedTwoCommodityIntegralFlow"));
    assert!(stdout.contains("capacities"));
    assert!(stdout.contains("source_1"));
    assert!(stdout.contains("requirement_2"));
}

#[test]
fn test_show_variant_info() {
    let output = pred().args(["show", "MIS"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    // Bare MIS shows default variant with complexity
    assert!(
        stdout.contains("Complexity:"),
        "should show complexity: {stdout}"
    );
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
        .args(["path", "MIS", "QUBO", "--all", "-o", dir.to_str().unwrap()])
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
        .args(["export-graph", "-o", tmp.to_str().unwrap()])
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
fn test_export_stdout() {
    let output = pred().args(["export-graph"]).output().unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    // Without -o, export-graph prints human-readable summary to stdout
    assert!(
        stdout.contains("Reduction graph:"),
        "stdout should contain summary, got: {stdout}"
    );
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
    assert!(json["variants"].is_array());
    std::fs::remove_file(&tmp).ok();
}

#[test]
fn test_unknown_problem() {
    let output = pred().args(["show", "NonExistent"]).output().unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("pred list"),
        "Unknown problem error should suggest `pred list`, got: {stderr}"
    );
}

#[test]
fn test_unknown_problem_suggests() {
    // "MISs" is close to "MIS" alias -> should suggest MaximumIndependentSet
    let output = pred().args(["show", "MISs"]).output().unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Did you mean"),
        "Close misspelling should trigger 'Did you mean', got: {stderr}"
    );
    assert!(
        stderr.contains("pred list"),
        "Should always suggest `pred list`, got: {stderr}"
    );
}

#[test]
fn test_unknown_problem_no_match() {
    // Totally unrelated name should still suggest pred list
    let output = pred().args(["show", "xyzxyzxyz"]).output().unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("pred list"),
        "Should suggest `pred list` even with no fuzzy matches, got: {stderr}"
    );
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
fn test_create_undirected_two_commodity_integral_flow() {
    let output = pred()
        .args([
            "create",
            "UndirectedTwoCommodityIntegralFlow",
            "--graph",
            "0-2,1-2,2-3",
            "--capacities",
            "1,1,2",
            "--source-1",
            "0",
            "--sink-1",
            "3",
            "--source-2",
            "1",
            "--sink-2",
            "3",
            "--requirement-1",
            "1",
            "--requirement-2",
            "1",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["type"], "UndirectedTwoCommodityIntegralFlow");
    assert_eq!(json["variant"], serde_json::json!({}));
    assert_eq!(json["data"]["capacities"], serde_json::json!([1, 1, 2]));
    assert_eq!(json["data"]["source_1"], 0);
    assert_eq!(json["data"]["sink_1"], 3);
    assert_eq!(json["data"]["source_2"], 1);
    assert_eq!(json["data"]["sink_2"], 3);
    assert_eq!(json["data"]["requirement_1"], 1);
    assert_eq!(json["data"]["requirement_2"], 1);
}

#[test]
fn test_create_undirected_two_commodity_integral_flow_missing_capacities_shows_usage() {
    let output = pred()
        .args([
            "create",
            "UndirectedTwoCommodityIntegralFlow",
            "--graph",
            "0-2,1-2,2-3",
            "--source-1",
            "0",
            "--sink-1",
            "3",
            "--source-2",
            "1",
            "--sink-2",
            "3",
            "--requirement-1",
            "1",
            "--requirement-2",
            "1",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("requires --capacities"));
    assert!(stderr.contains("Usage: pred create UndirectedTwoCommodityIntegralFlow"));
}

#[test]
fn test_create_undirected_two_commodity_integral_flow_rejects_invalid_capacity_token() {
    let output = pred()
        .args([
            "create",
            "UndirectedTwoCommodityIntegralFlow",
            "--graph",
            "0-2,1-2,2-3",
            "--capacities",
            "1,x,2",
            "--source-1",
            "0",
            "--sink-1",
            "3",
            "--source-2",
            "1",
            "--sink-2",
            "3",
            "--requirement-1",
            "1",
            "--requirement-2",
            "1",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Invalid capacity `x`"));
    assert!(stderr.contains("Usage: pred create UndirectedTwoCommodityIntegralFlow"));
}

#[test]
fn test_create_undirected_two_commodity_integral_flow_rejects_wrong_capacity_count() {
    let output = pred()
        .args([
            "create",
            "UndirectedTwoCommodityIntegralFlow",
            "--graph",
            "0-2,1-2,2-3",
            "--capacities",
            "1,2",
            "--source-1",
            "0",
            "--sink-1",
            "3",
            "--source-2",
            "1",
            "--sink-2",
            "3",
            "--requirement-1",
            "1",
            "--requirement-2",
            "1",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Expected 3 capacities but got 2"));
    assert!(stderr.contains("Usage: pred create UndirectedTwoCommodityIntegralFlow"));
}

#[test]
fn test_create_undirected_two_commodity_integral_flow_rejects_oversized_capacity() {
    let oversized = ((usize::MAX as u128) + 1).to_string();
    let capacities = format!("1,1,{oversized}");
    let output = pred()
        .args([
            "create",
            "UndirectedTwoCommodityIntegralFlow",
            "--graph",
            "0-2,1-2,2-3",
            "--capacities",
            capacities.as_str(),
            "--source-1",
            "0",
            "--sink-1",
            "3",
            "--source-2",
            "1",
            "--sink-2",
            "3",
            "--requirement-1",
            "1",
            "--requirement-2",
            "1",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains(format!("Invalid capacity `{oversized}`").as_str()));
    assert!(stderr.contains("number too large to fit in target type"));
    assert!(stderr.contains("Usage: pred create UndirectedTwoCommodityIntegralFlow"));
}

#[test]
fn test_create_undirected_two_commodity_integral_flow_rejects_out_of_range_terminal() {
    let output = pred()
        .args([
            "create",
            "UndirectedTwoCommodityIntegralFlow",
            "--graph",
            "0-2,1-2,2-3",
            "--capacities",
            "1,1,2",
            "--source-1",
            "99",
            "--sink-1",
            "3",
            "--source-2",
            "1",
            "--sink-2",
            "3",
            "--requirement-1",
            "1",
            "--requirement-2",
            "1",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("source-1 must be less than num_vertices (4)"));
    assert!(stderr.contains("Usage: pred create UndirectedTwoCommodityIntegralFlow"));
    assert!(!stderr.contains("panicked at"), "stderr: {stderr}");
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
    // 1. Create problem (use explicit variant to match path resolution)
    let problem_file = std::env::temp_dir().join("pred_test_reduce_via_in.json");
    let create_out = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "MIS/SimpleGraph/i32",
            "--graph",
            "0-1,1-2,2-3",
            "--weights",
            "1,1,1,1",
        ])
        .output()
        .unwrap();
    assert!(create_out.status.success());

    // 2. Generate path file (use same variant as the problem)
    let path_file = std::env::temp_dir().join("pred_test_reduce_via_path.json");
    let path_out = pred()
        .args([
            "path",
            "MIS/SimpleGraph/i32",
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
fn test_reduce_via_infer_target() {
    // --via without --to: target is inferred from the path file
    let problem_file = std::env::temp_dir().join("pred_test_reduce_via_infer_in.json");
    let create_out = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "MIS/SimpleGraph/i32",
            "--graph",
            "0-1,1-2,2-3",
            "--weights",
            "1,1,1,1",
        ])
        .output()
        .unwrap();
    assert!(create_out.status.success());

    let path_file = std::env::temp_dir().join("pred_test_reduce_via_infer_path.json");
    let path_out = pred()
        .args([
            "path",
            "MIS/SimpleGraph/i32",
            "QUBO",
            "-o",
            path_file.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(path_out.status.success());

    let output_file = std::env::temp_dir().join("pred_test_reduce_via_infer_out.json");
    let reduce_out = pred()
        .args([
            "-o",
            output_file.to_str().unwrap(),
            "reduce",
            problem_file.to_str().unwrap(),
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

    let content = std::fs::read_to_string(&output_file).unwrap();
    let bundle: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert_eq!(bundle["source"]["type"], "MaximumIndependentSet");
    assert_eq!(bundle["target"]["type"], "QUBO");

    std::fs::remove_file(&problem_file).ok();
    std::fs::remove_file(&path_file).ok();
    std::fs::remove_file(&output_file).ok();
}

#[test]
fn test_reduce_via_rejects_target_variant_mismatch() {
    let problem_file = std::env::temp_dir().join("pred_test_reduce_via_variant_in.json");
    let create_out = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "MIS/SimpleGraph/i32",
            "--graph",
            "0-1,1-2,2-3",
            "--weights",
            "1,1,1,1",
        ])
        .output()
        .unwrap();
    assert!(create_out.status.success());

    let path_file = std::env::temp_dir().join("pred_test_reduce_via_variant_path.json");
    let path_out = pred()
        .args([
            "path",
            "MIS/SimpleGraph/i32",
            "ILP/bool",
            "-o",
            path_file.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(
        path_out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&path_out.stderr)
    );

    let reduce_out = pred()
        .args([
            "reduce",
            problem_file.to_str().unwrap(),
            "--to",
            "ILP/i32",
            "--via",
            path_file.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(
        !reduce_out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&reduce_out.stderr)
    );
    let stderr = String::from_utf8_lossy(&reduce_out.stderr);
    assert!(
        stderr.contains("ILP") && stderr.contains("i32") && stderr.contains("bool"),
        "expected variant mismatch details, got: {stderr}"
    );

    std::fs::remove_file(&problem_file).ok();
    std::fs::remove_file(&path_file).ok();
}

#[test]
fn test_reduce_missing_to_and_via() {
    let problem_file = std::env::temp_dir().join("pred_test_reduce_missing.json");
    let create_out = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "MIS",
            "--graph",
            "0-1",
        ])
        .output()
        .unwrap();
    assert!(create_out.status.success());

    let output = pred()
        .args(["reduce", problem_file.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--to") || stderr.contains("--via"));

    std::fs::remove_file(&problem_file).ok();
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
            "--graph",
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
fn test_create_x3c_alias() {
    let output_file = std::env::temp_dir().join("pred_test_create_x3c.json");
    let output = pred()
        .args([
            "-o",
            output_file.to_str().unwrap(),
            "create",
            "X3C",
            "--universe",
            "6",
            "--sets",
            "0,1,2;3,4,5",
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
    assert_eq!(json["type"], "ExactCoverBy3Sets");

    std::fs::remove_file(&output_file).ok();
}

#[test]
fn test_create_x3c_rejects_duplicate_subset_elements() {
    let output = pred()
        .args(["create", "X3C", "--universe", "6", "--sets", "0,0,1;3,4,5"])
        .output()
        .unwrap();
    assert!(
        !output.status.success(),
        "stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("contains duplicate elements"),
        "stderr: {stderr}"
    );
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
            "--graph",
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

// ---- Solve command tests ----

#[test]
fn test_solve_brute_force() {
    // Create a small MIS problem, then solve it
    let problem_file = std::env::temp_dir().join("pred_test_solve_bf.json");
    let create_out = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "MIS",
            "--graph",
            "0-1,1-2",
        ])
        .output()
        .unwrap();
    assert!(create_out.status.success());

    let output = pred()
        .args([
            "solve",
            problem_file.to_str().unwrap(),
            "--solver",
            "brute-force",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    // auto_json: data commands output JSON when stdout is not a TTY (as in tests)
    assert!(stdout.contains("\"solver\": \"brute-force\""));
    assert!(stdout.contains("\"solution\""));

    std::fs::remove_file(&problem_file).ok();
}

#[test]
fn test_solve_ilp() {
    let problem_file = std::env::temp_dir().join("pred_test_solve_ilp.json");
    let create_out = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "MIS",
            "--graph",
            "0-1,1-2",
        ])
        .output()
        .unwrap();
    assert!(create_out.status.success());

    let output = pred()
        .args(["solve", problem_file.to_str().unwrap(), "--solver", "ilp"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("\"solver\": \"ilp\""));
    assert!(stdout.contains("\"solution\""));
    assert!(
        stdout.contains("\"reduced_to\": \"ILP\""),
        "MIS solved with ILP should show auto-reduction: {stdout}"
    );

    std::fs::remove_file(&problem_file).ok();
}

#[test]
fn test_solve_ilp_default() {
    // Default solver is ilp
    let problem_file = std::env::temp_dir().join("pred_test_solve_default.json");
    let create_out = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "MIS",
            "--graph",
            "0-1,1-2",
        ])
        .output()
        .unwrap();
    assert!(create_out.status.success());

    let output = pred()
        .args(["solve", problem_file.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    // auto_json: data commands output JSON when stdout is not a TTY
    assert!(
        stdout.contains("\"solver\": \"ilp\"") && stdout.contains("\"reduced_to\": \"ILP\""),
        "MIS with default solver should show auto-reduction: {stdout}"
    );

    std::fs::remove_file(&problem_file).ok();
}

#[test]
fn test_solve_ilp_shows_via_ilp() {
    // When solving a non-ILP problem with ILP solver, output should show "via ILP"
    let problem_file = std::env::temp_dir().join("pred_test_solve_via_ilp.json");
    let create_out = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "MIS",
            "--graph",
            "0-1,1-2",
        ])
        .output()
        .unwrap();
    assert!(create_out.status.success());

    let output = pred()
        .args(["solve", problem_file.to_str().unwrap(), "--solver", "ilp"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    // auto_json: data commands output JSON when stdout is not a TTY
    assert!(
        stdout.contains("\"reduced_to\": \"ILP\""),
        "Non-ILP problem solved with ILP should show auto-reduction indicator, got: {stdout}"
    );
    assert!(stdout.contains("\"problem\": \"MaximumIndependentSet\""));

    std::fs::remove_file(&problem_file).ok();
}

#[test]
fn test_solve_json_output() {
    let problem_file = std::env::temp_dir().join("pred_test_solve_json_in.json");
    let result_file = std::env::temp_dir().join("pred_test_solve_json_out.json");
    let create_out = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "MIS",
            "--graph",
            "0-1,1-2",
        ])
        .output()
        .unwrap();
    assert!(create_out.status.success());

    let output = pred()
        .args([
            "-o",
            result_file.to_str().unwrap(),
            "solve",
            problem_file.to_str().unwrap(),
            "--solver",
            "brute-force",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(result_file.exists());

    let content = std::fs::read_to_string(&result_file).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert!(json["solution"].is_array());
    assert_eq!(json["solver"], "brute-force");

    std::fs::remove_file(&problem_file).ok();
    std::fs::remove_file(&result_file).ok();
}

#[test]
fn test_solve_bundle() {
    // Create → Reduce → Solve bundle
    let problem_file = std::env::temp_dir().join("pred_test_solve_bundle_in.json");
    let bundle_file = std::env::temp_dir().join("pred_test_solve_bundle.json");

    let create_out = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "MIS",
            "--graph",
            "0-1,1-2",
        ])
        .output()
        .unwrap();
    assert!(create_out.status.success());

    let reduce_out = pred()
        .args([
            "-o",
            bundle_file.to_str().unwrap(),
            "reduce",
            problem_file.to_str().unwrap(),
            "--to",
            "QUBO",
        ])
        .output()
        .unwrap();
    assert!(
        reduce_out.status.success(),
        "reduce stderr: {}",
        String::from_utf8_lossy(&reduce_out.stderr)
    );

    // Solve the bundle with brute-force
    let output = pred()
        .args([
            "solve",
            bundle_file.to_str().unwrap(),
            "--solver",
            "brute-force",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    // auto_json: data commands output JSON when stdout is not a TTY
    assert!(stdout.contains("\"problem\""));
    assert!(stdout.contains("\"solution\""));

    std::fs::remove_file(&problem_file).ok();
    std::fs::remove_file(&bundle_file).ok();
}

#[test]
fn test_solve_bundle_ilp() {
    // Create → Reduce → Solve bundle with ILP
    // Use MVC as target since it has an ILP reduction path (QUBO does not)
    let problem_file = std::env::temp_dir().join("pred_test_solve_bundle_ilp_in.json");
    let bundle_file = std::env::temp_dir().join("pred_test_solve_bundle_ilp.json");

    let create_out = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "MIS",
            "--graph",
            "0-1,1-2",
        ])
        .output()
        .unwrap();
    assert!(create_out.status.success());

    let reduce_out = pred()
        .args([
            "-o",
            bundle_file.to_str().unwrap(),
            "reduce",
            problem_file.to_str().unwrap(),
            "--to",
            "MVC",
        ])
        .output()
        .unwrap();
    assert!(
        reduce_out.status.success(),
        "reduce stderr: {}",
        String::from_utf8_lossy(&reduce_out.stderr)
    );

    let output = pred()
        .args(["solve", bundle_file.to_str().unwrap(), "--solver", "ilp"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    // auto_json: data commands output JSON when stdout is not a TTY
    assert!(stdout.contains("\"problem\""));
    assert!(stdout.contains("\"solution\""));

    std::fs::remove_file(&problem_file).ok();
    std::fs::remove_file(&bundle_file).ok();
}

#[test]
fn test_solve_unknown_solver() {
    let problem_file = std::env::temp_dir().join("pred_test_solve_unknown.json");
    let create_out = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "MIS",
            "--graph",
            "0-1,1-2",
        ])
        .output()
        .unwrap();
    assert!(create_out.status.success());

    let output = pred()
        .args([
            "solve",
            problem_file.to_str().unwrap(),
            "--solver",
            "unknown-solver",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Unknown solver"));

    std::fs::remove_file(&problem_file).ok();
}

// ---- Create command: more problem types ----

#[test]
fn test_create_maxcut() {
    let output_file = std::env::temp_dir().join("pred_test_create_maxcut.json");
    let output = pred()
        .args([
            "-o",
            output_file.to_str().unwrap(),
            "create",
            "MaxCut",
            "--graph",
            "0-1,1-2,2-0",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let content = std::fs::read_to_string(&output_file).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert_eq!(json["type"], "MaxCut");
    std::fs::remove_file(&output_file).ok();
}

#[test]
fn test_create_mvc() {
    let output_file = std::env::temp_dir().join("pred_test_create_mvc.json");
    let output = pred()
        .args([
            "-o",
            output_file.to_str().unwrap(),
            "create",
            "MVC",
            "--graph",
            "0-1,1-2",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let content = std::fs::read_to_string(&output_file).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert_eq!(json["type"], "MinimumVertexCover");
    std::fs::remove_file(&output_file).ok();
}

#[test]
fn test_create_kcoloring() {
    let output_file = std::env::temp_dir().join("pred_test_create_kcol.json");
    let output = pred()
        .args([
            "-o",
            output_file.to_str().unwrap(),
            "create",
            "KColoring",
            "--graph",
            "0-1,1-2,2-0",
            "--k",
            "3",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let content = std::fs::read_to_string(&output_file).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert_eq!(json["type"], "KColoring");
    std::fs::remove_file(&output_file).ok();
}

#[test]
fn test_create_spinglass() {
    let output_file = std::env::temp_dir().join("pred_test_create_sg.json");
    let output = pred()
        .args([
            "-o",
            output_file.to_str().unwrap(),
            "create",
            "SpinGlass",
            "--graph",
            "0-1,1-2",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let content = std::fs::read_to_string(&output_file).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert_eq!(json["type"], "SpinGlass");
    std::fs::remove_file(&output_file).ok();
}

#[test]
fn test_create_3sat() {
    let output_file = std::env::temp_dir().join("pred_test_create_3sat.json");
    let output = pred()
        .args([
            "-o",
            output_file.to_str().unwrap(),
            "create",
            "KSAT/K3",
            "--num-vars",
            "3",
            "--clauses",
            "1,2,3;-1,2,-3",
            "--k",
            "3",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let content = std::fs::read_to_string(&output_file).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert_eq!(json["type"], "KSatisfiability");
    std::fs::remove_file(&output_file).ok();
}

#[test]
fn test_create_maximum_matching() {
    let output_file = std::env::temp_dir().join("pred_test_create_mm.json");
    let output = pred()
        .args([
            "-o",
            output_file.to_str().unwrap(),
            "create",
            "MaximumMatching",
            "--graph",
            "0-1,1-2,2-3",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let content = std::fs::read_to_string(&output_file).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert_eq!(json["type"], "MaximumMatching");
    std::fs::remove_file(&output_file).ok();
}

#[test]
fn test_create_steiner_tree() {
    let output_file = std::env::temp_dir().join("pred_test_create_steiner_tree.json");
    let output = pred()
        .args([
            "-o",
            output_file.to_str().unwrap(),
            "create",
            "SteinerTree",
            "--graph",
            "0-1,0-3,1-2,1-3,2-3,2-4,3-4",
            "--edge-weights",
            "2,5,2,1,5,6,1",
            "--terminals",
            "0,2,4",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let content = std::fs::read_to_string(&output_file).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert_eq!(json["type"], "SteinerTree");
    assert_eq!(json["variant"]["graph"], "SimpleGraph");
    assert_eq!(json["variant"]["weight"], "i32");
    assert_eq!(json["data"]["terminals"], serde_json::json!([0, 2, 4]));
    std::fs::remove_file(&output_file).ok();
}

#[test]
fn test_create_steiner_tree_rejects_duplicate_terminals() {
    let output = pred()
        .args([
            "create",
            "SteinerTree",
            "--graph",
            "0-1,1-2",
            "--edge-weights",
            "1,1",
            "--terminals",
            "0,0",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("terminals must be distinct"), "{stderr}");
}

#[test]
fn test_create_with_edge_weights() {
    let output_file = std::env::temp_dir().join("pred_test_create_ew.json");
    let output = pred()
        .args([
            "-o",
            output_file.to_str().unwrap(),
            "create",
            "MaxCut",
            "--graph",
            "0-1,1-2,2-0",
            "--edge-weights",
            "2,3,1",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    std::fs::remove_file(&output_file).ok();
}

#[test]
fn test_create_without_output() {
    // Create without -o prints JSON to stdout (not just "Created ...")
    let output = pred()
        .args(["create", "MIS", "--graph", "0-1,1-2"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["type"], "MaximumIndependentSet");
    assert!(json["data"].is_object());
}

#[test]
fn test_create_from_example_source() {
    let output = pred()
        .args([
            "create",
            "--example",
            "MVC/SimpleGraph/i32",
            "--to",
            "MIS/SimpleGraph/i32",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["type"], "MinimumVertexCover");
    assert_eq!(json["variant"]["graph"], "SimpleGraph");
}

#[test]
fn test_create_from_example_target() {
    let output = pred()
        .args([
            "create",
            "--example",
            "MVC/SimpleGraph/i32",
            "--to",
            "MIS/SimpleGraph/i32",
            "--example-side",
            "target",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["type"], "MaximumIndependentSet");
    assert_eq!(json["variant"]["graph"], "SimpleGraph");
}

// ---- Error cases ----

#[test]
fn test_create_unknown_problem() {
    let output = pred()
        .args(["create", "NonExistent", "--graph", "0-1"])
        .output()
        .unwrap();
    assert!(!output.status.success());
}

#[test]
fn test_create_unknown_example_problem() {
    let output = pred()
        .args(["create", "--example", "not_a_real_example"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Unknown problem"));
}

#[test]
fn test_create_model_example_mis() {
    let output = pred()
        .args(["create", "--example", "MIS/SimpleGraph/i32"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["type"], "MaximumIndependentSet");
    assert_eq!(json["variant"]["graph"], "SimpleGraph");
    assert_eq!(json["variant"]["weight"], "i32");
}

#[test]
fn test_create_model_example_mis_shorthand() {
    let output = pred()
        .args(["create", "--example", "MIS"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["type"], "MaximumIndependentSet");
    assert_eq!(json["variant"]["graph"], "SimpleGraph");
    assert_eq!(json["variant"]["weight"], "One");
}

#[test]
fn test_create_model_example_mis_weight_only() {
    let output = pred()
        .args(["create", "--example", "MIS/i32"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["type"], "MaximumIndependentSet");
    assert_eq!(json["variant"]["graph"], "SimpleGraph");
    assert_eq!(json["variant"]["weight"], "i32");
}

#[test]
fn test_create_model_example_steiner_tree() {
    let output = pred()
        .args(["create", "--example", "SteinerTree"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["type"], "SteinerTree");
    assert_eq!(json["variant"]["graph"], "SimpleGraph");
    assert_eq!(json["variant"]["weight"], "i32");
}

#[test]
fn test_create_missing_model_example() {
    let output = pred()
        .args(["create", "--example", "GraphPartitioning/SimpleGraph"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("No canonical model example exists"));
}

#[test]
fn test_create_no_flags_shows_help() {
    // pred create MIS with no data flags shows schema-driven help and exits non-zero
    let output = pred().args(["create", "MIS"]).output().unwrap();
    assert!(
        !output.status.success(),
        "should exit non-zero when showing help without data flags"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("--graph"),
        "expected '--graph' in help output, got: {stderr}"
    );
    assert!(
        stderr.contains("--weights"),
        "expected '--weights' in help output, got: {stderr}"
    );
    assert!(
        stderr.contains("Example:"),
        "expected 'Example:' in help output, got: {stderr}"
    );
}

#[test]
fn test_create_kcoloring_missing_k() {
    let output = pred()
        .args(["create", "KColoring", "--graph", "0-1,1-2"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--k"));
}

#[test]
fn test_evaluate_wrong_config_length() {
    let problem_file = std::env::temp_dir().join("pred_test_eval_wrong_len.json");
    let create_out = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "MIS",
            "--graph",
            "0-1,1-2",
        ])
        .output()
        .unwrap();
    assert!(create_out.status.success());

    let output = pred()
        .args([
            "evaluate",
            problem_file.to_str().unwrap(),
            "--config",
            "1,0",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("variables"));

    std::fs::remove_file(&problem_file).ok();
}

#[test]
fn test_evaluate_json_output() {
    let problem_file = std::env::temp_dir().join("pred_test_eval_json_in.json");
    let result_file = std::env::temp_dir().join("pred_test_eval_json_out.json");
    let create_out = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "MIS",
            "--graph",
            "0-1,1-2",
        ])
        .output()
        .unwrap();
    assert!(create_out.status.success());

    let output = pred()
        .args([
            "-o",
            result_file.to_str().unwrap(),
            "evaluate",
            problem_file.to_str().unwrap(),
            "--config",
            "1,0,1",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(result_file.exists());
    let content = std::fs::read_to_string(&result_file).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert!(json["config"].is_array());

    std::fs::remove_file(&problem_file).ok();
    std::fs::remove_file(&result_file).ok();
}

#[test]
fn test_path_unknown_source() {
    let output = pred()
        .args(["path", "NonExistent", "QUBO"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Unknown problem"),
        "stderr should contain 'Unknown problem', got: {stderr}"
    );
    assert!(
        stderr.contains("pred list"),
        "stderr should suggest `pred list`, got: {stderr}"
    );
}

#[test]
fn test_path_unknown_target() {
    let output = pred()
        .args(["path", "MIS", "NonExistent"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Unknown problem"),
        "stderr should contain 'Unknown problem', got: {stderr}"
    );
    assert!(
        stderr.contains("pred list"),
        "stderr should suggest `pred list`, got: {stderr}"
    );
}

#[test]
fn test_path_with_cost_minimize_field() {
    let output = pred()
        .args(["path", "MIS", "QUBO", "--cost", "minimize:num_variables"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Path"));
}

#[test]
fn test_path_unknown_cost() {
    let output = pred()
        .args(["path", "MIS", "QUBO", "--cost", "bad-cost"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Unknown cost function"));
}

#[test]
fn test_path_overall_overhead_text() {
    // Use a multi-step path so the "Overall" section appears
    let output = pred().args(["path", "KSAT/K3", "MIS"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("Overall"),
        "multi-step path should show Overall overhead"
    );
}

#[test]
fn test_path_overall_overhead_json() {
    let tmp = std::env::temp_dir().join("pred_test_path_overall.json");
    let output = pred()
        .args(["path", "KSAT/K3", "MIS", "-o", tmp.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    let content = std::fs::read_to_string(&tmp).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert!(
        json["overall_overhead"].is_array(),
        "JSON should contain overall_overhead"
    );
    let items = json["overall_overhead"].as_array().unwrap();
    assert!(!items.is_empty(), "overall_overhead should have entries");
    assert!(items[0]["field"].is_string());
    assert!(items[0]["formula"].is_string());
    std::fs::remove_file(&tmp).ok();
}

#[test]
fn test_path_overall_overhead_composition() {
    // Verify that overall overhead is the symbolic composition of per-step overheads,
    // not just the last step's overhead. For a multi-step path A→B→C, the overall
    // should substitute B's output expressions into C's input expressions.
    let tmp = std::env::temp_dir().join("pred_test_path_composition.json");
    // 3SAT → SAT → MIS gives a 2-step path where:
    //   Step 1 (3SAT→SAT): num_literals = num_literals (identity)
    //   Step 2 (SAT→MIS): num_vertices = num_literals, num_edges = num_literals^2
    //   Overall: num_vertices = num_literals, num_edges = num_literals^2
    let output = pred()
        .args(["path", "KSAT/K3", "MIS", "-o", tmp.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    let content = std::fs::read_to_string(&tmp).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();

    // Must have at least 2 steps (K3→KN variant cast adds an extra step)
    assert!(json["steps"].as_u64().unwrap() >= 2);

    // Collect overall overhead into a map
    let overall: std::collections::HashMap<String, String> = json["overall_overhead"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| {
            (
                e["field"].as_str().unwrap().to_string(),
                e["formula"].as_str().unwrap().to_string(),
            )
        })
        .collect();

    // The composed overhead should reference source (3SAT) variables, not intermediate ones.
    // num_vertices and num_edges should both be expressed in terms of num_literals.
    assert!(
        overall.contains_key("num_vertices"),
        "overall should have num_vertices"
    );
    assert!(
        overall.contains_key("num_edges"),
        "overall should have num_edges"
    );
    assert!(
        overall["num_vertices"].contains("num_literals"),
        "num_vertices should be in terms of source vars, got: {}",
        overall["num_vertices"]
    );
    assert!(
        overall["num_edges"].contains("num_literals"),
        "num_edges should be in terms of source vars, got: {}",
        overall["num_edges"]
    );

    std::fs::remove_file(&tmp).ok();
}

#[test]
fn test_path_all_overall_overhead() {
    // Every path in --all --json output should have overall_overhead
    let output = pred()
        .args(["path", "KSAT/K3", "MIS", "--all", "--json"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let envelope: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let paths = envelope["paths"]
        .as_array()
        .expect("should have paths array");
    assert!(!paths.is_empty());
    for (i, p) in paths.iter().enumerate() {
        assert!(
            p["overall_overhead"].is_array(),
            "path {} missing overall_overhead",
            i + 1
        );
        let items = p["overall_overhead"].as_array().unwrap();
        assert!(
            !items.is_empty(),
            "path {} has empty overall_overhead",
            i + 1
        );
    }
    // Verify envelope metadata
    assert!(envelope["returned"].is_number());
    assert!(envelope["max_paths"].is_number());
    assert!(envelope["truncated"].is_boolean());
}

#[test]
fn test_path_single_step_no_overall_text() {
    // Single-step path should NOT show the Overall section
    // MaxCut -> SpinGlass is a genuine 1-step path with matching default variants
    let output = pred()
        .args(["path", "MaxCut", "SpinGlass"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        !stdout.contains("Overall"),
        "single-step path should not show Overall, got: {stdout}"
    );
}

#[test]
fn test_show_json_output() {
    let tmp = std::env::temp_dir().join("pred_test_show.json");
    let output = pred()
        .args(["-o", tmp.to_str().unwrap(), "show", "MIS"])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(tmp.exists());
    let content = std::fs::read_to_string(&tmp).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert_eq!(json["name"], "MaximumIndependentSet");
    assert!(json["variant"].is_object());
    assert!(json["reduces_to"].is_array());
    assert!(json["default"].is_boolean());
    std::fs::remove_file(&tmp).ok();
}

#[test]
fn test_show_size_fields() {
    let output = pred().args(["show", "MIS"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Size fields"));
}

#[test]
fn test_reduce_unknown_target() {
    let problem_file = std::env::temp_dir().join("pred_test_reduce_unknown.json");
    let create_out = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "MIS",
            "--graph",
            "0-1",
        ])
        .output()
        .unwrap();
    assert!(create_out.status.success());

    let output = pred()
        .args([
            "reduce",
            problem_file.to_str().unwrap(),
            "--to",
            "NonExistent",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());

    std::fs::remove_file(&problem_file).ok();
}

#[test]
fn test_reduce_stdout() {
    // Reduce without -o prints to stdout
    let problem_file = std::env::temp_dir().join("pred_test_reduce_stdout.json");
    let create_out = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "MIS",
            "--graph",
            "0-1,1-2",
        ])
        .output()
        .unwrap();
    assert!(create_out.status.success());

    let output = pred()
        .args([
            "reduce",
            problem_file.to_str().unwrap(),
            "--to",
            "QUBO",
            "--json",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert!(json["source"].is_object());
    assert!(json["target"].is_object());

    std::fs::remove_file(&problem_file).ok();
}

#[test]
fn test_reduce_auto_json_output() {
    // auto_json: reduce outputs JSON when stdout is not a TTY (as in tests)
    let problem_file = std::env::temp_dir().join("pred_test_reduce_human.json");
    let create_out = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "MIS",
            "--graph",
            "0-1,1-2",
        ])
        .output()
        .unwrap();
    assert!(create_out.status.success());

    let output = pred()
        .args(["reduce", problem_file.to_str().unwrap(), "--to", "QUBO"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("MaximumIndependentSet"),
        "expected 'MaximumIndependentSet' in stdout, got: {stdout}"
    );
    assert!(
        stdout.contains("QUBO"),
        "expected 'QUBO' in stdout, got: {stdout}"
    );
    // auto_json: should be valid JSON when stdout is not a TTY
    assert!(
        serde_json::from_str::<serde_json::Value>(&stdout).is_ok(),
        "stdout should be valid JSON with auto_json, got: {stdout}"
    );

    std::fs::remove_file(&problem_file).ok();
}

// ---- Hint suppression tests ----

#[test]
fn test_solve_no_hint_when_piped() {
    // When stderr is a pipe (not a TTY), the solve hint should be suppressed.
    // In tests, subprocess stderr is captured via pipe, so it's not a TTY.
    let problem_file = std::env::temp_dir().join("pred_test_solve_no_hint.json");
    let create_out = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "MIS",
            "--graph",
            "0-1,1-2",
        ])
        .output()
        .unwrap();
    assert!(create_out.status.success());

    // Solve without -o (brute-force)
    let output = pred()
        .args([
            "solve",
            problem_file.to_str().unwrap(),
            "--solver",
            "brute-force",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains("Hint:"),
        "Hint should not appear when stderr is piped, got: {stderr}"
    );

    // Solve without -o (ilp)
    let output = pred()
        .args(["solve", problem_file.to_str().unwrap(), "--solver", "ilp"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains("Hint:"),
        "Hint should not appear when stderr is piped, got: {stderr}"
    );

    std::fs::remove_file(&problem_file).ok();
}

#[test]
fn test_solve_bundle_no_hint_when_piped() {
    // Bundle solve path: hint should also be suppressed when piped.
    let problem_file = std::env::temp_dir().join("pred_test_solve_bundle_no_hint.json");
    let bundle_file = std::env::temp_dir().join("pred_test_solve_bundle_no_hint_bundle.json");

    let create_out = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "MIS",
            "--graph",
            "0-1,1-2",
        ])
        .output()
        .unwrap();
    assert!(create_out.status.success());

    let reduce_out = pred()
        .args([
            "-o",
            bundle_file.to_str().unwrap(),
            "reduce",
            problem_file.to_str().unwrap(),
            "--to",
            "QUBO",
        ])
        .output()
        .unwrap();
    assert!(reduce_out.status.success());

    let output = pred()
        .args([
            "solve",
            bundle_file.to_str().unwrap(),
            "--solver",
            "brute-force",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains("Hint:"),
        "Hint should not appear when stderr is piped, got: {stderr}"
    );

    std::fs::remove_file(&problem_file).ok();
    std::fs::remove_file(&bundle_file).ok();
}

// ---- Help message tests ----

#[test]
fn test_incorrect_command_shows_help() {
    // Missing required arguments should show after_help
    let output = pred().args(["solve"]).output().unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    // The subcommand help hint should be shown
    assert!(
        stderr.contains("pred create") || stderr.contains("pred solve") || stderr.contains("Usage"),
        "stderr should contain help: {stderr}"
    );
}

#[test]
fn test_subcommand_help() {
    let output = pred().args(["solve", "--help"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("brute-force"));
    assert!(stdout.contains("pred create"));
}

// ---- Shell completions tests ----

#[test]
fn test_completions_bash() {
    let output = pred().args(["completions", "bash"]).output().unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("pred"),
        "completions should reference 'pred'"
    );
}

#[test]
fn test_completions_auto_detect() {
    // Without explicit shell arg, should still succeed (falls back to bash)
    let output = pred().args(["completions"]).output().unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("pred"));
}

// ---- k-neighbor exploration tests (pred to / pred from) ----

#[test]
fn test_to_incoming() {
    // `pred to MIS` shows what reduces TO MIS (incoming neighbors)
    let output = pred().args(["to", "MIS", "--hops", "2"]).output().unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("MaximumIndependentSet"));
    assert!(stdout.contains("incoming"));
    assert!(stdout.contains("reachable nodes"));
    // Should contain tree characters
    assert!(stdout.contains("├── ") || stdout.contains("└── "));
}

#[test]
fn test_from_outgoing() {
    // `pred from MIS` shows what MIS reduces to (outgoing neighbors)
    let output = pred()
        .args(["from", "MIS", "--hops", "1"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("MaximumIndependentSet"));
    assert!(stdout.contains("outgoing"));
}

#[test]
fn test_to_json() {
    let tmp = std::env::temp_dir().join("pred_test_to_hops.json");
    let output = pred()
        .args(["-o", tmp.to_str().unwrap(), "to", "MIS", "--hops", "2"])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(tmp.exists());
    let content = std::fs::read_to_string(&tmp).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert_eq!(json["source"], "MaximumIndependentSet");
    assert_eq!(json["hops"], 2);
    assert!(json["neighbors"].is_array());
    std::fs::remove_file(&tmp).ok();
}

#[test]
fn test_to_shows_variant_info() {
    let output = pred().args(["to", "MIS", "--hops", "1"]).output().unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    // Slash notation: either base name or Name/Variant
    assert!(
        stdout.contains("MaximumIndependentSet"),
        "expected problem name in tree output, got: {stdout}"
    );
}

#[test]
fn test_from_shows_variant_info() {
    let output = pred()
        .args(["from", "MIS", "--hops", "1"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    // Slash notation: either base name or Name/Variant
    assert!(
        stdout.contains("MaximumIndependentSet"),
        "expected problem name in tree output, got: {stdout}"
    );
}

#[test]
fn test_to_default_hops() {
    // Default --hops is 1
    let output = pred().args(["to", "MIS"]).output().unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("1-hop"));
    assert!(stdout.contains("reachable nodes"));
}

// ---- Quiet mode tests ----

#[test]
fn test_quiet_suppresses_hints() {
    // Solve with -q: even if stderr were a TTY, quiet suppresses hints.
    // In tests stderr is a pipe so hints are already suppressed by TTY check,
    // but we verify -q is accepted and doesn't break anything.
    let problem_file = std::env::temp_dir().join("pred_test_quiet_hint.json");
    let create_out = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "MIS",
            "--graph",
            "0-1,1-2",
        ])
        .output()
        .unwrap();
    assert!(create_out.status.success());

    let output = pred()
        .args([
            "-q",
            "solve",
            problem_file.to_str().unwrap(),
            "--solver",
            "brute-force",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains("Hint:"),
        "Hint should be suppressed with -q, got: {stderr}"
    );

    std::fs::remove_file(&problem_file).ok();
}

#[test]
fn test_quiet_suppresses_wrote() {
    // Create with -q -o: the "Wrote ..." message should be suppressed.
    let output_file = std::env::temp_dir().join("pred_test_quiet_wrote.json");
    let output = pred()
        .args([
            "-q",
            "-o",
            output_file.to_str().unwrap(),
            "create",
            "MIS",
            "--graph",
            "0-1,1-2",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains("Wrote"),
        "\"Wrote\" message should be suppressed with -q, got: {stderr}"
    );
    assert!(output_file.exists());

    std::fs::remove_file(&output_file).ok();
}

#[test]
fn test_quiet_still_shows_stdout() {
    // Solve with -q: stdout should still contain the solution output.
    let problem_file = std::env::temp_dir().join("pred_test_quiet_stdout.json");
    let create_out = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "MIS",
            "--graph",
            "0-1,1-2",
        ])
        .output()
        .unwrap();
    assert!(create_out.status.success());

    let output = pred()
        .args([
            "-q",
            "solve",
            problem_file.to_str().unwrap(),
            "--solver",
            "brute-force",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    // auto_json: data commands output JSON when stdout is not a TTY
    assert!(
        stdout.contains("\"solution\""),
        "stdout should still contain solution with -q, got: {stdout}"
    );

    std::fs::remove_file(&problem_file).ok();
}

// ---- Stdin/pipe support tests ----

#[test]
fn test_create_pipe_to_solve() {
    // pred create MIS --graph 0-1,1-2 | pred solve - --solver brute-force
    let create_out = pred()
        .args(["create", "MIS", "--graph", "0-1,1-2"])
        .output()
        .unwrap();
    assert!(
        create_out.status.success(),
        "create stderr: {}",
        String::from_utf8_lossy(&create_out.stderr)
    );

    use std::io::Write;
    let mut child = pred()
        .args(["solve", "-", "--solver", "brute-force"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(&create_out.stdout)
        .unwrap();
    let solve_result = child.wait_with_output().unwrap();
    assert!(
        solve_result.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&solve_result.stderr)
    );
    let stdout = String::from_utf8(solve_result.stdout).unwrap();
    // auto_json: data commands output JSON when stdout is not a TTY
    assert!(
        stdout.contains("\"solution\""),
        "stdout should contain solution, got: {stdout}"
    );
}

#[test]
fn test_create_pipe_to_evaluate() {
    // pred create MIS --graph 0-1,1-2 | pred evaluate - --config 1,0,1
    let create_out = pred()
        .args(["create", "MIS", "--graph", "0-1,1-2"])
        .output()
        .unwrap();
    assert!(
        create_out.status.success(),
        "create stderr: {}",
        String::from_utf8_lossy(&create_out.stderr)
    );

    use std::io::Write;
    let mut child = pred()
        .args(["evaluate", "-", "--config", "1,0,1"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(&create_out.stdout)
        .unwrap();
    let eval_result = child.wait_with_output().unwrap();
    assert!(
        eval_result.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&eval_result.stderr)
    );
    let stdout = String::from_utf8(eval_result.stdout).unwrap();
    assert!(
        stdout.contains("Valid"),
        "stdout should contain Valid, got: {stdout}"
    );
}

#[test]
fn test_create_pipe_to_reduce() {
    // pred create MIS --graph 0-1,1-2 | pred reduce - --to QUBO
    let create_out = pred()
        .args(["create", "MIS", "--graph", "0-1,1-2"])
        .output()
        .unwrap();
    assert!(
        create_out.status.success(),
        "create stderr: {}",
        String::from_utf8_lossy(&create_out.stderr)
    );

    use std::io::Write;
    let mut child = pred()
        .args(["reduce", "-", "--to", "QUBO", "--json"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(&create_out.stdout)
        .unwrap();
    let reduce_result = child.wait_with_output().unwrap();
    assert!(
        reduce_result.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&reduce_result.stderr)
    );
    let stdout = String::from_utf8(reduce_result.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert!(
        json["source"].is_object(),
        "expected source object in reduction bundle, got: {stdout}"
    );
}

// ---- Inspect command tests ----

#[test]
fn test_inspect_problem() {
    let problem_file = std::env::temp_dir().join("pred_test_inspect.json");
    let create_out = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "MIS",
            "--graph",
            "0-1,1-2,2-3",
        ])
        .output()
        .unwrap();
    assert!(create_out.status.success());

    let output = pred()
        .args(["inspect", problem_file.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    // auto_json: data commands output JSON when stdout is not a TTY
    assert!(
        stdout.contains("MaximumIndependentSet"),
        "expected 'MaximumIndependentSet', got: {stdout}"
    );
    assert!(
        stdout.contains("\"kind\""),
        "expected '\"kind\"', got: {stdout}"
    );
    assert!(
        serde_json::from_str::<serde_json::Value>(&stdout).is_ok(),
        "expected valid JSON, got: {stdout}"
    );

    std::fs::remove_file(&problem_file).ok();
}

#[test]
fn test_inspect_bundle() {
    let problem_file = std::env::temp_dir().join("pred_test_inspect_bundle_p.json");
    let bundle_file = std::env::temp_dir().join("pred_test_inspect_bundle.json");

    let create_out = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "MIS",
            "--graph",
            "0-1,1-2",
        ])
        .output()
        .unwrap();
    assert!(create_out.status.success());

    let reduce_out = pred()
        .args([
            "-o",
            bundle_file.to_str().unwrap(),
            "reduce",
            problem_file.to_str().unwrap(),
            "--to",
            "QUBO",
        ])
        .output()
        .unwrap();
    assert!(
        reduce_out.status.success(),
        "reduce stderr: {}",
        String::from_utf8_lossy(&reduce_out.stderr)
    );

    let output = pred()
        .args(["inspect", bundle_file.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    // auto_json: data commands output JSON when stdout is not a TTY
    assert!(
        stdout.contains("\"kind\": \"bundle\""),
        "expected '\"kind\": \"bundle\"' in output, got: {stdout}"
    );
    assert!(
        stdout.contains("\"source\""),
        "expected '\"source\"' in output, got: {stdout}"
    );
    assert!(
        stdout.contains("\"target\""),
        "expected '\"target\"' in output, got: {stdout}"
    );

    std::fs::remove_file(&problem_file).ok();
    std::fs::remove_file(&bundle_file).ok();
}

#[test]
fn test_inspect_stdin() {
    // Test pipe: create | inspect -
    let create_out = pred()
        .args(["create", "MIS", "--graph", "0-1,1-2"])
        .output()
        .unwrap();
    assert!(create_out.status.success());

    use std::io::Write;
    let mut child = pred()
        .args(["inspect", "-"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(&create_out.stdout)
        .unwrap();
    let result = child.wait_with_output().unwrap();
    assert!(
        result.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    let stdout = String::from_utf8(result.stdout).unwrap();
    assert!(
        stdout.contains("MaximumIndependentSet"),
        "expected 'MaximumIndependentSet', got: {stdout}"
    );
}

#[test]
fn test_inspect_json_output() {
    let problem_file = std::env::temp_dir().join("pred_test_inspect_json_in.json");
    let result_file = std::env::temp_dir().join("pred_test_inspect_json_out.json");
    let create_out = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "MIS",
            "--graph",
            "0-1,1-2,2-3",
        ])
        .output()
        .unwrap();
    assert!(create_out.status.success());

    let output = pred()
        .args([
            "-o",
            result_file.to_str().unwrap(),
            "inspect",
            problem_file.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(result_file.exists());

    let content = std::fs::read_to_string(&result_file).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert_eq!(json["kind"], "problem");
    assert_eq!(json["type"], "MaximumIndependentSet");
    let size_fields: Vec<&str> = json["size_fields"]
        .as_array()
        .expect("size_fields should be an array")
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();
    assert!(
        size_fields.contains(&"num_vertices"),
        "MIS size_fields should contain num_vertices, got: {:?}",
        size_fields
    );
    assert!(
        size_fields.contains(&"num_edges"),
        "MIS size_fields should contain num_edges, got: {:?}",
        size_fields
    );
    assert!(json["solvers"].is_array());
    assert!(json["reduces_to"].is_array());

    std::fs::remove_file(&problem_file).ok();
    std::fs::remove_file(&result_file).ok();
}

#[test]
fn test_inspect_undirected_two_commodity_integral_flow_reports_size_fields() {
    let problem_file = std::env::temp_dir().join("pred_test_utcif_inspect_in.json");
    let result_file = std::env::temp_dir().join("pred_test_utcif_inspect_out.json");
    let create_out = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "--example",
            "UndirectedTwoCommodityIntegralFlow",
        ])
        .output()
        .unwrap();
    assert!(
        create_out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&create_out.stderr)
    );

    let output = pred()
        .args([
            "-o",
            result_file.to_str().unwrap(),
            "inspect",
            problem_file.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(result_file.exists());

    let content = std::fs::read_to_string(&result_file).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    let size_fields: Vec<&str> = json["size_fields"]
        .as_array()
        .expect("size_fields should be an array")
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();
    assert!(
        size_fields.contains(&"num_vertices"),
        "UndirectedTwoCommodityIntegralFlow size_fields should contain num_vertices, got: {:?}",
        size_fields
    );
    assert!(
        size_fields.contains(&"num_edges"),
        "UndirectedTwoCommodityIntegralFlow size_fields should contain num_edges, got: {:?}",
        size_fields
    );

    std::fs::remove_file(&problem_file).ok();
    std::fs::remove_file(&result_file).ok();
}

// ---- Random generation tests ----

#[test]
fn test_create_random_mis() {
    let output = pred()
        .args([
            "create",
            "MIS",
            "--random",
            "--num-vertices",
            "10",
            "--edge-prob",
            "0.3",
            "--seed",
            "42",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["type"], "MaximumIndependentSet");
    assert!(json["data"].is_object());
}

#[test]
fn test_create_random_deterministic() {
    // Same seed should produce identical output
    let out1 = pred()
        .args([
            "create",
            "MIS",
            "--random",
            "--num-vertices",
            "5",
            "--seed",
            "123",
        ])
        .output()
        .unwrap();
    let out2 = pred()
        .args([
            "create",
            "MIS",
            "--random",
            "--num-vertices",
            "5",
            "--seed",
            "123",
        ])
        .output()
        .unwrap();
    assert!(out1.status.success());
    assert!(out2.status.success());
    assert_eq!(out1.stdout, out2.stdout);
}

#[test]
fn test_create_random_missing_num_vertices() {
    let output = pred().args(["create", "MIS", "--random"]).output().unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("--num-vertices"),
        "expected '--num-vertices' in error, got: {stderr}"
    );
}

#[test]
fn test_create_random_maxcut() {
    let output = pred()
        .args([
            "create",
            "MaxCut",
            "--random",
            "--num-vertices",
            "5",
            "--seed",
            "42",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["type"], "MaxCut");
}

#[test]
fn test_create_random_unsupported() {
    let output = pred()
        .args(["create", "SAT", "--random", "--num-vertices", "5"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("not supported"),
        "expected 'not supported' in error, got: {stderr}"
    );
}

#[test]
fn test_create_random_steiner_tree_requires_two_vertices() {
    let output = pred()
        .args(["create", "SteinerTree", "--random", "--num-vertices", "1"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("SteinerTree random generation requires --num-vertices >= 2"),
        "{stderr}"
    );
}

#[test]
fn test_create_random_invalid_edge_prob() {
    let output = pred()
        .args([
            "create",
            "MIS",
            "--random",
            "--num-vertices",
            "5",
            "--edge-prob",
            "1.5",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("--edge-prob must be between"),
        "expected edge-prob validation error, got: {stderr}"
    );
}

#[test]
fn test_create_random_spinglass() {
    let output = pred()
        .args([
            "create",
            "SpinGlass",
            "--random",
            "--num-vertices",
            "5",
            "--seed",
            "42",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["type"], "SpinGlass");
}

#[test]
fn test_create_random_kcoloring() {
    let output = pred()
        .args([
            "create",
            "KColoring",
            "--random",
            "--num-vertices",
            "5",
            "--seed",
            "42",
            "--k",
            "3",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["type"], "KColoring");
}

#[test]
fn test_create_random_to_file() {
    let output_file = std::env::temp_dir().join("pred_test_create_random.json");
    let output = pred()
        .args([
            "-o",
            output_file.to_str().unwrap(),
            "create",
            "MIS",
            "--random",
            "--num-vertices",
            "8",
            "--edge-prob",
            "0.4",
            "--seed",
            "99",
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

    std::fs::remove_file(&output_file).ok();
}

#[test]
fn test_create_random_default_edge_prob() {
    // Without --edge-prob, defaults to 0.5
    let output = pred()
        .args([
            "create",
            "MIS",
            "--random",
            "--num-vertices",
            "5",
            "--seed",
            "42",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["type"], "MaximumIndependentSet");
}

// ---- Factoring create tests (P8) ----

#[test]
fn test_create_factoring() {
    let output = pred()
        .args([
            "create",
            "Factoring",
            "--target",
            "15",
            "--m",
            "4",
            "--n",
            "4",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["type"], "Factoring");
    assert!(json["data"].is_object());
}

#[test]
fn test_create_factoring_with_bits() {
    let output_file = std::env::temp_dir().join("pred_test_create_factoring.json");
    let output = pred()
        .args([
            "-o",
            output_file.to_str().unwrap(),
            "create",
            "Factoring",
            "--target",
            "15",
            "--m",
            "4",
            "--n",
            "4",
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
    assert_eq!(json["type"], "Factoring");
    std::fs::remove_file(&output_file).ok();
}

#[test]
fn test_create_factoring_no_flags_shows_help() {
    // pred create Factoring with no data flags shows schema-driven help and exits non-zero
    let output = pred().args(["create", "Factoring"]).output().unwrap();
    assert!(
        !output.status.success(),
        "should exit non-zero when showing help without data flags"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("--target"),
        "expected '--target' in help output, got: {stderr}"
    );
    assert!(
        stderr.contains("--m"),
        "expected '--m' in help output, got: {stderr}"
    );
}

#[test]
fn test_create_factoring_missing_bits() {
    let output = pred()
        .args(["create", "Factoring", "--target", "15"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("--m"),
        "expected '--m' in error, got: {stderr}"
    );
}

// ---- Timeout tests (H3) ----

#[test]
fn test_solve_timeout_succeeds() {
    // Small problem with generous timeout should succeed
    let problem_file = std::env::temp_dir().join("pred_test_solve_timeout.json");
    let create_out = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "MIS",
            "--graph",
            "0-1,1-2",
        ])
        .output()
        .unwrap();
    assert!(create_out.status.success());

    let output = pred()
        .args([
            "solve",
            problem_file.to_str().unwrap(),
            "--solver",
            "brute-force",
            "--timeout",
            "30",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    // auto_json: data commands output JSON when stdout is not a TTY
    assert!(
        stdout.contains("\"solution\""),
        "expected solution in stdout, got: {stdout}"
    );

    std::fs::remove_file(&problem_file).ok();
}

#[test]
fn test_solve_timeout_zero_means_no_limit() {
    // --timeout 0 is the default (no limit), should work normally
    let problem_file = std::env::temp_dir().join("pred_test_solve_timeout0.json");
    let create_out = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "MIS",
            "--graph",
            "0-1,1-2",
        ])
        .output()
        .unwrap();
    assert!(create_out.status.success());

    let output = pred()
        .args([
            "solve",
            problem_file.to_str().unwrap(),
            "--solver",
            "brute-force",
            "--timeout",
            "0",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    // auto_json: data commands output JSON when stdout is not a TTY
    assert!(stdout.contains("\"solution\""));

    std::fs::remove_file(&problem_file).ok();
}

// ---------------------------------------------------------------------------
// Geometry-based graph tests
// ---------------------------------------------------------------------------

#[test]
fn test_create_mis_kings_subgraph() {
    let output = pred()
        .args([
            "create",
            "MIS/KingsSubgraph",
            "--positions",
            "0,0;1,0;1,1;0,1",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["type"], "MaximumIndependentSet");
    assert_eq!(json["variant"]["graph"], "KingsSubgraph");
    assert!(json["data"].is_object());
}

#[test]
fn test_create_mis_triangular_subgraph() {
    let output = pred()
        .args([
            "create",
            "MIS/TriangularSubgraph/i32",
            "--positions",
            "0,0;0,1;1,0;1,1",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["type"], "MaximumIndependentSet");
    assert_eq!(json["variant"]["graph"], "TriangularSubgraph");
}

#[test]
fn test_create_mis_unit_disk_graph() {
    let output = pred()
        .args([
            "create",
            "MIS/UnitDiskGraph",
            "--positions",
            "0,0;1,0;0.5,0.8",
            "--radius",
            "1.5",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["type"], "MaximumIndependentSet");
    assert_eq!(json["variant"]["graph"], "UnitDiskGraph");
}

#[test]
fn test_create_mvc_kings_subgraph_unsupported_variant() {
    // MVC doesn't have a KingsSubgraph variant registered
    let output = pred()
        .args(["create", "MVC/KingsSubgraph", "--positions", "0,0;1,0;1,1"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Unknown variant token \"KingsSubgraph\""),
        "should mention unknown variant token: {stderr}"
    );
}

#[test]
fn test_create_mis_unit_disk_graph_default_radius() {
    let output = pred()
        .args([
            "create",
            "MIS/UnitDiskGraph",
            "--positions",
            "0,0;0.5,0;1,0",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["type"], "MaximumIndependentSet");
    assert_eq!(json["variant"]["graph"], "UnitDiskGraph");
}

#[test]
fn test_create_mis_kings_subgraph_with_weights() {
    let output = pred()
        .args([
            "create",
            "MIS/KingsSubgraph/i32",
            "--positions",
            "0,0;1,0;1,1",
            "--weights",
            "2,3,1",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["type"], "MaximumIndependentSet");
    assert_eq!(json["variant"]["graph"], "KingsSubgraph");
    assert_eq!(json["variant"]["weight"], "i32");
}

#[test]
fn test_create_random_kings_subgraph() {
    let output = pred()
        .args([
            "create",
            "MIS/KingsSubgraph",
            "--random",
            "--num-vertices",
            "10",
            "--seed",
            "42",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["type"], "MaximumIndependentSet");
    assert_eq!(json["variant"]["graph"], "KingsSubgraph");
}

#[test]
fn test_create_random_triangular_subgraph() {
    let output = pred()
        .args([
            "create",
            "MIS/TriangularSubgraph/i32",
            "--random",
            "--num-vertices",
            "8",
            "--seed",
            "42",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["type"], "MaximumIndependentSet");
    assert_eq!(json["variant"]["graph"], "TriangularSubgraph");
}

#[test]
fn test_create_random_unit_disk_graph() {
    let output = pred()
        .args([
            "create",
            "MIS/UnitDiskGraph",
            "--random",
            "--num-vertices",
            "10",
            "--radius",
            "1.5",
            "--seed",
            "42",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["type"], "MaximumIndependentSet");
    assert_eq!(json["variant"]["graph"], "UnitDiskGraph");
}

#[test]
fn test_create_kings_subgraph_help() {
    let output = pred()
        .args(["create", "MIS/KingsSubgraph"])
        .output()
        .unwrap();
    assert!(
        !output.status.success(),
        "should exit non-zero when showing help"
    );
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("positions") || stderr.contains("MaximumIndependentSet"),
        "stderr should show help: {stderr}"
    );
}

#[test]
fn test_create_geometry_graph_missing_positions() {
    let output = pred()
        .args(["create", "MIS/KingsSubgraph", "--weights", "1,2,3"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("--positions"),
        "should mention --positions: {stderr}"
    );
}

// ---- Round-trip: canonical examples through solve ----

#[test]
fn test_create_model_example_mis_round_trips_into_solve() {
    let path = std::env::temp_dir().join(format!(
        "pred_test_model_example_mis_{}.json",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let create = pred()
        .args([
            "create",
            "--example",
            "MIS/SimpleGraph/i32",
            "-o",
            path.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(
        create.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&create.stderr)
    );

    let solve = pred()
        .args(["solve", path.to_str().unwrap(), "--solver", "brute-force"])
        .output()
        .unwrap();
    assert!(
        solve.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&solve.stderr)
    );

    std::fs::remove_file(&path).ok();
}

#[test]
fn test_create_rule_example_mvc_to_mis_round_trips_into_solve() {
    let path = std::env::temp_dir().join(format!(
        "pred_test_rule_example_mvc_to_mis_{}.json",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let create = pred()
        .args([
            "create",
            "--example",
            "MVC/SimpleGraph/i32",
            "--to",
            "MIS/SimpleGraph/i32",
            "-o",
            path.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(
        create.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&create.stderr)
    );

    let solve = pred()
        .args(["solve", path.to_str().unwrap(), "--solver", "brute-force"])
        .output()
        .unwrap();
    assert!(
        solve.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&solve.stderr)
    );

    std::fs::remove_file(&path).ok();
}

#[test]
fn test_create_rule_example_mvc_to_mis_weight_only() {
    let output = pred()
        .args(["create", "--example", "MVC/i32", "--to", "MIS/i32"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["type"], "MinimumVertexCover");
    assert_eq!(json["variant"]["graph"], "SimpleGraph");
    assert_eq!(json["variant"]["weight"], "i32");
}

#[test]
fn test_create_rule_example_mvc_to_mis_target_weight_only() {
    let output = pred()
        .args([
            "create",
            "--example",
            "MVC/i32",
            "--to",
            "MIS/i32",
            "--example-side",
            "target",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["type"], "MaximumIndependentSet");
    assert_eq!(json["variant"]["graph"], "SimpleGraph");
    assert_eq!(json["variant"]["weight"], "i32");
}

// ---- Variant-level show semantics ----

#[test]
fn test_show_with_slash_spec() {
    // `pred show MIS/UnitDiskGraph` should show that specific variant
    let output = pred().args(["show", "MIS/UnitDiskGraph"]).output().unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("UnitDiskGraph"),
        "should show UnitDiskGraph variant: {stdout}"
    );
}

#[test]
fn test_show_bare_name_uses_default() {
    // `pred show MIS` resolves to default variant and marks it
    let output = pred().args(["show", "MIS"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("SimpleGraph"),
        "bare MIS should resolve to SimpleGraph default: {stdout}"
    );
}

#[test]
fn test_show_ksat_works() {
    // `pred show KSAT` should succeed (alias resolves to KSatisfiability default variant)
    let output = pred().args(["show", "KSAT"]).output().unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("KSatisfiability"),
        "should show KSatisfiability: {stdout}"
    );
}

// ---- Capped multi-path ----

#[test]
fn test_path_all_max_paths_truncates() {
    // With --max-paths 3, should limit to 3 paths and indicate truncation
    let output = pred()
        .args(["path", "MIS", "QUBO", "--all", "--max-paths", "3", "--json"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let envelope: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let paths = envelope["paths"]
        .as_array()
        .expect("should have paths array");
    assert!(
        paths.len() <= 3,
        "should return at most 3 paths, got {}",
        paths.len()
    );
    assert_eq!(envelope["max_paths"], 3);
    // MIS -> QUBO has many paths, so truncation is expected
    assert_eq!(
        envelope["truncated"], true,
        "should be truncated since MIS->QUBO has many paths"
    );
}

#[test]
fn test_path_all_max_paths_text_truncation_note() {
    let output = pred()
        .args(["path", "MIS", "QUBO", "--all", "--max-paths", "2"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("--max-paths"),
        "truncation note should mention --max-paths: {stdout}"
    );
}

// ---- Default variant resolution for create ----

#[test]
fn test_create_bare_mis_default_variant() {
    // `pred create MIS --graph 0-1,1-2,2-3` should work with default variant
    let output = pred()
        .args(["create", "MIS", "--graph", "0-1,1-2,2-3"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["type"], "MaximumIndependentSet");
}

// ---- Show JSON includes default annotation ----

#[test]
fn test_show_json_has_default_field() {
    let output = pred().args(["show", "MIS", "--json"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    // Bare MIS resolves to default variant
    assert_eq!(
        json["default"], true,
        "bare MIS should be the default variant"
    );
    assert!(json["variant"].is_object(), "should have variant object");
}

// ---- path --all directory output includes manifest ----

#[test]
fn test_path_all_save_manifest() {
    let dir = std::env::temp_dir().join("pred_test_all_paths_manifest");
    let _ = std::fs::remove_dir_all(&dir);
    let output = pred()
        .args([
            "path",
            "MaxCut",
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

    let manifest_file = dir.join("manifest.json");
    assert!(manifest_file.exists(), "manifest.json should be created");
    let manifest_content = std::fs::read_to_string(&manifest_file).unwrap();
    let manifest: serde_json::Value = serde_json::from_str(&manifest_content).unwrap();
    assert!(manifest["paths"].is_number());
    assert!(manifest["max_paths"].is_number());
    assert!(manifest["truncated"].is_boolean());

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn test_create_nonunit_weights_require_weighted_variant() {
    let output = pred()
        .args([
            "create",
            "MIS",
            "--graph",
            "0-1,1-2,2-3",
            "--weights",
            "3,1,2,1",
        ])
        .output()
        .unwrap();
    assert!(
        !output.status.success(),
        "non-unit weights should require /i32"
    );
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Use the weighted variant instead"),
        "stderr should point to the explicit weighted variant: {stderr}"
    );
    assert!(
        stderr.contains("MaximumIndependentSet/SimpleGraph/i32"),
        "stderr should include the exact weighted variant: {stderr}"
    );
}

#[test]
fn test_create_unit_weights_stays_one() {
    // When all weights are 1, the variant should remain One.
    let output = pred()
        .args([
            "create",
            "MIS",
            "--graph",
            "0-1,1-2,2-3",
            "--weights",
            "1,1,1,1",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["variant"]["weight"], "One");
}

#[test]
fn test_create_weighted_mis_round_trips_into_solve() {
    // The explicit weighted MIS variant should be solvable end-to-end.
    let create_output = pred()
        .args([
            "create",
            "MIS/i32",
            "--graph",
            "0-1,1-2,2-3",
            "--weights",
            "3,1,2,1",
        ])
        .output()
        .unwrap();
    assert!(create_output.status.success());

    let solve_output = pred()
        .args(["solve", "-", "--solver", "brute-force"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            use std::io::Write;
            child
                .stdin
                .take()
                .unwrap()
                .write_all(&create_output.stdout)
                .unwrap();
            child.wait_with_output()
        })
        .unwrap();
    assert!(
        solve_output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&solve_output.stderr)
    );
    let stdout = String::from_utf8(solve_output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["evaluation"], "Valid(5)");
}
