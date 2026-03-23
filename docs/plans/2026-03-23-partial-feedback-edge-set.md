# PartialFeedbackEdgeSet Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add `PartialFeedbackEdgeSet` as a new graph satisfaction model for issue `#438`, with brute-force solving, `pred create` support, a canonical example, and a paper entry.

**Architecture:** Implement `PartialFeedbackEdgeSet<G>` as an undirected graph satisfaction problem with one binary variable per edge. A configuration selects edges to remove; it is satisfying iff the remaining graph contains no cycle of length at most `L` and the number of removed edges is at most `K`. Follow the repo’s current graph-model pattern: generic over `G: Graph`, register the default concrete variant as `SimpleGraph`, keep the corrected issue example as the source of truth, and do the paper step in a separate batch after code/tests are stable.

**Tech Stack:** Rust, serde, inventory registry, `declare_variants!`, `pred` CLI, Typst paper, example-db.

---

## Batch 1: add-model Steps 1-5.5

### Task 1: Add failing model tests for the corrected issue example

**Files:**
- Create: `src/unit_tests/models/graph/partial_feedback_edge_set.rs`
- Test: `cargo test partial_feedback_edge_set --lib`

**Step 1: Write the failing test**

Add tests that pin the issue’s corrected behavior:
- constructor/accessors/dimensions for `graph`, `budget`, `max_cycle_length`, `num_vertices`, `num_edges`
- `evaluate()` returns `true` on the corrected satisfying set `{{0,2}, {2,3}, {3,4}}`
- `evaluate()` returns `false` for an under-budget or cycle-missing configuration
- brute-force finds a satisfying solution for the YES instance and no solution when `budget = 2`
- serde round-trip for the model

**Step 2: Run test to verify it fails**

Run: `cargo test partial_feedback_edge_set --lib`
Expected: FAIL because the model/module do not exist yet.

**Step 3: Write minimal implementation hooks**

Create the module declaration and test link stubs needed so the test compiles far enough to drive implementation.

**Step 4: Run test to verify the failure is now about missing logic**

Run: `cargo test partial_feedback_edge_set --lib`
Expected: FAIL on unimplemented behavior rather than missing module wiring.

**Step 5: Commit**

```bash
git add src/unit_tests/models/graph/partial_feedback_edge_set.rs src/models/graph/mod.rs src/models/mod.rs src/lib.rs
git commit -m "test: add PartialFeedbackEdgeSet model coverage"
```

### Task 2: Implement the core model and registry wiring

**Files:**
- Create: `src/models/graph/partial_feedback_edge_set.rs`
- Modify: `src/models/graph/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`
- Test: `cargo test partial_feedback_edge_set --lib`

**Step 1: Write the failing test**

Extend the new test file with any missing assertions for:
- invalid config length
- invalid non-binary edge selections
- `max_cycle_length >= 3` / `budget <= num_edges` constructor guards
- the exact brute-force configuration-space size `2^num_edges`

**Step 2: Run test to verify it fails**

Run: `cargo test partial_feedback_edge_set --lib`
Expected: FAIL on evaluation/registration gaps.

**Step 3: Write minimal implementation**

Implement `PartialFeedbackEdgeSet<G>` with:
- `inventory::submit!` schema entry and `display_name`
- fields `graph`, `budget`, `max_cycle_length`
- accessors plus `num_vertices()` and `num_edges()`
- `Problem` + `SatisfactionProblem` impls with `dims() = vec![2; num_edges]`
- `declare_variants! { default sat PartialFeedbackEdgeSet<SimpleGraph> => "2^num_edges" }`
- a helper that checks whether the remaining graph has any simple cycle of length `<= max_cycle_length`
- `canonical_model_example_specs()` using the corrected issue example
- module/re-export/prelude registration

Use the issue comments as constraints:
- do not claim an optimization direction
- keep the example cleaned up and consistent with the `/fix-issue` changelog
- keep the implementation on undirected graphs; do not bundle rule work or ILP work here

**Step 4: Run test to verify it passes**

Run: `cargo test partial_feedback_edge_set --lib`
Expected: PASS for the new model tests.

**Step 5: Commit**

```bash
git add src/models/graph/partial_feedback_edge_set.rs src/models/graph/mod.rs src/models/mod.rs src/lib.rs src/unit_tests/models/graph/partial_feedback_edge_set.rs
git commit -m "feat: add PartialFeedbackEdgeSet model"
```

### Task 3: Add CLI create support for the new model

**Files:**
- Modify: `problemreductions-cli/src/cli.rs`
- Modify: `problemreductions-cli/src/commands/create.rs`
- Test: `cargo test partial_feedback_edge_set --package problemreductions-cli`

**Step 1: Write the failing test**

Add CLI tests that cover:
- parsing a new `--max-cycle-length` flag
- `pred create PartialFeedbackEdgeSet --graph ... --budget 3 --max-cycle-length 4`
- error messaging when `--budget` or `--max-cycle-length` is missing
- JSON output fields `graph`, `budget`, `max_cycle_length`

**Step 2: Run test to verify it fails**

Run: `cargo test partial_feedback_edge_set --package problemreductions-cli`
Expected: FAIL because the CLI does not recognize the new problem/flag yet.

**Step 3: Write minimal implementation**

Update CLI plumbing to:
- add `--max-cycle-length` to `CreateArgs`
- include the new problem in help text / flag tables / `all_data_flags_empty()`
- add the `PartialFeedbackEdgeSet` constructor branch in `create.rs`
- reuse existing graph parsing and integer parsing helpers where possible

**Step 4: Run test to verify it passes**

Run: `cargo test partial_feedback_edge_set --package problemreductions-cli`
Expected: PASS for the new CLI tests.

**Step 5: Commit**

```bash
git add problemreductions-cli/src/cli.rs problemreductions-cli/src/commands/create.rs
git commit -m "feat: add PartialFeedbackEdgeSet CLI creation"
```

### Task 4: Verify example-db and repo integration

**Files:**
- Modify: `src/models/graph/mod.rs` (example-spec chain only if still missing after Task 2)
- Test: `cargo test partial_feedback_edge_set`
- Test: `cargo test example_db`

**Step 1: Write the failing test**

If needed, add or extend tests so the canonical example is exercised through example-db / registry-backed serialization.

**Step 2: Run test to verify it fails**

Run: `cargo test example_db`
Expected: FAIL only if the canonical example is not fully wired in.

**Step 3: Write minimal implementation**

Ensure the graph-module example-spec chain includes `partial_feedback_edge_set::canonical_model_example_specs()` so `src/example_db/model_builders.rs` picks it up automatically.

**Step 4: Run test to verify it passes**

Run: `cargo test example_db`
Expected: PASS.

**Step 5: Commit**

```bash
git add src/models/graph/mod.rs src/unit_tests/models/graph/partial_feedback_edge_set.rs
git commit -m "test: verify PartialFeedbackEdgeSet example-db wiring"
```

### Task 5: Batch-1 verification

**Files:**
- Modify: none
- Test: `make test`
- Test: `make clippy`

**Step 1: Run focused verification**

Run:
- `cargo test partial_feedback_edge_set --lib`
- `cargo test partial_feedback_edge_set --package problemreductions-cli`
- `cargo test example_db`

Expected: PASS.

**Step 2: Run repo verification**

Run:
- `make test`
- `make clippy`

Expected: PASS.

**Step 3: Commit any final batch-1 fixes**

```bash
git add -A
git commit -m "chore: finish PartialFeedbackEdgeSet implementation batch"
```

## Batch 2: add-model Step 6

### Task 6: Add the paper entry after implementation is stable

**Files:**
- Modify: `docs/paper/reductions.typ`
- Modify: `docs/paper/references.bib`
- Test: `make paper`

**Step 1: Write the failing test**

Add or finalize `test_partial_feedback_edge_set_paper_example` in `src/unit_tests/models/graph/partial_feedback_edge_set.rs` so it matches the exact paper/example-db instance and the corrected satisfying configuration.

**Step 2: Run test to verify it fails**

Run: `cargo test partial_feedback_edge_set_paper_example --lib`
Expected: FAIL until the paper/example details are aligned.

**Step 3: Write minimal implementation**

Update the paper with:
- a `display-name` entry for `PartialFeedbackEdgeSet`
- a `problem-def("PartialFeedbackEdgeSet")` entry with formal definition
- background that cites Garey & Johnson and, if used for the fixed-`L` hardness remark, a new Yannakakis bibliography entry
- a clean worked example based on the corrected issue instance and satisfying edge set
- a `pred-commands()` block derived from the canonical example, not a hand-written guess

Avoid reintroducing the incorrect Fomin/FVS claim; if no reliable stronger exact-algorithm citation is needed, keep the exact-search claim at the brute-force level already implemented in the crate.

**Step 4: Run test to verify it passes**

Run:
- `cargo test partial_feedback_edge_set_paper_example --lib`
- `make paper`

Expected: PASS.

**Step 5: Commit**

```bash
git add docs/paper/reductions.typ docs/paper/references.bib src/unit_tests/models/graph/partial_feedback_edge_set.rs
git commit -m "docs: add PartialFeedbackEdgeSet paper entry"
```

### Task 7: Final verification before push

**Files:**
- Modify: none
- Test: `git status --short`
- Test: `make test`
- Test: `make clippy`
- Test: `make paper`

**Step 1: Run final verification**

Run:
- `make test`
- `make clippy`
- `make paper`
- `git status --short`

Expected: all commands pass; only intended tracked files are changed.

**Step 2: Commit any final fixes**

```bash
git add -A
git commit -m "chore: finalize #438 verification"
```
