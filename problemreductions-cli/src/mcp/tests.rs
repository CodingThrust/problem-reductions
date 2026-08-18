use crate::mcp::tools::{FindPathParams, McpServer, PathLimitParam};
use crate::test_support::{aggregate_bundle, aggregate_problem_json};

fn explicit_route(server: &McpServer, source: &str, target: &str, names: &[&str]) -> String {
    let response = server
        .find_path_inner(source, target, 999, true, None)
        .expect("path enumeration");
    let json: serde_json::Value = serde_json::from_str(&response).unwrap();
    let entry = json["paths"]
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
        .expect("requested explicit route");
    serde_json::to_string(entry).unwrap()
}

#[test]
fn test_list_problems_returns_json() {
    let server = McpServer::new();
    let json: serde_json::Value =
        serde_json::from_str(&server.list_problems_inner().unwrap()).unwrap();
    assert!(json["num_types"].as_u64().unwrap() > 0);
}

#[test]
fn test_show_problem_known_and_unknown() {
    let server = McpServer::new();
    assert!(server.show_problem_inner("MIS").is_ok());
    assert!(server.show_problem_inner("NonExistent").is_err());
}

#[test]
fn test_find_path_enumerates_without_a_mode_or_sizes() {
    let server = McpServer::new();
    let result: serde_json::Value = serde_json::from_str(
        &server
            .find_path_inner(
                "MIS/SimpleGraph/i32",
                "MaximumClique/SimpleGraph/i32",
                20,
                false,
                None,
            )
            .unwrap(),
    )
    .unwrap();
    assert!(!result["paths"].as_array().unwrap().is_empty());
}

#[test]
fn test_find_path_executes_complete_instance_and_reports_actual_size() {
    let server = McpServer::new();
    let problem_json = r#"{
            "type":"MaximumIndependentSet",
            "variant":{"graph":"SimpleGraph","weight":"i32"},
            "data":{"graph":{"num_vertices":5,"edges":[[0,1],[1,2],[2,3],[3,4]]},"weights":[1,1,1,1,1]}
        }"#;
    let result: serde_json::Value = serde_json::from_str(
        &server
            .find_path_inner(
                "MIS/SimpleGraph/i32",
                "MaximumClique/SimpleGraph/i32",
                20,
                false,
                Some(problem_json),
            )
            .unwrap(),
    )
    .unwrap();
    let fields = result["paths"][0]["actual_target_size"]["fields"]
        .as_array()
        .unwrap();
    let edges = fields
        .iter()
        .find(|field| field["field"] == "num_edges")
        .unwrap();
    assert_eq!(edges["value"], 6);
}

#[test]
fn test_find_path_schema_accepts_complete_problem_json() {
    let params: FindPathParams = serde_json::from_value(serde_json::json!({
        "source": "MIS",
        "target": "MaximumClique",
        "problem_json": "{\"type\":\"MaximumIndependentSet\",\"variant\":{},\"data\":{}}"
    }))
    .unwrap();
    assert!(params
        .problem_json
        .unwrap()
        .contains("MaximumIndependentSet"));
}

#[test]
fn test_find_path_limit_all_resolves_to_999() {
    let params: FindPathParams = serde_json::from_value(serde_json::json!({
        "source": "MIS",
        "target": "QUBO",
        "limit": "all"
    }))
    .unwrap();
    assert_eq!(params.limit.as_ref().unwrap().resolve().unwrap(), 999);

    let numeric: PathLimitParam = serde_json::from_value(serde_json::json!(999)).unwrap();
    assert_eq!(numeric.resolve().unwrap(), 999);

    let numeric_string: PathLimitParam = serde_json::from_value(serde_json::json!("20")).unwrap();
    assert!(numeric_string.resolve().is_err());
}

#[test]
fn test_find_path_is_capped_explicitly() {
    let server = McpServer::new();
    let json: serde_json::Value = serde_json::from_str(
        &server
            .find_path_inner("MIS", "QUBO", 1, false, None)
            .unwrap(),
    )
    .unwrap();
    assert_eq!(json["paths"].as_array().unwrap().len(), 1);
    assert!(json.get("returned").is_none());
    assert!(json.get("max_paths").is_none());
    assert!(json.get("analysis").is_none());
    assert_eq!(json["truncated"], true);
}

#[test]
fn test_find_path_rejects_limit_above_maximum() {
    let error = McpServer::new()
        .find_path_inner("MIS", "QUBO", 1000, true, None)
        .unwrap_err();
    assert_eq!(
        error.to_string(),
        "limit must be an integer from 1 to 999 or 'all'"
    );
}

#[test]
fn test_neighbors_and_export_graph() {
    let server = McpServer::new();
    assert!(server.neighbors_inner("MIS", 1, "out").is_ok());
    assert!(server.neighbors_inner("MIS", 1, "invalid").is_err());
    let graph: serde_json::Value =
        serde_json::from_str(&server.export_graph_inner().unwrap()).unwrap();
    assert!(graph.is_object());
}

// -- Instance tool tests --------------------------------------------------

fn create_test_mis(server: &McpServer) -> String {
    let params = serde_json::json!({"graph": [[0, 1], [1, 2], [2, 3]]});
    server
        .create_problem_inner("MIS/SimpleGraph/i32", &params)
        .unwrap()
}

#[test]
fn test_create_problem_mis() {
    let server = McpServer::new();
    let params = serde_json::json!({"graph": [[0, 1], [1, 2], [2, 3]]});
    let result = server.create_problem_inner("MIS", &params);
    assert!(result.is_ok());
    let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
    assert_eq!(json["type"], "MaximumIndependentSet");
}

#[test]
fn test_create_problem_sat() {
    let server = McpServer::new();
    let params = serde_json::json!({
        "num_vars": 3,
        "clauses": [{"literals": [1, 2]}, {"literals": [-1, 3]}]
    });
    let result = server.create_problem_inner("SAT", &params);
    assert!(result.is_ok());
    let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
    assert_eq!(json["type"], "Satisfiability");
}

#[test]
fn test_create_problem_qubo() {
    let server = McpServer::new();
    let params = serde_json::json!({"matrix": [[1.0, 0.5], [0.5, 2.0]]});
    let result = server.create_problem_inner("QUBO", &params);
    assert!(result.is_ok());
    let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
    assert_eq!(json["type"], "QUBO");
}

#[test]
fn test_create_problem_maxcut() {
    let server = McpServer::new();
    let params = serde_json::json!({"graph": [[0, 1], [1, 2], [2, 0]]});
    let result = server.create_problem_inner("MaxCut", &params);
    assert!(result.is_ok());
    let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
    assert_eq!(json["type"], "MaxCut");
}

#[test]
fn test_create_problem_longest_circuit() {
    let server = McpServer::new();
    let params = serde_json::json!({
        "graph": [[0, 1], [1, 2], [2, 0]],
        "edge_weights": [2, 3, 4]
    });
    let result = server.create_problem_inner("LongestCircuit", &params);
    assert!(result.is_ok());
    let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
    assert_eq!(json["type"], "LongestCircuit");
    assert_eq!(json["data"]["edge_lengths"], serde_json::json!([2, 3, 4]));
}

#[test]
fn test_create_problem_longest_circuit_random() {
    let server = McpServer::new();
    let params = serde_json::json!({
        "random": true,
        "num_vertices": 5,
        "seed": 7
    });
    let result = server.create_problem_inner("LongestCircuit", &params);
    assert!(result.is_ok());
    let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
    assert_eq!(json["type"], "LongestCircuit");
    assert_eq!(json["data"]["graph"]["num_vertices"], 5);
    assert!(json["data"]["edge_lengths"]
        .as_array()
        .unwrap()
        .iter()
        .all(|length| length == 1));
}

#[test]
fn test_create_problem_kcoloring() {
    let server = McpServer::new();
    let params = serde_json::json!({"graph": [[0, 1], [1, 2], [2, 0]], "k": 3});
    let result = server.create_problem_inner("KColoring", &params);
    assert!(result.is_ok());
    let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
    assert_eq!(json["type"], "KColoring");
}

#[test]
fn test_create_problem_factoring() {
    let server = McpServer::new();
    let params = serde_json::json!({"target": 15, "m": 4, "n": 4});
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
    assert!(result.is_ok(), "inspect failed: {result:?}");
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
    let route = explicit_route(
        &server,
        "MIS/SimpleGraph/i32",
        "QUBO",
        &[
            "MaximumIndependentSet",
            "MaximumSetPacking",
            "MaximumSetPacking",
            "QUBO",
        ],
    );
    let result = server.reduce_inner(&problem_json, &route);
    assert!(result.is_ok(), "{result:?}");
    let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
    assert!(json["target"].is_object());
    assert!(json["source"].is_object());
    assert!(json["path"].is_array());
}

#[test]
fn test_reduce_unknown_target() {
    let server = McpServer::new();
    let problem_json = create_test_mis(&server);
    let result = server.reduce_inner(&problem_json, "{}");
    assert!(result.is_err());
}

#[test]
fn test_reduce_rejects_discontinuous_explicit_route() {
    let server = McpServer::new();
    let problem_json = create_test_mis(&server);
    let route = explicit_route(
        &server,
        "MIS/SimpleGraph/i32",
        "QUBO",
        &[
            "MaximumIndependentSet",
            "MaximumSetPacking",
            "MaximumSetPacking",
            "QUBO",
        ],
    );
    let mut route: serde_json::Value = serde_json::from_str(&route).unwrap();
    route["path"][1]["from"]["name"] = serde_json::json!("MinimumVertexCover");
    let error = server
        .reduce_inner(&problem_json, &route.to_string())
        .expect_err("discontinuous route must be rejected");
    assert!(error.to_string().contains("not continuous"));
}

#[test]
fn test_solve() {
    let server = McpServer::new();
    let problem_json = create_test_mis(&server);
    let result = server.solve_inner(&problem_json, Some("brute-force"), None);
    assert!(result.is_ok());
    let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
    assert!(json["solution"].is_array());
    assert_eq!(json["solver"]["kind"], "brute-force");
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
fn deterministic_solver_dispatch_defaults_supported_problem_to_customized() {
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

    let result = server.solve_inner(&problem_json, None, None);
    assert!(result.is_ok(), "solve failed: {:?}", result);
    let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
    assert_eq!(json["solver"]["kind"], "customized");
    assert_eq!(
        json["solver"]["implementation"],
        "fd-minimum-cardinality-key"
    );
    assert!(json["solution"].is_array(), "{json}");

    let explicit = server
        .solve_inner(&problem_json, Some("customized"), None)
        .unwrap();
    let explicit_json: serde_json::Value = serde_json::from_str(&explicit).unwrap();
    assert_eq!(explicit_json["solver"]["kind"], "customized");
}

#[test]
fn test_solve_unknown_solver() {
    let server = McpServer::new();
    let problem_json = create_test_mis(&server);
    for rejected in ["auto", "native", "fd-minimum-cardinality-key"] {
        let error = server
            .solve_inner(&problem_json, Some(rejected), None)
            .unwrap_err();
        assert!(
            error
                .to_string()
                .contains(&format!("Unknown solver: {rejected}")),
            "unexpected error for {rejected}: {error}"
        );
    }
}

#[test]
fn deterministic_solver_dispatch_mcp_output_is_repeatable_for_each_solver_class() {
    let server = McpServer::new();
    let problem_json = serde_json::json!({
        "type": "RootedTreeArrangement",
        "variant": {"graph": "SimpleGraph"},
        "data": {
            "graph": {"num_vertices": 3, "edges": [[0, 1], [1, 2]]},
            "bound": 3
        }
    })
    .to_string();

    for solver in [None, Some("customized"), Some("ilp"), Some("brute-force")] {
        let first = server.solve_inner(&problem_json, solver, None).unwrap();
        let second = server.solve_inner(&problem_json, solver, None).unwrap();
        assert_eq!(first, second, "{solver:?} MCP output changed");
    }
}

#[test]
fn test_solve_bundle() {
    let server = McpServer::new();
    let problem_json = create_test_mis(&server);
    // Reduce first, then solve the bundle
    let bundle_json = server
        .reduce_inner(
            &problem_json,
            &explicit_route(
                &server,
                "MIS/SimpleGraph/i32",
                "QUBO",
                &[
                    "MaximumIndependentSet",
                    "MaximumSetPacking",
                    "MaximumSetPacking",
                    "QUBO",
                ],
            ),
        )
        .unwrap();
    let result = server.solve_inner(&bundle_json, Some("brute-force"), None);
    assert!(result.is_ok());
    let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
    assert!(json["solution"].is_array());
    assert_eq!(json["problem"], "MaximumIndependentSet");
}

#[test]
fn test_solve_bundle_distinguishes_infeasibility_from_missing_witness_capability() {
    let server = McpServer::new();

    for (clauses, status, evaluation) in [
        (
            serde_json::json!([{"literals": [1]}, {"literals": [-1]}]),
            "infeasible",
            None,
        ),
        (
            serde_json::json!([{"literals": [1]}]),
            "optimal",
            Some("Or(true)"),
        ),
    ] {
        let problem_json = server
            .create_problem_inner(
                "Satisfiability",
                &serde_json::json!({"num_vars": 1, "clauses": clauses}),
            )
            .unwrap();
        let bundle_json = server
            .reduce_inner(
                &problem_json,
                &explicit_route(
                    &server,
                    "Satisfiability",
                    "NAESatisfiability",
                    &["Satisfiability", "NAESatisfiability"],
                ),
            )
            .unwrap();
        let solved = server
            .solve_inner(&bundle_json, Some("brute-force"), None)
            .unwrap();
        let json: serde_json::Value = serde_json::from_str(&solved).unwrap();

        assert_eq!(json["status"], status);
        assert_eq!(json.get("evaluation").and_then(|v| v.as_str()), evaluation);
        assert_eq!(json.get("solution").is_some(), evaluation.is_some());
        assert_eq!(json["intermediate"]["status"], status);
        assert_eq!(
            json["intermediate"]
                .get("evaluation")
                .and_then(|v| v.as_str()),
            evaluation
        );
        assert_eq!(
            json["intermediate"].get("solution").is_some(),
            evaluation.is_some()
        );
    }
}

#[test]
fn test_solve_bundle_rejects_unavailable_customized_solver() {
    let server = McpServer::new();
    let problem_json = create_test_mis(&server);
    let bundle_json = server
        .reduce_inner(
            &problem_json,
            &explicit_route(
                &server,
                "MIS/SimpleGraph/i32",
                "QUBO",
                &[
                    "MaximumIndependentSet",
                    "MaximumSetPacking",
                    "MaximumSetPacking",
                    "QUBO",
                ],
            ),
        )
        .unwrap();
    let result = server.solve_inner(&bundle_json, Some("customized"), None);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("No customized solver is registered"),
        "unexpected error: {err}"
    );
}

#[test]
fn test_inspect_bundle() {
    let server = McpServer::new();
    let problem_json = create_test_mis(&server);
    let bundle_json = server
        .reduce_inner(
            &problem_json,
            &explicit_route(
                &server,
                "MIS/SimpleGraph/i32",
                "QUBO",
                &[
                    "MaximumIndependentSet",
                    "MaximumSetPacking",
                    "MaximumSetPacking",
                    "QUBO",
                ],
            ),
        )
        .unwrap();
    let result = server.inspect_problem_inner(&bundle_json);
    assert!(result.is_ok());
    let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
    assert_eq!(json["kind"], "bundle");
    assert_eq!(json["source"], "MaximumIndependentSet");
}

#[test]
fn test_inspect_minmaxmulticenter_reports_registered_ilp_pipeline() {
    let server = McpServer::new();
    let problem_json = serde_json::json!({
        "type": "MinMaxMulticenter",
        "variant": {"graph": "SimpleGraph", "weight": "i32"},
        "data": {
            "graph": {
                "num_vertices": 4,
                "edges": [[0, 1], [1, 2], [2, 3]]
            },
            "vertex_weights": [1, 1, 1, 1],
            "edge_lengths": [1, 1, 1],
            "k": 2
        }
    })
    .to_string();

    let result = server.inspect_problem_inner(&problem_json);
    assert!(result.is_ok(), "inspect failed: {result:?}");
    let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
    assert_eq!(json["default_solver"], "ilp");
    assert!(json["solver_capabilities"]["ilp"]["reduction_path"].is_array());
}

#[test]
fn test_inspect_minimum_cardinality_key_reports_customized_solver() {
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
    assert_eq!(json["default_solver"], "customized");
    assert_eq!(
        json["solver_capabilities"]["customized"]["implementation"],
        "fd-minimum-cardinality-key"
    );
}

#[test]
fn test_solve_sat_problem() {
    let server = McpServer::new();
    let params = serde_json::json!({
        "num_vars": 2,
        "clauses": [{"literals": [1]}, {"literals": [-2]}]
    });
    let problem_json = server.create_problem_inner("SAT", &params).unwrap();
    let result = server.solve_inner(&problem_json, Some("brute-force"), None);
    assert!(result.is_ok());
    let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
    assert_eq!(json["solver"]["kind"], "brute-force");
}

#[test]
fn test_reduce_rejects_aggregate_only_path() {
    let server = McpServer::new();
    let route = serde_json::json!({"path": [{
        "from": {"name": "CliTestAggregateValueSource", "variant": {}},
        "to": {"name": "CliTestAggregateValueTarget", "variant": {}}
    }]})
    .to_string();
    let result = server.reduce_inner(&aggregate_problem_json(), &route);
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
    assert!(
        err.contains("No ILP pipeline is registered"),
        "unexpected error: {err}"
    );
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
