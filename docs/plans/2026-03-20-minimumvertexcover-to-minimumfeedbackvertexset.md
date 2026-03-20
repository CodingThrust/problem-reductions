# MinimumVertexCover to MinimumFeedbackVertexSet Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the `MinimumVertexCover -> MinimumFeedbackVertexSet` reduction, its closed-loop coverage, a canonical example fixture, and the paper entry required for the reduction catalog.

**Architecture:** Follow the existing graph-to-graph reduction pattern used by [`src/rules/minimumvertexcover_maximumindependentset.rs`](/Users/jinguomini/rcode/problem-reductions/.worktrees/issue-207/src/rules/minimumvertexcover_maximumindependentset.rs), but target a directed graph model. The reduction keeps the same vertex set and weights, replaces each undirected edge `{u,v}` with directed arcs `(u,v)` and `(v,u)`, and uses identity solution extraction because both source and target configurations mark the selected vertices with `1`. The implementation batch should stop once the code, tests, and canonical rule example are in place; the paper/export batch runs afterward because it depends on generated fixture data.

**Tech Stack:** Rust, cargo test, the reduction registry macro `#[reduction]`, BruteForce solver, Typst paper docs, example-db fixture generation.

---

## Batch 1: Code, tests, and canonical example

### Task 1: Write the failing reduction tests

**Files:**
- Create: `src/unit_tests/rules/minimumvertexcover_minimumfeedbackvertexset.rs`
- Reference: `src/rules/test_helpers.rs`
- Reference: `src/unit_tests/rules/minimumvertexcover_maximumindependentset.rs`

**Step 1: Write the failing test**

Add three tests:

```rust
#[test]
fn test_minimumvertexcover_to_minimumfeedbackvertexset_closed_loop() {
    let source = MinimumVertexCover::new(
        SimpleGraph::new(7, vec![
            (0, 1), (0, 2), (0, 3), (1, 2),
            (1, 3), (3, 4), (4, 5), (5, 6),
        ]),
        vec![1i32; 7],
    );
    let reduction = ReduceTo::<MinimumFeedbackVertexSet<i32>>::reduce_to(&source);
    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "MVC -> MFVS closed loop",
    );
}

#[test]
fn test_minimumvertexcover_to_minimumfeedbackvertexset_structure() {
    let source = MinimumVertexCover::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        vec![10, 20, 30],
    );
    let reduction = ReduceTo::<MinimumFeedbackVertexSet<i32>>::reduce_to(&source);
    let target = reduction.target_problem();
    assert_eq!(target.num_vertices(), 3);
    assert_eq!(target.num_arcs(), 4);
    assert_eq!(target.weights(), &[10, 20, 30]);
    assert_eq!(
        target.graph().arcs(),
        vec![(0, 1), (1, 0), (1, 2), (2, 1)],
    );
}

#[test]
fn test_minimumvertexcover_to_minimumfeedbackvertexset_identity_extraction() {
    let source = MinimumVertexCover::new(SimpleGraph::new(2, vec![(0, 1)]), vec![1, 1]);
    let reduction = ReduceTo::<MinimumFeedbackVertexSet<i32>>::reduce_to(&source);
    assert_eq!(reduction.extract_solution(&[1, 0]), vec![1, 0]);
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_minimumvertexcover_to_minimumfeedbackvertexset_closed_loop -- --exact`
Expected: FAIL because the rule module is not implemented or not registered yet.

**Step 3: Commit**

Do not commit yet. This task stays uncommitted until the implementation turns the tests green.

### Task 2: Implement the reduction and register it

**Files:**
- Create: `src/rules/minimumvertexcover_minimumfeedbackvertexset.rs`
- Modify: `src/rules/mod.rs`
- Test: `src/unit_tests/rules/minimumvertexcover_minimumfeedbackvertexset.rs`

**Step 1: Write minimal implementation**

Create the rule module with this structure:

```rust
#[derive(Debug, Clone)]
pub struct ReductionVCToFVS<W> {
    target: MinimumFeedbackVertexSet<W>,
}

impl<W> ReductionResult for ReductionVCToFVS<W>
where
    W: WeightElement + crate::variant::VariantParam,
{
    type Source = MinimumVertexCover<SimpleGraph, W>;
    type Target = MinimumFeedbackVertexSet<W>;

    fn target_problem(&self) -> &Self::Target { &self.target }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(overhead = {
    num_vertices = "num_vertices",
    num_arcs = "2 * num_edges",
})]
impl ReduceTo<MinimumFeedbackVertexSet<i32>> for MinimumVertexCover<SimpleGraph, i32> {
    type Result = ReductionVCToFVS<i32>;

    fn reduce_to(&self) -> Self::Result {
        let arcs = self.graph().edges().iter().flat_map(|&(u, v)| [(u, v), (v, u)]).collect();
        let target = MinimumFeedbackVertexSet::new(
            DirectedGraph::new(self.graph().num_vertices(), arcs),
            self.weights().to_vec(),
        );
        ReductionVCToFVS { target }
    }
}
```

Register the module in `src/rules/mod.rs` next to the other `minimumvertexcover_*` rules.

**Step 2: Run test to verify it passes**

Run:
- `cargo test test_minimumvertexcover_to_minimumfeedbackvertexset_closed_loop -- --exact`
- `cargo test test_minimumvertexcover_to_minimumfeedbackvertexset_structure -- --exact`
- `cargo test test_minimumvertexcover_to_minimumfeedbackvertexset_identity_extraction -- --exact`

Expected: PASS.

**Step 3: Refactor**

Keep the arc construction readable and deterministic. If needed, extract a small helper inside the rule file, but do not add any abstraction beyond this rule.

**Step 4: Commit**

```bash
git add src/rules/minimumvertexcover_minimumfeedbackvertexset.rs src/rules/mod.rs src/unit_tests/rules/minimumvertexcover_minimumfeedbackvertexset.rs
git commit -m "Add MinimumVertexCover to MinimumFeedbackVertexSet reduction"
```

### Task 3: Add the canonical rule example

**Files:**
- Modify: `src/rules/minimumvertexcover_minimumfeedbackvertexset.rs`
- Reference: `src/rules/minimumvertexcover_maximumindependentset.rs`

**Step 1: Write the failing example-db expectation**

Add a `#[cfg(feature = "example-db")] pub(crate) fn canonical_rule_example_specs() -> Vec<RuleExampleSpec>` block to the new rule file. Use the issue’s 7-vertex example and one verified witness pair:

```rust
SolutionPair {
    source_config: vec![1, 1, 0, 1, 0, 1, 0],
    target_config: vec![1, 1, 0, 1, 0, 1, 0],
}
```

The source instance should be:

```rust
MinimumVertexCover::new(
    SimpleGraph::new(
        7,
        vec![
            (0, 1), (0, 2), (0, 3), (1, 2),
            (1, 3), (3, 4), (4, 5), (5, 6),
        ],
    ),
    vec![1i32; 7],
)
```

**Step 2: Run test to verify it fails**

Run: `cargo test example_db -- --nocapture`
Expected: FAIL if the new rule spec is not wired into the aggregated registry.

**Step 3: Write minimal implementation**

Add the new rule’s spec to its module and extend `src/rules/mod.rs` inside `canonical_rule_example_specs()` with:

```rust
specs.extend(minimumvertexcover_minimumfeedbackvertexset::canonical_rule_example_specs());
```

**Step 4: Run test to verify it passes**

Run: `cargo test example_db -- --nocapture`
Expected: PASS.

**Step 5: Commit**

```bash
git add src/rules/minimumvertexcover_minimumfeedbackvertexset.rs src/rules/mod.rs
git commit -m "Add canonical example for MinimumVertexCover to MinimumFeedbackVertexSet"
```

### Task 4: Run focused verification for the implementation batch

**Files:**
- No code changes expected

**Step 1: Run the relevant tests**

Run:
- `cargo test minimumvertexcover_minimumfeedbackvertexset`
- `cargo test minimum_feedback_vertex_set`
- `cargo test minimum_vertex_cover`

Expected: PASS.

**Step 2: Run the library quality checks for touched code**

Run:
- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`

Expected: PASS.

**Step 3: Commit**

Do not create a new commit unless one of the verification commands requires a small follow-up fix.

## Batch 2: Paper entry, exports, fixtures, and final verification

### Task 5: Add the paper reduction entry and worked example

**Files:**
- Modify: `docs/paper/reductions.typ`
- Reference: `docs/paper/reductions.typ` near `#reduction-rule("MinimumVertexCover", "MaximumIndependentSet", ...)`
- Reference: `docs/paper/reductions.typ` near `#problem-def("MinimumFeedbackVertexSet")`

**Step 1: Write the failing paper entry**

Insert a new block near the other `MinimumVertexCover` reductions:

```typst
#let mvc_fvs = load-example("MinimumVertexCover", "MinimumFeedbackVertexSet")
#let mvc_fvs_sol = mvc_fvs.solutions.at(0)
#reduction-rule("MinimumVertexCover", "MinimumFeedbackVertexSet",
  example: true,
  example-caption: [7-vertex graph: each undirected edge becomes a directed 2-cycle],
  extra: [
    ...
  ],
)[
  ...
][
  _Construction._ ...
  _Correctness._ ...
  _Solution extraction._ ...
]
```

The narrative must explicitly fix the earlier issue wording mistake: longer directed cycles can exist, but hitting every bidirected edge-pair already hits every longer cycle because each cycle uses source edges whose endpoints are covered.

**Step 2: Run test to verify it fails**

Run: `make paper`
Expected: FAIL before the reduction is documented or before fixture data exists.

**Step 3: Write minimal implementation**

Fill in:
- theorem body: bidirected-edge construction, same weights, same budget, overhead `n` vertices and `2m` arcs
- proof body: forward/backward correctness and identity extraction
- worked example driven from `mvc_fvs` fixture data, not hardcoded duplicate numbers

**Step 4: Run test to verify it passes**

Run: `make paper`
Expected: PASS once the example fixture data and exports are in place.

**Step 5: Commit**

```bash
git add docs/paper/reductions.typ
git commit -m "Document MinimumVertexCover to MinimumFeedbackVertexSet reduction"
```

### Task 6: Regenerate exports and run final verification

**Files:**
- Generated/updated: `docs/src/reductions/reduction_graph.json`
- Generated/updated: `docs/src/reductions/problem_schemas.json`
- Generated/updated: `src/example_db/fixtures/examples.json`

**Step 1: Regenerate derived artifacts**

Run:
- `cargo run --example export_graph`
- `cargo run --example export_schemas`
- `make regenerate-fixtures`

Expected: generated graph/schema/example fixture data include the new rule.

**Step 2: Run final verification**

Run:
- `make test`
- `make clippy`
- `make paper`

Expected: PASS.

**Step 3: Commit**

```bash
git add docs/src/reductions/reduction_graph.json docs/src/reductions/problem_schemas.json src/example_db/fixtures/examples.json
git commit -m "Regenerate exports for MinimumVertexCover to MinimumFeedbackVertexSet"
```

### Task 7: Prepare the branch for review-pipeline

**Files:**
- Delete: `docs/plans/2026-03-20-minimumvertexcover-to-minimumfeedbackvertexset.md`

**Step 1: Verify branch state**

Run: `git status --short`
Expected: only the plan file remains staged for deletion or the tree is otherwise clean.

**Step 2: Remove the plan file**

Run:

```bash
git rm docs/plans/2026-03-20-minimumvertexcover-to-minimumfeedbackvertexset.md
git commit -m "chore: remove plan file after implementation"
```

**Step 3: Push and summarize**

Run:
- `git push`
- post the PR implementation summary comment describing:
  - the new rule file, tests, and canonical example
  - the paper entry and regenerated fixture/export files
  - any deviations from this plan
