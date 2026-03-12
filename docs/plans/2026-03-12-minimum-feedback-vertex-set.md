# Plan: Add MinimumFeedbackVertexSet Model (#140)

## Overview

Add the MinimumFeedbackVertexSet problem — one of Karp's 21 NP-complete problems (GT7). This requires new DirectedGraph topology infrastructure since the problem operates on directed graphs, which don't exist in the codebase yet.

**Problem:** Given a directed graph G = (V, A) with vertex weights, find minimum-weight S ⊆ V such that G[V \ S] is a DAG.

## Batch 1: DirectedGraph Topology (independent)

### Task 1.1: Create `src/topology/directed_graph.rs`

New directed graph struct wrapping `petgraph::graph::DiGraph<(), ()>`.

**Struct:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectedGraph {
    inner: DiGraph<(), ()>,
}
```

**Methods:**
- `new(num_vertices: usize, arcs: Vec<(usize, usize)>) -> Self` — constructor with arc validation
- `empty(num_vertices: usize) -> Self`
- `num_vertices(&self) -> usize`
- `num_arcs(&self) -> usize`
- `arcs(&self) -> Vec<(usize, usize)>` — returns all arcs as (source, target) pairs
- `has_arc(&self, u: usize, v: usize) -> bool` — check if arc u→v exists
- `successors(&self, v: usize) -> Vec<usize>` — outgoing neighbors
- `predecessors(&self, v: usize) -> Vec<usize>` — incoming neighbors
- `is_dag(&self) -> bool` — cycle detection via topological sort (using petgraph's `toposort`)
- `induced_subgraph(&self, keep: &[bool]) -> Self` — subgraph on vertices where keep[v] == true (remaps vertex indices)

**Does NOT implement the `Graph` trait** (which is for undirected graphs with u < v edge semantics).

**Implements:**
- `PartialEq`, `Eq` (normalize and compare arc sets)
- `VariantParam` via `impl_variant_param!(DirectedGraph, "graph")`

**Tests:** `src/unit_tests/topology/directed_graph.rs` linked via `#[cfg(test)] #[path]`

Test cases:
- Construction and basic queries (num_vertices, num_arcs, arcs, has_arc)
- successors/predecessors correctness
- is_dag: true for DAG, false for graph with cycle
- induced_subgraph: verify removing vertices breaks cycles
- PartialEq: same graph in different arc order should be equal
- Serialization round-trip

### Task 1.2: Register DirectedGraph in `src/topology/mod.rs`

Add module declaration and re-export:
```rust
mod directed_graph;
pub use directed_graph::DirectedGraph;
```

## Batch 2: MinimumFeedbackVertexSet Model (depends on Batch 1)

### Task 2.1: Create `src/models/graph/minimum_feedback_vertex_set.rs`

Follow MinimumDominatingSet pattern but with DirectedGraph instead of generic G.

**Schema registration:**
```rust
inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumFeedbackVertexSet",
        module_path: module_path!(),
        description: "Find minimum weight feedback vertex set in a directed graph",
        fields: &[
            FieldInfo { name: "graph", type_name: "DirectedGraph", description: "The directed graph G=(V,A)" },
            FieldInfo { name: "weights", type_name: "Vec<W>", description: "Vertex weights w: V -> R" },
        ],
    }
}
```

**Struct:** `MinimumFeedbackVertexSet<W>` with fields `graph: DirectedGraph`, `weights: Vec<W>`

**Constructor & getters:**
- `new(graph: DirectedGraph, weights: Vec<W>)` — assert weights.len() == num_vertices
- `graph(&self) -> &DirectedGraph`
- `weights(&self) -> &[W]`
- `is_weighted(&self) -> bool` — via `W::IS_UNIT`

**NumericSize getters** (for overhead expressions):
- `num_vertices(&self) -> usize` — graph.num_vertices()
- `num_arcs(&self) -> usize` — graph.num_arcs()

**evaluate logic:**
1. Build induced subgraph on vertices where `config[v] == 0` (not selected for removal)
2. Check if induced subgraph `is_dag()` — if not, return `SolutionSize::Invalid`
3. Sum weights of selected vertices (config[v] == 1) → `SolutionSize::Valid(total)`

**Trait impls:**
- `Problem` with `NAME = "MinimumFeedbackVertexSet"`, `Metric = SolutionSize<W::Sum>`, `variant() = variant_params![W]`, `dims() = vec![2; n]`
- `OptimizationProblem` with `Value = W::Sum`, `direction() = Minimize`

**Variant complexity:**
```rust
crate::declare_variants! {
    MinimumFeedbackVertexSet<i32> => "1.8638^num_vertices",
}
```
Based on Razgon (2007), "Computing Minimum Directed Feedback Vertex Set in O*(1.9977^n)". Note: the issue cites 1.8638^n which comes from later improvements. Use the issue's value.

**Helper function:**
```rust
#[cfg(test)]
pub(crate) fn is_feedback_vertex_set(graph: &DirectedGraph, selected: &[bool]) -> bool
```

**Test link:** `#[cfg(test)] #[path = "../../unit_tests/models/graph/minimum_feedback_vertex_set.rs"] mod tests;`

### Task 2.2: Register in module hierarchy

1. `src/models/graph/mod.rs` — add `pub(crate) mod minimum_feedback_vertex_set;` and `pub use minimum_feedback_vertex_set::MinimumFeedbackVertexSet;`
2. `src/models/mod.rs` — add `MinimumFeedbackVertexSet` to graph re-exports
3. `src/lib.rs` prelude — add `MinimumFeedbackVertexSet` to prelude exports

### Task 2.3: Write unit tests

Create `src/unit_tests/models/graph/minimum_feedback_vertex_set.rs`:

Test cases:
- `test_minimum_feedback_vertex_set_basic` — create instance with issue's example (9 vertices, 15 arcs), verify dims=[2;9], evaluate valid FVS {0,3,8} returns Valid(3), evaluate invalid subset returns Invalid
- `test_minimum_feedback_vertex_set_direction` — verify Minimize
- `test_minimum_feedback_vertex_set_serialization` — round-trip serde
- `test_minimum_feedback_vertex_set_solver` — brute force finds optimal FVS of size 3 for the example
- `test_minimum_feedback_vertex_set_empty_set` — empty set is only valid FVS if graph is already a DAG
- `test_minimum_feedback_vertex_set_trivial` — selecting all vertices is always valid (but not optimal)

## Batch 3: CLI Registration (depends on Batch 2)

### Task 3.1: Update `problemreductions-cli/src/dispatch.rs`

Add imports and match arms:
- `load_problem`: `"MinimumFeedbackVertexSet" => deser_opt::<MinimumFeedbackVertexSet<i32>>(data)`
- `serialize_any_problem`: `try_ser::<MinimumFeedbackVertexSet<i32>>`

### Task 3.2: Update `problemreductions-cli/src/problem_name.rs`

Add lowercase alias: `"minimumfeedbackvertexset" => "MinimumFeedbackVertexSet"`
Add standard abbreviation: `("FVS", "MinimumFeedbackVertexSet")` to ALIASES array (FVS is well-established in the literature).

### Task 3.3: Update `problemreductions-cli/src/commands/create.rs`

Add a new match arm for MinimumFeedbackVertexSet:
- Parse `--arcs` flag (new flag for directed edges, format: "0>1,1>2,2>0")
- Parse `--weights` flag (reuse existing)
- Parse `--num-vertices` for `--random` mode
- Construct `DirectedGraph` and `MinimumFeedbackVertexSet`

### Task 3.4: Update `problemreductions-cli/src/cli.rs`

1. Add `--arcs` flag to `CreateArgs`: `pub arcs: Option<String>` with help "Directed arcs (e.g., 0>1,1>2,2>0)"
2. Update `all_data_flags_empty()` to include `args.arcs.is_none()`
3. Add to "Flags by problem type" help table: `MinFVS/FVS                      --arcs, --weights`

## Batch 4: Verification (depends on all above)

### Task 4.1: Run `make check`

Run `make fmt && make clippy && make test` — all must pass.

### Task 4.2: Run `make export-schemas`

Regenerate problem schemas to include the new problem type.
