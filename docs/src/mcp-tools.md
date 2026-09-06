# MCP tool reference

[Connect the server](mcp.md), then use its advertised tool schemas as the runtime contract. Tools return JSON strings; pass the returned instance or bundle as `problem_json` to the next operation.

## Graph queries

| Tool | Parameters | Description |
|------|-----------|-------------|
| `list_problems` | *(none)* | List all registered problem types with aliases, variant counts, and reduction counts |
| `show_problem` | `problem` (string) | Show details for a problem type: variants, size fields, schema, and incoming/outgoing reductions |
| `neighbors` | `problem` (string), `hops` (int, default: 1), `direction` ("out"\|"in"\|"both", default: "out") | Find neighboring problems reachable via reduction edges within a given hop distance |
| `find_path` | `source` (string), `target` (string), `cost` (string, default: "minimize-steps"), `all` (bool, default: false), `max_paths` (int, default: 20) | Find a reduction path between two problems, optionally minimizing a size field or returning all paths |
| `export_graph` | *(none)* | Export the full reduction graph as JSON (nodes, edges, overheads) |

## Instances

| Tool | Parameters | Description |
|------|-----------|-------------|
| `create_problem` | `problem_type` (string), `params` (JSON object) | Create a problem instance from parameters and return its JSON representation. Supports graph problems, SAT, QUBO, SpinGlass, KColoring, Factoring, and random graph generation |
| `inspect_problem` | `problem_json` (string) | Inspect a problem JSON or reduction bundle: returns type, size metrics, available solvers, and reduction targets |
| `evaluate` | `problem_json` (string), `config` (array of int) | Evaluate a configuration against a problem instance and return the objective value or feasibility |
| `reduce` | `problem_json` (string), `target` (string) | Reduce a problem instance to a target type, returning a reduction bundle with the transformed instance and path metadata |
| `solve` | `problem_json` (string), `solver` ("ilp"\|"brute-force"\|"customized", default: "ilp"), `timeout` (int, default: 0) | Solve a problem instance or reduction bundle using the chosen backend, with optional timeout |

## Prompt templates

The server also advertises `what_is`, `model_my_problem`, `compare`, `reduce`, `solve`, `find_reduction`, and `overview`. Inspect each prompt’s declared arguments in your MCP client.

Creation supports selected models and uses MCP parameter names such as `edges`; CLI creation flags are a separate interface. For models outside MCP creation support, create JSON with the CLI and pass it to the instance tools.

Next: [example session](mcp-walkthrough.md).
