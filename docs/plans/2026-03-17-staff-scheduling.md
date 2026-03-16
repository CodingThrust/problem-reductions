# StaffScheduling Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the `StaffScheduling` satisfaction model, wire it into the registry/CLI/example-db/paper, and verify the issue's worked example plus brute-force behavior.

**Architecture:** Model `StaffScheduling` as a satisfaction problem in `src/models/misc/` with one integer-valued variable per schedule pattern, where variable `i` ranges over `0..=num_workers` and denotes how many workers use schedule `i`. Feasibility checks enforce the workforce budget and per-period coverage constraints directly from the Garey-Johnson definition. CLI creation should accept schedule rows and requirements as explicit integer lists; canonical examples and paper content should reuse the issue's 7-day staffing instance.

**Tech Stack:** Rust workspace, inventory-based schema registry, Clap CLI, serde JSON, Typst paper, `make test`, `make clippy`, `make paper`.

---

## Context To Preserve

- Issue: `#512 [Model] StaffScheduling`
- Kind: model
- Solver guidance from the issue: brute force is valid now; ILP is a later rule
- Companion rule exists: `#487 [Rule] X3C to Staff Scheduling`
- Reviewer note to fold in:
  - use the published Bartholdi/Orlin/Ratliff year `1980`
  - provide a concrete complexity string
  - keep the example aligned with the issue's satisfying assignment

## Batch 1: Model + Registry + CLI + Tests (add-model Steps 1-5.5)

### Task 1: Write the failing model tests first

**Files:**
- Create: `src/unit_tests/models/misc/staff_scheduling.rs`
- Reference: `src/unit_tests/models/misc/flow_shop_scheduling.rs`
- Reference: `src/unit_tests/models/misc/sequencing_within_intervals.rs`

**Step 1: Write the failing test**

Add tests that exercise:
- constructor/getters/dims for the issue instance
- `evaluate()` on the issue's satisfying assignment `[1, 1, 1, 1, 0]`
- `evaluate()` on invalid configs:
  - wrong length
  - entry larger than `num_workers`
  - total workers exceeding `num_workers`
  - unmet coverage in at least one period
- brute-force solver finds a satisfying config for the issue instance
- brute-force solver returns `None` on a deliberately infeasible small instance
- serde round-trip
- a `test_staff_scheduling_paper_example` anchored to the exact worked example

**Step 2: Run test to verify it fails**

Run: `cargo test test_staff_scheduling -- --nocapture`
Expected: FAIL because `StaffScheduling` does not exist yet.

**Step 3: Write minimal implementation**

Do not start production code before the failure is confirmed.

**Step 4: Run test to verify it passes**

Run only after Task 2 is implemented.

**Step 5: Commit**

Defer commit until Batch 1 is green.

### Task 2: Implement the `StaffScheduling` model and register it

**Files:**
- Create: `src/models/misc/staff_scheduling.rs`
- Modify: `src/models/misc/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`

**Step 1: Write the minimal model implementation**

Implement:
- `ProblemSchemaEntry` with no variants and constructor-facing fields:
  - `shifts_per_schedule: usize`
  - `schedules: Vec<Vec<bool>>`
  - `requirements: Vec<u64>`
  - `num_workers: u64`
- `StaffScheduling` struct with getters:
  - `num_periods()`
  - `shifts_per_schedule()`
  - `schedules()`
  - `requirements()`
  - `num_workers()`
  - `num_schedules()`
- constructor validation:
  - all schedules have the same length as `requirements.len()`
  - every schedule has exactly `shifts_per_schedule` ones
- `dims()` = `vec![num_workers as usize + 1; num_schedules]`
- `evaluate()` returns `true` iff:
  - config length matches number of schedules
  - each assignment count is within range
  - total assigned workers is `<= num_workers`
  - coverage in every period meets or exceeds `requirements`
- `declare_variants! { default sat StaffScheduling => "(num_workers + 1)^num_schedules" }`
- test link at file bottom

**Step 2: Register the model**

Export the type from:
- `src/models/misc/mod.rs`
- `src/models/mod.rs`
- `src/lib.rs` prelude

**Step 3: Run the focused tests**

Run: `cargo test test_staff_scheduling -- --nocapture`
Expected: PASS

**Step 4: Refactor only if needed**

Keep helper methods small and explicit; avoid premature solver abstractions.

### Task 3: Add canonical example-db and CLI creation support

**Files:**
- Modify: `src/models/misc/staff_scheduling.rs`
- Modify: `problemreductions-cli/src/cli.rs`
- Modify: `problemreductions-cli/src/commands/create.rs`

**Step 1: Write the failing CLI/example tests or targeted assertions**

Add/extend tests as needed so the following behaviors are covered:
- canonical model example exists for `StaffScheduling`
- `pred create StaffScheduling ...` can construct the issue example

If no existing CLI unit test is the right home, add a focused model-db test near `src/unit_tests/example_db.rs`.

**Step 2: Run the failing scope**

Run: `cargo test example_db -- --nocapture`
Expected: FAIL until the canonical example is registered.

**Step 3: Implement minimal wiring**

Add:
- canonical example builder in `staff_scheduling.rs` under `#[cfg(feature = "example-db")]`
- `misc::canonical_model_example_specs()` aggregation entry
- CLI flags in `CreateArgs`:
  - `--schedules`
  - `--requirements`
  - `--num-workers`
  - reuse `--k` for `shifts-per-schedule`
- create help text entry for `StaffScheduling`
- `create.rs` match arm that parses semicolon-separated schedule rows like `"1,1,1,1,1,0,0;0,1,1,1,1,1,0"`

**Step 4: Run the focused tests**

Run:
- `cargo test example_db -- --nocapture`
- `cargo test create -- --nocapture`

Expected: PASS

### Task 4: Finish Batch 1 verification

**Files:**
- Reuse all files from Tasks 1-3

**Step 1: Run Batch 1 verification**

Run:
- `cargo test test_staff_scheduling -- --nocapture`
- `cargo test example_db -- --nocapture`
- `cargo test create -- --nocapture`

**Step 2: Commit Batch 1**

```bash
git add src/models/misc/staff_scheduling.rs \
  src/models/misc/mod.rs \
  src/models/mod.rs \
  src/lib.rs \
  src/unit_tests/models/misc/staff_scheduling.rs \
  problemreductions-cli/src/cli.rs \
  problemreductions-cli/src/commands/create.rs
git commit -m "Implement StaffScheduling model"
```

## Batch 2: Paper Entry (add-model Step 6)

### Task 5: Document `StaffScheduling` in the Typst paper

**Files:**
- Modify: `docs/paper/reductions.typ`

**Step 1: Write the failing paper change**

Add:
- display-name dictionary entry for `StaffScheduling`
- `problem-def("StaffScheduling")` with the formal definition from the issue
- 1-3 sentence background mentioning workforce scheduling and the cyclic-ones special case
- best-known algorithm prose:
  - general NP-complete status from Garey & Johnson
  - 1980 cyclic-ones polynomial-time result
  - brute-force bound matching the implementation complexity string
- tutorial-style worked example matching the issue's 7-day instance and satisfying assignment

**Step 2: Run paper build**

Run: `make paper`
Expected: PASS

**Step 3: Add/confirm paper-example test alignment**

If the worked example changed during writing, update `test_staff_scheduling_paper_example` to match it exactly, then run:

`cargo test test_staff_scheduling_paper_example -- --nocapture`

Expected: PASS

**Step 4: Commit Batch 2**

```bash
git add docs/paper/reductions.typ src/unit_tests/models/misc/staff_scheduling.rs
git commit -m "Document StaffScheduling"
```

## Final Verification (add-model Step 7 + issue-to-pr execute)

### Task 6: Full repo verification and cleanup

**Files:**
- Reuse prior files

**Step 1: Run required checks**

Run:
- `make test`
- `make clippy`
- `make paper`

**Step 2: Review and fix**

Run repo review workflow after implementation, then address findings before push.

**Step 3: Final implementation commit**

```bash
git add -A
git commit -m "Implement #512: [Model] StaffScheduling"
```

**Step 4: Remove the plan file before push**

```bash
git rm docs/plans/2026-03-17-staff-scheduling.md
git commit -m "chore: remove plan file after implementation"
```
