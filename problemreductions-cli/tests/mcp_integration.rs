//! Integration test for the MCP server (`pred mcp`).
//!
//! Spawns `pred mcp` as a subprocess, sends JSON-RPC messages over stdin using
//! newline-delimited JSON (the transport format used by rmcp's stdio transport),
//! reads responses from stdout, and validates the MCP protocol handshake and
//! tools/list response.

#[cfg(feature = "mcp")]
#[test]
fn test_mcp_server_initialize_and_list_tools() {
    use std::io::{BufRead, BufReader, Write};
    use std::process::{Command, Stdio};
    // Spawn `pred mcp` as a child process
    let mut child = Command::new(env!("CARGO_BIN_EXE_pred"))
        .arg("mcp")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start pred mcp");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    let stdout = child.stdout.take().expect("Failed to open stdout");
    let mut reader = BufReader::new(stdout);

    // Helper: send a JSON-RPC message as a single line + newline
    let send = |stdin: &mut std::process::ChildStdin, msg: &serde_json::Value| {
        let line = serde_json::to_string(msg).unwrap();
        writeln!(stdin, "{}", line).unwrap();
        stdin.flush().unwrap();
    };

    // Helper: read a JSON-RPC response line (newline-delimited JSON)
    let read_response = |reader: &mut BufReader<std::process::ChildStdout>| -> serde_json::Value {
        loop {
            let mut line = String::new();
            let bytes = reader.read_line(&mut line).expect("Failed to read line");
            if bytes == 0 {
                panic!("EOF from pred mcp before receiving a response");
            }
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue; // skip empty lines
            }
            // Try to parse; skip lines that are not valid JSON (e.g., log output)
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(trimmed) {
                // Only return JSON-RPC messages (have "jsonrpc" field)
                if val.get("jsonrpc").is_some() {
                    return val;
                }
            }
        }
    };

    // ---- Step 1: Send initialize request ----
    let init_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {
                "name": "mcp-integration-test",
                "version": "0.1.0"
            }
        }
    });
    send(stdin, &init_req);

    // ---- Step 2: Read initialize response ----
    let init_resp = read_response(&mut reader);

    // Validate it is a successful JSON-RPC response
    assert_eq!(
        init_resp["jsonrpc"], "2.0",
        "Response must be JSON-RPC 2.0"
    );
    assert_eq!(
        init_resp["id"], 1,
        "Response id must match request id"
    );
    assert!(
        init_resp.get("error").is_none(),
        "Initialize should not return an error: {:?}",
        init_resp
    );

    // Validate the result contains required fields
    let result = &init_resp["result"];
    assert!(
        result.get("protocolVersion").is_some(),
        "InitializeResult must contain protocolVersion, got: {}",
        serde_json::to_string_pretty(&init_resp).unwrap()
    );
    assert!(
        result.get("capabilities").is_some(),
        "InitializeResult must contain capabilities"
    );
    assert!(
        result.get("serverInfo").is_some(),
        "InitializeResult must contain serverInfo"
    );

    // The server should report tools and prompts capabilities
    let capabilities = &result["capabilities"];
    assert!(
        capabilities.get("tools").is_some(),
        "Server should advertise tools capability"
    );
    assert!(
        capabilities.get("prompts").is_some(),
        "Server should advertise prompts capability"
    );

    // Check server info
    let server_info = &result["serverInfo"];
    assert_eq!(
        server_info["name"], "problemreductions",
        "Server name should be 'problemreductions'"
    );

    // ---- Step 3: Send notifications/initialized notification ----
    let initialized_notif = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "notifications/initialized"
    });
    send(stdin, &initialized_notif);

    // No response expected for notifications — proceed to next request.

    // ---- Step 4: Send tools/list request ----
    let tools_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/list",
        "params": {}
    });
    send(stdin, &tools_req);

    // ---- Step 5: Read tools/list response ----
    let tools_resp = read_response(&mut reader);

    assert_eq!(tools_resp["jsonrpc"], "2.0");
    assert_eq!(tools_resp["id"], 2);
    assert!(
        tools_resp.get("error").is_none(),
        "tools/list should not return an error: {:?}",
        tools_resp
    );

    let tools_result = &tools_resp["result"];
    let tools = tools_result["tools"]
        .as_array()
        .expect("tools/list result should contain a 'tools' array");

    // The server should expose exactly 10 tools
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

    // Verify all expected tool names are present
    let tool_names: Vec<&str> = tools
        .iter()
        .filter_map(|t| t["name"].as_str())
        .collect();

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

    // Verify each tool has a description and inputSchema
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

    // ---- Step 6: Close stdin and wait for process to exit ----
    drop(child.stdin.take());
    let status = child.wait().expect("Failed to wait for pred mcp");
    // The server should exit cleanly when stdin is closed
    assert!(
        status.success(),
        "pred mcp should exit cleanly, got status: {}",
        status
    );
}

#[cfg(feature = "mcp")]
#[test]
fn test_mcp_server_prompts_list() {
    use std::io::{BufRead, BufReader, Write};
    use std::process::{Command, Stdio};

    let mut child = Command::new(env!("CARGO_BIN_EXE_pred"))
        .arg("mcp")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start pred mcp");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    let stdout = child.stdout.take().expect("Failed to open stdout");
    let mut reader = BufReader::new(stdout);

    let send = |stdin: &mut std::process::ChildStdin, msg: &serde_json::Value| {
        let line = serde_json::to_string(msg).unwrap();
        writeln!(stdin, "{}", line).unwrap();
        stdin.flush().unwrap();
    };

    let read_response = |reader: &mut BufReader<std::process::ChildStdout>| -> serde_json::Value {
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
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(trimmed) {
                if val.get("jsonrpc").is_some() {
                    return val;
                }
            }
        }
    };

    // Initialize handshake
    send(
        stdin,
        &serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": {"name": "test", "version": "0.1.0"}
            }
        }),
    );
    let _init_resp = read_response(&mut reader);

    send(
        stdin,
        &serde_json::json!({
            "jsonrpc": "2.0",
            "method": "notifications/initialized"
        }),
    );

    // Send prompts/list request
    send(
        stdin,
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
        3,
        "Expected 3 prompts, got {}: {:?}",
        prompts.len(),
        prompts
            .iter()
            .map(|p| p["name"].as_str().unwrap_or("?"))
            .collect::<Vec<_>>()
    );

    let prompt_names: Vec<&str> = prompts
        .iter()
        .filter_map(|p| p["name"].as_str())
        .collect();
    assert!(prompt_names.contains(&"analyze_problem"));
    assert!(prompt_names.contains(&"reduction_walkthrough"));
    assert!(prompt_names.contains(&"explore_graph"));

    // Clean shutdown
    drop(child.stdin.take());
    let status = child.wait().expect("Failed to wait for pred mcp");
    assert!(
        status.success(),
        "pred mcp should exit cleanly, got status: {}",
        status
    );
}
