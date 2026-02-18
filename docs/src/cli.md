# CLI Tool

The `pred` command-line tool lets you explore the reduction graph, create problem instances, solve problems, and perform reductions — all from your terminal.

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

### ILP Backend

The default ILP backend is HiGHS. To use a different backend:

```bash
cargo install problemreductions-cli --features coin-cbc
cargo install problemreductions-cli --features scip
cargo install problemreductions-cli --no-default-features --features clarabel
```

Available backends: `highs` (default), `coin-cbc`, `clarabel`, `scip`, `lpsolve`, `microlp`.

## Quick Start

```bash
# Create a Maximum Independent Set problem
pred create MIS --edges 0-1,1-2,2-3 -o problem.json

# Solve it (auto-reduces to ILP)
pred solve problem.json

# Or solve with brute-force
pred solve problem.json --solver brute-force

# Evaluate a specific configuration
pred evaluate problem.json --config 1,0,1,0

# Reduce to another problem type
pred reduce problem.json --to QUBO -o reduced.json

# Solve the reduced problem (maps solution back to source)
pred solve reduced.json
```

## Commands

### `pred list` — List all problem types

Lists all registered problem types with their short aliases.

```bash
$ pred list
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

### `pred show` — Inspect a problem

Show variants and reductions for a problem type. Use short aliases like `MIS` for `MaximumIndependentSet`.

```bash
$ pred show MIS
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

### `pred path` — Find a reduction path

Find the cheapest chain of reductions between two problems:

```bash
$ pred path MIS QUBO
Path (1 steps): MaximumIndependentSet {graph: "SimpleGraph", weight: "i32"} → QUBO {weight: "f64"}

  Step 1: MaximumIndependentSet {graph: "SimpleGraph", weight: "i32"} → QUBO {weight: "f64"}
```

Multi-step paths are discovered automatically:

```bash
$ pred path Factoring SpinGlass
Path (2 steps): Factoring → CircuitSAT → SpinGlass {graph: "SimpleGraph", weight: "i32"}

  Step 1: Factoring → CircuitSAT
  Step 2: CircuitSAT → SpinGlass {graph: "SimpleGraph", weight: "i32"}
```

Show all paths or save for later use with `pred reduce --via`:

```bash
pred path MIS QUBO --all                    # all paths
pred path MIS QUBO -o path.json             # save path for `pred reduce --via`
pred path MIS QUBO --all -o paths/          # save all paths to a folder
```

Use `--cost` to change the optimization strategy:

```bash
pred path MIS QUBO --cost minimize-steps           # default
pred path MIS QUBO --cost minimize:num_variables   # minimize a size field
```

### `pred export-graph` — Export the reduction graph

Export the full reduction graph as JSON:

```bash
pred export-graph reduction_graph.json
```

### `pred create` — Create a problem instance

Construct a problem instance from CLI arguments and save as JSON:

```bash
pred create MIS --edges 0-1,1-2,2-3 -o problem.json
pred create MIS --edges 0-1,1-2,2-3 --weights 2,1,3,1 -o problem.json
pred create SAT --num-vars 3 --clauses "1,2;-1,3" -o sat.json
pred create QUBO --matrix "1,0.5;0.5,2" -o qubo.json
pred create KColoring --k 3 --edges 0-1,1-2,2-0 -o kcol.json
```

The output file uses a standard wrapper format:

```json
{
  "type": "MaximumIndependentSet",
  "variant": {"graph": "SimpleGraph", "weight": "i32"},
  "data": { ... }
}
```

### `pred evaluate` — Evaluate a configuration

Evaluate a configuration against a problem instance:

```bash
$ pred evaluate problem.json --config 1,0,1,0
Valid(2)
```

### `pred reduce` — Reduce a problem

Reduce a problem to a target type. Outputs a reduction bundle containing source, target, and path:

```bash
$ pred reduce problem.json --to QUBO
Reduced MaximumIndependentSet to QUBO (1 steps)
```

Save the bundle for later solving:

```bash
pred reduce problem.json --to QUBO -o reduced.json
```

Use a specific reduction path (from `pred path -o`):

```bash
pred reduce problem.json --to QUBO --via path.json -o reduced.json
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

### `pred solve` — Solve a problem

Solve a problem instance using ILP (default) or brute-force:

```bash
pred solve problem.json                         # ILP solver (default)
pred solve problem.json --solver brute-force    # brute-force solver
```

When the problem is not ILP, the solver automatically reduces it to ILP, solves, and maps the solution back:

```bash
$ pred solve problem.json
Problem: MaximumIndependentSet (reduced to ILP)
Solver: ilp
Solution: [1, 0, 0, 1]
Evaluation: Valid(2)
```

Solve a reduction bundle (from `pred reduce`):

```bash
$ pred solve reduced.json
Source: MaximumIndependentSet
Target: QUBO (solved with ilp)
Target solution: [0, 1, 0, 1]
Target evaluation: Valid(-2.0)
Source solution: [0, 1, 0, 1]
Source evaluation: Valid(2)
```

## JSON Output

All commands support `-o` to write JSON output to a file:

```bash
pred list -o problems.json
pred path MIS QUBO -o path.json
pred solve problem.json -o solution.json
```

## Problem Name Aliases

You can use short aliases instead of full problem names (shown in `pred list`):

| Alias | Full Name |
|-------|-----------|
| `MIS` | `MaximumIndependentSet` |
| `MVC` | `MinimumVertexCover` |
| `SAT` | `Satisfiability` |
| `3SAT` / `KSAT` | `KSatisfiability` |
| `TSP` | `TravelingSalesman` |

You can also specify variants with a slash: `MIS/UnitDiskGraph`, `SpinGlass/SimpleGraph`.
