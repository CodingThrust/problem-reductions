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

# Reduce to another problem type and solve via brute-force
pred reduce problem.json --to QUBO -o reduced.json
pred solve reduced.json --solver brute-force
```

## Commands

### `pred list` — List all problem types

Lists all registered problem types with their short aliases.

```bash
$ pred list
Registered problems: 17 types, 48 reductions, 25 variant nodes

  Problem                Aliases     Variants  Reduces to
  ─────────────────────  ──────────  ────────  ──────────
  CircuitSAT                                1           1
  Factoring                                 1           2
  ILP                                       1           1
  KColoring                                 2           3
  KSatisfiability        3SAT, KSAT         3           7
  MaxCut                                    1           1
  MaximumClique                             1           1
  MaximumIndependentSet  MIS                4          10
  MaximumMatching                           1           2
  MaximumSetPacking                         2           4
  MinimumDominatingSet                      1           1
  MinimumSetCovering                        1           1
  MinimumVertexCover     MVC                1           4
  QUBO                                      1           1
  Satisfiability         SAT                1           5
  SpinGlass                                 2           3
  TravelingSalesman      TSP                1           1

Use `pred show <problem>` to see variants, reductions, and fields.
```

### `pred show` — Inspect a problem

Show variants, fields, size fields, and reductions for a problem type. Use short aliases like `MIS` for `MaximumIndependentSet`.

```bash
$ pred show MIS
MaximumIndependentSet
  Find maximum weight independent set in a graph

Variants (4):
  {graph=SimpleGraph, weight=i32}
  {graph=UnitDiskGraph, weight=i32}
  {graph=KingsSubgraph, weight=i32}
  {graph=TriangularSubgraph, weight=i32}

Fields (2):
  graph (G) -- The underlying graph G=(V,E)
  weights (Vec<W>) -- Vertex weights w: V -> R

Size fields (2):
  num_vertices
  num_edges

Reduces to (10):
  MaximumIndependentSet {graph=SimpleGraph, weight=i32} → MinimumVertexCover ...
  MaximumIndependentSet {graph=SimpleGraph, weight=i32} → ILP (default)
  MaximumIndependentSet {graph=SimpleGraph, weight=i32} → QUBO {weight=f64}
  ...

Reduces from (9):
  MinimumVertexCover {graph=SimpleGraph, weight=i32} → MaximumIndependentSet ...
  Satisfiability (default) → MaximumIndependentSet {graph=SimpleGraph, weight=i32}
  ...
```

Explore neighbors within k hops in the reduction graph:

```bash
$ pred show MIS --hops 2
MaximumIndependentSet — 2-hop neighbors (outgoing)

MaximumIndependentSet
├── MaximumIndependentSet
└── MaximumIndependentSet
    ├── ILP
    ├── MaximumIndependentSet
    ├── MaximumSetPacking
    ├── MinimumVertexCover
    └── QUBO

5 reachable problems in 2 hops
```

Use `--direction` to control traversal direction:

```bash
pred show MIS --hops 2 --direction out    # outgoing neighbors (default)
pred show QUBO --hops 1 --direction in    # incoming neighbors
pred show MIS --hops 1 --direction both   # both directions
```

### `pred path` — Find a reduction path

Find the cheapest chain of reductions between two problems:

```bash
$ pred path MIS QUBO
Path (1 steps): MaximumIndependentSet ... → QUBO {weight: "f64"}

  Step 1: MaximumIndependentSet {graph: "SimpleGraph", weight: "i32"} → QUBO {weight: "f64"}
    num_vars = num_vertices
```

Multi-step paths are discovered automatically:

```bash
$ pred path Factoring SpinGlass
Path (2 steps): Factoring → CircuitSAT → SpinGlass {graph: "SimpleGraph", weight: "i32"}

  Step 1: Factoring → CircuitSAT
    num_variables = num_bits_first * num_bits_second
    num_assignments = num_bits_first * num_bits_second

  Step 2: CircuitSAT → SpinGlass {graph: "SimpleGraph", weight: "i32"}
    num_spins = num_assignments
    num_interactions = num_assignments
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

Use `pred show <problem>` to see which size fields are available.

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
pred create SpinGlass --edges 0-1,1-2 -o sg.json
pred create MaxCut --edges 0-1,1-2,2-0 -o maxcut.json
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
pred reduce problem.json --to QUBO -o reduced.json
```

Use a specific reduction path (from `pred path -o`):

```bash
pred reduce problem.json --to QUBO --via path.json -o reduced.json
```

Without `-o`, the bundle JSON is printed to stdout:

```bash
pred reduce problem.json --to QUBO
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
$ pred solve reduced.json --solver brute-force
Source: MaximumIndependentSet
Target: QUBO (solved with brute-force)
Target solution: [0, 1, 0, 1]
Target evaluation: Valid(-2.0)
Source solution: [0, 1, 0, 1]
Source evaluation: Valid(2)
```

> **Note:** The ILP solver requires a reduction path from the target problem to ILP.
> Some problems (e.g., QUBO, SpinGlass, MaxCut, CircuitSAT) do not have this path yet.
> Use `--solver brute-force` for these, or reduce to a problem that supports ILP first.

## Shell Completions

Enable tab completion by adding one line to your shell config:

```bash
# bash (~/.bashrc)
eval "$(pred completions bash)"

# zsh (~/.zshrc)
eval "$(pred completions zsh)"

# fish (~/.config/fish/config.fish)
pred completions fish | source
```

If the shell argument is omitted, `pred completions` auto-detects your current shell.

## JSON Output

All commands support `-o` to write JSON output to a file:

```bash
pred list -o problems.json
pred show MIS -o mis.json
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
