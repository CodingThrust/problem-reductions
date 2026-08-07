#[cfg(test)]
mod tests {
    use crate::mcp::tools::{McpServer, SearchModeParam, SearchParams};
    use crate::test_support::{aggregate_bundle, aggregate_problem_json};

    fn explicit_route(server: &McpServer, source: &str, target: &str, names: &[&str]) -> String {
        let response = server
            .find_path_inner(source, target, false, 20, &SearchParams::default())
            .expect("front search");
        let json: serde_json::Value = serde_json::from_str(&response).unwrap();
        let entry = json["front"]
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
        let result = server.find_path_inner("MIS", "QUBO", false, 20, &SearchParams::default());
        assert!(result.is_ok());
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert!(!json["front"].as_array().unwrap().is_empty());
    }

    #[test]
    fn test_find_path_asymptotic_front() {
        // No `cost` and not `all` → the asymptotic Pareto front with structured Growth.
        let server = McpServer::new();
        let result = server.find_path_inner(
            "KSatisfiability",
            "QUBO",
            false,
            20,
            &SearchParams {
                search_mode: Some(SearchModeParam::Exact),
                ..Default::default()
            },
        );
        assert!(result.is_ok(), "err: {:?}", result.err());
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(json["mode"], "asymptotic");
        assert_eq!(json["completeness"]["status"], "exact");
        assert_eq!(json["limit_reasons"], serde_json::json!([]));
        assert!(json["stats"]["expanded_states"].is_number());
        let front = json["front"].as_array().unwrap();
        assert!(!front.is_empty());
        // The response includes structured Growth serialization.
        assert!(front[0]["growth"]["num_vars"]["Terms"].is_array());
        assert!(front[0]["big_o"]["num_vars"].is_string());
    }

    #[test]
    fn test_find_path_empty_bounded_result_is_incomplete_not_no_path() {
        let server = McpServer::new();
        let result = server.find_path_inner(
            "MIS",
            "QUBO",
            false,
            20,
            &SearchParams {
                max_hops: Some(0),
                ..Default::default()
            },
        );
        let error = result.expect_err("zero-hop bounded search must be incomplete");
        assert!(error.to_string().contains("Bounded search was incomplete"));
        assert!(!error.to_string().contains("No reduction path from"));
    }

    #[test]
    fn test_find_path_all_rejects_ranked_search_policy() {
        let server = McpServer::new();
        let result = server.find_path_inner(
            "MIS",
            "QUBO",
            true,
            20,
            &SearchParams {
                search_mode: Some(SearchModeParam::Exact),
                timeout: Some(1),
                ..Default::default()
            },
        );
        let error = result.expect_err("all-path enumeration must reject ranked search policy");
        assert!(error.to_string().contains("not all-path enumeration"));
    }

    #[test]
    fn test_find_path_front_has_no_top_level_winner() {
        let server = McpServer::new();
        let result = server.find_path_inner("MIS", "QUBO", false, 20, &SearchParams::default());
        assert!(result.is_ok(), "err: {:?}", result.err());
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(json["mode"], "asymptotic");
        assert!(json.get("path").is_none());
        let first = &json["front"][0]["path"][0];
        assert!(first["from"]["name"].is_string());
        assert!(first["to"]["name"].is_string());
        assert_eq!(first["from"]["name"], "MaximumIndependentSet");
    }

    #[test]
    fn test_find_path_all() {
        let server = McpServer::new();
        let result = server.find_path_inner("MIS", "QUBO", true, 20, &SearchParams::default());
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
        let result = server.find_path_inner("MIS", "QUBO", true, 20, &SearchParams::default());
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
    fn test_find_path_all_matches_library_order() {
        use crate::problem_name::resolve_problem_ref;
        use problemreductions::rules::ReductionGraph;

        // MCP `--all` must delegate to the library ordering (length-first, then
        // name+variant signature) with no local re-sort, so its ordered route list
        // is identical to what the library returns directly. This is also what the
        // CLI returns, since the CLI shares the same code path.
        let max_paths = 6usize;
        let server = McpServer::new();
        let result = server
            .find_path_inner(
                "KSatisfiability",
                "QUBO",
                true,
                max_paths,
                &SearchParams::default(),
            )
            .unwrap();
        let json: serde_json::Value = serde_json::from_str(&result).unwrap();
        let mcp_paths = json["paths"].as_array().unwrap();
        assert!(!mcp_paths.is_empty());

        // Reconstruct each MCP path as a sequence of node signatures "name/v1/v2".
        let node_sig = |node: &serde_json::Value| -> String {
            let mut s = node["name"].as_str().unwrap().to_string();
            if let Some(vars) = node["variant"].as_object() {
                // BTreeMap-like ordering: serde_json Map is insertion order, but the
                // library serialized from a BTreeMap so keys are already sorted.
                for v in vars.values() {
                    s.push('/');
                    s.push_str(v.as_str().unwrap());
                }
            }
            s
        };
        let mcp_sigs: Vec<Vec<String>> = mcp_paths
            .iter()
            .map(|p| {
                let steps = p["path"].as_array().unwrap();
                let mut seq = vec![node_sig(&steps[0]["from"])];
                for step in steps {
                    seq.push(node_sig(&step["to"]));
                }
                seq
            })
            .collect();

        // Reproduce the library-ordered, truncated route list the same way MCP/CLI do:
        // fetch max_paths + 1 then keep the first max_paths.
        let graph = ReductionGraph::new();
        let src = resolve_problem_ref("KSatisfiability", &graph).unwrap();
        let dst = resolve_problem_ref("QUBO", &graph).unwrap();
        let mut lib_paths = graph.find_paths_up_to(
            &src.name,
            &src.variant,
            &dst.name,
            &dst.variant,
            max_paths + 1,
        );
        lib_paths.truncate(max_paths);
        let lib_sigs: Vec<Vec<String>> = lib_paths
            .iter()
            .map(|p| {
                p.steps
                    .iter()
                    .map(|s| {
                        let mut sig = s.name.clone();
                        for v in s.variant.values() {
                            sig.push('/');
                            sig.push_str(v);
                        }
                        sig
                    })
                    .collect()
            })
            .collect();

        assert_eq!(
            mcp_sigs, lib_sigs,
            "MCP --all route list must equal the library-ordered list"
        );

        // And the route lengths are non-decreasing (length-first ordering).
        let lens: Vec<usize> = mcp_paths
            .iter()
            .map(|p| p["steps"].as_u64().unwrap() as usize)
            .collect();
        assert!(
            lens.windows(2).all(|w| w[0] <= w[1]),
            "MCP --all routes must be shortest-first, got {lens:?}"
        );
    }

    #[test]
    fn test_find_path_no_route() {
        let server = McpServer::new();
        // Pick two problems with no path (if any). Use an unknown problem to trigger an error.
        let result =
            server.find_path_inner("NonExistent", "QUBO", false, 20, &SearchParams::default());
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
    fn deterministic_solver_dispatch_defaults_supported_problem_to_native() {
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
        assert_eq!(json["solver"]["kind"], "native");
        assert_eq!(
            json["solver"]["implementation"],
            "fd-minimum-cardinality-key"
        );
        assert!(json["solution"].is_array(), "{json}");
    }

    #[test]
    fn test_solve_unknown_solver() {
        let server = McpServer::new();
        let problem_json = create_test_mis(&server);
        for rejected in ["auto", "customized", "native", "fd-minimum-cardinality-key"] {
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

        for solver in [None, Some("ilp"), Some("brute-force")] {
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

        for (clauses, evaluation, has_solution) in
            [("1;-1", "Or(false)", false), ("1", "Or(true)", true)]
        {
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

            assert_eq!(json["evaluation"], evaluation);
            assert_eq!(json["solution"].is_array(), has_solution);
            assert_eq!(json["intermediate"]["evaluation"], evaluation);
            assert_eq!(json["intermediate"]["solution"].is_array(), has_solution);
        }
    }

    #[test]
    fn test_solve_bundle_rejects_removed_customized_override() {
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
            err.contains("Unknown solver: customized"),
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
    fn test_inspect_minimum_cardinality_key_reports_native_solver() {
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
        assert_eq!(json["default_solver"], "native");
        assert_eq!(
            json["solver_capabilities"]["native"]["implementation"],
            "fd-minimum-cardinality-key"
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
}
