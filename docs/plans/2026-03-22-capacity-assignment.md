# CapacityAssignment Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the `CapacityAssignment` model, CLI creation path, canonical example, tests, and paper entry required by issue `#411`.

**Architecture:** Implement `CapacityAssignment` as a `misc` satisfaction problem with one multi-valued variable per communication link, where each variable selects an index into a shared ordered capacity set. The model owns the capacity levels, per-link cost/delay matrices, and the two budgets; `evaluate()` checks config length/range, then sums cost and delay and returns `true` iff both budgets are met. The issue already names the planned connecting rule `SubsetSum -> CapacityAssignment`, so this model is not an orphan even though no separate open rule issue currently matches the name.

**Tech Stack:** Rust workspace (`serde`, `inventory`, registry macros), Clap CLI, Typst paper, `make` verification targets.

---

## Batch 1: Model, Registry, Example, CLI

### Task 1: Write failing model tests for issue #411 behavior

**Files:**
- Create: `src/unit_tests/models/misc/capacity_assignment.rs`
- Reference: `src/models/misc/multiprocessor_scheduling.rs`
- Reference: `src/unit_tests/models/misc/multiprocessor_scheduling.rs`

**Step 1: Write the failing test**

Add focused tests for:
- construction/getters/dimensions on the fixed 3-link issue example
- `evaluate()` returning `true` for the validated witnesses `(2,2,2)` and `(1,2,3)` and `false` for `(1,1,1)` and `(3,3,3)` after converting to zero-based config indices
- invalid configs (wrong length and out-of-range capacity index) returning `false`
- brute-force finding exactly 5 satisfying assignments for the canonical 3-link example
- serde round-trip for the model

**Step 2: Run test to verify it fails**

Run: `cargo test capacity_assignment --lib`
Expected: FAIL because `CapacityAssignment` and its test module do not exist yet.

**Step 3: Write minimal implementation**

Do not implement production code in this task. Leave the failing test as the red state for the next task.

**Step 4: Run test to verify it still fails for the expected reason**

Run: `cargo test capacity_assignment --lib`
Expected: FAIL with missing type/module errors, not unrelated syntax errors.

**Step 5: Commit**

Do not commit yet; combine with Task 2 after the model exists and the tests are green.

### Task 2: Implement the `CapacityAssignment` model and register it in the crate

**Files:**
- Create: `src/models/misc/capacity_assignment.rs`
- Modify: `src/models/misc/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`
- Test: `src/unit_tests/models/misc/capacity_assignment.rs`

**Step 1: Write the failing test**

Use the red tests from Task 1 as the only driver for this task.

**Step 2: Run test to verify it fails**

Run: `cargo test capacity_assignment --lib`
Expected: FAIL for the missing model implementation.

**Step 3: Write minimal implementation**

Implement:
- `ProblemSchemaEntry` with constructor-facing fields: `capacities`, `cost`, `delay`, `cost_budget`, `delay_budget`
- `CapacityAssignment` struct with serde derives and constructor validation for:
  - non-empty capacity list
  - rectangular `cost` and `delay` matrices
  - `cost.len() == delay.len()` and each row length matching `capacities.len()`
  - monotonicity (`cost[i][j] <= cost[i][j+1]`, `delay[i][j] >= delay[i][j+1]`)
- getters `num_links()`, `num_capacities()`, `capacities()`, `cost()`, `delay()`, `cost_budget()`, `delay_budget()`
- `Problem` impl with `dims() == vec![num_capacities; num_links]`
- `evaluate()` that rejects bad config lengths / indices and checks both budgets
- `SatisfactionProblem` impl
- `declare_variants! { default sat CapacityAssignment => "num_capacities^num_links" }`
- `canonical_model_example_specs()` using the corrected 3-link YES instance from the issue
- module/export wiring in `src/models/misc/mod.rs`, `src/models/mod.rs`, and `src/lib.rs`

**Step 4: Run test to verify it passes**

Run: `cargo test capacity_assignment --lib`
Expected: PASS for the new model tests.

**Step 5: Commit**

```bash
git add src/models/misc/capacity_assignment.rs src/models/misc/mod.rs src/models/mod.rs src/lib.rs src/unit_tests/models/misc/capacity_assignment.rs
git commit -m "feat: add CapacityAssignment model"
```

### Task 3: Add failing CLI tests for `pred create CapacityAssignment`

**Files:**
- Modify: `problemreductions-cli/src/commands/create.rs`
- Reference: `problemreductions-cli/src/cli.rs`

**Step 1: Write the failing test**

Add `create.rs` unit tests that exercise:
- successful creation with dedicated CapacityAssignment arguments
- informative failures for mismatched row widths / row counts
- rejection of non-monotone cost or delay rows

Use a concrete CLI shape for this plan:
- `--capacities 1,2,3`
- `--cost-matrix "1,3,6;2,4,7;1,2,5"`
- `--delay-matrix "8,4,1;7,3,1;6,3,1"`
- `--cost-budget 10`
- `--delay-budget 12`

**Step 2: Run test to verify it fails**

Run: `cargo test -p problemreductions-cli capacity_assignment`
Expected: FAIL because the CLI flags / match arm do not exist yet.

**Step 3: Write minimal implementation**

Do not add production CLI code in this task. Leave the red test in place for Task 4.

**Step 4: Run test to verify it still fails for the expected reason**

Run: `cargo test -p problemreductions-cli capacity_assignment`
Expected: FAIL with missing flag / match-arm behavior, not unrelated parser errors.

**Step 5: Commit**

Do not commit yet; combine with Task 4 after the CLI path is green.

### Task 4: Implement CLI creation support and help text

**Files:**
- Modify: `problemreductions-cli/src/cli.rs`
- Modify: `problemreductions-cli/src/commands/create.rs`
- Test: `problemreductions-cli/src/commands/create.rs`

**Step 1: Write the failing test**

Use the red tests from Task 3 as the driver.

**Step 2: Run test to verify it fails**

Run: `cargo test -p problemreductions-cli capacity_assignment`
Expected: FAIL because `pred create CapacityAssignment` is still unsupported.

**Step 3: Write minimal implementation**

Implement:
- new `CreateArgs` fields for `--cost-matrix`, `--delay-matrix`, `--cost-budget`, `--delay-budget`
- `all_data_flags_empty()` updates for the new flags
- help-table entry and usage example in `problemreductions-cli/src/cli.rs`
- parsing helpers in `create.rs` for rectangular `u64` matrices using the existing semicolon/comma matrix style
- `create()` match arm constructing `CapacityAssignment::new(...)`
- CLI tests proving valid creation and the targeted validation failures above

Do not add a short alias; canonical-name lookup is already case-insensitive through the registry.

**Step 4: Run test to verify it passes**

Run: `cargo test -p problemreductions-cli capacity_assignment`
Expected: PASS for the new CLI tests.

**Step 5: Commit**

```bash
git add problemreductions-cli/src/cli.rs problemreductions-cli/src/commands/create.rs
git commit -m "feat: add CapacityAssignment CLI support"
```

## Batch 2: Paper Entry

### Task 5: Add the paper definition and paper-aligned regression test

**Files:**
- Modify: `docs/paper/reductions.typ`
- Modify: `src/unit_tests/models/misc/capacity_assignment.rs`

**Step 1: Write the failing test**

Add a `test_capacity_assignment_paper_example` that:
- constructs the exact 3-link paper example
- checks the documented satisfying config
- uses `BruteForce::find_all_satisfying()` to confirm the documented count of 5 satisfying assignments

**Step 2: Run test to verify it fails**

Run: `cargo test capacity_assignment_paper_example --lib`
Expected: FAIL because the paper-aligned test is not written yet.

**Step 3: Write minimal implementation**

Update `docs/paper/reductions.typ` with:
- display-name entry for `CapacityAssignment`
- `problem-def("CapacityAssignment")` with the formal GJ definition
- brief background and the corrected Van Sickle/Chandy IFIP 1977 citation
- algorithm paragraph noting brute force and pseudo-polynomial DP with citations
- a worked 3-link example consistent with the canonical example data
- `pred-commands()` based on the canonical example export pattern

Then finish the paper-aligned unit test.

**Step 4: Run test to verify it passes**

Run:
- `cargo test capacity_assignment_paper_example --lib`
- `make paper`

Expected: PASS for the test and a clean Typst build.

**Step 5: Commit**

```bash
git add docs/paper/reductions.typ src/unit_tests/models/misc/capacity_assignment.rs
git commit -m "docs: add CapacityAssignment paper entry"
```

### Task 6: Final verification before push

**Files:**
- Verify only; no new files unless generated tracked outputs change legitimately

**Step 1: Write the failing test**

No new tests. This task is verification-only.

**Step 2: Run test to verify it fails**

Skip. Use fresh full-project verification instead.

**Step 3: Write minimal implementation**

No implementation changes unless verification exposes a real failure. If it does, fix the smallest failing scope first and re-run the relevant command before returning to the full suite.

**Step 4: Run test to verify it passes**

Run:
- `make test`
- `make clippy`
- `make paper`

Inspect:
- `git status --short`

Expected: all commands exit 0; only intended tracked changes remain staged or ready to stage.

**Step 5: Commit**

If verification fixes were required:

```bash
git add -A
git commit -m "chore: finish CapacityAssignment verification"
```
