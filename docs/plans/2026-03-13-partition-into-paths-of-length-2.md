# Plan: Add PartitionIntoPathsOfLength2 Model

**Issue:** #227
**Type:** Model
**Skill:** add-model

## Summary

Add the PartitionIntoPathsOfLength2 satisfaction problem: given a graph G = (V, E) with |V| = 3q, determine whether V can be partitioned into q disjoint triples such that each triple induces at least 2 edges (i.e., a path of length 2 or a triangle).

## Collected Information

| # | Item | Value |
|---|------|-------|
| 1 | Problem name | `PartitionIntoPathsOfLength2` |
| 2 | Mathematical definition | Given G=(V,E) with \|V\|=3q, partition V into q triples each inducing >= 2 edges |
| 3 | Problem type | Satisfaction (Metric = bool) |
| 4 | Type parameters | `G: Graph` (graph type parameter) |
| 5 | Struct fields | `graph: G` |
| 6 | Configuration space | `vec![q; n]` where q = n/3, n = num_vertices |
| 7 | Feasibility check | Each group has exactly 3 vertices, and each group induces >= 2 edges |
| 8 | Objective function | N/A (satisfaction: returns bool) |
| 9 | Best known exact algorithm | O(3^n) naive set-partition DP; no better exact algorithm known |
| 10 | Solving strategy | BruteForce works |
| 11 | Category | `graph` |

## Steps

### Step 1: Create model file
**File:** `src/models/graph/partition_into_paths_of_length_2.rs`

- `inventory::submit!` for ProblemSchemaEntry with field `graph`
- Struct `PartitionIntoPathsOfLength2<G>` with `graph: G` field
- Constructor `new(graph: G)` — panics if `num_vertices % 3 != 0`
- Getter methods: `graph()`, `num_vertices()`, `num_edges()`
- `evaluate()`: decode config as group assignments (each vertex gets value 0..q-1), check:
  1. Each group has exactly 3 members
  2. Each group induces at least 2 edges
- `Problem` impl: NAME = "PartitionIntoPathsOfLength2", Metric = bool, dims = `vec![q; n]`, variant = `variant_params![G]`
- `SatisfactionProblem` impl (marker trait)
- `declare_variants!`: `PartitionIntoPathsOfLength2<SimpleGraph> => "3^num_vertices"`

### Step 2: Register the model
- `src/models/graph/mod.rs`: add `pub(crate) mod partition_into_paths_of_length_2;` and `pub use partition_into_paths_of_length_2::PartitionIntoPathsOfLength2;`
- `src/models/mod.rs`: add `PartitionIntoPathsOfLength2` to the graph re-export line

### Step 3: Register in CLI
- `problemreductions-cli/src/dispatch.rs`:
  - `load_problem`: add `"PartitionIntoPathsOfLength2" => deser_sat::<PartitionIntoPathsOfLength2<SimpleGraph>>(data)`
  - `serialize_any_problem`: add `"PartitionIntoPathsOfLength2" => try_ser::<PartitionIntoPathsOfLength2<SimpleGraph>>(any)`
- `problemreductions-cli/src/problem_name.rs`:
  - Add `"partitionintopathsoflength2" => "PartitionIntoPathsOfLength2".to_string()` to `resolve_alias()`

### Step 4: CLI creation support
- `problemreductions-cli/src/commands/create.rs`:
  - Add match arm for `"PartitionIntoPathsOfLength2"` that parses `--graph` and constructs the problem
  - No new CLI flags needed (just `--graph`)
- `problemreductions-cli/src/cli.rs`:
  - Add entry to "Flags by problem type" table in `after_help`
  - No new struct fields needed in `CreateArgs`
  - No changes to `all_data_flags_empty()`

### Step 5: Write unit tests
**File:** `src/unit_tests/models/graph/partition_into_paths_of_length_2.rs`

- `test_partition_into_paths_basic`: create 9-vertex instance from issue example, verify dims, evaluate valid/invalid configs
- `test_partition_into_paths_no_solution`: create 6-vertex NO instance from issue, verify brute force finds no solution
- `test_partition_into_paths_solver`: verify brute force solver finds valid solutions on YES instance
- `test_partition_into_paths_serialization`: round-trip serde test
- `test_partition_into_paths_invalid_group_size`: verify configs where groups don't have exactly 3 members return false

Link test file via `#[cfg(test)] #[path = "..."] mod tests;` at bottom of model file.

### Step 6: Document in paper
- Add `"PartitionIntoPathsOfLength2": [Partition into Paths of Length 2]` to `display-name` dict in `docs/paper/reductions.typ`
- Add `#problem-def("PartitionIntoPathsOfLength2")` entry with formal definition and background

### Step 7: Verify
- `make test clippy` must pass
