# SequencingToMinimizeWeightedTardiness Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the `SequencingToMinimizeWeightedTardiness` model as a decision/satisfaction problem with brute-force-compatible permutation encoding, CLI creation support, canonical example data, tests, and paper documentation.

**Architecture:** Implement a new `misc` model that follows the Lehmer-code permutation pattern already used by `FlowShopScheduling` and `MinimumTardinessSequencing`. The model stores `lengths`, `weights`, `deadlines`, and `bound`, decodes schedule permutations, computes total weighted tardiness for the induced single-machine schedule, and evaluates to `true` exactly when the permutation is valid and the total weighted tardiness is at most `bound`. Batch 1 covers code, registry, CLI, example-db, and tests. Batch 2 covers the Typst paper entry and bibliography, after Batch 1 makes the canonical example export available.

**Tech Stack:** Rust workspace (`problemreductions` and `problemreductions-cli`), serde/inventory registry, brute-force solver, Typst paper, GitHub issue `#498`, companion rule issue `#473`.

---

## Issue Notes

- Treat this as the decision form from Garey & Johnson SS5: given lengths `l_j`, weights `w_j`, deadlines `d_j`, and bound `K`, ask whether some one-machine schedule has total weighted tardiness at most `K`.
- Reuse the issue's 5-task example data, but fix the bound inconsistency called out in the issue comments.
- Brute-force verification on the issue instance shows a unique optimal schedule `(t_1, t_2, t_5, t_4, t_3)` with total weighted tardiness `13`.
- Use `K = 13` for the canonical YES example and `K = 12` for the corresponding NO test.
- Associated rule issue already exists: `#473 [Rule] 3-Partition to Sequencing to Minimize Weighted Tardiness`, so this model is not orphaned.

## Batch 1: Model, CLI, Example-DB, Tests

### Task 1: Write the failing model tests first

**Files:**
- Create: `src/unit_tests/models/misc/sequencing_to_minimize_weighted_tardiness.rs`
- Modify: `src/unit_tests/trait_consistency.rs`

**Step 1: Write the failing test file**

Cover the concrete behaviors below before any production code exists:
- construction/accessors/dims/problem metadata
- total weighted tardiness helper on the issue example
- YES evaluation for the optimal Lehmer code `[0, 0, 2, 1, 0]` with `K = 13`
- NO evaluation for the same schedule under `K = 12`
- invalid configs: wrong length and invalid Lehmer digits
- brute-force solver returns a satisfying schedule for `K = 13` and none for `K = 12`
- serde round-trip
- trait consistency registration entry

**Step 2: Run the focused tests to verify RED**

Run: `cargo test sequencing_to_minimize_weighted_tardiness --lib`

Expected: compile failure because the new model module and test target do not exist yet.

**Step 3: Add the trait-consistency failing assertions**

Add:
- `check_problem_trait(&SequencingToMinimizeWeightedTardiness::new(...), "SequencingToMinimizeWeightedTardiness")`

Run: `cargo test trait_consistency --lib`

Expected: compile failure because the type is not defined or not re-exported yet.

**Step 4: Commit the red state if the branch policy allows partial commits**

```bash
git add src/unit_tests/models/misc/sequencing_to_minimize_weighted_tardiness.rs src/unit_tests/trait_consistency.rs
git commit -m "test: add failing tests for weighted tardiness sequencing"
```

### Task 2: Implement the new model and core wiring

**Files:**
- Create: `src/models/misc/sequencing_to_minimize_weighted_tardiness.rs`
- Modify: `src/models/misc/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`

**Step 1: Implement the model with the smallest code needed to satisfy Task 1**

The new file should include:
- `ProblemSchemaEntry` with fields `lengths`, `weights`, `deadlines`, `bound`
- struct definition with serde derives
- constructor `new(lengths: Vec<u64>, weights: Vec<u64>, deadlines: Vec<u64>, bound: u64)`
- accessors `lengths()`, `weights()`, `deadlines()`, `bound()`, `num_tasks()`
- helper(s) to decode Lehmer code into job order
- helper `total_weighted_tardiness(&self, config: &[usize]) -> Option<u128>` or equivalent safe numeric return
- `Problem` impl with `type Metric = bool`, `dims() = [n, n-1, ..., 1]`, and `evaluate()` returning `true` iff the config is a valid Lehmer permutation with total weighted tardiness `<= bound`
- `SatisfactionProblem` impl
- `declare_variants! { default sat SequencingToMinimizeWeightedTardiness => "factorial(num_tasks)", }`
- `#[cfg(test)]` link to the new unit test file
- `#[cfg(feature = "example-db")]` canonical example spec using the issue instance and the optimal config `[0, 0, 2, 1, 0]`

**Step 2: Wire the model into exports**

Update module/re-export files so the type is visible through:
- `crate::models::misc::*`
- `crate::models::*`
- `crate::prelude::*`

Also extend `src/models/misc/mod.rs` so the example-db aggregator includes the new canonical example spec.

**Step 3: Run the focused tests to reach GREEN**

Run:
- `cargo test sequencing_to_minimize_weighted_tardiness --lib`
- `cargo test trait_consistency --lib`

Expected: the new tests and trait-consistency entries pass.

**Step 4: Refactor only after green**

If needed, extract small private helpers so the model matches the style of `FlowShopScheduling` and `MinimumTardinessSequencing`. Do not add solver features beyond what the tests require.

**Step 5: Commit**

```bash
git add src/models/misc/sequencing_to_minimize_weighted_tardiness.rs src/models/misc/mod.rs src/models/mod.rs src/lib.rs src/unit_tests/models/misc/sequencing_to_minimize_weighted_tardiness.rs src/unit_tests/trait_consistency.rs
git commit -m "feat: add sequencing to minimize weighted tardiness model"
```

### Task 3: Add CLI creation support and user-facing help

**Files:**
- Modify: `problemreductions-cli/src/commands/create.rs`
- Modify: `problemreductions-cli/src/cli.rs`

**Step 1: Write or extend the failing CLI-facing tests if nearby coverage exists**

If there is an established CLI create test location for similar models, add a focused regression there. If not, keep this task implementation-focused and rely on targeted command execution for verification.

**Step 2: Add the create-arm**

Implement `pred create SequencingToMinimizeWeightedTardiness` using existing flags:
- `--sizes` for processing lengths
- `--weights` for tardiness weights
- `--deadlines` for job deadlines
- `--bound` for the decision threshold

Requirements:
- parse all four inputs
- require equal vector lengths
- convert `--bound` from the existing integer flag type to `u64` with a clear error if negative
- instantiate `SequencingToMinimizeWeightedTardiness::new(...)`

**Step 3: Update help text**

Add the new problem to the `CreateArgs` after-help table with the exact flag combination above. No new CLI flags should be introduced for this model.

**Step 4: Verify the CLI path**

Run:
- `cargo test create --package problemreductions-cli`
- `cargo run -p problemreductions-cli -- create SequencingToMinimizeWeightedTardiness --sizes 3,4,2,5,3 --weights 2,3,1,4,2 --deadlines 5,8,4,15,10 --bound 13`

Expected:
- tests stay green
- the command emits JSON for the new model with the expected fields

**Step 5: Commit**

```bash
git add problemreductions-cli/src/commands/create.rs problemreductions-cli/src/cli.rs
git commit -m "feat: add CLI creation for weighted tardiness sequencing"
```

### Task 4: Finalize Batch 1 verification

**Files:**
- Review only: all files touched in Tasks 1-3

**Step 1: Run the model-level verification suite**

Run:
- `cargo test sequencing_to_minimize_weighted_tardiness --lib`
- `cargo test trait_consistency --lib`
- `cargo test example_db --lib --features example-db`

Expected: all pass.

**Step 2: Run the repo quick check if Batch 1 is stable**

Run: `make check`

Expected: format, clippy, and test checks pass for the workspace.

**Step 3: Note any generated-file changes**

If verification regenerates tracked exports, inspect them and keep only expected changes related to the new model.

## Batch 2: Paper Entry and Citation Wiring

### Task 5: Add the paper entry and bibliography

**Files:**
- Modify: `docs/paper/reductions.typ`
- Modify: `docs/paper/references.bib`
- Test: `src/unit_tests/models/misc/sequencing_to_minimize_weighted_tardiness.rs`

**Step 1: Add bibliography entries**

Add the citations needed by the paper text if they are missing:
- Garey & Johnson book entry already exists; reuse it
- Lawler 1977 reference for strong NP-hardness / special equal-weight case
- Potts & Van Wassenhove 1985 branch-and-bound
- Tanaka, Fujikuma, and Araki 2009 exact algorithm

**Step 2: Register the display name**

Add:

```typst
"SequencingToMinimizeWeightedTardiness": [Sequencing to Minimize Weighted Tardiness],
```

**Step 3: Write the `problem-def(...)` entry**

Place it in the scheduling section near `MinimumTardinessSequencing`. The entry should:
- define the decision problem with lengths, weights, deadlines, and bound `K`
- mention standard notation `1 || sum w_j T_j`
- explain strong NP-hardness and the key tractable special cases from the issue context
- use the canonical example data and the optimal order `(t_1, t_2, t_5, t_4, t_3)`
- show that the completion times are `(3, 7, 10, 15, 17)` and only `t_3` is tardy, contributing `13`
- make the example visually clear from the start instead of changing `K` midway

**Step 4: Add the paper-example regression test**

Extend `src/unit_tests/models/misc/sequencing_to_minimize_weighted_tardiness.rs` with a `test_sequencing_to_minimize_weighted_tardiness_paper_example` that:
- constructs the exact paper instance
- asserts the canonical config `[0, 0, 2, 1, 0]` is satisfying for `K = 13`
- asserts the same ordering is not satisfying for `K = 12`
- uses brute force to confirm the YES instance has at least one satisfying schedule and the NO instance has none

**Step 5: Verify the paper batch**

Run:
- `cargo test sequencing_to_minimize_weighted_tardiness --lib`
- `make paper`

Expected: tests pass and the Typst paper builds successfully.

**Step 6: Commit**

```bash
git add docs/paper/reductions.typ docs/paper/references.bib src/unit_tests/models/misc/sequencing_to_minimize_weighted_tardiness.rs
git commit -m "docs: add paper entry for weighted tardiness sequencing"
```

## Final Verification and Review Handoff

### Task 6: Full repo verification and review preparation

**Files:**
- Review only: all touched files

**Step 1: Run final verification**

Run:
- `make check`
- `make paper`

If runtime is acceptable, also run:
- `make test`

**Step 2: Run implementation review**

Invoke the repo-local review skill after code is complete:
- `review-implementation`

Fix structural or Important quality findings before the implementation summary is posted to the PR.

**Step 3: Prepare the PR summary**

Call out:
- decision-model choice (not optimization-model choice)
- corrected canonical example (`K = 13` YES, `K = 12` NO)
- CLI flags used for construction
- any deviations from this plan
