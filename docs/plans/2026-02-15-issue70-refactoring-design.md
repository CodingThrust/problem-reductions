# Issue #70: KISS and DRY Refactoring Design

**Date:** 2026-02-15
**Issue:** [#70](https://github.com/GiggleLiu/problemreductions/issues/70)

## Scope

Three high-impact items from the issue, selected by priority:

1. **DRY #1** — Trim vertex-weighted graph problem API
2. **KISS #4** — Extract testable functions from `to_json()`
3. **DRY #2 (expanded)** — Implement real `PlanarGraph` and `BipartiteGraph`

## Item 1: Trim Vertex-Weighted Graph Problem API

### Problem

Five vertex-weighted graph problems share ~65 lines of identical delegation methods each. These convenience methods (`num_vertices()`, `num_edges()`, `edges()`, etc.) duplicate `Graph` trait methods already available via `problem.graph()`.

**Affected files:**
- `src/models/graph/maximum_independent_set.rs`
- `src/models/graph/minimum_vertex_cover.rs`
- `src/models/graph/maximum_clique.rs`
- `src/models/graph/maximal_is.rs`
- `src/models/graph/minimum_dominating_set.rs`

### Design

**Remove these methods from all 5 problems:**
- `num_vertices()` — callers use `problem.graph().num_vertices()`
- `num_edges()` — callers use `problem.graph().num_edges()`
- `edges()` — callers use `problem.graph().edges()`
- `has_edge(u, v)` — callers use `problem.graph().has_edge(u, v)`
- `set_weights()` — 0 external call sites
- `from_graph_unit_weights()` — 0 external call sites
- `weights()` (the cloning version) — replaced by renaming `weights_ref()`

**Rename:**
- `weights_ref() -> &Vec<W>` becomes `weights() -> &[W]`

**Keep:**
- `graph() -> &G`
- `weights() -> &[W]` (the renamed borrow version)
- `is_weighted() -> bool`
- `new(num_vertices, edges)` — 14 call sites
- `with_weights(num_vertices, edges, weights)` — 25 call sites
- `from_graph(graph, weights)` — 3 call sites

### Call site migration

| Old call | New call | Sites |
|----------|----------|-------|
| `problem.num_vertices()` | `problem.graph().num_vertices()` | ~49 |
| `problem.num_edges()` | `problem.graph().num_edges()` | ~36 |
| `problem.edges()` | `problem.graph().edges()` | ~29 |
| `problem.has_edge(u, v)` | `problem.graph().has_edge(u, v)` | 0 |
| `problem.weights_ref()` | `problem.weights()` | ~12 |
| `problem.weights()` (clone) | `problem.weights().to_vec()` | ~8 |

## Item 2: Extract Testable Functions from `to_json()`

### Problem

`ReductionGraph::to_json()` (`src/rules/graph.rs`, ~194 lines) is a monolith doing 5+ distinct things. Complex logic is embedded inline and untestable in isolation.

### Design

Extract three pure, testable utility functions while keeping `to_json()` as the orchestrator:

1. **`is_natural_edge(variant_a, variant_b, hierarchy) -> Option<Direction>`**
   Given two variant maps for the same problem name, determine if one is a subtype of the other. Core logic from the 65-line natural edge generation loop. Pure function.

2. **`classify_problem_category(module_path: &str) -> &str`**
   Map module path to category: `"graph"`, `"sat"`, `"set"`, or `"optimization"`. Currently inline in node-building phase.

3. **`filter_redundant_base_nodes(node_set) -> filtered_set`**
   Remove base nodes (empty variant) when a variant-specific sibling exists. ~15 lines of inline logic.

Each function gets its own unit test. `to_json()` calls these helpers but retains the orchestration flow.

## Item 3: Implement PlanarGraph and BipartiteGraph

### Problem

`PlanarGraph` and `BipartiteGraph` are currently ZST markers with no data or graph behavior. They manually implement `VariantParam` (12 lines each) instead of using `impl_variant_param!` because they have no cast closure.

### Design

Replace the ZST markers with real graph types.

#### PlanarGraph — Validated wrapper

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanarGraph {
    inner: SimpleGraph,
}
```

- **Constructor:** `PlanarGraph::new(num_vertices, edges)` — validates planarity via `|E| <= 3|V| - 6` (necessary condition). Panics on non-planar input.
- **Graph trait:** All methods delegate to `inner`.
- **Variant:** `impl_variant_param!(PlanarGraph, "graph", parent: SimpleGraph, cast: |g| g.inner.clone())`

#### BipartiteGraph — Standard bipartite representation

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BipartiteGraph {
    left_size: usize,
    right_size: usize,
    edges: Vec<(usize, usize)>,  // (u, v) with u in [0, left_size), v in [0, right_size)
}
```

- **Constructor:** `BipartiteGraph::new(left_size, right_size, edges)` — validates that edges are within bounds. Edges use bipartite-local coordinates.
- **Graph trait:** Maps to unified vertex space: left vertices `0..left_size`, right vertices `left_size..left_size+right_size`. `edges()` returns `(u, left_size + v)` for each stored `(u, v)`.
- **Accessors:** `left_size()`, `right_size()`, `left_edges()` (local coords).
- **Variant:** `impl_variant_param!(BipartiteGraph, "graph", parent: SimpleGraph, cast: |g| SimpleGraph::new(g.num_vertices(), g.edges()))`

### Follow-up issue

File a separate issue for full data structure implementations:
- PlanarGraph: half-edge (DCEL) data structure for proper planar embedding
- BipartiteGraph: additional bipartite-specific algorithms

## Testing

- **Item 1:** Update all ~114 call sites. Run `make test clippy` to verify nothing breaks.
- **Item 2:** Add unit tests for each extracted function.
- **Item 3:** Add tests for PlanarGraph (construction, planarity validation, graph trait) and BipartiteGraph (construction, edge mapping, partition accessors).

## Non-goals

- No macro extraction for constructor/trait boilerplate (accept remaining duplication as cost of explicitness)
- No changes to the cost function zoo (KISS #2)
- No changes to `find_shortest_path` (KISS #1)
- No full DCEL or bipartite algorithm implementation (deferred)
