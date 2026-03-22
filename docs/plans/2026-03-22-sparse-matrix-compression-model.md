# Sparse Matrix Compression Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement `SparseMatrixCompression` as a new algebraic satisfaction model, register it for CLI/example-db/paper workflows, and preserve the issue's verified `4 x 4`, `K = 2` example and complexity metadata.

**Architecture:** Represent configurations as row-shift assignments only, one variable per matrix row with domain `0..bound_k-1`. `evaluate()` should build the implied storage vector internally and reject configurations whose shifted supports collide. This keeps brute-force search at `bound_k ^ num_rows`, matching the issue and avoiding a much larger search space from enumerating storage-vector entries explicitly. Batch 1 delivers code, tests, registry wiring, CLI creation, and the canonical example. Batch 2 adds the paper entry after the example fixture is available.

**Tech Stack:** Rust workspace, serde, inventory registry, `pred` CLI, Typst paper

---

## Issue Packet

| Item | Decision |
|---|---|
| Issue | `#416` — `[Model] SparseMatrixCompression` |
| Repo skill | `.claude/skills/add-model/SKILL.md` |
| Category | `src/models/algebraic/` |
| Problem type | Satisfaction (`Metric = bool`) |
| Type parameters | None |
| Core fields | `matrix: Vec<Vec<bool>>`, `bound_k: usize` |
| Search space | `vec![bound_k; num_rows]` |
| Complexity string | `"(bound_k ^ num_rows) * num_rows * num_cols"` |
| Solver strategy for this PR | Brute-force over row shifts only |
| Associated rule | `Graph 3-Colorability -> SparseMatrixCompression` (issue crossref `R107`) |
| Important issue comments | Preserve the fixed approximation note, concrete complexity string, and the clean unique-solution example from the `/fix-issue` changelog |

## Batch Layout

- **Batch 1:** add model code, tests, registry/module wiring, canonical example, and CLI creation/help.
- **Batch 2:** add the paper `problem-def(...)` entry after Batch 1 is green.

## Design Notes

- The mathematical definition mentions both row shifts `s(i)` and storage-vector entries `b_j`, but the implementation should not enumerate `b_j` in `dims()`. Instead, it should construct the unique implied storage vector for a chosen shift assignment and check whether every `1` lands on an empty slot or the same row label.
- Use zero-based config values for solver compatibility. The issue's satisfying shift assignment `s = (2, 2, 2, 1)` should therefore appear in code as config `[1, 1, 1, 0]`.
- Keep the storage vector representation explicit in helper methods and tests because the issue's expected outcome includes `b = (4, 1, 2, 3, 1, 0)`.
- Do **not** add an ILP reduction in this PR. The issue already supplies a valid brute-force solver path, and this PR should stay a model-only change.

### Task 1: Lock Down Model Behavior with Failing Tests

**Files:**
- Create: `src/unit_tests/models/algebraic/sparse_matrix_compression.rs`
- Modify: `problemreductions-cli/src/commands/create.rs`

**Step 1: Write the failing model tests**

Add a new unit test file for the exact issue example matrix:

```rust
fn issue_example_matrix() -> Vec<Vec<bool>> {
    vec![
        vec![true, false, false, true],
        vec![false, true, false, false],
        vec![false, false, true, false],
        vec![true, false, false, false],
    ]
}
```

Cover these behaviors:
- constructor/getters: `num_rows() == 4`, `num_cols() == 4`, `bound_k() == 2`, `storage_len() == 6`, `dims() == vec![2; 4]`
- satisfying config: `[1, 1, 1, 0]` evaluates to `true`
- implied storage vector: `[1, 1, 1, 0]` produces `vec![4, 1, 2, 3, 1, 0]`
- unsatisfying configs from the issue: `[0, 0, 0, 0]`, `[0, 1, 1, 1]`, `[1, 1, 1, 1]`
- brute-force uniqueness: exactly one satisfying config, and it is `[1, 1, 1, 0]`
- serde shape: JSON uses `matrix` and `bound_k`
- metadata check: registry complexity string is `"(bound_k ^ num_rows) * num_rows * num_cols"`
- constructor guards: ragged rows panic; `bound_k == 0` panics

**Step 2: Run the model tests to verify they fail**

Run:

```bash
cargo test sparse_matrix_compression --lib
```

Expected: FAIL because the model file and registrations do not exist yet.

**Step 3: Write the failing CLI creation test**

In `problemreductions-cli/src/commands/create.rs`, add a focused test that creates the issue example via CLI:

```rust
#[test]
fn test_create_sparse_matrix_compression_json() { /* parse --matrix and --bound */ }
```

Assert:
- output `type == "SparseMatrixCompression"`
- serialized data contains the `4 x 4` matrix
- serialized `bound_k == 2`

Also add one negative test that omits `--bound` and confirms the usage message mentions `SparseMatrixCompression requires --matrix and --bound`.

**Step 4: Run the CLI tests to verify they fail**

Run:

```bash
cargo test -p problemreductions-cli sparse_matrix_compression
```

Expected: FAIL because the CLI arm/import/help text do not exist yet.

### Task 2: Implement the Model and Registry Wiring

**Files:**
- Create: `src/models/algebraic/sparse_matrix_compression.rs`
- Modify: `src/models/algebraic/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`

**Step 1: Write the minimal model implementation**

Create `src/models/algebraic/sparse_matrix_compression.rs` with:
- `inventory::submit!` `ProblemSchemaEntry`
- `#[derive(Debug, Clone, Serialize, Deserialize)]`
- constructor `new(matrix, bound_k)` with rectangular-row and positive-`bound_k` checks
- getters: `matrix()`, `bound_k()`, `num_rows()`, `num_cols()`, `storage_len()`
- helper(s):
  - `decode_shifts(config) -> Option<Vec<usize>>` or equivalent
  - `storage_vector(config) -> Option<Vec<usize>>` that returns the implied `b` vector when the overlay is valid
- `Problem` impl:
  - `NAME = "SparseMatrixCompression"`
  - `Metric = bool`
  - `dims() = vec![self.bound_k; self.num_rows()]`
  - `evaluate(config)` returns `self.storage_vector(config).is_some()`
  - `variant() = crate::variant_params![]`
- `SatisfactionProblem` impl
- `declare_variants!`:

```rust
crate::declare_variants! {
    default sat SparseMatrixCompression => "(bound_k ^ num_rows) * num_rows * num_cols",
}
```

- `#[cfg(feature = "example-db")] canonical_model_example_specs()` using the issue's fixed example and `optimal_config: vec![1, 1, 1, 0]`
- test link:

```rust
#[cfg(test)]
#[path = "../../unit_tests/models/algebraic/sparse_matrix_compression.rs"]
mod tests;
```

**Step 2: Register the model**

Update:
- `src/models/algebraic/mod.rs`
  - add module + public re-export
  - extend `canonical_model_example_specs()` with `sparse_matrix_compression::canonical_model_example_specs()`
- `src/models/mod.rs`
  - add `SparseMatrixCompression` to the algebraic re-export list
- `src/lib.rs`
  - add `SparseMatrixCompression` to `prelude`

**Step 3: Run the model tests again**

Run:

```bash
cargo test sparse_matrix_compression --lib
```

Expected: the new model tests pass, or fail only on CLI/paper gaps not yet implemented.

**Step 4: Commit the model slice**

```bash
git add src/models/algebraic/sparse_matrix_compression.rs \
        src/models/algebraic/mod.rs \
        src/models/mod.rs \
        src/lib.rs \
        src/unit_tests/models/algebraic/sparse_matrix_compression.rs
git commit -m "Add SparseMatrixCompression model"
```

### Task 3: Wire CLI Creation and Help

**Files:**
- Modify: `problemreductions-cli/src/commands/create.rs`
- Modify: `problemreductions-cli/src/cli.rs`

**Step 1: Implement the CLI create arm**

In `problemreductions-cli/src/commands/create.rs`:
- import `SparseMatrixCompression`
- add `example_for("SparseMatrixCompression")`
- add a `help_flag_name()` override so schema field `bound_k` maps to CLI flag `--bound`
- add a `help_flag_hint()` override so `matrix` is documented as semicolon-separated `0/1` rows
- add a `"SparseMatrixCompression"` create arm that:
  - uses `parse_bool_matrix(args)?`
  - requires `--bound`
  - converts `bound` to `usize`
  - serializes `SparseMatrixCompression::new(matrix, bound)`

Use this usage string:

```text
Usage: pred create SparseMatrixCompression --matrix "1,0,0,1;0,1,0,0;0,0,1,0;1,0,0,0" --bound 2
```

**Step 2: Update top-level CLI help**

In `problemreductions-cli/src/cli.rs`, add:

```text
SparseMatrixCompression         --matrix (0/1), --bound
```

to the "Flags by problem type" table.

**Step 3: Re-run the targeted CLI tests**

Run:

```bash
cargo test -p problemreductions-cli sparse_matrix_compression
```

Expected: the new create tests pass.

**Step 4: Commit the CLI slice**

```bash
git add problemreductions-cli/src/commands/create.rs problemreductions-cli/src/cli.rs
git commit -m "Add CLI support for SparseMatrixCompression"
```

### Task 4: Verify Batch 1 End-to-End

**Files:**
- Reference: `src/models/algebraic/sparse_matrix_compression.rs`
- Reference: `src/unit_tests/models/algebraic/sparse_matrix_compression.rs`
- Reference: `problemreductions-cli/src/commands/create.rs`

**Step 1: Run focused verification**

Run:

```bash
cargo test sparse_matrix_compression --lib
cargo test -p problemreductions-cli sparse_matrix_compression
```

Expected: PASS.

**Step 2: Run project verification for code changes**

Run:

```bash
make test clippy
```

Expected: PASS.

**Step 3: Inspect the canonical example through the CLI**

Run:

```bash
pred create --example SparseMatrixCompression -o /tmp/smc-example.json
pred evaluate /tmp/smc-example.json --config 1,1,1,0
pred solve /tmp/smc-example.json
```

Expected:
- example JSON matches the issue matrix and `bound_k = 2`
- `pred evaluate` returns `true`
- `pred solve` finds the unique satisfying config

### Task 5: Add the Paper Entry (Batch 2)

**Files:**
- Modify: `docs/paper/reductions.typ`

**Step 1: Write the paper entry after code is green**

Add a display-name entry:

```typst
"SparseMatrixCompression": [Sparse Matrix Compression],
```

Add a `#problem-def("SparseMatrixCompression")[...][...]` block that:
- states the formal row-overlay definition
- explains that the implementation searches over shifts and reconstructs the storage vector internally
- cites the Garey & Johnson catalog entry and the practical DFA/parser-table motivation
- uses `load-model-example("SparseMatrixCompression")`
- explains that stored config `[1, 1, 1, 0]` encodes shifts `(2, 2, 2, 1)`
- shows the resulting storage vector `(4, 1, 2, 3, 1, 0)`
- states that the fixture has exactly one satisfying shift assignment
- includes a `pred-commands(...)` block derived from the loaded example

**Step 2: Build the paper**

Run:

```bash
make paper
```

Expected: PASS.

**Step 3: Re-run final repo verification**

Run:

```bash
make test clippy
```

Expected: PASS.

**Step 4: Commit the documentation slice**

```bash
git add docs/paper/reductions.typ
git commit -m "Document SparseMatrixCompression"
```

### Task 6: Final Push and Cleanup

**Files:**
- Modify: `docs/plans/2026-03-22-sparse-matrix-compression-model.md` (remove at the end per pipeline workflow)

**Step 1: Summarize deviations before push**

Record whether any of these changed during implementation:
- field names (`bound_k` vs `bound`)
- helper method names
- exact paper example wording

**Step 2: Push implementation commits**

```bash
git push
```

**Step 3: Remove the plan file after implementation**

```bash
git rm docs/plans/2026-03-22-sparse-matrix-compression-model.md
git commit -m "chore: remove plan file after implementation"
git push
```
