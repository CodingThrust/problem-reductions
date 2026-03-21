# MinimumHittingSet Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add `MinimumHittingSet` as a new set-based optimization model for issue `#209`, with brute-force support, canonical example data, CLI creation support, and paper documentation.

**Architecture:** Implement `MinimumHittingSet` as a new `src/models/set/` optimization model whose binary variables correspond to universe elements, not sets. Reuse the `MinimumSetCovering` registration and example-db patterns, but define `dims()` over `universe_size`, treat a configuration as valid iff every input set contains a selected element, and minimize the number of selected elements. Keep this PR model-only: no reduction rules or ILP reduction in this branch.

**Tech Stack:** Rust workspace, `inventory` schema registry, `declare_variants!`, `BruteForce`, Typst paper, CLI `pred create`

---

## Issue Checklist

| Item | Value |
|---|---|
| Problem name | `MinimumHittingSet` |
| Category | `set` |
| Problem type | Optimization (`Direction::Minimize`) |
| Struct fields | `universe_size: usize`, `sets: Vec<Vec<usize>>` |
| Configuration space | `vec![2; universe_size]` |
| Feasibility rule | Every set in `sets` contains at least one selected universe element |
| Objective | Minimize the number of selected elements |
| Complexity string | `"2^universe_size"` |
| Solver for this PR | `BruteForce` only |
| Canonical example outcome | Universe `{0,1,2,3,4,5}`, sets `[{0,1,2},{0,3,4},{1,3,5},{2,4,5},{0,1,5},{2,3},{1,4}]`, optimal hitting set `{1,3,4}` with config `[0,1,0,1,1,0]` and value `3` |
| Associated open rules | `#200`, `#460`, `#462`, `#467` |

## Design Notes

- Use `SolutionSize<usize>` instead of a weight parameter. The issue explicitly models cardinality minimization, so a generic weight dimension would add unsupported surface area.
- Add `universe_size()` and `num_sets()` getters. `universe_size()` is required by the complexity string and `num_sets()` is useful for diagnostics/tests.
- Normalize each input set in the constructor by sorting/deduplicating it and assert that every element index is `< universe_size`. This prevents panics during evaluation on malformed instances.
- Treat empty input sets as making the instance unsatisfiable: every configuration should evaluate to `Invalid` if any set is empty.
- Do not add a short alias like `MHS`; the repo only adds short aliases that are standard in the literature.

## Batch 1: Model, Registry, Tests, and CLI

### Task 1: Add the failing model tests first

**Files:**
- Create: `src/unit_tests/models/set/minimum_hitting_set.rs`
- Modify: `src/unit_tests/problem_size.rs`
- Modify: `src/unit_tests/trait_consistency.rs`
- Modify: `src/lib.rs` (only if a missing re-export breaks test compilation)

**Step 1: Write the failing tests**

Add tests for:
- creation/accessors/dimensions
- valid vs invalid evaluation
- constructor normalization / out-of-range rejection
- brute-force optimum on the issue example
- serialization round-trip
- paper example consistency (`[0,1,0,1,1,0]` gives `Valid(3)`)
- `problem_size()` exposes `universe_size` and `num_sets`
- generic trait-consistency coverage includes `MinimumHittingSet`

**Step 2: Run the targeted tests to verify RED**

Run:

```bash
cargo test minimum_hitting_set --lib
```

Expected: compile failure because `MinimumHittingSet` does not exist yet.

**Step 3: Write the minimal implementation to satisfy the tests**

Create `src/models/set/minimum_hitting_set.rs` with:
- `ProblemSchemaEntry` registration
- `MinimumHittingSet` struct with `new()`, `universe_size()`, `num_sets()`, `sets()`, `get_set()`
- helpers such as `selected_elements()` / `is_valid_solution()` as needed
- `Problem` impl with `Metric = SolutionSize<usize>`, `dims() = vec![2; universe_size]`, `evaluate()`
- `OptimizationProblem` impl with `Direction::Minimize`
- `declare_variants! { default opt MinimumHittingSet => "2^universe_size", }`
- `#[cfg(test)]` link to the new unit test file

Mirror the style of `src/models/set/minimum_set_covering.rs`, but keep the semantics element-based rather than set-based.

**Step 4: Run the targeted tests to verify GREEN**

Run:

```bash
cargo test minimum_hitting_set --lib
```

Expected: the new model tests pass.

**Step 5: Commit**

```bash
git add src/models/set/minimum_hitting_set.rs src/unit_tests/models/set/minimum_hitting_set.rs src/unit_tests/problem_size.rs src/unit_tests/trait_consistency.rs
git commit -m "Add MinimumHittingSet model core"
```

### Task 2: Register the model and canonical example data

**Files:**
- Modify: `src/models/set/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`
- Modify: `src/models/set/minimum_hitting_set.rs`

**Step 1: Add the failing example/registration checks**

Extend the new model tests so they exercise:
- crate-level/prelude imports if needed
- `canonical_model_example_specs()` contents for `MinimumHittingSet`
- brute-force optimal config/value for the canonical example

**Step 2: Run the focused tests to verify RED**

Run:

```bash
cargo test minimum_hitting_set --features example-db --lib
```

Expected: failure until the model is exported and its canonical example is registered.

**Step 3: Wire the model into the registry surface**

- Add `pub(crate) mod minimum_hitting_set;` and `pub use minimum_hitting_set::MinimumHittingSet;` in `src/models/set/mod.rs`
- Extend the set example chain with `minimum_hitting_set::canonical_model_example_specs()`
- Re-export `MinimumHittingSet` from `src/models/mod.rs` and the crate prelude in `src/lib.rs`
- Add `#[cfg(feature = "example-db")] canonical_model_example_specs()` in the model file using the issue example and `optimal_value = {"Valid": 3}`

**Step 4: Run the focused tests to verify GREEN**

Run:

```bash
cargo test minimum_hitting_set --features example-db --lib
```

Expected: the example-db aware tests pass.

**Step 5: Commit**

```bash
git add src/models/set/mod.rs src/models/mod.rs src/lib.rs src/models/set/minimum_hitting_set.rs
git commit -m "Register MinimumHittingSet in the model catalog"
```

### Task 3: Add CLI creation support and CLI tests

**Files:**
- Modify: `problemreductions-cli/src/commands/create.rs`
- Modify: `problemreductions-cli/src/cli.rs`
- Modify: `problemreductions-cli/tests/cli_tests.rs`
- Modify: `problemreductions-cli/src/problem_name.rs` only if a lowercase full-name lookup test proves it is needed

**Step 1: Write the failing CLI tests**

Add CLI coverage for:
- `pred create MinimumHittingSet --universe 6 --sets "0,1,2;0,3,4;1,3,5;2,4,5;0,1,5;2,3;1,4"`
- optional `--example` flow if existing set-model tests cover that pattern
- help table text includes `MinimumHittingSet              --universe, --sets`

**Step 2: Run the targeted CLI tests to verify RED**

Run:

```bash
cargo test -p problemreductions-cli minimum_hitting_set
```

Expected: failure because CLI creation/help do not know the new model yet.

**Step 3: Implement the minimal CLI support**

- Add a `create()` match arm analogous to `MinimumSetCovering`, but call `MinimumHittingSet::new(universe, sets)`
- Update the create-help table in `problemreductions-cli/src/cli.rs`
- Only touch alias resolution if the tests show the registry does not already resolve the canonical name adequately

**Step 4: Run the targeted CLI tests to verify GREEN**

Run:

```bash
cargo test -p problemreductions-cli minimum_hitting_set
```

Expected: the new CLI tests pass.

**Step 5: Commit**

```bash
git add problemreductions-cli/src/commands/create.rs problemreductions-cli/src/cli.rs problemreductions-cli/tests/cli_tests.rs problemreductions-cli/src/problem_name.rs
git commit -m "Add MinimumHittingSet CLI creation support"
```

## Batch 2: Paper Entry

### Task 4: Document the model in the paper after the implementation exists

**Files:**
- Modify: `docs/paper/reductions.typ`
- Modify: `docs/paper/references.bib` only if the chosen citations are missing
- Modify: `src/unit_tests/models/set/minimum_hitting_set.rs` if the paper example wording requires tightening the paper-example assertions

**Step 1: Write or tighten the failing paper-example test**

Ensure the unit test enforces the issue example exactly:
- instance matches the paper example
- config `[0,1,0,1,1,0]` is valid
- brute-force confirms optimum value `3`

**Step 2: Run the paper-example test to verify RED if needed**

Run:

```bash
cargo test test_minimum_hitting_set_paper_example --lib
```

Expected: fail only if the paper-facing example and code diverge.

**Step 3: Add the paper entry**

In `docs/paper/reductions.typ`:
- add `"MinimumHittingSet": [Minimum Hitting Set]` to `display-name`
- add a `problem-def("MinimumHittingSet")` entry near the other set problems
- cite `@karp1972` for historical context
- explain the duality with Set Covering and the Vertex Cover special case
- use the issue example with a set-system diagram and the optimal hitting set `{1,3,4}`

Prefer the `MinimumSetCovering` entry as the layout template, but highlight selected universe elements instead of selected sets.

**Step 4: Verify the paper build**

Run:

```bash
cargo test test_minimum_hitting_set_paper_example --lib
make paper
```

Expected: the paper example test passes and the Typst build succeeds.

**Step 5: Commit**

```bash
git add docs/paper/reductions.typ docs/paper/references.bib src/unit_tests/models/set/minimum_hitting_set.rs
git commit -m "Document MinimumHittingSet in the paper"
```

## Final Verification

After both batches are complete, run:

```bash
make test
make clippy
git status --short
```

Expected:
- tests pass
- clippy passes
- only intended tracked files are modified

## issue-to-pr Cleanup

After implementation is committed and verified:

1. Delete this file with `git rm docs/plans/2026-03-21-minimum-hitting-set.md`
2. Commit the removal: `git commit -m "chore: remove plan file after implementation"`
3. Post the PR implementation summary comment
4. Push the branch
