use std::process::Command;

fn pred() -> Command {
    Command::new(env!("CARGO_BIN_EXE_pred"))
}

fn write_named_route(source: &str, target: &str, names: &[&str], output: &std::path::Path) {
    let command = pred()
        .args(["path", source, target, "--max-paths", "999", "--json"])
        .output()
        .unwrap();
    assert!(
        command.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&command.stderr)
    );
    let envelope: serde_json::Value = serde_json::from_slice(&command.stdout).unwrap();
    let entry = envelope["paths"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| {
            let edges = entry["path"].as_array().unwrap();
            let mut actual = vec![edges[0]["from"]["name"].as_str().unwrap()];
            actual.extend(
                edges
                    .iter()
                    .map(|edge| edge["to"]["name"].as_str().unwrap()),
            );
            actual == names
        })
        .expect("requested route must be present in path enumeration");
    std::fs::write(output, serde_json::to_vec_pretty(entry).unwrap()).unwrap();
}

fn reduce_named_to_file(
    problem: &std::path::Path,
    source: &str,
    target: &str,
    names: &[&str],
    output: &std::path::Path,
) -> std::process::Output {
    let route = output.with_extension("route.json");
    write_named_route(source, target, names, &route);
    let result = pred()
        .args([
            "-o",
            output.to_str().unwrap(),
            "reduce",
            problem.to_str().unwrap(),
            "--via",
            route.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    std::fs::remove_file(route).ok();
    result
}

fn write_direct_route(source: &str, target: &str, output: &std::path::Path) {
    let command = pred()
        .args(["path", source, target, "--max-paths", "999", "--json"])
        .output()
        .unwrap();
    assert!(command.status.success());
    let envelope: serde_json::Value = serde_json::from_slice(&command.stdout).unwrap();
    let route = envelope["paths"]
        .as_array()
        .unwrap()
        .iter()
        .find(|path| path["steps"] == 1)
        .expect("advertised direct reduction must have a direct route");
    std::fs::write(output, serde_json::to_vec_pretty(route).unwrap()).unwrap();
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
    assert!(stdout.contains("Registered catalog"));
    assert!(stdout.contains("graph"));
    for category in ["algebraic", "formula", "graph", "misc", "set"] {
        assert!(stdout.contains(category));
    }
    assert!(!stdout.contains("MaximumIndependentSet"));
    assert!(stdout.lines().count() < 30, "default list is too verbose");
}

#[test]
fn test_list_filters_by_category() {
    let output = pred()
        .args(["list", "--category", "formula"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("KSatisfiability"));
    assert!(!stdout.contains("MaximumIndependentSet"));
}

#[test]
fn test_list_json_respects_category_filter() {
    let output = pred()
        .args(["list", "--category", "formula", "--json"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let variants = json["variants"].as_array().unwrap();
    assert_eq!(json["num_types"], 9);
    assert!(variants
        .iter()
        .all(|variant| variant["name"] != "MaximumIndependentSet"));
    assert!(variants
        .iter()
        .all(|variant| variant["category"] == "formula"));
    assert!(variants
        .iter()
        .any(|variant| variant["name"] == "KSatisfiability/K3"));
}

#[test]
fn test_list_category_rejects_unknown_value() {
    let output = pred()
        .args(["list", "--category", "unknown"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("unknown problem category `unknown`"));
    assert!(stderr.contains("algebraic, formula, graph, misc, set"));
}

#[test]
fn test_list_searches_variant_aliases() {
    let output = pred().args(["list", "3SAT"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("KSatisfiability"));
    assert!(stdout.contains("3SAT"));
}

#[test]
fn test_list_includes_undirected_two_commodity_integral_flow() {
    let output = pred()
        .args(["list", "UndirectedTwoCommodity"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("UndirectedTwoCommodityIntegralFlow"));
}

#[test]
fn test_list_includes_integral_flow_homologous_arcs() {
    let output = pred()
        .args(["list", "IntegralFlowHomologousArcs"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("IntegralFlowHomologousArcs"));
}

#[test]
fn test_solve_help_mentions_string_to_string_correction_bruteforce() {
    let output = pred().args(["solve", "--help"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("StringToStringCorrection"),
        "stdout: {stdout}"
    );
    assert!(stdout.contains("--solver brute-force"), "stdout: {stdout}");
}

#[test]
fn test_list_rules() {
    let output = pred()
        .args(["list", "--rules", "--all", "--verbose"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Registered reduction rules:"));
    assert!(stdout.contains("Source"));
    assert!(stdout.contains("Target"));
    assert!(stdout.contains("Size change"));
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
    assert!(rules[0]["size_contract"].is_string());
}

#[test]
fn test_list_rules_searches_problem_aliases() {
    let output = pred().args(["list", "--rules", "3SAT"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("KSatisfiability"));
}

#[test]
fn test_list_rules_json_respects_query() {
    let output = pred()
        .args(["list", "--rules", "3SAT", "--json"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let rules = json["rules"].as_array().unwrap();
    assert_eq!(json["num_rules"].as_u64().unwrap() as usize, rules.len());
    assert!(rules.iter().all(|rule| {
        rule["source"].as_str().unwrap().contains("KSatisfiability")
            || rule["target"].as_str().unwrap().contains("KSatisfiability")
    }));
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
fn test_show_balanced_complete_bipartite_subgraph_complexity() {
    let output = pred()
        .args(["show", "BalancedCompleteBipartiteSubgraph"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("1.3803^num_vertices"),
        "expected updated complexity metadata, got: {stdout}"
    );
}

#[test]
fn test_create_stacker_crane_schema_help_uses_documented_flags() {
    let output = pred().args(["create", "StackerCrane"]).output().unwrap();
    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("StackerCrane"), "stderr: {stderr}");
    assert!(stderr.contains("--arcs"), "stderr: {stderr}");
    assert!(stderr.contains("--graph"), "stderr: {stderr}");
    assert!(stderr.contains("--arc-lengths"), "stderr: {stderr}");
    assert!(stderr.contains("--edge-lengths"), "stderr: {stderr}");
    assert!(stderr.contains("--num-vertices"), "stderr: {stderr}");
    assert!(!stderr.contains("--bound"), "stderr: {stderr}");
    assert!(!stderr.contains("--biedges"), "stderr: {stderr}");
    assert!(!stderr.contains("--arc-weights"), "stderr: {stderr}");
    assert!(!stderr.contains("--edge-weights"), "stderr: {stderr}");
}

#[test]
fn test_solve_balanced_complete_bipartite_subgraph_default_solver_uses_ilp() {
    let tmp = std::env::temp_dir().join("pred_test_bcbs_problem.json");
    let create = pred()
        .args([
            "create",
            "--example",
            "BalancedCompleteBipartiteSubgraph",
            "--json",
        ])
        .output()
        .unwrap();
    assert!(create.status.success());
    std::fs::write(&tmp, create.stdout).unwrap();

    let solve = pred()
        .args(["solve", tmp.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(
        solve.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&solve.stderr)
    );
    let stdout = String::from_utf8(solve.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["problem"], "BalancedCompleteBipartiteSubgraph");
    assert_eq!(json["solver"]["kind"], "ilp");
    assert!(json["solver"]["reduction_path"]
        .as_array()
        .and_then(|path| path.last())
        .and_then(|step| step.as_str())
        .is_some_and(|step| step.starts_with("ILP<")));
    assert_eq!(json["evaluation"], "Or(true)");
    assert!(
        json["solution"]
            .as_array()
            .is_some_and(|solution| !solution.is_empty()),
        "expected a non-empty solution array, got: {stdout}"
    );

    std::fs::remove_file(tmp).ok();
}

#[test]
fn test_path_enumerates_without_mode_or_sizes() {
    let output = pred().args(["path", "MIS", "QUBO"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Found"));
    assert!(stdout.contains("paths from"));
}

#[test]
fn test_path_concrete_execution_is_deterministic_and_measures_constructed_target() {
    let instance = std::env::temp_dir().join("pred_path_concrete_mis.json");
    std::fs::write(
        &instance,
        r#"{"type":"MaximumIndependentSet","variant":{"graph":"SimpleGraph","weight":"i32"},"data":{"graph":{"num_vertices":5,"edges":[[0,1],[1,2],[2,3],[3,4]]},"weights":[1,1,1,1,1]}}"#,
    )
    .unwrap();
    let run = || {
        let output = pred()
            .args([
                "path",
                "MIS/SimpleGraph/i32",
                "MaximumClique/SimpleGraph/i32",
                instance.to_str().unwrap(),
                "--json",
            ])
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout).unwrap()
    };
    let first = run();
    let second = run();
    std::fs::remove_file(instance).ok();
    assert_eq!(first, second);
    let json: serde_json::Value = serde_json::from_str(&first).unwrap();
    let overall = json["paths"][0]["actual_target_size"]["fields"]
        .as_array()
        .unwrap();
    let value = |field: &str| &overall.iter().find(|item| item["field"] == field).unwrap()["value"];
    assert_eq!(value("num_vertices"), 5);
    assert_eq!(value("num_edges"), 6);
    assert!(json.get("comparison").is_none());
    assert!(json.get("pareto_frontier").is_none());
    assert!(json["paths"][0].get("pareto_nondominated").is_none());
}

#[test]
fn test_path_selection_defaults_to_pareto_and_all_returns_every_candidate() {
    let instance = std::env::temp_dir().join("pred_path_selection_mis.json");
    std::fs::write(
        &instance,
        r#"{"type":"MaximumIndependentSet","variant":{"graph":"SimpleGraph","weight":"i32"},"data":{"graph":{"num_vertices":5,"edges":[[0,1],[1,2],[2,3],[3,4]]},"weights":[1,1,1,1,1]}}"#,
    )
    .unwrap();
    let run = |max_paths: &str, selection: Option<&str>| {
        let mut args = vec![
            "path",
            "MIS/SimpleGraph/i32",
            "QUBO",
            instance.to_str().unwrap(),
            "--max-paths",
            max_paths,
            "--json",
        ];
        if let Some(selection) = selection {
            args.extend(["--selection", selection]);
        }
        let output = pred().args(args).output().unwrap();
        assert!(
            output.status.success(),
            "stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        serde_json::from_slice::<serde_json::Value>(&output.stdout).unwrap()
    };

    let pareto = run("3", None);
    let pareto_capped = run("1", None);
    let all = run("3", Some("all"));
    std::fs::remove_file(instance).unwrap();

    assert_eq!(pareto["paths"].as_array().unwrap().len(), 1);
    assert_eq!(pareto_capped["paths"].as_array().unwrap().len(), 1);
    assert_eq!(all["paths"].as_array().unwrap().len(), 3);
    assert_eq!(pareto["truncated"], false);
    assert_eq!(pareto_capped["truncated"], false);
    assert_eq!(all["truncated"], true);
    let pareto_size = &pareto["paths"][0]["actual_target_size"]["fields"];
    assert!(pareto_size
        .as_array()
        .unwrap()
        .iter()
        .any(|field| field["field"] == "num_vars" && field["value"] == 5));
    assert_eq!(pareto_capped["paths"], pareto["paths"]);
}

#[test]
fn test_path_save() {
    let tmp = std::env::temp_dir().join("pred_test_path.json");
    let output = pred()
        .args([
            "path",
            "MIS/SimpleGraph/i32",
            "MaximumClique/SimpleGraph/i32",
            "-o",
            tmp.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(tmp.exists());
    let content = std::fs::read_to_string(&tmp).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert!(json.get("path").is_none());
    assert!(json["paths"]
        .as_array()
        .is_some_and(|paths| !paths.is_empty()));
    std::fs::remove_file(&tmp).ok();
}

#[test]
fn test_path_max_paths_caps_selected_output() {
    let output = pred()
        .args(["path", "MIS", "QUBO", "--max-paths", "1", "--json"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["paths"].as_array().unwrap().len(), 1);
    assert_eq!(json["truncated"], true);
    assert!(json.get("returned").is_none());
    assert!(json.get("max_paths").is_none());
}

#[test]
fn test_path_rejects_max_paths_above_output_limit() {
    let output = pred()
        .args([
            "path",
            "MIS",
            "QUBO",
            "--selection",
            "all",
            "--max-paths",
            "1000",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("max_paths must not exceed 999"));
}

#[test]
fn test_path_set_save() {
    let file = std::env::temp_dir().join("pred_test_paths.json");
    let output = pred()
        .args(["path", "MIS", "QUBO", "-o", file.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let content = std::fs::read_to_string(&file).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert!(json["paths"].is_array());

    std::fs::remove_file(&file).ok();
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
fn test_create_balanced_complete_bipartite_subgraph_help_uses_bipartite_flags() {
    let output = pred()
        .args(["create", "BalancedCompleteBipartiteSubgraph"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--left"), "stderr: {stderr}");
    assert!(stderr.contains("--right"), "stderr: {stderr}");
    assert!(stderr.contains("--biedges"), "stderr: {stderr}");
    assert!(!stderr.contains("--left-size"), "stderr: {stderr}");
    assert!(!stderr.contains("--right-size"), "stderr: {stderr}");
    assert!(!stderr.contains("--edges"), "stderr: {stderr}");
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
            "graph": {"num_vertices": 4, "edges": [[0,1],[1,2],[2,3]]},
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
    assert!(stdout.contains("Max(2)"), "stdout: {stdout}");
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
fn test_evaluate_consecutive_block_minimization_rejects_ragged_matrix() {
    let problem_json = r#"{
        "type": "ConsecutiveBlockMinimization",
        "data": {
            "matrix": [[true, false], [true]],
            "bound": 1
        }
    }"#;
    let tmp = std::env::temp_dir().join("pred_test_eval_cbm_ragged_matrix.json");
    std::fs::write(&tmp, problem_json).unwrap();

    let output = pred()
        .args(["evaluate", tmp.to_str().unwrap(), "--config", "0,1"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("same length"));
    assert!(!stderr.contains("panicked at"), "stderr: {stderr}");
    std::fs::remove_file(&tmp).ok();
}

#[test]
fn test_evaluate_multiple_choice_branching_rejects_invalid_partition_without_panicking() {
    let problem_json = r#"{
        "type": "MultipleChoiceBranching",
        "variant": {"weight": "i32"},
        "data": {
            "graph": {"num_vertices": 2, "arcs": [[0,1]]},
            "weights": [1],
            "partition": [[1]],
            "threshold": 1
        }
    }"#;
    let tmp = std::env::temp_dir().join("pred_test_eval_invalid_mcb_partition.json");
    std::fs::write(&tmp, problem_json).unwrap();

    let output = pred()
        .args(["evaluate", tmp.to_str().unwrap(), "--config", "1"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        !stderr.contains("panicked at"),
        "invalid partition should return a user error, got panic output: {stderr}"
    );
    assert!(
        stderr.contains("partition"),
        "stderr should mention the invalid partition: {stderr}"
    );
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
    assert!(stderr.contains("missing required construction input(s): capacities"));
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
    assert!(stderr.contains("invalid digit found in string"));
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
    assert!(stderr.contains("capacities length must match graph edge count"));
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
    assert!(stderr.contains("source_1 must be less than num_vertices"));
    assert!(stderr.contains("Usage: pred create UndirectedTwoCommodityIntegralFlow"));
    assert!(!stderr.contains("panicked at"), "stderr: {stderr}");
}

#[test]
fn test_create_integral_flow_bundles() {
    let output = pred()
        .args([
            "create",
            "IntegralFlowBundles",
            "--arcs",
            "0>1,0>2,1>3,2>3,1>2,2>1",
            "--bundles",
            "0,1;2,5;3,4",
            "--bundle-capacities",
            "1,1,1",
            "--source",
            "0",
            "--sink",
            "3",
            "--requirement",
            "1",
            "--num-vertices",
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
    assert_eq!(json["type"], "IntegralFlowBundles");
    assert_eq!(json["variant"], serde_json::json!({}));
    assert_eq!(json["data"]["graph"]["num_vertices"], 4);
    assert_eq!(json["data"]["graph"]["arcs"].as_array().unwrap().len(), 6);
    assert_eq!(
        json["data"]["bundles"],
        serde_json::json!([[0, 1], [2, 5], [3, 4]])
    );
    assert_eq!(
        json["data"]["bundle_capacities"],
        serde_json::json!([1, 1, 1])
    );
    assert_eq!(json["data"]["source"], 0);
    assert_eq!(json["data"]["sink"], 3);
    assert_eq!(json["data"]["requirement"], 1);
}

#[test]
fn test_create_integral_flow_bundles_missing_bundles_shows_usage() {
    let output = pred()
        .args([
            "create",
            "IntegralFlowBundles",
            "--arcs",
            "0>1,0>2,1>3,2>3,1>2,2>1",
            "--bundle-capacities",
            "1,1,1",
            "--source",
            "0",
            "--sink",
            "3",
            "--requirement",
            "1",
            "--num-vertices",
            "4",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("missing required construction input(s): bundles"));
    assert!(stderr.contains("Usage: pred create IntegralFlowBundles"));
}

#[test]
fn test_create_integral_flow_bundles_rejects_wrong_bundle_capacity_count() {
    let output = pred()
        .args([
            "create",
            "IntegralFlowBundles",
            "--arcs",
            "0>1,0>2,1>3,2>3,1>2,2>1",
            "--bundles",
            "0,1;2,5;3,4",
            "--bundle-capacities",
            "1,1",
            "--source",
            "0",
            "--sink",
            "3",
            "--requirement",
            "1",
            "--num-vertices",
            "4",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("bundles length must match bundle_capacities length"));
    assert!(stderr.contains("Usage: pred create IntegralFlowBundles"));
}

#[test]
fn test_create_integral_flow_bundles_rejects_out_of_range_bundle_arc() {
    let output = pred()
        .args([
            "create",
            "IntegralFlowBundles",
            "--arcs",
            "0>1,0>2,1>3,2>3,1>2,2>1",
            "--bundles",
            "0,1;2,7;3,4",
            "--bundle-capacities",
            "1,1,1",
            "--source",
            "0",
            "--sink",
            "3",
            "--requirement",
            "1",
            "--num-vertices",
            "4",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("bundle 1 arc is out of range"));
    assert!(stderr.contains("Usage: pred create IntegralFlowBundles"));
    assert!(!stderr.contains("panicked at"), "stderr: {stderr}");
}

#[test]
fn test_create_integral_flow_bundles_example() {
    let output = pred()
        .args(["create", "--example", "IntegralFlowBundles"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["type"], "IntegralFlowBundles");
    assert_eq!(json["data"]["requirement"], 1);
    assert_eq!(json["data"]["bundles"].as_array().unwrap().len(), 3);
}

#[test]
fn test_create_integral_flow_homologous_arcs() {
    let output_file =
        std::env::temp_dir().join("pred_test_create_integral_flow_homologous_arcs.json");
    let output = pred()
        .args([
            "-o",
            output_file.to_str().unwrap(),
            "create",
            "IntegralFlowHomologousArcs",
            "--arcs",
            "0>1,0>2,1>3,2>3,1>4,2>4,3>5,4>5",
            "--capacities",
            "1,1,1,1,1,1,1,1",
            "--source",
            "0",
            "--sink",
            "5",
            "--requirement",
            "2",
            "--homologous-pairs",
            "2=5;4=3",
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
    assert_eq!(json["type"], "IntegralFlowHomologousArcs");

    std::fs::remove_file(&output_file).ok();
}

#[test]
fn test_create_integral_flow_homologous_arcs_requires_homologous_pairs() {
    let output = pred()
        .args([
            "create",
            "IntegralFlowHomologousArcs",
            "--arcs",
            "0>1,0>2,1>3,2>3,1>4,2>4,3>5,4>5",
            "--capacities",
            "1,1,1,1,1,1,1,1",
            "--source",
            "0",
            "--sink",
            "5",
            "--requirement",
            "2",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("missing required construction input(s): homologous_pairs"));
    assert!(stderr.contains("Usage: pred create IntegralFlowHomologousArcs"));
}

#[test]
fn test_create_integral_flow_homologous_arcs_rejects_invalid_pair_token() {
    let output = pred()
        .args([
            "create",
            "IntegralFlowHomologousArcs",
            "--arcs",
            "0>1,0>2,1>3,2>3,1>4,2>4,3>5,4>5",
            "--capacities",
            "1,1,1,1,1,1,1,1",
            "--source",
            "0",
            "--sink",
            "5",
            "--requirement",
            "2",
            "--homologous-pairs",
            "2-5;4=3",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("expected format left=right"));
    assert!(stderr.contains("Usage: pred create IntegralFlowHomologousArcs"));
}

#[test]
fn test_create_integral_flow_with_multipliers() {
    let output = pred()
        .args([
            "create",
            "IntegralFlowWithMultipliers",
            "--arcs",
            "0>1,0>2,0>3,0>4,0>5,0>6,1>7,2>7,3>7,4>7,5>7,6>7",
            "--capacities",
            "1,1,1,1,1,1,2,3,4,5,6,4",
            "--source",
            "0",
            "--sink",
            "7",
            "--multipliers",
            "1,2,3,4,5,6,4,1",
            "--requirement",
            "12",
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
    assert_eq!(json["type"], "IntegralFlowWithMultipliers");
    assert_eq!(json["variant"], serde_json::json!({}));
    assert_eq!(json["data"]["source"], 0);
    assert_eq!(json["data"]["sink"], 7);
    assert_eq!(json["data"]["requirement"], 12);
    assert_eq!(
        json["data"]["multipliers"],
        serde_json::json!([1, 2, 3, 4, 5, 6, 4, 1])
    );
    assert_eq!(
        json["data"]["capacities"],
        serde_json::json!([1, 1, 1, 1, 1, 1, 2, 3, 4, 5, 6, 4])
    );
}

#[test]
fn test_create_integral_flow_with_multipliers_missing_multipliers_shows_usage() {
    let output = pred()
        .args([
            "create",
            "IntegralFlowWithMultipliers",
            "--arcs",
            "0>1,0>2,1>3,2>3",
            "--capacities",
            "1,1,2,2",
            "--source",
            "0",
            "--sink",
            "3",
            "--requirement",
            "2",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("missing required construction input(s): multipliers"));
    assert!(stderr.contains("Usage: pred create IntegralFlowWithMultipliers"));
}

#[test]
fn test_create_integral_flow_with_multipliers_rejects_wrong_multiplier_count() {
    let output = pred()
        .args([
            "create",
            "IntegralFlowWithMultipliers",
            "--arcs",
            "0>1,0>2,1>3,2>3",
            "--capacities",
            "1,1,2,2",
            "--source",
            "0",
            "--sink",
            "3",
            "--multipliers",
            "1,2,3",
            "--requirement",
            "2",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("multipliers length must match num_vertices"));
    assert!(stderr.contains("Usage: pred create IntegralFlowWithMultipliers"));
}

#[test]
fn test_create_integral_flow_with_multipliers_rejects_zero_nonterminal_multiplier() {
    let output = pred()
        .args([
            "create",
            "IntegralFlowWithMultipliers",
            "--arcs",
            "0>1,0>2,1>3,2>3",
            "--capacities",
            "1,1,2,2",
            "--source",
            "0",
            "--sink",
            "3",
            "--multipliers",
            "1,0,3,1",
            "--requirement",
            "2",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("non-terminal multipliers must be positive"));
    assert!(stderr.contains("Usage: pred create IntegralFlowWithMultipliers"));
}

#[test]
fn test_create_integral_flow_with_multipliers_rejects_identical_source_and_sink() {
    let output = pred()
        .args([
            "create",
            "IntegralFlowWithMultipliers",
            "--arcs",
            "0>1,0>2,1>3,2>3",
            "--capacities",
            "1,1,2,2",
            "--source",
            "0",
            "--sink",
            "0",
            "--multipliers",
            "1,2,3,1",
            "--requirement",
            "2",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("source and sink must be distinct"));
    assert!(stderr.contains("Usage: pred create IntegralFlowWithMultipliers"));
}

#[test]
fn test_create_consecutive_block_minimization_rejects_ragged_matrix() {
    let output = pred()
        .args([
            "create",
            "ConsecutiveBlockMinimization",
            "--matrix",
            "1;1,0",
            "--bound-k",
            "2",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("All rows in --matrix must have the same length"));
    assert!(stderr.contains("Usage: pred create ConsecutiveBlockMinimization"));
    assert!(!stderr.contains("panicked at"), "stderr: {stderr}");
}

#[test]
fn test_reduce() {
    let problem_json = r#"{
        "type": "MIS",
        "variant": {"graph": "SimpleGraph", "weight": "i32"},
        "data": {
            "graph": {"num_vertices": 4, "edges": [[0,1],[1,2],[2,3]]},
            "weights": [1, 1, 1, 1]
        }
    }"#;
    let input = std::env::temp_dir().join("pred_test_reduce_in.json");
    let output_file = std::env::temp_dir().join("pred_test_reduce_out.json");
    let route_file = std::env::temp_dir().join("pred_test_reduce_route.json");
    std::fs::write(&input, problem_json).unwrap();
    write_named_route(
        "MIS/SimpleGraph/i32",
        "QUBO",
        &[
            "MaximumIndependentSet",
            "MaximumSetPacking",
            "MaximumSetPacking",
            "QUBO",
        ],
        &route_file,
    );

    let output = pred()
        .args([
            "-o",
            output_file.to_str().unwrap(),
            "reduce",
            input.to_str().unwrap(),
            "--via",
            route_file.to_str().unwrap(),
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
    std::fs::remove_file(&route_file).ok();
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

    // 2. Explicitly extract a named route from the enumerated path set.
    let path_file = std::env::temp_dir().join("pred_test_reduce_via_path.json");
    write_named_route(
        "MIS/SimpleGraph/i32",
        "QUBO",
        &[
            "MaximumIndependentSet",
            "MaximumSetPacking",
            "MaximumSetPacking",
            "QUBO",
        ],
        &path_file,
    );

    // 3. Reduce via path file
    let output_file = std::env::temp_dir().join("pred_test_reduce_via_out.json");
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
fn test_reduce_rejects_discontinuous_explicit_route() {
    let problem_file = std::env::temp_dir().join("pred_test_reduce_discontinuous_in.json");
    let route_file = std::env::temp_dir().join("pred_test_reduce_discontinuous_route.json");
    let create = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "MIS/SimpleGraph/i32",
            "--graph",
            "0-1,1-2",
            "--weights",
            "1,1,1",
        ])
        .output()
        .unwrap();
    assert!(create.status.success());
    write_named_route(
        "MIS/SimpleGraph/i32",
        "QUBO",
        &[
            "MaximumIndependentSet",
            "MaximumSetPacking",
            "MaximumSetPacking",
            "QUBO",
        ],
        &route_file,
    );
    let mut route: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&route_file).unwrap()).unwrap();
    route["path"][1]["from"]["name"] = serde_json::json!("MinimumVertexCover");
    std::fs::write(&route_file, serde_json::to_vec_pretty(&route).unwrap()).unwrap();

    let output = pred()
        .args([
            "reduce",
            problem_file.to_str().unwrap(),
            "--via",
            route_file.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("not continuous"));

    std::fs::remove_file(problem_file).ok();
    std::fs::remove_file(route_file).ok();
}

/// A path-set envelope is not itself an executable route.
#[test]
fn test_reduce_rejects_unselected_path_set() {
    // 1. Create a small source problem (small so the target brute-force stays tiny).
    let problem_file = std::env::temp_dir().join("pred_test_reduce_via_bare_in.json");
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

    // 2. Save the path set without choosing a route.
    let path_file = std::env::temp_dir().join("pred_test_reduce_via_bare_path.json");
    let path_out = pred()
        .args([
            "path",
            "MaximumIndependentSet/SimpleGraph/i32",
            "MaximumClique/SimpleGraph/i32",
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
            "--via",
            path_file.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(!reduce_out.status.success());
    assert!(String::from_utf8_lossy(&reduce_out.stderr).contains("explicit route"));

    std::fs::remove_file(&problem_file).ok();
    std::fs::remove_file(&path_file).ok();
}

/// Every path-set item carries its route, while the envelope selects none.
#[test]
fn test_path_set_envelope_has_only_per_item_paths() {
    let output = pred()
        .args([
            "path",
            "MIS/SimpleGraph/i32",
            "MaximumClique/SimpleGraph/i32",
            "--json",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let json: serde_json::Value =
        serde_json::from_str(&String::from_utf8(output.stdout).unwrap()).unwrap();

    assert!(json["paths"]
        .as_array()
        .is_some_and(|paths| !paths.is_empty()));

    assert!(json.get("path").is_none());
    let path = json["paths"][0]["path"]
        .as_array()
        .expect("path-set item route");
    assert!(!path.is_empty(), "path-set item must have ≥ 1 step");
    let first = &path[0];
    assert!(first["from"]["name"].is_string(), "step needs from.name");
    assert!(first["to"]["name"].is_string(), "step needs to.name");
    assert_eq!(first["from"]["name"], "MaximumIndependentSet");
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
    write_named_route(
        "MIS/SimpleGraph/i32",
        "QUBO",
        &[
            "MaximumIndependentSet",
            "MaximumSetPacking",
            "MaximumSetPacking",
            "QUBO",
        ],
        &path_file,
    );

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
fn test_reduce_via_preserves_explicit_target_variant() {
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
    write_named_route(
        "MIS/SimpleGraph/i32",
        "ILP/bool",
        &["MaximumIndependentSet", "MaximumClique", "ILP"],
        &path_file,
    );

    let reduce_out = pred()
        .args([
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
    let bundle: serde_json::Value = serde_json::from_slice(&reduce_out.stdout).unwrap();
    assert_eq!(bundle["target"]["variant"]["variable"], "bool");

    std::fs::remove_file(&problem_file).ok();
    std::fs::remove_file(&path_file).ok();
}

#[test]
fn test_reduce_missing_via() {
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
    assert!(stderr.contains("--via"));

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
fn test_create_multiprocessor_scheduling() {
    let output_file = std::env::temp_dir().join("pred_test_create_multiprocessor_scheduling.json");
    let output = pred()
        .args([
            "-o",
            output_file.to_str().unwrap(),
            "create",
            "MultiprocessorScheduling",
            "--lengths",
            "4,5,3,2,6",
            "--num-processors",
            "2",
            "--deadline",
            "10",
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
    assert_eq!(json["type"], "MultiprocessorScheduling");
    assert_eq!(json["data"]["lengths"], serde_json::json!([4, 5, 3, 2, 6]));
    assert_eq!(json["data"]["num_processors"], 2);
    assert_eq!(json["data"]["deadline"], 10);

    std::fs::remove_file(&output_file).ok();
}

#[test]
fn test_create_multiprocessor_scheduling_rejects_zero_processors() {
    let output = pred()
        .args([
            "create",
            "MultiprocessorScheduling",
            "--lengths",
            "4,5,3,2,6",
            "--num-processors",
            "0",
            "--deadline",
            "10",
        ])
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains("panicked at"),
        "zero processors should return a user error, got panic output: {stderr}"
    );
    assert!(
        stderr.contains("num_processors must be positive"),
        "expected a validation error for zero processors, got: {stderr}"
    );
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
            "--universe-size",
            "6",
            "--subsets",
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
fn test_create_d2cif_alias() {
    let output_file = std::env::temp_dir().join("pred_test_create_d2cif.json");
    let output = pred()
        .args([
            "-o",
            output_file.to_str().unwrap(),
            "create",
            "D2CIF",
            "--arcs",
            "0>2,0>3,1>2,1>3,2>4,2>5,3>4,3>5",
            "--capacities",
            "1,1,1,1,1,1,1,1",
            "--source-1",
            "0",
            "--sink-1",
            "4",
            "--source-2",
            "1",
            "--sink-2",
            "5",
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
    assert!(output_file.exists());

    let content = std::fs::read_to_string(&output_file).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert_eq!(json["type"], "DirectedTwoCommodityIntegralFlow");

    std::fs::remove_file(&output_file).ok();
}

#[test]
fn test_solve_d2cif_default_solver_uses_ilp() {
    let output_file = std::env::temp_dir().join("pred_test_solve_d2cif.json");
    let create_output = pred()
        .args([
            "-o",
            output_file.to_str().unwrap(),
            "create",
            "D2CIF",
            "--arcs",
            "0>2,0>3,1>2,1>3,2>4,2>5,3>4,3>5",
            "--capacities",
            "1,1,1,1,1,1,1,1",
            "--source-1",
            "0",
            "--sink-1",
            "4",
            "--source-2",
            "1",
            "--sink-2",
            "5",
            "--requirement-1",
            "1",
            "--requirement-2",
            "1",
        ])
        .output()
        .unwrap();
    assert!(
        create_output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&create_output.stderr)
    );

    let solve_output = pred()
        .args(["solve", output_file.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(
        solve_output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&solve_output.stderr)
    );
    let stdout = String::from_utf8(solve_output.stdout).unwrap();
    assert!(
        stdout.contains("\"kind\": \"ilp\""),
        "expected ILP solver output, got: {stdout}"
    );
    assert!(
        stdout.contains("\"reduction_path\""),
        "expected registered ILP pipeline metadata, got: {stdout}"
    );

    std::fs::remove_file(&output_file).ok();
}

#[test]
fn test_inspect_rectilinear_picture_compression_lists_ilp_and_bruteforce() {
    let output_file = std::env::temp_dir().join("pred_test_inspect_rpc.json");
    let create_output = pred()
        .args([
            "-o",
            output_file.to_str().unwrap(),
            "create",
            "RectilinearPictureCompression",
            "--matrix",
            "1,1;1,1",
            "--bound",
            "1",
        ])
        .output()
        .unwrap();
    assert!(
        create_output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&create_output.stderr)
    );

    let inspect_output = pred()
        .args(["inspect", output_file.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(
        inspect_output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&inspect_output.stderr)
    );
    let stdout = String::from_utf8(inspect_output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert!(
        json["solvers"] == serde_json::json!(["ilp", "brute-force"]),
        "inspect should list ILP first when available, got: {json}"
    );

    std::fs::remove_file(&output_file).ok();
}

#[test]
fn test_create_x3c_rejects_duplicate_subset_elements() {
    let output = pred()
        .args([
            "create",
            "X3C",
            "--universe-size",
            "6",
            "--subsets",
            "0,0,1;3,4,5",
        ])
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
fn test_create_comparative_containment() {
    let output_file = std::env::temp_dir().join("pred_test_create_comparative_containment.json");
    let output = pred()
        .args([
            "-o",
            output_file.to_str().unwrap(),
            "create",
            "ComparativeContainment",
            "--universe-size",
            "4",
            "--r-sets",
            "0,1,2,3;0,1",
            "--s-sets",
            "0,1,2,3;2,3",
            "--r-weights",
            "2,5",
            "--s-weights",
            "3,6",
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
    assert_eq!(json["type"], "ComparativeContainment");
    assert_eq!(json["variant"]["weight"], "i32");
    assert_eq!(json["data"]["universe_size"], 4);
    assert_eq!(
        json["data"]["r_sets"],
        serde_json::json!([[0, 1, 2, 3], [0, 1]])
    );
    assert_eq!(
        json["data"]["s_sets"],
        serde_json::json!([[0, 1, 2, 3], [2, 3]])
    );
    assert_eq!(json["data"]["r_weights"], serde_json::json!([2, 5]));
    assert_eq!(json["data"]["s_weights"], serde_json::json!([3, 6]));

    std::fs::remove_file(&output_file).ok();
}

#[test]
fn test_create_comparative_containment_rejects_out_of_range_elements_without_panicking() {
    let output = pred()
        .args([
            "create",
            "ComparativeContainment",
            "--universe-size",
            "4",
            "--r-sets",
            "0,1,4",
            "--s-sets",
            "0,1",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("outside universe of size 4"),
        "stderr: {stderr}"
    );
    assert!(!stderr.contains("panicked at"), "stderr: {stderr}");
}

#[test]
fn test_create_comparative_containment_rejects_nonpositive_weights_without_panicking() {
    let output = pred()
        .args([
            "create",
            "ComparativeContainment",
            "--universe-size",
            "4",
            "--r-sets",
            "0,1",
            "--s-sets",
            "0,1",
            "--r-weights",
            "0",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("positive"), "stderr: {stderr}");
    assert!(!stderr.contains("panicked at"), "stderr: {stderr}");
}

#[test]
fn test_create_set_basis() {
    let output_file = std::env::temp_dir().join("pred_test_create_set_basis.json");
    let output = pred()
        .args([
            "-o",
            output_file.to_str().unwrap(),
            "create",
            "SetBasis",
            "--universe-size",
            "4",
            "--subsets",
            "0,1;1,2;0,2;0,1,2",
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
    assert_eq!(json["type"], "SetBasis");
    assert_eq!(json["data"]["universe_size"], 4);
    assert_eq!(json["data"]["k"], 3);
    assert_eq!(json["data"]["collection"][0], serde_json::json!([0, 1]));

    std::fs::remove_file(&output_file).ok();
}

#[test]
fn test_create_comparative_containment_f64() {
    let output_file =
        std::env::temp_dir().join("pred_test_create_comparative_containment_f64.json");
    let output = pred()
        .args([
            "-o",
            output_file.to_str().unwrap(),
            "create",
            "ComparativeContainment/f64",
            "--universe-size",
            "4",
            "--r-sets",
            "0,1,2,3;0,1",
            "--s-sets",
            "0,1,2,3;2,3",
            "--r-weights",
            "2.5,5.0",
            "--s-weights",
            "3.5,6.0",
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
    assert_eq!(json["type"], "ComparativeContainment");
    assert_eq!(json["variant"]["weight"], "f64");
    assert_eq!(json["data"]["r_weights"], serde_json::json!([2.5, 5.0]));
    assert_eq!(json["data"]["s_weights"], serde_json::json!([3.5, 6.0]));

    std::fs::remove_file(&output_file).ok();
}

#[test]
fn test_create_comparative_containment_one_rejects_nonunit_weights() {
    let output = pred()
        .args([
            "create",
            "ComparativeContainment/One",
            "--universe-size",
            "4",
            "--r-sets",
            "0,1,2,3;0,1",
            "--s-sets",
            "0,1,2,3;2,3",
            "--r-weights",
            "2,5",
            "--s-weights",
            "3,6",
        ])
        .output()
        .unwrap();
    assert!(
        !output.status.success(),
        "stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("expected 1 for One, got 2"),
        "stderr: {stderr}"
    );
}

#[test]
fn test_create_comparative_containment_no_flags_shows_help() {
    let output = pred()
        .args(["create", "ComparativeContainment"])
        .output()
        .unwrap();
    assert!(
        !output.status.success(),
        "should exit non-zero when showing help without data flags"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--universe-size"), "stderr: {stderr}");
    assert!(stderr.contains("--r-sets"), "stderr: {stderr}");
    assert!(stderr.contains("--s-sets"), "stderr: {stderr}");
}

#[test]
fn test_create_minimum_hitting_set() {
    let output_file = std::env::temp_dir().join("pred_test_create_minimum_hitting_set.json");
    let output = pred()
        .args([
            "-o",
            output_file.to_str().unwrap(),
            "create",
            "MinimumHittingSet",
            "--universe-size",
            "6",
            "--subsets",
            "0,1,2;0,3,4;1,3,5;2,4,5;0,1,5;2,3;1,4",
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
    assert_eq!(json["type"], "MinimumHittingSet");
    assert_eq!(json["data"]["universe_size"], 6);
    assert_eq!(
        json["data"]["sets"],
        serde_json::json!([
            [0, 1, 2],
            [0, 3, 4],
            [1, 3, 5],
            [2, 4, 5],
            [0, 1, 5],
            [2, 3],
            [1, 4]
        ])
    );

    std::fs::remove_file(&output_file).ok();
}

#[test]
fn test_create_minimum_hitting_set_rejects_out_of_range_elements_without_panicking() {
    let output = pred()
        .args([
            "create",
            "MinimumHittingSet",
            "--universe-size",
            "4",
            "--subsets",
            "0,1,4;1,2",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("outside universe of size 4"),
        "stderr: {stderr}"
    );
    assert!(!stderr.contains("panicked at"), "stderr: {stderr}");
}

#[test]
fn test_create_set_basis_requires_k() {
    let output = pred()
        .args([
            "create",
            "SetBasis",
            "--universe-size",
            "4",
            "--subsets",
            "0,1;1,2;0,2;0,1,2",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("missing required construction input(s): k"),
        "stderr: {stderr}"
    );
}

#[test]
fn test_create_set_basis_rejects_out_of_range_elements() {
    let output = pred()
        .args([
            "create",
            "SetBasis",
            "--universe-size",
            "4",
            "--subsets",
            "0,4",
            "--k",
            "1",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("outside universe of size 4"),
        "stderr: {stderr}"
    );
    assert!(!stderr.contains("panicked at"), "stderr: {stderr}");
}

#[test]
fn test_create_sequencing_to_minimize_weighted_tardiness() {
    let output_file =
        std::env::temp_dir().join("pred_test_create_weighted_tardiness_sequencing.json");
    let output = pred()
        .args([
            "-o",
            output_file.to_str().unwrap(),
            "create",
            "SequencingToMinimizeWeightedTardiness",
            "--lengths",
            "3,4,2,5,3",
            "--weights",
            "2,3,1,4,2",
            "--deadlines",
            "5,8,4,15,10",
            "--bound",
            "13",
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
    assert_eq!(json["type"], "SequencingToMinimizeWeightedTardiness");
    assert_eq!(json["data"]["lengths"], serde_json::json!([3, 4, 2, 5, 3]));
    assert_eq!(json["data"]["weights"], serde_json::json!([2, 3, 1, 4, 2]));
    assert_eq!(
        json["data"]["deadlines"],
        serde_json::json!([5, 8, 4, 15, 10])
    );
    assert_eq!(json["data"]["bound"], 13);

    std::fs::remove_file(&output_file).ok();
}

#[test]
fn test_create_sequencing_to_minimize_weighted_tardiness_rejects_mismatched_lengths() {
    let output = pred()
        .args([
            "create",
            "SequencingToMinimizeWeightedTardiness",
            "--lengths",
            "3,4,2",
            "--weights",
            "2,3",
            "--deadlines",
            "5,8,4",
            "--bound",
            "13",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("weights length must equal lengths length"),
        "stderr: {stderr}"
    );
}

#[test]
fn test_create_minimum_cardinality_key_problem_help_uses_supported_flags() {
    let output = pred()
        .args(["create", "MinimumCardinalityKey"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--num-attributes"), "stderr: {stderr}");
    assert!(stderr.contains("--dependencies"), "stderr: {stderr}");
}

#[test]
fn test_create_minimum_cardinality_key_allows_empty_lhs_dependency() {
    let output = pred()
        .args([
            "create",
            "MinimumCardinalityKey",
            "--num-attributes",
            "1",
            "--dependencies",
            ">0",
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
    assert_eq!(json["type"], "MinimumCardinalityKey");
    assert_eq!(json["data"]["num_attributes"], 1);
    assert_eq!(json["data"]["dependencies"][0][0], serde_json::json!([]));
    assert_eq!(json["data"]["dependencies"][0][1], serde_json::json!([0]));
}

#[test]
fn test_create_minimum_cardinality_key_missing_num_attributes_message() {
    let output = pred()
        .args(["create", "MinimumCardinalityKey", "--dependencies", "0>0"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("MinimumCardinalityKey requires --num-attributes"));
    assert!(!stderr.contains("--num-vertices"), "stderr: {stderr}");
}

#[test]
fn test_create_two_dimensional_consecutive_sets_accepts_alphabet_size_flag() {
    let output_file =
        std::env::temp_dir().join("pred_test_create_two_dimensional_consecutive_sets.json");
    let output = pred()
        .args([
            "-o",
            output_file.to_str().unwrap(),
            "create",
            "TwoDimensionalConsecutiveSets",
            "--alphabet-size",
            "6",
            "--subsets",
            "0,1,2;3,4,5;1,3;2,4;0,5",
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
    assert_eq!(json["type"], "TwoDimensionalConsecutiveSets");
    assert_eq!(json["data"]["alphabet_size"], 6);
    assert_eq!(json["data"]["subsets"][0], serde_json::json!([0, 1, 2]));

    std::fs::remove_file(&output_file).ok();
}

#[test]
fn test_create_two_dimensional_consecutive_sets_rejects_zero_alphabet_size_without_panic() {
    let output = pred()
        .args([
            "create",
            "TwoDimensionalConsecutiveSets",
            "--alphabet-size",
            "0",
            "--subsets",
            "0",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Alphabet size must be positive"),
        "stderr: {stderr}"
    );
    assert!(!stderr.contains("panicked at"), "stderr: {stderr}");
}

#[test]
fn test_create_two_dimensional_consecutive_sets_rejects_duplicate_elements_without_panic() {
    let output = pred()
        .args([
            "create",
            "TwoDimensionalConsecutiveSets",
            "--alphabet-size",
            "3",
            "--subsets",
            "0,0",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("duplicate element"), "stderr: {stderr}");
    assert!(!stderr.contains("panicked at"), "stderr: {stderr}");
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
    assert!(stdout.contains("Max(2)"), "stdout: {stdout}");

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
fn test_create_multiple_choice_branching() {
    let output_file = std::env::temp_dir().join("pred_test_create_mcb.json");
    let output = pred()
        .args([
            "-o",
            output_file.to_str().unwrap(),
            "create",
            "MultipleChoiceBranching/i32",
            "--arcs",
            "0>1,0>2,1>3,2>3,1>4,3>5,4>5,2>4",
            "--weights",
            "3,2,4,1,2,3,1,3",
            "--partition",
            "0,1;2,3;4,7;5,6",
            "--threshold",
            "10",
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
    assert_eq!(json["type"], "MultipleChoiceBranching");
    assert_eq!(json["variant"]["weight"], "i32");
    assert_eq!(
        json["data"]["weights"],
        serde_json::json!([3, 2, 4, 1, 2, 3, 1, 3])
    );
    assert_eq!(
        json["data"]["partition"],
        serde_json::json!([[0, 1], [2, 3], [4, 7], [5, 6]])
    );
    assert_eq!(json["data"]["threshold"], 10);

    std::fs::remove_file(&output_file).ok();
}

#[test]
fn test_create_model_example_multiple_choice_branching() {
    let output = pred()
        .args(["create", "--example", "MultipleChoiceBranching/i32"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["type"], "MultipleChoiceBranching");
    assert_eq!(json["variant"]["weight"], "i32");
    assert_eq!(json["data"]["threshold"], 10);
    assert_eq!(json["data"]["partition"].as_array().unwrap().len(), 4);
}

#[test]
fn test_create_model_example_multiple_choice_branching_round_trips_into_solve() {
    let path = std::env::temp_dir().join(format!(
        "pred_test_model_example_mcb_{}.json",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let create = pred()
        .args([
            "create",
            "--example",
            "MultipleChoiceBranching/i32",
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
fn test_kth_largest_m_tuple_solve_uses_k_threshold() {
    let solve = |k: u64| {
        let create = pred()
            .args([
                "create",
                "KthLargestMTuple",
                "--subsets",
                "2,5,8;3,6;1,4,7",
                "--k",
                &k.to_string(),
                "--bound",
                "12",
            ])
            .output()
            .unwrap();
        assert!(
            create.status.success(),
            "stderr: {}",
            String::from_utf8_lossy(&create.stderr)
        );

        let path = std::env::temp_dir().join(format!(
            "pred_test_kth_largest_m_tuple_{}_{}.json",
            std::process::id(),
            k
        ));
        std::fs::write(&path, create.stdout).unwrap();

        let output = pred()
            .args(["solve", path.to_str().unwrap(), "--solver", "brute-force"])
            .output()
            .unwrap();
        std::fs::remove_file(path).unwrap();
        assert!(
            output.status.success(),
            "stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        serde_json::from_slice::<serde_json::Value>(&output.stdout).unwrap()
    };

    let at_threshold = solve(14);
    let above_threshold = solve(15);

    assert_eq!(at_threshold["status"], "optimal");
    assert_eq!(at_threshold["evaluation"], "Or(true)");
    assert_eq!(above_threshold["status"], "infeasible");
    assert!(above_threshold.get("evaluation").is_none());
    assert!(above_threshold.get("solution").is_none());
}

#[test]
fn test_create_acyclic_partition() {
    let output = pred()
        .args([
            "create",
            "AcyclicPartition/i32",
            "--arcs",
            "0>1,0>2,1>3,1>4,2>4,2>5,3>5,4>5",
            "--weights",
            "2,3,2,1,3,1",
            "--arc-costs",
            "1,1,1,1,1,1,1,1",
            "--weight-bound",
            "5",
            "--cost-bound",
            "5",
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
    assert_eq!(json["type"], "AcyclicPartition");
    assert_eq!(json["variant"]["weight"], "i32");
    assert_eq!(
        json["data"]["vertex_weights"],
        serde_json::json!([2, 3, 2, 1, 3, 1])
    );
    assert_eq!(
        json["data"]["arc_costs"],
        serde_json::json!([1, 1, 1, 1, 1, 1, 1, 1])
    );
    assert_eq!(json["data"]["weight_bound"], 5);
    assert_eq!(json["data"]["cost_bound"], 5);
}

#[test]
fn test_create_model_example_acyclic_partition() {
    let output = pred()
        .args(["create", "--example", "AcyclicPartition/i32"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["type"], "AcyclicPartition");
    assert_eq!(json["variant"]["weight"], "i32");
    assert_eq!(json["data"]["weight_bound"], 5);
    assert_eq!(json["data"]["cost_bound"], 5);
    assert_eq!(json["data"]["graph"]["num_vertices"], 6);
}

#[test]
fn test_create_model_example_acyclic_partition_round_trips_into_solve() {
    let path = std::env::temp_dir().join(format!(
        "pred_test_model_example_acyclic_partition_{}.json",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let create = pred()
        .args([
            "create",
            "--example",
            "AcyclicPartition/i32",
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
fn test_create_mixed_chinese_postman() {
    let output = pred()
        .args([
            "create",
            "MixedChinesePostman",
            "--graph",
            "0-2,1-3,0-4,4-2",
            "--arcs",
            "0>1,1>2,2>3,3>0",
            "--edge-weights",
            "2,3,1,2",
            "--arc-weights",
            "2,3,1,4",
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
    assert_eq!(json["type"], "MixedChinesePostman");
    assert_eq!(json["variant"]["weight"], "i32");
    assert_eq!(json["data"]["graph"]["num_vertices"], 5);
    assert_eq!(json["data"]["arc_weights"], serde_json::json!([2, 3, 1, 4]));
    assert_eq!(
        json["data"]["edge_weights"],
        serde_json::json!([2, 3, 1, 2])
    );
}

#[test]
fn test_create_model_example_mixed_chinese_postman() {
    let output = pred()
        .args(["create", "--example", "MixedChinesePostman/i32"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["type"], "MixedChinesePostman");
    assert_eq!(json["variant"]["weight"], "i32");
}

#[test]
fn test_create_mixed_chinese_postman_missing_arcs_shows_usage() {
    let output = pred()
        .args([
            "create",
            "MixedChinesePostman",
            "--graph",
            "0-2,1-3,0-4,4-2",
            "--edge-weights",
            "2,3,1,2",
            "--arc-weights",
            "2,3,1,4",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("missing required construction input(s): arcs"),
        "expected missing --arcs error, got: {stderr}"
    );
    assert!(
        stderr.contains("Usage: pred create MixedChinesePostman"),
        "expected recovery usage hint, got: {stderr}"
    );
}

#[test]
fn test_create_mixed_chinese_postman_rejects_edge_weight_length_mismatch() {
    let output = pred()
        .args([
            "create",
            "MixedChinesePostman",
            "--graph",
            "0-2,1-3,0-4,4-2",
            "--arcs",
            "0>1,1>2,2>3,3>0",
            "--edge-weights",
            "2,3",
            "--arc-weights",
            "2,3,1,4",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("edge_weights length must match num_edges"),
        "expected edge-weight mismatch diagnostic, got: {stderr}"
    );
}

#[test]
fn test_create_multiple_choice_branching_rejects_invalid_partition_without_panicking() {
    let output = pred()
        .args([
            "create",
            "MultipleChoiceBranching/i32",
            "--arcs",
            "0>1,0>2,1>3,2>3,1>4,3>5,4>5,2>4",
            "--weights",
            "3,2,4,1,2,3,1,3",
            "--partition",
            "0,1;2,3;4,7;5,7",
            "--threshold",
            "10",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        !stderr.contains("panicked at"),
        "invalid partition should return a user error, got panic output: {stderr}"
    );
    assert!(
        stderr.contains("partition"),
        "stderr should mention the invalid partition: {stderr}"
    );
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
    assert!(stdout.contains("\"kind\": \"brute-force\""));
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
    assert!(stdout.contains("\"kind\": \"ilp\""));
    assert!(stdout.contains("\"solution\""));
    assert!(
        stdout.contains("\"reduction_path\""),
        "MIS solved with ILP should report its registered pipeline: {stdout}"
    );

    std::fs::remove_file(&problem_file).ok();
}

#[test]
fn test_solve_ilp_default() {
    // MIS has no customized solver, so its registered ILP pipeline is the default.
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
        stdout.contains("\"kind\": \"ilp\"") && stdout.contains("\"reduction_path\""),
        "MIS with default solver should report its registered ILP pipeline: {stdout}"
    );

    std::fs::remove_file(&problem_file).ok();
}

#[test]
fn test_solve_ilp_reports_registered_pipeline() {
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
        stdout.contains("\"reduction_path\""),
        "Non-ILP problem solved with ILP should report its registered pipeline, got: {stdout}"
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
    assert_eq!(json["solver"]["kind"], "brute-force");

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

    let reduce_out = reduce_named_to_file(
        &problem_file,
        "MIS/SimpleGraph/One",
        "QUBO",
        &[
            "MaximumIndependentSet",
            "MaximumIndependentSet",
            "MaximumSetPacking",
            "MaximumSetPacking",
            "QUBO",
        ],
        &bundle_file,
    );
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

fn solve_sat_to_nae_bundle(case: &str, clauses: &str) -> serde_json::Value {
    let temp_dir = std::env::temp_dir();
    let process_id = std::process::id();
    let problem_file = temp_dir.join(format!("pred_test_{case}_{process_id}_sat.json"));
    let bundle_file = temp_dir.join(format!("pred_test_{case}_{process_id}_sat_nae_bundle.json"));

    let create = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "Satisfiability",
            "--num-vars",
            "1",
            "--clauses",
            clauses,
        ])
        .output()
        .unwrap();
    assert!(
        create.status.success(),
        "create stderr: {}",
        String::from_utf8_lossy(&create.stderr)
    );

    let reduce = reduce_named_to_file(
        &problem_file,
        "Satisfiability",
        "NAESatisfiability",
        &["Satisfiability", "NAESatisfiability"],
        &bundle_file,
    );
    assert!(
        reduce.status.success(),
        "reduce stderr: {}",
        String::from_utf8_lossy(&reduce.stderr)
    );

    let solve = pred()
        .args([
            "solve",
            bundle_file.to_str().unwrap(),
            "--solver",
            "brute-force",
            "--json",
        ])
        .output()
        .unwrap();
    assert!(
        solve.status.success(),
        "solve stderr: {}",
        String::from_utf8_lossy(&solve.stderr)
    );

    std::fs::remove_file(problem_file).unwrap();
    std::fs::remove_file(bundle_file).unwrap();
    serde_json::from_slice(&solve.stdout).unwrap()
}

#[test]
fn test_solve_bundle_distinguishes_infeasibility_from_missing_witness_capability() {
    let infeasible = solve_sat_to_nae_bundle("infeasible", "1;-1");
    assert_eq!(infeasible["status"], "infeasible");
    assert!(infeasible.get("evaluation").is_none());
    assert!(infeasible.get("solution").is_none());
    assert_eq!(infeasible["intermediate"]["status"], "infeasible");
    assert!(infeasible["intermediate"].get("evaluation").is_none());
    assert!(infeasible["intermediate"].get("solution").is_none());

    let feasible = solve_sat_to_nae_bundle("feasible", "1");
    assert_eq!(feasible["status"], "optimal");
    assert_eq!(feasible["evaluation"], "Or(true)");
    assert!(feasible["solution"].is_array());
    assert_eq!(feasible["intermediate"]["status"], "optimal");
    assert_eq!(feasible["intermediate"]["evaluation"], "Or(true)");
    assert!(feasible["intermediate"]["solution"].is_array());
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

    let reduce_out = reduce_named_to_file(
        &problem_file,
        "MIS/SimpleGraph/One",
        "MVC/SimpleGraph/i32",
        &[
            "MaximumIndependentSet",
            "MaximumIndependentSet",
            "MinimumVertexCover",
        ],
        &bundle_file,
    );
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
fn test_solve_direct_ilp_i32_problem() {
    let problem_file = std::env::temp_dir().join("pred_test_solve_ilp_i32_problem.json");

    let create_out = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "--example",
            "SequencingToMinimizeWeightedCompletionTime",
            "--to",
            "ILP/i32",
            "--example-side",
            "target",
        ])
        .output()
        .unwrap();
    assert!(
        create_out.status.success(),
        "create stderr: {}",
        String::from_utf8_lossy(&create_out.stderr)
    );

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
    assert!(stdout.contains("\"problem\": \"ILP\""), "{stdout}");
    assert!(stdout.contains("\"kind\": \"ilp\""), "{stdout}");

    std::fs::remove_file(&problem_file).ok();
}

#[test]
fn test_solve_partial_ilp_route_defaults_to_brute_force() {
    let problem_file = std::env::temp_dir()
        .join("pred_test_solve_sequencing_to_minimize_weighted_completion_time.json");

    let create_out = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "SequencingToMinimizeWeightedCompletionTime",
            "--lengths",
            "2,1,3,1,2",
            "--weights",
            "3,5,1,4,2",
            "--precedences",
            "0>2,1>4",
        ])
        .output()
        .unwrap();
    assert!(
        create_out.status.success(),
        "create stderr: {}",
        String::from_utf8_lossy(&create_out.stderr)
    );

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
        stdout.contains("\"problem\": \"SequencingToMinimizeWeightedCompletionTime\""),
        "{stdout}"
    );
    assert!(stdout.contains("\"kind\": \"brute-force\""), "{stdout}");
    assert!(stdout.contains("\"solution\": ["), "{stdout}");

    std::fs::remove_file(&problem_file).ok();
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

#[test]
fn test_solve_help_describes_deterministic_dispatch_and_overrides() {
    let output = pred().args(["solve", "--help"]).output().unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("deterministically selects"),
        "stdout: {stdout}"
    );
    assert!(stdout.contains("never searches"), "stdout: {stdout}");
    assert!(stdout.contains("--solver brute-force"), "stdout: {stdout}");
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
fn test_create_bounded_component_spanning_forest() {
    let output_file = std::env::temp_dir().join("pred_test_create_bcsf.json");
    let output = pred()
        .args([
            "-o",
            output_file.to_str().unwrap(),
            "create",
            "BoundedComponentSpanningForest",
            "--graph",
            "0-1,1-2,2-3,3-4,4-5,5-6,6-7,0-7,1-5,2-6",
            "--weights",
            "2,3,1,2,3,1,2,1",
            "--k",
            "3",
            "--max-weight",
            "6",
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
    assert_eq!(json["type"], "BoundedComponentSpanningForest");
    assert_eq!(json["data"]["max_components"], 3);
    assert_eq!(json["data"]["max_weight"], 6);
    std::fs::remove_file(&output_file).ok();
}

#[test]
fn test_create_bounded_component_spanning_forest_rejects_zero_k() {
    let output = pred()
        .args([
            "create",
            "BoundedComponentSpanningForest",
            "--graph",
            "0-1,1-2,2-3",
            "--weights",
            "1,1,1,1",
            "--k",
            "0",
            "--max-weight",
            "2",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("k must be at least 1"), "stderr: {stderr}");
}

#[test]
fn test_create_bounded_component_spanning_forest_accepts_k_larger_than_num_vertices() {
    let out = std::env::temp_dir().join("pred_test_bcsf_large_k.json");
    let output = pred()
        .args([
            "create",
            "BoundedComponentSpanningForest",
            "--graph",
            "0-1,1-2,2-3",
            "--weights",
            "1,1,1,1",
            "--k",
            "5",
            "--max-weight",
            "2",
            "-o",
        ])
        .arg(&out)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(out.exists());
    let _ = std::fs::remove_file(&out);
}

#[test]
fn test_create_bounded_component_spanning_forest_rejects_negative_weights() {
    let output = pred()
        .args([
            "create",
            "BoundedComponentSpanningForest",
            "--graph",
            "0-1,1-2,2-3",
            "--weights",
            "1,-1,1,1",
            "--k",
            "2",
            "--max-weight",
            "2",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("weights must be nonnegative"),
        "stderr: {stderr}"
    );
}

#[test]
fn test_create_bounded_component_spanning_forest_rejects_negative_bound() {
    let output = pred()
        .args([
            "create",
            "BoundedComponentSpanningForest",
            "--graph",
            "0-1,1-2,2-3",
            "--weights",
            "1,1,1,1",
            "--k",
            "2",
            "--max-weight",
            "-1",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("max_weight must be positive"),
        "stderr: {stderr}"
    );
}

#[test]
fn test_create_bounded_component_spanning_forest_no_flags_shows_actual_cli_flags() {
    let output = pred()
        .args(["create", "BoundedComponentSpanningForest"])
        .output()
        .unwrap();
    assert!(
        !output.status.success(),
        "should exit non-zero when showing help without data flags"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("--k"),
        "expected '--k' in help output, got: {stderr}"
    );
    assert!(
        stderr.contains("--max-weight"),
        "expected '--max-weight' in help output, got: {stderr}"
    );
    assert!(
        !stderr.contains("--max-components"),
        "help should not advertise nonexistent '--max-components' flag: {stderr}"
    );
}

#[test]
fn test_create_rooted_tree_arrangement() {
    let output_file = std::env::temp_dir().join("pred_test_create_rooted_tree_arrangement.json");
    let output = pred()
        .args([
            "-o",
            output_file.to_str().unwrap(),
            "create",
            "RootedTreeArrangement",
            "--graph",
            "0-1,0-2,1-2,2-3,3-4",
            "--bound",
            "7",
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
    assert_eq!(json["type"], "RootedTreeArrangement");
    assert_eq!(json["data"]["bound"], 7);
    std::fs::remove_file(&output_file).ok();
}

#[test]
fn test_create_string_to_string_correction() {
    let output_file =
        std::env::temp_dir().join("pred_test_create_string_to_string_correction.json");
    let output = pred()
        .args([
            "-o",
            output_file.to_str().unwrap(),
            "create",
            "StringToStringCorrection",
            "--source-string",
            "0,1,2,3,1,0",
            "--target-string",
            "0,1,3,2,1",
            "--bound",
            "2",
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
    assert_eq!(json["type"], "StringToStringCorrection");
    assert_eq!(
        json["data"]["source"],
        serde_json::json!([0, 1, 2, 3, 1, 0])
    );
    assert_eq!(json["data"]["target"], serde_json::json!([0, 1, 3, 2, 1]));
    assert_eq!(json["data"]["bound"], 2);
    std::fs::remove_file(&output_file).ok();
}

#[test]
fn test_create_model_example_string_to_string_correction() {
    let output = pred()
        .args(["create", "--example", "StringToStringCorrection"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["type"], "StringToStringCorrection");
    assert_eq!(
        json["data"]["source"],
        serde_json::json!([0, 1, 2, 3, 1, 0])
    );
    assert_eq!(json["data"]["target"], serde_json::json!([0, 1, 3, 2, 1]));
    assert_eq!(json["data"]["bound"], 2);
}

#[test]
fn test_create_string_to_string_correction_help_uses_cli_flags() {
    let output = pred()
        .args(["create", "StringToStringCorrection"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--source-string"), "stderr: {stderr}");
    assert!(stderr.contains("--target-string"), "stderr: {stderr}");
    assert!(stderr.contains("--bound"), "stderr: {stderr}");
}

#[test]
fn test_create_string_to_string_correction_rejects_negative_bound() {
    let output = pred()
        .args([
            "create",
            "StringToStringCorrection",
            "--source-string",
            "0,1,2,3,1,0",
            "--target-string",
            "0,1,3,2,1",
            "--bound",
            "-1",
        ])
        .output()
        .unwrap();
    assert!(
        !output.status.success(),
        "negative bound should be rejected"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("invalid value '-1'"), "stderr: {stderr}");
}

#[test]
fn test_create_grouping_by_swapping() {
    let output = pred()
        .args([
            "create",
            "GroupingBySwapping",
            "--string",
            "0,1,2,0,1,2",
            "--bound",
            "5",
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
    assert_eq!(json["type"], "GroupingBySwapping");
    assert_eq!(json["data"]["alphabet_size"], 3);
    assert_eq!(
        json["data"]["string"],
        serde_json::json!([0, 1, 2, 0, 1, 2])
    );
    assert_eq!(json["data"]["budget"], 5);
}

#[test]
fn test_create_model_example_grouping_by_swapping() {
    let output = pred()
        .args(["create", "--example", "GroupingBySwapping"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["type"], "GroupingBySwapping");
    assert_eq!(json["data"]["alphabet_size"], 3);
    assert_eq!(
        json["data"]["string"],
        serde_json::json!([0, 1, 2, 0, 1, 2])
    );
    assert_eq!(json["data"]["budget"], 5);
}

#[test]
fn test_create_grouping_by_swapping_help_uses_cli_flags() {
    let output = pred()
        .args(["create", "GroupingBySwapping"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--string"), "stderr: {stderr}");
    assert!(stderr.contains("--bound"), "stderr: {stderr}");
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
fn test_create_sequencing_to_minimize_weighted_completion_time() {
    let output_file = std::env::temp_dir()
        .join("pred_test_create_sequencing_to_minimize_weighted_completion_time.json");
    let output = pred()
        .args([
            "-o",
            output_file.to_str().unwrap(),
            "create",
            "SequencingToMinimizeWeightedCompletionTime",
            "--lengths",
            "2,1,3,1,2",
            "--weights",
            "3,5,1,4,2",
            "--precedences",
            "0>2,1>4",
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
    assert_eq!(json["type"], "SequencingToMinimizeWeightedCompletionTime");
    assert_eq!(json["data"]["lengths"], serde_json::json!([2, 1, 3, 1, 2]));
    assert_eq!(json["data"]["weights"], serde_json::json!([3, 5, 1, 4, 2]));
    assert_eq!(
        json["data"]["precedences"],
        serde_json::json!([[0, 2], [1, 4]])
    );
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
        .args([
            "create",
            "--example",
            "MaximumIndependentSet/KingsSubgraph/One",
        ])
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
}

#[test]
fn test_create_sequencing_to_minimize_weighted_tardiness_no_flags_shows_help() {
    let output = pred()
        .args(["create", "SequencingToMinimizeWeightedTardiness"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--lengths"));
    assert!(stderr.contains("--weights"));
    assert!(stderr.contains("--deadlines"));
    assert!(stderr.contains("--bound"));
    assert!(stderr.contains("pred create SequencingToMinimizeWeightedTardiness"));
}

#[test]
fn test_create_multiple_choice_branching_help_uses_threshold_flag() {
    let output = pred()
        .args(["create", "MultipleChoiceBranching/i32"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("--threshold"),
        "expected '--threshold' in help output, got: {stderr}"
    );
    assert!(
        !stderr.contains("--bound"),
        "help output should not advertise '--bound', got: {stderr}"
    );
}

#[test]
fn test_create_set_basis_no_flags_uses_actual_cli_flag_names() {
    let output = pred().args(["create", "SetBasis"]).output().unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("--universe-size"),
        "expected '--universe-size' in help output, got: {stderr}"
    );
    assert!(
        stderr.contains("--subsets"),
        "expected '--subsets' in help output, got: {stderr}"
    );
    assert!(
        stderr.contains("--k"),
        "expected '--k' in help output, got: {stderr}"
    );
    assert!(
        !stderr.contains("--collection"),
        "help should not advertise schema field names: {stderr}"
    );
}

#[test]
fn test_create_rectilinear_picture_compression_help_uses_bound_flag() {
    let output = pred()
        .args(["create", "RectilinearPictureCompression"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("--matrix"),
        "expected '--matrix' in help output, got: {stderr}"
    );
    assert!(
        stderr.contains("--bound"),
        "expected '--bound' in help output, got: {stderr}"
    );
}

#[test]
fn test_create_rectilinear_picture_compression_rejects_ragged_matrix() {
    let output = pred()
        .args([
            "create",
            "RectilinearPictureCompression",
            "--matrix",
            "1,0;1",
            "--bound",
            "1",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("All rows in --matrix must have the same length"),
        "expected rectangular-matrix validation error, got: {stderr}"
    );
    assert!(
        !stderr.contains("panicked at"),
        "ragged matrix should not crash the CLI, got: {stderr}"
    );
}

#[test]
fn test_create_register_sufficiency() {
    let output_file = std::env::temp_dir().join("pred_test_create_register_sufficiency.json");
    let output = pred()
        .args([
            "-o",
            output_file.to_str().unwrap(),
            "create",
            "RegisterSufficiency",
            "--arcs",
            "2>0,2>1,3>1,4>2,4>3,5>0,6>4,6>5",
            "--bound",
            "3",
            "--num-vertices",
            "7",
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
    assert_eq!(json["type"], "RegisterSufficiency");
    assert_eq!(json["data"]["num_vertices"], 7);
    assert_eq!(json["data"]["bound"], 3);
    assert_eq!(json["data"]["arcs"].as_array().unwrap().len(), 8);
    std::fs::remove_file(&output_file).ok();
}

#[test]
fn test_create_length_bounded_disjoint_paths_help_uses_max_length_flag() {
    let output = pred()
        .args(["create", "LengthBoundedDisjointPaths"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("--max-length"),
        "expected '--max-length' in help output, got: {stderr}"
    );
    assert!(
        !stderr.contains("--bound"),
        "help should advertise the canonical CLI flag name, got: {stderr}"
    );
}

#[test]
fn test_create_consecutive_ones_submatrix_no_flags_uses_actual_cli_help() {
    let output = pred()
        .args(["create", "ConsecutiveOnesSubmatrix"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("--matrix"),
        "expected '--matrix' in help output, got: {stderr}"
    );
    assert!(
        stderr.contains("--bound"),
        "expected '--bound' in help output, got: {stderr}"
    );
}

#[test]
fn test_create_prime_attribute_name_no_flags_uses_actual_cli_flag_names() {
    let output = pred()
        .args(["create", "PrimeAttributeName"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("--universe-size"),
        "expected '--universe-size' in help output, got: {stderr}"
    );
    assert!(
        stderr.contains("--dependencies"),
        "expected '--dependencies' in help output, got: {stderr}"
    );
    assert!(
        stderr.contains("--query-attribute"),
        "expected '--query-attribute' in help output, got: {stderr}"
    );
    assert!(
        !stderr.contains("--num-attributes"),
        "help should not advertise schema field names: {stderr}"
    );
    assert!(
        !stderr.contains("--deps"),
        "help should not advertise the legacy flag name: {stderr}"
    );
    assert!(
        !stderr.contains("--query\n"),
        "help should not advertise the legacy flag name: {stderr}"
    );
}

#[test]
fn test_create_lcs_with_raw_strings_infers_alphabet() {
    let output = pred()
        .args(["create", "LCS", "--strings", "ABAC;BACA"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["type"], "LongestCommonSubsequence");
    assert_eq!(json["data"]["alphabet_size"], 3);
    assert_eq!(
        json["data"]["strings"],
        serde_json::json!([[0, 1, 0, 2], [1, 0, 2, 0]])
    );
}

#[test]
fn test_create_shortest_common_supersequence_derives_internal_fields() {
    let output = pred()
        .args([
            "create",
            "ShortestCommonSupersequence",
            "--strings",
            "0,1,2;1,2,0",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["data"]["alphabet_size"], 3);
    assert_eq!(json["data"]["max_length"], 6);
}

#[test]
fn test_create_lcs_rejects_empty_strings_without_panicking() {
    let output = pred()
        .args(["create", "LCS", "--strings", ""])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("at least one input string must be non-empty"),
        "expected user-facing validation error, got: {stderr}"
    );
    assert!(
        !stderr.contains("panicked at"),
        "create command should reject invalid LCS input without panicking: {stderr}"
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
fn test_create_minmaxmulticenter_success() {
    let output = pred()
        .args([
            "create",
            "MinMaxMulticenter",
            "--graph",
            "0-1,1-2,2-3",
            "--weights",
            "1,2,3,4",
            "--edge-weights",
            "5,6,7",
            "--k",
            "2",
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
    assert_eq!(json["type"], "MinMaxMulticenter");
    assert_eq!(json["variant"]["graph"], "SimpleGraph");
    assert_eq!(json["variant"]["weight"], "i32");
    assert_eq!(json["data"]["k"], 2);
    assert_eq!(
        json["data"]["vertex_weights"],
        serde_json::json!([1, 2, 3, 4])
    );
    assert_eq!(json["data"]["edge_lengths"], serde_json::json!([5, 6, 7]));
}

#[test]
fn test_create_minmaxmulticenter_help_uses_cli_flag_names() {
    let output = pred()
        .args(["create", "MinMaxMulticenter"])
        .output()
        .unwrap();
    assert!(
        !output.status.success(),
        "should exit non-zero when showing help without data flags"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--weights"), "stderr: {stderr}");
    assert!(stderr.contains("--edge-weights"), "stderr: {stderr}");
    assert!(!stderr.contains("--vertex-weights"), "stderr: {stderr}");
    assert!(!stderr.contains("--edge-lengths"), "stderr: {stderr}");
}

#[test]
fn test_create_minmaxmulticenter_negative_inputs_rejected() {
    let vertex_weights = pred()
        .args([
            "create",
            "MinMaxMulticenter",
            "--graph",
            "0-1",
            "--weights",
            "1,-1",
            "--edge-weights",
            "1",
            "--k",
            "1",
        ])
        .output()
        .unwrap();
    assert!(!vertex_weights.status.success());
    assert!(String::from_utf8_lossy(&vertex_weights.stderr).contains("must be non-negative"));

    let edge_weights = pred()
        .args([
            "create",
            "MinMaxMulticenter",
            "--graph",
            "0-1",
            "--weights",
            "1,1",
            "--edge-weights=-1",
            "--k",
            "1",
        ])
        .output()
        .unwrap();
    assert!(!edge_weights.status.success());
    assert!(String::from_utf8_lossy(&edge_weights.stderr).contains("must be non-negative"));
}

#[test]
fn test_solve_minmaxmulticenter_default_solver_uses_ilp() {
    let problem_file = std::env::temp_dir().join("pred_test_minmaxmulticenter_solve.json");
    let create_out = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "MinMaxMulticenter",
            "--graph",
            "0-1,1-2,2-3",
            "--weights",
            "1,1,1,1",
            "--edge-weights",
            "1,1,1",
            "--k",
            "2",
        ])
        .output()
        .unwrap();
    assert!(create_out.status.success());

    let solve_out = pred()
        .args(["solve", problem_file.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(
        solve_out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&solve_out.stderr)
    );
    let stdout = String::from_utf8(solve_out.stdout).unwrap();
    assert!(stdout.contains("\"kind\": \"ilp\""), "stdout: {stdout}");
    assert!(stdout.contains("\"reduction_path\""), "stdout: {stdout}");

    std::fs::remove_file(&problem_file).ok();
}

#[test]
fn test_create_consecutive_ones_submatrix_succeeds() {
    let output = pred()
        .args([
            "create",
            "ConsecutiveOnesSubmatrix",
            "--matrix",
            "1,1,0,1;1,0,1,1;0,1,1,0",
            "--bound",
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
    assert_eq!(json["type"], "ConsecutiveOnesSubmatrix");
    assert_eq!(json["data"]["bound"], 3);
    assert_eq!(
        json["data"]["matrix"][0],
        serde_json::json!([true, true, false, true])
    );
}

#[test]
fn test_create_kth_best_spanning_tree_rejects_zero_k() {
    let output = pred()
        .args([
            "create",
            "KthBestSpanningTree",
            "--graph",
            "0-1,1-2,0-2",
            "--edge-weights",
            "2,3,1",
            "--k",
            "0",
            "--bound",
            "3",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("must be positive"),
        "expected positive-k validation error, got: {stderr}"
    );
}

#[test]
fn test_create_kth_best_spanning_tree_help_uses_edge_weights() {
    let output = pred()
        .args(["create", "KthBestSpanningTree"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("--edge-weights"),
        "expected edge-weight help, got: {stderr}"
    );
    assert!(
        !stderr.contains("\n  --weights"),
        "vertex-weight flag should not be suggested, got: {stderr}"
    );
}

#[test]
fn test_create_kth_best_spanning_tree_rejects_vertex_weights_flag() {
    let output = pred()
        .args([
            "create",
            "KthBestSpanningTree",
            "--graph",
            "0-1,0-2,1-2",
            "--weights",
            "9,9,9",
            "--k",
            "1",
            "--bound",
            "3",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("--edge-weights"),
        "expected guidance toward edge weights, got: {stderr}"
    );
}

#[test]
fn test_create_length_bounded_disjoint_paths_rejects_equal_terminals() {
    let output = pred()
        .args([
            "create",
            "LengthBoundedDisjointPaths",
            "--graph",
            "0-1,1-2",
            "--source",
            "0",
            "--sink",
            "0",
            "--max-length",
            "1",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("source and sink must be distinct"),
        "expected user-facing validation error, got: {stderr}"
    );
    assert!(
        !stderr.contains("panicked at"),
        "create command should reject equal terminals without panicking: {stderr}"
    );
}

#[test]
fn test_create_length_bounded_disjoint_paths_succeeds() {
    let output = pred()
        .args([
            "create",
            "LengthBoundedDisjointPaths",
            "--graph",
            "0-1,1-3,0-2,2-3",
            "--source",
            "0",
            "--sink",
            "3",
            "--max-length",
            "2",
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
    assert_eq!(json["type"], "LengthBoundedDisjointPaths");
    assert_eq!(json["data"]["source"], 0);
    assert_eq!(json["data"]["sink"], 3);
    // max_paths is auto-computed: min(deg(0), deg(3)) = min(2, 2) = 2
    assert_eq!(json["data"]["max_paths"], 2);
    assert_eq!(json["data"]["max_length"], 2);
}

#[test]
fn test_create_length_bounded_disjoint_paths_rejects_negative_bound_value() {
    let output = pred()
        .args([
            "create",
            "LengthBoundedDisjointPaths",
            "--graph",
            "0-1,1-2",
            "--source",
            "0",
            "--sink",
            "1",
            "--max-length",
            "-1",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("invalid value '-1'"), "stderr: {stderr}");
}

#[test]
fn test_create_random_length_bounded_disjoint_paths_rejects_negative_bound_value() {
    let output = pred()
        .args([
            "create",
            "LengthBoundedDisjointPaths",
            "--random",
            "--num-vertices",
            "3",
            "--seed",
            "7",
            "--max-length=-1",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("invalid value '-1'"), "stderr: {stderr}");
}

#[test]
fn test_create_longest_circuit_succeeds() {
    let output = pred()
        .args([
            "create",
            "LongestCircuit",
            "--graph",
            "0-1,1-2,2-3,3-0",
            "--edge-weights",
            "2,2,2,2",
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
    assert_eq!(json["type"], "LongestCircuit");
    assert_eq!(
        json["data"]["edge_lengths"],
        serde_json::json!([2, 2, 2, 2])
    );
}

#[test]
fn test_create_longest_circuit_defaults_unit_edge_weights() {
    let output = pred()
        .args(["create", "LongestCircuit", "--graph", "0-1,1-2,2-3,3-0"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["type"], "LongestCircuit");
    assert_eq!(
        json["data"]["edge_lengths"],
        serde_json::json!([1, 1, 1, 1])
    );
}

#[test]
fn test_create_longest_circuit_no_flags_shows_help() {
    let output = pred().args(["create", "LongestCircuit"]).output().unwrap();
    assert!(
        !output.status.success(),
        "should exit non-zero when showing help without data flags"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("--edge-weights"),
        "expected '--edge-weights' in help output, got: {stderr}"
    );
    assert!(
        !stderr.contains("--edge-lengths"),
        "help should advertise the actual CLI flag name, got: {stderr}"
    );
}

#[test]
fn test_create_random_longest_circuit_succeeds() {
    let output = pred()
        .args([
            "create",
            "LongestCircuit",
            "--random",
            "--num-vertices",
            "6",
            "--seed",
            "7",
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
    assert_eq!(json["type"], "LongestCircuit");
    assert_eq!(json["data"]["graph"]["num_vertices"], 6);
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
fn test_path_rejects_removed_cost_selection() {
    let output = pred()
        .args(["path", "MIS", "QUBO", "--cost", "minimize:num_variables"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unexpected argument '--cost'"));
}

#[test]
fn test_path_overall_exact_map_text() {
    let output = pred().args(["path", "KSAT/K3", "MIS"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("Overall"),
        "multi-step path should show Overall exact-map accounting"
    );
}

#[test]
fn test_path_overall_exact_map_json() {
    let output = pred()
        .args([
            "path",
            "MIS/SimpleGraph/i32",
            "MaximumClique/SimpleGraph/i32",
            "--json",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let envelope: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let json = &envelope["paths"][0];
    assert!(
        json["overall_size"]["fields"].is_array(),
        "JSON should contain an overall exact size relation"
    );
    let items = json["overall_size"]["fields"].as_array().unwrap();
    assert!(!items.is_empty(), "overall exact map should have entries");
    assert!(items[0]["field"].is_string());
    assert!(items[0]["formula"].is_string());
}

#[test]
fn test_path_overall_exact_map_composition() {
    // The One → i32 cast and graph complement are both exact. Their composition
    // must remain in source fields rather than consulting a bound or Growth.
    let output = pred()
        .args([
            "path",
            "MIS/SimpleGraph/One",
            "MaximumClique/SimpleGraph/i32",
            "--json",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let envelope: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let json = envelope["paths"]
        .as_array()
        .unwrap()
        .iter()
        .find(|path| path["steps"].as_u64().is_some_and(|steps| steps >= 2))
        .expect("multi-step route");

    assert!(json["steps"].as_u64().unwrap() >= 2);

    let overall: std::collections::HashMap<String, String> = json["overall_size"]["fields"]
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

    assert!(
        overall.contains_key("num_vertices"),
        "overall should have num_vertices"
    );
    assert!(
        overall.contains_key("num_edges"),
        "overall should have num_edges"
    );
    assert!(
        overall["num_vertices"] == "num_vertices",
        "num_vertices should be in terms of source vars, got: {}",
        overall["num_vertices"]
    );
    assert!(
        overall["num_edges"].contains("num_vertices") && overall["num_edges"].contains("num_edges"),
        "complement edges should be in terms of source vars, got: {}",
        overall["num_edges"]
    );
}

#[test]
fn test_path_set_has_explicit_strongest_size_information() {
    let output = pred()
        .args(["path", "KSAT/K3", "MIS", "--json"])
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
            p["overall_size"]["fields"].is_array(),
            "path {} has no explicit size result",
            i + 1
        );
    }
    // Verify envelope metadata
    assert!(envelope.get("returned").is_none());
    assert!(envelope.get("max_paths").is_none());
    assert!(envelope.get("analysis").is_none());
    assert!(envelope["truncated"].is_boolean());
}

#[test]
fn test_path_overall_unavailable_is_reported_per_field_without_internal_modes() {
    let output = pred()
        .args(["path", "Factoring", "SpinGlass", "--json"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let envelope: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let overall = &envelope["paths"][0]["overall_size"];
    let fields = overall["fields"].as_array().unwrap();
    assert!(!fields.is_empty());
    assert!(fields.iter().all(|field| {
        field["relation"] == "unavailable"
            && field["field"].is_string()
            && field["reason"].is_string()
    }));
    assert!(overall.get("exact_composition_error").is_none());
    assert!(overall.get("bound_composition_error").is_none());
}

#[test]
fn test_path_overall_preserves_unavailable_fields_alongside_exact_fields() {
    let output = pred()
        .args([
            "path",
            "MaximumClique/SimpleGraph/i32",
            "ILP/bool",
            "--max-paths",
            "1",
            "--json",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let envelope: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let fields = envelope["paths"][0]["overall_size"]["fields"]
        .as_array()
        .unwrap();
    let relations = fields
        .iter()
        .map(|field| {
            (
                field["field"].as_str().unwrap(),
                field["relation"].as_str().unwrap(),
            )
        })
        .collect::<std::collections::BTreeMap<_, _>>();
    assert_eq!(relations["num_vars"], "exact");
    assert_eq!(relations["num_constraints"], "unavailable");
}

#[test]
fn test_path_overall_unavailable_reason_matches_each_target_field() {
    let output = pred()
        .args([
            "path",
            "Factoring",
            "ILP/bool",
            "--max-paths",
            "7",
            "--json",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let envelope: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let path = envelope["paths"]
        .as_array()
        .unwrap()
        .iter()
        .find(|path| {
            path["path"].as_array().is_some_and(|steps| {
                steps
                    .iter()
                    .any(|step| step["from"]["name"] == "Clustering")
            })
        })
        .expect("Factoring -> ... -> Clustering -> ILP path");
    let fields = path["overall_size"]["fields"]
        .as_array()
        .unwrap()
        .iter()
        .map(|field| (field["field"].as_str().unwrap(), field))
        .collect::<std::collections::BTreeMap<_, _>>();

    assert!(fields["num_constraints"]["reason"]
        .as_str()
        .unwrap()
        .contains("constraint count depends"));
    assert!(fields["num_vars"]["reason"]
        .as_str()
        .unwrap()
        .contains("has no symbolic size transform"));
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
    let route_file = std::env::temp_dir().join("pred_test_reduce_stdout_route.json");
    write_named_route(
        "MIS/SimpleGraph/One",
        "QUBO",
        &[
            "MaximumIndependentSet",
            "MaximumIndependentSet",
            "MaximumSetPacking",
            "MaximumSetPacking",
            "QUBO",
        ],
        &route_file,
    );

    let output = pred()
        .args([
            "reduce",
            problem_file.to_str().unwrap(),
            "--via",
            route_file.to_str().unwrap(),
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
    std::fs::remove_file(&route_file).ok();
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
    let route_file = std::env::temp_dir().join("pred_test_reduce_human_route.json");
    write_named_route(
        "MIS/SimpleGraph/One",
        "QUBO",
        &[
            "MaximumIndependentSet",
            "MaximumIndependentSet",
            "MaximumSetPacking",
            "MaximumSetPacking",
            "QUBO",
        ],
        &route_file,
    );

    let output = pred()
        .args([
            "reduce",
            problem_file.to_str().unwrap(),
            "--via",
            route_file.to_str().unwrap(),
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
    std::fs::remove_file(&route_file).ok();
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

    let reduce_out = reduce_named_to_file(
        &problem_file,
        "MIS/SimpleGraph/One",
        "QUBO",
        &[
            "MaximumIndependentSet",
            "MaximumIndependentSet",
            "MaximumSetPacking",
            "MaximumSetPacking",
            "QUBO",
        ],
        &bundle_file,
    );
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
fn test_solve_sum_of_squares_partition_default_solver_uses_ilp() {
    let problem_json = r#"{
        "type": "SumOfSquaresPartition",
        "data": {
            "sizes": [5, 3, 8, 2, 7, 1],
            "num_groups": 3,
            "bound": 240
        }
    }"#;
    let tmp = std::env::temp_dir().join("pred_test_sum_of_squares_partition.json");
    std::fs::write(&tmp, problem_json).unwrap();

    let output = pred()
        .args(["solve", tmp.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("\"kind\": \"ilp\""),
        "stdout should report the ILP solver, got: {stdout}"
    );
    assert!(
        stdout.contains("\"reduction_path\""),
        "stdout should report the ILP reduction target, got: {stdout}"
    );

    std::fs::remove_file(&tmp).ok();
}

#[test]
fn test_create_multiple_choice_branching_pipe_to_solve() {
    let create_out = pred()
        .args([
            "create",
            "MultipleChoiceBranching/i32",
            "--arcs",
            "0>1,0>2,1>3,2>3,1>4,3>5,4>5,2>4",
            "--weights",
            "3,2,4,1,2,3,1,3",
            "--partition",
            "0,1;2,3;4,7;5,6",
            "--threshold",
            "10",
        ])
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
        stdout.contains("Max("),
        "stdout should contain Max(...), got: {stdout}"
    );
}

#[test]
fn test_create_pipe_to_reduce() {
    // pred create MIS --graph 0-1,1-2 | pred reduce - --via route.json
    let create_out = pred()
        .args(["create", "MIS", "--graph", "0-1,1-2"])
        .output()
        .unwrap();
    assert!(
        create_out.status.success(),
        "create stderr: {}",
        String::from_utf8_lossy(&create_out.stderr)
    );
    let route_file = std::env::temp_dir().join("pred_test_pipe_reduce_route.json");
    write_named_route(
        "MIS/SimpleGraph/One",
        "QUBO",
        &[
            "MaximumIndependentSet",
            "MaximumIndependentSet",
            "MaximumSetPacking",
            "MaximumSetPacking",
            "QUBO",
        ],
        &route_file,
    );

    use std::io::Write;
    let mut child = pred()
        .args([
            "reduce",
            "-",
            "--via",
            route_file.to_str().unwrap(),
            "--json",
        ])
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
    std::fs::remove_file(route_file).ok();
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
fn test_inspect_reports_only_executable_reductions_for_exact_variant() {
    let unit_file = std::env::temp_dir().join("pred_test_inspect_exact_variant_unit.json");
    let weighted_file = std::env::temp_dir().join("pred_test_inspect_exact_variant_weighted.json");

    let unit_create = pred()
        .args([
            "create",
            "MIS",
            "--graph",
            "0-1,1-2,2-3",
            "-o",
            unit_file.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(
        unit_create.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&unit_create.stderr)
    );

    let weighted_create = pred()
        .args([
            "create",
            "MIS/SimpleGraph/i32",
            "--graph",
            "0-1,1-2,2-3",
            "--weights",
            "3,1,2,1",
            "-o",
            weighted_file.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(
        weighted_create.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&weighted_create.stderr)
    );

    for (source, source_ref, expected, excluded) in [
        (
            &unit_file,
            "MIS/SimpleGraph/One",
            "MaximumSetPacking",
            "IntegralFlowBundles",
        ),
        (
            &weighted_file,
            "MIS/SimpleGraph/i32",
            "IntegralFlowBundles",
            "MaximumIndependentSet/KingsSubgraph/One",
        ),
    ] {
        let inspect = pred()
            .args(["inspect", source.to_str().unwrap(), "--json"])
            .output()
            .unwrap();
        assert!(
            inspect.status.success(),
            "stderr: {}",
            String::from_utf8_lossy(&inspect.stderr)
        );
        let json: serde_json::Value = serde_json::from_slice(&inspect.stdout).unwrap();
        let targets = json["reduces_to"].as_array().unwrap();
        assert!(targets.iter().any(|target| target == expected));
        assert!(!targets.iter().any(|target| target == excluded));

        for (index, target) in targets.iter().enumerate() {
            let target = target.as_str().unwrap();
            let bundle = std::env::temp_dir().join(format!(
                "pred_test_inspect_exact_variant_bundle_{index}.json"
            ));
            let route = bundle.with_extension("route.json");
            write_direct_route(source_ref, target, &route);
            let reduce = pred()
                .args([
                    "reduce",
                    source.to_str().unwrap(),
                    "--via",
                    route.to_str().unwrap(),
                    "-o",
                    bundle.to_str().unwrap(),
                ])
                .output()
                .unwrap();
            assert!(
                reduce.status.success(),
                "inspect advertised non-executable target {target}: {}",
                String::from_utf8_lossy(&reduce.stderr)
            );
            std::fs::remove_file(route).unwrap();
            std::fs::remove_file(bundle).unwrap();
        }
    }

    std::fs::remove_file(unit_file).unwrap();
    std::fs::remove_file(weighted_file).unwrap();
}

#[test]
fn test_inspect_excludes_non_witness_reductions() {
    let problem_file = std::env::temp_dir().join("pred_test_inspect_witness_reductions_only.json");
    let create = pred()
        .args([
            "create",
            "--example",
            "MinimumDominatingSet",
            "-o",
            problem_file.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(
        create.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&create.stderr)
    );

    let inspect = pred()
        .args(["inspect", problem_file.to_str().unwrap(), "--json"])
        .output()
        .unwrap();
    assert!(
        inspect.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&inspect.stderr)
    );
    let json: serde_json::Value = serde_json::from_slice(&inspect.stdout).unwrap();
    assert_eq!(json["reduces_to"], serde_json::json!(["ILP"]));

    std::fs::remove_file(problem_file).unwrap();
}

#[test]
fn test_inspect_minmaxmulticenter_lists_ilp_and_bruteforce() {
    let problem_file = std::env::temp_dir().join("pred_test_inspect_minmaxmulticenter.json");
    let create_out = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "MinMaxMulticenter",
            "--graph",
            "0-1,1-2,2-3",
            "--weights",
            "1,1,1,1",
            "--edge-weights",
            "1,1,1",
            "--k",
            "2",
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
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let solvers: Vec<&str> = json["solvers"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();
    assert_eq!(solvers, vec!["ilp", "brute-force"]);

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

    let reduce_out = reduce_named_to_file(
        &problem_file,
        "MIS/SimpleGraph/One",
        "QUBO",
        &[
            "MaximumIndependentSet",
            "MaximumIndependentSet",
            "MaximumSetPacking",
            "MaximumSetPacking",
            "QUBO",
        ],
        &bundle_file,
    );
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
fn test_inspect_multiprocessor_scheduling_reports_ilp_and_brute_force() {
    let problem_file = std::env::temp_dir().join("pred_test_inspect_mps_in.json");
    let result_file = std::env::temp_dir().join("pred_test_inspect_mps_out.json");
    let create_out = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "MultiprocessorScheduling",
            "--lengths",
            "4,5,3,2,6",
            "--num-processors",
            "2",
            "--deadline",
            "10",
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

    let content = std::fs::read_to_string(&result_file).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    let solvers: Vec<&str> = json["solvers"]
        .as_array()
        .expect("solvers should be an array")
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();
    assert_eq!(
        solvers,
        vec!["ilp", "brute-force"],
        "unexpected solvers: {solvers:?}"
    );

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

#[test]
fn test_inspect_integral_flow_with_multipliers_reports_size_fields() {
    let problem_file = std::env::temp_dir().join("pred_test_ifwm_inspect_in.json");
    let result_file = std::env::temp_dir().join("pred_test_ifwm_inspect_out.json");
    let create_out = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "--example",
            "IntegralFlowWithMultipliers",
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

    let content = std::fs::read_to_string(&result_file).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    let size_fields: Vec<&str> = json["size_fields"]
        .as_array()
        .expect("size_fields should be an array")
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();
    assert!(size_fields.contains(&"num_vertices"));
    assert!(size_fields.contains(&"num_arcs"));
    assert!(size_fields.contains(&"max_capacity"));
    assert!(size_fields.contains(&"requirement"));

    std::fs::remove_file(&problem_file).ok();
    std::fs::remove_file(&result_file).ok();
}

#[test]
fn test_inspect_acyclic_partition_reports_size_fields() {
    let problem_file = std::env::temp_dir().join("pred_test_acyclic_partition_inspect_in.json");
    let result_file = std::env::temp_dir().join("pred_test_acyclic_partition_inspect_out.json");
    let create_out = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "--example",
            "AcyclicPartition/i32",
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
        "AcyclicPartition size_fields should contain num_vertices, got: {:?}",
        size_fields
    );
    assert!(
        size_fields.contains(&"num_arcs"),
        "AcyclicPartition size_fields should contain num_arcs, got: {:?}",
        size_fields
    );

    std::fs::remove_file(&problem_file).ok();
    std::fs::remove_file(&result_file).ok();
}

#[test]
fn test_inspect_multiple_copy_file_allocation_reports_size_fields() {
    let problem_file = std::env::temp_dir().join("pred_test_mcfa_inspect_in.json");
    let result_file = std::env::temp_dir().join("pred_test_mcfa_inspect_out.json");
    let create_out = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "--example",
            "MultipleCopyFileAllocation",
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
        "MultipleCopyFileAllocation size_fields should contain num_vertices, got: {:?}",
        size_fields
    );
    assert!(
        size_fields.contains(&"num_edges"),
        "MultipleCopyFileAllocation size_fields should contain num_edges, got: {:?}",
        size_fields
    );
    let solvers: Vec<&str> = json["solvers"]
        .as_array()
        .expect("solvers should be an array")
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();
    assert_eq!(solvers, vec!["ilp", "brute-force"]);

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
        stderr.contains("unexpected argument '--random'"),
        "expected Clap to reject unsupported random generation, got: {stderr}"
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
        stderr.contains("num_vertices must be at least 2"),
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
        stderr.contains("edge_prob must be between"),
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

#[test]
fn test_create_bcnf_rejects_out_of_range_attribute_indices() {
    let output = pred()
        .args([
            "create",
            "BoyceCoddNormalFormViolation",
            "--n",
            "3",
            "--subsets",
            "0:4",
            "--target",
            "0,1,2",
        ])
        .output()
        .unwrap();
    assert!(
        !output.status.success(),
        "expected invalid indices to be rejected"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains("panicked at"),
        "CLI should return a user-facing error, got: {stderr}"
    );
    assert!(
        stderr.contains("outside universe of size 3"),
        "expected out-of-range error, got: {stderr}"
    );
}

#[test]
fn test_create_bcnf_rejects_out_of_range_lhs_attribute_indices() {
    let output = pred()
        .args([
            "create",
            "BoyceCoddNormalFormViolation",
            "--n",
            "3",
            "--subsets",
            "4:0",
            "--target",
            "0,1,2",
        ])
        .output()
        .unwrap();
    assert!(
        !output.status.success(),
        "expected invalid lhs indices to be rejected"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("subsets[0] contains attribute 4 outside universe of size 3"),
        "expected lhs-specific out-of-range error, got: {stderr}"
    );
}

#[test]
fn test_create_bcnf_rejects_out_of_range_target_attribute_indices() {
    let output = pred()
        .args([
            "create",
            "BoyceCoddNormalFormViolation",
            "--n",
            "3",
            "--subsets",
            "0:1",
            "--target",
            "0,1,4",
        ])
        .output()
        .unwrap();
    assert!(
        !output.status.success(),
        "expected invalid target indices to be rejected"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("target contains attribute 4 outside universe of size 3"),
        "expected target-specific out-of-range error, got: {stderr}"
    );
}

#[test]
fn test_create_consistency_of_database_frequency_tables() {
    let output = pred()
        .args([
            "create",
            "ConsistencyOfDatabaseFrequencyTables",
            "--num-objects",
            "6",
            "--attribute-domains",
            "2,3,2",
            "--frequency-tables",
            r#"[{"attribute_a":0,"attribute_b":1,"counts":[[1,1,1],[1,1,1]]},{"attribute_a":1,"attribute_b":2,"counts":[[1,1],[0,2],[1,1]]}]"#,
            "--known-values",
            r#"[{"object":0,"attribute":0,"value":0},{"object":3,"attribute":0,"value":1},{"object":1,"attribute":2,"value":1}]"#,
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
    assert_eq!(json["type"], "ConsistencyOfDatabaseFrequencyTables");
    assert_eq!(json["data"]["num_objects"], 6);
    assert_eq!(
        json["data"]["attribute_domains"],
        serde_json::json!([2, 3, 2])
    );
    assert_eq!(json["data"]["frequency_tables"][0]["attribute_a"], 0);
    assert_eq!(json["data"]["frequency_tables"][0]["attribute_b"], 1);
    assert_eq!(
        json["data"]["frequency_tables"][0]["counts"],
        serde_json::json!([[1, 1, 1], [1, 1, 1]])
    );
    assert_eq!(
        json["data"]["known_values"],
        serde_json::json!([
            {"object": 0, "attribute": 0, "value": 0},
            {"object": 3, "attribute": 0, "value": 1},
            {"object": 1, "attribute": 2, "value": 1}
        ])
    );
}

#[test]
fn test_create_consistency_of_database_frequency_tables_problem_help_uses_supported_flags() {
    let output = pred()
        .args(["create", "ConsistencyOfDatabaseFrequencyTables"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--num-objects"), "stderr: {stderr}");
    assert!(stderr.contains("--attribute-domains"), "stderr: {stderr}");
    assert!(stderr.contains("--frequency-tables"), "stderr: {stderr}");
    assert!(stderr.contains("--known-values"), "stderr: {stderr}");
}

#[test]
fn test_create_multiple_copy_file_allocation() {
    let output = pred()
        .args([
            "create",
            "MultipleCopyFileAllocation",
            "--graph",
            "0-1,1-2,2-3",
            "--usage",
            "5,4,3,2",
            "--storage",
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
    assert_eq!(json["type"], "MultipleCopyFileAllocation");
    assert_eq!(json["data"]["usage"], serde_json::json!([5, 4, 3, 2]));
    assert_eq!(json["data"]["storage"], serde_json::json!([1, 1, 1, 1]));
    assert_eq!(json["data"]["graph"]["num_vertices"], 4);
    assert_eq!(json["data"]["graph"]["edges"].as_array().unwrap().len(), 3);
}

#[test]
fn test_create_sequencing_to_minimize_maximum_cumulative_cost() {
    let output = pred()
        .args([
            "create",
            "SequencingToMinimizeMaximumCumulativeCost",
            "--costs",
            "2,-1,3,-2,1,-3",
            "--precedences",
            "0>2,1>2,1>3,2>4,3>5,4>5",
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
    assert_eq!(json["type"], "SequencingToMinimizeMaximumCumulativeCost");
    assert_eq!(
        json["data"]["costs"],
        serde_json::json!([2, -1, 3, -2, 1, -3])
    );
    assert_eq!(
        json["data"]["precedences"],
        serde_json::json!([[0, 2], [1, 2], [1, 3], [2, 4], [3, 5], [4, 5]])
    );
}

#[test]
fn test_create_multiple_copy_file_allocation_no_flags_shows_help() {
    let output = pred()
        .args(["create", "MultipleCopyFileAllocation"])
        .output()
        .unwrap();
    assert!(
        !output.status.success(),
        "should exit non-zero when showing help without data flags"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("--usage"),
        "expected '--usage' in help output, got: {stderr}"
    );
    assert!(
        stderr.contains("--storage"),
        "expected '--storage' in help output, got: {stderr}"
    );
}

#[test]
fn test_create_sequencing_to_minimize_maximum_cumulative_cost_no_flags_shows_help() {
    let output = pred()
        .args(["create", "SequencingToMinimizeMaximumCumulativeCost"])
        .output()
        .unwrap();
    assert!(
        !output.status.success(),
        "should exit non-zero when showing help without data flags"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("--costs"),
        "expected '--costs' in help output, got: {stderr}"
    );
    assert!(
        !stderr.contains("--bound"),
        "should not mention --bound after optimization upgrade, got: {stderr}"
    );
}

#[test]
fn test_create_multiple_copy_file_allocation_rejects_length_mismatch() {
    let output = pred()
        .args([
            "create",
            "MultipleCopyFileAllocation",
            "--graph",
            "0-1,1-2,2-3",
            "--usage",
            "5,4",
            "--storage",
            "1,1,1,1",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("usage"),
        "expected usage-length diagnostic, got: {stderr}"
    );
    assert!(
        stderr.contains("Usage: pred create MultipleCopyFileAllocation"),
        "expected recovery usage hint, got: {stderr}"
    );
}

#[test]
fn test_create_sequencing_to_minimize_maximum_cumulative_cost_missing_costs() {
    let output = pred()
        .args([
            "create",
            "SequencingToMinimizeMaximumCumulativeCost",
            "--precedences",
            "0>1",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("missing required construction input(s): costs"),
        "expected missing --costs message, got: {stderr}"
    );
}

#[test]
fn test_create_multiple_copy_file_allocation_rejects_storage_length_mismatch() {
    let output = pred()
        .args([
            "create",
            "MultipleCopyFileAllocation",
            "--graph",
            "0-1,1-2,2-3",
            "--usage",
            "5,4,3,2",
            "--storage",
            "1,1",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("storage"),
        "expected storage-length diagnostic, got: {stderr}"
    );
    assert!(
        stderr.contains("Usage: pred create MultipleCopyFileAllocation"),
        "expected recovery usage hint, got: {stderr}"
    );
}

#[test]
fn test_create_sequencing_to_minimize_maximum_cumulative_cost_bad_precedence() {
    let output = pred()
        .args([
            "create",
            "SequencingToMinimizeMaximumCumulativeCost",
            "--costs",
            "1,-1,2",
            "--precedences",
            "0>3",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("precedence"),
        "expected precedence validation error, got: {stderr}"
    );
}

#[test]
fn test_create_multiple_copy_file_allocation_rejects_invalid_usage_values() {
    let output = pred()
        .args([
            "create",
            "MultipleCopyFileAllocation",
            "--graph",
            "0-1,1-2,2-3",
            "--usage",
            "5,x,3,2",
            "--storage",
            "1,1,1,1",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("invalid digit found in string"),
        "expected usage parse diagnostic, got: {stderr}"
    );
    assert!(
        stderr.contains("Usage: pred create MultipleCopyFileAllocation"),
        "expected recovery usage hint, got: {stderr}"
    );
}

#[test]
fn test_create_sequencing_to_minimize_maximum_cumulative_cost_invalid_precedence_pair() {
    let output = pred()
        .args([
            "create",
            "SequencingToMinimizeMaximumCumulativeCost",
            "--costs",
            "1,-1,2",
            "--precedences",
            "a>b",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("--precedences"),
        "expected flag-specific precedence parse error, got: {stderr}"
    );
}

#[test]
fn test_create_sequencing_to_minimize_maximum_cumulative_cost_allows_negative_values() {
    let output = pred()
        .args([
            "create",
            "SequencingToMinimizeMaximumCumulativeCost",
            "--costs",
            "-1,2,-3",
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
    assert_eq!(json["data"]["costs"], serde_json::json!([-1, 2, -3]));
}

#[test]
fn test_evaluate_multiprocessor_scheduling_rejects_zero_processors_json() {
    let problem_file =
        std::env::temp_dir().join("pred_test_eval_multiprocessor_zero_processors.json");
    std::fs::write(
        &problem_file,
        r#"{
  "type": "MultiprocessorScheduling",
  "variant": {},
  "data": {
    "lengths": [1, 2],
    "num_processors": 0,
    "deadline": 5
  }
}"#,
    )
    .unwrap();

    let output = pred()
        .args([
            "evaluate",
            problem_file.to_str().unwrap(),
            "--config",
            "0,0",
        ])
        .output()
        .unwrap();
    assert!(
        !output.status.success(),
        "stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("expected positive integer, got 0"),
        "stderr: {stderr}"
    );

    std::fs::remove_file(&problem_file).ok();
}

#[test]
fn test_solve_multiple_copy_file_allocation_brute_force() {
    let problem_file = std::env::temp_dir().join("pred_test_solve_mcfa_bf.json");
    let create_out = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "--example",
            "MultipleCopyFileAllocation",
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
        stdout.contains("\"kind\": \"brute-force\""),
        "MultipleCopyFileAllocation should solve with brute-force: {stdout}"
    );

    std::fs::remove_file(&problem_file).ok();
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
        stderr.contains("Unknown variant value \"KingsSubgraph\""),
        "should reject the unregistered variant: {stderr}"
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
fn test_path_max_paths_truncates() {
    let output = pred()
        .args(["path", "KSat", "QUBO", "--max-paths", "3", "--json"])
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
    // KSat -> QUBO has many paths, so truncation is expected
    assert_eq!(
        envelope["truncated"], true,
        "should be truncated since KSat->QUBO has many paths"
    );
}

// Helper: run `pred path S T --max-paths N --json` and return the ordered
// list of per-path step counts.
fn path_step_counts(max_paths: &str) -> Vec<u64> {
    let output = pred()
        .args(["path", "KSat", "QUBO", "--max-paths", max_paths, "--json"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let envelope: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    envelope["paths"]
        .as_array()
        .expect("should have paths array")
        .iter()
        .map(|p| p["steps"].as_u64().expect("steps is a number"))
        .collect()
}

#[test]
fn test_path_truncates_after_sorting_not_before() {
    // Path enumeration must order length-first and truncate only after
    // ordering, so a small --max-paths returns the SHORTEST routes, not whichever
    // routes DFS discovered first. Compare a tightly-truncated run against a run
    // with a generous budget.
    let full = path_step_counts("500");
    assert!(full.len() > 3, "KSat->QUBO should have many routes");

    // Full list is sorted shortest-first.
    assert!(
        full.windows(2).all(|w| w[0] <= w[1]),
        "paths must be returned shortest-first, got {full:?}"
    );
    let shortest = *full.first().unwrap();

    let truncated = path_step_counts("3");
    assert!(truncated.len() <= 3);
    // Truncated result is still sorted shortest-first...
    assert!(
        truncated.windows(2).all(|w| w[0] <= w[1]),
        "truncated paths must be shortest-first, got {truncated:?}"
    );
    // ...and it must include the known shortest length (the bug returned long
    // early-discovered routes and dropped the short ones).
    assert_eq!(
        truncated[0], shortest,
        "truncated result must start with the known shortest route length {shortest}"
    );
    // The truncated step counts are exactly the shortest prefix of the full order.
    assert_eq!(truncated.as_slice(), &full[..truncated.len()]);
}

#[test]
fn test_path_max_paths_text_truncation_note() {
    let output = pred()
        .args(["path", "KSat", "QUBO", "--max-paths", "2"])
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

#[test]
fn test_create_shortest_weight_constrained_path() {
    let output = pred()
        .args([
            "create",
            "ShortestWeightConstrainedPath",
            "--graph",
            "0-1,0-2,1-3,2-3,2-4,3-5,4-5,1-4",
            "--edge-lengths",
            "2,4,3,1,5,4,2,6",
            "--edge-weights",
            "5,1,2,3,2,3,1,1",
            "--source-vertex",
            "0",
            "--target-vertex",
            "5",
            "--weight-bound",
            "8",
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
    assert_eq!(json["type"], "ShortestWeightConstrainedPath");
    assert_eq!(json["data"]["source_vertex"], 0);
    assert_eq!(json["data"]["target_vertex"], 5);
    assert_eq!(json["data"]["weight_bound"], 8);
}

#[test]
fn test_create_shortest_weight_constrained_path_missing_source_vertex() {
    let output = pred()
        .args([
            "create",
            "ShortestWeightConstrainedPath",
            "--graph",
            "0-1,0-2,1-3,2-3,2-4,3-5,4-5,1-4",
            "--edge-lengths",
            "2,4,3,1,5,4,2,6",
            "--edge-weights",
            "5,1,2,3,2,3,1,1",
            "--target-vertex",
            "5",
            "--weight-bound",
            "8",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--source-vertex"), "stderr: {stderr}");
}

#[test]
fn test_create_shortest_weight_constrained_path_edge_length_count_mismatch() {
    let output = pred()
        .args([
            "create",
            "ShortestWeightConstrainedPath",
            "--graph",
            "0-1,0-2,1-3,2-3,2-4,3-5,4-5,1-4",
            "--edge-lengths",
            "2,4,3,1,5,4,2",
            "--edge-weights",
            "5,1,2,3,2,3,1,1",
            "--source-vertex",
            "0",
            "--target-vertex",
            "5",
            "--weight-bound",
            "8",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("edge_lengths has 7 entries, expected 8"),
        "stderr: {stderr}"
    );
}

#[test]
fn test_create_shortest_weight_constrained_path_no_flags_shows_vector_hints() {
    let output = pred()
        .args(["create", "ShortestWeightConstrainedPath"])
        .output()
        .unwrap();
    assert!(
        !output.status.success(),
        "should exit non-zero when showing help"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("--edge-lengths"),
        "expected '--edge-lengths' in help output, got: {stderr}"
    );
}

#[test]
fn test_create_shortest_weight_constrained_path_rejects_out_of_bounds_source_vertex() {
    let output = pred()
        .args([
            "create",
            "ShortestWeightConstrainedPath",
            "--graph",
            "0-1,0-2,1-3,2-3,2-4,3-5,4-5,1-4",
            "--edge-lengths",
            "2,4,3,1,5,4,2,6",
            "--edge-weights",
            "5,1,2,3,2,3,1,1",
            "--source-vertex",
            "9",
            "--target-vertex",
            "5",
            "--weight-bound",
            "8",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("source_vertex 9 is outside graph with 6 vertices"),
        "stderr: {stderr}"
    );
    assert!(
        !stderr.contains("panicked at"),
        "out-of-bounds input should produce a normal CLI error, got: {stderr}"
    );
}

#[test]
fn test_create_shortest_weight_constrained_path_requires_edge_lengths() {
    let output = pred()
        .args([
            "create",
            "ShortestWeightConstrainedPath",
            "--graph",
            "0-1,0-2,1-3,2-3,2-4,3-5,4-5,1-4",
            "--edge-weights",
            "5,1,2,3,2,3,1,1",
            "--source-vertex",
            "0",
            "--target-vertex",
            "5",
            "--weight-bound",
            "8",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("missing required construction input(s): edge_lengths"),
        "stderr: {stderr}"
    );
}

#[test]
fn test_create_shortest_weight_constrained_path_rejects_weights_flag_typo() {
    let output = pred()
        .args([
            "create",
            "ShortestWeightConstrainedPath",
            "--graph",
            "0-1,0-2,1-3,2-3,2-4,3-5,4-5,1-4",
            "--edge-lengths",
            "2,4,3,1,5,4,2,6",
            "--weights",
            "5,1,2,3,2,3,1,1",
            "--source-vertex",
            "0",
            "--target-vertex",
            "5",
            "--weight-bound",
            "8",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("unexpected argument '--weights'"),
        "stderr: {stderr}"
    );
}

#[test]
fn test_create_shortest_weight_constrained_path_rejects_non_positive_edge_lengths() {
    let output = pred()
        .args([
            "create",
            "ShortestWeightConstrainedPath",
            "--graph",
            "0-1,0-2,1-3,2-3,2-4,3-5,4-5,1-4",
            "--edge-lengths=-2,4,3,1,5,4,2,6",
            "--edge-weights",
            "5,1,2,3,2,3,1,1",
            "--source-vertex",
            "0",
            "--target-vertex",
            "5",
            "--weight-bound",
            "8",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("edge_lengths must be positive"),
        "stderr: {stderr}"
    );
}

#[test]
fn test_show_shortest_weight_constrained_path_uses_weight_schema_type_names() {
    let output = pred()
        .args(["show", "ShortestWeightConstrainedPath"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("edge_lengths (Vec<i32>)"),
        "expected concrete Vec<i32> construction type for edge_lengths, got: {stdout}"
    );
    assert!(
        stdout.contains("edge_weights (Vec<i32>)"),
        "expected concrete Vec<i32> construction type for edge_weights, got: {stdout}"
    );
    assert!(
        stdout.contains("weight_bound (i64)"),
        "expected concrete i64 construction type for weight_bound, got: {stdout}"
    );
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
        stderr.contains("expected 1 for One, got 3"),
        "stderr should reject non-unit input for the One variant: {stderr}"
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
    assert_eq!(json["evaluation"], "Max(5)");
}

#[test]
fn test_create_minimum_multiway_cut() {
    let output_file = std::env::temp_dir().join("pred_test_create_minimum_multiway_cut.json");
    let output = pred()
        .args([
            "-o",
            output_file.to_str().unwrap(),
            "create",
            "MinimumMultiwayCut",
            "--graph",
            "0-1,1-2,2-3",
            "--terminals",
            "0,2",
            "--edge-weights",
            "1,1,1",
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
    assert_eq!(json["type"], "MinimumMultiwayCut");
    assert_eq!(json["variant"]["graph"], "SimpleGraph");
    assert_eq!(json["variant"]["weight"], "i32");
    assert_eq!(json["data"]["terminals"], serde_json::json!([0, 2]));
    assert_eq!(json["data"]["edge_weights"], serde_json::json!([1, 1, 1]));
    std::fs::remove_file(&output_file).ok();
}

#[test]
fn test_create_sequencing_within_intervals() {
    let output_file =
        std::env::temp_dir().join("pred_test_create_sequencing_within_intervals.json");
    let output = pred()
        .args([
            "-o",
            output_file.to_str().unwrap(),
            "create",
            "SequencingWithinIntervals",
            "--release-times",
            "0,0,0,0,5",
            "--deadlines",
            "11,11,11,11,6",
            "--lengths",
            "3,1,2,4,1",
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
    assert_eq!(json["type"], "SequencingWithinIntervals");
    assert_eq!(
        json["data"]["release_times"],
        serde_json::json!([0, 0, 0, 0, 5])
    );
    assert_eq!(
        json["data"]["deadlines"],
        serde_json::json!([11, 11, 11, 11, 6])
    );
    assert_eq!(json["data"]["lengths"], serde_json::json!([3, 1, 2, 4, 1]));
    std::fs::remove_file(&output_file).ok();
}

#[test]
fn test_create_ensemble_computation() {
    let output_file = std::env::temp_dir().join("pred_test_create_ensemble_computation.json");
    let output = pred()
        .args([
            "-o",
            output_file.to_str().unwrap(),
            "create",
            "EnsembleComputation",
            "--universe-size",
            "4",
            "--subsets",
            "0,1,2;0,1,3",
            "--budget",
            "4",
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
    assert_eq!(json["type"], "EnsembleComputation");
    assert_eq!(json["data"]["universe_size"], 4);
    assert_eq!(
        json["data"]["subsets"],
        serde_json::json!([[0, 1, 2], [0, 1, 3]])
    );
    assert_eq!(json["data"]["budget"], 4);
    std::fs::remove_file(&output_file).ok();
}

#[test]
fn test_create_ensemble_computation_no_flags_uses_cli_flag_names() {
    let output = pred()
        .args(["create", "EnsembleComputation"])
        .output()
        .unwrap();
    assert!(
        !output.status.success(),
        "problem-specific help should exit non-zero"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("--universe-size"),
        "expected --universe-size in help, got: {stderr}"
    );
    assert!(
        stderr.contains("--subsets"),
        "expected --subsets in help, got: {stderr}"
    );
    assert!(
        stderr.contains("--budget"),
        "expected --budget in help, got: {stderr}"
    );
}

#[test]
fn test_create_ensemble_computation_rejects_out_of_range_elements_without_panicking() {
    let output = pred()
        .args([
            "create",
            "EnsembleComputation",
            "--universe-size",
            "4",
            "--subsets",
            "0,1,5",
            "--budget",
            "4",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains("panicked at"),
        "expected graceful CLI error, got panic: {stderr}"
    );
    assert!(
        stderr.contains("outside universe") || stderr.contains("universe of size"),
        "expected out-of-range subset error, got: {stderr}"
    );
}

#[test]
fn test_create_scheduling_with_individual_deadlines() {
    let output_file =
        std::env::temp_dir().join("pred_test_create_scheduling_with_individual_deadlines.json");
    let output = pred()
        .args([
            "-o",
            output_file.to_str().unwrap(),
            "create",
            "SchedulingWithIndividualDeadlines",
            "--num-tasks",
            "7",
            "--deadlines",
            "2,1,2,2,3,3,2",
            "--num-processors",
            "3",
            "--precedences",
            "0>3,1>3,1>4,2>4,2>5",
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
    assert_eq!(json["type"], "SchedulingWithIndividualDeadlines");
    assert_eq!(json["data"]["num_processors"], 3);
    assert_eq!(json["data"]["num_tasks"], 7);
    std::fs::remove_file(&output_file).ok();
}

#[test]
fn test_create_model_example_multiprocessor_scheduling() {
    let output = pred()
        .args(["create", "--example", "MultiprocessorScheduling"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["type"], "MultiprocessorScheduling");
}

#[test]
fn test_create_model_example_consistency_of_database_frequency_tables() {
    let output = pred()
        .args([
            "create",
            "--example",
            "ConsistencyOfDatabaseFrequencyTables",
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
    assert_eq!(json["type"], "ConsistencyOfDatabaseFrequencyTables");
    assert_eq!(json["data"]["num_objects"], 6);
}

#[test]
fn test_create_model_example_minimum_multiway_cut() {
    let output = pred()
        .args(["create", "--example", "MinimumMultiwayCut"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["type"], "MinimumMultiwayCut");
    assert_eq!(json["variant"]["graph"], "SimpleGraph");
    assert_eq!(json["variant"]["weight"], "i32");
}

#[test]
fn test_create_model_example_sequencing_within_intervals() {
    let output = pred()
        .args(["create", "--example", "SequencingWithinIntervals"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["type"], "SequencingWithinIntervals");
}

#[test]
fn test_create_model_example_ensemble_computation() {
    let output = pred()
        .args(["create", "--example", "EnsembleComputation"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["type"], "EnsembleComputation");
}

#[test]
fn test_create_minimum_multiway_cut_rejects_single_terminal() {
    let output = pred()
        .args([
            "create",
            "MinimumMultiwayCut",
            "--graph",
            "0-1,1-2",
            "--edge-weights",
            "1,1",
            "--terminals",
            "0",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("terminal") || stderr.contains("Terminal"),
        "expected terminal-related error, got: {stderr}"
    );
}

#[test]
fn test_create_sequencing_within_intervals_rejects_empty_window() {
    let output = pred()
        .args([
            "create",
            "SequencingWithinIntervals",
            "--release-times",
            "5",
            "--deadlines",
            "3",
            "--lengths",
            "2",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains("panicked at"),
        "expected graceful CLI error, got panic: {stderr}"
    );
    assert!(
        stderr.contains("task 0 has an empty time window"),
        "expected empty-window validation error, got: {stderr}"
    );
}

#[test]
fn test_create_sequencing_within_intervals_rejects_mismatched_lengths() {
    let output = pred()
        .args([
            "create",
            "SequencingWithinIntervals",
            "--release-times",
            "0,1",
            "--deadlines",
            "2",
            "--lengths",
            "1,1",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains("panicked at"),
        "expected graceful CLI error, got panic: {stderr}"
    );
    assert!(
        stderr.contains("must have the same length"),
        "expected length validation error, got: {stderr}"
    );
}

#[test]
fn test_create_sequencing_within_intervals_rejects_overflow() {
    let output = pred()
        .args([
            "create",
            "SequencingWithinIntervals",
            "--release-times",
            "18446744073709551615",
            "--deadlines",
            "18446744073709551615",
            "--lengths",
            "1",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains("panicked at"),
        "expected graceful CLI error, got panic: {stderr}"
    );
    assert!(
        stderr.contains("task 0 release time plus length overflows u64"),
        "expected overflow validation error, got: {stderr}"
    );
}

#[test]
fn deterministic_solver_dispatch_rejects_non_override_solver_names() {
    let problem_file = std::env::temp_dir().join("pred_test_solve_customized_unsupported.json");
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

    for rejected in ["auto", "native", "fd-minimum-cardinality-key"] {
        let output = pred()
            .args([
                "solve",
                problem_file.to_str().unwrap(),
                "--solver",
                rejected,
            ])
            .output()
            .unwrap();
        assert!(!output.status.success(), "accepted --solver {rejected}");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains(&format!("Unknown solver: {rejected}")),
            "unexpected error for {rejected}: {stderr}"
        );
    }

    std::fs::remove_file(&problem_file).ok();
}

#[test]
fn deterministic_solver_dispatch_cli_output_is_repeatable_for_each_solver_class() {
    let problem_file = std::env::temp_dir().join("pred_test_solver_repeatability.json");
    let problem = serde_json::json!({
        "type": "RootedTreeArrangement",
        "variant": {"graph": "SimpleGraph"},
        "data": {
            "graph": {"num_vertices": 3, "edges": [[0, 1], [1, 2]]},
            "bound": 3
        }
    });
    std::fs::write(&problem_file, serde_json::to_vec(&problem).unwrap()).unwrap();

    for solver in [None, Some("customized"), Some("ilp"), Some("brute-force")] {
        let run = || {
            let mut command = pred();
            command.args(["--json", "solve", problem_file.to_str().unwrap()]);
            if let Some(solver) = solver {
                command.args(["--solver", solver]);
            }
            command.output().unwrap()
        };
        let first = run();
        let second = run();
        assert!(
            first.status.success(),
            "first {solver:?} solve failed: {}",
            String::from_utf8_lossy(&first.stderr)
        );
        assert!(
            second.status.success(),
            "second {solver:?} solve failed: {}",
            String::from_utf8_lossy(&second.stderr)
        );
        assert_eq!(first.stdout, second.stdout, "{solver:?} output changed");
    }

    std::fs::remove_file(&problem_file).ok();
}

#[test]
fn deterministic_solver_dispatch_defaults_minimum_cardinality_key_to_customized() {
    let problem_file = std::env::temp_dir().join("pred_test_solve_customized_mck.json");
    let create_out = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "MinimumCardinalityKey",
            "--num-attributes",
            "4",
            "--dependencies",
            "0>1,2;1,2>3",
        ])
        .output()
        .unwrap();
    assert!(
        create_out.status.success(),
        "create failed: {}",
        String::from_utf8_lossy(&create_out.stderr)
    );

    let output = pred()
        .args(["solve", problem_file.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "solve failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["solver"]["kind"], "customized");
    assert_eq!(
        json["solver"]["implementation"],
        "fd-minimum-cardinality-key"
    );
    assert!(
        stdout.contains("Min("),
        "expected Min(...) evaluation, got: {stdout}"
    );

    std::fs::remove_file(&problem_file).ok();
}

#[test]
fn test_solve_bundle_rejects_unavailable_customized_solver_without_panicking() {
    let problem_file = std::env::temp_dir().join("pred_test_solve_customized_bundle_problem.json");
    let bundle_file = std::env::temp_dir().join("pred_test_solve_customized_bundle.json");

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

    let reduce_out = reduce_named_to_file(
        &problem_file,
        "MIS/SimpleGraph/One",
        "QUBO",
        &[
            "MaximumIndependentSet",
            "MaximumIndependentSet",
            "MaximumSetPacking",
            "MaximumSetPacking",
            "QUBO",
        ],
        &bundle_file,
    );
    assert!(
        reduce_out.status.success(),
        "reduce failed: {}",
        String::from_utf8_lossy(&reduce_out.stderr)
    );

    let solve_out = pred()
        .args([
            "solve",
            bundle_file.to_str().unwrap(),
            "--solver",
            "customized",
        ])
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&solve_out.stderr);
    assert!(
        !stderr.contains("panicked at"),
        "unavailable customized solver should fail gracefully, got: {stderr}"
    );
    assert!(
        !solve_out.status.success(),
        "unavailable customized solver should not silently succeed"
    );
    assert!(
        stderr.contains("No customized solver is registered"),
        "expected missing customized capability error, got: {stderr}"
    );

    std::fs::remove_file(&problem_file).ok();
    std::fs::remove_file(&bundle_file).ok();
}

#[test]
fn test_inspect_minimum_cardinality_key_reports_customized_capability() {
    let problem_file = std::env::temp_dir().join("pred_test_inspect_customized_mck.json");
    let create_out = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "MinimumCardinalityKey",
            "--num-attributes",
            "4",
            "--dependencies",
            "0>1,2;1,2>3",
        ])
        .output()
        .unwrap();
    assert!(
        create_out.status.success(),
        "create failed: {}",
        String::from_utf8_lossy(&create_out.stderr)
    );

    let inspect_out = pred()
        .args(["inspect", problem_file.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(
        inspect_out.status.success(),
        "inspect failed: {}",
        String::from_utf8_lossy(&inspect_out.stderr)
    );

    let stdout = String::from_utf8(inspect_out.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["default_solver"], "customized");
    assert_eq!(
        json["solver_capabilities"]["customized"]["implementation"],
        "fd-minimum-cardinality-key"
    );

    std::fs::remove_file(&problem_file).ok();
}

/// Solve a bundle with brute-force and return `(target_config_csv, source_evaluation)`.
///
/// Used by extract tests so they do not depend on the exact reduction path chosen
/// (which differs between `--features mcp` and default builds).
fn extract_test_solve_bundle(bundle_file: &std::path::Path) -> (String, String) {
    let solve_out = pred()
        .args([
            "--json",
            "solve",
            bundle_file.to_str().unwrap(),
            "--solver",
            "brute-force",
        ])
        .output()
        .unwrap();
    assert!(
        solve_out.status.success(),
        "solve stderr: {}",
        String::from_utf8_lossy(&solve_out.stderr)
    );
    let json: serde_json::Value = serde_json::from_slice(&solve_out.stdout).unwrap();
    let target_cfg: Vec<String> = json["intermediate"]["solution"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap().to_string())
        .collect();
    let source_eval = json["evaluation"].as_str().unwrap().to_string();
    (target_cfg.join(","), source_eval)
}

#[test]
fn test_extract_roundtrip_mis_to_qubo() {
    let problem_file = std::env::temp_dir().join("pred_test_extract_in.json");
    let bundle_file = std::env::temp_dir().join("pred_test_extract_bundle.json");

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

    let reduce_out = reduce_named_to_file(
        &problem_file,
        "MIS/SimpleGraph/One",
        "QUBO",
        &[
            "MaximumIndependentSet",
            "MaximumIndependentSet",
            "MaximumSetPacking",
            "MaximumSetPacking",
            "QUBO",
        ],
        &bundle_file,
    );
    assert!(
        reduce_out.status.success(),
        "reduce stderr: {}",
        String::from_utf8_lossy(&reduce_out.stderr)
    );

    // Derive a valid target config from `pred solve`, so this test remains
    // independent of the reduction path selected by the graph search.
    let (target_cfg, expected_source_eval) = extract_test_solve_bundle(&bundle_file);

    let extract_out = pred()
        .args([
            "--json",
            "extract",
            bundle_file.to_str().unwrap(),
            "--config",
            &target_cfg,
        ])
        .output()
        .unwrap();
    assert!(
        extract_out.status.success(),
        "extract stderr: {}",
        String::from_utf8_lossy(&extract_out.stderr)
    );
    let stdout = String::from_utf8(extract_out.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["problem"].as_str().unwrap(), "MaximumIndependentSet");
    assert_eq!(json["reduced_to"].as_str().unwrap(), "QUBO");
    assert_eq!(json["solver"].as_str().unwrap(), "external");
    // extract on pred-solve's own target config must round-trip to the same source evaluation.
    assert_eq!(json["evaluation"].as_str().unwrap(), expected_source_eval);
    assert_eq!(json["intermediate"]["problem"].as_str().unwrap(), "QUBO");

    // intermediate.solution must be exactly the target config we passed in
    // (extract echoes the input target config unchanged).
    let expected_target: Vec<serde_json::Value> = target_cfg
        .split(',')
        .map(|s| serde_json::json!(s.parse::<u64>().unwrap()))
        .collect();
    assert_eq!(
        json["intermediate"]["solution"].as_array().unwrap(),
        &expected_target
    );

    // Source config is over 4 MIS variables and must describe an independent set
    // whose size matches `expected_source_eval` (e.g. "Max(2)" -> 2 ones).
    let source_sol: Vec<u64> = json["solution"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap())
        .collect();
    assert_eq!(source_sol.len(), 4);
    assert!(source_sol.iter().all(|b| *b == 0 || *b == 1));
    let ones = source_sol.iter().filter(|b| **b == 1).count();
    assert_eq!(
        expected_source_eval,
        format!("Max({ones})"),
        "MIS size in solution should match declared evaluation"
    );

    std::fs::remove_file(&problem_file).ok();
    std::fs::remove_file(&bundle_file).ok();
}

#[test]
fn test_extract_rejects_structurally_invalid_one_hot_config() {
    let problem_file = std::env::temp_dir().join("pred_test_extract_tsp_in.json");
    let bundle_file = std::env::temp_dir().join("pred_test_extract_tsp_bundle.json");

    let create_out = pred()
        .args([
            "-o",
            problem_file.to_str().unwrap(),
            "create",
            "TSP",
            "--graph",
            "0-1,1-2,0-2",
            "--edge-weights",
            "1,1,1",
        ])
        .output()
        .unwrap();
    assert!(
        create_out.status.success(),
        "create stderr: {}",
        String::from_utf8_lossy(&create_out.stderr)
    );

    let reduce_out = reduce_named_to_file(
        &problem_file,
        "TSP/SimpleGraph/i32",
        "QUBO",
        &["TravelingSalesman", "QUBO"],
        &bundle_file,
    );
    assert!(
        reduce_out.status.success(),
        "reduce stderr: {}",
        String::from_utf8_lossy(&reduce_out.stderr)
    );

    let extract_out = pred()
        .args([
            "extract",
            bundle_file.to_str().unwrap(),
            "--config",
            "0,0,0,0,0,0,0,0,0",
        ])
        .output()
        .unwrap();
    assert!(!extract_out.status.success());
    let stderr = String::from_utf8(extract_out.stderr).unwrap();
    assert!(
        stderr.contains("assignment slot 0 has no selected item"),
        "unexpected stderr: {stderr}"
    );

    std::fs::remove_file(&problem_file).ok();
    std::fs::remove_file(&bundle_file).ok();
}

#[test]
fn test_extract_rejects_plain_problem_file() {
    let problem_file = std::env::temp_dir().join("pred_test_extract_plain.json");

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

    let extract_out = pred()
        .args([
            "extract",
            problem_file.to_str().unwrap(),
            "--config",
            "0,1,0",
        ])
        .output()
        .unwrap();
    assert!(!extract_out.status.success());
    let stderr = String::from_utf8(extract_out.stderr).unwrap();
    assert!(
        stderr.contains("not a reduction bundle"),
        "unexpected stderr: {stderr}"
    );

    std::fs::remove_file(&problem_file).ok();
}

#[test]
fn test_extract_rejects_wrong_config_length() {
    let problem_file = std::env::temp_dir().join("pred_test_extract_wrong_len_in.json");
    let bundle_file = std::env::temp_dir().join("pred_test_extract_wrong_len_bundle.json");

    pred()
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
    reduce_named_to_file(
        &problem_file,
        "MIS/SimpleGraph/One",
        "QUBO",
        &[
            "MaximumIndependentSet",
            "MaximumIndependentSet",
            "MaximumSetPacking",
            "MaximumSetPacking",
            "QUBO",
        ],
        &bundle_file,
    );

    let extract_out = pred()
        .args(["extract", bundle_file.to_str().unwrap(), "--config", "0,1"])
        .output()
        .unwrap();
    assert!(!extract_out.status.success());
    let stderr = String::from_utf8(extract_out.stderr).unwrap();
    assert!(
        stderr.contains("Target config has 2 values"),
        "unexpected stderr: {stderr}"
    );

    std::fs::remove_file(&problem_file).ok();
    std::fs::remove_file(&bundle_file).ok();
}

#[test]
fn test_extract_rejects_out_of_range_config_value() {
    let problem_file = std::env::temp_dir().join("pred_test_extract_range_in.json");
    let bundle_file = std::env::temp_dir().join("pred_test_extract_range_bundle.json");

    pred()
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
    reduce_named_to_file(
        &problem_file,
        "MIS/SimpleGraph/One",
        "QUBO",
        &[
            "MaximumIndependentSet",
            "MaximumIndependentSet",
            "MaximumSetPacking",
            "MaximumSetPacking",
            "QUBO",
        ],
        &bundle_file,
    );

    // Build a valid-length config from pred solve, then flip one entry to 9
    // (always out of range for a binary QUBO regardless of path).
    let (target_cfg, _) = extract_test_solve_bundle(&bundle_file);
    let mut parts: Vec<String> = target_cfg.split(',').map(|s| s.to_string()).collect();
    parts[0] = "9".to_string();
    let bad_cfg = parts.join(",");

    let extract_out = pred()
        .args([
            "extract",
            bundle_file.to_str().unwrap(),
            "--config",
            &bad_cfg,
        ])
        .output()
        .unwrap();
    assert!(!extract_out.status.success());
    let stderr = String::from_utf8(extract_out.stderr).unwrap();
    assert!(
        stderr.contains("out of range"),
        "unexpected stderr: {stderr}"
    );

    std::fs::remove_file(&problem_file).ok();
    std::fs::remove_file(&bundle_file).ok();
}

#[test]
fn test_extract_rejects_malformed_bundle_path_source_mismatch() {
    use std::io::Write;

    let problem_file = std::env::temp_dir().join("pred_test_extract_malformed_in.json");
    let bundle_file = std::env::temp_dir().join("pred_test_extract_malformed_bundle.json");
    let tampered_file = std::env::temp_dir().join("pred_test_extract_malformed_tampered.json");

    pred()
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
    reduce_named_to_file(
        &problem_file,
        "MIS/SimpleGraph/One",
        "QUBO",
        &[
            "MaximumIndependentSet",
            "MaximumIndependentSet",
            "MaximumSetPacking",
            "MaximumSetPacking",
            "QUBO",
        ],
        &bundle_file,
    );

    let bundle_text = std::fs::read_to_string(&bundle_file).unwrap();
    let mut bundle: serde_json::Value = serde_json::from_str(&bundle_text).unwrap();
    // Tamper: make the source type disagree with path[0].
    bundle["source"]["type"] = serde_json::json!("NotTheRealSource");
    let mut f = std::fs::File::create(&tampered_file).unwrap();
    f.write_all(bundle.to_string().as_bytes()).unwrap();

    let extract_out = pred()
        .args([
            "extract",
            tampered_file.to_str().unwrap(),
            "--config",
            "0,1,0",
        ])
        .output()
        .unwrap();
    assert!(
        !extract_out.status.success(),
        "expected failure on malformed bundle; stdout: {}",
        String::from_utf8_lossy(&extract_out.stdout)
    );
    let stderr = String::from_utf8(extract_out.stderr).unwrap();
    assert!(
        stderr.contains("Malformed bundle"),
        "unexpected stderr: {stderr}"
    );

    std::fs::remove_file(&problem_file).ok();
    std::fs::remove_file(&bundle_file).ok();
    std::fs::remove_file(&tampered_file).ok();
}

#[test]
fn test_extract_rejects_tampered_target_data() {
    use std::io::Write;

    let problem_file = std::env::temp_dir().join("pred_test_extract_tampered_target_in.json");
    let bundle_file = std::env::temp_dir().join("pred_test_extract_tampered_target_bundle.json");
    let tampered_file =
        std::env::temp_dir().join("pred_test_extract_tampered_target_tampered.json");

    pred()
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
    reduce_named_to_file(
        &problem_file,
        "MIS/SimpleGraph/One",
        "QUBO",
        &[
            "MaximumIndependentSet",
            "MaximumIndependentSet",
            "MaximumSetPacking",
            "MaximumSetPacking",
            "QUBO",
        ],
        &bundle_file,
    );

    // Tamper: flip one QUBO matrix entry so target.data no longer matches
    // what the reduction chain actually produces.
    let bundle_text = std::fs::read_to_string(&bundle_file).unwrap();
    let mut bundle: serde_json::Value = serde_json::from_str(&bundle_text).unwrap();
    bundle["target"]["data"]["matrix"][0][0] = serde_json::json!(999.0);
    let mut f = std::fs::File::create(&tampered_file).unwrap();
    f.write_all(bundle.to_string().as_bytes()).unwrap();

    // Any config long enough to reach the coherence check; it must fail before
    // config validation kicks in because prepare() runs first.
    let (target_cfg, _) = extract_test_solve_bundle(&bundle_file);
    let extract_out = pred()
        .args([
            "extract",
            tampered_file.to_str().unwrap(),
            "--config",
            &target_cfg,
        ])
        .output()
        .unwrap();
    assert!(
        !extract_out.status.success(),
        "expected failure on tampered target.data; stdout: {}",
        String::from_utf8_lossy(&extract_out.stdout)
    );
    let stderr = String::from_utf8(extract_out.stderr).unwrap();
    assert!(
        stderr.contains("`target.data` does not match"),
        "unexpected stderr: {stderr}"
    );

    // Same check must also fire through `pred solve` on the tampered bundle —
    // BundleReplay::prepare is the shared gate.
    let solve_out = pred()
        .args([
            "solve",
            tampered_file.to_str().unwrap(),
            "--solver",
            "brute-force",
        ])
        .output()
        .unwrap();
    assert!(!solve_out.status.success());
    let solve_err = String::from_utf8(solve_out.stderr).unwrap();
    assert!(
        solve_err.contains("`target.data` does not match"),
        "pred solve should also reject tampered bundles; got: {solve_err}"
    );

    std::fs::remove_file(&problem_file).ok();
    std::fs::remove_file(&bundle_file).ok();
    std::fs::remove_file(&tampered_file).ok();
}

#[test]
fn test_extract_reads_bundle_from_stdin() {
    use std::io::Write;
    use std::process::Stdio;

    let problem_file = std::env::temp_dir().join("pred_test_extract_stdin_in.json");
    let bundle_file = std::env::temp_dir().join("pred_test_extract_stdin_bundle.json");

    pred()
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
    reduce_named_to_file(
        &problem_file,
        "MIS/SimpleGraph/One",
        "QUBO",
        &[
            "MaximumIndependentSet",
            "MaximumIndependentSet",
            "MaximumSetPacking",
            "MaximumSetPacking",
            "QUBO",
        ],
        &bundle_file,
    );
    let (target_cfg, _) = extract_test_solve_bundle(&bundle_file);
    let bundle_text = std::fs::read_to_string(&bundle_file).unwrap();

    let mut child = pred()
        .args(["--json", "extract", "-", "--config", &target_cfg])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(bundle_text.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["problem"].as_str().unwrap(), "MaximumIndependentSet");
    assert_eq!(json["reduced_to"].as_str().unwrap(), "QUBO");
    assert_eq!(json["solver"].as_str().unwrap(), "external");
    assert_eq!(json["evaluation"].as_str().unwrap(), "Max(2)");

    std::fs::remove_file(&problem_file).ok();
    std::fs::remove_file(&bundle_file).ok();
}
