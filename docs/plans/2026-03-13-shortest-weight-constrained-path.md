# Plan: Add ShortestWeightConstrainedPath Model

**Issue:** #289
**Type:** [Model]
**Skill:** add-model

## Summary

Add the ShortestWeightConstrainedPath satisfaction problem: given a graph with edge lengths and weights, source/target vertices, and bounds K and W, determine if a simple s-t path exists with total length <= K and total weight <= W.

## Steps

### Step 1: Implement the model
- File: `src/models/graph/shortest_weight_constrained_path.rs`
- Satisfaction problem: `Metric = bool`, implement `SatisfactionProblem`
- Type params: `G: Graph`, `N: NumericSize` (numeric type for lengths/weights)
- Fields: graph, lengths, weights, source, target, length_bound, weight_bound
- `dims()`: `vec![2; num_edges]`
- `evaluate()`: check selected edges form simple s-t path, total length <= K, total weight <= W
- `variant()`: `variant_params![G, N]`
- `declare_variants!`: `ShortestWeightConstrainedPath<SimpleGraph, i32> => "2^num_edges"`
- Getters: `num_vertices()`, `num_edges()`

### Step 2: Register the model
- `src/models/graph/mod.rs`: add module + pub use
- `src/models/mod.rs`: add re-export
- `src/lib.rs` prelude: add to prelude

### Step 3: Register in CLI
- `problemreductions-cli/src/dispatch.rs`: add `deser_sat` arms
- `problemreductions-cli/src/problem_name.rs`: add alias
- `problemreductions-cli/src/commands/create.rs`: add create handler
- `problemreductions-cli/src/cli.rs`: add to help table, add new flags (--lengths, --source, --target-vertex, --length-bound, --weight-bound)

### Step 4: Write unit tests
- File: `src/unit_tests/models/graph/shortest_weight_constrained_path.rs`
- Tests: creation, evaluation (valid/invalid paths), brute force solver, serialization

### Step 5: Document in paper
- Add display-name entry
- Add problem-def entry in `docs/paper/reductions.typ`

### Step 6: Verify
- `make test clippy`
