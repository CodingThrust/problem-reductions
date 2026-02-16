# Graph Constructor Refactoring Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Remove `new(num_vertices, edges)` constructors from 9 graph problem types, rename `from_graph` to `new`, and update all call sites.

**Architecture:** Each graph problem currently has a `new()` that internally constructs a `SimpleGraph`, hiding the topology type. We rename `from_graph(graph, ...)` → `new(graph, ...)` and delete the old SimpleGraph-only `new()` / `with_weights()` / `unweighted()` constructors. Auxiliary graph-based constructors (`from_graph_unweighted`, `from_graph_unit_weights`, `from_graph_with_k`) are renamed to drop the `from_graph_` prefix.

**Tech Stack:** Rust, no new dependencies.

---

## Three Constructor Families

### A. Vertex-weight types (5 types)
`MaximumIndependentSet`, `MinimumVertexCover`, `MinimumDominatingSet`, `MaximumClique`, `MaximalIS`

| Before | After |
|--------|-------|
| `new(num_vertices, edges)` | **Removed** |
| `with_weights(num_vertices, edges, weights)` | **Removed** |
| `from_graph(graph, weights)` | `new(graph, weights)` |

### B. Edge-weight types (3 types)
`MaxCut`, `MaximumMatching`, `TravelingSalesman`

| Before | After |
|--------|-------|
| `new(num_vertices, edges_with_weights)` | **Removed** |
| `unweighted(num_vertices, edges)` | **Removed** |
| `with_weights(num_vertices, edges, weights)` | **Removed** (MaxCut only) |
| `from_graph(graph, edge_weights)` | `new(graph, edge_weights)` |
| `from_graph_unweighted(graph)` | `unweighted(graph)` |
| `from_graph_unit_weights(graph)` | `unit_weights(graph)` |

### C. KColoring (1 type)

| Before | After |
|--------|-------|
| `new(num_vertices, edges)` | **Removed** |
| `from_graph(graph)` | `new(graph)` |
| `from_graph_with_k(graph, k)` | `with_k(graph, k)` |

## Call Site Migration Pattern

Every `ProblemType::new(n, edges)` becomes:
```rust
// Before
let p = MaximumIndependentSet::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2)]);

// After
let p = MaximumIndependentSet::new(SimpleGraph::new(4, vec![(0, 1), (1, 2)]), vec![1; 4]);
```

For unweighted convenience, callers construct the graph inline and provide unit weights explicitly. This is intentional — it makes the graph topology visible.

---

## Task 1: Refactor MaximumIndependentSet constructors

**Files:**
- Modify: `src/models/graph/maximum_independent_set.rs`

**Step 1: Remove `impl<W: Clone + Default> MaximumIndependentSet<SimpleGraph, W>` block**

Delete the entire `impl` block (lines 62-87) that contains `new()` and `with_weights()`.

**Step 2: Rename `from_graph` → `new` in generic impl block**

In the `impl<G: Graph, W: Clone + Default> MaximumIndependentSet<G, W>` block (line 89), rename `from_graph` to `new`.

**Step 3: Update doc comment example**

Update the rustdoc example (lines 39-53) to use the new constructor pattern.

**Step 4: Run `make test clippy` to identify all broken call sites**

Expected: compilation errors at all call sites using the old `new()`.

**Step 5: Commit**

```bash
git add src/models/graph/maximum_independent_set.rs
git commit -m "refactor(MaximumIndependentSet): rename from_graph → new, remove SimpleGraph constructors"
```

---

## Task 2: Fix MaximumIndependentSet call sites in rules

**Files:**
- Modify: `src/rules/sat_maximumindependentset.rs` (line 158)
- Modify: `src/rules/maximumindependentset_casts.rs` (lines 15, 23, 31) — rename `from_graph` → `new`
- Modify: `src/rules/maximumindependentset_gridgraph.rs` (lines 53, 100) — rename `from_graph` → `new`
- Modify: `src/rules/maximumindependentset_triangular.rs` (line 55) — rename `from_graph` → `new`
- Modify: `src/rules/mod.rs` (line 107) — update doc example

**Step 1: Update each file**

For `sat_maximumindependentset.rs:158`:
```rust
// Before
let target = MaximumIndependentSet::new(vertex_count, edges);
// After
let target = MaximumIndependentSet::new(SimpleGraph::new(vertex_count, edges), vec![1i32; vertex_count]);
```

For files using `from_graph`, simply rename to `new`.

**Step 2: Run `make test clippy`**

**Step 3: Commit**

---

## Task 3: Fix MaximumIndependentSet call sites in unit tests

**Files:**
- Modify: `src/unit_tests/models/graph/maximum_independent_set.rs` — all `::new()` calls
- Modify: `src/unit_tests/rules/` — any files using MIS::new()
- Modify: `src/unit_tests/trait_consistency.rs` — if applicable

Each `MaximumIndependentSet::<SimpleGraph, i32>::new(n, edges)` becomes:
```rust
MaximumIndependentSet::new(SimpleGraph::new(n, edges), vec![1; n])
```

Each `MaximumIndependentSet::with_weights(n, edges, weights)` becomes:
```rust
MaximumIndependentSet::new(SimpleGraph::new(n, edges), weights)
```

**Step 1: Update all call sites in unit test files**

**Step 2: Run `make test`**

**Step 3: Commit**

---

## Task 4: Fix MaximumIndependentSet call sites in integration tests and examples

**Files:**
- Modify: `tests/suites/integration.rs` — multiple calls
- Modify: `tests/suites/reductions.rs` — multiple calls
- Modify: `examples/reduction_maximumindependentset_to_qubo.rs`
- Modify: `examples/reduction_maximumindependentset_to_ilp.rs`
- Modify: `examples/reduction_maximumindependentset_to_minimumvertexcover.rs`
- Modify: `examples/reduction_maximumindependentset_to_maximumsetpacking.rs`
- Modify: `benches/solver_benchmarks.rs`
- Modify: `src/topology/mod.rs` (doc example at line 18)

**Step 1: Update all call sites**

**Step 2: Run `make test clippy`**

**Step 3: Commit**

---

## Task 5: Refactor MinimumVertexCover constructors + call sites

**Files:**
- Modify: `src/models/graph/minimum_vertex_cover.rs` — remove SimpleGraph impl, rename `from_graph` → `new`
- Modify: `tests/suites/integration.rs`
- Modify: `tests/suites/reductions.rs`
- Modify: `examples/reduction_minimumvertexcover_to_*.rs` (4 example files)
- Modify: `benches/solver_benchmarks.rs`

Same pattern as MaximumIndependentSet. Remove SimpleGraph-only constructors, rename `from_graph` → `new`, update all call sites.

**Step 1-3: Update model, call sites, test**

**Step 4: Commit**

---

## Task 6: Refactor MinimumDominatingSet constructors + call sites

**Files:**
- Modify: `src/models/graph/minimum_dominating_set.rs`
- Modify: `src/rules/sat_minimumdominatingset.rs` (line 169)
- Modify: `src/unit_tests/models/graph/minimum_dominating_set.rs`
- Modify: `tests/suites/integration.rs`
- Modify: `examples/reduction_minimumdominatingset_to_ilp.rs`

**Step 1-3: Update model, call sites, test**

**Step 4: Commit**

---

## Task 7: Refactor MaximumClique constructors + call sites

**Files:**
- Modify: `src/models/graph/maximum_clique.rs`
- Modify: `src/unit_tests/models/graph/maximum_clique.rs`
- Modify: `src/unit_tests/rules/maximumclique_ilp.rs` (10 calls)
- Modify: `tests/suites/integration.rs`
- Modify: `examples/reduction_maximumclique_to_ilp.rs`

**Step 1-3: Update model, call sites, test**

**Step 4: Commit**

---

## Task 8: Refactor MaximalIS constructors + call sites

**Files:**
- Modify: `src/models/graph/maximal_is.rs`
- Modify: `src/unit_tests/models/graph/maximal_is.rs`
- Modify: `tests/suites/integration.rs`

**Step 1-3: Update model, call sites, test**

**Step 4: Commit**

---

## Task 9: Refactor KColoring constructors + call sites

**Files:**
- Modify: `src/models/graph/kcoloring.rs` — remove SimpleGraph impl, rename `from_graph` → `new`, rename `from_graph_with_k` → `with_k`
- Modify: `src/rules/sat_coloring.rs` (line 204)
- Modify: `src/rules/kcoloring_casts.rs` (line 12) — rename `from_graph_with_k` → `with_k`
- Modify: `src/unit_tests/models/graph/kcoloring.rs` (~15 calls)
- Modify: `src/unit_tests/graph_models.rs` (~10 calls)
- Modify: `src/unit_tests/rules/coloring_qubo.rs` (4 calls)
- Modify: `src/unit_tests/rules/coloring_ilp.rs` (~15 calls)
- Modify: `src/unit_tests/trait_consistency.rs`
- Modify: `tests/suites/integration.rs`
- Modify: `tests/suites/reductions.rs`
- Modify: `examples/reduction_kcoloring_to_qubo.rs`
- Modify: `examples/reduction_kcoloring_to_ilp.rs`
- Modify: `benches/solver_benchmarks.rs`

Each `KColoring::<K3, SimpleGraph>::new(n, edges)` becomes:
```rust
KColoring::<K3, _>::new(SimpleGraph::new(n, edges))
```

**Step 1-3: Update model, call sites, test**

**Step 4: Commit**

---

## Task 10: Refactor MaxCut constructors + call sites

**Files:**
- Modify: `src/models/graph/max_cut.rs` — remove SimpleGraph impl, rename `from_graph` → `new`, rename `from_graph_unweighted` → `unweighted`
- Modify: `src/unit_tests/models/graph/max_cut.rs`
- Modify: `tests/suites/integration.rs`
- Modify: `tests/suites/reductions.rs`
- Modify: `examples/reduction_maxcut_to_spinglass.rs` (uses `unweighted`)
- Modify: `benches/solver_benchmarks.rs`

Each `MaxCut::new(n, vec![(u, v, w), ...])` becomes:
```rust
MaxCut::new(SimpleGraph::new(n, edge_list), weights_vec)
```

Each `MaxCut::unweighted(n, edges)` becomes:
```rust
MaxCut::unweighted(SimpleGraph::new(n, edges))
```

**Step 1-3: Update model, call sites, test**

**Step 4: Commit**

---

## Task 11: Refactor MaximumMatching constructors + call sites

**Files:**
- Modify: `src/models/graph/maximum_matching.rs` — remove SimpleGraph impl, rename `from_graph` → `new`, rename `from_graph_unit_weights` → `unit_weights`
- Modify: `src/unit_tests/models/graph/maximum_matching.rs`
- Modify: `src/unit_tests/rules/maximummatching_ilp.rs`
- Modify: `tests/suites/integration.rs`
- Modify: `examples/reduction_maximummatching_to_ilp.rs` (uses `unweighted`)
- Modify: `examples/reduction_maximummatching_to_maximumsetpacking.rs` (uses `unweighted`)
- Modify: `benches/solver_benchmarks.rs`

**Step 1-3: Update model, call sites, test**

**Step 4: Commit**

---

## Task 12: Refactor TravelingSalesman constructors + call sites

**Files:**
- Modify: `src/models/graph/traveling_salesman.rs` — remove SimpleGraph impl, rename `from_graph` → `new`, rename `from_graph_unit_weights` → `unit_weights`
- Modify: `src/unit_tests/models/graph/traveling_salesman.rs`
- Modify: `examples/reduction_travelingsalesman_to_ilp.rs`

**Step 1-3: Update model, call sites, test**

**Step 4: Commit**

---

## Task 13: Final validation

**Step 1: Run full test suite**

```bash
make test clippy
```

**Step 2: Run doc tests**

```bash
cargo test --doc
```

**Step 3: Update any remaining documentation**

Check `docs/src/design.md` for any stale constructor examples.

**Step 4: Final commit if needed**
