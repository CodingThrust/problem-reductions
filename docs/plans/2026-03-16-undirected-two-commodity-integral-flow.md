# Undirected Two-Commodity Integral Flow Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the `UndirectedTwoCommodityIntegralFlow` satisfaction model, register it across the crate/CLI/example-db/paper, and verify the issue #296 YES/NO instances end to end.

**Architecture:** Represent each undirected edge `{u, v}` by four integer variables in a fixed edge order from `graph.edges()`: `f1(u,v)`, `f1(v,u)`, `f2(u,v)`, `f2(v,u)`. `dims()` should expose the exact finite search space by repeating `(capacity + 1)` four times per edge, while `evaluate()` directly enforces antisymmetry, shared edge capacity, per-commodity flow conservation on `V \ {s_1, s_2, t_1, t_2}`, and the sink-demand constraints from the issue body. Use the issue's small even-capacity YES instance as the canonical example because it is brute-force tractable; keep the complexity string `5^num_edges` to match the current issue text/comments.

**Tech Stack:** Rust workspace, `SimpleGraph`, `Problem`/`SatisfactionProblem`, registry `inventory` + `declare_variants!`, `pred` CLI, example-db fixtures, Typst paper, `make test`, `make clippy`, `make regenerate-fixtures`, `make paper`.

---

## Issue Notes To Carry Into Implementation

- Issue: `#296 [Model] UndirectedTwoCommodityIntegralFlow`
- Reference: Even, Itai, and Shamir, "On the Complexity of Timetable and Multicommodity Flow Problems," *SIAM J. Comput.* 5(4), 1976.
- Maintainer comments already settled two implementation details:
  - keep `graph: SimpleGraph` in the schema instead of raw edge lists
  - keep the declared complexity string `5^num_edges`
- The directed companion issue `#295` uses the same conservation interpretation (`V - {s_1, s_2, t_1, t_2}`), so do not silently "correct" that to per-commodity terminals in this PR.
- There is a planned incoming rule (`DirectedTwoCommodityIntegralFlow -> UndirectedTwoCommodityIntegralFlow`), so the model is not orphaned even though the rule is not on `main` yet.

## Batching

- **Batch 1 (add-model Steps 1-5.5):** model implementation, registration, CLI support, canonical example, tests, trait consistency, fixture regeneration.
- **Batch 2 (add-model Step 6):** Typst paper entry + bibliography + paper build.
- Keep Batch 2 separate. It depends on Batch 1 because the paper loads checked-in example fixtures and exported schemas.

## Concrete Design Decisions

- Category: `src/models/graph/`
- Canonical problem name: `UndirectedTwoCommodityIntegralFlow`
- Problem type: satisfaction (`Metric = bool`)
- Struct fields:
  - `graph: SimpleGraph`
  - `capacities: Vec<u64>`
  - `source_1: usize`
  - `sink_1: usize`
  - `source_2: usize`
  - `sink_2: usize`
  - `requirement_1: u64`
  - `requirement_2: u64`
- Size getters:
  - `num_vertices()`
  - `num_edges()`
- Variable layout per edge `{u,v}` in `graph.edges()` order:
  - offset `4 * edge_index + 0`: commodity 1, `u -> v`
  - offset `4 * edge_index + 1`: commodity 1, `v -> u`
  - offset `4 * edge_index + 2`: commodity 2, `u -> v`
  - offset `4 * edge_index + 3`: commodity 2, `v -> u`
- Canonical paper/example-db instance:
  - use issue Instance 3: graph `0-2,1-2,2-3`, capacities `[1, 1, 2]`, `source_1=0`, `sink_1=3`, `source_2=1`, `sink_2=3`, requirements `(1,1)`
  - this keeps brute-force enumeration feasible while still illustrating the even-capacity phenomenon mentioned in the issue

### Task 1: Add the Red Tests for the New Model Surface

**Files:**
- Create: `src/unit_tests/models/graph/undirected_two_commodity_integral_flow.rs`
- Modify: `src/unit_tests/trait_consistency.rs`
- Modify: `problemreductions-cli/tests/cli_tests.rs`

**Step 1: Write the failing tests**

Add model tests that cover:

- `test_undirected_two_commodity_integral_flow_creation`
  - construct the 3-edge even-capacity instance
  - assert getters and `dims() == vec![2,2,2,2, 2,2,2,2, 3,3,3,3]`
- `test_undirected_two_commodity_integral_flow_evaluation_yes`
  - encode the issue Instance 3 satisfying flow:
    - edge `(0,2)`: commodity 1 forward = 1
    - edge `(1,2)`: commodity 2 forward = 1
    - edge `(2,3)`: commodity 1 forward = 1, commodity 2 forward = 1
  - assert `evaluate(...) == true`
- `test_undirected_two_commodity_integral_flow_evaluation_no_shared_bottleneck`
  - use issue Instance 2 with capacities `[1,1,1]`
  - assert the obvious "both commodities use the bottleneck edge" configuration is rejected
  - assert `BruteForce::new().find_satisfying(&problem).is_none()`
- `test_undirected_two_commodity_integral_flow_serialization`
- `test_undirected_two_commodity_integral_flow_paper_example`
  - reuse the canonical example instance
  - verify the issue's displayed satisfying configuration is accepted
  - compute all satisfying solutions and assert the count matches the paper's stated count

Add a `trait_consistency` entry with a tiny valid instance.

Add CLI smoke tests in `problemreductions-cli/tests/cli_tests.rs`:

- `test_list_includes_undirected_two_commodity_integral_flow`
- `test_show_undirected_two_commodity_integral_flow`
- `test_create_undirected_two_commodity_integral_flow`

For the create test, call:

```bash
pred create UndirectedTwoCommodityIntegralFlow \
  --graph 0-2,1-2,2-3 \
  --capacities 1,1,2 \
  --source-1 0 --sink-1 3 \
  --source-2 1 --sink-2 3 \
  --requirement-1 1 --requirement-2 1
```

and assert the JSON contains the new type plus the expected field values.

**Step 2: Run the targeted tests to verify RED**

Run:

```bash
cargo test undirected_two_commodity_integral_flow --lib
cargo test -p problemreductions-cli undirected_two_commodity_integral_flow
```

Expected:
- compile failure or failing assertions because the model is not implemented/registered yet
- no unrelated test failures

**Step 3: Do not fix anything yet beyond test hygiene**

Only adjust typos or obviously broken test code. Do not add production code in this task beyond what is needed to make the test targets compile once the model file exists in Task 2.

**Step 4: Checkpoint**

Do not create a standalone commit here; `issue-to-pr --execute` owns the final implementation commit flow.

### Task 2: Implement the Core Model and Make the Model Tests Green

**Files:**
- Create: `src/models/graph/undirected_two_commodity_integral_flow.rs`
- Modify: `src/models/graph/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`

**Step 1: Implement the minimal model to satisfy the tests**

Add `src/models/graph/undirected_two_commodity_integral_flow.rs` with:

- `inventory::submit!` metadata
- `#[derive(Debug, Clone, Serialize, Deserialize)]`
- constructor validation:
  - `capacities.len() == graph.num_edges()`
  - all terminal indices are `< graph.num_vertices()`
  - each capacity fits into `usize` for `dims()`
- getters for graph/capacities/terminals/requirements
- `num_vertices()` / `num_edges()`
- helper methods:
  - `edge_variables(config, edge_index)` or similar
  - `net_flow_for_commodity(config, commodity, vertex)`
  - `is_valid_solution(config)`
- `Problem` impl:
  - `NAME = "UndirectedTwoCommodityIntegralFlow"`
  - `Metric = bool`
  - `variant() = crate::variant_params![]`
  - `dims()` repeats `(capacity + 1)` four times per edge
  - `evaluate()` returns `false` unless all constraints are satisfied
- `SatisfactionProblem` impl
- `declare_variants!` entry:

```rust
crate::declare_variants! {
    default sat UndirectedTwoCommodityIntegralFlow => "5^num_edges",
}
```

Link the new unit test file at the bottom with:

```rust
#[cfg(test)]
#[path = "../../unit_tests/models/graph/undirected_two_commodity_integral_flow.rs"]
mod tests;
```

Register the model in:

- `src/models/graph/mod.rs`
- `src/models/mod.rs`
- `src/lib.rs` prelude / re-exports

Also extend `src/models/graph/mod.rs::canonical_model_example_specs()` once the file exposes its example builder in Task 3.

**Step 2: Run the targeted model tests**

Run:

```bash
cargo test undirected_two_commodity_integral_flow --lib
```

Expected:
- the new model tests and the new `trait_consistency` entry pass
- failures, if any, should now be about missing CLI/example-db integration rather than core evaluation logic

**Step 3: Refactor only after green**

If needed, extract small helpers for:

- edge-variable indexing
- commodity balance computation
- terminal-set membership

Keep the representation simple and local to the model file.

**Step 4: Checkpoint**

No standalone commit; keep moving to registry/example integration.

### Task 3: Register the Canonical Example, CLI Create Support, and Remaining Batch-1 Wiring

**Files:**
- Modify: `src/models/graph/undirected_two_commodity_integral_flow.rs`
- Modify: `src/models/graph/mod.rs`
- Modify: `problemreductions-cli/src/cli.rs`
- Modify: `problemreductions-cli/src/commands/create.rs`
- Modify: `problemreductions-cli/src/problem_name.rs` (only if a lowercase/manual alias hook is still needed after registry registration)

**Step 1: Add the canonical model example**

Inside `src/models/graph/undirected_two_commodity_integral_flow.rs`, add:

- `#[cfg(feature = "example-db")]`
- `canonical_model_example_specs()`
- a single example spec using issue Instance 3
- `crate::example_db::specs::satisfaction_example(...)`

Register the example by extending `src/models/graph/mod.rs::canonical_model_example_specs()`.

**Step 2: Add CLI flags and create support**

Extend `CreateArgs` in `problemreductions-cli/src/cli.rs` with:

- `capacities: Option<String>`
- `source_1: Option<usize>`
- `sink_1: Option<usize>`
- `source_2: Option<usize>`
- `sink_2: Option<usize>`
- `requirement_1: Option<u64>`
- `requirement_2: Option<u64>`

Update:

- `all_data_flags_empty()`
- the `Flags by problem type` help block
- the `pred create <PROBLEM>` examples/help text

Add `create.rs` parsing helpers for capacities and the four terminal/requirement fields, then wire a new match arm for `"UndirectedTwoCommodityIntegralFlow"`.

Use this exact usage string in error paths:

```bash
Usage: pred create UndirectedTwoCommodityIntegralFlow --graph 0-2,1-2,2-3 --capacities 1,1,2 --source-1 0 --sink-1 3 --source-2 1 --sink-2 3 --requirement-1 1 --requirement-2 1
```

**Step 3: Run the targeted tests to verify GREEN**

Run:

```bash
cargo test undirected_two_commodity_integral_flow --lib
cargo test -p problemreductions-cli undirected_two_commodity_integral_flow
```

Expected:
- model tests pass
- CLI smoke tests pass
- `pred list` / `pred show` now include the new problem

**Step 4: Regenerate fixtures required by Batch 2**

Run:

```bash
make regenerate-fixtures
```

Expected:
- `src/example_db/fixtures/examples.json` updates to include the new canonical model example

If fixture generation surfaces a mismatch in the canonical example count, fix the example or paper wording before moving on.

**Step 5: Checkpoint**

Still no standalone commit. Batch 1 is complete once the fixture regeneration is clean.

### Task 4: Batch 2 — Write the Paper Entry and Bibliography

**Files:**
- Modify: `docs/paper/reductions.typ`
- Modify: `docs/paper/references.bib`

**Step 1: Add the bibliography entry**

Add an `@article` entry for the Even-Itai-Shamir 1976 paper if it is not already present. Reuse the existing `@garey1979` citation for the Garey & Johnson catalog pointer instead of duplicating the textbook.

**Step 2: Add the display-name and `problem-def` block**

Update `docs/paper/reductions.typ`:

- add `"UndirectedTwoCommodityIntegralFlow": [Undirected Two-Commodity Integral Flow],` to `display-name`
- add a `problem-def("UndirectedTwoCommodityIntegralFlow")` section that:
  - states the formal decision problem with undirected edges, capacities, and two commodities
  - explains the four-variables-per-edge encoding used in the code
  - mentions the parity dichotomy from the issue/reference
  - uses the canonical issue Instance 3 example and names one satisfying configuration explicitly
  - states the exact satisfying-solution count found in Task 3 so the paper and test agree

Prefer a compact explanatory figure/table over a large graph drawing; this instance is small enough that a textual edge/capacity table is acceptable if a full CeTZ graph adds little value.

**Step 3: Build the paper**

Run:

```bash
make paper
```

Expected:
- `docs/paper/reductions.typ` compiles cleanly
- exported schema files update if the new model changed them

**Step 4: Fix any paper/example drift immediately**

If `make paper` or the paper-example test disagrees about the satisfying count or displayed configuration, fix the code/example/paper now before moving to final verification.

### Task 5: Final Verification for issue-to-pr Handoff

**Files:**
- Review all files changed above

**Step 1: Run the repo checks required before claiming success**

Run:

```bash
make test
make clippy
```

If `make test` is too broad to diagnose quickly, first run the targeted tests above, then rerun the full command.

**Step 2: Run the repo-local completeness review**

After Batch 2 is green, invoke the repo-local skill:

```text
/review-implementation model UndirectedTwoCommodityIntegralFlow
```

Auto-fix any concrete findings before handing control back to `issue-to-pr`.

**Step 3: Capture the implementation summary for the PR comment**

Prepare a short summary covering:

- new model file + configuration encoding
- CLI flags/support added
- canonical example/fixture updates
- paper entry and bibliography additions
- any deliberate deviations from the issue (expected: none, unless the paper example uses Instance 3 instead of Instance 1 for tractability)

**Step 4: Leave the tree ready for issue-to-pr**

Do not remove the plan file manually here; `issue-to-pr` handles:

- the implementation commit
- review-fix commit(s)
- plan-file removal commit
- push and PR comment
