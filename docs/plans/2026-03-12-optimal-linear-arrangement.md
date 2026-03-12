# Plan: Add OptimalLinearArrangement Model (#406)

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

## Overview

Add the Optimal Linear Arrangement (OLA) problem as a graph optimization model. Given a graph G=(V,E), find a bijection f: V → {0,1,...,n-1} minimizing Σ_{(u,v)∈E} |f(u)-f(v)|.

**Reference:** Garey & Johnson, A1.3 GT42. NP-complete [Garey, Johnson, Stockmeyer 1976].

## Design Decisions

- **Optimization problem**: `Direction::Minimize`, `Metric = SolutionSize<i64>`
- **Config space**: n variables, each with domain size n (positions 0..n-1). `dims() = vec![n; n]`. Valid configs are permutations.
- **Type parameter**: Only `G: Graph` (no weight type — objective is purely structural)
- **Complexity**: `2^num_vertices` (Held-Karp-style DP, analogous to TSP)
- **No weight methods**: No `weights()`, `set_weights()`, `is_weighted()` — not applicable

## Tasks

### Task 1: Create model file

**File:** `src/models/graph/optimal_linear_arrangement.rs`

Follow `src/models/graph/traveling_salesman.rs` as reference.

#### Struct
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimalLinearArrangement<G> {
    graph: G,
}
```

#### Schema registration
```rust
inventory::submit! {
    ProblemSchemaEntry {
        name: "OptimalLinearArrangement",
        module_path: module_path!(),
        description: "Find vertex ordering minimizing total edge length (Optimal Linear Arrangement)",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The undirected graph G=(V,E)" },
        ],
    }
}
```

#### Inherent methods
- `new(graph: G) -> Self`
- `graph(&self) -> &G`
- `num_vertices(&self) -> usize` (under `G: Graph` bound, for overhead expressions)
- `num_edges(&self) -> usize` (under `G: Graph` bound, for overhead expressions)
- `is_valid_permutation(config: &[usize]) -> bool` — checks config has length n and is a permutation of 0..n-1
- `arrangement_cost(config: &[usize]) -> i64` — computes Σ|config[u] - config[v]| for all edges (u,v)

#### Problem trait
```rust
impl<G: Graph + crate::variant::VariantParam> Problem for OptimalLinearArrangement<G> {
    const NAME: &'static str = "OptimalLinearArrangement";
    type Metric = SolutionSize<i64>;
    fn variant() -> Vec<(&'static str, &'static str)> { crate::variant_params![G] }
    fn dims(&self) -> Vec<usize> { vec![self.graph.num_vertices(); self.graph.num_vertices()] }
    fn evaluate(&self, config: &[usize]) -> SolutionSize<i64> {
        if !self.is_valid_permutation(config) { return SolutionSize::Invalid; }
        SolutionSize::Valid(self.arrangement_cost(config))
    }
}
```

#### OptimizationProblem trait
```rust
impl<G: Graph + crate::variant::VariantParam> OptimizationProblem for OptimalLinearArrangement<G> {
    type Value = i64;
    fn direction(&self) -> Direction { Direction::Minimize }
}
```

#### Variant declaration
```rust
crate::declare_variants! {
    OptimalLinearArrangement<SimpleGraph> => "2^num_vertices",
}
```

#### Test module link
```rust
#[cfg(test)]
#[path = "../../unit_tests/models/graph/optimal_linear_arrangement.rs"]
mod tests;
```

### Task 2: Register in module hierarchy

1. `src/models/graph/mod.rs`: Add `pub(crate) mod optimal_linear_arrangement;` and `pub use optimal_linear_arrangement::OptimalLinearArrangement;`
2. `src/models/mod.rs`: Add `OptimalLinearArrangement` to the `pub use graph::{...}` line

### Task 3: Register in CLI

#### dispatch.rs
In `load_problem()` match:
```rust
"OptimalLinearArrangement" => deser_opt::<OptimalLinearArrangement<SimpleGraph>>(data),
```

In `serialize_any_problem()` match:
```rust
"OptimalLinearArrangement" => try_ser::<OptimalLinearArrangement<SimpleGraph>>(any),
```

Import `OptimalLinearArrangement` from the models at the top.

#### problem_name.rs
In `resolve_alias()` match add:
```rust
"optimallineararrangement" | "ola" => "OptimalLinearArrangement".to_string(),
```

In `ALIASES` const add:
```rust
("OLA", "OptimalLinearArrangement"),
```

#### commands/create.rs
Add `"OptimalLinearArrangement"` as a graph-only problem (no weights):
- In the main `match` in `create()`, add a new arm that parses `--graph` and creates `OptimalLinearArrangement::new(graph)`
- Add to `example_for()`: `"OptimalLinearArrangement" => "--graph 0-1,1-2,2-3,0-3"`
- Add to `create_random()` support for random graph generation

### Task 4: Write unit tests

**File:** `src/unit_tests/models/graph/optimal_linear_arrangement.rs`

Follow `src/unit_tests/models/graph/traveling_salesman.rs` as reference.

Tests to include:
1. `test_optimal_linear_arrangement_creation` — basic construction, dims, num_vertices
2. `test_evaluate_valid_permutation` — identity permutation on path graph 0-1-2-3-4, cost = 4
3. `test_evaluate_invalid_not_permutation` — duplicate positions → Invalid
4. `test_evaluate_invalid_out_of_range` — position >= n → Invalid
5. `test_evaluate_invalid_wrong_length` — wrong config length → Invalid
6. `test_direction` — Direction::Minimize
7. `test_problem_name` — NAME = "OptimalLinearArrangement"
8. `test_brute_force_path_graph` — path 0-1-2-3-4: optimal cost = 4 (identity arrangement)
9. `test_brute_force_issue_example` — 6-vertex graph from issue with 7 edges: verify optimal cost
10. `test_size_getters` — num_vertices, num_edges
11. `test_is_valid_permutation` — valid/invalid cases directly

### Task 5: Write paper entry

Add to `docs/paper/reductions.typ`:
- Add `"OptimalLinearArrangement": [Optimal Linear Arrangement]` to `display-name` dict
- Add `#problem-def("OptimalLinearArrangement")` entry with formal definition, background, example

### Task 6: Verify

Run `make check` (fmt + clippy + test). Fix any issues.
