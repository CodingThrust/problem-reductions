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

```bash
$ pred graph list
Registered problems: 17 types, 48 reductions, 25 variant nodes

  CircuitSAT
  Factoring
  ILP
  KColoring
  KSatisfiability
  MaxCut
  MaximumClique
  MaximumIndependentSet
  ...
```

### `pred graph show` — Inspect a problem

Show the reductions available for a problem type. Use short aliases like `MIS` for `MaximumIndependentSet`.

```bash
$ pred graph show MIS
MaximumIndependentSet

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

Add `--variants` to see all registered graph/weight variants:

```bash
$ pred graph show MIS --variants
MaximumIndependentSet

Variants (4):
  {graph=KingsSubgraph, weight=i32}
  {graph=SimpleGraph, weight=i32}
  {graph=TriangularSubgraph, weight=i32}
  {graph=UnitDiskGraph, weight=i32}
  ...
```

### `pred graph path` — Find a reduction path

Find the cheapest chain of reductions between two problems:

```bash
$ pred graph path MIS QUBO
Path (1 steps): MaximumIndependentSet {graph: "SimpleGraph", weight: "i32"} → QUBO {weight: "f64"}
```

Multi-step paths are discovered automatically:

```bash
$ pred graph path Factoring SpinGlass
Path (2 steps): Factoring → CircuitSAT → SpinGlass {graph: "SimpleGraph", weight: "i32"}
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
pred graph export                          # writes to reduction_graph.json
pred graph export --output my_graph.json   # custom output path
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

You can use short aliases instead of full problem names:

| Alias | Full Name |
|-------|-----------|
| `MIS` | `MaximumIndependentSet` |
| `MVC` | `MinimumVertexCover` |
| `SAT` | `Satisfiability` |

You can also specify variants with a slash: `MIS/UnitDiskGraph`, `SpinGlass/SimpleGraph`.
