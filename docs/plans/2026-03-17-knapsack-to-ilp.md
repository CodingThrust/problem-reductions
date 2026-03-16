# Knapsack to ILP Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add a `Knapsack -> ILP<bool>` reduction with closed-loop tests, a canonical example-db fixture, and a paper entry that documents the binary ILP formulation.

**Architecture:** Mirror the existing direct optimization-to-ILP rules: create one binary ILP variable per item, add a single capacity constraint, maximize the item values, and keep solution extraction as the identity map on the item bits. Split implementation into two batches so the paper work happens only after the Rust rule, example export, and verification data exist.

**Tech Stack:** Rust workspace, reduction registry macros, `BruteForce` and `ILPSolver`, Typst paper, GitHub PR pipeline scripts.

---

## Batch 1: Rust Implementation, Registration, and Verification

### Task 1: Add the failing rule tests first

**Files:**
- Create: `src/unit_tests/rules/knapsack_ilp.rs`
- Reference: `src/unit_tests/rules/knapsack_qubo.rs`
- Reference: `src/unit_tests/rules/maximumclique_ilp.rs`
- Reference: `src/rules/test_helpers.rs`

**Step 1: Write the failing tests**

Add tests that cover:
- `test_knapsack_to_ilp_closed_loop` using the canonical 4-item example from issue `#639`
- `test_knapsack_to_ilp_structure` asserting `num_vars == num_items`, `num_constraints == 1`, `sense == ObjectiveSense::Maximize`, objective/value coefficients, and the single `<= capacity` constraint
- `test_knapsack_to_ilp_zero_capacity` asserting the optimal extracted source solution is the all-zero selection
- `#[cfg(feature = "example-db")] test_knapsack_to_ilp_canonical_example_spec`

Use `ILPSolver` for the closed-loop path and compare the extracted solution against the source optimum/value. Reuse `assert_optimization_round_trip_from_optimization_target` if it fits cleanly; otherwise assert validity and objective preservation directly.

**Step 2: Run the new test target to verify it fails**

Run: `cargo test --features ilp-solver test_knapsack_to_ilp -- --nocapture`

Expected: FAIL because `src/rules/knapsack_ilp.rs` and its registrations do not exist yet.

**Step 3: Commit the failing test scaffold**

Run:
```bash
git add src/unit_tests/rules/knapsack_ilp.rs
git commit -m "test: add Knapsack to ILP coverage"
```

Only do this if the repo policy for the current branch allows intermediate commits during execution.

### Task 2: Implement the reduction and register it

**Files:**
- Create: `src/rules/knapsack_ilp.rs`
- Modify: `src/rules/mod.rs`
- Reference: `src/models/misc/knapsack.rs`
- Reference: `src/models/algebraic/ilp.rs`
- Reference: `src/rules/knapsack_qubo.rs`
- Reference: `src/rules/maximumclique_ilp.rs`

**Step 1: Write the minimal rule implementation**

Create `ReductionKnapsackToILP` with:
- `target: ILP<bool>`
- `num_items: usize` only if you need explicit truncation during extraction; otherwise keep extraction as `to_vec()`

Implement `ReductionResult` with:
- `type Source = Knapsack`
- `type Target = ILP<bool>`
- `target_problem()` returning the constructed ILP
- `extract_solution()` returning the item-selection bits unchanged

Implement:
```rust
#[reduction(overhead = {
    num_vars = "num_items",
    num_constraints = "1",
})]
impl ReduceTo<ILP<bool>> for Knapsack { ... }
```

Construct the target ILP as:
- `num_vars = self.num_items()`
- `constraints = vec![LinearConstraint::le((0..n).map(|i| (i, self.weights()[i] as f64)).collect(), self.capacity() as f64)]`
- `objective = self.values().iter().enumerate().map(|(i, &v)| (i, v as f64)).collect()`
- `sense = ObjectiveSense::Maximize`

**Step 2: Register the rule**

In `src/rules/mod.rs`:
- add `#[cfg(feature = "ilp-solver")] pub(crate) mod knapsack_ilp;`
- extend `canonical_rule_example_specs()` with `knapsack_ilp::canonical_rule_example_specs()`

Place both entries in the ILP-gated section beside the other `*_ilp` rules.

**Step 3: Run the targeted tests**

Run: `cargo test --features ilp-solver test_knapsack_to_ilp -- --nocapture`

Expected: PASS for the new rule tests.

**Step 4: Commit the minimal working rule**

Run:
```bash
git add src/rules/knapsack_ilp.rs src/rules/mod.rs src/unit_tests/rules/knapsack_ilp.rs
git commit -m "feat: add Knapsack to ILP reduction"
```

### Task 3: Add the canonical example and bibliography support

**Files:**
- Modify: `src/rules/knapsack_ilp.rs`
- Modify: `docs/paper/references.bib`
- Reference: `src/rules/maximumclique_ilp.rs`
- Reference: `src/rules/knapsack_qubo.rs`

**Step 1: Add the canonical example spec**

Inside `src/rules/knapsack_ilp.rs`, add:
- `#[cfg(feature = "example-db")] pub(crate) fn canonical_rule_example_specs() -> Vec<...>`
- a single `RuleExampleSpec` with id `knapsack_to_ilp`
- builder using `crate::example_db::specs::direct_ilp_example::<_, bool, _>(Knapsack::new(vec![1, 3, 4, 5], vec![1, 4, 5, 7], 7), |_, _| true)` or the exact issue example instance after confirming the optimum remains `(0, 1, 1, 0)`

The canonical example should match the issue’s 4-item tutorial instance unless a solver/export limitation forces a different but equivalent example.

**Step 2: Add the reference entry**

Add a BibTeX entry for Papadimitriou and Steiglitz (1982) in `docs/paper/references.bib` so the later paper theorem can cite it.

**Step 3: Run the example-db targeted test**

Run: `cargo test --features "ilp-solver example-db" test_knapsack_to_ilp_canonical_example_spec -- --nocapture`

Expected: PASS, with serialized source/target instances present.

**Step 4: Commit**

Run:
```bash
git add src/rules/knapsack_ilp.rs docs/paper/references.bib
git commit -m "test: add Knapsack to ILP example fixture"
```

### Task 4: Export metadata and verify the Rust side

**Files:**
- Modify: generated files as needed from export commands
- Reference: `docs/src/reductions/` (ignored; do not stage)

**Step 1: Regenerate exports**

Run:
```bash
cargo run --features "ilp-solver example-db" --example export_graph
cargo run --features "ilp-solver example-db" --example export_schemas
```

Inspect `git status --short` afterward and stage only tracked files that belong in the PR.

**Step 2: Run focused verification first**

Run:
```bash
cargo test --features "ilp-solver example-db" knapsack_ilp -- --nocapture
```

Expected: PASS.

**Step 3: Run repo verification**

Run:
```bash
make test
make clippy
```

Expected: PASS with `ilp-solver` support enabled by the project defaults used in CI. If `make test` is too broad during iteration, run the minimum equivalent command set before the final pass, then return here for full verification.

**Step 4: Commit verification-driven fixes**

Run:
```bash
git add -A
git commit -m "chore: verify Knapsack to ILP integration"
```

Only commit if exports or verification required tracked source changes.

## Batch 2: Paper Entry

### Task 5: Document the theorem in the paper

**Files:**
- Modify: `docs/paper/reductions.typ`
- Reference: `docs/paper/reductions.typ` around `#reduction-rule("MaximumClique", "ILP")`
- Reference: `docs/paper/reductions.typ` around `#reduction-rule("Knapsack", "QUBO")`

**Step 1: Load the generated canonical example**

Use the exported example database in Typst, following the `Knapsack -> QUBO` worked-example pattern:
- `#let ks_ilp = load-example("Knapsack", "ILP")`
- derive counts, selected items, total weight, and total value from `ks_ilp`

**Step 2: Add the theorem body**

Insert a new `#reduction-rule("Knapsack", "ILP", example: true, ...)` in the ILP formulations section. The theorem should:
- cite Papadimitriou and Steiglitz
- explain the binary item variables, single capacity inequality, and maximize-value objective
- mention the exact overhead: `n` variables and `1` constraint

**Step 3: Add the proof / construction block**

Include:
- `_Construction._` with the full ILP formulation
- `_Correctness._` both directions, arguing feasibility and objective preservation
- `_Solution extraction._` identity on the item indicators

**Step 4: Add the worked example**

Walk through the 4-item example from the exported fixture:
- source weights, values, capacity
- the single ILP constraint and objective
- the optimal binary solution and its extracted knapsack witness

State clearly that the fixture stores one canonical optimal witness even if multiple optima exist.

**Step 5: Build the paper**

Run: `make paper`

Expected: PASS with the new theorem and bibliography entry.

**Step 6: Commit the paper batch**

Run:
```bash
git add docs/paper/reductions.typ docs/paper/references.bib
git commit -m "docs: add Knapsack to ILP theorem"
```

## Final Review, PR Update, and Cleanup

### Task 6: Review, summarize deviations, and prepare the branch for push

**Files:**
- Modify: implementation files only if review finds issues
- Remove: `docs/plans/2026-03-17-knapsack-to-ilp.md` before the final push

**Step 1: Run review-implementation**

Run the repo-local review skill in the worktree after the Rust and paper batches are complete. Reuse the current diff instead of re-deriving the subject manually.

**Step 2: Fix review findings**

Address structural gaps, semantic issues, or important quality findings. Re-run the smallest relevant verification command after each fix, then rerun `review-implementation` if needed.

**Step 3: Create the implementation summary PR comment**

Summarize:
- new rule file and registration
- new tests and canonical example
- bibliography/paper additions
- any deviations from the issue example or plan

**Step 4: Remove the plan file**

Run:
```bash
git rm docs/plans/2026-03-17-knapsack-to-ilp.md
git commit -m "chore: remove plan file after implementation"
```

**Step 5: Final verification before push**

Run:
```bash
git status --short
test ! -e docs/plans/2026-03-17-knapsack-to-ilp.md
```

Ensure ignored generated exports under `docs/src/reductions/` are not staged.
