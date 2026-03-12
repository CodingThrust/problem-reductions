# Plan: Add GraphPartitioning Model (#117)

## Overview

Add the GraphPartitioning (Minimum Bisection) problem to the codebase. Given an undirected graph G = (V, E) with |V| = n (even), partition V into two equal halves A, B with |A| = |B| = n/2, minimizing the number of crossing edges.

## Design Decisions

- **Name:** `GraphPartitioning` (no prefix, per issue)
- **Type parameters:** Single parameter `G: Graph` (no weight type — unweighted edge counting)
- **Metric:** `SolutionSize<i32>` — cut size is an integer count
- **Direction:** `Minimize`
- **Feasibility:** Config is valid iff exactly n/2 vertices are assigned to each side (balanced bisection). If n is odd, all configs are `Invalid`.
- **Complexity:** `"2^num_vertices"` — brute-force bound (no known sub-exponential exact algorithm for general minimum bisection)
- **Solver:** Existing BruteForce works via OptimizationProblem trait
- **Category:** `graph/` (graph input)

## Tasks

### Task 1: Implement the model [independent]

**File:** `src/models/graph/graph_partitioning.rs`

Follow MaxCut as template, with these differences:
- Single type param `G` (no `W`)
- No edge weights — just count crossing edges
- `evaluate()` checks balanced bisection (sum of config == n/2), returns `Invalid` if not balanced or n is odd
- `direction()` returns `Minimize`
- `variant_params![G]` (single param)

Structure:
1. `inventory::submit!` with `ProblemSchemaEntry` — fields: `[graph: G]`
2. Struct `GraphPartitioning<G>` with field `graph: G`
3. `new(graph)` constructor + `graph()` accessor
4. Size getters: `num_vertices()`, `num_edges()`
5. `Problem` impl with `Metric = SolutionSize<i32>`, `dims() = vec![2; n]`
6. `OptimizationProblem` impl with `Value = i32`, `direction() = Minimize`
7. `declare_variants! { GraphPartitioning<SimpleGraph> => "2^num_vertices" }`
8. `#[cfg(test)] #[path = "..."] mod tests;`

### Task 2: Register the model [depends on Task 1]

**Files to update:**
1. `src/models/graph/mod.rs` — add `pub(crate) mod graph_partitioning;` and `pub use graph_partitioning::GraphPartitioning;`
2. `src/models/mod.rs` — add `GraphPartitioning` to the graph re-export line

### Task 3: Register in CLI [depends on Task 1]

**Files to update:**

1. `problemreductions-cli/src/dispatch.rs`:
   - `load_problem()`: add `"GraphPartitioning" => deser_opt::<GraphPartitioning<SimpleGraph>>(data)`
   - `serialize_any_problem()`: add `"GraphPartitioning" => try_ser::<GraphPartitioning<SimpleGraph>>(any)`
   - Add import: `use problemreductions::models::graph::GraphPartitioning;`

2. `problemreductions-cli/src/problem_name.rs`:
   - `resolve_alias()`: add `"graphpartitioning" => "GraphPartitioning".to_string()`

3. `problemreductions-cli/src/commands/create.rs`:
   - `example_for()`: add `"GraphPartitioning" => "--graph 0-1,1-2,2-3,0-2,1-3,0-3"`
   - Main `create()` match: add a new arm for `"GraphPartitioning"` that parses `--graph` and constructs with `GraphPartitioning::new(graph)`
   - Random generation arm: add `"GraphPartitioning"` with random graph + `variant_map(&[("graph", "SimpleGraph")])`
   - Add import for `GraphPartitioning`

### Task 4: Write unit tests [depends on Task 1]

**File:** `src/unit_tests/models/graph/graph_partitioning.rs`

Tests to write:
- `test_graphpartitioning_basic` — construct instance, verify dims, evaluate valid/invalid configs
- `test_graphpartitioning_direction` — verify `Minimize`
- `test_graphpartitioning_serialization` — round-trip serde
- `test_graphpartitioning_solver` — brute-force finds optimal partition matching issue example
- `test_graphpartitioning_odd_vertices` — all configs Invalid when n is odd
- `test_graphpartitioning_unbalanced_invalid` — non-balanced partitions return Invalid

Use issue example: 6 vertices, edges (0,1),(0,2),(1,2),(1,3),(2,3),(2,4),(3,4),(3,5),(4,5), optimal cut = 3.

Also register in `src/unit_tests/models/graph/mod.rs`.

### Task 5: Document in paper [depends on Task 1]

**File:** `docs/paper/reductions.typ`

1. Add to `display-name` dict: `"GraphPartitioning": [Graph Partitioning]`
2. Add `#problem-def("GraphPartitioning")[...]` with:
   - Formal definition of minimum bisection
   - Background on VLSI, parallel computing applications
   - Example with 6-vertex graph and CeTZ visualization
   - Algorithm list (brute-force)

### Task 6: Verify [depends on Tasks 1-5]

```bash
make fmt
make clippy
make test
```
