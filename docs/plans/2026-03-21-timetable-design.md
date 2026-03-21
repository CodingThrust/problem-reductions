# TimetableDesign Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the `TimetableDesign` satisfaction model from issue `#511`, register it across the library and CLI, and document the verified worked example in the paper.

**Architecture:** Implement `TimetableDesign` as a `misc` satisfaction problem whose configuration is a flattened binary tensor `f(c,t,h)` in craftsman-major, task-next, period-last order. Keep this PR scoped to the model and its brute-force-compatible verifier; do not bundle any new reduction rule. Reuse the issue’s verified timetable as the canonical example and make the paper entry read from the same example-db instance so the code, exports, and documentation stay aligned.

**Tech Stack:** Rust, serde, inventory registry, clap CLI, Typst paper, example-db, `cargo test`, `make test`, `make clippy`, `make paper`.

---

## Inputs Locked From Issue #511

- Problem name: `TimetableDesign`
- Category: `src/models/misc/`
- Problem type: `SatisfactionProblem` (`Metric = bool`)
- Core fields: `num_periods`, `num_craftsmen`, `num_tasks`, `craftsman_avail`, `task_avail`, `requirements`
- Complexity string: `"2^(num_craftsmen * num_tasks * num_periods)"`
- Associated rule already exists: issue `#486` (`[Rule] 3SAT to Timetable Design`)
- Solver scope for this PR: brute-force only; no ILP reduction rule in this branch
- Canonical worked example: the 5 craftsmen / 5 tasks / 3 periods YES instance from issue `#511`

## Representation Decisions

- Store availability tables as dense boolean matrices:
  - `craftsman_avail[c][h]`
  - `task_avail[t][h]`
  - `requirements[c][t]`
- Flatten the schedule variable `f(c,t,h)` to config index
  - `idx = ((c * num_tasks) + t) * num_periods + h`
- `dims()` returns `vec![2; num_craftsmen * num_tasks * num_periods]`
- `evaluate()` must reject:
  - wrong config length
  - any assignment outside `A(c) ∩ A(t)`
  - two tasks for the same craftsman in one period
  - two craftsmen on the same task in one period
  - any `(c,t)` pair whose assigned periods do not match `R(c,t)` exactly

## Batch 1: Model, Registration, Example, CLI, Tests

### Task 1: Write the failing TimetableDesign model tests

**Files:**
- Create: `src/unit_tests/models/misc/timetable_design.rs`
- Reference: `src/unit_tests/models/misc/resource_constrained_scheduling.rs`
- Reference: `src/unit_tests/models/misc/staff_scheduling.rs`
- Reference: `src/unit_tests/models/formula/sat.rs`

**Step 1: Write the failing test**

Add targeted tests for:
- constructor/getter coverage and `dims()`
- a valid timetable instance from a small toy example
- invalid configs for each constraint family
- brute-force solver on a tiny satisfiable instance
- serde round-trip
- issue/paper example validity by checking the provided satisfying config directly (no brute force on the large example)

**Step 2: Run test to verify it fails**

Run:

```bash
cargo test timetable_design --lib
```

Expected: compile failure because the `TimetableDesign` model module does not exist yet.

**Step 3: Commit**

Do not commit yet. This task intentionally stays red until Task 2.

### Task 2: Implement the TimetableDesign model and wire it into the crate

**Files:**
- Create: `src/models/misc/timetable_design.rs`
- Modify: `src/models/misc/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`

**Step 1: Write minimal implementation**

Implement:
- `ProblemSchemaEntry` metadata for `TimetableDesign`
- `TimetableDesign::new(...)` with validation that matrix dimensions match the declared counts
- inherent getters: `num_periods()`, `num_craftsmen()`, `num_tasks()`, `craftsman_avail()`, `task_avail()`, `requirements()`
- a private `index(c, t, h)` helper and any tiny decoding helpers needed by tests/example code
- `Problem` + `SatisfactionProblem` impls
- `declare_variants! { default sat TimetableDesign => "2^(num_craftsmen * num_tasks * num_periods)" }`
- `#[cfg(test)]` link to the new unit-test file
- module/export registrations in `src/models/misc/mod.rs`, `src/models/mod.rs`, and `src/lib.rs` prelude/root re-exports

**Step 2: Run tests to verify green for the model slice**

Run:

```bash
cargo test timetable_design --lib
```

Expected: the new unit tests pass.

**Step 3: Commit**

```bash
git add src/models/misc/timetable_design.rs src/models/misc/mod.rs src/models/mod.rs src/lib.rs src/unit_tests/models/misc/timetable_design.rs
git commit -m "Add TimetableDesign model"
```

### Task 3: Register the canonical example and align tests to the issue’s worked timetable

**Files:**
- Modify: `src/models/misc/timetable_design.rs`
- Modify: `src/models/misc/mod.rs`

**Step 1: Write the failing example assertions first**

Extend the model tests to include:
- a helper that builds the exact issue example
- the exact satisfying config from the issue in flattened `(c,t,h)` order
- assertions that `evaluate()` returns `true`
- negative checks produced by flipping one forced assignment or adding a conflicting assignment

**Step 2: Run the targeted tests to verify they fail for the missing example hookup**

Run:

```bash
cargo test timetable_design::tests::test_timetable_design_paper_example_is_valid --lib
```

Expected: failure until the canonical example spec exists and the test helper can reuse it cleanly.

**Step 3: Write minimal implementation**

Add `canonical_model_example_specs()` in `src/models/misc/timetable_design.rs` using the verified issue instance and its satisfying config, then register it in the `src/models/misc/mod.rs` example chain.

**Step 4: Run tests to verify green**

Run:

```bash
cargo test timetable_design --lib
```

Expected: all TimetableDesign model tests pass, including the issue example check.

**Step 5: Commit**

```bash
git add src/models/misc/timetable_design.rs src/models/misc/mod.rs src/unit_tests/models/misc/timetable_design.rs
git commit -m "Add TimetableDesign canonical example"
```

### Task 4: Add CLI creation support and CLI-level tests

**Files:**
- Modify: `problemreductions-cli/src/cli.rs`
- Modify: `problemreductions-cli/src/commands/create.rs`

**Step 1: Write the failing CLI tests first**

Add tests in `problemreductions-cli/src/commands/create.rs` for:
- `pred create TimetableDesign ...` producing JSON with the expected type and dimensions
- malformed availability/requirement matrices returning user-facing errors instead of panicking
- help-flag naming/hints for any new TimetableDesign-specific flags if the helper tests need updates

**Step 2: Run the targeted CLI tests to verify red**

Run:

```bash
cargo test create_timetable_design --package problemreductions-cli
```

Expected: failure because the CLI flags and create-arm do not exist yet.

**Step 3: Write minimal implementation**

Add TimetableDesign CLI support:
- new `CreateArgs` fields for `--num-periods`, `--num-craftsmen`, `--num-tasks`, `--craftsman-avail`, `--task-avail`
- reuse `--requirements` with a TimetableDesign-specific matrix parser
- add the problem to the `after_help` “Flags by problem type” table
- add a `"TimetableDesign"` match arm in `create()` with validation and a clear usage string
- add parsing helpers that mirror existing boolean-matrix helpers instead of inventing ad hoc string parsing

**Step 4: Run tests to verify green**

Run:

```bash
cargo test create_timetable_design --package problemreductions-cli
```

Expected: the new CLI tests pass.

**Step 5: Commit**

```bash
git add problemreductions-cli/src/cli.rs problemreductions-cli/src/commands/create.rs
git commit -m "Add TimetableDesign CLI support"
```

### Task 5: Batch-1 verification

**Files:**
- No new files; verification only

**Step 1: Run focused verification**

Run:

```bash
cargo test timetable_design --lib
cargo test create_timetable_design --package problemreductions-cli
```

Expected: both commands pass before starting the paper batch.

**Step 2: Commit**

No new commit if the tree is clean.

## Batch 2: Paper Entry And Final Verification

### Task 6: Add the paper entry for TimetableDesign

**Files:**
- Modify: `docs/paper/reductions.typ`

**Step 1: Write the failing paper check first**

Run:

```bash
make paper
```

Expected: if the display name or `problem-def("TimetableDesign")` entry is missing, the paper/export checks fail or the model is omitted from the paper coverage.

**Step 2: Write minimal implementation**

Add:
- `"TimetableDesign": [Timetable Design]` to the display-name dictionary
- a `#problem-def("TimetableDesign")[...][...]` entry that:
  - states the formal definition from issue `#511`
  - cites Garey & Johnson / Even-Itai-Shamir
  - explains the flattened assignment viewpoint used in the code
  - uses the issue’s canonical example and satisfying timetable
  - presents the worked schedule in a table or similarly compact visualization appropriate for a 5×5×3 example

**Step 3: Run the paper build**

Run:

```bash
make paper
```

Expected: the paper compiles cleanly and includes the new problem entry.

**Step 4: Commit**

```bash
git add docs/paper/reductions.typ
git commit -m "Document TimetableDesign in paper"
```

### Task 7: Final verification before push

**Files:**
- No new files; verification only

**Step 1: Run repo verification**

Run:

```bash
make test
make clippy
make paper
git status --short
```

Expected:
- `make test` passes
- `make clippy` passes
- `make paper` passes
- `git status --short` shows only intended tracked changes (and no lingering plan artifacts after the implementation phase deletes this file)

**Step 2: Commit**

No extra verification-only commit unless a final fix was required.
