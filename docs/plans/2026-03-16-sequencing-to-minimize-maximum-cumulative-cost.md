# Sequencing to Minimize Maximum Cumulative Cost Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the `SequencingToMinimizeMaximumCumulativeCost` decision model, its registry/CLI/example-db/test wiring, and the matching paper entry for issue #500.

**Architecture:** Implement this as a `misc` satisfaction problem with constructor-facing fields `costs: Vec<i64>`, `precedences: Vec<(usize, usize)>`, and `bound: i64`. Encode schedules with Lehmer-code dimensions `[n, n-1, ..., 1]`, decode to a permutation during evaluation, reject precedence violations, and accept exactly those schedules whose running cumulative cost never exceeds `bound`.

**Tech Stack:** Rust library crate, registry inventory metadata, clap-based CLI creation flow, example-db canonical examples, Typst paper docs.

---

## Batch 1: Model, registration, CLI, tests

### Task 1: Add the core model and its failing tests

**Files:**
- Create: `src/models/misc/sequencing_to_minimize_maximum_cumulative_cost.rs`
- Create: `src/unit_tests/models/misc/sequencing_to_minimize_maximum_cumulative_cost.rs`
- Modify: `src/models/misc/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`

**Step 1: Write the failing tests**

Add tests that pin down the intended representation and semantics:
- construction/getters: `costs()`, `precedences()`, `bound()`, `num_tasks()`, `num_precedences()`
- `dims()` uses Lehmer code (`[n, n-1, ..., 1]`)
- `evaluate()` accepts the issue’s feasible schedule and rejects:
  - a Lehmer digit outside its domain
  - a precedence-violating schedule
  - a schedule whose cumulative maximum exceeds `bound`
- brute-force finds a satisfying schedule for the paper/issue example and finds none for a cyclic-precedence or too-tight instance
- serde round-trip

Use the issue example as the main satisfaction instance:
- `costs = [2, -1, 3, -2, 1, -3]`
- `precedences = [(0, 2), (1, 2), (1, 3), (2, 4), (3, 5), (4, 5)]`
- `bound = 4`
- schedule `t2, t1, t4, t3, t5, t6` (0-indexed `[1, 0, 3, 2, 4, 5]`) should be encoded to its Lehmer form and asserted satisfying

**Step 2: Run the tests to verify they fail**

Run: `cargo test sequencing_to_minimize_maximum_cumulative_cost --lib`

Expected: failures because the model file and test module link do not exist yet.

**Step 3: Write the minimal model implementation**

Implement `src/models/misc/sequencing_to_minimize_maximum_cumulative_cost.rs` with:
- `inventory::submit!` schema metadata
- `#[derive(Debug, Clone, Serialize, Deserialize)]` model struct
- constructor validations:
  - `costs.len()` defines `num_tasks`
  - every precedence endpoint is in range
- getters: `costs()`, `precedences()`, `bound()`, `num_tasks()`, `num_precedences()`
- `Problem` impl with `Metric = bool`
- Lehmer-code `dims()`
- permutation decoder helper
- cumulative-cost checker over decoded schedule
- `impl SatisfactionProblem`
- `crate::declare_variants! { default sat ... => "factorial(num_tasks)" }`
- test-module link at file end

Register the model in:
- `src/models/misc/mod.rs`
- `src/models/mod.rs`
- `src/lib.rs`

**Step 4: Run the tests to verify they pass**

Run: `cargo test sequencing_to_minimize_maximum_cumulative_cost --lib`

Expected: the new model tests pass.

**Step 5: Commit**

```bash
git add src/models/misc/sequencing_to_minimize_maximum_cumulative_cost.rs \
  src/unit_tests/models/misc/sequencing_to_minimize_maximum_cumulative_cost.rs \
  src/models/misc/mod.rs src/models/mod.rs src/lib.rs
git commit -m "Add SequencingToMinimizeMaximumCumulativeCost model"
```

### Task 2: Wire example-db and trait consistency

**Files:**
- Modify: `src/models/misc/sequencing_to_minimize_maximum_cumulative_cost.rs`
- Modify: `src/models/misc/mod.rs`
- Modify: `src/unit_tests/trait_consistency.rs`

**Step 1: Write the failing tests**

Add:
- a canonical model example builder in the model file under `#[cfg(feature = "example-db")]`
- a `trait_consistency` entry for the new model
- a paper/example consistency test in the model test file that:
  - constructs the exact issue example
  - checks the documented satisfying Lehmer code
  - checks the brute-force satisfying set is non-empty and includes a satisfying schedule

**Step 2: Run the focused tests**

Run: `cargo test trait_consistency sequencing_to_minimize_maximum_cumulative_cost --lib`

Expected: failures until the example-db hook and consistency wiring are added.

**Step 3: Implement the wiring**

Add the canonical example spec in the model file and extend `src/models/misc/mod.rs` so the misc example registry includes it. Update `src/unit_tests/trait_consistency.rs` with `check_problem_trait(...)` for a small instance.

**Step 4: Re-run the focused tests**

Run: `cargo test trait_consistency sequencing_to_minimize_maximum_cumulative_cost --lib`

Expected: passing tests.

**Step 5: Commit**

```bash
git add src/models/misc/sequencing_to_minimize_maximum_cumulative_cost.rs \
  src/models/misc/mod.rs \
  src/unit_tests/trait_consistency.rs
git commit -m "Wire examples and trait checks for issue #500"
```

### Task 3: Add CLI create support

**Files:**
- Modify: `problemreductions-cli/src/cli.rs`
- Modify: `problemreductions-cli/src/commands/create.rs`

**Step 1: Write the failing CLI tests**

Add or extend `pred create` tests covering:
- explicit creation with `--costs`, `--bound`, and optional `--precedence-pairs`
- validation that precedence endpoints are within range
- omission of `--precedence-pairs` for an unconstrained instance

Prefer constructor-facing CLI fields:
- `--costs`
- `--bound`
- `--precedence-pairs`

**Step 2: Run the CLI tests to verify they fail**

Run: `cargo test create:: sequencing_to_minimize_maximum_cumulative_cost`

Expected: failures because the new flags/help/match arm do not exist yet.

**Step 3: Implement CLI support**

Update:
- `CreateArgs` with a `costs` string flag for this problem
- `all_data_flags_empty()` to include the new flag
- the create help table with a `SequencingToMinimizeMaximumCumulativeCost --costs, --bound [--precedence-pairs]` entry
- `problem_examples()` / usage text in `create.rs`
- the main `create()` match to parse `i64` costs, reuse `--precedence-pairs`, validate endpoint ranges against `costs.len()`, and serialize the new model

**Step 4: Re-run the CLI tests**

Run: `cargo test create:: sequencing_to_minimize_maximum_cumulative_cost`

Expected: passing CLI tests.

**Step 5: Commit**

```bash
git add problemreductions-cli/src/cli.rs problemreductions-cli/src/commands/create.rs
git commit -m "Add pred create support for issue #500"
```

## Batch 2: Paper entry

### Task 4: Document the model in the paper

**Files:**
- Modify: `docs/paper/reductions.typ`
- Modify: `docs/paper/references.bib` (only if one of the needed citations is missing)

**Step 1: Write the paper changes**

Add:
- display-name entry for `SequencingToMinimizeMaximumCumulativeCost`
- `problem-def("SequencingToMinimizeMaximumCumulativeCost")`
- background and algorithm notes citing:
  - Garey & Johnson A5 SS7
  - Abdel-Wahab (1976) for NP-completeness / register-sufficiency connection
  - Abdel-Wahab & Kameda (1978) for the series-parallel polynomial case
  - Monma & Sidney (1979), correcting the issue’s wrong year
- a worked example using the same 6-task instance as the model tests
- explicit explanation that the example schedule keeps the running cumulative cost at most `K = 4`

Keep the paper example aligned with the canonical example/tested Lehmer code from Batch 1.

**Step 2: Build the paper**

Run: `make paper`

Expected: Typst compiles successfully and the new `problem-def` renders without missing display-name or example-db references.

**Step 3: Commit**

```bash
git add docs/paper/reductions.typ
git commit -m "Document issue #500 in the paper"
```

## Final verification and review

### Task 5: Full verification

**Files:**
- Modify as needed from verification fixes

**Step 1: Run repo verification**

Run:
- `cargo test sequencing_to_minimize_maximum_cumulative_cost --lib`
- `cargo test trait_consistency --lib`
- `cargo test create:: --package problemreductions-cli`
- `make test`
- `make clippy`

Expected: all commands pass.

**Step 2: Run implementation review**

Run the repo-local review skill after Batch 1 and again after any verification-driven fixes if needed. Auto-fix structural issues before pushing.

**Step 3: Clean up the plan file after implementation**

When the code and docs are complete, remove this plan file before the final implementation push:

```bash
git rm docs/plans/2026-03-16-sequencing-to-minimize-maximum-cumulative-cost.md
git commit -m "chore: remove issue #500 plan file"
```
