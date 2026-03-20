# SubsetSum To Knapsack Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the `SubsetSum -> Knapsack` reduction rule, register a canonical example, and document the rule in the paper so the reduction graph exposes the path `KSatisfiability -> SubsetSum -> Knapsack -> QUBO`.

**Architecture:** This reduction is a sat-to-opt embedding against the existing optimization `Knapsack` model, not a decision-threshold variant. The implementation maps each subset-sum element to a knapsack item with identical weight and value, uses the subset target as capacity, stores no extra mapping state beyond the target instance, and relies on source-side evaluation after solution extraction to distinguish SAT from UNSAT cases. Because `SubsetSum` uses `BigUint` and `Knapsack` uses `i64`, the reduction must perform checked conversion and fail loudly when the source instance cannot be represented in the target model.

**Tech Stack:** Rust, cargo test, example-db exports, Typst paper, GitHub PR pipeline helpers.

---

## Batch 1: Rule, Tests, and Canonical Example

### Task 1: Add the failing rule tests first

**Files:**
- Create: `src/unit_tests/rules/subsetsum_knapsack.rs`
- Read for reference: `src/unit_tests/rules/sat_maximumindependentset.rs`
- Read for helper APIs: `src/rules/test_helpers.rs`

**Step 1: Write the failing tests**

Add focused tests that define the intended behavior before implementing the rule:
- `test_subsetsum_to_knapsack_closed_loop`:
  - Build `SubsetSum::new(vec![3u32, 7, 1, 8, 4, 12, 5], 15u32)`.
  - Reduce to `Knapsack`.
  - Assert `assert_satisfaction_round_trip_from_optimization_target(&source, &reduction, "SubsetSum->Knapsack closed loop")`.
- `test_subsetsum_to_knapsack_target_structure`:
  - Assert `weights == [3, 7, 1, 8, 4, 12, 5]`.
  - Assert `values == [3, 7, 1, 8, 4, 12, 5]`.
  - Assert `capacity == 15`.
  - Assert `num_items == source.num_elements()`.
- `test_subsetsum_to_knapsack_unsat_extracts_non_solution`:
  - Build an UNSAT source such as `SubsetSum::new(vec![2u32, 4, 6], 5u32)`.
  - Reduce and solve the target with `solve_optimization_problem`.
  - Extract the source config and assert `!source.evaluate(&extracted)`.
- `test_subsetsum_to_knapsack_panics_on_i64_overflow`:
  - Build a source with one size larger than `i64::MAX` using `BigUint`.
  - Assert the reduction panics with a message that mentions conversion to `i64`.
- `#[cfg(feature = "example-db")] test_subsetsum_to_knapsack_canonical_example_spec`:
  - Find the rule example spec by id.
  - Assert the exported example names are `SubsetSum` and `Knapsack`.
  - Assert the example contains at least one witness.

**Step 2: Run the focused test file to verify RED**

Run:
```bash
cargo test --features "ilp-highs example-db" subsetsum_to_knapsack -- --nocapture
```

Expected:
- Compile or test failure because the rule module and registrations do not exist yet.

**Step 3: Commit note**

Do not commit in RED state. Move directly to implementation after confirming the failure is for the missing rule.

### Task 2: Implement the reduction rule and registration

**Files:**
- Create: `src/rules/subsetsum_knapsack.rs`
- Modify: `src/rules/mod.rs`
- Read for reference: `src/rules/sat_maximumindependentset.rs`
- Read for model semantics: `src/models/misc/subset_sum.rs`
- Read for model semantics: `src/models/misc/knapsack.rs`

**Step 1: Write the minimal production code**

Implement `src/rules/subsetsum_knapsack.rs` with:
- `ReductionSubsetSumToKnapsack { target: Knapsack }`
- `ReductionResult` impl:
  - `type Source = SubsetSum`
  - `type Target = Knapsack`
  - `target_problem()` returns `&self.target`
  - `extract_solution()` returns `target_solution.to_vec()`
- `#[reduction(overhead = { num_items = "num_elements" })]`
- `impl ReduceTo<Knapsack> for SubsetSum`
  - Convert each source size to `i64` with `i64::try_from(...)`
  - Use the converted vector for both target weights and target values
  - Convert `target()` to `i64` for capacity using the same checked conversion
  - Panic with a clear message if any conversion does not fit in `i64`
- `#[cfg(test)] #[path = "../unit_tests/rules/subsetsum_knapsack.rs"] mod tests;`

Update `src/rules/mod.rs` to register `pub(crate) mod subsetsum_knapsack;` and extend `canonical_rule_example_specs()` with `subsetsum_knapsack::canonical_rule_example_specs()`.

**Step 2: Run the focused tests to verify GREEN**

Run:
```bash
cargo test --features "ilp-highs example-db" subsetsum_to_knapsack -- --nocapture
```

Expected:
- The new rule tests pass.
- Any failures should now be behavioral, not missing-module errors.

**Step 3: Refactor if needed**

Keep the implementation minimal. Only extract a local helper if the conversion logic is duplicated enough to obscure the rule.

### Task 3: Add the canonical example in the rule module

**Files:**
- Modify: `src/rules/subsetsum_knapsack.rs`
- Read for reference: `src/rules/knapsack_qubo.rs`
- Read for example helpers: `src/example_db/specs.rs`

**Step 1: Implement the canonical example builder**

Inside `#[cfg(feature = "example-db")] pub(crate) fn canonical_rule_example_specs() -> Vec<RuleExampleSpec>`:
- Reuse the issue’s 7-element canonical instance:
  - Source: `SubsetSum::new(vec![3u32, 7, 1, 8, 4, 12, 5], 15u32)`
  - Canonical witness: `source_config = target_config = vec![1, 0, 0, 0, 0, 1, 0]`
- Export it with `rule_example_with_witness::<_, Knapsack>(...)`
- Use id `subsetsum_to_knapsack`

The witness must be valid in both directions, but it does not need to enumerate all optimal or satisfying assignments.

**Step 2: Re-run the focused tests**

Run:
```bash
cargo test --features "ilp-highs example-db" subsetsum_to_knapsack -- --nocapture
```

Expected:
- The canonical example test now passes as well.

### Task 4: Verify the rule and exports before the paper batch

**Files:**
- Modify if generated and tracked: exported JSON/SVG files touched by the commands below

**Step 1: Run broader verification for the new rule**

Run:
```bash
cargo test --features "ilp-highs example-db" subsetsum_to_knapsack -- --include-ignored
cargo run --features "example-db" --example export_examples
cargo run --example export_graph
cargo run --example export_schemas
```

Expected:
- Tests pass.
- Export commands succeed.
- Generated fixtures now contain the new rule example data needed by the paper.

**Step 2: Inspect the exported example data**

Check that the exported rule example contains:
- Source problem `SubsetSum`
- Target problem `Knapsack`
- The canonical witness pair with identical source/target configs

**Step 3: Commit Batch 1**

Run:
```bash
git add src/rules/subsetsum_knapsack.rs src/rules/mod.rs src/unit_tests/rules/subsetsum_knapsack.rs src/example_db/fixtures/examples.json docs/src/reduction_graph.json docs/src/problem_schemas.json
git commit -m "Implement #138: add SubsetSum to Knapsack reduction"
```

If the export paths differ from the expected tracked files, inspect `git status --short` and stage the actual tracked outputs instead of inventing paths.

## Batch 2: Paper Entry and Final Verification

### Task 5: Document the rule in the paper

**Files:**
- Modify: `docs/paper/reductions.typ`
- Read for reference: nearby `SubsetSum` and `Knapsack` problem definitions
- Read for reference: `reduction-rule("KSatisfiability", "SubsetSum", ...)`
- Read for reference: `reduction-rule("Knapsack", "QUBO", ...)`

**Step 1: Add the reduction-rule entry**

Insert a new `#let` block and `#reduction-rule("SubsetSum", "Knapsack", ...)` between the existing `KSatisfiability -> SubsetSum` and `Knapsack -> QUBO` sections.

The paper entry must reflect the implemented semantics:
- State that the reduction maps each element size to both item weight and item value, with capacity equal to the subset target.
- Explain the sat-to-opt correctness condition: SubsetSum is satisfiable iff the target Knapsack optimum reaches value `T`.
- Mention the checked `BigUint -> i64` representation boundary in prose or the worked example if needed, but do not claim the target supports arbitrary precision.
- Use exported example data rather than hardcoded duplicate constants where practical.
- Make the witness semantics explicit: the fixture stores one canonical witness, while multiple satisfying subsets may exist.

**Step 2: Add a worked example**

Use the exported 7-element canonical instance and walk through:
- Source set and target value
- Target weights, values, and capacity
- Why the chosen witness reaches value 15 exactly
- Why equality of weights and values makes extracted solutions identical

**Step 3: Build the paper**

Run:
```bash
make paper
```

Expected:
- Typst compiles successfully.
- No completeness warnings for an undocumented `SubsetSum -> Knapsack` edge remain.

### Task 6: Final verification, cleanup, and push-ready state

**Files:**
- Delete after execution: `docs/plans/2026-03-20-subsetsum-to-knapsack.md`

**Step 1: Run final verification**

Run:
```bash
make fmt
cargo test --features "ilp-highs example-db" subsetsum_to_knapsack -- --include-ignored
make clippy
git status --short
```

Expected:
- Formatting clean
- Rule-specific tests pass
- Clippy passes
- Only intended tracked files are modified

**Step 2: Commit paper and generated artifacts**

Run:
```bash
git add docs/paper/reductions.typ
git add -A
git commit -m "Document #138: add SubsetSum to Knapsack paper entry"
```

**Step 3: Remove the plan file after implementation**

Run:
```bash
git rm docs/plans/2026-03-20-subsetsum-to-knapsack.md
git commit -m "chore: remove plan file after implementation"
```

**Step 4: Pre-push summary**

Before push, prepare the PR implementation summary comment with:
- The new rule module and test coverage
- The canonical example and exported fixture updates
- The paper entry
- Any deviation from the original issue text:
  - used optimization Knapsack semantics
  - added checked `BigUint -> i64` conversion boundary

**Step 5: Push**

Run:
```bash
git push
```

Stop if verification reveals unrelated changes or failures. Fix them before pushing.
