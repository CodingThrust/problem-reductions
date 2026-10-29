//! Integration test for the MCP server (`pred mcp`).
//!
//! Spawns `pred mcp` as a subprocess, sends JSON-RPC messages over stdin using
//! newline-delimited JSON (the transport format used by rmcp's stdio transport),
//! reads responses from stdout, and validates the MCP protocol handshake and
//! tools/list response.

#[cfg(feature = "mcp")]
mod mcp_tests {
    use std::io::{BufRead, BufReader, Write};
    use std::process::{ChildStdin, Command, Stdio};

    /// Spawn `pred mcp` and return (stdin, reader, child).
    fn spawn_mcp() -> (
        ChildStdin,
        BufReader<std::process::ChildStdout>,
        std::process::Child,
    ) {
        let mut child = Command::new(env!("CARGO_BIN_EXE_pred"))
            .arg("mcp")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .expect("Failed to start pred mcp");

        let stdin = child.stdin.take().expect("Failed to open stdin");
        let stdout = child.stdout.take().expect("Failed to open stdout");
        let reader = BufReader::new(stdout);
        (stdin, reader, child)
    }

    /// Send a JSON-RPC message as a single line + newline.
    fn send(stdin: &mut ChildStdin, msg: &serde_json::Value) {
        let line = serde_json::to_string(msg).unwrap();
        writeln!(stdin, "{}", line).unwrap();
        stdin.flush().unwrap();
    }

    /// Read a JSON-RPC response line (newline-delimited JSON), skipping non-JSON lines.
    fn read_response(reader: &mut BufReader<std::process::ChildStdout>) -> serde_json::Value {
        loop {
            let mut line = String::new();
            let bytes = reader.read_line(&mut line).expect("Failed to read line");
            if bytes == 0 {
                panic!("EOF from pred mcp before receiving a response");
            }
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(trimmed)
                && val.get("jsonrpc").is_some()
            {
                return val;
            }
        }
    }

    /// Perform the MCP initialize handshake (initialize request + initialized notification).
    fn initialize(stdin: &mut ChildStdin, reader: &mut BufReader<std::process::ChildStdout>) {
        send(
            stdin,
            &serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "initialize",
                "params": {
                    "protocolVersion": "2024-11-05",
                    "capabilities": {},
                    "clientInfo": {"name": "mcp-integration-test", "version": "0.1.0"}
                }
            }),
        );

        let init_resp = read_response(reader);
        assert_eq!(init_resp["jsonrpc"], "2.0");
        assert_eq!(init_resp["id"], 1);
        assert!(
            init_resp.get("error").is_none(),
            "Initialize should not return an error: {:?}",
            init_resp
        );

        let result = &init_resp["result"];
        assert!(result.get("protocolVersion").is_some());
        assert!(result.get("capabilities").is_some());
        assert!(result.get("serverInfo").is_some());

        let capabilities = &result["capabilities"];
        assert!(capabilities.get("tools").is_some());
        assert!(capabilities.get("prompts").is_some());

        assert_eq!(result["serverInfo"]["name"], "problemreductions");

        send(
            stdin,
            &serde_json::json!({
                "jsonrpc": "2.0",
                "method": "notifications/initialized"
            }),
        );
    }

    /// Shut down the child process cleanly and assert success.
    /// Takes ownership of `stdin` so it is dropped before waiting, triggering EOF.
    fn shutdown(stdin: ChildStdin, mut child: std::process::Child) {
        drop(stdin);
        let status = child.wait().expect("Failed to wait for pred mcp");
        assert!(
            status.success(),
            "pred mcp should exit cleanly, got status: {}",
            status
        );
    }

    #[test]
    fn test_mcp_server_initialize_list_and_call_tools() {
        let (mut stdin, mut reader, child) = spawn_mcp();
        initialize(&mut stdin, &mut reader);

        // Send tools/list request
        send(
            &mut stdin,
            &serde_json::json!({
                "jsonrpc": "2.0",
                "id": 2,
                "method": "tools/list",
                "params": {}
            }),
        );

        let tools_resp = read_response(&mut reader);
        assert_eq!(tools_resp["jsonrpc"], "2.0");
        assert_eq!(tools_resp["id"], 2);
        assert!(
            tools_resp.get("error").is_none(),
            "tools/list should not return an error: {:?}",
            tools_resp
        );

        let tools = tools_resp["result"]["tools"]
            .as_array()
            .expect("tools/list result should contain a 'tools' array");

        assert_eq!(
            tools.len(),
            10,
            "Expected 10 tools, got {}: {:?}",
            tools.len(),
            tools
                .iter()
                .map(|t| t["name"].as_str().unwrap_or("?"))
                .collect::<Vec<_>>()
        );

        let tool_names: Vec<&str> = tools.iter().filter_map(|t| t["name"].as_str()).collect();

        let expected_tools = [
            "list_problems",
            "show_problem",
            "neighbors",
            "find_path",
            "export_graph",
            "create_problem",
            "inspect_problem",
            "evaluate",
            "reduce",
            "solve",
        ];

        for expected in &expected_tools {
            assert!(
                tool_names.contains(expected),
                "Expected tool '{}' not found in tool list: {:?}",
                expected,
                tool_names
            );
        }

        for tool in tools {
            let name = tool["name"].as_str().unwrap_or("?");
            assert!(
                tool.get("description").is_some(),
                "Tool '{}' should have a description",
                name
            );
            assert!(
                tool.get("inputSchema").is_some(),
                "Tool '{}' should have an inputSchema",
                name
            );
        }

        send(
            &mut stdin,
            &serde_json::json!({
                "jsonrpc": "2.0",
                "id": 3,
                "method": "tools/call",
                "params": {"name": "list_problems", "arguments": {}}
            }),
        );
        let call_resp = read_response(&mut reader);
        assert_eq!(call_resp["id"], 3);
        assert!(call_resp.get("error").is_none(), "{call_resp}");
        assert_ne!(call_resp["result"]["isError"], true);
        assert!(
            call_resp["result"]["content"][0]["text"]
                .as_str()
                .unwrap()
                .contains("MaximumIndependentSet")
        );
        assert!(call_resp["result"].get("resultType").is_none());

        shutdown(stdin, child);
    }

    #[test]
    fn schema_graph_create_reports_construction_errors_over_mcp() {
        let (mut stdin, mut reader, child) = spawn_mcp();
        initialize(&mut stdin, &mut reader);
        let graph_cases = [
            (
                "HamiltonianCircuit",
                serde_json::json!({"graph": {"num_vertices": 3, "edges": [[0,0]]}}),
                "edge (0, 0) is a self-loop",
            ),
            (
                "HamiltonianCircuit",
                serde_json::json!({"graph": {"num_vertices": 3, "edges": [[0,1],[1,0]]}}),
                "edge (1, 0) duplicates an undirected edge",
            ),
            (
                "MaximumDomaticNumber",
                serde_json::json!({"graph": {"num_vertices": 3, "edges": [[0,1],[1,0]]}}),
                "edge (1, 0) duplicates an undirected edge",
            ),
            (
                "DirectedHamiltonianPath",
                serde_json::json!({"graph": {"num_vertices": 3, "arcs": [[0,1],[0,1]]}}),
                "arc (0, 1) duplicates a directed arc",
            ),
        ];
        let bipartite_cases = ["BicliqueCover", "BalancedCompleteBipartiteSubgraph"]
            .into_iter()
            .flat_map(|model| [
                (model, serde_json::json!({"left": 1, "right": 1, "biedges": [[0,0],[0,0]], "k": 1}), "edge (0, 1) duplicates an undirected edge"),
                (model, serde_json::json!({"left": usize::MAX, "right": 1, "biedges": [[0,0]], "k": 1}), "bipartite vertex count overflows usize"),
            ]);
        for (model, params, expected) in graph_cases.into_iter().chain(bipartite_cases) {
            send(
                &mut stdin,
                &serde_json::json!({
                    "jsonrpc": "2.0", "id": 2, "method": "tools/call",
                    "params": {"name": "create_problem", "arguments": {
                        "problem_type": model, "params": params,
                    }},
                }),
            );
            let response = read_response(&mut reader);
            assert_eq!(response["id"], 2);
            assert_eq!(response["result"]["isError"], true, "{response}");
            let error = response["result"]["content"][0]["text"].as_str().unwrap();
            assert!(error.contains(expected), "{error}");
            assert!(!error.contains("panicked"), "{error}");
        }
        shutdown(stdin, child);
    }

    #[test]
    fn simple_graph_create_rejects_loops_and_parallel_edges_over_mcp() {
        let (mut stdin, mut reader, child) = spawn_mcp();
        initialize(&mut stdin, &mut reader);
        for (edges, offending) in [
            (serde_json::json!([[0, 0]]), "edge (0, 0)"),
            (serde_json::json!([[0, 1], [0, 1]]), "edge (0, 1)"),
            (serde_json::json!([[0, 1], [1, 0]]), "edge (1, 0)"),
        ] {
            send(
                &mut stdin,
                &serde_json::json!({
                    "jsonrpc": "2.0", "id": 2, "method": "tools/call",
                    "params": {"name": "create_problem", "arguments": {
                        "problem_type": "MaximumIndependentSet/SimpleGraph/One",
                        "params": {"graph": edges, "num_vertices": 3},
                    }},
                }),
            );
            let response = read_response(&mut reader);
            assert_eq!(response["id"], 2);
            assert!(
                response.get("error").is_some() || response["result"]["isError"] == true,
                "{response}"
            );
            assert!(response.to_string().contains(offending), "{response}");
        }
        shutdown(stdin, child);
    }

    #[test]
    fn test_mcp_server_prompts_list_and_get() {
        let (mut stdin, mut reader, child) = spawn_mcp();
        initialize(&mut stdin, &mut reader);

        // Send prompts/list request
        send(
            &mut stdin,
            &serde_json::json!({
                "jsonrpc": "2.0",
                "id": 2,
                "method": "prompts/list",
                "params": {}
            }),
        );

        let prompts_resp = read_response(&mut reader);
        assert_eq!(prompts_resp["jsonrpc"], "2.0");
        assert_eq!(prompts_resp["id"], 2);
        assert!(
            prompts_resp.get("error").is_none(),
            "prompts/list should not return an error: {:?}",
            prompts_resp
        );

        let prompts = prompts_resp["result"]["prompts"]
            .as_array()
            .expect("prompts/list result should contain a 'prompts' array");

        assert_eq!(
            prompts.len(),
            7,
            "Expected 7 prompts, got {}: {:?}",
            prompts.len(),
            prompts
                .iter()
                .map(|p| p["name"].as_str().unwrap_or("?"))
                .collect::<Vec<_>>()
        );

        let prompt_names: Vec<&str> = prompts.iter().filter_map(|p| p["name"].as_str()).collect();
        assert!(prompt_names.contains(&"what_is"));
        assert!(prompt_names.contains(&"model_my_problem"));
        assert!(prompt_names.contains(&"compare"));
        assert!(prompt_names.contains(&"reduce"));
        assert!(prompt_names.contains(&"solve"));
        assert!(prompt_names.contains(&"find_reduction"));
        assert!(prompt_names.contains(&"overview"));

        send(
            &mut stdin,
            &serde_json::json!({
                "jsonrpc": "2.0",
                "id": 3,
                "method": "prompts/get",
                "params": {"name": "what_is", "arguments": {"problem": "QUBO"}}
            }),
        );
        let prompt_resp = read_response(&mut reader);
        assert_eq!(prompt_resp["id"], 3);
        let message = &prompt_resp["result"]["messages"][0];
        assert_eq!(message["role"], "user");
        assert_eq!(message["content"]["type"], "text");
        assert!(
            message["content"]["text"]
                .as_str()
                .unwrap()
                .contains("QUBO")
        );

        send(
            &mut stdin,
            &serde_json::json!({
                "jsonrpc": "2.0",
                "id": 4,
                "method": "prompts/get",
                "params": {"name": "unknown"}
            }),
        );
        let error_resp = read_response(&mut reader);
        assert_eq!(error_resp["id"], 4);
        assert_eq!(error_resp["error"]["code"], -32602);
        assert_eq!(error_resp["error"]["message"], "Unknown prompt: unknown");

        shutdown(stdin, child);
    }
    #[test]
    fn mixed_chinese_postman_parallel_loops_round_trip_over_mcp() {
        let (mut stdin, mut reader, child) = spawn_mcp();
        initialize(&mut stdin, &mut reader);
        send(
            &mut stdin,
            &serde_json::json!({
                "jsonrpc": "2.0", "id": 2, "method": "tools/call",
                "params": {"name": "create_problem", "arguments": {
                    "problem_type": "MixedChinesePostman/i64",
                    "params": {"graph": [[0,0],[0,0]], "arcs": [[0,0],[0,0]], "num_vertices": 1,
                        "edge_weights": [5,7], "arc_weights": [2,3]},
                }},
            }),
        );
        let response = read_response(&mut reader);
        assert!(response.get("error").is_none(), "{response}");
        assert_ne!(response["result"]["isError"], true, "{response}");
        let problem_json = response["result"]["content"][0]["text"].as_str().unwrap();
        let created: serde_json::Value = serde_json::from_str(problem_json).unwrap();
        assert_eq!(
            created["data"]["graph"]["edges"],
            serde_json::json!([[0, 0], [0, 0]])
        );
        assert_eq!(
            created["data"]["graph"]["arcs"],
            serde_json::json!([[0, 0], [0, 0]])
        );
        assert_eq!(created["data"]["arc_weights"], serde_json::json!([2, 3]));
        assert_eq!(created["data"]["edge_weights"], serde_json::json!([5, 7]));
        send(
            &mut stdin,
            &serde_json::json!({
                "jsonrpc": "2.0", "id": 3, "method": "tools/call",
                "params": {"name": "evaluate", "arguments": {
                    "problem_json": problem_json, "config": [false,true],
                }},
            }),
        );
        let response = read_response(&mut reader);
        assert!(response.get("error").is_none(), "{response}");
        assert_ne!(response["result"]["isError"], true, "{response}");
        let evaluated: serde_json::Value =
            serde_json::from_str(response["result"]["content"][0]["text"].as_str().unwrap())
                .unwrap();
        assert_eq!(evaluated["result"], "Min(17)");
        shutdown(stdin, child);
    }

    #[test]
    fn mixed_chinese_postman_create_rejects_invalid_parameters_over_mcp() {
        let (mut stdin, mut reader, child) = spawn_mcp();
        initialize(&mut stdin, &mut reader);
        for (arcs, edge_weights, arc_weights, expected) in [
            (vec![(0, 1)], vec![3], vec![2], "endpoint is out of range"),
            (vec![(0, 0)], vec![2, 3], vec![2], "edge_weights has length"),
            (vec![(0, 0)], vec![3], vec![-1], "must be nonnegative"),
        ] {
            send(
                &mut stdin,
                &serde_json::json!({
                    "jsonrpc": "2.0", "id": 2, "method": "tools/call",
                    "params": {"name": "create_problem", "arguments": {
                        "problem_type": "MixedChinesePostman/i64",
                        "params": {"graph": [[0,0]], "arcs": arcs, "num_vertices": 1,
                            "edge_weights": edge_weights, "arc_weights": arc_weights},
                    }},
                }),
            );
            let response = read_response(&mut reader);
            assert_eq!(response["id"], 2);
            assert_eq!(response["result"]["isError"], true, "{response}");
            let error = response["result"]["content"][0]["text"].as_str().unwrap();
            assert!(error.contains(expected), "{error}");
        }
        shutdown(stdin, child);
    }
}
