# Plan: PartitionIntoPathsOfLength2 -> BoundedComponentSpanningForest

Fixes #241

## Context

This is a parameter-setting reduction from Hadlock (1974), referenced in Garey & Johnson ND10.
The reduction copies the graph as-is, sets all vertex weights to 1, K = |V|/3, B = 3.
A P3-partition (each triple has >= 2 edges) iff a BCSF solution with unit weights, K components of size exactly 3.

Both models already exist in the codebase:
- `src/models/graph/partition_into_paths_of_length_2.rs` (feasibility, `Value = Or`)
- `src/models/graph/bounded_component_spanning_forest.rs` (feasibility, `Value = Or`, parameterized by `<G, W>`)

## Batch 1: Implement reduction, register, tests, example_db

### Step 1: Implement the reduction rule

File: `src/rules/partitionintopathsoflength2_boundedcomponentspanningforest.rs`

Following add-rule Step 1:

1. **ReductionResult struct** `ReductionPIPL2ToBCSF` holding the target `BoundedComponentSpanningForest<SimpleGraph, i32>`.
2. **ReductionResult trait impl**: `extract_solution` maps BCSF config (component indices 0..K-1 for each vertex) back to PPL2 config (group indices 0..q-1). Since both are feasibility problems with the same graph and both assign each vertex to a group/component, the config is identity.
3. **`#[reduction]` with overhead**:
   - `num_vertices = "num_vertices"`
   - `num_edges = "num_edges"`
   - `max_components = "num_vertices / 3"`
4. **`ReduceTo` impl**: Copy graph, set unit weights `vec![1i32; n]`, K = n/3, B = 3.
5. **Test module link** at bottom.

### Step 2: Register in mod.rs

Add `pub(crate) mod partitionintopathsoflength2_boundedcomponentspanningforest;` to `src/rules/mod.rs`.

### Step 3: Write unit tests

File: `src/unit_tests/rules/partitionintopathsoflength2_boundedcomponentspanningforest.rs`

Tests:
- `test_partitionintopathsoflength2_to_boundedcomponentspanningforest_closed_loop`: 9-vertex graph from the issue example (3 valid P3 triples). Use `assert_satisfaction_round_trip_from_satisfaction_target`.
- `test_partitionintopathsoflength2_to_boundedcomponentspanningforest_structure`: Verify target has correct K=3, B=3, unit weights.
- `test_partitionintopathsoflength2_to_boundedcomponentspanningforest_no_solution`: Star graph K_{1,5} on 6 vertices -- no valid P3 partition. Verify target is also unsatisfiable.
- `test_partitionintopathsoflength2_to_boundedcomponentspanningforest_small`: 6-vertex path graph 0-1-2-3-4-5 with extra edges (0,1),(1,2),(3,4),(4,5) forming two P3 paths.

### Step 4: Add canonical example to example_db

Add `canonical_rule_example_specs()` in the rule file and register in `src/rules/mod.rs` `canonical_rule_example_specs()`.

Example: 9-vertex graph from issue with known partition {0,1,2}, {3,4,5}, {6,7,8}.

## Batch 2: Paper entry, exports, verification

### Step 5: Document in paper

Add `reduction-rule("PartitionIntoPathsOfLength2", "BoundedComponentSpanningForest", ...)` in `docs/paper/reductions.typ` near the existing problem-def entries.

### Step 6: Regenerate exports and verify

```bash
cargo run --example export_graph
cargo run --example export_schemas
make regenerate-fixtures
make test clippy
make paper
```
