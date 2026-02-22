# MCP Server

The [Model Context Protocol (MCP)](https://modelcontextprotocol.io/) is an open standard that allows AI assistants to interact with external tools and data sources. The `pred` CLI includes a built-in MCP server that exposes the full reduction graph, problem creation, solving, and reduction capabilities to any MCP-compatible AI assistant (such as Claude Code, Cursor, Windsurf, or OpenCode).

## Walkthrough

A complete session showing how an AI assistant uses MCP tools behind the scenes to answer your questions.

**You:** Create a weighted MIS problem on a 4-vertex graph with edges 0-1, 1-2, 2-3, 3-0, 0-2 and weights [1, 2, 3, 4], explain the result.

> **Assistant:** I created a Maximum Independent Set instance on a SimpleGraph with 4 vertices, 5 edges, and integer weights [1, 2, 3, 4]. Each vertex has a weight, and the goal is to find a subset of non-adjacent vertices that maximizes total weight.

**You:** What can I do with this problem? Explain the result.

> **Assistant:** This instance has 4 binary variables (one per vertex). It can be reduced to 5 other problem types: ILP, MaximumSetPacking, MinimumVertexCover, QUBO, and other MIS variants. You can solve it directly using either an ILP solver or brute-force search.

**You:** Check if selecting vertices 1 and 3 is a valid solution, explain the result.

> **Assistant:** Yes! Selecting vertices 1 and 3 (configuration [0,1,0,1]) is a valid independent set — these two vertices are not adjacent. The total weight is 2 + 4 = 6.

**You:** Reduce this to QUBO, explain the result.

> **Assistant:** Done. The MIS instance was transformed into a 4x4 QUBO matrix:
>
> ```
> [-1.0, 11.0, 11.0, 11.0]
> [ 0.0, -2.0, 11.0,  0.0]
> [ 0.0,  0.0, -3.0, 11.0]
> [ 0.0,  0.0,  0.0, -4.0]
> ```
>
> The diagonal entries (-1, -2, -3, -4) encode vertex weights negated for minimization. The off-diagonal entries (11.0) are large penalties on edges — selecting two adjacent vertices would incur a penalty that outweighs any benefit. The reduction path is: MaximumIndependentSet → QUBO.

**You:** Now solve it through the reduction, explain the result.

> **Assistant:** The brute-force solver found the optimal solution by solving the QUBO, then mapping the result back to the original MIS:
>
> - **QUBO solution:** [0,1,0,1] with objective -6.0 (minimization)
> - **MIS solution:** vertices {1, 3} with weight **6** (maximization)
>
> The QUBO objective of -6.0 corresponds to the MIS maximum of 6 — the reduction correctly inverts the sign. This confirms that {1, 3} is the optimal independent set.

## Installation

Install via cargo:

```bash
cargo install problemreductions-cli
```

Or build from source:

```bash
git clone https://github.com/CodingThrust/problem-reductions
cd problem-reductions
make cli    # builds and installs target/release/pred
```

## Configuration

### Claude Code

```bash
# Project-scoped (current project only)
claude mcp add --scope project problemreductions -- pred mcp

# User-scoped (all projects)
claude mcp add --scope user problemreductions -- pred mcp
```

### Cursor / Windsurf

Add to the appropriate config file:
- **Cursor:** `.cursor/mcp.json` (project) or `~/.cursor/mcp.json` (global)
- **Windsurf:** `~/.codeium/windsurf/mcp_config.json`

```json
{ "mcpServers": { "problemreductions": { "command": "pred", "args": ["mcp"] } } }
```

### OpenCode

Add to `opencode.json` in your project root:

```json
{ "mcp": { "problemreductions": { "type": "local", "command": ["pred", "mcp"] } } }
```

### Generic MCP Client

Any MCP client that supports the stdio transport can connect by running `pred mcp`. The server communicates over stdin/stdout using the MCP JSON-RPC protocol.

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

The server provides 7 task-oriented prompt templates:

| Prompt | Arguments | Description |
|--------|-----------|-------------|
| `what_is` | `problem_type` (required) | Explain a problem type |
| `model_my_problem` | `description` (required) | Map a real-world problem to an NP-hard type |
| `compare` | `problem_a` (required), `problem_b` (required) | Compare two problem types |
| `reduce` | `source` (required), `target` (required) | Step-by-step reduction walkthrough |
| `solve` | `problem_type` (required), `params` (required) | Create and solve a problem instance |
| `find_reduction` | `source` (required), `target` (required) | Find the best reduction path between two problems |
| `overview` | *(none)* | Explore the full landscape of NP-hard problems |
