# CLI Tool

The `pred` command-line tool lets you explore the reduction graph, create problem instances, solve problems, and perform reductions — all from your terminal.

## Installation

Install from crates.io:

```bash
cargo install problemreductions-cli
```

Or build from source:

```bash
git clone https://github.com/CodingThrust/problem-reductions
cd problem-reductions
cargo build -p problemreductions-cli --release   # builds target/release/pred
cargo install --path problemreductions-cli       # optional: installs `pred` to ~/.cargo/bin
```

Verify the installation:

```bash
pred --version
```

For a workspace-local run without installing globally, use:

```bash
cargo run -p problemreductions-cli --bin pred -- --version
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
pred create MIS --graph 0-1,1-2,2-3 -o problem.json

# Create a weighted instance (variant auto-upgrades to i32)
pred create MIS --graph 0-1,1-2,2-3 --weights 3,1,2,1 -o weighted.json

# Create a Steiner Tree instance
pred create SteinerTree --graph 0-1,0-3,1-2,1-3,2-3,2-4,3-4 --edge-weights 2,5,2,1,5,6,1 --terminals 0,2,4 -o steiner.json

# Create a Length-Bounded Disjoint Paths instance
pred create LengthBoundedDisjointPaths --graph 0-1,1-6,0-2,2-3,3-6,0-4,4-5,5-6 --source 0 --sink 6 --bound 4 -o lbdp.json

# Create a Consecutive Block Minimization instance (alias: CBM)
pred create CBM --matrix '[[true,false,true],[false,true,true]]' --bound 2 -o cbm.json

# CBM currently needs the brute-force solver
pred solve cbm.json --solver brute-force

# Or start from a canonical model example
pred create --example MIS/SimpleGraph/i32 -o example.json

# Or from a canonical rule example
pred create --example MVC/SimpleGraph/i32 --to MIS/SimpleGraph/i32 -o example.json

# Inspect what's inside a problem file
pred inspect problem.json

# Inspect the new path problem
pred inspect lbdp.json

# Solve it (auto-reduces to ILP)
pred solve problem.json

# Or solve with brute-force
pred solve problem.json --solver brute-force

# LengthBoundedDisjointPaths currently needs brute-force
pred solve lbdp.json --solver brute-force

# Evaluate a specific configuration (shows the aggregate value, e.g. Max(2) or Min(None))
pred evaluate problem.json --config 1,0,1,0

# Reduce to another problem type and solve via brute-force
pred reduce problem.json --to QUBO -o reduced.json
pred solve reduced.json --solver brute-force

# Pipe commands together (use - to read from stdin)
pred create MIS --graph 0-1,1-2,2-3 | pred solve -   # when an ILP reduction path exists
pred create StringToStringCorrection --source-string "0,1,2,3,1,0" --target-string "0,1,3,2,1" --bound 2 | pred solve - --solver brute-force
pred create MIS --graph 0-1,1-2,2-3 | pred reduce - --to QUBO | pred solve -
```

> **Note:** When you provide `--weights` with non-unit values (e.g., `3,1,2,1`), the variant is
> automatically upgraded from the default unit-weight (`One`) to `i32`. You can also specify the
> weighted variant explicitly: `pred create MIS/SimpleGraph/i32 --graph 0-1 --weights 3,1`.

## Global Flags

| Flag | Description |
|------|-------------|
| `-o, --output <FILE>` | Save JSON output to a file |
| `--json` | Output JSON to stdout instead of human-readable text |
| `-q, --quiet` | Suppress informational messages on stderr |

## Commands

### `pred list` — List all problem types

Lists all registered problem types with their short aliases.

```text
{{#include generated/pred-list.txt}}
```

### `pred show` — Inspect a problem

Show fields, size fields, and reductions for a problem's default variant. Use short aliases like `MIS` for `MaximumIndependentSet`. Use `pred to` or `pred from` for variant-level neighborhood exploration.

```text
{{#include generated/pred-show-mis.txt}}
```

### `pred to` — Explore incoming neighbors

Explore which problems can reduce **to** the given problem within k hops:

```text
{{#include generated/pred-to-mis.txt}}
```

### `pred from` — Explore outgoing neighbors

Explore which problems the given problem can reduce to, starting **from** it:

```text
{{#include generated/pred-from-qubo.txt}}
```

### `pred path` — Find a reduction path

Find the cheapest chain of reductions between two problems:

```text
{{#include generated/pred-path-mis-qubo.txt}}
```

Multi-step paths are discovered automatically:

```text
{{#include generated/pred-path-factoring-spinglass.txt}}
```

Show all paths or save for later use with `pred reduce --via`:

```bash
pred path MIS QUBO --all                    # all paths (up to 20)
pred path MIS QUBO --all --max-paths 50     # increase limit
pred path MIS QUBO -o path.json             # save path for `pred reduce --via`
pred path MIS QUBO --all -o paths/          # save all paths to a folder
```

When using `--all`, the output is capped at `--max-paths` (default: 20). If more paths exist, the output indicates truncation.

Use `--cost` to change the optimization strategy:

```bash
pred path MIS QUBO --cost minimize-steps           # default
pred path MIS QUBO --cost minimize:num_variables   # minimize a size field
```

Use `pred show <problem>` to see which size fields are available.

### `pred export-graph` — Export the reduction graph

Export the full reduction graph as JSON:

```bash
pred export-graph                           # print to stdout
pred export-graph -o reduction_graph.json   # save to file
```

### `pred create` — Create a problem instance

Construct a problem instance from CLI arguments and save as JSON:

```bash
pred create --example MIS/SimpleGraph/i32 -o model.json
pred create --example MVC/SimpleGraph/i32 --to MIS/SimpleGraph/i32 -o problem.json
pred create --example MVC/SimpleGraph/i32 --to MIS/SimpleGraph/i32 --example-side target -o target.json
pred create MIS --graph 0-1,1-2,2-3 -o problem.json
pred create MIS --graph 0-1,1-2,2-3 --weights 2,1,3,1 -o problem.json
pred create SAT --num-vars 3 --clauses "1,2;-1,3" -o sat.json
pred create QUBO --matrix "1,0.5;0.5,2" -o qubo.json
pred create CBM --matrix '[[true,false,true],[false,true,true]]' --bound 2 -o cbm.json
pred create KColoring --k 3 --graph 0-1,1-2,2-0 -o kcol.json
pred create KthBestSpanningTree --graph 0-1,0-2,1-2 --edge-weights 2,3,1 --k 1 --bound 3 -o kth.json
pred create SpinGlass --graph 0-1,1-2 -o sg.json
pred create MaxCut --graph 0-1,1-2,2-0 -o maxcut.json
pred create MinMaxMulticenter --graph 0-1,1-2,2-3 --weights 1,1,1,1 --edge-weights 1,1,1 --k 2 -o pcenter.json
pred create ShortestWeightConstrainedPath --graph 0-1,0-2,1-3,2-3,2-4,3-5,4-5,1-4 --edge-lengths 2,4,3,1,5,4,2,6 --edge-weights 5,1,2,3,2,3,1,1 --source-vertex 0 --target-vertex 5 --weight-bound 8 -o swcp.json
pred create RectilinearPictureCompression --matrix "1,1,0,0;1,1,0,0;0,0,1,1;0,0,1,1" --k 2 -o rpc.json
pred solve rpc.json --solver brute-force
pred create MinimumMultiwayCut --graph 0-1,1-2,2-3,3-0 --terminals 0,2 --edge-weights 3,1,2,4 -o mmc.json
pred create SteinerTree --graph 0-1,0-3,1-2,1-3,2-3,2-4,3-4 --edge-weights 2,5,2,1,5,6,1 --terminals 0,2,4 -o steiner.json
pred create UndirectedTwoCommodityIntegralFlow --graph 0-2,1-2,2-3 --capacities 1,1,2 --source-1 0 --sink-1 3 --source-2 1 --sink-2 3 --requirement-1 1 --requirement-2 1 -o utcif.json
pred create LengthBoundedDisjointPaths --graph 0-1,1-6,0-2,2-3,3-6,0-4,4-5,5-6 --source 0 --sink 6 --bound 4 -o lbdp.json
pred create Factoring --target 15 --bits-m 4 --bits-n 4 -o factoring.json
pred create Factoring --target 21 --bits-m 3 --bits-n 3 -o factoring2.json
pred create X3C --universe 9 --sets "0,1,2;0,2,4;3,4,5;3,5,7;6,7,8;1,4,6;2,5,8" -o x3c.json
pred create MinimumCardinalityKey --num-attributes 6 --dependencies "0,1>2;0,2>3;1,3>4;2,4>5" -o mck.json
pred create MinimumTardinessSequencing --n 5 --deadlines 5,5,5,3,3 --precedence-pairs "0>3,1>3,1>4,2>4" -o mts.json
pred create SchedulingWithIndividualDeadlines --n 7 --deadlines 2,1,2,2,3,3,2 --num-processors 3 --precedence-pairs "0>3,1>3,1>4,2>4,2>5" -o swid.json
pred solve swid.json --solver brute-force
pred create SequencingToMinimizeWeightedCompletionTime --lengths 2,1,3,1,2 --weights 3,5,1,4,2 --precedence-pairs "0>2,1>4" -o stmwct.json
pred create StringToStringCorrection --source-string "0,1,2,3,1,0" --target-string "0,1,3,2,1" --bound 2 | pred solve - --solver brute-force
pred create StrongConnectivityAugmentation --arcs "0>1,1>2,2>0,3>4,4>3,2>3,4>5,5>3" --candidate-arcs "3>0:5,3>1:3,3>2:4,4>0:6,4>1:2,4>2:7,5>0:4,5>1:3,5>2:1,0>3:8,0>4:3,0>5:2,1>3:6,1>4:4,1>5:5,2>4:3,2>5:7,1>0:2" --bound 1 -o sca.json
```

For `LengthBoundedDisjointPaths`, the CLI flag `--bound` maps to the JSON field
`max_length`.

For `ConsecutiveBlockMinimization`, the `--matrix` flag expects a JSON 2D bool array such as
`'[[true,false,true],[false,true,true]]'`. The example above shows the accepted shape, and solving
CBM instances currently requires `--solver brute-force`.

For problem-specific create help, run `pred create <PROBLEM>` with no additional flags.
The generic `pred create --help` output lists all flags across all problem types.

Canonical examples are useful when you want a known-good instance from the paper/example database.
For model examples, `pred create --example <PROBLEM_SPEC>` emits the canonical instance for that
graph node.
For rule examples, `pred create --example <SOURCE_SPEC> --to <TARGET_SPEC>` emits the source
instance by default; use `--example-side target` to emit the reduction target instance instead.

Generate random instances for graph-based problems:

```bash
pred create MIS --random --num-vertices 10 --edge-prob 0.3
pred create MIS --random --num-vertices 100 --seed 42 -o big.json
pred create MaxCut --random --num-vertices 20 --edge-prob 0.5 -o maxcut.json
```

Without `-o`, the problem JSON is printed to stdout, which can be piped to other commands:

```bash
pred create MIS --graph 0-1,1-2,2-3 | pred solve -   # when an ILP reduction path exists
pred create StringToStringCorrection --source-string "0,1,2,3,1,0" --target-string "0,1,3,2,1" --bound 2 | pred solve - --solver brute-force
pred create MIS --random --num-vertices 10 | pred inspect -
```

The output file uses a standard wrapper format:

```json
{
  "type": "MaximumIndependentSet",
  "variant": {"graph": "SimpleGraph", "weight": "i32"},
  "data": { ... }
}
```

#### Example: Bounded Component Spanning Forest

`BoundedComponentSpanningForest` uses one component label per vertex in the
evaluation config. If the graph has `n` vertices and limit `k`, then
`--config` expects `n` comma-separated integers in `0..k-1`.

```bash
pred create BoundedComponentSpanningForest \
  --graph 0-1,1-2,2-3,3-4,4-5,5-6,6-7,0-7,1-5,2-6 \
  --weights 2,3,1,2,3,1,2,1 \
  --k 3 \
  --bound 6 \
  -o bcsf.json

pred evaluate bcsf.json --config 0,0,1,1,1,2,2,0
pred solve bcsf.json --solver brute-force
```

The brute-force solver is required here because this model does not yet have an
ILP reduction path.

### `pred evaluate` — Evaluate a configuration

Evaluate a configuration against a problem instance:

```text
{{#include generated/pred-evaluate.txt}}
```

Stdin is supported with `-`:

```bash
pred create MIS --graph 0-1,1-2,2-3 | pred evaluate - --config 1,0,1,0
```

### `pred inspect` — Inspect a problem file

Show a summary of what's inside a problem JSON or reduction bundle:

```bash
$ pred inspect problem.json
Type: MaximumIndependentSet {graph=SimpleGraph, weight=i32}
Size: 5 vertices, 5 edges
```

Works with reduction bundles and stdin:

```bash
pred inspect bundle.json
pred create MIS --graph 0-1,1-2 | pred inspect -
```

### `pred reduce` — Reduce a problem

Reduce a problem to a target type. Outputs a reduction bundle containing source, target, and path:

```bash
pred reduce problem.json --to QUBO -o reduced.json
```

Use a specific reduction path (from `pred path -o`). The target is inferred from the path file, so `--to` is not needed:

```bash
pred reduce problem.json --via path.json -o reduced.json
```

Stdin is supported with `-`:

```bash
pred create MIS --graph 0-1,1-2,2-3 | pred reduce - --to QUBO
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

Solve a problem instance using ILP (default), brute-force, or the customized solver:

```bash
pred solve problem.json                         # ILP solver (default)
pred solve problem.json --solver brute-force    # brute-force solver
pred solve problem.json --solver customized     # structure-exploiting exact solver
pred solve problem.json --timeout 30            # abort after 30 seconds
```

Stdin is supported with `-`:

```bash
pred create MIS --graph 0-1,1-2,2-3 | pred solve -
pred create MIS --graph 0-1,1-2,2-3 | pred solve - --solver brute-force
pred create MinMaxMulticenter --graph 0-1,1-2,2-3 --weights 1,1,1,1 --edge-weights 1,1,1 --k 2 | pred solve - --solver brute-force
pred create TwoDimensionalConsecutiveSets --alphabet-size 6 --sets "0,1,2;3,4,5;1,3;2,4;0,5" | pred solve - --solver brute-force
```

Output is JSON. When the problem is not ILP, the solver automatically reduces it to ILP, solves, and maps the solution back:

```json
{{#include generated/pred-solve-ilp.txt}}
```

Solve a reduction bundle (from `pred reduce`):

```json
{{#include generated/pred-solve-bundle.txt}}
```

> **Note:** The ILP solver requires a reduction path from the target problem to ILP.
> Some problems do not currently have one. Examples include BoundedComponentSpanningForest,
> LengthBoundedDisjointPaths, MinimumCardinalityKey, QUBO, SpinGlass, MaxCut, CircuitSAT, MinMaxMulticenter, and MultiprocessorScheduling.
> Use `pred solve <file> --solver brute-force` for these, or reduce to a problem that supports ILP first.
> For other problems, use `pred path <PROBLEM> ILP` to check whether an ILP reduction path exists.

For example, the canonical Minimum Cardinality Key instance can be created and solved with:

```bash
pred create MinimumCardinalityKey --num-attributes 6 --dependencies "0,1>2;0,2>3;1,3>4;2,4>5" -o mck.json
pred solve mck.json --solver brute-force
```

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

All commands support `-o` to write JSON to a file and `--json` to print JSON to stdout:

```bash
pred list -o problems.json       # save to file
pred list --json                 # print JSON to stdout
pred show MIS --json             # works on any command
pred path MIS QUBO --json
pred solve problem.json --json
```

This is useful for scripting and piping:

```bash
pred list --json | jq '.variants[].name'
pred path MIS QUBO --json | jq '.path'
```

## Problem Name Aliases

You can use short aliases instead of full problem names (shown in `pred list`):

{{#include generated/pred-aliases.txt}}

You can also specify variants with a slash: `MIS/UnitDiskGraph`, `SpinGlass/SimpleGraph`.

When a bare name (no slash) is used in commands like `path`, `to`, `from`, `create`, or `reduce`, it resolves to the **declared default variant** for that problem type. For example, `MIS` resolves to `MaximumIndependentSet/SimpleGraph/One`.

If you mistype a problem name, `pred` will suggest the closest match:

```text
{{#include generated/pred-show-typo.txt}}
```
