#[cfg(test)]
mod tests {
    use crate::mcp::tools::McpServer;
    use crate::test_support::{aggregate_bundle, aggregate_problem_json};

    #[test]
    fn test_list_problems_returns_json() {
        let server = McpServer::new();
        let result = server.list_problems_inner();
        assert!(result.is_ok());
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert!(json["num_types"].as_u64().unwrap() > 0);
        assert!(json["problems"].as_array().unwrap().len() > 0);
    }

    #[test]
    fn test_show_problem_known() {
        let server = McpServer::new();
        let result = server.show_problem_inner("MIS");
        assert!(result.is_ok());
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(json["name"], "MaximumIndependentSet");
    }

    #[test]
    fn test_show_problem_unknown() {
        let server = McpServer::new();
        let result = server.show_problem_inner("NonExistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_find_path() {
        let server = McpServer::new();
        let result = server.find_path_inner("MIS", "QUBO", "minimize-steps", false, 20);
        assert!(result.is_ok());
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert!(json["path"].as_array().unwrap().len() > 0);
    }

    #[test]
    fn test_find_path_all() {
        let server = McpServer::new();
        let result = server.find_path_inner("MIS", "QUBO", "minimize-steps", true, 20);
        assert!(result.is_ok());
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        // --all returns a structured envelope
        assert!(json["paths"].as_array().unwrap().len() > 0);
        assert!(json["truncated"].is_boolean());
        assert!(json["returned"].is_u64());
        assert!(json["max_paths"].is_u64());
    }

    #[test]
    fn test_find_path_all_structured_response() {
        let server = McpServer::new();
        let result = server.find_path_inner("MIS", "QUBO", "minimize-steps", true, 20);
        assert!(result.is_ok());
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        // Verify the structured envelope fields
        let paths = json["paths"].as_array().unwrap();
        assert!(!paths.is_empty());
        let returned = json["returned"].as_u64().unwrap() as usize;
        assert_eq!(returned, paths.len());
        assert_eq!(json["max_paths"].as_u64().unwrap(), 20);
        // Each path should have steps, path, and overall_overhead
        let first = &paths[0];
        assert!(first["steps"].is_u64());
        assert!(first["path"].is_array());
        assert!(first["overall_overhead"].is_array());
    }

    #[test]
    fn test_find_path_no_route() {
        let server = McpServer::new();
        // Pick two problems with no path (if any). Use an unknown problem to trigger an error.
        let result = server.find_path_inner("NonExistent", "QUBO", "minimize-steps", false, 20);
        assert!(result.is_err());
    }

    #[test]
    fn test_show_problem_rejects_slash_spec() {
        let server = McpServer::new();
        let result = server.show_problem_inner("MIS/UnitDiskGraph");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("type level"),
            "error should mention type level: {err}"
        );
    }

    #[test]
    fn test_show_problem_marks_default() {
        let server = McpServer::new();
        let result = server.show_problem_inner("MIS");
        assert!(result.is_ok());
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        let variants = json["variants"].as_array().unwrap();
        // At least one variant should be marked as default
        let has_default = variants
            .iter()
            .any(|v| v["is_default"].as_bool() == Some(true));
        assert!(
            has_default,
            "expected at least one variant marked is_default=true"
        );
        // All variants should have the is_default field
        for v in variants {
            assert!(
                v["is_default"].is_boolean(),
                "expected is_default field on variant: {v}"
            );
        }
    }

    #[test]
    fn test_neighbors_out() {
        let server = McpServer::new();
        let result = server.neighbors_inner("MIS", 1, "out");
        assert!(result.is_ok());
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(json["direction"], "out");
        assert_eq!(json["hops"], 1);
    }

    #[test]
    fn test_neighbors_in() {
        let server = McpServer::new();
        let result = server.neighbors_inner("QUBO", 1, "in");
        assert!(result.is_ok());
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(json["direction"], "in");
    }

    #[test]
    fn test_neighbors_both() {
        let server = McpServer::new();
        let result = server.neighbors_inner("MIS", 1, "both");
        assert!(result.is_ok());
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(json["direction"], "both");
    }

    #[test]
    fn test_neighbors_unknown_problem() {
        let server = McpServer::new();
        let result = server.neighbors_inner("NonExistent", 1, "out");
        assert!(result.is_err());
    }

    #[test]
    fn test_neighbors_invalid_direction() {
        let server = McpServer::new();
        let result = server.neighbors_inner("MIS", 1, "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_export_graph() {
        let server = McpServer::new();
        let result = server.export_graph_inner();
        assert!(result.is_ok());
        // Verify it parses as valid JSON
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert!(json.is_object());
    }

    // -- Instance tool tests --------------------------------------------------

    fn create_test_mis(server: &McpServer) -> String {
        let params = serde_json::json!({"edges": "0-1,1-2,2-3"});
        server.create_problem_inner("MIS", &params).unwrap()
    }

    #[test]
    fn test_create_problem_mis() {
        let server = McpServer::new();
        let params = serde_json::json!({"edges": "0-1,1-2,2-3"});
        let result = server.create_problem_inner("MIS", &params);
        assert!(result.is_ok());
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(json["type"], "MaximumIndependentSet");
    }

    #[test]
    fn test_create_problem_sat() {
        let server = McpServer::new();
        let params = serde_json::json!({"num_vars": 3, "clauses": "1,2;-1,3"});
        let result = server.create_problem_inner("SAT", &params);
        assert!(result.is_ok());
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(json["type"], "Satisfiability");
    }

    #[test]
    fn test_create_problem_qubo() {
        let server = McpServer::new();
        let params = serde_json::json!({"matrix": "1,0.5;0.5,2"});
        let result = server.create_problem_inner("QUBO", &params);
        assert!(result.is_ok());
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(json["type"], "QUBO");
    }

    #[test]
    fn test_create_problem_maxcut() {
        let server = McpServer::new();
        let params = serde_json::json!({"edges": "0-1,1-2,2-0"});
        let result = server.create_problem_inner("MaxCut", &params);
        assert!(result.is_ok());
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(json["type"], "MaxCut");
    }

    #[test]
    fn test_create_problem_longest_circuit() {
        let server = McpServer::new();
        let params = serde_json::json!({
            "edges": "0-1,1-2,2-0",
            "edge_lengths": "2,3,4",
            "bound": 3
        });
        let result = server.create_problem_inner("LongestCircuit", &params);
        assert!(result.is_ok());
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(json["type"], "LongestCircuit");
        assert_eq!(json["data"]["edge_lengths"], serde_json::json!([2, 3, 4]));
        assert_eq!(json["data"]["bound"], 3);
    }

    #[test]
    fn test_create_problem_longest_circuit_random() {
        let server = McpServer::new();
        let params = serde_json::json!({
            "random": true,
            "num_vertices": 5,
            "seed": 7,
            "bound": 4
        });
        let result = server.create_problem_inner("LongestCircuit", &params);
        assert!(result.is_ok());
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(json["type"], "LongestCircuit");
        assert_eq!(json["data"]["bound"], 4);
    }

    #[test]
    fn test_create_problem_kcoloring() {
        let server = McpServer::new();
        let params = serde_json::json!({"edges": "0-1,1-2,2-0", "k": 3});
        let result = server.create_problem_inner("KColoring", &params);
        assert!(result.is_ok());
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(json["type"], "KColoring");
    }

    #[test]
    fn test_create_problem_factoring() {
        let server = McpServer::new();
        let params = serde_json::json!({"target": 15, "bits_m": 4, "bits_n": 4});
        let result = server.create_problem_inner("Factoring", &params);
        assert!(result.is_ok());
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(json["type"], "Factoring");
    }

    #[test]
    fn test_create_problem_unknown() {
        let server = McpServer::new();
        let params = serde_json::json!({"edges": "0-1"});
        let result = server.create_problem_inner("NonExistent", &params);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_problem_missing_edges() {
        let server = McpServer::new();
        let params = serde_json::json!({});
        let result = server.create_problem_inner("MIS", &params);
        assert!(result.is_err());
    }

    #[test]
    fn test_inspect_problem() {
        let server = McpServer::new();
        let problem_json = create_test_mis(&server);
        let result = server.inspect_problem_inner(&problem_json);
        assert!(result.is_ok());
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(json["type"], "MaximumIndependentSet");
        assert_eq!(json["kind"], "problem");
        assert!(json["num_variables"].as_u64().unwrap() > 0);
    }

    #[test]
    fn test_evaluate() {
        let server = McpServer::new();
        let problem_json = create_test_mis(&server);
        let result = server.evaluate_inner(&problem_json, &[1, 0, 1, 0]);
        assert!(result.is_ok());
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(json["problem"], "MaximumIndependentSet");
        assert_eq!(json["config"], serde_json::json!([1, 0, 1, 0]));
    }

    #[test]
    fn test_evaluate_wrong_config_length() {
        let server = McpServer::new();
        let problem_json = create_test_mis(&server);
        let result = server.evaluate_inner(&problem_json, &[1, 0]);
        assert!(result.is_err());
    }

    #[test]
    fn test_reduce() {
        let server = McpServer::new();
        let problem_json = create_test_mis(&server);
        let result = server.reduce_inner(&problem_json, "QUBO");
        assert!(result.is_ok());
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert!(json["target"].is_object());
        assert!(json["source"].is_object());
        assert!(json["path"].is_array());
    }

    #[test]
    fn test_reduce_unknown_target() {
        let server = McpServer::new();
        let problem_json = create_test_mis(&server);
        let result = server.reduce_inner(&problem_json, "NonExistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_solve() {
        let server = McpServer::new();
        let problem_json = create_test_mis(&server);
        let result = server.solve_inner(&problem_json, Some("brute-force"), None);
        assert!(result.is_ok());
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert!(json["solution"].is_array());
        assert_eq!(json["solver"], "brute-force");
    }

    #[test]
    fn test_solve_ilp() {
        let server = McpServer::new();
        let problem_json = create_test_mis(&server);
        let result = server.solve_inner(&problem_json, Some("ilp"), None);
        assert!(result.is_ok());
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert!(json["solution"].is_array());
    }

    #[test]
    fn test_solve_customized_supported_problem() {
        let server = McpServer::new();
        let problem_json = serde_json::json!({
            "type": "MinimumCardinalityKey",
            "variant": {},
            "data": {
                "num_attributes": 4,
                "dependencies": [[[0], [1, 2]], [[1, 2], [3]]],
                "bound": 2
            }
        })
        .to_string();

        let result = server.solve_inner(&problem_json, Some("customized"), None);
        assert!(result.is_ok(), "solve failed: {:?}", result);
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(json["solver"], "customized");
        assert!(json["solution"].is_array(), "{json}");
    }

    #[test]
    fn test_solve_unknown_solver() {
        let server = McpServer::new();
        let problem_json = create_test_mis(&server);
        let result = server.solve_inner(&problem_json, Some("unknown"), None);
        assert!(result.is_err());
    }

    #[test]
    fn test_solve_bundle() {
        let server = McpServer::new();
        let problem_json = create_test_mis(&server);
        // Reduce first, then solve the bundle
        let bundle_json = server.reduce_inner(&problem_json, "QUBO").unwrap();
        let result = server.solve_inner(&bundle_json, Some("brute-force"), None);
        assert!(result.is_ok());
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert!(json["solution"].is_array());
        assert_eq!(json["problem"], "MaximumIndependentSet");
    }

    #[test]
    fn test_solve_customized_bundle_rejects_unsupported_target_without_panicking() {
        let server = McpServer::new();
        let problem_json = create_test_mis(&server);
        let bundle_json = server.reduce_inner(&problem_json, "QUBO").unwrap();
        let result = server.solve_inner(&bundle_json, Some("customized"), None);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("unsupported by customized solver"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn test_inspect_bundle() {
        let server = McpServer::new();
        let problem_json = create_test_mis(&server);
        let bundle_json = server.reduce_inner(&problem_json, "QUBO").unwrap();
        let result = server.inspect_problem_inner(&bundle_json);
        assert!(result.is_ok());
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(json["kind"], "bundle");
        assert_eq!(json["source"], "MaximumIndependentSet");
    }

    #[test]
    fn test_inspect_minmaxmulticenter_lists_bruteforce_only() {
        let server = McpServer::new();
        let problem_json = serde_json::json!({
            "type": "MinMaxMulticenter",
            "variant": {"graph": "SimpleGraph", "weight": "i32"},
            "data": {
                "graph": {
                    "inner": {
                        "nodes": [null, null, null, null],
                        "node_holes": [],
                        "edge_property": "undirected",
                        "edges": [[0, 1, null], [1, 2, null], [2, 3, null]]
                    }
                },
                "vertex_weights": [1, 1, 1, 1],
                "edge_lengths": [1, 1, 1],
                "k": 2
            }
        })
        .to_string();

        let result = server.inspect_problem_inner(&problem_json);
        assert!(result.is_ok());
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        let solvers: Vec<&str> = json["solvers"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap())
            .collect();
        assert_eq!(solvers, vec!["brute-force"]);
    }

    #[test]
    fn test_inspect_minimum_cardinality_key_lists_customized_solver() {
        let server = McpServer::new();
        let problem_json = serde_json::json!({
            "type": "MinimumCardinalityKey",
            "variant": {},
            "data": {
                "num_attributes": 4,
                "dependencies": [[[0], [1, 2]], [[1, 2], [3]]],
                "bound": 2
            }
        })
        .to_string();

        let result = server.inspect_problem_inner(&problem_json);
        assert!(result.is_ok(), "inspect failed: {:?}", result);
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        let solvers: Vec<&str> = json["solvers"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap())
            .collect();
        assert!(
            solvers.contains(&"customized"),
            "inspect should list customized when supported, got: {json}"
        );
    }

    #[test]
    fn test_solve_sat_problem() {
        let server = McpServer::new();
        let params = serde_json::json!({"num_vars": 2, "clauses": "1;-2"});
        let problem_json = server.create_problem_inner("SAT", &params).unwrap();
        let result = server.solve_inner(&problem_json, Some("brute-force"), None);
        assert!(result.is_ok());
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(json["solver"], "brute-force");
    }

    #[test]
    fn test_reduce_rejects_aggregate_only_path() {
        let server = McpServer::new();
        let result = server.reduce_inner(&aggregate_problem_json(), "CliTestAggregateValueTarget");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("witness"), "unexpected error: {err}");
    }

    #[test]
    fn test_solve_aggregate_only_problem_omits_solution() {
        let server = McpServer::new();
        let result = server.solve_inner(&aggregate_problem_json(), Some("brute-force"), None);
        assert!(result.is_ok());
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(json["evaluation"], "Sum(56)");
        assert!(json.get("solution").is_none(), "{json}");
    }

    #[test]
    fn test_solve_ilp_rejects_aggregate_only_problem() {
        let server = McpServer::new();
        let result = server.solve_inner(&aggregate_problem_json(), Some("ilp"), None);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("witness-capable"), "unexpected error: {err}");
    }

    #[test]
    fn test_solve_bundle_rejects_aggregate_only_path() {
        let server = McpServer::new();
        let bundle_json = serde_json::to_string(&aggregate_bundle()).unwrap();
        let result = server.solve_inner(&bundle_json, Some("brute-force"), None);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("witness"), "unexpected error: {err}");
    }
}
