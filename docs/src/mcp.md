# MCP Server

The [Model Context Protocol (MCP)](https://modelcontextprotocol.io/) is an open standard that allows AI assistants to interact with external tools and data sources. The `pred` CLI includes a built-in MCP server that exposes the full reduction graph, problem creation, solving, and reduction capabilities to any MCP-compatible AI assistant (such as Claude Code, Cursor, or Windsurf).

## Installation

Install the `pred` CLI tool:

```bash
cargo install problemreductions-cli
```

Or build from source:

```bash
git clone https://github.com/CodingThrust/problem-reductions
cd problem-reductions
make cli    # builds target/release/pred
```

## Configuration

### Claude Code

Add the following to your project's `.mcp.json` file (or `~/.claude/mcp.json` for global configuration):

```json
{
  "mcpServers": {
    "problemreductions": {
      "command": "pred",
      "args": ["mcp"]
    }
  }
}
```

### Generic MCP Client

Any MCP client that supports the stdio transport can connect to the server by running:

```bash
pred mcp
```

The server communicates over stdin/stdout using the MCP JSON-RPC protocol.

## Available Tools

The MCP server provides 10 tools organized into two categories: **graph query tools** for exploring the reduction graph, and **instance tools** for working with concrete problem instances.

### Graph Query Tools

| Tool | Parameters | Description |
|------|-----------|-------------|
| `list_problems` | *(none)* | List all registered problem types with aliases, variant counts, and reduction counts |
| `show_problem` | `problem` (string) | Show details for a problem type: variants, size fields, schema, and incoming/outgoing reductions |
| `neighbors` | `problem` (string), `hops` (int, default: 1), `direction` ("out"\|"in"\|"both", default: "out") | Find neighboring problems reachable via reduction edges within a given hop distance |
| `find_path` | `source` (string), `target` (string), `cost` (string, default: "minimize-steps"), `all` (bool, default: false) | Find a reduction path between two problems, optionally minimizing a size field or returning all paths |
| `export_graph` | *(none)* | Export the full reduction graph as JSON (nodes, edges, overheads) |

### Instance Tools

| Tool | Parameters | Description |
|------|-----------|-------------|
| `create_problem` | `problem_type` (string), `params` (JSON object) | Create a problem instance from parameters and return its JSON representation. Supports graph problems, SAT, QUBO, SpinGlass, KColoring, Factoring, and random graph generation |
| `inspect_problem` | `problem_json` (string) | Inspect a problem JSON or reduction bundle: returns type, size metrics, available solvers, and reduction targets |
| `evaluate` | `problem_json` (string), `config` (array of int) | Evaluate a configuration against a problem instance and return the objective value or feasibility |
| `reduce` | `problem_json` (string), `target` (string) | Reduce a problem instance to a target type, returning a reduction bundle with the transformed instance and path metadata |
| `solve` | `problem_json` (string), `solver` ("ilp"\|"brute-force", default: "ilp"), `timeout` (int, default: 0) | Solve a problem instance or reduction bundle using ILP or brute-force, with optional timeout |

## Available Prompts

The server provides 3 prompt templates that guide the AI assistant through common workflows:

| Prompt | Arguments | Description |
|--------|-----------|-------------|
| `analyze_problem` | `problem_type` (required) | Analyze a problem type: show its definition, variants, size fields, and reduction edges |
| `reduction_walkthrough` | `source` (required), `target` (required) | End-to-end reduction walkthrough: find a path, create an instance, reduce it, and solve the result |
| `explore_graph` | *(none)* | Explore the reduction graph: list all problems, export the graph, and analyze its structure |

## Example Usage with Claude Code

Once configured, you can interact with the reduction graph naturally through conversation:

```
> What problems can MaximumIndependentSet reduce to?

> Walk me through reducing a 5-vertex MIS instance to QUBO

> Create a random graph with 8 vertices and solve the MaxCut problem on it

> Find all reduction paths from Satisfiability to SpinGlass
```

The AI assistant will automatically call the appropriate MCP tools to answer your questions, create problem instances, perform reductions, and solve problems.

## Testing with MCP Inspector

You can test the MCP server using the [MCP Inspector](https://github.com/modelcontextprotocol/inspector):

```bash
npx @modelcontextprotocol/inspector pred mcp
```

This opens a web UI where you can:

1. Browse the list of available tools and their schemas
2. Call tools with custom parameters and inspect the JSON responses
3. Browse and invoke prompt templates
4. Verify the server is working correctly before configuring your AI assistant

## Running MCP Tests

To run the MCP server test suite:

```bash
make mcp-test
```

This runs both unit tests (tool logic) and integration tests (MCP protocol-level tool calls).
