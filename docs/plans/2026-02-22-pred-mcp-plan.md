# `pred mcp` Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add a `pred mcp` subcommand that runs a stdio-based MCP server exposing all CLI functionality as tools.

**Architecture:** Feature-gated `mcp` module using `rmcp` crate. The MCP server struct wraps the existing dispatch/commands layer. Tools are defined via `#[tool_router]` macros, prompts via `ServerHandler::list_prompts`.

**Tech Stack:** rmcp 0.16 (server + macros + transport-io), tokio 1, schemars 1.0

**Design doc:** `docs/plans/2026-02-22-pred-mcp-design.md`

---

### Task 1: Add dependencies and feature gate

**Files:**
- Modify: `problemreductions-cli/Cargo.toml`

**Step 1: Add MCP dependencies to Cargo.toml**

Add these to `[dependencies]`:
```toml
rmcp = { version = "0.16", features = ["server", "macros", "transport-io"], optional = true }
tokio = { version = "1", features = ["full"], optional = true }
schemars = { version = "1.0", optional = true }
tracing = { version = "0.1", optional = true }
tracing-subscriber = { version = "0.3", optional = true }
```

Add to `[features]`:
```toml
mcp = ["dep:rmcp", "dep:tokio", "dep:schemars", "dep:tracing", "dep:tracing-subscriber"]
```

Update `default`:
```toml
default = ["highs", "mcp"]
```

**Step 2: Verify it compiles**

Run: `cd problemreductions-cli && cargo check --features mcp`
Expected: compiles without errors

**Step 3: Verify it also compiles without the feature**

Run: `cargo check --no-default-features`
Expected: compiles without errors (existing CLI still works without MCP)

**Step 4: Commit**

```bash
git add problemreductions-cli/Cargo.toml
git commit -m "feat(cli): add rmcp dependencies behind mcp feature gate"
```

---

### Task 2: Add `Mcp` subcommand to CLI

**Files:**
- Modify: `problemreductions-cli/src/cli.rs`
- Modify: `problemreductions-cli/src/main.rs`

**Step 1: Add `Mcp` variant to Commands enum**

In `cli.rs`, add to the `Commands` enum (before `Completions`):

```rust
    /// Start MCP (Model Context Protocol) server for AI assistant integration
    #[cfg(feature = "mcp")]
    #[command(after_help = "\
Start a stdio-based MCP server that exposes problem reduction tools
to AI assistants like Claude Desktop and Claude Code.

Configuration (Claude Code .mcp.json):
  {
    \"mcpServers\": {
      \"problemreductions\": {
        \"command\": \"pred\",
        \"args\": [\"mcp\"]
      }
    }
  }

Test with MCP Inspector:
  npx @modelcontextprotocol/inspector pred mcp")]
    Mcp,
```

**Step 2: Add match arm in main.rs**

In `main.rs`, add the module declaration (gated):

```rust
#[cfg(feature = "mcp")]
mod mcp;
```

Add the match arm in the `match cli.command` block:

```rust
        #[cfg(feature = "mcp")]
        Commands::Mcp => mcp::run(),
```

**Step 3: Create stub mcp module**

Create `problemreductions-cli/src/mcp/mod.rs`:

```rust
pub fn run() -> anyhow::Result<()> {
    eprintln!("MCP server starting...");
    Ok(())
}
```

**Step 4: Verify it compiles and runs**

Run: `cargo run --features mcp -- mcp`
Expected: prints "MCP server starting..." and exits

Run: `cargo run --no-default-features -- --help`
Expected: help output does NOT show `mcp` subcommand

**Step 5: Commit**

```bash
git add problemreductions-cli/src/cli.rs problemreductions-cli/src/main.rs problemreductions-cli/src/mcp/mod.rs
git commit -m "feat(cli): add mcp subcommand (stub)"
```

---

### Task 3: Implement McpServer with graph query tools (list, show, neighbors, path, export)

These 6 tools are read-only graph metadata queries — no problem instances involved.

**Files:**
- Create: `problemreductions-cli/src/mcp/tools.rs`
- Modify: `problemreductions-cli/src/mcp/mod.rs`

**Step 1: Write a test for list_problems tool**

Create `problemreductions-cli/src/mcp/tests.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

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
        let result = server.find_path_inner("MIS", "QUBO", "minimize-steps", false);
        assert!(result.is_ok());
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert!(json["path"].as_array().unwrap().len() > 0);
    }

    #[test]
    fn test_neighbors_to() {
        let server = McpServer::new();
        let result = server.neighbors_inner("MIS", 1, "out");
        assert!(result.is_ok());
    }

    #[test]
    fn test_export_graph() {
        let server = McpServer::new();
        let result = server.export_graph_inner();
        assert!(result.is_ok());
    }
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test --features mcp -p problemreductions-cli mcp::tests`
Expected: FAIL — `McpServer` not defined

**Step 3: Implement McpServer struct and graph query tools**

In `problemreductions-cli/src/mcp/tools.rs`, implement `McpServer` with inner methods that return `anyhow::Result<String>` (JSON strings), and `#[tool_router]` annotated async wrappers that call these inner methods.

The inner methods reuse the logic from `commands/graph.rs` — specifically they construct `ReductionGraph::new()`, call the same methods, and build the same JSON structures. Do NOT call the existing command functions directly (they write to stdout via `OutputConfig`). Instead, replicate the JSON construction logic.

Key patterns:
- `McpServer` holds a `ToolRouter<McpServer>` field (required by rmcp macros)
- Each tool's async handler calls the sync inner method and converts to `CallToolResult`
- Error results use `CallToolResult::error(vec![Content::text(msg)])`
- Success results use `CallToolResult::success(vec![Content::text(json_string)])`
- Use `crate::problem_name::resolve_alias` for alias resolution
- Use `crate::problem_name::parse_problem_spec` for variant parsing

See `commands/graph.rs` lines 82-96 for the `list` JSON format, lines 99-250 for `show`, lines 252-450 for `neighbors`, lines 452-607 for `path`, and the `export` function.

Parameter structs (derive `Deserialize` + `schemars::JsonSchema`):

```rust
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ShowProblemParams {
    #[schemars(description = "Problem name or alias (e.g., MIS, QUBO, MaximumIndependentSet)")]
    pub problem: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct NeighborsParams {
    #[schemars(description = "Problem name or alias")]
    pub problem: String,
    #[schemars(description = "Number of hops to explore (default: 1)")]
    pub hops: Option<usize>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct FindPathParams {
    #[schemars(description = "Source problem name or alias")]
    pub source: String,
    #[schemars(description = "Target problem name or alias")]
    pub target: String,
    #[schemars(description = "Cost function: minimize-steps (default), or minimize:<field>")]
    pub cost: Option<String>,
    #[schemars(description = "Return all paths instead of just the cheapest")]
    pub all: Option<bool>,
}
```

**Step 4: Update mod.rs with ServerHandler impl**

In `mod.rs`:
- Import rmcp types
- Implement `ServerHandler` for `McpServer` with `#[tool_handler]`
- Implement `run()` using `tokio::runtime::Runtime::new()` (since main.rs is sync)
- The `run()` function: build runtime, create `McpServer`, call `server.serve(stdio()).await`, then `service.waiting().await`

```rust
use rmcp::{ServiceExt, transport::stdio};

pub fn run() -> anyhow::Result<()> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        tracing_subscriber::fmt()
            .with_writer(std::io::stderr)
            .with_ansi(false)
            .init();

        let server = tools::McpServer::new();
        let service = server.serve(stdio()).await
            .map_err(|e| anyhow::anyhow!("MCP server error: {e}"))?;
        service.waiting().await
            .map_err(|e| anyhow::anyhow!("MCP server error: {e}"))?;
        Ok(())
    })
}
```

**Step 5: Run tests**

Run: `cargo test --features mcp -p problemreductions-cli mcp::tests`
Expected: all 6 tests PASS

**Step 6: Manual test with MCP inspector**

Run: `npx @modelcontextprotocol/inspector cargo run --features mcp -- mcp`
Expected: inspector shows 6 tools, calling `list_problems` returns problem data

**Step 7: Commit**

```bash
git add problemreductions-cli/src/mcp/
git commit -m "feat(cli): implement MCP graph query tools (list, show, neighbors, path, export)"
```

---

### Task 4: Implement instance tools (create, inspect, evaluate, reduce, solve)

These tools operate on problem instances passed as JSON strings.

**Files:**
- Modify: `problemreductions-cli/src/mcp/tools.rs`
- Modify: `problemreductions-cli/src/mcp/tests.rs`

**Step 1: Write tests for instance tools**

Add to `tests.rs`:

```rust
    #[test]
    fn test_create_problem_mis() {
        let server = McpServer::new();
        let params = serde_json::json!({
            "edges": "0-1,1-2,2-3"
        });
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
            "clauses": "1,2;-1,3"
        });
        let result = server.create_problem_inner("SAT", &params);
        assert!(result.is_ok());
    }

    #[test]
    fn test_inspect_problem() {
        let server = McpServer::new();
        let problem_json = create_test_mis();
        let result = server.inspect_problem_inner(&problem_json);
        assert!(result.is_ok());
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(json["type"], "MaximumIndependentSet");
    }

    #[test]
    fn test_evaluate() {
        let server = McpServer::new();
        let problem_json = create_test_mis();
        let result = server.evaluate_inner(&problem_json, &[1, 0, 1, 0]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_reduce() {
        let server = McpServer::new();
        let problem_json = create_test_mis();
        let result = server.reduce_inner(&problem_json, "QUBO");
        assert!(result.is_ok());
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert!(json["target"].is_object());
    }

    #[test]
    fn test_solve() {
        let server = McpServer::new();
        let problem_json = create_test_mis();
        let result = server.solve_inner(&problem_json, Some("brute-force"), None);
        assert!(result.is_ok());
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert!(json["solution"].is_array());
    }

    fn create_test_mis() -> String {
        let server = McpServer::new();
        let params = serde_json::json!({"edges": "0-1,1-2,2-3"});
        server.create_problem_inner("MIS", &params).unwrap()
    }
```

**Step 2: Run tests to verify they fail**

Run: `cargo test --features mcp -p problemreductions-cli mcp::tests`
Expected: new tests FAIL — methods not defined

**Step 3: Implement create_problem**

Add `CreateProblemParams`:
```rust
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct CreateProblemParams {
    #[schemars(description = "Problem type (e.g., MIS, SAT, QUBO, MaxCut). Use list_problems to see all types.")]
    pub problem_type: String,
    #[schemars(description = "Problem parameters as JSON object. Graph problems: {\"edges\": \"0-1,1-2\", \"weights\": \"1,2,3\"}. SAT: {\"num_vars\": 3, \"clauses\": \"1,2;-1,3\"}. QUBO: {\"matrix\": \"1,0.5;0.5,2\"}. KColoring: {\"edges\": \"0-1,1-2\", \"k\": 3}. Factoring: {\"target\": 15, \"bits_m\": 4, \"bits_n\": 4}. Random graph: {\"random\": true, \"num_vertices\": 10, \"edge_prob\": 0.3}")]
    pub params: serde_json::Value,
}
```

The `create_problem_inner` method should:
1. Parse `params` to extract fields (edges, weights, clauses, etc.) — same logic as `CreateArgs` parsing in `commands/create.rs`
2. Build the `CreateArgs` struct and call the same construction logic
3. Return the JSON string of `ProblemJsonOutput`

Key: the `create` function in `commands/create.rs` writes to `OutputConfig`. Instead, extract the problem construction logic into a shared helper that returns `ProblemJsonOutput`, or replicate the construction in the MCP tool. The simplest approach: build a `CreateArgs` from the JSON params, then call the same graph/SAT/QUBO construction code. Since `CreateArgs` is just a data struct, you can construct it directly.

**Step 4: Implement inspect, evaluate, reduce, solve**

These are simpler — they take `problem_json: String` and parse it as `ProblemJson`:

- `inspect_problem_inner(problem_json)`: parse as `ProblemJson` or `ReductionBundle`, build the same JSON as `commands/inspect.rs`
- `evaluate_inner(problem_json, config)`: parse, `load_problem`, call `evaluate_dyn`, return result
- `reduce_inner(problem_json, target)`: parse, find path, execute reduction chain, serialize bundle
- `solve_inner(problem_json, solver, timeout)`: parse, `load_problem`, call solver, return result

Reuse from `dispatch.rs`: `load_problem`, `ProblemJson`, `ReductionBundle`, `serialize_any_problem`
Reuse from `problem_name.rs`: `resolve_alias`, `parse_problem_spec`

Parameter structs:
```rust
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct InspectParams {
    #[schemars(description = "Problem JSON string (from create_problem) or reduction bundle JSON")]
    pub problem_json: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct EvaluateParams {
    #[schemars(description = "Problem JSON string (from create_problem)")]
    pub problem_json: String,
    #[schemars(description = "Configuration to evaluate as array of integers (e.g., [1, 0, 1, 0])")]
    pub config: Vec<usize>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ReduceParams {
    #[schemars(description = "Problem JSON string (from create_problem)")]
    pub problem_json: String,
    #[schemars(description = "Target problem type (e.g., QUBO, ILP, SpinGlass)")]
    pub target: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SolveParams {
    #[schemars(description = "Problem JSON string (from create_problem or reduce)")]
    pub problem_json: String,
    #[schemars(description = "Solver: 'ilp' (default) or 'brute-force'")]
    pub solver: Option<String>,
    #[schemars(description = "Timeout in seconds (0 = no limit, default: 0)")]
    pub timeout: Option<u64>,
}
```

**Step 5: Run tests**

Run: `cargo test --features mcp -p problemreductions-cli mcp::tests`
Expected: all tests PASS

**Step 6: Manual test with MCP inspector**

Run: `npx @modelcontextprotocol/inspector cargo run --features mcp -- mcp`
Expected: all 10 tools visible. Test create → solve pipeline.

**Step 7: Commit**

```bash
git add problemreductions-cli/src/mcp/
git commit -m "feat(cli): implement MCP instance tools (create, inspect, evaluate, reduce, solve)"
```

---

### Task 5: Add MCP prompts

**Files:**
- Create: `problemreductions-cli/src/mcp/prompts.rs`
- Modify: `problemreductions-cli/src/mcp/mod.rs`

**Step 1: Implement prompts**

In `prompts.rs`, define 3 prompt templates. MCP prompts are returned via `ServerHandler::list_prompts` and `ServerHandler::get_prompt`.

Each prompt returns a `GetPromptResult` with a list of `PromptMessage` objects containing the prompt text.

```rust
use rmcp::model::*;

pub fn list_prompts() -> Vec<Prompt> {
    vec![
        Prompt {
            name: "analyze_problem".into(),
            description: Some("Analyze a problem type: show details, variants, reductions, and suggest strategies".into()),
            arguments: Some(vec![PromptArgument {
                name: "problem_type".into(),
                description: Some("Problem name or alias (e.g., MIS, QUBO)".into()),
                required: Some(true),
            }]),
        },
        Prompt {
            name: "reduction_walkthrough".into(),
            description: Some("End-to-end reduction walkthrough: find path, create instance, reduce, solve, verify".into()),
            arguments: Some(vec![
                PromptArgument {
                    name: "source".into(),
                    description: Some("Source problem (e.g., MIS)".into()),
                    required: Some(true),
                },
                PromptArgument {
                    name: "target".into(),
                    description: Some("Target problem (e.g., QUBO)".into()),
                    required: Some(true),
                },
            ]),
        },
        Prompt {
            name: "explore_graph".into(),
            description: Some("Explore the full reduction graph: list all problems, show structure, highlight hub problems".into()),
            arguments: None,
        },
    ]
}

pub fn get_prompt(name: &str, args: &std::collections::HashMap<String, String>) -> Option<GetPromptResult> {
    match name {
        "analyze_problem" => {
            let problem_type = args.get("problem_type")?;
            Some(GetPromptResult {
                description: Some(format!("Analyze the {} problem type", problem_type)),
                messages: vec![PromptMessage {
                    role: Role::User,
                    content: Content::text(format!(
                        "Please analyze the {} problem type using the problemreductions MCP tools:\n\
                        1. Use show_problem to get its details, variants, and available reductions\n\
                        2. Use neighbors_to to see what it can reduce to\n\
                        3. Use neighbors_from to see what reduces to it\n\
                        4. Suggest the most useful reduction strategies based on the graph structure\n\
                        5. Create a small example instance and solve it to demonstrate",
                        problem_type
                    )),
                }],
            })
        }
        "reduction_walkthrough" => {
            let source = args.get("source")?;
            let target = args.get("target")?;
            Some(GetPromptResult {
                description: Some(format!("Reduction walkthrough: {} to {}", source, target)),
                messages: vec![PromptMessage {
                    role: Role::User,
                    content: Content::text(format!(
                        "Please do an end-to-end reduction walkthrough from {} to {} using the problemreductions MCP tools:\n\
                        1. Use find_path to find the cheapest reduction path\n\
                        2. Explain what each step in the path does\n\
                        3. Use create_problem to create a small {} instance\n\
                        4. Use reduce to reduce it to {}\n\
                        5. Use solve to solve both the original and reduced instances\n\
                        6. Verify that the solutions are consistent",
                        source, target, source, target
                    )),
                }],
            })
        }
        "explore_graph" => {
            Some(GetPromptResult {
                description: Some("Explore the reduction graph".to_string()),
                messages: vec![PromptMessage {
                    role: Role::User,
                    content: Content::text(
                        "Please explore the problem reduction graph using the problemreductions MCP tools:\n\
                        1. Use list_problems to see all available problem types\n\
                        2. Use export_graph to get the full graph structure\n\
                        3. Identify the most connected hub problems (most reductions to/from)\n\
                        4. Show the key reduction pathways between major problem families\n\
                        5. Summarize the overall structure of the reduction landscape"
                    ),
                }],
            })
        }
        _ => None,
    }
}
```

**Step 2: Wire prompts into ServerHandler**

In `mod.rs`, override `list_prompts` and `get_prompt` in the `ServerHandler` impl. Update `ServerCapabilities` to include `.enable_prompts()`.

**Step 3: Run existing tests**

Run: `cargo test --features mcp -p problemreductions-cli mcp::tests`
Expected: all previous tests still PASS

**Step 4: Manual test**

Run: `npx @modelcontextprotocol/inspector cargo run --features mcp -- mcp`
Expected: 3 prompts visible in the inspector's Prompts tab

**Step 5: Commit**

```bash
git add problemreductions-cli/src/mcp/prompts.rs problemreductions-cli/src/mcp/mod.rs
git commit -m "feat(cli): add MCP prompt templates (analyze, walkthrough, explore)"
```

---

### Task 6: Integration test

**Files:**
- Create: `problemreductions-cli/tests/mcp_integration.rs`

**Step 1: Write integration test**

The test spawns `pred mcp` as a subprocess, sends JSON-RPC initialize + tools/list requests, and validates responses.

```rust
#[cfg(feature = "mcp")]
#[test]
fn test_mcp_server_initialize_and_list_tools() {
    use std::io::{BufRead, BufReader, Write};
    use std::process::{Command, Stdio};

    let mut child = Command::new(env!("CARGO_BIN_EXE_pred"))
        .arg("mcp")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start pred mcp");

    let stdin = child.stdin.as_mut().unwrap();
    let stdout = BufReader::new(child.stdout.as_mut().unwrap());

    // Send initialize request
    let init_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {"name": "test", "version": "0.1.0"}
        }
    });
    let msg = serde_json::to_string(&init_req).unwrap();
    writeln!(stdin, "{}", msg).unwrap();
    stdin.flush().unwrap();

    // Read response
    let mut line = String::new();
    stdout.read_line(&mut line).unwrap();
    let response: serde_json::Value = serde_json::from_str(&line).unwrap();
    assert_eq!(response["result"]["protocolVersion"], "2024-11-05");

    // Send tools/list
    let list_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/list",
        "params": {}
    });
    let msg = serde_json::to_string(&list_req).unwrap();
    writeln!(stdin, "{}", msg).unwrap();
    stdin.flush().unwrap();

    let mut line = String::new();
    stdout.read_line(&mut line).unwrap();
    let response: serde_json::Value = serde_json::from_str(&line).unwrap();
    let tools = response["result"]["tools"].as_array().unwrap();
    assert_eq!(tools.len(), 10);

    drop(stdin); // close stdin to terminate server
    child.wait().unwrap();
}
```

Note: the exact transport framing depends on rmcp's stdio implementation. It may use Content-Length headers (LSP-style) rather than newline-delimited JSON. Adjust the test accordingly after checking rmcp's stdio transport format. If it uses headers, you'll need to write/read `Content-Length: N\r\n\r\n{json}` frames.

**Step 2: Run the integration test**

Run: `cargo test --features mcp -p problemreductions-cli mcp_integration`
Expected: PASS

**Step 3: Commit**

```bash
git add problemreductions-cli/tests/mcp_integration.rs
git commit -m "test(cli): add MCP server integration test"
```

---

### Task 7: Add Makefile target and documentation

**Files:**
- Modify: `Makefile`
- Create: `docs/book/src/mcp.md`
- Modify: `docs/book/src/SUMMARY.md`
- Modify: `README.md`

**Step 1: Add Makefile target**

Add to Makefile:
```makefile
mcp-test:  ## Run MCP server tests
	cargo test --features mcp -p problemreductions-cli mcp
```

**Step 2: Add mdBook page**

Create `docs/book/src/mcp.md` with:
- What is MCP
- How to install and configure
- Available tools (table from design doc)
- Available prompts
- Example usage with Claude
- Testing with MCP Inspector

**Step 3: Add to SUMMARY.md**

Add `- [MCP Server](mcp.md)` to the book summary.

**Step 4: Update README**

Add a brief MCP section to README.md with install + configure instructions (3-4 lines).

**Step 5: Verify docs build**

Run: `make doc`
Expected: mdBook builds without errors

**Step 6: Run all tests**

Run: `make test clippy`
Expected: all tests pass, no clippy warnings

**Step 7: Commit**

```bash
git add Makefile docs/book/src/mcp.md docs/book/src/SUMMARY.md README.md
git commit -m "docs: add MCP server documentation and Makefile target"
```

---

### Task 8: Final verification

**Step 1: Full test suite**

Run: `make check`
Expected: fmt + clippy + test all pass

**Step 2: Build release binary**

Run: `cargo build --release -p problemreductions-cli`
Expected: builds successfully

**Step 3: End-to-end manual test**

Run: `npx @modelcontextprotocol/inspector ./target/release/pred mcp`

Test this sequence in the inspector:
1. Call `list_problems` — verify problem list
2. Call `create_problem` with `{problem_type: "MIS", params: {edges: "0-1,1-2,2-3"}}` — verify JSON
3. Call `solve` with the returned JSON — verify solution
4. Call `find_path` with `{source: "MIS", target: "QUBO"}` — verify path
5. Call `reduce` with the MIS JSON and target "QUBO" — verify bundle
6. Check prompts tab — verify 3 prompts appear

**Step 4: Verify no-mcp build still works**

Run: `cargo build --no-default-features --features highs -p problemreductions-cli`
Expected: builds successfully, `pred --help` shows no `mcp` subcommand
