# ComparativeContainment Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the `ComparativeContainment` model, register it across the library/CLI/example-db/paper surfaces, and verify the issue's YES example and solver behavior end-to-end.

**Architecture:** Implement `ComparativeContainment<W>` as a new set-based satisfaction problem with one binary variable per universe element. A configuration selects a subset `Y` of the universe, and evaluation compares the total weights of `R`-sets and `S`-sets that contain `Y`; the model then plugs into the existing registry, brute-force solver, CLI `pred create`, canonical example-db, paper export, and trait-consistency checks.

**Tech Stack:** Rust workspace, serde, inventory registry, `WeightElement`, brute-force solver, Typst paper, GitHub issue #401.

---

## Batch 1: Model, Registry, CLI, Examples, Tests

### Task 1: Add failing model tests for the issue behavior

**Files:**
- Create: `src/unit_tests/models/set/comparative_containment.rs`
- Modify: `src/unit_tests/trait_consistency.rs`
- Reference: `src/unit_tests/models/set/minimum_set_covering.rs`
- Reference: `src/unit_tests/models/formula/sat.rs`

**Step 1: Write the failing tests**

Add tests that assume `ComparativeContainment` exists and cover:
- construction/getters (`universe_size`, `num_r_sets`, `num_s_sets`, weight accessors)
- containment helper behavior for a chosen `Y`
- `evaluate()` for the issue's YES example and NO example
- brute-force solver returning a satisfying assignment for the YES example and none for the NO example
- paper-example consistency using the issue's YES example (`Y = {0}`) and the claim that it satisfies the inequality
- trait-consistency registration with a small instance

**Step 2: Run the focused tests to verify RED**

Run:
```bash
cargo test comparative_containment --lib
```

Expected: compile failure because the model and exports do not exist yet.

**Step 3: Commit the red state**

Do not commit yet. Leave the tree dirty and move directly to implementation once the failure is observed.

### Task 2: Implement the new set model and register it in the library

**Files:**
- Create: `src/models/set/comparative_containment.rs`
- Modify: `src/models/set/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`

**Step 1: Write the minimal implementation**

Implement `ComparativeContainment<W = i32>` with:
- schema registration (`ProblemSchemaEntry`) including constructor-facing fields
- fields `universe_size`, `r_sets`, `s_sets`, `r_weights`, `s_weights`
- constructors for unit weights and explicit weights
- getters `universe_size()`, `num_r_sets()`, `num_s_sets()`, `r_sets()`, `s_sets()`, `r_weights()`, `s_weights()`
- helper(s) to check whether a config-selected subset `Y` is contained in a candidate set
- `Problem` impl with `Metric = bool`, `dims() = vec![2; universe_size]`, and `evaluate()` comparing the two containment sums
- `SatisfactionProblem` impl
- `declare_variants!` entries using the inferred size getter `universe_size`; default to the equal-weight variant and also register explicit weighted variants that match the chosen `W` support
- canonical model example spec for the issue's YES instance

**Step 2: Register module exports**

Wire the model through `src/models/set/mod.rs`, `src/models/mod.rs`, and `src/lib.rs`.

**Step 3: Run the focused tests to verify GREEN**

Run:
```bash
cargo test comparative_containment --lib
```

Expected: the new model tests compile and pass.

### Task 3: Add CLI discovery and creation support

**Files:**
- Modify: `problemreductions-cli/src/problem_name.rs`
- Modify: `problemreductions-cli/src/commands/create.rs`
- Modify: `problemreductions-cli/src/cli.rs`

**Step 1: Write the failing CLI test or failing create invocation**

Use an invocation that should work once the model is registered:

```bash
cargo run -p problemreductions-cli -- create ComparativeContainment --universe 4 --r-sets "0,1,2,3;0,1" --s-sets "0,1,2,3;2,3" --r-weights 2,5 --s-weights 3,6
```

Expected now: failure because the problem name and/or flags are not wired up yet.

**Step 2: Implement the minimal CLI support**

Add:
- alias resolution entry for `comparativecontainment`
- problem-specific create support in `commands/create.rs`
- any new CLI flags needed for two set-families and two weight vectors (`--r-sets`, `--s-sets`, `--r-weights`, `--s-weights`) plus `all_data_flags_empty()` and help text updates
- an example string in the create help output if the command uses the problem-specific help path

**Step 3: Re-run the invocation**

Run the same `cargo run -p problemreductions-cli -- create ...` command and verify it succeeds and emits JSON for the new problem.

### Task 4: Register example-db and trait consistency surfaces

**Files:**
- Modify: `src/example_db/model_builders.rs`
- Modify: `src/unit_tests/trait_consistency.rs`

**Step 1: Write the failing checks if still missing**

Run:
```bash
cargo test trait_consistency --lib
```

Expected: fail until the new model is added to the trait-consistency test.

**Step 2: Implement the registrations**

Make sure:
- the new canonical model example is reachable through the set-module example spec aggregation
- `test_all_problems_implement_trait_correctly` includes a `ComparativeContainment` instance

**Step 3: Re-run the focused checks**

Run:
```bash
cargo test trait_consistency --lib
```

Expected: pass.

### Task 5: Run the batch-1 verification

**Files:**
- No new files

**Step 1: Run the targeted verification**

Run:
```bash
cargo test comparative_containment --lib
cargo test trait_consistency --lib
cargo run -p problemreductions-cli -- create ComparativeContainment --universe 4 --r-sets "0,1,2,3;0,1" --s-sets "0,1,2,3;2,3" --r-weights 2,5 --s-weights 3,6 >/tmp/comparative_containment.json
```

Expected: all commands succeed.

**Step 2: Commit batch 1**

```bash
git add src/models/set/comparative_containment.rs src/models/set/mod.rs src/models/mod.rs src/lib.rs src/unit_tests/models/set/comparative_containment.rs src/unit_tests/trait_consistency.rs problemreductions-cli/src/problem_name.rs problemreductions-cli/src/commands/create.rs problemreductions-cli/src/cli.rs
git commit -m "Add ComparativeContainment model"
```

## Batch 2: Paper Entry and Full Verification

### Task 6: Add the paper entry after the example data exists

**Files:**
- Modify: `docs/paper/reductions.typ`
- Reference: `docs/paper/reductions.typ` (`problem-def("MinimumSetCovering")`)

**Step 1: Write the failing paper build**

Run:
```bash
make paper
```

Expected: fail or remain incomplete until the display-name and `problem-def("ComparativeContainment")` entry are added.

**Step 2: Implement the Typst entry**

Add:
- display-name dictionary entry for `ComparativeContainment`
- a `problem-def("ComparativeContainment")` block with formal definition, background, best-known exact algorithm note, and a worked example based on the issue's YES instance
- an explanation of why `Y = {1}` in 1-indexed paper notation satisfies the inequality for the example

**Step 3: Re-run the paper build**

Run:
```bash
make paper
```

Expected: pass.

### Task 7: Run the full verification for the issue branch

**Files:**
- No new files

**Step 1: Run project verification**

Run:
```bash
make test
make clippy
```

Expected: both pass.

**Step 2: Run implementation review**

Invoke the repo-local review workflow after code is in place:
- `.claude/skills/review-implementation/SKILL.md`

Fix any structural or quality findings before pushing.

**Step 3: Commit the documentation/review fixes**

```bash
git add docs/paper/reductions.typ src/example_db/model_builders.rs
git commit -m "Document ComparativeContainment model"
```

### Task 8: Prepare the branch for PR push

**Files:**
- Modify: `docs/plans/2026-03-16-comparative-containment.md` (remove after execution)

**Step 1: Remove the plan file**

```bash
git rm docs/plans/2026-03-16-comparative-containment.md
```

**Step 2: Verify the plan file is gone and expected generated exports are staged if needed**

Run:
```bash
git status --short
test ! -e docs/plans/2026-03-16-comparative-containment.md
```

**Step 3: Commit the cleanup**

```bash
git commit -m "chore: remove plan file after implementation"
```
