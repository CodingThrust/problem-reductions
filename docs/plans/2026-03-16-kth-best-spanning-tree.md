# KthBestSpanningTree Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the `KthBestSpanningTree` satisfaction model, wire it into the registry/CLI/example database/paper, and implement the issue #249 example semantics.

**Architecture:** Model `KthBestSpanningTree` as a `SimpleGraph` plus per-edge weights, a target multiplicity `k`, and a weight bound `B`. Each configuration is `k` consecutive binary edge-selection blocks of length `num_edges`; `evaluate()` accepts exactly those assignments where every block is a distinct spanning tree whose total weight is at most `B`.

**Tech Stack:** Rust, serde, petgraph-backed `SimpleGraph`, registry macros (`inventory::submit!`, `declare_variants!`), CLI `pred create`, Typst paper.

**Notes:** The issue comment thread already settled two design points that this plan follows: `dims()` uses `k * num_edges` binary variables, and the registry complexity uses the general-case bound `2^(num_edges * k)`. There is no open associated rule issue whose title mentions `KthBestSpanningTree`; carry that orphan-model warning into the PR summary instead of blocking execution.

---

## Batch 1: Model, Registry, CLI, Tests

### Task 1: Add failing model tests for the issue semantics

**Files:**
- Create: `src/unit_tests/models/graph/kth_best_spanning_tree.rs`

**Step 1: Write the failing test**

Add focused tests that reference the new type before it exists:
- `test_kthbestspanningtree_creation`
- `test_kthbestspanningtree_evaluation_yes_instance`
- `test_kthbestspanningtree_evaluation_rejects_duplicate_trees`
- `test_kthbestspanningtree_evaluation_rejects_overweight_tree`
- `test_kthbestspanningtree_solver_yes_and_no_examples`
- `test_kthbestspanningtree_serialization`
- `test_kthbestspanningtree_paper_example`

Use the issue’s two worked instances as the source of truth:
- YES instance: 5-vertex, 8-edge graph with `k = 3`, `bound = 12`
- NO instance: 4-vertex path with `k = 2`, `bound = 3`

**Step 2: Run test to verify it fails**

Run: `cargo test kthbestspanningtree --lib`
Expected: FAIL because `KthBestSpanningTree` does not exist yet.

**Step 3: Write minimal implementation**

Create `src/models/graph/kth_best_spanning_tree.rs` with:
- `ProblemSchemaEntry` for `graph = SimpleGraph`, `weight = i32`
- `#[derive(Debug, Clone, Serialize, Deserialize)] pub struct KthBestSpanningTree<W: WeightElement>`
- Constructor `new(graph, weights, k, bound)` validating edge-weight count and positive `k`
- Getters `graph()`, `weights()`, `k()`, `bound()`, `num_vertices()`, `num_edges()`
- Helper predicates for one block:
  - length/range check for binary edge choices
  - exact `n - 1` selected-edge count
  - selected-edge subgraph connectivity
  - total selected weight `<= bound`
- Pairwise distinctness check across the `k` blocks
- `Problem<Metric = bool>` implementation with `dims() = vec![2; self.k * self.graph.num_edges()]`
- `variant() = crate::variant_params![W]`
- `impl SatisfactionProblem`
- `declare_variants! { default sat KthBestSpanningTree<i32> => "2^(num_edges * k)", }`
- `#[cfg(feature = "example-db")] canonical_model_example_specs()` using the issue YES instance
- test link at the bottom

Prefer the `RuralPostman` pattern for `bound: W::Sum` rather than `bound: W`, and only register the `i32` variant because the issue requires integer edge weights.

**Step 4: Run test to verify it passes**

Run: `cargo test kthbestspanningtree --lib`
Expected: PASS for the new targeted model tests.

**Step 5: Commit**

Run:
```bash
git add src/models/graph/kth_best_spanning_tree.rs src/unit_tests/models/graph/kth_best_spanning_tree.rs
git commit -m "Add KthBestSpanningTree model"
```

### Task 2: Register the model across the crate and example-db

**Files:**
- Modify: `src/models/graph/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`
- Modify: `src/unit_tests/trait_consistency.rs`

**Step 1: Write the failing test**

Add a trait-consistency entry for a tiny positive instance so the new model is checked by the shared trait suite.

**Step 2: Run test to verify it fails**

Run: `cargo test trait_consistency --lib`
Expected: FAIL before the new type is exported and referenced consistently.

**Step 3: Write minimal implementation**

Wire the type through the graph module, top-level model exports, and prelude exports. In `trait_consistency.rs`, add one `check_problem_trait(...)` call using a 3-vertex triangle with `k = 1`, plus any minimal imports needed.

Also add the new example-db spec to `src/models/graph/mod.rs` so `build_model_examples()` includes the canonical example.

**Step 4: Run test to verify it passes**

Run: `cargo test trait_consistency --lib`
Expected: PASS with the new model included.

**Step 5: Commit**

Run:
```bash
git add src/models/graph/mod.rs src/models/mod.rs src/lib.rs src/unit_tests/trait_consistency.rs
git commit -m "Register KthBestSpanningTree across the crate"
```

### Task 3: Add CLI creation support for the new model

**Files:**
- Modify: `problemreductions-cli/src/commands/create.rs`
- Modify: `problemreductions-cli/src/cli.rs`

**Step 1: Write the failing test**

Use a direct CLI smoke command as the behavior check:
- `pred create KthBestSpanningTree --graph 0-1,1-2,2-0 --edge-weights 1,2,3 --k 1 --bound 3`

No dedicated Rust test file is required if the smoke command is the verification target for this task.

**Step 2: Run test to verify it fails**

Run: `cargo run -p problemreductions-cli -- create KthBestSpanningTree --graph 0-1,1-2,2-0 --edge-weights 1,2,3 --k 1 --bound 3`
Expected: FAIL with unknown problem or unsupported create-path errors before the CLI wiring exists.

**Step 3: Write minimal implementation**

Update CLI creation support by:
- Adding `KthBestSpanningTree` to the help text table in `problemreductions-cli/src/cli.rs`
- Adding an `example_for()` entry in `problemreductions-cli/src/commands/create.rs`
- Adding a `match` arm that parses:
  - `--graph`
  - `--edge-weights`
  - `--k`
  - `--bound`
- Constructing `KthBestSpanningTree::new(graph, edge_weights, k, bound)`

Use the existing `RuralPostman` and weighted-graph parsing helpers as the template.

**Step 4: Run test to verify it passes**

Run: `cargo run -p problemreductions-cli -- create KthBestSpanningTree --graph 0-1,1-2,2-0 --edge-weights 1,2,3 --k 1 --bound 3`
Expected: PASS and emit valid problem JSON.

**Step 5: Commit**

Run:
```bash
git add problemreductions-cli/src/commands/create.rs problemreductions-cli/src/cli.rs
git commit -m "Add CLI support for KthBestSpanningTree"
```

## Batch 2: Paper Entry and Paper-Aligned Verification

### Task 4: Add the paper entry and align the paper example test

**Files:**
- Modify: `docs/paper/reductions.typ`
- Modify: `src/unit_tests/models/graph/kth_best_spanning_tree.rs`

**Step 1: Write the failing test**

Ensure `test_kthbestspanningtree_paper_example` uses the exact same instance and witness that the paper will show. If the current test uses the issue example already, tighten it so it also checks the claimed count of satisfying witnesses from brute force.

**Step 2: Run test to verify it fails**

Run: `cargo test test_kthbestspanningtree_paper_example --lib`
Expected: FAIL until the test and paper example agree on one concrete instance.

**Step 3: Write minimal implementation**

In `docs/paper/reductions.typ`:
- Add `"KthBestSpanningTree": [Kth Best Spanning Tree],` to `display-name`
- Add a `problem-def("KthBestSpanningTree")[...][...]` entry near the other graph problems
- Load the canonical example with `load-model-example("KthBestSpanningTree")`
- Use the issue’s YES instance as the worked example, including:
  - the graph and edge weights
  - `k = 3`, `B = 12`
  - three distinct spanning trees under the bound
  - the explanation that the witness is accepted because all three blocks are valid, distinct, and under the bound
- Cite Garey & Johnson and mention the comment-thread correction about Eppstein’s bounds in prose instead of restating the incorrect issue draft

Then update `test_kthbestspanningtree_paper_example` so it matches the exact paper instance and asserted satisfying-witness count.

**Step 4: Run test to verify it passes**

Run:
- `cargo test test_kthbestspanningtree_paper_example --lib`
- `make paper`

Expected: both PASS.

**Step 5: Commit**

Run:
```bash
git add docs/paper/reductions.typ src/unit_tests/models/graph/kth_best_spanning_tree.rs
git commit -m "Document KthBestSpanningTree in the paper"
```

## Final Verification

After all tasks:

1. Run targeted verification first:
   - `cargo test kthbestspanningtree --lib`
   - `cargo test trait_consistency --lib`
   - `cargo test example_db --lib`
   - `cargo run -p problemreductions-cli -- create KthBestSpanningTree --graph 0-1,1-2,2-0 --edge-weights 1,2,3 --k 1 --bound 3`
2. Run repo verification:
   - `make test`
   - `make clippy`
   - `make paper`
3. Run `review-implementation` after code lands to catch structural omissions before the final implementation commit.
