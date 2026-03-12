# Plan: Add IsomorphicSpanningTree Model (#244)

## Overview

Add the `IsomorphicSpanningTree` satisfaction problem: given a graph G and a tree T with |V(G)| = |V(T)|, does G contain a spanning tree isomorphic to T?

This is a **SatisfactionProblem** (Metric = bool). The configuration encodes a permutation π: V(T) → V(G) as n variables each with domain {0, 1, ..., n-1}.

## Steps

### Step 1: Implement the model

**File:** `src/models/graph/isomorphic_spanning_tree.rs`

**Struct:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IsomorphicSpanningTree {
    graph: SimpleGraph,
    tree: SimpleGraph,
}
```

**Fields:**
- `graph`: The host graph G
- `tree`: The target tree T (must be a tree with same vertex count as G)

**Constructor:** `new(graph: SimpleGraph, tree: SimpleGraph)` — panics if `|V(G)| != |V(T)|` or if T is not a tree (not connected or wrong edge count).

**Getter methods:**
- `graph() -> &SimpleGraph`
- `tree() -> &SimpleGraph`
- `num_vertices() -> usize` (= |V(G)|)
- `num_graph_edges() -> usize` (= |E(G)|)
- `num_tree_edges() -> usize` (= |E(T)|, always n-1)

**Problem impl:**
- `NAME = "IsomorphicSpanningTree"`
- `Metric = bool`
- `dims()`: `vec![n; n]` — n variables, each with domain size n (encoding a permutation)
- `evaluate(config)`: Check that config is a valid permutation (all distinct values in 0..n), then check that for every edge {u,v} in T, {π(u), π(v)} is an edge in G.
- `variant()`: `crate::variant_params![]` (no type parameters)

**SatisfactionProblem impl:** marker trait, empty.

**declare_variants!:**
```rust
crate::declare_variants! {
    IsomorphicSpanningTree => "num_vertices^num_vertices",
}
```
Complexity: O(n^n) brute-force over all mappings (n! permutations, each checked in O(n) time). The naive upper bound; no faster general algorithm is known.

**inventory::submit!** with ProblemSchemaEntry for schema registration.

**Test link:** `#[cfg(test)] #[path = "../../unit_tests/models/graph/isomorphic_spanning_tree.rs"] mod tests;`

### Step 2: Register the model

**`src/models/graph/mod.rs`:**
- Add `pub(crate) mod isomorphic_spanning_tree;`
- Add `pub use isomorphic_spanning_tree::IsomorphicSpanningTree;`
- Add to module doc comment

**`src/models/mod.rs`:**
- Add `IsomorphicSpanningTree` to the graph re-export line

### Step 3: Register in CLI

**`problemreductions-cli/src/dispatch.rs`:**
- In `load_problem()`: add `"IsomorphicSpanningTree" => deser_sat::<IsomorphicSpanningTree>(data)`
- In `serialize_any_problem()`: add `"IsomorphicSpanningTree" => try_ser::<IsomorphicSpanningTree>(any)`

**`problemreductions-cli/src/problem_name.rs`:**
- In `resolve_alias()`: add `"isomorphicspanningtree" => "IsomorphicSpanningTree".to_string()`
- No short alias (no established abbreviation in literature)

### Step 4: Add CLI creation support

**`problemreductions-cli/src/commands/create.rs`:**
- Add a new match arm for `"IsomorphicSpanningTree"` that requires `--graph` and a `--tree` flag
- The `--tree` flag takes edges as comma-separated pairs (same format as `--graph`)

**`problemreductions-cli/src/cli.rs`:**
- Add `--tree` flag to `CreateArgs` (edges for the target tree)
- Update help table with IsomorphicSpanningTree entry

### Step 5: Write unit tests

**File:** `src/unit_tests/models/graph/isomorphic_spanning_tree.rs`

Tests:
1. `test_isomorphicspanningtree_basic` — Create instance with 4-vertex graph and path tree, verify dims = [4,4,4,4], verify evaluate returns true for valid permutation mapping
2. `test_isomorphicspanningtree_no_solution` — Star tree K_{1,3} on a cycle graph C_4: no vertex has degree 3, so answer is NO
3. `test_isomorphicspanningtree_evaluation` — Test evaluate returns false for non-permutation configs and for permutations that don't map tree edges to graph edges
4. `test_isomorphicspanningtree_serialization` — serde_json round-trip
5. `test_isomorphicspanningtree_solver` — Use BruteForce::find_all_satisfying on a small instance, verify all solutions are valid

### Step 6: Document in paper

Add `problem-def("IsomorphicSpanningTree")` in `docs/paper/reductions.typ` with:
- Display name: "Isomorphic Spanning Tree"
- Formal definition
- Background (generalizes Hamiltonian Path)
- Example using the caterpillar tree instance from the issue

### Step 7: Verify

- `make check` (fmt + clippy + test)
- `make export-schemas` to regenerate schemas
