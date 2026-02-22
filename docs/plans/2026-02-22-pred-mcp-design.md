# Design: `pred mcp` — MCP Server for problemreductions-cli

## Summary

Add a `pred mcp` subcommand that runs a stdio-based MCP (Model Context Protocol) server, exposing all CLI functionality as MCP tools. This lets AI assistants (Claude Desktop, Claude Code, Cursor, etc.) interact with the problem reductions library natively.

## Approach

Native `rmcp` integration (Approach A). The MCP server calls directly into the existing library and dispatch code — no subprocess shelling. Dependencies gated behind an `mcp` Cargo feature (included in default features).

## User Experience

```bash
# Install (mcp included by default)
cargo install problemreductions-cli

# Configure in Claude Code (.mcp.json) or Claude Desktop
{
  "mcpServers": {
    "problemreductions": {
      "command": "pred",
      "args": ["mcp"]
    }
  }
}

# Verify
npx @modelcontextprotocol/inspector pred mcp
```

## Tools (10)

All CLI subcommands (minus `completions`) exposed as MCP tools. Problem instances are passed as inline JSON strings (stateless — no server-side state).

| Tool | Parameters | Returns |
|------|-----------|---------|
| `list_problems` | *(none)* | All registered problem types with aliases and reduction counts |
| `show_problem` | `problem: String` | Problem details: variants, size fields, reductions, schema |
| `neighbors_to` | `problem: String, hops?: u32` | Outgoing reduction neighbors |
| `neighbors_from` | `problem: String, hops?: u32` | Incoming reduction neighbors |
| `find_path` | `source: String, target: String, cost?: String, all?: bool` | Cheapest reduction path(s) |
| `export_graph` | *(none)* | Full reduction graph JSON |
| `create_problem` | `problem_type: String, params: JsonObject` | Problem instance JSON |
| `inspect_problem` | `problem_json: String` | Problem metadata (type, size, available solvers, reductions) |
| `evaluate` | `problem_json: String, config: Vec<usize>` | Evaluation result |
| `reduce` | `problem_json: String, target: String` | Reduction bundle JSON (source + target + path) |
| `solve` | `problem_json: String, solver?: String, timeout?: u64` | Solution + evaluation |

Notes:
- `params` for `create_problem` is a free-form JSON object. Tool description includes examples for common problem types.
- Alias resolution (MIS, 3SAT, etc.) works the same as CLI.

## Prompts (3)

| Prompt | Parameters | Purpose |
|--------|-----------|---------|
| `analyze_problem` | `problem_type: String` | Show problem details, variants, reductions, and suggest strategies |
| `reduction_walkthrough` | `source: String, target: String` | End-to-end: find path, explain steps, create instance, reduce, solve, verify |
| `explore_graph` | *(none)* | List all problems, show graph structure, highlight hub problems |

## Architecture

```
problemreductions-cli/
├── Cargo.toml              # add rmcp, tokio, schemars behind `mcp` feature
├── src/
│   ├── main.rs             # add Commands::Mcp variant
│   ├── cli.rs              # add Mcp subcommand to enum
│   ├── mcp/
│   │   ├── mod.rs          # McpServer struct + ServerHandler impl
│   │   ├── tools.rs        # #[tool_router] impl with all 10 tools
│   │   └── prompts.rs      # 3 prompt templates
│   ├── commands/            # existing, unchanged
│   └── dispatch.rs          # existing, reused by MCP tools
```

### Key Decisions

1. **Feature-gated** — `mcp` feature adds rmcp + tokio + schemars. Included in default features so `cargo install` just works.

2. **Reuse dispatch layer** — MCP tool handlers call the same functions the CLI commands use. No logic duplication.

3. **Sync in async** — Existing CLI logic is synchronous. Tool handlers use `tokio::task::spawn_blocking` where needed (mainly `solve` with timeout). Graph query tools are fast enough to call directly.

4. **Logging** — `tracing` output goes to stderr (required by MCP stdio protocol). Consistent with existing CLI stderr printing.

5. **Error handling** — Tool errors return `CallToolResult` with `is_error: true` and human-readable message. No panics.

### Data Flow

```
Claude Desktop
  ↓ JSON-RPC: tools/call "solve" {problem_json, solver: "brute-force"}
McpServer::solve()
  ↓ parse problem_json → serde_json::Value
dispatch::load_problem(name, variant, data)
  ↓ → LoadedProblem
LoadedProblem::solve_brute_force()
  ↓ → Option<Vec<usize>>
  ↓ format result as JSON string
CallToolResult::success(vec![Content::text(result)])
  ↑ JSON-RPC response
Claude Desktop
```

## Dependencies

```toml
[dependencies]
rmcp = { version = "0.16", features = ["server", "macros", "transport-io"], optional = true }
tokio = { version = "1", features = ["full"], optional = true }
schemars = { version = "1.0", optional = true }

[features]
mcp = ["dep:rmcp", "dep:tokio", "dep:schemars"]
default = ["highs", "mcp"]
```

## Testing

- Unit tests for each tool handler — call handler functions directly, assert JSON output structure
- Integration test: spawn `pred mcp` as subprocess, send JSON-RPC over stdin, validate responses
- Manual testing: `npx @modelcontextprotocol/inspector pred mcp`
- Add `make mcp-test` target

## Documentation

- Add `docs/book/mcp.md` to mdBook
- Update README with MCP quickstart section
- Include example conversation showing tool usage
