# CLI Tool Design: `pred`

## Overview

A command-line tool for researchers and students to explore NP-hard problem reductions and solve problem instances without writing Rust code. Implemented as a separate workspace crate (`problemreductions-cli`), binary name `pred`.

## Audience

Researchers and students studying NP-hard reductions who want to explore and visualize without writing any Rust code.

## Command Structure

Subcommand-based CLI with two top-level groups: `graph` (exploration) and solve/reduce/evaluate (computation).

### Graph Exploration

```
pred graph list                                    # List all registered problems
pred graph show <Problem>                          # Show problem details, variants, reductions
pred graph show MIS --variants                     # List all variants
pred graph path <Source> <Target>                   # Find cheapest reduction path
pred graph path MIS QUBO --cost minimize:num_vars   # Custom cost function
pred graph export [--output path]                   # Export reduction_graph.json
```

### Computation

```
pred solve <input.json> --via <Target>              # Reduce + solve + map solution back
pred solve --problem MIS --edges 0-1,1-2 --via QUBO # Inline input
pred reduce <input.json> --to <Target>              # Reduce only, output target as JSON
pred evaluate <input.json> --config 1,0,1           # Evaluate a configuration
pred schema <Problem>                               # Show JSON schema for a problem type
```

### Global Flags

- `--json` — structured JSON output, saved to file (default filename derived from command)
- `--output <path>` — custom output file path (used with `--json`)
- `--help` / `-h` — per-command help

## Problem Name Resolution

Case-insensitive matching with common aliases:

| Input | Resolves to |
|-------|-------------|
| `MIS`, `mis` | `MaximumIndependentSet` |
| `MVC` | `MinimumVertexCover` |
| `SAT` | `Satisfiability` |
| `3SAT` | `KSatisfiability` (K=3) |
| `QUBO` | `QUBO` |
| `MaxCut` | `MaxCut` |

Unambiguous prefix matching: `MaximumI` → `MaximumIndependentSet`, but `Maximum` is rejected (ambiguous).

## Variant Syntax

Slash-based positional notation after the problem name. Order follows `Problem::variant()` key order. Partial specification fills from the left; no skipping.

```
MIS                         →  defaults (SimpleGraph, One)
MIS/UnitDiskGraph           →  UnitDiskGraph, default weight
MIS/SimpleGraph/f64         →  must spell out graph to set weight
KColoring/K3                →  SimpleGraph, K=3
3SAT                        →  alias for KSatisfiability/K3
QUBO                        →  no variants
```

## Input Formats

### JSON Files

Reuses the library's existing serde serialization:

```json
{
  "problem": "MaximumIndependentSet",
  "graph": {"edges": [[0,1], [1,2], [2,0]], "num_vertices": 3},
  "weights": [1, 1, 1]
}
```

### Inline Arguments

For simple cases without a JSON file:

```
pred solve --problem MIS --edges 0-1,1-2,2-0 --weights 1,1,1 --via QUBO
```

## Output

- **Human-readable (default):** plain text to stdout
- **`--json`:** structured JSON saved to file (default name derived from command, e.g., `pred_path_MIS_QUBO.json`)
- **`--json --output custom.json`:** custom output path
- **Errors:** always to stderr
- **Exit codes:** non-zero on any error

## Architecture

### Crate Layout

Separate workspace crate: `problemreductions-cli/`

```
src/
  main.rs              # Cli::parse(), dispatch to commands
  cli.rs               # Clap derive structs (Cli, Commands, GraphCommands)
  commands/
    graph.rs           # list, show, path, export
    solve.rs           # reduce + solve + extract solution
    reduce.rs          # reduce only, output target problem
    evaluate.rs        # evaluate a config
    schema.rs          # show JSON schema for a problem type
  output.rs            # OutputMode enum, write_json_file(), print_human()
  problem_name.rs      # Alias resolution + variant parsing (slash notation)
```

### Dependencies

- `clap` (derive) — argument parsing
- `anyhow` — error handling
- `serde_json` — JSON I/O
- `problemreductions` — the library (all features)

### Dynamic Dispatch

- **Graph commands:** use `ReductionGraph` directly — already works with string names
- **Solve/reduce/evaluate:** dispatch table — a `match` over known problem names that constructs concrete types from JSON. ~20 match arms, one per problem type.

### Error Handling

`anyhow::Result` throughout, with `.context()` for actionable error messages. Non-zero exit code on any error.

## V1 Scope

### In scope

- `pred graph list`
- `pred graph show <Problem>` (with `--variants`)
- `pred graph path <Source> <Target>` (with `--cost`)
- `pred graph export`
- `pred solve` (JSON file and inline input, brute-force solver)
- `pred reduce` (reduce only)
- `pred evaluate`
- `pred schema`
- `--json` output to file

### Out of scope (v2+)

See GitHub issue for future plans.
