# Plan: MinimumVertexCover -> EnsembleComputation

Fixes #204

## Context

Implement the reduction from MinimumVertexCover to EnsembleComputation, following Garey & Johnson (1979) Appendix Problem PO9.

- **Source:** `MinimumVertexCover<SimpleGraph, i32>` (optimization, `Value = Min<i32>`)
- **Target:** `EnsembleComputation` (feasibility, `Value = Or`)
- **Reference:** Garey & Johnson, *Computers and Intractability*, Appendix PO9

### Reduction Algorithm

Given MVC instance G = (V, E) with |V| = n, |E| = m:

1. Let a0 = n (fresh element not in V).
2. Universe A = V union {a0}, so universe_size = n + 1.
3. For each edge {u, v} in E, add subset {a0, u, v} to collection C. So num_subsets = m.
4. Budget J = n + m (upper bound: K <= n always holds).
5. The EC instance is satisfiable iff G has a vertex cover of size <= n (always true for non-empty graphs).

### Solution Extraction

From the EC satisfying sequence, extract the cover vertices:
- For each operation z_i = {a0} union {u}, vertex u is in the cover.
- Return binary config where selected[u] = 1 if u appeared in such an operation.

Note: Since budget = n + m is an upper bound, the extracted cover is valid but not necessarily minimum. This is acceptable for a witness-preserving reduction from optimization to feasibility.

### Overhead

- `universe_size = "num_vertices + 1"`
- `num_subsets = "num_edges"`

### Example (C5 cycle)

Source: C5 with vertices {0,1,2,3,4}, edges {0,1},{1,2},{2,3},{3,4},{4,0}
- a0 = 5, A = {0,1,2,3,4,5}, universe_size = 6
- C = {{5,0,1}, {5,1,2}, {5,2,3}, {5,3,4}, {5,0,4}}, num_subsets = 5
- budget = 10

## Batch 1: Implementation (add-rule Steps 1-4)

### Step 1: Create reduction file

File: `src/rules/minimumvertexcover_ensemblecomputation.rs`

- `ReductionVCToEC` struct holding `target: EnsembleComputation` and `num_vertices: usize`
- `ReductionResult` impl: `extract_solution` decodes the EC configuration to find operations of form z_i = {a0} union {u}, returning binary vector
- `#[reduction(overhead = { universe_size = "num_vertices + 1", num_subsets = "num_edges" })]`
- `ReduceTo<EnsembleComputation> for MinimumVertexCover<SimpleGraph, i32>`
- `canonical_rule_example_specs()` function

### Step 2: Register in mod.rs

Add `pub(crate) mod minimumvertexcover_ensemblecomputation;` to `src/rules/mod.rs`.
Add example_db registration in `canonical_rule_example_specs()`.

### Step 3: Write unit tests

File: `src/unit_tests/rules/minimumvertexcover_ensemblecomputation.rs`

Tests:
- `test_minimumvertexcover_to_ensemblecomputation_closed_loop`: C5 graph, reduce, solve target with BruteForce, extract solution, verify valid vertex cover using `assert_optimization_round_trip_from_satisfaction_target`
- `test_reduction_structure`: verify universe_size = n+1, num_subsets = m, budget = n+m
- `test_reduction_empty_graph`: empty graph (no edges) -> EC trivially satisfiable
- `test_reduction_triangle`: K3 graph, verify correctness

### Step 4: Add canonical example to example_db

Add builder in `src/rules/minimumvertexcover_ensemblecomputation.rs` using a small graph (triangle K3: 3V, 3E). Use `rule_example_with_witness` pattern.

Register in `src/rules/mod.rs` `canonical_rule_example_specs()`.

## Batch 2: Paper entry and exports (add-rule Steps 5-6)

### Step 5: Document in paper

Add `reduction-rule("MinimumVertexCover", "EnsembleComputation", ...)` entry in `docs/paper/reductions.typ` near other MVC reductions.

### Step 6: Regenerate exports and verify

```bash
cargo run --example export_graph
cargo run --example export_schemas
make regenerate-fixtures
make test clippy
```
