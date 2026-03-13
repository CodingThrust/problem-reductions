# Plan: Add BiconnectivityAugmentation Model

**Issue:** #230
**Type:** [Model]
**Skill:** add-model

## Information Checklist

| # | Item | Value |
|---|------|-------|
| 1 | Problem name | `BiconnectivityAugmentation` |
| 2 | Mathematical definition | Given graph G=(V,E), weighted potential edges, budget B. Is there E' with sum(w(e)) <= B such that G'=(V, E union E') is biconnected? |
| 3 | Problem type | Satisfaction (Metric = bool) |
| 4 | Type parameters | `G: Graph`, `W: WeightElement` |
| 5 | Struct fields | `graph: G`, `potential_weights: Vec<(usize, usize, W)>`, `budget: W::Sum` |
| 6 | Configuration space | `vec![2; self.potential_weights.len()]` — binary per potential edge |
| 7 | Feasibility check | G' = (V, E union selected_edges) is biconnected AND total weight <= budget |
| 8 | Objective function | N/A (satisfaction) — returns true if feasible |
| 9 | Best known exact algorithm | O*(2^num_potential_edges) brute-force (Eswaran & Tarjan 1976) |
| 10 | Solving strategy | BruteForce enumeration over subsets |
| 11 | Category | `graph` |

## Implementation Steps

### Step 1: Implement the model (`src/models/graph/biconnectivity_augmentation.rs`)

- `inventory::submit!` for `ProblemSchemaEntry`
- Struct `BiconnectivityAugmentation<G, W>` with fields: `graph: G`, `potential_weights: Vec<(usize, usize, W)>`, `budget: W::Sum`
- Constructor `new(graph, potential_weights, budget)` — validate potential edge indices are in range
- Getters: `graph()`, `potential_weights()`, `budget()`, `num_vertices()`, `num_edges()`, `num_potential_edges()`
- `is_weighted()` inherent method
- `Problem` impl: NAME = "BiconnectivityAugmentation", Metric = bool, dims = vec![2; potential_weights.len()]
- `evaluate()`: build augmented graph, check biconnectivity (no articulation points) AND weight sum <= budget
- `SatisfactionProblem` impl (marker)
- `declare_variants!` with `BiconnectivityAugmentation<SimpleGraph, i32> => "2^num_potential_edges"`
- `#[cfg(test)] #[path = "..."] mod tests;`
- Biconnectivity check: implement as a helper function using Tarjan's algorithm to find articulation points

### Step 2: Register the model

- `src/models/graph/mod.rs` — add module and re-export
- `src/models/mod.rs` — add to re-export line

### Step 3: Register in CLI

- `problemreductions-cli/src/dispatch.rs` — add `deser_sat::<BiconnectivityAugmentation<SimpleGraph, i32>>` match arm in both `load_problem` and `serialize_any_problem`
- `problemreductions-cli/src/problem_name.rs` — add `"biconnectivityaugmentation"` alias in `resolve_alias()`

### Step 4: Add CLI creation support

- `problemreductions-cli/src/commands/create.rs` — add match arm parsing `--graph`, `--potential-edges`, `--budget`
- `problemreductions-cli/src/cli.rs` — add `--potential-edges` and `--budget` flags to `CreateArgs`, update `all_data_flags_empty()` and help table

### Step 5: Write unit tests (`src/unit_tests/models/graph/biconnectivity_augmentation.rs`)

- `test_biconnectivity_augmentation_creation` — construct instance, verify dimensions
- `test_biconnectivity_augmentation_evaluation` — verify evaluate on valid/invalid configs
- `test_biconnectivity_augmentation_serialization` — round-trip serde
- `test_biconnectivity_augmentation_solver` — BruteForce finds correct satisfying configs
- `test_biconnectivity_augmentation_no_solution` — verify unsatisfiable instance
- `test_is_biconnected` — test the helper function directly

### Step 6: Document in paper

- Add `display-name` entry: `"BiconnectivityAugmentation": [Biconnectivity Augmentation]`
- Add `problem-def("BiconnectivityAugmentation")` with formal definition and background

### Step 7: Verify

- `make test clippy` must pass
