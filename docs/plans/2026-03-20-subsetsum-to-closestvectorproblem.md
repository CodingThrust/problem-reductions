# SubsetSum to ClosestVectorProblem Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the `SubsetSum -> ClosestVectorProblem` reduction, its tests, canonical example, and paper entry so issue `#125` is fully implemented and review-ready.

**Architecture:** Model the classical lattice embedding with an `(n+1) x n` integer basis whose columns are the identity vectors extended with one sizes row, a target vector `(1/2, ..., 1/2, B)`, and binary bounds on every CVP variable. Because the current `ClosestVectorProblem` variants are `i32` and `f64`, implement the reduction against the default `ClosestVectorProblem<i32>` variant and fail fast if any `SubsetSum` size or target does not fit in `i32`.

**Tech Stack:** Rust workspace, `#[reduction]` registry macros, brute-force solver tests, example-db exports, Typst paper.

---

## Batch 1: Rule implementation, tests, and example data

### Task 1: Add rule tests first

**Files:**
- Create: `src/unit_tests/rules/subsetsum_closestvectorproblem.rs`
- Reference: `src/unit_tests/rules/ksatisfiability_subsetsum.rs`
- Reference: `src/rules/test_helpers.rs`

**Step 1: Write the failing tests**

Add tests for:
- `test_subsetsum_to_closestvectorproblem_closed_loop`
  - Build the issue example `SubsetSum::new(vec![3u32, 7, 1, 8], 11u32)`
  - Reduce with `ReduceTo::<ClosestVectorProblem<i32>>::reduce_to(&source)`
  - Use `assert_satisfaction_round_trip_from_optimization_target`
  - Assert the target has `num_basis_vectors() == 4`, `ambient_dimension() == 5`, and all bounds are binary
- `test_subsetsum_to_closestvectorproblem_structure`
  - Assert the four basis columns are `[1,0,0,0,3]`, `[0,1,0,0,7]`, `[0,0,1,0,1]`, `[0,0,0,1,8]`
  - Assert the target vector is `[0.5, 0.5, 0.5, 0.5, 11.0]`
- `test_subsetsum_to_closestvectorproblem_issue_example_minimizers`
  - Use `BruteForce::find_all_best`
  - Assert the two minimizers are exactly `vec![1,0,0,1]` and `vec![1,1,1,0]`
  - Assert `target.evaluate(...) == SolutionSize::Valid(1.0)` for each minimizer
- `test_subsetsum_to_closestvectorproblem_unsatisfiable_instance`
  - Use a small unsatisfiable instance such as `SubsetSum::new(vec![2u32, 4, 6], 5u32)`
  - Assert the best CVP objective is strictly greater than `sqrt(n as f64) / 2.0`
- `test_subsetsum_to_closestvectorproblem_panics_on_large_coefficients`
  - Use a `SubsetSum` size or target larger than `i32::MAX`
  - Assert reduction panics with a message that the values must fit in `i32`

**Step 2: Run the tests to verify RED**

Run: `cargo test subsetsum_to_closestvectorproblem -- --nocapture`

Expected: FAIL because the rule module and tests are not wired up yet.

### Task 2: Implement and register the reduction

**Files:**
- Create: `src/rules/subsetsum_closestvectorproblem.rs`
- Modify: `src/rules/mod.rs`
- Test: `src/unit_tests/rules/subsetsum_closestvectorproblem.rs`
- Reference: `src/rules/minimumvertexcover_maximumindependentset.rs`
- Reference: `src/rules/ksatisfiability_subsetsum.rs`
- Reference: `src/models/misc/subset_sum.rs`
- Reference: `src/models/algebraic/closest_vector_problem.rs`

**Step 1: Write the minimal implementation**

Implement:
- `ReductionSubsetSumToClosestVectorProblem` storing `ClosestVectorProblem<i32>`
- `ReductionResult` with direct solution extraction (`target_solution.to_vec()`)
- `#[reduction(overhead = { ambient_dimension = "num_elements + 1", num_basis_vectors = "num_elements" })]`
- `impl ReduceTo<ClosestVectorProblem<i32>> for SubsetSum`

Construction details:
- Convert each `BigUint` size and the target to `i32` via a helper that panics with a precise message on overflow
- For each element index `i`, build one column vector of length `n + 1`
- Set the `i`-th coordinate to `1`
- Set the last coordinate to the converted size
- Use `VarBounds::binary()` for all variables
- Use target vector `vec![0.5; n]` extended with the converted target sum as `f64`

Link the new test file at the bottom of the rule module and register the module in `src/rules/mod.rs`. Also add the rule to `canonical_rule_example_specs()` once the example builder exists.

**Step 2: Run the focused tests to verify GREEN**

Run: `cargo test subsetsum_to_closestvectorproblem -- --nocapture`

Expected: PASS for the new rule tests.

### Task 3: Add the canonical rule example and export-facing coverage

**Files:**
- Modify: `src/rules/subsetsum_closestvectorproblem.rs`
- Reference: `src/example_db/specs.rs`
- Reference: `src/rules/ksatisfiability_subsetsum.rs`

**Step 1: Add `canonical_rule_example_specs()`**

Use the issue example instance:
- Source: `SubsetSum::new(vec![3u32, 7, 1, 8], 11u32)`
- Canonical witness: `source_config = vec![1, 0, 0, 1]`
- Canonical target witness: `target_config = vec![1, 0, 0, 1]`

Expose it behind `#[cfg(feature = "example-db")]`, then add it to `src/rules/mod.rs` alongside the other rule example registrations.

**Step 2: Verify exports and targeted integration**

Run:
- `cargo test subsetsum_to_closestvectorproblem -- --nocapture`
- `cargo run --example export_graph`
- `cargo run --example export_schemas`
- `make regenerate-fixtures`

Expected:
- the targeted tests still pass
- exports regenerate cleanly
- the new example is present in `src/example_db/fixtures/examples.json`

## Batch 2: Paper entry and final verification

### Task 4: Document the reduction in the paper

**Files:**
- Modify: `docs/paper/reductions.typ`
- Reference: `docs/paper/reductions.typ` `KSatisfiability -> SubsetSum`
- Reference: `docs/paper/reductions.typ` `Knapsack -> QUBO`

**Step 1: Add the `reduction-rule("SubsetSum", "ClosestVectorProblem", ...)` entry**

Use the exported example fixture instead of hardcoded values. The entry should include:
- theorem body explaining the identity-block plus sizes-row embedding
- correctness argument stating that binary selections have constant identity contribution and the last coordinate is zero exactly when the subset sum hits `B`
- explicit solution extraction statement that the CVP binary vector is reused directly as the SubsetSum selection
- worked example using the canonical fixture, including the two satisfying assignments and the observed CVP minimum distance `1.0`
- citations to Lagarias-Odlyzko 1985 and Coster et al. 1992, with the note that the repository uses the CVP `1/2`-target adaptation

**Step 2: Build the paper**

Run: `make paper`

Expected: PASS with the new reduction entry rendered and no completeness warnings for the new rule.

### Task 5: Full verification and cleanup

**Files:**
- Verify workspace changes only in the new rule/test/example/paper/export outputs

**Step 1: Run final verification**

Run:
- `make test`
- `make clippy`

If either command exposes unrelated failures caused by export regeneration or the new rule, fix them before continuing.

**Step 2: Inspect git status**

Run: `git status --short`

Expected tracked changes:
- `docs/paper/reductions.typ`
- `src/rules/mod.rs`
- `src/rules/subsetsum_closestvectorproblem.rs`
- `src/unit_tests/rules/subsetsum_closestvectorproblem.rs`
- generated export/fixture files from the required export commands

Ignored `docs/src/reductions/` output should remain unstaged.

**Step 3: Commit guidance**

Use coherent commits such as:
- `Add plan for #125: [Rule] SubsetSum to ClosestVectorProblem`
- `Implement #125: [Rule] SubsetSum to ClosestVectorProblem`
- `chore: remove plan file after implementation`
