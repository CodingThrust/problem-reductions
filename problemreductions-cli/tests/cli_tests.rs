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
    assert!(json["problems"].is_array());
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
        .args(["path", "MIS", "QUBO", "-o", path_file.to_str().unwrap()])
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
            "MIS",
            "--edges",
            "0-1,1-2,2-3",
        ])
        .output()
        .unwrap();
    assert!(create_out.status.success());

    let path_file = std::env::temp_dir().join("pred_test_reduce_via_infer_path.json");
    let path_out = pred()
        .args(["path", "MIS", "QUBO", "-o", path_file.to_str().unwrap()])
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
fn test_reduce_missing_to_and_via() {
    let problem_file = std::env::temp_dir().join("pred_test_reduce_missing.json");
    let create_out = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "MIS",
            "--edges",
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
            "--edges",
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
    assert!(stdout.contains("brute-force"));
    assert!(stdout.contains("Solution"));

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
            "--edges",
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
    assert!(stdout.contains("ilp"));
    assert!(stdout.contains("Solution"));
    assert!(
        stdout.contains("via ILP"),
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
            "--edges",
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
    assert!(
        stdout.contains("Solver: ilp (via ILP)"),
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
            "--edges",
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
    assert!(
        stdout.contains("Solver: ilp (via ILP)"),
        "Non-ILP problem solved with ILP should show auto-reduction indicator, got: {stdout}"
    );
    assert!(stdout.contains("Problem: MaximumIndependentSet"));

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
            "--edges",
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
            "--edges",
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
    assert!(stdout.contains("Problem"));
    assert!(stdout.contains("via"));

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
            "--edges",
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
    assert!(stdout.contains("Problem"));
    assert!(stdout.contains("via"));

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
            "--edges",
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
            "--edges",
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
            "--edges",
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
            "--edges",
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
            "--edges",
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
            "3SAT",
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
    let content = std::fs::read_to_string(&output_file).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert_eq!(json["type"], "MaximumMatching");
    std::fs::remove_file(&output_file).ok();
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
            "--edges",
            "0-1,1-2,2-0",
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
    std::fs::remove_file(&output_file).ok();
}

#[test]
fn test_create_without_output() {
    // Create without -o prints JSON to stdout (not just "Created ...")
    let output = pred()
        .args(["create", "MIS", "--edges", "0-1,1-2"])
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

// ---- Error cases ----

#[test]
fn test_create_unknown_problem() {
    let output = pred()
        .args(["create", "NonExistent", "--edges", "0-1"])
        .output()
        .unwrap();
    assert!(!output.status.success());
}

#[test]
fn test_create_missing_edges() {
    let output = pred().args(["create", "MIS"]).output().unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--edges"));
}

#[test]
fn test_create_kcoloring_missing_k() {
    let output = pred()
        .args(["create", "KColoring", "--edges", "0-1,1-2"])
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
            "--edges",
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
            "--edges",
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
    assert!(json["variants"].is_array());
    assert!(json["reduces_to"].is_array());
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
            "--edges",
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
            "--edges",
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
fn test_reduce_human_output() {
    // Without --json or -o, reduce shows human-readable summary
    let problem_file = std::env::temp_dir().join("pred_test_reduce_human.json");
    let create_out = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "MIS",
            "--edges",
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
        stdout.contains("Reduced"),
        "expected 'Reduced' in stdout, got: {stdout}"
    );
    assert!(
        stdout.contains("MaximumIndependentSet"),
        "expected 'MaximumIndependentSet' in stdout, got: {stdout}"
    );
    assert!(
        stdout.contains("QUBO"),
        "expected 'QUBO' in stdout, got: {stdout}"
    );
    // Should NOT be parseable as JSON
    assert!(
        serde_json::from_str::<serde_json::Value>(&stdout).is_err(),
        "stdout should not be valid JSON in human-readable mode, got: {stdout}"
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
            "--edges",
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
            "--edges",
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
fn test_to_outgoing() {
    let output = pred()
        .args(["to", "MIS", "--hops", "2"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("MaximumIndependentSet"));
    assert!(stdout.contains("reachable problems"));
    // Should contain tree characters
    assert!(stdout.contains("├── ") || stdout.contains("└── "));
}

#[test]
fn test_from_incoming() {
    let output = pred()
        .args(["from", "QUBO", "--hops", "1"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("QUBO"));
    assert!(stdout.contains("incoming"));
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
    let output = pred()
        .args(["to", "MIS", "--hops", "1"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    // Variant info should appear in the tree output
    assert!(
        stdout.contains("{graph=") || stdout.contains("(default)"),
        "expected variant info in tree output, got: {stdout}"
    );
}

#[test]
fn test_from_shows_variant_info() {
    let output = pred()
        .args(["from", "QUBO", "--hops", "1"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    // Variant info should appear in the tree output
    assert!(
        stdout.contains("{graph=") || stdout.contains("{weight=") || stdout.contains("(default)"),
        "expected variant info in tree output, got: {stdout}"
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
    assert!(stdout.contains("reachable problems"));
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
            "--edges",
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
            "--edges",
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
            "--edges",
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
    assert!(
        stdout.contains("Solution"),
        "stdout should still contain solution with -q, got: {stdout}"
    );

    std::fs::remove_file(&problem_file).ok();
}

// ---- Stdin/pipe support tests ----

#[test]
fn test_create_pipe_to_solve() {
    // pred create MIS --edges 0-1,1-2 | pred solve - --solver brute-force
    let create_out = pred()
        .args(["create", "MIS", "--edges", "0-1,1-2"])
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
    assert!(
        stdout.contains("Solution"),
        "stdout should contain Solution, got: {stdout}"
    );
}

#[test]
fn test_create_pipe_to_evaluate() {
    // pred create MIS --edges 0-1,1-2 | pred evaluate - --config 1,0,1
    let create_out = pred()
        .args(["create", "MIS", "--edges", "0-1,1-2"])
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
    // pred create MIS --edges 0-1,1-2 | pred reduce - --to QUBO
    let create_out = pred()
        .args(["create", "MIS", "--edges", "0-1,1-2"])
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
            "--edges",
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
    assert!(
        stdout.contains("Type: MaximumIndependentSet"),
        "expected 'Type: MaximumIndependentSet', got: {stdout}"
    );
    assert!(stdout.contains("Size:"), "expected 'Size:', got: {stdout}");
    assert!(
        stdout.contains("Variables:"),
        "expected 'Variables:', got: {stdout}"
    );
    assert!(
        stdout.contains("Solvers:"),
        "expected 'Solvers:', got: {stdout}"
    );
    assert!(
        stdout.contains("Reduces to:"),
        "expected 'Reduces to:', got: {stdout}"
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
            "--edges",
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
    assert!(
        stdout.contains("Bundle"),
        "expected 'Bundle' in output, got: {stdout}"
    );
    assert!(
        stdout.contains("Source:"),
        "expected 'Source:' in output, got: {stdout}"
    );
    assert!(
        stdout.contains("Target:"),
        "expected 'Target:' in output, got: {stdout}"
    );
    assert!(
        stdout.contains("Path:"),
        "expected 'Path:' in output, got: {stdout}"
    );

    std::fs::remove_file(&problem_file).ok();
    std::fs::remove_file(&bundle_file).ok();
}

#[test]
fn test_inspect_stdin() {
    // Test pipe: create | inspect -
    let create_out = pred()
        .args(["create", "MIS", "--edges", "0-1,1-2"])
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
            "--edges",
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
    assert!(json["size"].is_array());
    assert!(json["solvers"].is_array());
    assert!(json["reduces_to"].is_array());

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
