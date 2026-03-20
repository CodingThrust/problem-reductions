# GraphPartitioning to QUBO Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the `GraphPartitioning -> QUBO` reduction, its closed-loop tests, canonical example fixture, and paper entry for issue #119.

**Architecture:** Implement a direct penalty-method QUBO in a new `src/rules/graphpartitioning_qubo.rs` module. The reduction keeps one binary variable per source vertex, uses the source partition bit-vector as the extracted solution unchanged, and chooses `P = num_edges + 1` so any imbalanced assignment is dominated by every balanced assignment. The paper entry should reuse the canonical exported fixture for the 6-vertex issue example instead of hardcoding duplicated numbers.

**Tech Stack:** Rust, cargo, existing reduction macros/traits, Typst paper, GitHub pipeline scripts

---

## Batch 1: Rule + tests + exported example

### Task 1: Add the failing rule tests first

**Files:**
- Create: `src/unit_tests/rules/graphpartitioning_qubo.rs`
- Modify: `src/rules/mod.rs`
- Create: `src/rules/graphpartitioning_qubo.rs`

**Step 1: Write the failing tests**

Create `src/unit_tests/rules/graphpartitioning_qubo.rs` with these concrete checks:
- `test_graphpartitioning_to_qubo_closed_loop`: build the 6-vertex issue graph, reduce with `ReduceTo::<QUBO<f64>>::reduce_to`, solve the target with `BruteForce`, and verify every optimal QUBO solution extracts to a `GraphPartitioning` optimum with cut `3`.
- `test_graphpartitioning_to_qubo_matrix_matches_issue_example`: assert `num_vars == 6`, diagonal entries `[-48.0, -47.0, -46.0, -46.0, -47.0, -48.0]`, edge off-diagonals `18.0`, and non-edge off-diagonals `20.0`.
- `test_graphpartitioning_to_qubo_canonical_example_spec` behind `#[cfg(feature = "example-db")]`: verify the canonical rule example exports `GraphPartitioning` -> `QUBO` with `num_vars == 6` and at least one witness.

Wire the new rule module into `src/rules/mod.rs` and add only the minimal new `src/rules/graphpartitioning_qubo.rs` skeleton needed for the tests to compile against the module path.

**Step 2: Run the targeted test to verify RED**

Run:
```bash
cargo test --features "ilp-highs example-db" graphpartitioning_qubo -- --include-ignored
```

Expected:
- FAIL because the `ReduceTo<QUBO<f64>> for GraphPartitioning<SimpleGraph>` implementation and canonical example are still missing/incomplete.

**Step 3: Commit the red test scaffold**

```bash
git add src/unit_tests/rules/graphpartitioning_qubo.rs src/rules/mod.rs src/rules/graphpartitioning_qubo.rs
git commit -m "test: add failing GraphPartitioning to QUBO coverage"
```

### Task 2: Implement the reduction module

**Files:**
- Modify: `src/rules/graphpartitioning_qubo.rs`
- Modify: `src/rules/mod.rs`

**Step 1: Write the minimal implementation**

Implement `ReductionGraphPartitioningToQUBO` with:
- `target: QUBO<f64>`
- `ReductionResult::extract_solution()` returning `target_solution[..num_vertices].to_vec()` or the full vector if the target has exactly one bit per vertex
- `#[reduction(overhead = { num_vars = "num_vertices" })]`
- `ReduceTo<QUBO<f64>> for GraphPartitioning<SimpleGraph>`

Construct the upper-triangular Q matrix exactly from the issue:
- `P = num_edges as f64 + 1.0`
- diagonal `Q[i][i] = degree(i) as f64 + P * (1.0 - n as f64)`
- off-diagonal `Q[i][j] += 2P` for every `i < j`
- subtract `2.0` from `Q[u][v]` for each graph edge `(u, v)` after normalizing to upper-triangular coordinates

Add the canonical example spec in the same rule file under `#[cfg(feature = "example-db")]` using the issue’s 6-vertex graph and witness `source_config == target_config == vec![0, 0, 0, 1, 1, 1]`.

**Step 2: Run the targeted test to verify GREEN**

Run:
```bash
cargo test --features "ilp-highs example-db" graphpartitioning_qubo -- --include-ignored
```

Expected:
- PASS for the new rule tests.

**Step 3: Refactor only if needed**

Keep the implementation small. If repeated upper-triangular updates are awkward, extract one tiny local helper in the rule module and rerun the same targeted command.

**Step 4: Commit**

```bash
git add src/rules/graphpartitioning_qubo.rs src/rules/mod.rs src/unit_tests/rules/graphpartitioning_qubo.rs
git commit -m "feat: add GraphPartitioning to QUBO reduction"
```

### Task 3: Regenerate exported data and verify the canonical example

**Files:**
- Modify: `docs/src/data/examples.json`
- Modify: generated graph/schema exports if changed by the commands below

**Step 1: Regenerate the fixture/export artifacts**

Run:
```bash
cargo run --features "example-db" --example export_examples
cargo run --example export_graph
cargo run --example export_schemas
```

Expected:
- The canonical `GraphPartitioning -> QUBO` example appears in the checked-in example export.
- Reduction graph/schema exports include the new primitive rule.

**Step 2: Verify the example-driven data is stable**

Run:
```bash
git status --short
```

Expected:
- Only the new rule/test/module changes plus expected generated exports are modified.

**Step 3: Commit**

```bash
git add docs/src/data/examples.json
git add .
git commit -m "chore: export GraphPartitioning to QUBO fixtures"
```

## Batch 2: Paper entry

### Task 4: Document the reduction in the paper

**Files:**
- Modify: `docs/paper/reductions.typ`

**Step 1: Write the failing documentation build**

Add a `#let gp_qubo = load-example("GraphPartitioning", "QUBO")` block and a `#reduction-rule("GraphPartitioning", "QUBO", ...)` entry in the penalty-method QUBO section near the other QUBO reductions. Use the exported fixture data for the 6-vertex example instead of duplicating the matrix constants by hand.

The theorem/proof must cover:
- binary variable mapping `x_i in {0,1}`
- cut-counting term `sum_(uv in E) (x_u + x_v - 2 x_u x_v)`
- balance penalty `P (sum_i x_i - n/2)^2`
- explicit QUBO coefficients `Q_(ii) = deg(i) + P(1 - n)` and `Q_(ij) = 2P - 2` on edges / `2P` otherwise
- correctness argument that `P > m` forces balance and the remaining objective equals the cut size
- solution extraction as the identity map from the QUBO bit-vector back to the partition encoding

**Step 2: Run the documentation build**

Run:
```bash
make paper
```

Expected:
- PASS with the new reduction-rule entry and no completeness warnings for `GraphPartitioning -> QUBO`.

**Step 3: Commit**

```bash
git add docs/paper/reductions.typ
git commit -m "docs: add GraphPartitioning to QUBO paper entry"
```

## Batch 3: Full verification and cleanup

### Task 5: Run fresh verification before claiming completion

**Files:**
- Verify only; no planned edits

**Step 1: Re-run focused tests**

Run:
```bash
cargo test --features "ilp-highs example-db" graphpartitioning_qubo -- --include-ignored
```

Expected:
- PASS.

**Step 2: Run repository verification**

Run:
```bash
make fmt
make test
make clippy
```

Expected:
- All commands pass.

**Step 3: Inspect the working tree before final push**

Run:
```bash
git status --short
```

Expected:
- No unexpected tracked changes remain.

**Step 4: Commit any final formatting/export deltas**

```bash
git add -A
git commit -m "chore: finalize GraphPartitioning to QUBO implementation"
```

**Step 5: Remove the plan file after implementation**

```bash
git rm docs/plans/2026-03-20-graphpartitioning-to-qubo.md
git commit -m "chore: remove plan file after implementation"
```
