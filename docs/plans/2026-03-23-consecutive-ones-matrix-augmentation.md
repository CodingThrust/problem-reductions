# ConsecutiveOnesMatrixAugmentation Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the `ConsecutiveOnesMatrixAugmentation` satisfaction model for issue #419, wire it into the registry and CLI, register a canonical example, and document it in the paper using the corrected issue examples.

**Architecture:** Represent a witness as a full column permutation. `evaluate()` should validate that permutation, compute the minimum number of `0 -> 1` augmentations needed to make each row consecutive under that order, and accept iff the total is at most `bound`. Keep the schema aligned with the issue comments: `matrix: Vec<Vec<bool>>`, `bound: i64`, and complexity `factorial(num_cols) * num_rows * num_cols`.

**Tech Stack:** Rust workspace, serde + inventory registry metadata, `pred` CLI, example-db, Typst paper, `make` verification targets.

---

## Issue-Locked Inputs

- GitHub issue: `#419 [Model] ConsecutiveOnesMatrixAugmentation`
- Associated rule issue: `#434 [Rule] Optimal Linear Arrangement to Consecutive Ones Matrix Augmentation`
- Canonical YES example: the corrected `4 x 5` graph-incidence matrix with `bound = 2` and witness permutation `[0, 1, 4, 2, 3]`
- Canonical NO example: the corrected `4 x 4` matrix with `bound = 0`
- Required complexity string: `"factorial(num_cols) * num_rows * num_cols"`
- Do not revert to the older broken examples from the original issue body

## Add-Model Step Mapping

- Batch 1 covers add-model Steps `1`, `1.5`, `2`, `2.5`, `3`, `4`, `4.5`, `4.6`, and `5`
- Batch 2 covers add-model Step `6` (`write-model-in-paper` style work)
- Final verification covers add-model Step `7`

## Batch 1: Model, Registry, CLI, and Tests

### Task 1: Scaffold the Model and Lock Down Core Behavior

**Files:**
- Create: `src/models/algebraic/consecutive_ones_matrix_augmentation.rs`
- Create: `src/unit_tests/models/algebraic/consecutive_ones_matrix_augmentation.rs`
- Modify: `src/models/algebraic/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`

**Step 1: Write the failing unit tests**

Add tests for:
- constructor/getters/`dims()`/`num_variables()`
- the corrected YES issue example using permutation `[0, 1, 4, 2, 3]`
- the corrected NO issue example with `bound = 0`
- invalid configs: wrong length, duplicate columns, out-of-range columns
- brute-force solver behavior on the YES and NO instances
- serde round-trip
- complexity metadata string

Run:
```bash
cargo test consecutive_ones_matrix_augmentation --features "ilp-highs example-db" -- --nocapture
```

Expected: compile failure until the model and exports exist.

**Step 2: Add the model skeleton and exports**

Implement the basic file structure:
- `ProblemSchemaEntry` with `display_name`, empty aliases, and fields `matrix` / `bound`
- `ConsecutiveOnesMatrixAugmentation` struct with `matrix`, `bound`, `num_rows`, and `num_cols`
- `try_new` + `new`, plus `matrix()`, `bound()`, `num_rows()`, and `num_cols()`
- `#[cfg(test)]` link to the new unit test file
- module/export wiring in `src/models/algebraic/mod.rs`, `src/models/mod.rs`, and `src/lib.rs`

Run:
```bash
cargo test consecutive_ones_matrix_augmentation --features "ilp-highs example-db" -- --nocapture
```

Expected: unresolved-symbol errors should be gone; remaining failures should be behavioral.

**Step 3: Implement the evaluator**

Implement the actual model logic:
- config is a full column permutation, so `dims()` is `vec![num_cols; num_cols]`
- validate permutations exactly once in a helper
- compute the minimum augmentation cost for one row under a fixed permutation by filling the holes between the first and last `1`
- sum row costs, short-circuit when the total exceeds `bound`
- make `evaluate()` return `false` for invalid permutations
- implement `SatisfactionProblem`
- add:

```rust
crate::declare_variants! {
    default sat ConsecutiveOnesMatrixAugmentation => "factorial(num_cols) * num_rows * num_cols",
}
```

Run:
```bash
cargo test consecutive_ones_matrix_augmentation --features "ilp-highs example-db" -- --nocapture
```

Expected: the targeted model tests pass.

**Step 4: Commit**

```bash
git add src/models/algebraic/consecutive_ones_matrix_augmentation.rs \
        src/unit_tests/models/algebraic/consecutive_ones_matrix_augmentation.rs \
        src/models/algebraic/mod.rs \
        src/models/mod.rs \
        src/lib.rs
git commit -m "Add ConsecutiveOnesMatrixAugmentation model"
```

### Task 2: Add CLI Create Support and Discovery

**Files:**
- Modify: `problemreductions-cli/src/commands/create.rs`
- Modify: `problemreductions-cli/src/cli.rs`
- Inspect and modify only if needed: `problemreductions-cli/src/problem_name.rs`

**Step 1: Add failing CLI tests**

Add or extend CLI tests covering:
- successful `pred create ConsecutiveOnesMatrixAugmentation --matrix "..." --bound 2`
- missing `--bound` error text
- malformed permutation-independent matrix input stays consistent with the existing `parse_bool_matrix` path

Run:
```bash
cargo test -p problemreductions-cli consecutive_ones_matrix_augmentation -- --nocapture
```

Expected: failures until the new create arm and help text exist.

**Step 2: Implement CLI wiring**

Update the CLI to:
- import `ConsecutiveOnesMatrixAugmentation`
- add a `create()` match arm beside the other algebraic matrix problems
- add the help-table entry in `problemreductions-cli/src/cli.rs`
- add any `help_flag_name()` / `help_flag_hint()` overrides needed so the schema/help shows `--bound`
- only touch `problem_name.rs` if canonical-name resolution does not already work through the registry; do **not** invent a short alias

Run:
```bash
cargo test -p problemreductions-cli consecutive_ones_matrix_augmentation -- --nocapture
```

Expected: the targeted CLI tests pass.

**Step 3: Commit**

```bash
git add problemreductions-cli/src/commands/create.rs \
        problemreductions-cli/src/cli.rs \
        problemreductions-cli/src/problem_name.rs
git commit -m "Add CLI support for ConsecutiveOnesMatrixAugmentation"
```

### Task 3: Register the Canonical Example and Paper-Test Coverage

**Files:**
- Modify: `src/models/algebraic/consecutive_ones_matrix_augmentation.rs`
- Modify: `src/models/algebraic/mod.rs`
- Inspect: `src/example_db/model_builders.rs`

**Step 1: Add canonical example specs**

Inside the model file, add `canonical_model_example_specs()` with:
- the corrected YES instance from issue #419
- `bound = 2`
- `optimal_config = vec![0, 1, 4, 2, 3]`
- `optimal_value = serde_json::json!(true)`

Update the algebraic category spec chain so the new model is included.

**Step 2: Add paper-example-focused tests**

Extend the new unit test file with a `test_consecutive_ones_matrix_augmentation_paper_example` that:
- constructs the same canonical YES instance
- verifies the issue witness permutation is satisfying
- uses `BruteForce` to confirm at least one satisfying permutation exists

Run:
```bash
cargo test consecutive_ones_matrix_augmentation_paper_example --features "ilp-highs example-db" -- --nocapture
cargo test canonical_model_example --features "ilp-highs example-db" -- --nocapture
```

Expected: the example-db path and paper example test pass.

**Step 3: Commit**

```bash
git add src/models/algebraic/consecutive_ones_matrix_augmentation.rs \
        src/models/algebraic/mod.rs \
        src/unit_tests/models/algebraic/consecutive_ones_matrix_augmentation.rs
git commit -m "Add example-db coverage for ConsecutiveOnesMatrixAugmentation"
```

## Batch 2: Paper Entry

### Task 4: Document the Model in `docs/paper/reductions.typ`

**Files:**
- Modify: `docs/paper/reductions.typ`

**Step 1: Add the display name and `problem-def`**

Add:
- `"ConsecutiveOnesMatrixAugmentation": [Consecutive Ones Matrix Augmentation],`
- a `#problem-def("ConsecutiveOnesMatrixAugmentation")[ ... ][ ... ]` entry

The paper entry should explicitly say:
- the matrix is binary and the goal is to flip at most `K` zeros to ones
- the witness displayed in the paper is a column permutation
- the augmentation set is derived by filling holes between the first and last `1` in each row under that permutation
- the historical notes and complexity match the corrected issue discussion

**Step 2: Use the canonical example data**

Follow the existing `load-model-example(...)` pattern and derive:
- the rendered matrix
- the example permutation
- the CLI `pred create --example`, `pred solve`, and `pred evaluate ... --config ...` commands

Do not hand-write a bare alias or a mismatched witness.

**Step 3: Build the paper**

Run:
```bash
make paper
```

Expected: Typst compiles cleanly and the new problem entry renders without completeness regressions.

**Step 4: Commit**

```bash
git add docs/paper/reductions.typ
git commit -m "Document ConsecutiveOnesMatrixAugmentation in the paper"
```

## Final Verification

### Task 5: Run Full Verification and Stage Expected Generated Outputs

**Files:**
- Modify if regenerated: tracked schema/example/paper outputs only

**Step 1: Run focused checks**

```bash
cargo test consecutive_ones_matrix_augmentation --features "ilp-highs example-db" -- --nocapture
cargo test -p problemreductions-cli consecutive_ones_matrix_augmentation -- --nocapture
```

Expected: all new targeted tests pass.

**Step 2: Run repo verification**

```bash
make test
make clippy
make paper
```

If the new model requires regenerated tracked outputs, run the repo-standard regeneration command and stage only the expected files.

**Step 3: Inspect the tree**

```bash
git status --short
```

Expected: only intentional source/docs/generated changes remain; the plan file is still present until the implementation is complete.

**Step 4: Final implementation commit**

```bash
git add -A
git commit -m "Implement #419: ConsecutiveOnesMatrixAugmentation"
```
