# Plan: Add MinimumFeedbackArcSet Model (#213)

## Overview

Add the `MinimumFeedbackArcSet` problem model. This requires first adding a `DirectedGraph` topology type since the codebase currently only has undirected graph types.

## Prerequisite: DirectedGraph topology type

### Step P1: Create `src/topology/directed_graph.rs`

- Create `DirectedGraph` struct wrapping petgraph's `DiGraph<(), ()>`
- Constructor: `new(num_vertices, arcs)` where arcs are `(usize, usize)` directed pairs
- Methods: `num_vertices()`, `num_arcs()`, `arcs()`, `has_arc(u, v)`, `successors(v)`, `predecessors(v)`
- Implement `Serialize`/`Deserialize`, `Clone`, `Debug`, `PartialEq`, `Eq`
- Register as `VariantParam` with category `"graph"` (no parent — directed graphs are not a subtype of undirected)
- NOTE: Do NOT implement the `Graph` trait (which is for undirected graphs). DirectedGraph has its own API.

### Step P2: Register DirectedGraph in topology/mod.rs

- Add `mod directed_graph;` and `pub use directed_graph::DirectedGraph;`

### Step P3: Write unit tests for DirectedGraph

- Create `src/unit_tests/topology/directed_graph.rs`
- Test construction, arc queries, successor/predecessor queries, serialization round-trip
- Link via `#[cfg(test)] #[path]` in the source file

## Model Implementation (add-model steps)

### Step 1: Create `src/models/graph/minimum_feedback_arc_set.rs`

**Problem definition:**
- `MinimumFeedbackArcSet<G>` where `G` is a directed graph type
- Single field: `graph: G`
- Optimization (Minimize), `Metric = SolutionSize<i32>`
- Variables: m = num_arcs binary variables (one per arc)
- `dims()` -> `vec![2; self.graph.num_arcs()]`
- `evaluate()`: check if selected arcs form a valid FAS (removing them makes DAG), return count
- Feasibility check: for each arc, if x_a=1 it's removed; verify remaining graph is acyclic (DFS-based cycle detection)
- `direction()` -> `Direction::Minimize`
- Getter methods: `num_vertices()`, `num_arcs()`

**Complexity:** `2^num_vertices` (best known exact: DP over vertex subsets, O*(2^n))

### Step 2: Register in module hierarchy

- `src/models/graph/mod.rs`: add `pub(crate) mod minimum_feedback_arc_set;` and `pub use`
- `src/models/mod.rs`: add to graph re-exports
- `src/lib.rs` prelude: add `MinimumFeedbackArcSet`

### Step 3: Register variant complexity

```rust
crate::declare_variants! {
    MinimumFeedbackArcSet<DirectedGraph> => "2^num_vertices",
}
```

### Step 4: Register in CLI

- `problemreductions-cli/src/dispatch.rs`: add match arms in `load_problem()` and `serialize_any_problem()`
- `problemreductions-cli/src/problem_name.rs`: add `"minimumfeedbackarcset"` alias in `resolve_alias()`
- `problemreductions-cli/src/commands/create.rs`: add creation handler using `--arcs` flag for directed arc list
- `problemreductions-cli/src/cli.rs`: add `--arcs` flag to `CreateArgs`, update `all_data_flags_empty()`, update help table

### Step 5: Write unit tests

Create `src/unit_tests/models/graph/minimum_feedback_arc_set.rs`:
- `test_minimum_feedback_arc_set_creation`: construct instance, verify dimensions
- `test_minimum_feedback_arc_set_evaluation`: verify evaluate on valid/invalid configs
- `test_minimum_feedback_arc_set_direction`: verify Minimize
- `test_minimum_feedback_arc_set_solver`: verify brute-force finds correct solution
- Use the example from the issue: 6 vertices, 9 arcs, optimal FAS = {(0->1), (3->4)}, size 2

### Step 6: Document in paper

Add problem-def entry to `docs/paper/reductions.typ`:
- Add `"MinimumFeedbackArcSet"` to `display-name` dict
- Write `#problem-def("MinimumFeedbackArcSet")[...]` with formal definition

### Step 7: Verify

```bash
make check  # fmt + clippy + test
```
