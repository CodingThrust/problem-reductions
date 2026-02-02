# Consistent Type Parameterization Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Refactor all problem types to use consistent `Problem<G, W>` parameterization with explicit graph type.

**Architecture:** Add graph type parameter `G: Graph` to all graph-based problems. Rename `Coloring` to `KColoring<K, G, W>` with const generic. Remove template-based types. No default type parameters.

**Tech Stack:** Rust, petgraph, serde, num-traits

---

## Task 1: Add Graph::NAME constant to Graph trait

**Files:**
- Modify: `src/topology/graph.rs:41-80`
- Modify: `src/topology/grid_graph.rs` (impl)
- Modify: `src/topology/unit_disk_graph.rs` (impl)

**Step 1: Add NAME constant to Graph trait**

In `src/topology/graph.rs`, add to the `Graph` trait:

```rust
pub trait Graph: Clone + Send + Sync + 'static {
    /// The name of this graph type for variant identification.
    const NAME: &'static str;

    // ... existing methods
}
```

**Step 2: Implement NAME for SimpleGraph**

In `src/topology/graph.rs`, add to `impl Graph for SimpleGraph`:

```rust
impl Graph for SimpleGraph {
    const NAME: &'static str = "SimpleGraph";
    // ... existing methods
}
```

**Step 3: Implement NAME for other graph types**

In `src/topology/grid_graph.rs`:
```rust
impl Graph for GridGraph {
    const NAME: &'static str = "GridGraph";
    // ...
}
```

In `src/topology/unit_disk_graph.rs`:
```rust
impl Graph for UnitDiskGraph {
    const NAME: &'static str = "UnitDiskGraph";
    // ...
}
```

**Step 4: Run tests**

```bash
cargo test --lib topology
```
Expected: PASS

**Step 5: Commit**

```bash
git add src/topology/
git commit -m "feat(topology): add NAME constant to Graph trait"
```

---

## Task 2: Refactor IndependentSet<W> to IndependentSet<G, W>

**Files:**
- Modify: `src/models/graph/independent_set.rs`
- Test: `src/models/graph/independent_set.rs` (inline tests)

**Step 1: Update struct definition**

Replace:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndependentSet<W = i32> {
    graph: UnGraph<(), ()>,
    weights: Vec<W>,
}
```

With:
```rust
use crate::topology::{Graph, SimpleGraph};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndependentSet<G, W> {
    graph: G,
    weights: Vec<W>,
}
```

**Step 2: Update impl blocks for SimpleGraph**

Replace constructors to work with SimpleGraph:

```rust
impl<W: Clone + Default> IndependentSet<SimpleGraph, W> {
    pub fn new(num_vertices: usize, edges: Vec<(usize, usize)>) -> Self
    where
        W: From<i32>,
    {
        let graph = SimpleGraph::new(num_vertices, edges);
        let weights = vec![W::from(1); num_vertices];
        Self { graph, weights }
    }

    pub fn with_weights(num_vertices: usize, edges: Vec<(usize, usize)>, weights: Vec<W>) -> Self {
        assert_eq!(weights.len(), num_vertices);
        let graph = SimpleGraph::new(num_vertices, edges);
        Self { graph, weights }
    }
}
```

**Step 3: Add generic impl block for any Graph**

```rust
impl<G: Graph, W: Clone + Default> IndependentSet<G, W> {
    pub fn from_graph(graph: G, weights: Vec<W>) -> Self {
        assert_eq!(weights.len(), graph.num_vertices());
        Self { graph, weights }
    }

    pub fn from_graph_unit_weights(graph: G) -> Self
    where
        W: From<i32>,
    {
        let weights = vec![W::from(1); graph.num_vertices()];
        Self { graph, weights }
    }

    pub fn graph(&self) -> &G {
        &self.graph
    }

    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    pub fn edges(&self) -> Vec<(usize, usize)> {
        self.graph.edges()
    }

    pub fn has_edge(&self, u: usize, v: usize) -> bool {
        self.graph.has_edge(u, v)
    }

    pub fn weights_ref(&self) -> &Vec<W> {
        &self.weights
    }
}
```

**Step 4: Update Problem impl**

```rust
impl<G, W> Problem for IndependentSet<G, W>
where
    G: Graph,
    W: Clone + Default + PartialOrd + num_traits::Num + num_traits::Zero + std::ops::AddAssign + 'static,
{
    const NAME: &'static str = "IndependentSet";

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("graph", G::NAME),
            ("weight", short_type_name::<W>()),
        ]
    }

    type Size = W;

    fn num_variables(&self) -> usize {
        self.graph.num_vertices()
    }

    // ... rest unchanged but use self.graph.edges() etc
}
```

**Step 5: Update ConstraintSatisfactionProblem impl**

Update to use `self.graph.edges()` instead of `self.graph.edge_references()`.

**Step 6: Update helper function**

Replace `is_independent_set_config` to use `G: Graph`:

```rust
fn is_independent_set_config<G: Graph>(graph: &G, config: &[usize]) -> bool {
    for (u, v) in graph.edges() {
        if config.get(u).copied().unwrap_or(0) == 1 && config.get(v).copied().unwrap_or(0) == 1 {
            return false;
        }
    }
    true
}
```

**Step 7: Update tests**

Change all test instances from `IndependentSet::<i32>::new(...)` to `IndependentSet::<SimpleGraph, i32>::new(...)`.

**Step 8: Run tests**

```bash
cargo test --lib independent_set
```
Expected: PASS

**Step 9: Commit**

```bash
git add src/models/graph/independent_set.rs
git commit -m "refactor(models): add graph type parameter to IndependentSet<G, W>"
```

---

## Task 3: Refactor VertexCovering<W> to VertexCovering<G, W>

**Files:**
- Modify: `src/models/graph/vertex_covering.rs`

Follow same pattern as Task 2. Key changes:

1. Add `G: Graph` parameter
2. Replace `UnGraph<(), ()>` with `G`
3. Add `from_graph` constructors
4. Update `variant()` to use `G::NAME`
5. Update tests to use `VertexCovering::<SimpleGraph, i32>`

**Commit:**
```bash
git add src/models/graph/vertex_covering.rs
git commit -m "refactor(models): add graph type parameter to VertexCovering<G, W>"
```

---

## Task 4: Refactor DominatingSet<W> to DominatingSet<G, W>

**Files:**
- Modify: `src/models/graph/dominating_set.rs`

Same pattern as Task 2.

**Commit:**
```bash
git add src/models/graph/dominating_set.rs
git commit -m "refactor(models): add graph type parameter to DominatingSet<G, W>"
```

---

## Task 5: Refactor MaximalIS<W> to MaximalIS<G, W>

**Files:**
- Modify: `src/models/graph/maximal_is.rs`

Same pattern as Task 2.

**Commit:**
```bash
git add src/models/graph/maximal_is.rs
git commit -m "refactor(models): add graph type parameter to MaximalIS<G, W>"
```

---

## Task 6: Refactor MaxCut<W> to MaxCut<G, W>

**Files:**
- Modify: `src/models/graph/max_cut.rs`

Note: MaxCut uses edge weights, not vertex weights. Structure:

```rust
pub struct MaxCut<G, W> {
    graph: G,
    edge_weights: Vec<W>,
}
```

**Commit:**
```bash
git add src/models/graph/max_cut.rs
git commit -m "refactor(models): add graph type parameter to MaxCut<G, W>"
```

---

## Task 7: Refactor Matching<W> to Matching<G, W>

**Files:**
- Modify: `src/models/graph/matching.rs`

Same pattern, uses edge weights.

**Commit:**
```bash
git add src/models/graph/matching.rs
git commit -m "refactor(models): add graph type parameter to Matching<G, W>"
```

---

## Task 8: Rename Coloring to KColoring<K, G, W>

**Files:**
- Rename: `src/models/graph/coloring.rs` → `src/models/graph/kcoloring.rs`
- Modify: `src/models/graph/mod.rs`

**Step 1: Create new file with const generic K**

```rust
// src/models/graph/kcoloring.rs
use crate::topology::Graph;
use crate::traits::{ConstraintSatisfactionProblem, Problem};
use crate::variant::short_type_name;
// ...

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KColoring<const K: usize, G, W> {
    graph: G,
    weights: Vec<W>,
}

impl<const K: usize, W: Clone + Default> KColoring<K, SimpleGraph, W> {
    pub fn new(num_vertices: usize, edges: Vec<(usize, usize)>) -> Self
    where
        W: From<i32>,
    {
        let graph = SimpleGraph::new(num_vertices, edges);
        let weights = vec![W::from(1); num_vertices];
        Self { graph, weights }
    }
}

impl<const K: usize, G: Graph, W: Clone + Default> KColoring<K, G, W> {
    pub fn num_colors(&self) -> usize {
        K
    }
    // ...
}
```

**Step 2: Add const_str helper for K**

In `src/variant.rs` or inline:

```rust
/// Convert const generic to static str (for common values).
pub const fn const_usize_str<const N: usize>() -> &'static str {
    match N {
        1 => "1", 2 => "2", 3 => "3", 4 => "4", 5 => "5",
        6 => "6", 7 => "7", 8 => "8", 9 => "9", 10 => "10",
        _ => "N",
    }
}
```

**Step 3: Update variant() for KColoring**

```rust
impl<const K: usize, G, W> Problem for KColoring<K, G, W>
where
    G: Graph,
    W: Clone + Default + PartialOrd + num_traits::Num + 'static,
{
    const NAME: &'static str = "KColoring";

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("k", const_usize_str::<K>()),
            ("graph", G::NAME),
            ("weight", short_type_name::<W>()),
        ]
    }

    fn num_flavors(&self) -> usize {
        K
    }
    // ...
}
```

**Step 4: Update mod.rs**

```rust
mod kcoloring;
pub use kcoloring::{is_valid_coloring, KColoring};
```

**Step 5: Delete old coloring.rs**

```bash
rm src/models/graph/coloring.rs
```

**Step 6: Run tests**

```bash
cargo test --lib kcoloring
```

**Step 7: Commit**

```bash
git add src/models/graph/
git commit -m "refactor(models): rename Coloring to KColoring<K, G, W> with const generic"
```

---

## Task 9: Refactor SpinGlass<W> to SpinGlass<G, W>

**Files:**
- Modify: `src/models/optimization/spin_glass.rs`

SpinGlass stores interactions as `Vec<((usize, usize), W)>`. Change to use graph:

```rust
pub struct SpinGlass<G, W> {
    graph: G,
    couplings: Vec<W>,  // One per edge, matching graph.edges() order
    fields: Vec<W>,
}
```

**Commit:**
```bash
git add src/models/optimization/spin_glass.rs
git commit -m "refactor(models): add graph type parameter to SpinGlass<G, W>"
```

---

## Task 10: Update reduction IS ↔ VC

**Files:**
- Modify: `src/rules/vertexcovering_independentset.rs`

**Step 1: Update type signatures**

```rust
pub struct ReductionISToVC<G, W> {
    target: VertexCovering<G, W>,
    source_size: ProblemSize,
}

impl<G, W> ReductionResult for ReductionISToVC<G, W>
where
    G: Graph,
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign + 'static,
{
    type Source = IndependentSet<G, W>;
    type Target = VertexCovering<G, W>;
    // ...
}

impl<G, W> ReduceTo<VertexCovering<G, W>> for IndependentSet<G, W>
where
    G: Graph + Clone,
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign + From<i32> + 'static,
{
    // ...
}
```

**Step 2: Update tests**

Use explicit types: `IndependentSet::<SimpleGraph, i32>::new(...)`.

**Step 3: Run tests**

```bash
cargo test --lib vertexcovering_independentset
```

**Step 4: Commit**

```bash
git add src/rules/vertexcovering_independentset.rs
git commit -m "refactor(rules): update IS-VC reductions for graph type parameter"
```

---

## Task 11: Update remaining reduction files

**Files to modify:** (same pattern as Task 10)
- `src/rules/independentset_setpacking.rs`
- `src/rules/vertexcovering_setcovering.rs`
- `src/rules/matching_setpacking.rs`
- `src/rules/sat_independentset.rs`
- `src/rules/sat_dominatingset.rs`
- `src/rules/sat_coloring.rs` → update to `KColoring<3, SimpleGraph, W>`
- `src/rules/spinglass_maxcut.rs`
- `src/rules/spinglass_qubo.rs`
- `src/rules/circuit_spinglass.rs`
- `src/rules/independentset_ilp.rs`
- `src/rules/vertexcovering_ilp.rs`
- `src/rules/dominatingset_ilp.rs`
- `src/rules/matching_ilp.rs`
- `src/rules/coloring_ilp.rs` → `kcoloring_ilp.rs`
- `src/rules/clique_ilp.rs`

For each file:
1. Update struct definitions to include `G` parameter
2. Update `impl ReduceTo<...>` with graph bounds
3. Update tests to use explicit types
4. Run tests
5. Commit

**Commit after each file or batch:**
```bash
git commit -m "refactor(rules): update <reduction> for graph type parameter"
```

---

## Task 12: Remove template-based types

**Files:**
- Modify: `src/models/graph/template.rs` (delete most content)
- Modify: `src/models/graph/mod.rs` (remove exports)

**Step 1: Remove type aliases and GraphConstraint**

Delete:
- `GraphProblem<C, G, W>` struct
- `GraphConstraint` trait
- `IndependentSetT`, `VertexCoverT`, `CliqueT` type aliases
- `IndependentSetConstraint`, `VertexCoverConstraint`, `CliqueConstraint`

**Step 2: Keep any utility functions if used elsewhere**

Check if anything in template.rs is imported elsewhere.

**Step 3: Update mod.rs**

Remove template exports.

**Step 4: Run full tests**

```bash
cargo test
```

**Step 5: Commit**

```bash
git add src/models/graph/
git commit -m "refactor(models): remove template-based graph problem types"
```

---

## Task 13: Update documentation

**Files:**
- Modify: `docs/paper/reductions.typ`
- Modify: `README.md`
- Modify: doc comments in source files

**Step 1: Update type definitions in paper**

Replace all code blocks showing old signatures.

**Step 2: Regenerate reduction graph**

```bash
make export-graph
```

**Step 3: Build paper**

```bash
make paper
```

**Step 4: Commit**

```bash
git add docs/ README.md
git commit -m "docs: update for consistent type parameterization"
```

---

## Task 14: Final verification

**Step 1: Run all tests**

```bash
make test
```

**Step 2: Run clippy**

```bash
make clippy
```

**Step 3: Check coverage**

```bash
make coverage
```
Expected: >95%

**Step 4: Run export-graph**

```bash
make export-graph
```

**Step 5: Final commit if any fixes**

```bash
git add .
git commit -m "fix: address issues from final verification"
```

---

## Summary

| Task | Files | Estimated Complexity |
|------|-------|---------------------|
| 1 | topology/*.rs | Low |
| 2 | independent_set.rs | Medium |
| 3 | vertex_covering.rs | Medium |
| 4 | dominating_set.rs | Medium |
| 5 | maximal_is.rs | Medium |
| 6 | max_cut.rs | Medium |
| 7 | matching.rs | Medium |
| 8 | coloring.rs → kcoloring.rs | Medium |
| 9 | spin_glass.rs | Medium |
| 10 | vertexcovering_independentset.rs | Medium |
| 11 | ~15 reduction files | High (repetitive) |
| 12 | template.rs removal | Low |
| 13 | docs | Medium |
| 14 | verification | Low |

**Total: ~25 files, mostly mechanical changes**
