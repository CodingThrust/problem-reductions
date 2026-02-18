# CLI Tool

The `pred` command-line tool lets you explore the reduction graph, inspect problem schemas, and find reduction paths — all from your terminal.

## Installation

Install from the repository:

```bash
cargo install --git https://github.com/CodingThrust/problem-reductions problemreductions-cli
```

Or build from source:

```bash
git clone https://github.com/CodingThrust/problem-reductions
cd problem-reductions
make cli    # builds target/release/pred
```

Verify the installation:

```bash
pred --version
```

## Commands

### `pred graph list` — List all problem types

Lists all registered problem types with their short aliases.

```bash
$ pred graph list
Registered problems: 17 types, 48 reductions, 25 variant nodes

  CircuitSAT
  Factoring
  ILP
  KColoring
  KSatisfiability (3SAT, KSAT)
  MaxCut
  MaximumClique
  MaximumIndependentSet (MIS)
  MaximumMatching
  MaximumSetPacking
  MinimumDominatingSet
  MinimumSetCovering
  MinimumVertexCover (MVC)
  QUBO
  Satisfiability (SAT)
  SpinGlass
  TravelingSalesman (TSP)
```

### `pred graph show` — Inspect a problem

Show variants and reductions for a problem type. Use short aliases like `MIS` for `MaximumIndependentSet`.

```bash
$ pred graph show MIS
MaximumIndependentSet

Variants (4):
  {graph=KingsSubgraph, weight=i32}
  {graph=SimpleGraph, weight=i32}
  {graph=TriangularSubgraph, weight=i32}
  {graph=UnitDiskGraph, weight=i32}

Reduces to (10):
  -> ILP
  -> MinimumVertexCover {graph=SimpleGraph, weight=i32}
  -> QUBO {weight=f64}
  ...

Reduces from (9):
  <- Satisfiability
  <- MinimumVertexCover {graph=SimpleGraph, weight=i32}
  ...
```

### `pred graph path` — Find a reduction path

Find the cheapest chain of reductions between two problems, shown step by step:

```bash
$ pred graph path MIS QUBO
Path (1 steps): MaximumIndependentSet {graph: "SimpleGraph", weight: "i32"} → QUBO {weight: "f64"}

  Step 1: MaximumIndependentSet {graph: "SimpleGraph", weight: "i32"} → QUBO {weight: "f64"}
```

Multi-step paths are discovered automatically:

```bash
$ pred graph path Factoring SpinGlass
Path (2 steps): Factoring → CircuitSAT → SpinGlass {graph: "SimpleGraph", weight: "i32"}

  Step 1: Factoring → CircuitSAT

  Step 2: CircuitSAT → SpinGlass {graph: "SimpleGraph", weight: "i32"}
```

Use `--cost` to change the optimization strategy:

```bash
# Default: minimize number of reduction steps
pred graph path MIS QUBO --cost minimize-steps

# Minimize a specific size field (e.g., number of variables)
pred graph path MIS QUBO --cost minimize:num_variables
```

### `pred graph export` — Export the reduction graph

Export the full reduction graph as JSON:

```bash
pred graph export                     # writes to reduction_graph.json (default)
pred graph export my_graph.json       # custom output path
```

### `pred evaluate` — Evaluate a configuration

Evaluate a configuration against a problem instance from a JSON file:

```bash
$ pred evaluate problem.json --config 1,0,1,0
Valid(2)
```

The JSON file uses a wrapper format:

```json
{
  "type": "MaximumIndependentSet",
  "variant": {"graph": "SimpleGraph", "weight": "i32"},
  "data": { ... }
}
```

- `type`: Problem name (aliases like `MIS` accepted)
- `variant`: Optional, defaults to base variant
- `data`: The problem struct fields as JSON (matching serde format)

### `pred reduce` — Reduce a problem

Reduce a problem to a target type. Outputs a full reduction bundle with source, target, and path:

```bash
$ pred reduce problem.json --to QUBO
Reduced MaximumIndependentSet to QUBO (1 steps)
Bundle written with source + target + path.
```

Save the bundle as JSON for later use:

```bash
pred reduce problem.json --to QUBO --json -o bundle.json
```

The bundle contains everything needed to map solutions back:

```json
{
  "source": { "type": "MaximumIndependentSet", "variant": {...}, "data": {...} },
  "target": { "type": "QUBO", "variant": {...}, "data": {...} },
  "path": [
    {"name": "MaximumIndependentSet", "variant": {"graph": "SimpleGraph", "weight": "i32"}},
    {"name": "QUBO", "variant": {"weight": "f64"}}
  ]
}
```

### `pred schema` — Show problem schema

Display the fields and types for a problem:

```bash
$ pred schema MIS
MaximumIndependentSet
  Find maximum weight independent set in a graph

Fields:
  graph (G) -- The underlying graph G=(V,E)
  weights (Vec<W>) -- Vertex weights w: V -> R
```

## JSON Output

All commands support `--json` for machine-readable output, optionally written to a file:

```bash
pred graph list --json                     # print JSON to stdout
pred graph list --json -o problems.json    # write to file
pred graph path MIS QUBO --json            # path result as JSON
```

## Problem Name Aliases

You can use short aliases instead of full problem names (shown in `pred graph list`):

| Alias | Full Name |
|-------|-----------|
| `MIS` | `MaximumIndependentSet` |
| `MVC` | `MinimumVertexCover` |
| `SAT` | `Satisfiability` |
| `3SAT` / `KSAT` | `KSatisfiability` |
| `TSP` | `TravelingSalesman` |

You can also specify variants with a slash: `MIS/UnitDiskGraph`, `SpinGlass/SimpleGraph`.
