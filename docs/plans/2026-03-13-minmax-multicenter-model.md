# Plan: Add MinMaxMulticenter Model

**Issue:** #330 — [Model] MinMaxMulticenter (p-Center Problem)
**Skill:** add-model
**Date:** 2026-03-13

## Summary

Implement the MinMaxMulticenter (vertex p-center) problem as a **satisfaction problem** (`Metric = bool`). Given a graph with vertex weights and edge lengths, K centers to place, and a distance bound B, determine whether K vertices can be chosen such that the maximum weighted shortest-path distance from any vertex to its nearest center is at most B.

## Information Checklist

| # | Item | Value |
|---|------|-------|
| 1 | Problem name | `MinMaxMulticenter` |
| 2 | Mathematical definition | Given G=(V,E), vertex weights w(v), edge lengths l(e), integer K, bound B: is there S ⊆ V with \|S\|=K such that max{d(v)·w(v)} ≤ B? |
| 3 | Problem type | Satisfaction (`Metric = bool`) |
| 4 | Type parameters | `G: Graph`, `W: WeightElement` (for vertex weights, edge lengths, and bound) |
| 5 | Struct fields | `graph: G`, `vertex_weights: Vec<W>`, `edge_lengths: Vec<W>`, `k: usize`, `bound: W::Sum` |
| 6 | Configuration space | `vec![2; num_vertices]` — binary selection of centers |
| 7 | Feasibility check | Exactly K vertices selected AND max weighted distance ≤ B |
| 8 | Objective function | N/A (satisfaction: returns `true` if feasible) |
| 9 | Best known exact | O*(1.4969^num_vertices) via binary search over distance thresholds + dominating set (van Rooij & Bodlaender, 2011) |
| 10 | Solving strategy | BruteForce (enumerate all C(n,K) subsets); existing ILP reduction possible |
| 11 | Category | `graph` |

## Implementation Steps

### Step 1: Create model file
**File:** `src/models/graph/min_max_multicenter.rs`

Implement:
- `inventory::submit!` for `ProblemSchemaEntry`
- Struct `MinMaxMulticenter<G, W>` with fields: `graph`, `vertex_weights`, `edge_lengths`, `k`, `bound`
- Constructor `new(graph, vertex_weights, edge_lengths, k, bound)` with assertions
- Accessor methods: `graph()`, `vertex_weights()`, `edge_lengths()`, `k()`, `bound()`
- Getter methods for overhead: `num_vertices()`, `num_edges()`, `num_centers()`
- Helper: `compute_shortest_paths()` using BFS/Dijkstra on the weighted graph
- `evaluate()`: check exactly K centers selected, compute shortest weighted distances, check max{d(v)·w(v)} ≤ B
- `Problem` impl with `Metric = bool`, `SatisfactionProblem` impl
- `variant_params![G, W]`
- `declare_variants!` with complexity `"1.4969^num_vertices"`

### Step 2: Register the model
- `src/models/graph/mod.rs`: add `pub(crate) mod min_max_multicenter;` and `pub use min_max_multicenter::MinMaxMulticenter;`
- `src/models/mod.rs`: add `MinMaxMulticenter` to graph re-exports
- `src/lib.rs` prelude: add `MinMaxMulticenter`

### Step 3: Register in CLI
- `problemreductions-cli/src/dispatch.rs`: add `"MinMaxMulticenter"` arms in `load_problem()` (using `deser_sat`) and `serialize_any_problem()` (using `try_ser`)
- `problemreductions-cli/src/problem_name.rs`: add `"minmaxmulticenter"` to `resolve_alias()`

### Step 4: Add CLI creation support
- `problemreductions-cli/src/cli.rs`: add `--edge-lengths` and `--bound` flags to `CreateArgs`, update `all_data_flags_empty()`, update help table
- `problemreductions-cli/src/commands/create.rs`: add `"MinMaxMulticenter"` match arm parsing `--graph`, `--weights`, `--edge-lengths`, `--k`, `--bound`

### Step 5: Write unit tests
**File:** `src/unit_tests/models/graph/min_max_multicenter.rs`

Tests:
- `test_minmaxmulticenter_creation`: construct instance, verify dims
- `test_minmaxmulticenter_evaluation`: test valid/invalid configs (from issue example)
- `test_minmaxmulticenter_serialization`: round-trip serde
- `test_minmaxmulticenter_solver`: brute-force finds correct satisfying solution on example instance

### Step 6: Document in paper
- Add `"MinMaxMulticenter": [Min-Max Multicenter]` to `display-name` dict in `docs/paper/reductions.typ`
- Add `#problem-def("MinMaxMulticenter")` with formal definition and background

### Step 7: Verify
- `make test clippy` must pass
