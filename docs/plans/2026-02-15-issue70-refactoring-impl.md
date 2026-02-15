# Issue #70 Refactoring Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Address top 3 high-impact KISS/DRY violations from issue #70: trim graph problem APIs, extract testable functions from `to_json()`, and implement real PlanarGraph/BipartiteGraph types.

**Architecture:** Remove delegation methods from 5 graph problem structs so callers go through `.graph()` directly. Extract pure utility functions from the `to_json()` monolith for independent testability. Replace PlanarGraph/BipartiteGraph ZST markers with validated wrapper types.

**Tech Stack:** Rust, petgraph (for SimpleGraph internals), serde, inventory crate

---

## Task 1: Remove delegation methods from MaximumIndependentSet

**Files:**
- Modify: `src/models/graph/maximum_independent_set.rs:100-148` (remove methods)
- Modify: `src/rules/maximumindependentset_qubo.rs` (update call sites)
- Modify: `src/rules/maximumindependentset_ilp.rs` (update call sites)
- Modify: `src/rules/maximumindependentset_maximumsetpacking.rs` (update call sites)
- Modify: `src/rules/maximumindependentset_gridgraph.rs` (update call sites)
- Modify: `src/rules/maximumindependentset_triangular.rs` (update call sites)
- Modify: `src/rules/sat_maximumindependentset.rs` (update call sites)
- Modify: `src/rules/minimumvertexcover_maximumindependentset.rs` (update MIS call sites)
- Test: `src/unit_tests/models/graph/maximum_independent_set.rs`

**Step 1: Edit MaximumIndependentSet — remove methods and rename**

In `src/models/graph/maximum_independent_set.rs`:
- Delete `from_graph_unit_weights()` (lines 100-107)
- Delete `num_vertices()` (lines 114-117)
- Delete `num_edges()` (lines 119-122)
- Delete `edges()` (lines 124-127)
- Delete `has_edge()` (lines 129-132)
- Delete `set_weights()` (lines 139-143)
- Delete `weights()` clone version (lines 145-148)
- Rename `weights_ref()` to `weights()`, change return type from `&Vec<W>` to `&[W]`

Internal code in the same file that uses `self.graph.num_vertices()` etc. should already work since it accesses the field directly.

**Step 2: Update rule call sites**

Replace in each rule file:
- `self.num_vertices()` → `self.graph().num_vertices()`
- `self.num_edges()` → `self.graph().num_edges()`
- `self.edges()` → `self.graph().edges()`
- `self.weights_ref()` → `self.weights()`
- `self.weights()` (where clone was intended) → `self.weights().to_vec()`

Files to update:
- `src/rules/maximumindependentset_qubo.rs`: `self.num_vertices()` → `self.graph().num_vertices()`, `self.weights_ref()` → `self.weights()`
- `src/rules/maximumindependentset_ilp.rs`: `self.num_vertices()` → `self.graph().num_vertices()`, `self.weights_ref()` → `self.weights()`
- `src/rules/maximumindependentset_maximumsetpacking.rs`: `self.num_vertices()` → `self.graph().num_vertices()`, `self.weights_ref()` → `self.weights()`
- `src/rules/maximumindependentset_gridgraph.rs`: `self.num_vertices()` → `self.graph().num_vertices()`
- `src/rules/maximumindependentset_triangular.rs`: `self.num_vertices()` → `self.graph().num_vertices()`
- `src/rules/sat_maximumindependentset.rs`: update any MIS delegation calls
- `src/rules/minimumvertexcover_maximumindependentset.rs`: uses `self.num_vertices()` and `self.weights_ref()` on MVC (handled in Task 2), but also constructs MIS

**Step 3: Update test call sites**

In `src/unit_tests/models/graph/maximum_independent_set.rs` and `src/unit_tests/graph_models.rs`:
- `problem.num_vertices()` → `problem.graph().num_vertices()`
- `problem.num_edges()` → `problem.graph().num_edges()`
- `problem.edges()` → `problem.graph().edges()`
- `problem.weights()` (cloning) → `problem.weights().to_vec()`
- `problem.weights_ref()` → `problem.weights()`

**Step 4: Run tests**

Run: `cargo test --all-features -- --include-ignored 2>&1 | head -50`
Expected: All tests pass for MaximumIndependentSet

**Step 5: Commit**

```bash
git add src/models/graph/maximum_independent_set.rs src/rules/maximumindependentset_*.rs src/rules/sat_maximumindependentset.rs src/rules/minimumvertexcover_maximumindependentset.rs src/unit_tests/
git commit -m "refactor: trim MaximumIndependentSet API — remove delegation methods"
```

---

## Task 2: Remove delegation methods from MinimumVertexCover

**Files:**
- Modify: `src/models/graph/minimum_vertex_cover.rs:84-143` (remove methods)
- Modify: `src/rules/minimumvertexcover_qubo.rs`
- Modify: `src/rules/minimumvertexcover_ilp.rs`
- Modify: `src/rules/minimumvertexcover_maximumindependentset.rs`
- Modify: `src/rules/minimumvertexcover_minimumsetcovering.rs`
- Test: `src/unit_tests/models/graph/minimum_vertex_cover.rs`

**Step 1: Edit MinimumVertexCover — same removal pattern as Task 1**

In `src/models/graph/minimum_vertex_cover.rs`:
- Delete `from_graph_unit_weights()`, `num_vertices()`, `num_edges()`, `edges()`, `has_edge()`, `set_weights()`, `weights()` (clone)
- Rename `weights_ref()` → `weights()` returning `&[W]`

**Step 2: Update rule call sites**

- `src/rules/minimumvertexcover_qubo.rs`: `self.num_vertices()` → `self.graph().num_vertices()`, `self.weights_ref()` → `self.weights()`
- `src/rules/minimumvertexcover_ilp.rs`: `self.num_vertices()` → `self.graph().num_vertices()`, `self.weights_ref()` → `self.weights()`
- `src/rules/minimumvertexcover_maximumindependentset.rs`: `self.num_vertices()` → `self.graph().num_vertices()`, `self.weights_ref()` → `self.weights()`
- `src/rules/minimumvertexcover_minimumsetcovering.rs`: `self.num_vertices()` → `self.graph().num_vertices()`, `self.weights_ref()` → `self.weights()`

**Step 3: Update test call sites**

- `src/unit_tests/models/graph/minimum_vertex_cover.rs`
- `src/unit_tests/graph_models.rs` (MVC sections)
- `src/unit_tests/rules/minimumvertexcover_*.rs`

**Step 4: Run tests**

Run: `cargo test --all-features -- --include-ignored 2>&1 | head -50`

**Step 5: Commit**

```bash
git add src/models/graph/minimum_vertex_cover.rs src/rules/minimumvertexcover_*.rs src/unit_tests/
git commit -m "refactor: trim MinimumVertexCover API — remove delegation methods"
```

---

## Task 3: Remove delegation methods from MaximumClique

**Files:**
- Modify: `src/models/graph/maximum_clique.rs:101-148`
- Modify: `src/rules/maximumclique_ilp.rs`
- Test: `src/unit_tests/models/graph/maximum_clique.rs`

**Step 1-5: Same pattern as Tasks 1-2**

- Delete the same set of methods, rename `weights_ref()` → `weights()`
- Update `src/rules/maximumclique_ilp.rs`: `self.num_vertices()` → `self.graph().num_vertices()`, `self.weights()` → `self.weights().to_vec()`
- Update test files
- Run tests, commit

```bash
git commit -m "refactor: trim MaximumClique API — remove delegation methods"
```

---

## Task 4: Remove delegation methods from MaximalIS

**Files:**
- Modify: `src/models/graph/maximal_is.rs:86-133`
- Test: `src/unit_tests/models/graph/maximal_is.rs`

**Step 1-5: Same pattern**

Note: MaximalIS has no reduction rules that call delegation methods directly. Only tests need updating.

```bash
git commit -m "refactor: trim MaximalIS API — remove delegation methods"
```

---

## Task 5: Remove delegation methods from MinimumDominatingSet

**Files:**
- Modify: `src/models/graph/minimum_dominating_set.rs:84-143`
- Modify: `src/rules/minimumdominatingset_ilp.rs`
- Modify: `src/rules/sat_minimumdominatingset.rs`
- Test: `src/unit_tests/models/graph/minimum_dominating_set.rs`

**Step 1-5: Same pattern**

- Update `src/rules/minimumdominatingset_ilp.rs`: `self.num_vertices()` → `self.graph().num_vertices()`, `self.weights()` → `self.weights().to_vec()`
- Update `src/rules/sat_minimumdominatingset.rs`: update call sites
- Update tests
- Run tests, commit

```bash
git commit -m "refactor: trim MinimumDominatingSet API — remove delegation methods"
```

---

## Task 6: Update remaining shared call sites

**Files:**
- Modify: `src/rules/mod.rs` (the `impl_natural_reduction!` macro uses `.weights()`)
- Modify: `src/rules/spinglass_maxcut.rs` (uses `.num_vertices()` on SpinGlass/MaxCut — check if affected)
- Modify: `src/rules/coloring_qubo.rs`, `src/rules/coloring_ilp.rs` (uses `.num_vertices()` on KColoring)
- Modify: `src/rules/maximummatching_maximumsetpacking.rs` (uses `.weights()`, `.edges()`, `.num_edges()` on MaximumMatching)
- Modify: `src/rules/maximummatching_ilp.rs` (uses `.num_edges()`, `.weights()` on MaximumMatching)
- Modify: `src/rules/travelingsalesman_ilp.rs` (uses `.num_vertices()` on TravelingSalesman)
- Modify: remaining test files in `src/unit_tests/`

**Step 1: Check which non-target problems also have delegation methods**

The 5 problems above are not the only ones with these methods. Other graph problems (MaxCut, SpinGlass, KColoring, MaximumMatching, TravelingSalesman, MaximumSetPacking, MinimumSetCovering) may also have delegation methods. These are OUT OF SCOPE for this PR — only update call sites that break because they called methods on the 5 target problem types.

**Step 2: Fix any remaining compilation errors**

Run: `cargo check --all-features 2>&1`
Fix any remaining call sites that the compiler identifies.

**Step 3: Run full test suite**

Run: `cargo test --all-features -- --include-ignored`
Expected: All tests pass

**Step 4: Run clippy**

Run: `cargo clippy --all-features -- -D warnings`
Expected: No warnings

**Step 5: Commit**

```bash
git commit -m "refactor: update remaining call sites for trimmed graph problem APIs"
```

---

## Task 7: Extract `classify_problem_category` from `to_json()`

**Files:**
- Modify: `src/rules/graph.rs` (extract function, update to_json)
- Test: `src/unit_tests/rules/graph.rs` (add unit test)

**Step 1: Write the failing test**

In the test file for graph.rs, add:

```rust
#[test]
fn test_classify_problem_category() {
    assert_eq!(
        classify_problem_category("problemreductions::models::graph::maximum_independent_set"),
        "graph"
    );
    assert_eq!(
        classify_problem_category("problemreductions::models::sat::satisfiability"),
        "sat"
    );
    assert_eq!(
        classify_problem_category("problemreductions::models::set::maximum_set_packing"),
        "set"
    );
    assert_eq!(
        classify_problem_category("problemreductions::models::optimization::qubo"),
        "optimization"
    );
    assert_eq!(
        classify_problem_category("unknown::path"),
        "other"
    );
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test --all-features test_classify_problem_category`
Expected: FAIL — function not found

**Step 3: Extract the function**

In `src/rules/graph.rs`, extract the existing inline logic (around the `category_from_module_path` helper) into a standalone `pub(crate) fn classify_problem_category(module_path: &str) -> &str`. Replace the inline usage in `to_json()` with a call to this function.

```rust
/// Classify a problem's category from its module path.
/// Expected format: "problemreductions::models::<category>::<module_name>"
pub(crate) fn classify_problem_category(module_path: &str) -> &str {
    let parts: Vec<&str> = module_path.split("::").collect();
    if parts.len() >= 3 {
        // Return the segment after "models"
        if let Some(pos) = parts.iter().position(|&p| p == "models") {
            if pos + 1 < parts.len() {
                return parts[pos + 1];
            }
        }
    }
    "other"
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test --all-features test_classify_problem_category`
Expected: PASS

**Step 5: Commit**

```bash
git commit -m "refactor: extract classify_problem_category from to_json()"
```

---

## Task 8: Extract `filter_redundant_base_nodes` from `to_json()`

**Files:**
- Modify: `src/rules/graph.rs`
- Test: `src/unit_tests/rules/graph.rs`

**Step 1: Write the failing test**

```rust
#[test]
fn test_filter_redundant_base_nodes() {
    use std::collections::{BTreeMap, HashSet};

    let mut node_set: HashSet<(String, BTreeMap<String, String>)> = HashSet::new();

    // Base node (empty variant) — should be removed because variant-specific sibling exists
    node_set.insert(("MIS".to_string(), BTreeMap::new()));

    // Variant-specific node
    let mut variant = BTreeMap::new();
    variant.insert("graph".to_string(), "GridGraph".to_string());
    node_set.insert(("MIS".to_string(), variant));

    // Base node with no siblings — should be kept
    node_set.insert(("QUBO".to_string(), BTreeMap::new()));

    filter_redundant_base_nodes(&mut node_set);

    assert_eq!(node_set.len(), 2);
    assert!(!node_set.iter().any(|(name, v)| name == "MIS" && v.is_empty()));
    assert!(node_set.iter().any(|(name, _)| name == "QUBO"));
}
```

**Step 2: Run test to verify it fails**

**Step 3: Extract the function**

```rust
/// Remove base nodes (empty variant) when a variant-specific sibling exists.
pub(crate) fn filter_redundant_base_nodes(
    node_set: &mut HashSet<(String, BTreeMap<String, String>)>,
) {
    let names_with_variants: HashSet<String> = node_set
        .iter()
        .filter(|(_, variant)| !variant.is_empty())
        .map(|(name, _)| name.clone())
        .collect();
    node_set.retain(|(name, variant)| !variant.is_empty() || !names_with_variants.contains(name));
}
```

Replace the inline logic in `to_json()` with `filter_redundant_base_nodes(&mut node_set);`.

**Step 4: Run test, verify pass**

**Step 5: Commit**

```bash
git commit -m "refactor: extract filter_redundant_base_nodes from to_json()"
```

---

## Task 9: Extract `is_natural_edge` from `to_json()`

**Files:**
- Modify: `src/rules/graph.rs`
- Test: `src/unit_tests/rules/graph.rs`

**Step 1: Write the failing test**

```rust
#[test]
fn test_is_natural_edge() {
    use std::collections::BTreeMap;

    let graph = ReductionGraph::new();

    // Same variant — no edge
    let mut a = BTreeMap::new();
    a.insert("graph".to_string(), "SimpleGraph".to_string());
    let b = a.clone();
    assert!(is_natural_edge(&a, &b, &graph).is_none());

    // a is subtype of b — edge from a to b
    let mut sub = BTreeMap::new();
    sub.insert("graph".to_string(), "GridGraph".to_string());
    let mut sup = BTreeMap::new();
    sup.insert("graph".to_string(), "SimpleGraph".to_string());
    // Direction depends on hierarchy — GridGraph is subtype of SimpleGraph
    let result = is_natural_edge(&sub, &sup, &graph);
    assert!(result.is_some());
}
```

Note: The exact test depends on how the natural edge determination works in the existing code. Read the inline logic at lines 917-950 of `src/rules/graph.rs` carefully before writing the extraction.

**Step 2: Extract the function**

Extract the inner loop body from lines 917-950 into:
```rust
/// Determine if there is a natural (subtype) edge between two variant nodes.
/// Returns Some(...) with edge data if a→b is a valid natural edge, None otherwise.
pub(crate) fn is_natural_edge(
    variant_a: &BTreeMap<String, String>,
    variant_b: &BTreeMap<String, String>,
    graph: &ReductionGraph,
) -> Option</* edge direction info */> {
    // ... extracted logic
}
```

Replace the inline logic in `to_json()` with a call to this function.

**Step 3: Run tests**

Run: `cargo test --all-features -- --include-ignored`

**Step 4: Commit**

```bash
git commit -m "refactor: extract is_natural_edge from to_json()"
```

---

## Task 10: Implement BipartiteGraph

**Files:**
- Create: `src/topology/bipartite_graph.rs`
- Modify: `src/topology/mod.rs` (add module + export)
- Modify: `src/graph_types.rs` (remove ZST BipartiteGraph + manual VariantParam impl)
- Test: `src/unit_tests/topology/bipartite_graph.rs`

**Step 1: Write the failing test**

Create `src/unit_tests/topology/bipartite_graph.rs`:

```rust
use crate::topology::{BipartiteGraph, Graph};

#[test]
fn test_bipartite_graph_basic() {
    // K_{2,3}: left={0,1}, right={0,1,2}, all edges
    let edges = vec![(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2)];
    let g = BipartiteGraph::new(2, 3, edges);

    assert_eq!(g.num_vertices(), 5);
    assert_eq!(g.num_edges(), 6);
    assert_eq!(g.left_size(), 2);
    assert_eq!(g.right_size(), 3);
}

#[test]
fn test_bipartite_graph_edges_unified() {
    // Left={0}, Right={0,1}, edges: (0,0), (0,1)
    let g = BipartiteGraph::new(1, 2, vec![(0, 0), (0, 1)]);
    let edges = g.edges();
    // Unified: left vertex 0, right vertices 1 and 2
    assert!(edges.contains(&(0, 1)));
    assert!(edges.contains(&(0, 2)));
    assert_eq!(edges.len(), 2);
}

#[test]
fn test_bipartite_graph_has_edge() {
    let g = BipartiteGraph::new(2, 2, vec![(0, 0), (1, 1)]);
    // Unified: edges (0, 2) and (1, 3)
    assert!(g.has_edge(0, 2));
    assert!(g.has_edge(1, 3));
    assert!(!g.has_edge(0, 1)); // both left — no edge
    assert!(!g.has_edge(0, 3)); // not in edge list
}

#[test]
fn test_bipartite_graph_neighbors() {
    let g = BipartiteGraph::new(2, 2, vec![(0, 0), (0, 1), (1, 1)]);
    // Unified: (0,2), (0,3), (1,3)
    let mut n0 = g.neighbors(0);
    n0.sort();
    assert_eq!(n0, vec![2, 3]);

    let mut n3 = g.neighbors(3); // right vertex 1
    n3.sort();
    assert_eq!(n3, vec![0, 1]);
}

#[test]
fn test_bipartite_graph_left_edges() {
    let edges = vec![(0, 0), (1, 1)];
    let g = BipartiteGraph::new(2, 2, edges.clone());
    assert_eq!(g.left_edges(), &edges);
}

#[test]
#[should_panic]
fn test_bipartite_graph_invalid_left_index() {
    BipartiteGraph::new(2, 2, vec![(2, 0)]); // left index out of bounds
}

#[test]
#[should_panic]
fn test_bipartite_graph_invalid_right_index() {
    BipartiteGraph::new(2, 2, vec![(0, 2)]); // right index out of bounds
}
```

Wire up test module: add `#[path]` reference in the appropriate unit test module file.

**Step 2: Run tests to verify they fail**

Run: `cargo test --all-features test_bipartite_graph`
Expected: FAIL — module not found

**Step 3: Implement BipartiteGraph**

Create `src/topology/bipartite_graph.rs`:

```rust
use serde::{Deserialize, Serialize};
use super::graph::{Graph, SimpleGraph};

/// Bipartite graph with explicit left/right partitions.
///
/// Vertices are split into left (indices `0..left_size`) and right (`0..right_size`).
/// Edges connect left vertices to right vertices using bipartite-local coordinates.
/// The `Graph` trait maps to a unified vertex space where right vertices are offset by `left_size`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BipartiteGraph {
    left_size: usize,
    right_size: usize,
    /// Edges in bipartite-local coordinates: (u, v) with u < left_size, v < right_size.
    edges: Vec<(usize, usize)>,
}

impl BipartiteGraph {
    /// Create a new bipartite graph.
    ///
    /// # Arguments
    /// * `left_size` - Number of vertices in the left partition
    /// * `right_size` - Number of vertices in the right partition
    /// * `edges` - Edges in bipartite-local coordinates: (u, v) with u < left_size, v < right_size
    ///
    /// # Panics
    /// Panics if any edge index is out of bounds.
    pub fn new(left_size: usize, right_size: usize, edges: Vec<(usize, usize)>) -> Self {
        for &(u, v) in &edges {
            assert!(
                u < left_size,
                "left vertex {} out of bounds (left_size={})",
                u, left_size
            );
            assert!(
                v < right_size,
                "right vertex {} out of bounds (right_size={})",
                v, right_size
            );
        }
        Self { left_size, right_size, edges }
    }

    /// Number of left-partition vertices.
    pub fn left_size(&self) -> usize {
        self.left_size
    }

    /// Number of right-partition vertices.
    pub fn right_size(&self) -> usize {
        self.right_size
    }

    /// Edges in bipartite-local coordinates.
    pub fn left_edges(&self) -> &[(usize, usize)] {
        &self.edges
    }
}

impl Graph for BipartiteGraph {
    const NAME: &'static str = "BipartiteGraph";

    fn num_vertices(&self) -> usize {
        self.left_size + self.right_size
    }

    fn num_edges(&self) -> usize {
        self.edges.len()
    }

    fn edges(&self) -> Vec<(usize, usize)> {
        self.edges
            .iter()
            .map(|&(u, v)| {
                let a = u;
                let b = self.left_size + v;
                if a < b { (a, b) } else { (b, a) }
            })
            .collect()
    }

    fn has_edge(&self, u: usize, v: usize) -> bool {
        let (u, v) = if u < v { (u, v) } else { (v, u) };
        // u must be left, v must be right (in unified space)
        if u >= self.left_size || v < self.left_size {
            return false;
        }
        let local_v = v - self.left_size;
        self.edges.contains(&(u, local_v))
    }

    fn neighbors(&self, v: usize) -> Vec<usize> {
        if v < self.left_size {
            // Left vertex: find all right neighbors
            self.edges
                .iter()
                .filter(|(u, _)| *u == v)
                .map(|(_, rv)| self.left_size + rv)
                .collect()
        } else {
            // Right vertex: find all left neighbors
            let local_v = v - self.left_size;
            self.edges
                .iter()
                .filter(|(_, rv)| *rv == local_v)
                .map(|(u, _)| *u)
                .collect()
        }
    }
}
```

**Step 4: Register with variant system**

Add at the bottom of `src/topology/bipartite_graph.rs`:

```rust
use crate::impl_variant_param;
impl_variant_param!(BipartiteGraph, "graph", parent: SimpleGraph,
    cast: |g| SimpleGraph::new(g.num_vertices(), g.edges()));
```

**Step 5: Wire up module**

In `src/topology/mod.rs`, add:
```rust
mod bipartite_graph;
pub use bipartite_graph::BipartiteGraph;
```

Remove the `BipartiteGraph` ZST and its manual `VariantParam` impl from `src/graph_types.rs` (lines 31-46).

**Step 6: Run tests**

Run: `cargo test --all-features test_bipartite_graph`
Expected: All pass

**Step 7: Commit**

```bash
git commit -m "feat: implement BipartiteGraph with standard bipartite representation"
```

---

## Task 11: Implement PlanarGraph

**Files:**
- Create: `src/topology/planar_graph.rs`
- Modify: `src/topology/mod.rs` (add module + export)
- Modify: `src/graph_types.rs` (remove ZST PlanarGraph + manual VariantParam impl)
- Test: `src/unit_tests/topology/planar_graph.rs`

**Step 1: Write the failing test**

```rust
use crate::topology::{PlanarGraph, Graph};

#[test]
fn test_planar_graph_basic() {
    // K4 is planar: 4 vertices, 6 edges, 6 <= 3*4 - 6 = 6
    let edges = vec![(0,1),(0,2),(0,3),(1,2),(1,3),(2,3)];
    let g = PlanarGraph::new(4, edges);
    assert_eq!(g.num_vertices(), 4);
    assert_eq!(g.num_edges(), 6);
}

#[test]
fn test_planar_graph_delegates_to_inner() {
    let g = PlanarGraph::new(3, vec![(0,1),(1,2)]);
    assert!(g.has_edge(0, 1));
    assert!(!g.has_edge(0, 2));
    let mut n1 = g.neighbors(1);
    n1.sort();
    assert_eq!(n1, vec![0, 2]);
}

#[test]
#[should_panic]
fn test_planar_graph_rejects_k5() {
    // K5 has 10 edges, but 3*5 - 6 = 9. Fails necessary condition.
    let mut edges = Vec::new();
    for i in 0..5 {
        for j in (i+1)..5 {
            edges.push((i, j));
        }
    }
    PlanarGraph::new(5, edges);
}

#[test]
fn test_planar_graph_empty() {
    let g = PlanarGraph::new(3, vec![]);
    assert_eq!(g.num_vertices(), 3);
    assert_eq!(g.num_edges(), 0);
}

#[test]
fn test_planar_graph_tree() {
    // Trees are always planar
    let g = PlanarGraph::new(4, vec![(0,1),(1,2),(2,3)]);
    assert_eq!(g.num_edges(), 3);
}
```

**Step 2: Implement PlanarGraph**

Create `src/topology/planar_graph.rs`:

```rust
use serde::{Deserialize, Serialize};
use super::graph::{Graph, SimpleGraph};

/// Planar graph — validated wrapper around SimpleGraph.
///
/// Construction validates the necessary planarity condition: |E| <= 3|V| - 6 for |V| >= 3.
/// This is a necessary but not sufficient condition. A follow-up issue will add
/// full planarity testing and half-edge (DCEL) representation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanarGraph {
    inner: SimpleGraph,
}

impl PlanarGraph {
    /// Create a new planar graph.
    ///
    /// # Panics
    /// Panics if the graph violates the necessary planarity condition |E| <= 3|V| - 6.
    pub fn new(num_vertices: usize, edges: Vec<(usize, usize)>) -> Self {
        let inner = SimpleGraph::new(num_vertices, edges);
        if num_vertices >= 3 {
            let max_edges = 3 * num_vertices - 6;
            assert!(
                inner.num_edges() <= max_edges,
                "graph has {} edges but a planar graph on {} vertices can have at most {} edges",
                inner.num_edges(), num_vertices, max_edges
            );
        }
        Self { inner }
    }

    /// Get a reference to the underlying SimpleGraph.
    pub fn inner(&self) -> &SimpleGraph {
        &self.inner
    }
}

impl Graph for PlanarGraph {
    const NAME: &'static str = "PlanarGraph";

    fn num_vertices(&self) -> usize { self.inner.num_vertices() }
    fn num_edges(&self) -> usize { self.inner.num_edges() }
    fn edges(&self) -> Vec<(usize, usize)> { self.inner.edges() }
    fn has_edge(&self, u: usize, v: usize) -> bool { self.inner.has_edge(u, v) }
    fn neighbors(&self, v: usize) -> Vec<usize> { self.inner.neighbors(v) }
}

use crate::impl_variant_param;
impl_variant_param!(PlanarGraph, "graph", parent: SimpleGraph,
    cast: |g| g.inner.clone());
```

**Step 3: Wire up module and remove ZST**

In `src/topology/mod.rs`:
```rust
mod planar_graph;
pub use planar_graph::PlanarGraph;
```

Remove PlanarGraph ZST and manual VariantParam impl from `src/graph_types.rs` (lines 10-25).

**Step 4: Run tests**

Run: `cargo test --all-features -- --include-ignored`

**Step 5: Commit**

```bash
git commit -m "feat: implement PlanarGraph as validated SimpleGraph wrapper"
```

---

## Task 12: Final verification and cleanup

**Step 1: Run full test suite**

Run: `cargo test --all-features -- --include-ignored`
Expected: All tests pass

**Step 2: Run clippy**

Run: `cargo clippy --all-features -- -D warnings`
Expected: No warnings

**Step 3: Run format check**

Run: `cargo fmt -- --check`
Expected: No formatting issues

**Step 4: File follow-up issue**

Create a GitHub issue for full data structure implementations:
- PlanarGraph: half-edge (DCEL) data structure
- BipartiteGraph: bipartite-specific algorithms

**Step 5: Final commit if any cleanup needed**

```bash
git commit -m "chore: final cleanup for issue #70 refactoring"
```
