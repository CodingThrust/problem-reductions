# ProductionPlanning Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the `ProductionPlanning` model from issue #513 as a `misc` satisfaction problem with brute-force support, CLI creation, canonical example data, unit tests, and a paper entry, without bundling any reduction-rule work into this PR.

**Architecture:** Implement `ProductionPlanning` as a per-period bounded-integer decision problem: each configuration chooses production amounts `x_i in {0, ..., c_i}` and `evaluate()` checks capacity bounds, nonnegative prefix inventory, and the total production + holding + setup cost budget. Register it through `ProblemSchemaEntry` and `declare_variants!`, expose a canonical example via `canonical_model_example_specs()`, add `pred create ProductionPlanning` support with dedicated vector flags, and document the corrected SS21 / Florian-Lenstra-Rinnooy Kan (1980) references plus the cleaned example from the issue comments.

**Tech Stack:** Rust workspace, serde/inventory registry, clap CLI parsing, Typst paper, GitHub issue context, Garey & Johnson SS21, Florian-Lenstra-Rinnooy Kan (1980).

---

## Inputs And Constraints

- Issue: `#513 [Model] ProductionPlanning`
- Associated rule already exists: `#488 [Rule] Partition to Production Planning`, so this model will not be orphaned
- Category: `src/models/misc/`
- Problem type: satisfaction (`Metric = bool`, `SatisfactionProblem`)
- Constructor shape:
  - `num_periods: usize`
  - `demands: Vec<u64>`
  - `capacities: Vec<u64>`
  - `setup_costs: Vec<u64>`
  - `production_costs: Vec<u64>`
  - `inventory_costs: Vec<u64>`
  - `cost_bound: u64`
- Size getters required by the complexity expression:
  - `num_periods() -> usize`
  - `max_capacity() -> u64`
- Complexity string: `"(max_capacity + 1)^num_periods"`
- Source-of-truth example for tests and paper:
  - demands `[5, 3, 7, 2, 8, 5]`
  - capacities `[12, 12, 12, 12, 12, 12]`
  - setup costs `[10, 10, 10, 10, 10, 10]`
  - production costs `[1, 1, 1, 1, 1, 1]`
  - inventory costs `[1, 1, 1, 1, 1, 1]`
  - cost bound `80`
  - satisfying plan `[8, 0, 10, 0, 12, 0]`
- Keep ILP discussion out of scope for this PR. This issue is a model-only pipeline item; reduction-rule work stays in separate rule issues / PRs.

## Batch 1: Model, Registration, CLI, Tests

### Task 1: Scaffold The Model And Core Registration

**Files:**
- Create: `src/models/misc/production_planning.rs`
- Modify: `src/models/misc/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`
- Create: `src/unit_tests/models/misc/production_planning.rs`

**Step 1: Write the failing tests**

Add initial tests in `src/unit_tests/models/misc/production_planning.rs` for:
- constructor/getter round-trip
- `dims()` equals `capacities[i] + 1` per period
- `num_periods()` and `max_capacity()` getters
- constructor panics on mismatched vector lengths
- constructor panics when any capacity cannot fit into `usize` for `dims()`

**Step 2: Run the targeted test to verify RED**

Run:

```bash
cargo test production_planning --lib
```

Expected:
- compile or test failure because the model/module does not exist yet

**Step 3: Write the minimal implementation**

Implement `src/models/misc/production_planning.rs` with:
- `inventory::submit!` `ProblemSchemaEntry`
- `ProductionPlanning` struct deriving `Debug, Clone, Serialize, Deserialize`
- `new(...)` constructor that validates:
  - `num_periods > 0`
  - every per-period vector length equals `num_periods`
  - every capacity fits in `usize` and `capacity + 1` fits in `usize` for `dims()`
- accessors for all fields
- size getters `num_periods()` and `max_capacity()`
- `Problem` impl:
  - `NAME = "ProductionPlanning"`
  - `Metric = bool`
  - `variant() = variant_params![]`
  - `dims() = capacities.iter().map(|c| (c + 1) as usize).collect()`
  - placeholder `evaluate()` logic sufficient for the creation tests to compile
- `SatisfactionProblem` impl
- `declare_variants! { default sat ProductionPlanning => "(max_capacity + 1)^num_periods", }`
- test link at file bottom

Wire the new model through:
- `src/models/misc/mod.rs`
- `src/models/mod.rs`
- `src/lib.rs` prelude exports

**Step 4: Run the targeted test to verify GREEN**

Run:

```bash
cargo test production_planning --lib
```

Expected:
- the creation/getter/module wiring tests pass
- semantic tests still fail or remain to be added later

**Step 5: Commit**

```bash
git add src/models/misc/production_planning.rs src/models/misc/mod.rs src/models/mod.rs src/lib.rs src/unit_tests/models/misc/production_planning.rs
git commit -m "Add ProductionPlanning model scaffold"
```

### Task 2: Implement The Real Evaluation Semantics

**Files:**
- Modify: `src/models/misc/production_planning.rs`
- Modify: `src/unit_tests/models/misc/production_planning.rs`

**Step 1: Write the failing tests**

Extend the test file with behavior-driven tests for:
- the issue example plan `[8, 0, 10, 0, 12, 0]` evaluates to `true`
- a plan that exceeds a period capacity evaluates to `false`
- a plan that creates negative prefix inventory evaluates to `false`
- a plan that exceeds the total cost bound evaluates to `false`
- wrong config length evaluates to `false`
- `BruteForce::find_satisfying()` returns `Some(_)` on the issue example instance
- serde round-trip preserves all vectors and `cost_bound`

**Step 2: Run the targeted test to verify RED**

Run:

```bash
cargo test production_planning --lib
```

Expected:
- the new semantic tests fail because `evaluate()` is incomplete

**Step 3: Write the minimal implementation**

Finish `evaluate()` using issue #513 semantics:
- reject wrong config length
- reject any `x_i > capacities[i]`
- compute cumulative production and cumulative demand in `u128`
- reject any prefix where cumulative production `<` cumulative demand
- compute inventory `I_i` as the nonnegative prefix surplus
- compute total cost as:
  - `sum_i production_costs[i] * x_i`
  - `+ sum_i inventory_costs[i] * I_i`
  - `+ sum_{x_i > 0} setup_costs[i]`
- compare against `cost_bound` using `u128` to avoid overflow during intermediate arithmetic
- return `true` iff all constraints hold and total cost is within budget

Add small private helpers only if they remove duplication cleanly, for example:
- a prefix-balance helper
- a checked `u128` cost accumulator

**Step 4: Run the targeted test to verify GREEN**

Run:

```bash
cargo test production_planning --lib
```

Expected:
- all `production_planning` unit tests pass

**Step 5: Commit**

```bash
git add src/models/misc/production_planning.rs src/unit_tests/models/misc/production_planning.rs
git commit -m "Implement ProductionPlanning evaluation"
```

### Task 3: Add Canonical Example Data And CLI Creation Support

**Files:**
- Modify: `src/models/misc/production_planning.rs`
- Modify: `src/models/misc/mod.rs`
- Modify: `problemreductions-cli/src/cli.rs`
- Modify: `problemreductions-cli/src/commands/create.rs`

**Step 1: Write the failing tests**

Add or extend CLI tests in `problemreductions-cli/src/commands/create.rs` for:
- `pred create ProductionPlanning --num-periods 6 --demands 5,3,7,2,8,5 --capacities 12,12,12,12,12,12 --setup-costs 10,10,10,10,10,10 --production-costs 1,1,1,1,1,1 --inventory-costs 1,1,1,1,1,1 --cost-budget 80`
- missing required vectors produce a clear usage error
- mismatched vector lengths produce a clear validation error
- `pred create --example ProductionPlanning` succeeds once the canonical example is registered

**Step 2: Run the targeted test to verify RED**

Run:

```bash
cargo test create::tests::production_planning
```

Expected:
- failures because the CLI flags/match arm/example data do not exist yet

**Step 3: Write the minimal implementation**

In `src/models/misc/production_planning.rs`:
- add `canonical_model_example_specs()` using the cleaned issue example and satisfying config

In `src/models/misc/mod.rs`:
- include `production_planning::canonical_model_example_specs()` in the misc example chain

In `problemreductions-cli/src/cli.rs`:
- add new `CreateArgs` fields:
  - `demands`
  - `setup_costs`
  - `production_costs`
  - `inventory_costs`
- include them in `all_data_flags_empty()`
- add a `ProductionPlanning` row to the create help table
- add at least one concrete example command to the create help text

In `problemreductions-cli/src/commands/create.rs`:
- import `ProductionPlanning`
- add a `ProductionPlanning` match arm
- require:
  - `--num-periods`
  - `--demands`
  - `--capacities`
  - `--setup-costs`
  - `--production-costs`
  - `--inventory-costs`
  - `--cost-budget`
- parse the per-period vectors as `Vec<u64>` with one shared helper for consistent error messages
- validate that all vector lengths equal `num_periods`
- construct `ProductionPlanning::new(...)`

Do **not** add a manual `problem_name.rs` alias unless testing proves it is needed. The registry already resolves canonical names case-insensitively.

**Step 4: Run the targeted test to verify GREEN**

Run:

```bash
cargo test create::tests::production_planning
cargo test production_planning --lib
```

Expected:
- the new CLI tests pass
- canonical example registration compiles and example-backed creation works

**Step 5: Commit**

```bash
git add src/models/misc/production_planning.rs src/models/misc/mod.rs problemreductions-cli/src/cli.rs problemreductions-cli/src/commands/create.rs
git commit -m "Add ProductionPlanning CLI support"
```

### Task 4: Batch-1 Verification

**Files:**
- Modify: any files required by verification fixes

**Step 1: Run the batch verification commands**

Run:

```bash
make test clippy
```

Expected:
- tests and clippy pass for the implemented model/CLI work

**Step 2: Fix any failures**

If verification fails:
- make the minimal correction
- rerun the exact failing command
- keep fixes in scope for `ProductionPlanning`

**Step 3: Commit any verification-driven fixes**

```bash
git add -A
git commit -m "Fix ProductionPlanning verification issues"
```

Only create this commit if verification required code changes.

## Batch 2: Paper Entry And Paper-Example Alignment

### Task 5: Add The Paper Entry And Lock The Paper Example

**Files:**
- Modify: `docs/paper/reductions.typ`
- Modify: `src/unit_tests/models/misc/production_planning.rs`

**Step 1: Write the failing test**

Add a `test_production_planning_paper_example` that:
- reconstructs the paper/example-db instance
- checks the documented plan `[8, 0, 10, 0, 12, 0]` evaluates to `true`
- confirms `BruteForce::find_satisfying()` returns at least one satisfying configuration for that instance

**Step 2: Run the targeted test to verify RED**

Run:

```bash
cargo test production_planning_paper_example --lib
```

Expected:
- failure because the paper-aligned test or example details are not fully wired yet

**Step 3: Write the minimal implementation**

Update `docs/paper/reductions.typ`:
- add `"ProductionPlanning": [Production Planning]` to the display-name dictionary
- add `#problem-def("ProductionPlanning")[...]` in the scheduling/misc section
- use the corrected references and wording:
  - Garey & Johnson `SS21`
  - Florian, Lenstra, and Rinnooy Kan `(1980)`
- describe the model as a lot-sizing / production-planning feasibility problem with setup, production, and inventory costs
- present the cleaned six-period example from the fixed issue body
- include reproducibility commands using the canonical example helper pattern already used elsewhere in the paper

Keep the paper content aligned with the implemented model:
- no rule theorem in this PR
- no invented smaller replacement example
- no reintroduction of the old trial-and-error example text

**Step 4: Run the targeted paper verification**

Run:

```bash
cargo test production_planning_paper_example --lib
make paper
```

Expected:
- the paper-example test passes
- the Typst paper builds successfully

**Step 5: Commit**

```bash
git add docs/paper/reductions.typ src/unit_tests/models/misc/production_planning.rs
git commit -m "Document ProductionPlanning in paper"
```

## Final Verification Checklist

Before handing back to `issue-to-pr` cleanup/push steps, rerun and confirm:

```bash
make test clippy
make paper
git status --short
```

Success criteria:
- `ProductionPlanning` is registered and exported
- `pred create ProductionPlanning ...` works with the new flags
- canonical example data is available
- the issue example plan is encoded consistently across tests and paper
- no reduction rule was bundled into this PR
