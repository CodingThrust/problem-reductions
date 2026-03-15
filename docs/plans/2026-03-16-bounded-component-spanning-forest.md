# BoundedComponentSpanningForest Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the `BoundedComponentSpanningForest` graph satisfaction model, wire it into the registry/CLI/example flows, document it in the paper, and verify the issue's YES/NO instances end-to-end.

**Architecture:** Implement `BoundedComponentSpanningForest<G, W>` as a graph-based satisfaction problem where each vertex is assigned one of at most `K` component ids. Evaluation groups vertices by assigned component, rejects any component whose total vertex weight exceeds `B`, and rejects any non-empty component whose induced subgraph is disconnected. Start with the default concrete registry variant `BoundedComponentSpanningForest<SimpleGraph, i32>` and keep the paper example aligned with the issue's YES instance.

**Tech Stack:** Rust workspace, `serde`, graph `Problem`/`SatisfactionProblem` traits, registry metadata via `inventory` + `declare_variants!`, `pred create` CLI, Typst paper, mdBook schema/graph exports.

---

**Execution notes**

- This issue is currently an orphan model: `gh issue list --label rule --state open` returns no open rule issues whose title references `BoundedComponentSpanningForest`. Keep a visible orphan-model warning in the PR description unless a companion rule issue is filed separately.
- The issue-to-PR helper currently overmatches existing PRs for this issue number. Execute this plan in the dedicated `issue-251-bounded-component-spanning-forest` worktree and do not resume the unrelated PR `#631`.
- Use the issue's YES instance as the canonical implementation-facing example:
  - Graph edges: `(0,1) (1,2) (2,3) (3,4) (4,5) (5,6) (6,7) (0,7) (1,5) (2,6)`
  - Weights: `[2, 3, 1, 2, 3, 1, 2, 1]`
  - `K = 3`, `B = 6`
  - Satisfying assignment: `[0, 0, 1, 1, 1, 2, 2, 0]`, representing components `{0,1,7}`, `{2,3,4}`, `{5,6}`
- Also preserve a NO instance in tests:
  - Graph edges: `(0,1) (1,2) (3,4) (4,5)`
  - Weights: `[1, 1, 1, 1, 1, 1]`
  - `K = 2`, `B = 2`
  - Expected result: no satisfying assignment exists.

## Batch 1: Model + Registry + CLI + Tests

### Task 1: Write the failing model tests first

**Files:**
- Create: `src/unit_tests/models/graph/bounded_component_spanning_forest.rs`

**Step 1: Add failing tests for the issue behavior**

Write tests that cover:
- construction + getters (`graph`, `weights`, `max_components`, `max_weight`, `dims`)
- YES-instance evaluation using config `[0, 0, 1, 1, 1, 2, 2, 0]`
- rejection when a component exceeds the weight bound
- rejection when a component is disconnected
- rejection when a config uses an out-of-range component id or wrong length
- serde round-trip
- solver behavior: `find_satisfying()` succeeds on the YES instance and returns `None` on the NO instance
- paper/example alignment test using the canonical YES instance

**Step 2: Run the new test file to verify RED**

Run:
```bash
cargo test bounded_component_spanning_forest --lib
```

Expected: FAIL because `BoundedComponentSpanningForest` does not exist yet.

### Task 2: Implement the model and expose it from the crate

**Files:**
- Create: `src/models/graph/bounded_component_spanning_forest.rs`
- Modify: `src/models/graph/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`

**Step 1: Implement `BoundedComponentSpanningForest<G, W>`**

Use the same broad shape as `RuralPostman` and `KColoring`:
- `graph: G`
- `weights: Vec<W>`
- `max_components: usize`
- `max_weight: W::Sum`

Include:
- `inventory::submit!` registration with `name = "BoundedComponentSpanningForest"`
- dimensions:
  - `graph` default/allowed: `SimpleGraph`
  - `weight` default/allowed: `i32`
- field schema entries for `graph`, `weights`, `max_components`, `max_weight`
- constructor validation:
  - `weights.len() == graph.num_vertices()`
  - `max_components >= 1`
- getters:
  - `graph()`
  - `weights()`
  - `max_components()`
  - `max_weight()`
  - `num_vertices()`
  - `num_edges()`
  - `is_weighted()`
- helper methods:
  - `is_valid_solution(&self, config: &[usize]) -> bool`
  - an internal connectivity helper that BFS/DFSes only within one assigned component

**Step 2: Implement the trait layer**

Implement:
- `Problem<Metric = bool>`
- `SatisfactionProblem`
- `variant()` via `crate::variant_params![G, W]`
- `dims()` as `vec![self.max_components; self.graph.num_vertices()]`
- `evaluate()` as a pure feasibility check returning `true`/`false`

For feasibility, reject when:
- config length differs from `num_vertices`
- any component id is `>= max_components`
- any non-empty component induces a disconnected subgraph
- any component's total weight exceeds `max_weight`

**Step 3: Register the concrete variant + example-db hook**

At the bottom of the model file:
- add `crate::declare_variants! { default sat BoundedComponentSpanningForest<SimpleGraph, i32> => "max_components^num_vertices" }`
- add `canonical_model_example_specs()` returning a satisfaction example built from the issue YES instance
- link the test file with `#[cfg(test)] #[path = "../../unit_tests/models/graph/bounded_component_spanning_forest.rs"]`

**Step 4: Re-export the new type**

Update:
- `src/models/graph/mod.rs`
- `src/models/mod.rs`
- `src/lib.rs` prelude exports

**Step 5: Run the targeted tests to verify GREEN**

Run:
```bash
cargo test bounded_component_spanning_forest --lib
```

Expected: PASS for the new model test file, plus any matching integration hooks.

### Task 3: Add trait-consistency coverage and exported schema support

**Files:**
- Modify: `src/unit_tests/trait_consistency.rs`

**Step 1: Add the new problem to trait consistency**

Add a small instance such as a 3-vertex path with unit weights expressed as `i32`, `K = 2`, `B = 2` to `test_all_problems_implement_trait_correctly`.

Because this is a satisfaction problem, do not add it to `test_direction()`.

**Step 2: Verify the new trait-consistency entry**

Run:
```bash
cargo test trait_consistency --lib
```

Expected: PASS with the new problem included.

### Task 4: Wire `pred create` for the new model

**Files:**
- Modify: `problemreductions-cli/src/commands/create.rs`
- Modify: `problemreductions-cli/src/cli.rs`

**Step 1: Add a concrete `pred create BoundedComponentSpanningForest` arm**

Use existing flags only:
- `--graph`
- `--weights`
- `--k`
- `--bound`

The create arm should:
- parse `graph` as `SimpleGraph`
- require `weights`
- require `k`
- require `bound`
- validate `weights.len() == graph.num_vertices()`
- serialize `BoundedComponentSpanningForest::<SimpleGraph, i32>::new(graph, weights, k, bound as i32)`

Do not add a new alias unless there is a literature-standard abbreviation.

**Step 2: Update CLI help/examples**

Add:
- an example string in `example_for(...)`
- a help-table row in `problemreductions-cli/src/cli.rs` for `BoundedComponentSpanningForest`

If the existing `--bound` doc string only mentions other problems, extend it to include this model.

**Step 3: Verify the CLI path**

Run:
```bash
cargo run -p problemreductions-cli -- create BoundedComponentSpanningForest --graph 0-1,1-2,2-3,3-4,4-5,5-6,6-7,0-7,1-5,2-6 --weights 2,3,1,2,3,1,2,1 --k 3 --bound 6
```

Expected: PASS and print JSON for a `BoundedComponentSpanningForest` instance.

### Task 5: Regenerate exports and run focused verification

**Files:**
- Modify/generated: `docs/src/reductions/problem_schemas.json`
- Modify/generated: `docs/src/reductions/reduction_graph.json`

**Step 1: Refresh the generated schema/graph artifacts**

Run:
```bash
cargo run --example export_graph
cargo run --example export_schemas
```

Expected:
- `problem_schemas.json` gains `BoundedComponentSpanningForest`
- `reduction_graph.json` updates consistently for the new catalog state

**Step 2: Run the focused verification set**

Run:
```bash
cargo test bounded_component_spanning_forest --lib
cargo test trait_consistency --lib
cargo test example_db --lib --features example-db
```

Expected: PASS.

## Batch 2: Paper Entry

### Task 6: Add the paper entry after the code and exports are stable

**Files:**
- Modify: `docs/paper/reductions.typ`

**Step 1: Register the display name**

Add:
- `"BoundedComponentSpanningForest": [Bounded Component Spanning Forest],`

to the `display-name` dictionary.

**Step 2: Add `problem-def("BoundedComponentSpanningForest")`**

Model it after the existing graph problem entries. Include:
- formal definition with graph `G = (V, E)`, vertex weights `w`, component limit `K`, and weight bound `B`
- short background on Garey & Johnson ND10 and its partitioning/redistricting interpretation
- a sentence on the exponential exact search bound used in the catalog (`O^*(K^n)`)
- the issue YES instance as the tutorial example, with one explicitly shown satisfying partition and a short verification argument

Avoid claiming a stronger exact algorithm unless the citation is precise and directly supports this exact weighted connected-partition variant.

**Step 3: Build the paper**

Run:
```bash
make paper
```

Expected: PASS with the new model rendered in the PDF.

## Batch 3: Final Verification + Review

### Task 7: Run the full repo checks required before review

**Files:**
- No new files; verification only

**Step 1: Run formatting and lint/test verification**

Run:
```bash
make fmt
make clippy
make test
```

Expected: PASS.

**Step 2: Run structural implementation review**

After all code is green, run the repo-local review workflow:

```bash
/review-implementation
```

Auto-fix any issues it finds, then rerun the relevant verification commands until green.

**Step 3: Prepare the PR summary**

Before pushing the implementation commits:
- summarize the new model, tests, CLI support, generated exports, and paper entry
- note the deliberate deviation from the automated helper: the pipeline helper falsely matched PR `#631`, so this issue was executed in a fresh dedicated worktree instead of resuming that unrelated branch
- include the orphan-model warning unless a companion rule issue now exists
