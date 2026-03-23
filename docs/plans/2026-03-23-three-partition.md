# ThreePartition Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the `ThreePartition` satisfaction model, CLI/example-db integration, and paper entry for GitHub issue #492 using the corrected issue witness and registry metadata.

**Architecture:** `ThreePartition` belongs in `src/models/misc/` as a satisfaction problem with one `m`-ary variable per element, where `m = sizes.len() / 3` is derived from the instance. The constructor should validate the canonical 3-Partition invariants (`|A| = 3m`, `B/4 < s(a) < B/2`, and `sum s(a) = mB`) and `evaluate()` should accept exactly the group assignments that partition all elements into triples summing to `bound`. Batch 1 covers code, tests, registry, CLI, and canonical example support; Batch 2 covers the Typst paper entry after the example/export path is in place.

**Tech Stack:** Rust workspace, serde/inventory registry metadata, `pred` CLI, example-db, Typst paper, `make test`, `make clippy`, `make paper`

---

## Batch 1: Model, tests, registry, CLI, example-db

### Task 1: Add the failing ThreePartition model tests first

**Files:**
- Create: `src/unit_tests/models/misc/three_partition.rs`
- Modify: `src/models/misc/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`

**Step 1: Write the failing test**

Create `src/unit_tests/models/misc/three_partition.rs` with tests that pin the required behavior before the model exists:
- `test_three_partition_basic` for getters, derived `num_groups`, and `dims() == vec![2; 6]` on the witness `sizes = [4, 5, 6, 4, 6, 5], bound = 15`
- `test_three_partition_evaluate_yes_instance` for a satisfying configuration such as `[0, 0, 0, 1, 1, 1]`
- `test_three_partition_rejects_wrong_group_sizes_or_sums` for invalid assignments
- `test_three_partition_paper_example` for the corrected issue witness and brute-force confirmation
- validation tests for bad length, zero sizes, broken inequalities, and wrong total sum

**Step 2: Run test to verify it fails**

Run: `cargo test test_three_partition_basic --lib`

Expected: FAIL because `ThreePartition` is not defined/exported yet.

**Step 3: Write minimal implementation to satisfy the first red tests**

Add the module/export plumbing only:
- declare `mod three_partition;` and `pub use three_partition::ThreePartition;` in `src/models/misc/mod.rs`
- re-export `ThreePartition` from `src/models/mod.rs` and `src/lib.rs`

Do not add any extra behavior here beyond what is needed to let the compiler reach the missing-model errors in the new test file.

**Step 4: Run the focused test again**

Run: `cargo test test_three_partition_basic --lib`

Expected: FAIL with missing struct/constructor/getter errors from the new test file rather than missing module exports.

### Task 2: Implement the ThreePartition model with strict invariant checking

**Files:**
- Create: `src/models/misc/three_partition.rs`
- Modify: `src/models/misc/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`
- Test: `src/unit_tests/models/misc/three_partition.rs`

**Step 1: Write the next failing test**

Extend `src/unit_tests/models/misc/three_partition.rs` so the core API is explicit:
- constructor/getter expectations for `sizes()`, `bound()`, `num_elements()`, `num_groups()`, and `total_sum()`
- `Problem::NAME == "ThreePartition"` and `variant() == vec![]`
- serialization/deserialization round-trip with invalid JSON cases rejected

**Step 2: Run test to verify it fails**

Run: `cargo test three_partition --lib`

Expected: FAIL because the model file is still missing.

**Step 3: Write minimal implementation**

Create `src/models/misc/three_partition.rs` with:
- `ProblemSchemaEntry` metadata:
  - `name: "ThreePartition"`
  - `display_name: "3-Partition"`
  - aliases containing `"3Partition"`
  - fields `sizes: Vec<u64>` and `bound: u64`
- `ThreePartition { sizes: Vec<u64>, bound: u64 }`
- `try_new()` + `new()` validation:
  - non-empty input
  - `sizes.len() % 3 == 0`
  - all sizes strictly positive
  - strict bounds checked without floats via integer arithmetic (`4 * size > bound` and `2 * size < bound`, using `u128` math)
  - `sum(sizes) == bound * num_groups`
- getters for `sizes`, `bound`, `num_elements`, `num_groups`, `total_sum`
- helper that computes per-group counts/sums from a config and rejects wrong length or out-of-range group ids
- `Problem` impl:
  - `type Metric = bool`
  - `dims() = vec![num_groups; num_elements]`
  - `evaluate()` returns true iff every group has exactly 3 elements and sum exactly `bound`
- `impl SatisfactionProblem for ThreePartition {}`
- `crate::declare_variants! { default sat ThreePartition => "3^num_elements", }`
- canonical model example spec using the corrected issue witness and satisfying config `[0, 0, 0, 1, 1, 1]`
- test link at file bottom

**Step 4: Run tests to verify it passes**

Run: `cargo test three_partition --lib`

Expected: PASS for the new model tests.

**Step 5: Commit**

Run:
```bash
git add src/models/misc/three_partition.rs src/models/misc/mod.rs src/models/mod.rs src/lib.rs src/unit_tests/models/misc/three_partition.rs
git commit -m "feat: add ThreePartition model"
```

### Task 3: Add CLI discovery and `pred create` support

**Files:**
- Modify: `problemreductions-cli/src/problem_name.rs`
- Modify: `problemreductions-cli/src/commands/create.rs`
- Modify: `problemreductions-cli/src/cli.rs`
- Test: `problemreductions-cli/src/problem_name.rs`
- Test: `problemreductions-cli/src/commands/create.rs`

**Step 1: Write the failing test**

Add CLI-facing tests for:
- alias resolution from `3Partition` to `ThreePartition`
- `pred create ThreePartition --sizes 4,5,6,4,6,5 --bound 15`
- missing `--sizes` / missing `--bound` errors
- invalid instance data rejected by `ThreePartition::try_new`

**Step 2: Run test to verify it fails**

Run: `cargo test -p problemreductions-cli three_partition`

Expected: FAIL because alias resolution and create-arm support do not exist yet.

**Step 3: Write minimal implementation**

Update CLI code:
- `problemreductions-cli/src/problem_name.rs`: add `3Partition` special-case alias if registry alias lookup does not already cover the schema alias cleanly in tests
- `problemreductions-cli/src/commands/create.rs`:
  - add example/help text for `ThreePartition`
  - add a `match` arm parsing `--sizes` and `--bound`
  - construct with `ThreePartition::try_new(...)`
- `problemreductions-cli/src/cli.rs`:
  - add `ThreePartition` to the “Flags by problem type” table with `--sizes, --bound`

**Step 4: Run tests to verify it passes**

Run: `cargo test -p problemreductions-cli three_partition`

Expected: PASS for the new CLI tests.

### Task 4: Register the canonical example path end-to-end

**Files:**
- Modify: `src/models/misc/mod.rs`
- Modify: `src/example_db/model_builders.rs` (only if ordering or coverage requires it)
- Test: `src/unit_tests/models/misc/three_partition.rs`
- Test: `src/unit_tests/example_db.rs`

**Step 1: Write the failing test**

Add or extend tests that prove the canonical example survives the example-db path:
- the example spec list in `misc::canonical_model_example_specs()` includes `ThreePartition`
- the stored witness config evaluates to `true`

**Step 2: Run test to verify it fails**

Run: `cargo test canonical_model_example_specs --lib`

Expected: FAIL if the new example is not yet included in the misc example chain.

**Step 3: Write minimal implementation**

Ensure `src/models/misc/mod.rs` includes:
- module declaration/export for `three_partition`
- `specs.extend(three_partition::canonical_model_example_specs());`

Only touch `src/example_db/model_builders.rs` if the existing chained collectors need explicit updates after the new misc spec is registered.

**Step 4: Run tests to verify it passes**

Run: `cargo test example_db --lib`

Expected: PASS for example-db coverage that includes the new `ThreePartition` fixture.

## Batch 2: Paper entry after code + example support are stable

### Task 5: Add the Typst paper entry and paper-example alignment

**Files:**
- Modify: `docs/paper/reductions.typ`
- Modify: `src/unit_tests/models/misc/three_partition.rs`

**Step 1: Write the failing test**

Finalize `test_three_partition_paper_example` so it matches the exact paper/example witness:
- instance `sizes = [4, 5, 6, 4, 6, 5], bound = 15`
- satisfying partition into two triples
- brute-force confirms the witness is valid and that satisfying solutions exist

**Step 2: Run test to verify it fails**

Run: `cargo test test_three_partition_paper_example --lib`

Expected: FAIL until the exact witness/config in code matches the paper story.

**Step 3: Write minimal implementation**

Add to `docs/paper/reductions.typ`:
- display-name entry: `"ThreePartition": [3-Partition],`
- `#problem-def("ThreePartition")[...]` near the other misc partitioning/scheduling problems
- prose that reflects the verified issue comments:
  - classical strongly NP-complete source problem from Garey & Johnson SP15
  - strong NP-completeness motivation for scheduling/packing reductions
  - exact baseline aligned with the registry expression `O^*(3^n)`
- example based on the corrected witness with `pred-commands(...)`

Keep the paper example consistent with the canonical model example and the paper-example unit test.

**Step 4: Run tests and paper build**

Run:
```bash
cargo test test_three_partition_paper_example --lib
make paper
```

Expected: PASS and successful Typst build.

### Task 6: Final verification and integration sweep

**Files:**
- Modify: none expected
- Verify: working tree only contains intended tracked changes

**Step 1: Run the full verification commands**

Run:
```bash
make test
make clippy
make paper
```

Expected: all commands succeed.

**Step 2: Review issue requirements against the implementation**

Check explicitly that the branch contains:
- new `ThreePartition` model in `misc`
- corrected witness and `3^num_elements` complexity
- CLI creation support and alias handling
- canonical example registration
- paper entry + paper-aligned test

**Step 3: Commit**

Run:
```bash
git add src/models/misc/three_partition.rs src/models/misc/mod.rs src/models/mod.rs src/lib.rs src/unit_tests/models/misc/three_partition.rs problemreductions-cli/src/problem_name.rs problemreductions-cli/src/commands/create.rs problemreductions-cli/src/cli.rs docs/paper/reductions.typ
git commit -m "Implement #492: add ThreePartition"
```
