# MultipleCopyFileAllocation Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the `MultipleCopyFileAllocation` model as a graph-based satisfaction problem with registry metadata, CLI creation support, canonical example coverage, and paper documentation.

**Architecture:** Implement `MultipleCopyFileAllocation` as a concrete `SimpleGraph` satisfaction model with no type parameters. The evaluator should treat a configuration as a subset of copy vertices, run a multi-source BFS to compute each vertex's distance to the nearest copy, add weighted access cost plus selected-vertex storage cost, and accept exactly when the total is at most `bound`. Use the issue's Garey-Johnson formulation as the source of truth, keep the original 1977-report claims conservative, and use the associated rule issue `#425` to avoid orphan-model handling.

**Tech Stack:** Rust core crate, `inventory` registry metadata, serde, `problemreductions-cli` clap command wiring, example-db fixtures, Typst paper.

---

## Batch 1: Model, Tests, Registry, CLI

### Task 1: Add red tests for the new model

**Files:**
- Create: `src/unit_tests/models/graph/multiple_copy_file_allocation.rs`
- Modify: `src/unit_tests/trait_consistency.rs`

**Step 1: Write the failing model tests**

Create `src/unit_tests/models/graph/multiple_copy_file_allocation.rs` with helpers and tests that pin the intended semantics from issue `#410`:
- constructor/accessor coverage for `graph`, `usage`, `storage`, `bound`
- `dims() == vec![2; n]`
- `evaluate()` returns `true` for the issue's positive witness on the 6-cycle
- `evaluate()` returns `false` for:
  - the issue's negative bound example
  - an empty placement on a non-empty graph
  - a disconnected graph where some component has no copy
  - wrong-length or non-binary configs
- a `total_cost`-style helper expectation for the paper/example configuration
- brute-force solver finds a satisfying assignment for the YES instance and none for the NO instance
- serde round-trip
- `test_multiple_copy_file_allocation_paper_example` using the exact canonical example chosen later for the paper

Also add a failing `check_problem_trait(...)` entry in `src/unit_tests/trait_consistency.rs`.

**Step 2: Run the focused tests and verify RED**

Run:
```bash
cargo test test_multiple_copy_file_allocation --lib
```

Expected: compile/test failure because `MultipleCopyFileAllocation` does not exist yet.

**Step 3: Write the minimal production implementation**

Create `src/models/graph/multiple_copy_file_allocation.rs` with:
- `inventory::submit!` registration:
  - `name: "MultipleCopyFileAllocation"`
  - `display_name: "Multiple Copy File Allocation"`
  - `aliases: &[]`
  - `dimensions: &[]`
  - `fields`: `graph`, `usage`, `storage`, `bound`
- `#[derive(Debug, Clone, Serialize, Deserialize)]`
- concrete struct:
  - `graph: SimpleGraph`
  - `usage: Vec<i64>`
  - `storage: Vec<i64>`
  - `bound: i64`
- constructor validation:
  - `usage.len() == graph.num_vertices()`
  - `storage.len() == graph.num_vertices()`
- accessors:
  - `graph()`
  - `usage()`
  - `storage()`
  - `bound()`
  - `num_vertices()`
  - `num_edges()`
- helper methods:
  - `selected_vertices(config) -> Option<Vec<usize>>` or equivalent binary/length validation
  - multi-source BFS over `SimpleGraph::neighbors(...)`
  - `total_cost(config) -> Option<i64>` returning `None` for invalid configs or unreachable vertices
  - `is_valid_solution(config) -> bool`
- `Problem` impl:
  - `NAME = "MultipleCopyFileAllocation"`
  - `Metric = bool`
  - `variant() = crate::variant_params![]`
  - `dims() = vec![2; num_vertices]`
  - `evaluate() = is_valid_solution(config)`
- `SatisfactionProblem` impl
- `declare_variants!` entry:
  ```rust
  crate::declare_variants! {
      default sat MultipleCopyFileAllocation => "2^num_vertices * (num_vertices + num_edges)"
  }
  ```
- canonical example builder using the small positive issue instance that will also drive the paper example
- `#[cfg(test)]` link to the new unit test file

**Step 4: Register the model everywhere it must appear**

Update:
- `src/models/graph/mod.rs`
  - module declaration
  - public re-export
  - `canonical_model_example_specs()` aggregation
- `src/models/mod.rs`
  - public re-export
- `src/lib.rs`
  - add `MultipleCopyFileAllocation` to the graph prelude exports
- `src/unit_tests/trait_consistency.rs`
  - `check_problem_trait(...)` entry using a small instance

**Step 5: Run the focused tests and verify GREEN**

Run:
```bash
cargo test test_multiple_copy_file_allocation --lib
```

Expected: the new model/unit-test slice passes.

**Step 6: Commit**

```bash
git add \
  src/models/graph/multiple_copy_file_allocation.rs \
  src/models/graph/mod.rs \
  src/models/mod.rs \
  src/lib.rs \
  src/unit_tests/models/graph/multiple_copy_file_allocation.rs \
  src/unit_tests/trait_consistency.rs
git commit -m "feat: add MultipleCopyFileAllocation model"
```

### Task 2: Add CLI creation support and CLI regression tests

**Files:**
- Modify: `problemreductions-cli/src/cli.rs`
- Modify: `problemreductions-cli/src/commands/create.rs`
- Modify: `problemreductions-cli/tests/cli_tests.rs`

**Step 1: Write the failing CLI tests**

Add focused CLI tests that assert:
- `pred create MultipleCopyFileAllocation --graph ... --usage ... --storage ... --bound ...` succeeds and emits type `"MultipleCopyFileAllocation"`
- `pred create MultipleCopyFileAllocation` with no data flags shows schema-driven help mentioning `--usage`, `--storage`, and `--bound`
- mismatched `--usage` / `--storage` lengths fail with clear diagnostics

**Step 2: Run the focused CLI tests and verify RED**

Run:
```bash
cargo test -p problemreductions-cli test_create_multiple_copy_file_allocation
```

Expected: failure because the CLI does not know this problem or its flags yet.

**Step 3: Implement the minimal CLI wiring**

Update `problemreductions-cli/src/cli.rs`:
- add `usage: Option<String>` and `storage: Option<String>` to `CreateArgs`
- include them in `all_data_flags_empty()`
- add `MultipleCopyFileAllocation --graph, --usage, --storage, --bound` to the help table

Update `problemreductions-cli/src/commands/create.rs`:
- add an example string in `example_for(...)`
- add a `match` arm for `MultipleCopyFileAllocation`
- parse:
  - `--graph`
  - `--usage` as `Vec<i64>`
  - `--storage` as `Vec<i64>`
  - `--bound` as `i64`
- validate both vectors against `graph.num_vertices()`
- serialize `MultipleCopyFileAllocation::new(...)`

Do **not** add a short alias unless the literature shows one is standard.

**Step 4: Run the focused CLI tests and verify GREEN**

Run:
```bash
cargo test -p problemreductions-cli test_create_multiple_copy_file_allocation -- --nocapture
```

Expected: the new CLI tests pass.

**Step 5: Regenerate canonical example fixtures**

Run:
```bash
cargo run --release --features "ilp-highs example-db" --example regenerate_fixtures
```

Expected: `src/example_db/fixtures/examples.json` updates to include the new canonical model example.

**Step 6: Commit**

```bash
git add \
  problemreductions-cli/src/cli.rs \
  problemreductions-cli/src/commands/create.rs \
  problemreductions-cli/tests/cli_tests.rs \
  src/example_db/fixtures/examples.json
git commit -m "feat: add CLI support for MultipleCopyFileAllocation"
```

## Batch 2: Paper Entry and Final Verification

### Task 3: Document the model in the paper and lock the paper/example contract

**Files:**
- Modify: `docs/paper/reductions.typ`
- Modify: `src/unit_tests/models/graph/multiple_copy_file_allocation.rs`

**Step 1: Finalize the canonical example choice from the implemented builder**

Use the computed canonical example from Batch 1 as the paper source of truth. Prefer the smallest positive witness from the issue that keeps the distance arithmetic easy to read; if the chosen instance has many satisfying placements, describe one witness in the paper and only claim the exact satisfying-count after brute-force confirms it.

**Step 2: Write the paper entry**

Update `docs/paper/reductions.typ`:
- add `"MultipleCopyFileAllocation": [Multiple Copy File Allocation],` to `display-name`
- add `#problem-def("MultipleCopyFileAllocation")[...]` in the graph-problem section
- cover:
  - the formal definition from the issue packet
  - brief motivation tying it to distributed file placement / facility location
  - conservative complexity prose matching the implemented `declare_variants!` expression
  - a small graph example using the canonical example instance
  - explicit cost evaluation: storage term + access term = total cost

Keep citations conservative:
- use the existing Garey-Johnson citation for NP-completeness context
- avoid adding stronger claims that depend on the unverified 1977 technical report unless a citable entry is added intentionally

**Step 3: Update the paper test to match the final example**

Back in `src/unit_tests/models/graph/multiple_copy_file_allocation.rs`, make `test_multiple_copy_file_allocation_paper_example` assert:
- the exact witness shown in the paper is satisfying
- `total_cost(...)` matches the paper arithmetic
- brute force confirms the paper's stated satisfying count (or confirms at least one witness if the prose intentionally avoids an exact count)

**Step 4: Build the paper and verify**

Run:
```bash
make paper
```

Expected: Typst build passes and any generated reduction exports update cleanly.

**Step 5: Run full verification**

Run:
```bash
make test
make clippy
```

If the paper/export steps modified tracked generated files (notably under `docs/src/reductions/`), stage those expected updates too.

**Step 6: Commit**

```bash
git add \
  docs/paper/reductions.typ \
  src/unit_tests/models/graph/multiple_copy_file_allocation.rs \
  docs/src/reductions/problem_schemas.json \
  docs/src/reductions/reduction_graph.json
git commit -m "docs: add MultipleCopyFileAllocation paper entry"
```

### Task 4: Review and branch cleanup

**Files:**
- Inspect only

**Step 1: Run structural review**

Run the repo review workflow after the implementation batch is complete:
```bash
make test
make clippy
```

Then manually confirm:
- no orphan-model warning is needed because rule issue `#425` exists
- the plan file will be removed before the final push
- the PR summary comment mentions the chosen canonical example and any conservative citation choices

**Step 2: Commit any remaining review fixes**

```bash
git add -A
git commit -m "chore: finish MultipleCopyFileAllocation review fixes"
```
