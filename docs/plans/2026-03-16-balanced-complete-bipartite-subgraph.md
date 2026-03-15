# BalancedCompleteBipartiteSubgraph Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add `BalancedCompleteBipartiteSubgraph` as a new graph satisfaction model for the GT24 decision problem, with registry/CLI/example-db integration, paper documentation, and issue-backed tests.

**Architecture:** Implement a concrete `BipartiteGraph`-based satisfaction problem with fields `graph: BipartiteGraph` and `k: usize`. The configuration is one binary decision per vertex: `config[0..left_size]` selects the left side of the biclique and `config[left_size..left_size + right_size]` selects the right side. A configuration is satisfying iff exactly `k` left vertices and exactly `k` right vertices are selected and every selected left/right pair is an edge of the bipartite graph. Register the model through `ProblemSchemaEntry`, `declare_variants!`, graph module exports, example-db hooks, CLI creation, and the paper.

**Tech Stack:** Rust, serde, inventory registry, `BipartiteGraph`, brute-force solver, Typst, `pred` CLI.

---

## Required workflow references

- Implementation skill: `.claude/skills/add-model/SKILL.md` Steps 1-7
- Execution skill: `superpowers:subagent-driven-development`
- Per-task discipline: `superpowers:test-driven-development`

## Issue-derived decisions to preserve

- Treat this as the **decision** problem from issue `#239`, not an optimization variant.
- Use the binary left-then-right encoding from the corrected issue text.
- Keep the model concrete over `BipartiteGraph`; do not introduce a generic graph type parameter.
- Do not edit `problemreductions-cli/src/problem_name.rs`: alias resolution is registry-backed now.
- Companion rule issue already exists: `#231 [Rule] CLIQUE to BALANCED COMPLETE BIPARTITE SUBGRAPH`.
- For the canonical paper/example-db instance, use **Issue Instance 2** so the satisfying witness is unique:
  - `A = {a0,a1,a2,a3}`
  - `B = {b0,b1,b2,b3}`
  - edges `(0,0),(0,1),(0,2),(1,0),(1,1),(1,2),(2,0),(2,1),(2,2),(3,0),(3,1),(3,3)`
  - `k = 3`
  - satisfying sets `{a0,a1,a2}` and `{b0,b1,b2}`
- Keep **Issue Instance 1** as the smaller regression case:
  - `k = 2` is satisfiable, for example with `{a0,a1}` and `{b0,b1}`
  - `k = 3` is unsatisfiable on that same graph
- Complexity caution: the `O*(1.3803^n)` Chen et al. bound is issue-reviewed as **dense-graph-specific**. Unless the source check proves it is a valid worst-case bound for the general problem, keep `declare_variants!` conservative (`"2^num_vertices"`) and mention the dense-case algorithm only in paper prose.

## Batch structure

- **Batch 1:** `.claude/skills/add-model/SKILL.md` Steps 1-5.5
  - model implementation
  - registry/module wiring
  - example-db registration
  - CLI creation support
  - unit tests
  - trait consistency / problem size coverage
- **Batch 2:** `.claude/skills/add-model/SKILL.md` Step 6
  - paper entry
  - bibliography updates
  - paper-aligned test finalization

### Task 1: Model Semantics and Red Tests

**Files:**
- Create: `src/models/graph/balanced_complete_bipartite_subgraph.rs`
- Create: `src/unit_tests/models/graph/balanced_complete_bipartite_subgraph.rs`
- Modify: `src/models/graph/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`

**Step 1: Write the failing model tests first**

Add a new unit test file that covers these exact behaviors before the model exists:

- `test_balanced_complete_bipartite_subgraph_creation`
  - construct `BipartiteGraph::new(4, 4, vec![...])`
  - assert `left_size() == 4`, `right_size() == 4`, `num_vertices() == 8`, `num_edges()` matches edge count, `k() == 2 or 3`
  - assert `dims() == vec![2; 8]`
- `test_balanced_complete_bipartite_subgraph_evaluation_yes_instance`
  - Issue Instance 1 with `k = 2`
  - config `[1,1,0,0, 1,1,0,0]` must satisfy
- `test_balanced_complete_bipartite_subgraph_evaluation_no_instance`
  - same graph as Instance 1 with `k = 3`
  - any candidate using `{a0,a1,a2}` and `{b0,b1,b2}` must fail because edge `(1,2)` is missing
- `test_balanced_complete_bipartite_subgraph_invalid_pairing`
  - choose balanced left/right sets of size `k`, but include at least one missing cross edge; `evaluate()` must return `false`
- `test_balanced_complete_bipartite_subgraph_solver_yes_instance`
  - `BruteForce::find_satisfying()` returns `Some`
  - `BruteForce::find_all_satisfying()` on Instance 2 returns exactly `1` solution
- `test_balanced_complete_bipartite_subgraph_solver_no_instance`
  - on Instance 1 with `k = 3`, `find_satisfying()` returns `None`
- `test_balanced_complete_bipartite_subgraph_serialization`
  - serde round-trip preserves `k`, partition sizes, and edges
- `test_balanced_complete_bipartite_subgraph_is_valid_solution`
  - delegates to `evaluate()`
- `test_balanced_complete_bipartite_subgraph_paper_example`
  - reuse the exact Issue Instance 2 canonical example
  - assert the issue witness satisfies the problem
  - assert `find_all_satisfying()` returns `1`

**Step 2: Run the targeted tests to verify RED**

Run:

```bash
cargo test balanced_complete_bipartite_subgraph --lib
```

Expected: compile failure because the model module/type does not exist yet.

**Step 3: Implement the minimal model to make the tests pass**

Create `src/models/graph/balanced_complete_bipartite_subgraph.rs` with this shape:

- `inventory::submit!` for `ProblemSchemaEntry`
  - `name: "BalancedCompleteBipartiteSubgraph"`
  - `display_name: "Balanced Complete Bipartite Subgraph"`
  - `aliases: &[]`
  - `dimensions: &[]`
  - `description: "Decide whether a bipartite graph contains a K_{k,k} subgraph"`
  - fields mirroring the user-facing bipartite input style used by `BicliqueCover`:
    - `left_size`
    - `right_size`
    - `edges`
    - `k`
- `#[derive(Debug, Clone, Serialize, Deserialize)]`
- `pub struct BalancedCompleteBipartiteSubgraph { graph: BipartiteGraph, k: usize }`
- inherent methods:
  - `new(graph: BipartiteGraph, k: usize) -> Self`
  - `graph(&self) -> &BipartiteGraph`
  - `left_size(&self) -> usize`
  - `right_size(&self) -> usize`
  - `num_vertices(&self) -> usize`
  - `num_edges(&self) -> usize`
  - `k(&self) -> usize`
  - helper to split a config into selected left/right vertex indices
  - `is_valid_solution(&self, config: &[usize]) -> bool`
- `impl Problem`
  - `const NAME = "BalancedCompleteBipartiteSubgraph"`
  - `type Metric = bool`
  - `dims() -> vec![2; self.num_vertices()]`
  - `evaluate()` must:
    - reject wrong config length
    - reject any non-binary entry
    - count selected left and right vertices separately
    - require both counts equal `k`
    - verify every selected pair `(left, right)` exists in the bipartite graph
  - `variant() -> crate::variant_params![]`
- `impl SatisfactionProblem for BalancedCompleteBipartiteSubgraph {}`
- `crate::declare_variants! { default sat BalancedCompleteBipartiteSubgraph => "2^num_vertices", }`
- `#[cfg(feature = "example-db")] canonical_model_example_specs()`
  - use `crate::example_db::specs::satisfaction_example(...)`
  - encode Issue Instance 2 and its unique satisfying configuration
- test link at the bottom:
  - `#[cfg(test)] #[path = "../../unit_tests/models/graph/balanced_complete_bipartite_subgraph.rs"] mod tests;`

**Step 4: Wire the model into graph exports**

Update:

- `src/models/graph/mod.rs`
  - add `pub(crate) mod balanced_complete_bipartite_subgraph;`
  - add `pub use balanced_complete_bipartite_subgraph::BalancedCompleteBipartiteSubgraph;`
  - extend `canonical_model_example_specs()` with the new model helper
- `src/models/mod.rs`
  - add `BalancedCompleteBipartiteSubgraph` to the graph re-export list
- `src/lib.rs`
  - add `BalancedCompleteBipartiteSubgraph` to the `prelude` graph exports

**Step 5: Run the same tests to verify GREEN**

Run:

```bash
cargo test balanced_complete_bipartite_subgraph --lib
```

Expected: the new model unit tests pass.

**Step 6: Commit**

```bash
git add src/models/graph/balanced_complete_bipartite_subgraph.rs \
        src/unit_tests/models/graph/balanced_complete_bipartite_subgraph.rs \
        src/models/graph/mod.rs src/models/mod.rs src/lib.rs
git commit -m "Add BalancedCompleteBipartiteSubgraph model"
```

### Task 2: Registry Completeness, Example DB, Trait Checks, and CLI Creation

**Files:**
- Modify: `src/unit_tests/trait_consistency.rs`
- Modify: `src/unit_tests/problem_size.rs`
- Modify: `problemreductions-cli/src/commands/create.rs`
- Modify: `problemreductions-cli/src/cli.rs`

**Step 1: Write the failing integration tests first**

Add or extend tests before editing the CLI/create logic:

- `src/unit_tests/trait_consistency.rs`
  - add one `check_problem_trait(...)` call for `BalancedCompleteBipartiteSubgraph::new(BipartiteGraph::new(...), 2)`
- `src/unit_tests/problem_size.rs`
  - add `test_problem_size_balanced_complete_bipartite_subgraph`
  - assert `left_size`, `right_size`, `num_edges`, and `k` appear with the correct values
- `problemreductions-cli/src/commands/create.rs`
  - add a small unit test module for `pred create BalancedCompleteBipartiteSubgraph --left 4 --right 4 --biedges ... --k 3`
  - parse the emitted JSON and assert the created problem type is `BalancedCompleteBipartiteSubgraph`
  - assert the serialized instance round-trips into the new model and preserves `k`, partitions, and edges

**Step 2: Run the targeted tests to verify RED**

Run:

```bash
cargo test test_problem_size_balanced_complete_bipartite_subgraph --lib
cargo test test_all_problems_implement_trait_correctly --lib
cargo test -p problemreductions-cli balanced_complete_bipartite_subgraph
```

Expected: failures because the trait/problem-size entries and CLI create arm do not exist yet.

**Step 3: Implement the wiring**

Update `src/unit_tests/trait_consistency.rs`:

- add a `check_problem_trait(...)` entry
- do **not** add a `test_direction` assertion because this is a satisfaction problem

Update `src/unit_tests/problem_size.rs`:

- add a new problem-size test covering the issue-backed example graph

Update `problemreductions-cli/src/commands/create.rs`:

- add a new match arm:
  - `"BalancedCompleteBipartiteSubgraph" => { ... }`
- require:
  - `--left`
  - `--right`
  - `--biedges`
  - `--k`
- parse bipartite-local edges with the same helper used by `BicliqueCover`
- construct `BipartiteGraph::new(left, right, edges)`
- serialize `BalancedCompleteBipartiteSubgraph::new(graph, k)`
- use an error message and usage string parallel to `BicliqueCover`

Update `problemreductions-cli/src/cli.rs`:

- extend the “Flags by problem type” table with:
  - `BalancedCompleteBipartiteSubgraph --left, --right, --biedges, --k`

Do **not** edit `problemreductions-cli/src/problem_name.rs` unless a test proves a real alias-resolution gap. Canonical-name lookup is already case-insensitive through the registry.

**Step 4: Run the targeted tests to verify GREEN**

Run:

```bash
cargo test test_problem_size_balanced_complete_bipartite_subgraph --lib
cargo test test_all_problems_implement_trait_correctly --lib
cargo test -p problemreductions-cli balanced_complete_bipartite_subgraph
```

Expected: all targeted tests pass.

**Step 5: Commit**

```bash
git add src/unit_tests/trait_consistency.rs src/unit_tests/problem_size.rs \
        problemreductions-cli/src/commands/create.rs problemreductions-cli/src/cli.rs
git commit -m "Wire BalancedCompleteBipartiteSubgraph into CLI and registry checks"
```

### Task 3: Paper Entry, Citations, and Paper-Aligned Example

**Files:**
- Modify: `docs/paper/reductions.typ`
- Modify: `docs/paper/references.bib`
- Modify: `src/unit_tests/models/graph/balanced_complete_bipartite_subgraph.rs`

**Step 1: Write or finalize the failing paper-aligned test first**

Before editing the paper, make sure the `test_balanced_complete_bipartite_subgraph_paper_example` test is locked to the exact canonical example chosen above:

- use Issue Instance 2 only
- assert the witness `{a0,a1,a2}` / `{b0,b1,b2}` is satisfying
- assert `find_all_satisfying()` returns exactly `1`

**Step 2: Run the focused test to verify RED if the example changed**

Run:

```bash
cargo test test_balanced_complete_bipartite_subgraph_paper_example --lib
```

Expected: fail if the example data or expected satisfying count is not yet aligned.

**Step 3: Implement the paper entry**

Update `docs/paper/reductions.typ`:

- add display name entry:
  - `"BalancedCompleteBipartiteSubgraph": [Balanced Complete Bipartite Subgraph],`
- add a new `#problem-def("BalancedCompleteBipartiteSubgraph")[ ... ][ ... ]`
- follow the structure/style of:
  - `MaximumIndependentSet` for overall quality
  - `GraphPartitioning` for balanced-language phrasing
  - `BicliqueCover` for bipartite layout and figure structure
  - `SubgraphIsomorphism` / `HamiltonianPath` for satisfaction-problem tone
- definition should state:
  - input bipartite graph `G = (A, B, E)`
  - integer `k`
  - question whether there exist `A' subseteq A` and `B' subseteq B` with `|A'| = |B'| = k` and `A' times B' subseteq E`
- background should:
  - mention the classical Garey-Johnson listing and the relationship to biclique search
  - avoid over-claiming the GT24 tag if the reference check remains ambiguous
- exact algorithm sentence should:
  - mention brute force / subset enumeration unconditionally
  - mention Chen et al. only with a dense-graph qualifier if cited
- add a CeTZ figure using the bipartite-node layout style from `BicliqueCover`
  - draw edges first
  - highlight the six selected vertices and the nine biclique edges for Instance 2
  - explain why `a3` is excluded

Update `docs/paper/references.bib` only as needed:

- `@book{garey1979}` already exists
- add `chen2020` only if the paper prose mentions the dense exact algorithm
- do not add Lin/Manurangsi unless the final prose cites parameterized or approximation results

**Step 4: Verify the paper entry and the aligned test**

Run:

```bash
cargo test test_balanced_complete_bipartite_subgraph_paper_example --lib
make paper
```

Expected: the focused test passes and the Typst paper builds cleanly.

**Step 5: Commit**

```bash
git add docs/paper/reductions.typ docs/paper/references.bib \
        src/unit_tests/models/graph/balanced_complete_bipartite_subgraph.rs
git commit -m "Document BalancedCompleteBipartiteSubgraph in the paper"
```

### Task 4: Full Verification Before Review

**Files:**
- No new files; verification only

**Step 1: Run the full verification suite**

Run:

```bash
cargo test
make clippy
make export-schemas
make paper
```

Expected:

- `cargo test` passes across the workspace
- `make clippy` passes
- `make export-schemas` updates checked-in schema/export artifacts if needed
- `make paper` passes after any schema/export changes

**Step 2: Inspect generated diffs**

Check for expected generated-file updates, especially:

- `docs/src/reductions/problem_schemas.json`
- `src/example_db/fixtures/examples.json` if example-db exports were regenerated indirectly

Only keep generated-file changes that are actually required by the new model.

**Step 3: Commit the final implementation batch**

```bash
git add -A
git commit -m "Finish BalancedCompleteBipartiteSubgraph integration"
```

**Step 4: Hand off to review**

After this plan is executed, run the repo-local `review-implementation` skill before pushing the implementation summary comment.
