# GraphPartitioning to MaxCut Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development to implement this plan task-by-task in this session.

**Goal:** Add the `GraphPartitioning -> MaxCut` reduction from issue #120, including closed-loop tests, a canonical rule example, exported fixtures, and a paper entry.

**Architecture:** Reuse the same vertex set, build a weighted complete graph on every unordered vertex pair, and set edge weights to `P - 1` for original edges and `P` for non-edges with `P = num_edges + 1`. This makes every optimal MaxCut solution balanced first, then equivalent to a minimum-bisection solution on the source graph; solution extraction is the identity mapping on the partition bit-vector.

**Tech Stack:** Rust crate code under `src/rules/` and `src/unit_tests/`, example-db exports, Typst paper docs, cargo/make verification commands.

---

## Batch 1: Rule implementation, tests, and exports

### Task 1: Add failing rule tests first

**Files:**
- Create: `src/unit_tests/rules/graphpartitioning_maxcut.rs`
- Modify: `src/rules/mod.rs`
- Reference: `src/rules/minimumvertexcover_maximumindependentset.rs`
- Reference: `src/unit_tests/rules/minimumvertexcover_maximumindependentset.rs`
- Reference: `src/models/graph/graph_partitioning.rs`
- Reference: `src/models/graph/max_cut.rs`

**Step 1: Write the failing test**

Add tests that exercise the exact issue construction on the 6-vertex example from the issue:
- `test_graphpartitioning_to_maxcut_closed_loop`
- `test_graphpartitioning_to_maxcut_target_structure`
- `test_graphpartitioning_to_maxcut_extract_solution_identity`

The structure test should assert:
- target `num_vertices == source.num_vertices()`
- target `num_edges == num_vertices * (num_vertices - 1) / 2`
- weights are `9` on source edges and `10` on non-edges for the issue example (`m = 9`, `P = 10`)

The closed-loop test should use `assert_optimization_round_trip_from_optimization_target`.

**Step 2: Run test to verify it fails**

Run: `cargo test graphpartitioning_to_maxcut --lib`

Expected: compile or link failure because the new rule module and `ReduceTo<MaxCut<SimpleGraph, i32>> for GraphPartitioning<SimpleGraph>` do not exist yet.

**Step 3: Commit**

```bash
git add src/unit_tests/rules/graphpartitioning_maxcut.rs src/rules/mod.rs
git commit -m "test: add GraphPartitioning to MaxCut reduction tests"
```

### Task 2: Implement the reduction and register it

**Files:**
- Create: `src/rules/graphpartitioning_maxcut.rs`
- Modify: `src/rules/mod.rs`
- Test: `src/unit_tests/rules/graphpartitioning_maxcut.rs`

**Step 1: Write minimal implementation**

Implement:
- `ReductionGPToMaxCut` storing the target `MaxCut<SimpleGraph, i32>`
- `ReductionResult` with identity `extract_solution`
- `#[reduction(overhead = { num_vertices = "num_vertices", num_edges = "num_vertices * (num_vertices - 1) / 2" })]`
- `ReduceTo<MaxCut<SimpleGraph, i32>> for GraphPartitioning<SimpleGraph>`

Construction details:
- `P = self.num_edges() as i32 + 1`
- Enumerate all pairs `(u, v)` with `u < v`
- Weight is `P - 1` if `(u, v)` is an original source edge, else `P`
- Build `MaxCut::new(SimpleGraph::new(n, complete_edges), weights)`

Register the module in `src/rules/mod.rs` and extend `canonical_rule_example_specs()`.

**Step 2: Run tests to verify they pass**

Run: `cargo test graphpartitioning_to_maxcut --lib`

Expected: all new reduction tests pass.

**Step 3: Refactor only if needed**

Keep the rule code minimal. If edge-membership checks are awkward, extract a tiny local helper inside the rule file rather than changing model APIs.

**Step 4: Commit**

```bash
git add src/rules/graphpartitioning_maxcut.rs src/rules/mod.rs src/unit_tests/rules/graphpartitioning_maxcut.rs
git commit -m "feat: add GraphPartitioning to MaxCut reduction"
```

### Task 3: Add canonical rule example and export support

**Files:**
- Modify: `src/rules/graphpartitioning_maxcut.rs`
- Modify: `src/rules/mod.rs`
- Reference: `src/example_db/specs.rs`

**Step 1: Add canonical example spec**

Inside the new rule file, add `canonical_rule_example_specs()` under `#[cfg(feature = "example-db")]` using the issue’s 6-vertex graph and a stored witness:
- source config: `[0, 0, 0, 1, 1, 1]`
- target config: `[0, 0, 0, 1, 1, 1]`

Use `rule_example_with_witness::<_, MaxCut<SimpleGraph, i32>>`.

**Step 2: Run export/fixture commands**

Run in order:
- `cargo run --example export_graph`
- `cargo run --example export_schemas`
- `make regenerate-fixtures`

Expected: updated reduction graph, schemas, and example fixtures include the new rule.

**Step 3: Run focused verification**

Run:
- `cargo test graphpartitioning_to_maxcut --features example-db --lib`

Expected: example-backed rule code still passes after export wiring.

**Step 4: Commit**

```bash
git add src/rules/graphpartitioning_maxcut.rs src/rules/mod.rs docs/src/reduction_graph.json docs/src/problem_schemas.json src/example_db/fixtures/examples.json
git commit -m "test: add GraphPartitioning to MaxCut fixtures"
```

## Batch 2: Paper entry after exports exist

### Task 4: Document the reduction in the paper

**Files:**
- Modify: `docs/paper/reductions.typ`
- Reference: `docs/paper/reductions.typ` section `reduction-rule("MaxCut", "SpinGlass", ...)`
- Reference: issue #120 body and comments cached in `/tmp/issue-120-context.json`

**Step 1: Write the paper entry**

Add a `GraphPartitioning -> MaxCut` `reduction-rule` block with:
- statement that the weighted-complete-graph construction is folklore in combinatorial optimization
- citation to `@garey1976` for the surrounding hardness context
- `_Construction._` with explicit piecewise weight formula
- `_Correctness._` showing `P = m + 1` forces balance and then complements the source objective
- `_Solution extraction._` as identity on vertex assignments
- worked example sourced from `load-example("GraphPartitioning", "MaxCut")`

The example should enumerate the 15 target edges by weight class and verify the canonical witness cut value `87`.

**Step 2: Verify the paper build**

Run: `make paper`

Expected: Typst compiles successfully with the new reduction example.

**Step 3: Run final repo verification**

Run:
- `make test`
- `make clippy`

Expected: both commands pass after the new rule and paper entry land.

**Step 4: Commit**

```bash
git add docs/paper/reductions.typ
git commit -m "docs: add GraphPartitioning to MaxCut reduction"
```

## Notes for Execution

- Use TDD strictly: do not write production code for the rule until the new tests are present and have been run red first.
- Do not change either model API unless the tests demonstrate an unavoidable gap.
- The issue comments already resolved the ambiguity around `P`: use `P = num_edges + 1`.
- The cited paper in the issue does not describe the exact construction; present the construction as folklore and avoid overstating the citation.
