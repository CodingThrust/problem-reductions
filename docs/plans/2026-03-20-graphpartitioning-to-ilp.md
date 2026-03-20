# GraphPartitioning to ILP Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add a `GraphPartitioning -> ILP` reduction with closed-loop tests, canonical example data, and paper documentation for issue #118.

**Architecture:** Implement a binary ILP reduction with one partition variable `x_v` per vertex and one crossing-indicator variable `y_e` per edge. Preserve GraphPartitioning's "odd number of vertices is invalid" semantics by encoding the balance equation as `sum x_v = n / 2` over reals, so odd `n` becomes infeasible automatically because the RHS is fractional.

**Tech Stack:** Rust workspace, reduction registry macros, `good_lp`-backed ILP model, example-db exports, Typst paper.

---

## Batch 1: Rule implementation and tests

Aligned with `add-rule` Steps 1-4 and Step 6's code-side exports.

### Task 1: Write failing rule tests first

**Files:**
- Create: `src/unit_tests/rules/graphpartitioning_ilp.rs`
- Read for patterns: `src/unit_tests/rules/minimummultiwaycut_ilp.rs`
- Read for patterns: `src/unit_tests/rules/maximumclique_ilp.rs`

**Step 1: Write the failing tests**

Add focused tests for:
- ILP structure on the canonical 6-vertex example: `num_vars = n + m = 15`, `num_constraints = 2m + 1 = 19`, objective sense is minimize, objective coefficients are attached only to edge variables.
- Constraint shape on a tiny graph: balance equality plus two `>=` linking constraints per edge.
- Closed-loop solving on the canonical example: brute-force `GraphPartitioning` optimum is 3 and the ILP solution extracts to the same optimum.
- Odd-vertex behavior: a 3-vertex graph reduces to an infeasible ILP, so `ILPSolver::solve` returns `None`.
- Solution extraction: a hand-written ILP witness maps back to the expected partition config.
- `solve_reduced` path works end-to-end.

**Step 2: Run the new test target and verify it fails**

Run:
```bash
cargo test --features "ilp-highs example-db" graphpartitioning_ilp -- --include-ignored
```

Expected: compile failure because the rule module does not exist yet.

### Task 2: Implement the reduction rule

**Files:**
- Create: `src/rules/graphpartitioning_ilp.rs`
- Modify: `src/rules/mod.rs`
- Read for patterns: `src/models/graph/graph_partitioning.rs`
- Read for patterns: `src/models/algebraic/ilp.rs`

**Step 1: Write the minimal implementation**

Implement:
- `ReductionGraphPartitioningToILP { target: ILP<bool>, num_vertices: usize }`
- `ReductionResult` impl returning the target ILP and extracting the first `n` variables as the source partition assignment.
- `ReduceTo<ILP<bool>> for GraphPartitioning<SimpleGraph>` with:
  - variable layout: vertex variables first, then edge variables in source edge order
  - balance equality `sum_v x_v = n as f64 / 2.0`
  - two linking constraints per edge:
    - `y_e - x_u + x_v >= 0`
    - `y_e + x_u - x_v >= 0`
  - objective `min sum_e y_e`
  - overhead:
    - `num_vars = "num_vertices + num_edges"`
    - `num_constraints = "2 * num_edges + 1"`
- `#[cfg(feature = "example-db")] canonical_rule_example_specs()` with the issue's 6-vertex graph and a canonical witness:
  - source config: `[0, 0, 0, 1, 1, 1]`
  - target config: `[0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0]`
- Feature-gated registration in `src/rules/mod.rs` and inclusion in `canonical_rule_example_specs()`.

**Step 2: Run the focused tests and verify they pass**

Run:
```bash
cargo test --features "ilp-highs example-db" graphpartitioning_ilp -- --include-ignored
```

Expected: all `graphpartitioning_ilp` tests pass.

**Step 3: Refactor only if needed**

Keep only cleanup that preserves behavior:
- small helper for balance constraint construction
- comment documenting the variable ordering used by `extract_solution`

### Task 3: Verify registry and example-db wiring

**Files:**
- Modify: `src/rules/mod.rs`
- Read for patterns: `src/rules/minimummultiwaycut_ilp.rs`

**Step 1: Run export commands that depend on rule registration**

Run:
```bash
cargo run --example export_graph
cargo run --example export_schemas
cargo run --features "example-db" --example export_examples
```

Expected:
- `export_graph` includes a `GraphPartitioning -> ILP` edge.
- `export_examples` completes and writes `src/example_db/fixtures/examples.json`.

**Step 2: If export output reveals missing wiring, fix only the missing registration and rerun the failing command**

### Task 4: Run broader code verification for the implementation batch

**Files:**
- No new files unless a verification failure requires a minimal follow-up fix

**Step 1: Run the relevant batch verification**

Run:
```bash
cargo test --features "ilp-highs example-db" graphpartitioning_ilp graph_partitioning -- --include-ignored
cargo clippy --all-targets --features ilp-highs -- -D warnings
```

Expected: tests pass and clippy reports no warnings.

**Step 2: Commit the implementation batch**

Run:
```bash
git add src/rules/graphpartitioning_ilp.rs src/rules/mod.rs src/unit_tests/rules/graphpartitioning_ilp.rs
git commit -m "Add GraphPartitioning to ILP reduction"
```

## Batch 2: Paper entry and fixture-backed documentation

Aligned with `add-rule` Step 5.

### Task 5: Add the paper reduction entry

**Files:**
- Modify: `docs/paper/reductions.typ`
- Read for patterns: nearby `GraphPartitioning` problem definition
- Read for patterns: `#reduction-rule("MinimumMultiwayCut", "ILP")`
- Read for patterns: `#reduction-rule("TravelingSalesman", "ILP", example: true, ...)`

**Step 1: Write the fixture-backed reduction-rule entry**

Add a `GraphPartitioning -> ILP` theorem near the other ILP reductions that includes:
- theorem body summarizing the vertex/edge-indicator formulation and the `n + m` / `2m + 1` size
- proof body sections for construction, correctness, and solution extraction
- an `example: true` walkthrough backed by `load-rule-example("GraphPartitioning", "ILP")`
- explicit note that odd `n` produces an infeasible balance equality because `n / 2` is fractional

The example should read the exported witness data rather than hardcoding a second copy of the solution.

**Step 2: Build the paper and verify it passes**

Run:
```bash
make paper
```

Expected: Typst builds successfully and the new reduction entry resolves its example data.

### Task 6: Final verification for the branch state

**Files:**
- Any generated tracked outputs from the export and paper build steps

**Step 1: Run final verification commands**

Run:
```bash
cargo test --features "ilp-highs example-db" graphpartitioning_ilp graph_partitioning -- --include-ignored
cargo clippy --all-targets --features ilp-highs -- -D warnings
make paper
git status --short
```

Expected:
- tests pass
- clippy passes
- paper builds
- `git status --short` shows only intended tracked changes plus ignored/generated files under ignored export paths

**Step 2: Commit the documentation/export batch**

Run:
```bash
git add docs/paper/reductions.typ src/example_db/fixtures/examples.json docs/paper/reductions.pdf docs/paper/graph.json docs/src/schema/problem_schemas.json
git commit -m "Document GraphPartitioning to ILP reduction"
```

If generated tracked file paths differ, stage the actual tracked outputs reported by `git status --short` instead of forcing this exact list.

### Task 7: Remove the plan file after implementation

**Files:**
- Delete: `docs/plans/2026-03-20-graphpartitioning-to-ilp.md`

**Step 1: Remove the temporary plan artifact**

Run:
```bash
git rm docs/plans/2026-03-20-graphpartitioning-to-ilp.md
git commit -m "chore: remove plan file after implementation"
```

## Notes for the executor

- Stay inside the issue worktree branch `issue-118`.
- Do not touch the user's main checkout.
- Do not "fix" unrelated changes in the parent repository.
- Keep the implementation scoped to this one rule; do not add model changes.
