# Plan: Add OptimalLinearArrangement Model (Issue #406)

## Overview

Add the OptimalLinearArrangement problem model -- a classical NP-complete graph optimization problem (Garey & Johnson GT42) that asks for a vertex ordering on a line minimizing total edge length.

## Design Decisions

- **Optimization problem** with `Direction::Minimize` -- minimize sum of |f(u)-f(v)| over edges
- **Type parameter**: `G: Graph` only (no weight parameter -- edges are unweighted)
- **Configuration space**: `vec![n; n]` where n = |V|. Each variable assigns a position (0..n) to a vertex. A valid configuration is a permutation (all positions distinct).
- **Metric**: `SolutionSize<usize>` -- total edge length as usize
- **Category**: `graph/` (input is a graph)
- **Complexity**: O*(2^n) via Held-Karp-style DP over subsets

## Steps

### Step 1: Create model file `src/models/graph/optimal_linear_arrangement.rs`

- `inventory::submit!` for ProblemSchemaEntry
- Struct `OptimalLinearArrangement<G>` with field `graph: G`
- Constructor `new(graph: G)`
- Accessors: `graph()`, `num_vertices()`, `num_edges()`
- `is_valid_solution()` -- checks permutation validity
- `total_edge_length()` helper -- computes sum |f(u)-f(v)| for a permutation
- `Problem` impl: NAME="OptimalLinearArrangement", Metric=SolutionSize<usize>, dims=vec![n;n], evaluate checks permutation then returns Valid(cost)
- `OptimizationProblem` impl: Value=usize, direction=Minimize
- `declare_variants!` with `SimpleGraph => "2^num_vertices"`
- `variant_params![G]` (single type parameter)
- `#[cfg(test)] #[path]` link to unit tests

### Step 2: Register model

- `src/models/graph/mod.rs`: add `pub(crate) mod optimal_linear_arrangement;` and `pub use`
- `src/models/mod.rs`: add to graph re-export line
- `src/lib.rs` prelude: add `OptimalLinearArrangement`

### Step 3: Register in CLI

- `problemreductions-cli/src/dispatch.rs`: add match arms in `load_problem()` and `serialize_any_problem()`
- `problemreductions-cli/src/problem_name.rs`: add `"optimallineararrangement" => "OptimalLinearArrangement"` alias
- `problemreductions-cli/src/commands/create.rs`: add creation handler (graph-only, no weights)
- `problemreductions-cli/src/cli.rs`: add entry to "Flags by problem type" help table

### Step 4: Write unit tests `src/unit_tests/models/graph/optimal_linear_arrangement.rs`

- `test_optimal_linear_arrangement_creation` -- construct instance, verify dims
- `test_optimal_linear_arrangement_evaluation` -- valid and invalid configs
- `test_optimal_linear_arrangement_direction` -- verify Minimize
- `test_optimal_linear_arrangement_solver` -- brute-force finds optimal permutation
- `test_optimal_linear_arrangement_path_graph` -- path graph optimal = n-1
- `test_optimal_linear_arrangement_serialization` -- round-trip serde

### Step 5: Write paper entry

- Add `"OptimalLinearArrangement": [Optimal Linear Arrangement]` to display-name dict
- Add `#problem-def("OptimalLinearArrangement")` entry

### Step 6: Verify

- `make check` (fmt + clippy + test)
