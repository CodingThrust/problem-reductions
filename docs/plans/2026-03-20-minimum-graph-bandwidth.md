# MinimumGraphBandwidth Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the `MinimumGraphBandwidth` graph optimization model, register it across the crate and CLI surfaces, document it in the paper, and verify the canonical `P_2 × P_3` grid example from issue #133.

**Architecture:** Model Graph Bandwidth as an `OptimizationProblem` over a permutation-valued configuration: `config[i]` is the position assigned to vertex `i`, valid solutions are bijections on `{0, ..., n-1}`, and the metric is the maximum edge span `max_{(u,v) in E} |config[u] - config[v]|`. Follow the permutation-validation structure from `src/models/graph/optimal_linear_arrangement.rs`, but return `SolutionSize::Valid(usize)` with `Direction::Minimize` like the optimization-style graph models in `src/models/graph/traveling_salesman.rs` and `src/models/graph/graph_partitioning.rs`.

**Tech Stack:** Rust workspace crates, `serde`, `inventory`, registry-backed `declare_variants!`, Typst paper in `docs/paper`, CLI/MCP create paths in `problemreductions-cli`.

---

## Batch 1: Model, registration, CLI, examples

### Task 1: Write the red tests for the new model semantics

**Files:**
- Create: `src/unit_tests/models/graph/minimum_graph_bandwidth.rs`
- Modify: `src/unit_tests/trait_consistency.rs`

**Step 1: Write the failing tests**

Add tests that lock the intended behavior before implementing the model:
- `test_minimumgraphbandwidth_creation` for `graph()`, `num_vertices()`, `num_edges()`, `dims()`
- `test_minimumgraphbandwidth_direction` asserting `Direction::Minimize`
- `test_minimumgraphbandwidth_invalid_permutation` covering repeated labels, out-of-range labels, wrong length
- `test_minimumgraphbandwidth_issue_example` using the issue’s `2x3` grid with column-major labeling `config = [0, 2, 4, 1, 3, 5]` and expected optimum `2`
- `test_minimumgraphbandwidth_closed_form_graphs` covering small graphs with known bandwidth:
  - path `P4` has optimum `1`
  - clique `K4` has optimum `3`
  - star `K1,3` has optimum `2`
- `test_minimumgraphbandwidth_solver` using `BruteForce::find_best` / `find_all_best`
- `test_minimumgraphbandwidth_serialization`

Also add one `trait_consistency` entry once the type exists, mirroring the existing `OptimalLinearArrangement` check.

**Step 2: Run the tests to verify RED**

Run:

```bash
cargo test minimum_graph_bandwidth --lib
```

Expected: compilation fails because `MinimumGraphBandwidth` does not exist yet.

**Step 3: Commit nothing yet**

Do not commit until the model exists and the tests go green.

### Task 2: Implement the core model and crate registration

**Files:**
- Create: `src/models/graph/minimum_graph_bandwidth.rs`
- Modify: `src/models/graph/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`
- Modify: `src/unit_tests/trait_consistency.rs`

**Step 1: Write the minimal implementation**

Create `MinimumGraphBandwidth<G>` with:
- `graph: G`
- accessors `new`, `graph`, `num_vertices`, `num_edges`
- helpers:
  - `is_valid_permutation(&self, config: &[usize]) -> bool`
  - `max_edge_span(&self, config: &[usize]) -> Option<usize>`
  - optional `is_valid_solution` convenience wrapper if it keeps tests clearer
- `Problem` impl:
  - `const NAME = "MinimumGraphBandwidth"`
  - `type Metric = SolutionSize<usize>`
  - `dims() -> vec![n; n]`
  - `evaluate()` returns `Invalid` for non-permutations and `Valid(max_span)` otherwise
- `OptimizationProblem` impl with `Direction::Minimize`
- `ProblemSchemaEntry` metadata:
  - display name `Minimum Graph Bandwidth`
  - graph dimension only (`SimpleGraph`)
  - field list contains only constructor-facing field `graph`
- `declare_variants!` entry:
  - `default opt MinimumGraphBandwidth<SimpleGraph> => "4.473^num_vertices"`
- `#[cfg(test)]` link to the test file created in Task 1

Register the model in:
- `src/models/graph/mod.rs` module list, re-export list, and `canonical_model_example_specs()`
- `src/models/mod.rs` graph re-exports
- `src/lib.rs` root/prelude re-exports

Add one `trait_consistency` instance such as `MinimumGraphBandwidth::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]))`.

**Step 2: Add the canonical model example in the model file**

Inside `src/models/graph/minimum_graph_bandwidth.rs`, add `canonical_model_example_specs()` under `#[cfg(feature = "example-db")]` using the issue’s grid:
- graph edges `[(0,1), (1,2), (0,3), (1,4), (2,5), (3,4), (4,5)]`
- `optimal_config = vec![0, 2, 4, 1, 3, 5]`
- `optimal_value = serde_json::json!({"Valid": 2})`
- stable example id such as `"minimum_graph_bandwidth"`

**Step 3: Run the focused tests to verify GREEN**

Run:

```bash
cargo test minimum_graph_bandwidth --lib
cargo test trait_consistency --lib
cargo test example_db --features example-db --lib
```

Expected: the new model tests and the trait/example-db checks pass.

**Step 4: Commit**

```bash
git add src/models/graph/minimum_graph_bandwidth.rs src/models/graph/mod.rs src/models/mod.rs src/lib.rs src/unit_tests/models/graph/minimum_graph_bandwidth.rs src/unit_tests/trait_consistency.rs
git commit -m "Add MinimumGraphBandwidth model"
```

### Task 3: Add CLI and MCP creation support for the new graph-only model

**Files:**
- Modify: `problemreductions-cli/src/commands/create.rs`
- Modify: `problemreductions-cli/src/cli.rs`
- Modify: `problemreductions-cli/src/mcp/tools.rs`
- Modify: `problemreductions-cli/tests/cli_tests.rs`

**Step 1: Write the failing CLI tests**

Add CLI coverage for:
- `pred create MinimumGraphBandwidth --graph 0-1,1-2,2-3`
  - verifies JSON `problem_type`, graph payload, and successful exit
- `pred create MinimumGraphBandwidth --random --num-vertices 5`
  - verifies random creation works for `SimpleGraph`
- help/example coverage:
  - the `after_help` table mentions `MinimumGraphBandwidth --graph`
  - the example argument helper returns a graph-only example

If the MCP tool layer already has creation tests, extend them; otherwise rely on CLI tests plus focused manual invocation of the mirrored code path.

**Step 2: Run the tests to verify RED**

Run:

```bash
cargo test -p problemreductions-cli minimum_graph_bandwidth
```

Expected: failure because create/help/random support is missing.

**Step 3: Implement the minimal CLI and MCP support**

Update `problemreductions-cli/src/commands/create.rs` in all three places that currently special-case graph-only or random-create models:
- `example_for()` with a graph-only example string
- main `create()` match with a graph-only branch like `GraphPartitioning`
- random-create match with a `SimpleGraph` branch like `GraphPartitioning` / `HamiltonianCircuit`

Update `problemreductions-cli/src/cli.rs`:
- include `MinimumGraphBandwidth` in the `Flags by problem type` help table
- update any user-facing bound/help text only if the new model truly needs it (it should not)

Update `problemreductions-cli/src/mcp/tools.rs`:
- add `MinimumGraphBandwidth` to the graph-only create path
- add it to the graph-only random-create path
- keep the supported-random-problems error text in sync

Do **not** add a manual `problem_name.rs` alias unless tests prove registry-based resolution is insufficient; there is no well-established short alias to add.

**Step 4: Run the focused tests to verify GREEN**

Run:

```bash
cargo test -p problemreductions-cli minimum_graph_bandwidth
```

Expected: the new CLI tests pass.

**Step 5: Commit**

```bash
git add problemreductions-cli/src/commands/create.rs problemreductions-cli/src/cli.rs problemreductions-cli/src/mcp/tools.rs problemreductions-cli/tests/cli_tests.rs
git commit -m "Add CLI support for MinimumGraphBandwidth"
```

### Task 4: Tighten model regressions around examples and solver behavior

**Files:**
- Modify: `src/unit_tests/models/graph/minimum_graph_bandwidth.rs`

**Step 1: Add any still-missing focused tests**

If Task 1’s first pass was minimal, finish the model test matrix now:
- assert the exact maximum span for the canonical grid example is `2`
- assert the row-major arrangement `vec![0, 1, 2, 3, 4, 5]` evaluates to `Valid(3)`
- assert empty graph behavior (`n` isolated vertices) gives optimum `0`
- assert `BruteForce::find_all_best()` contains all optimal orderings for very small symmetric graphs where that count is easy to verify

**Step 2: Run the focused tests**

Run:

```bash
cargo test minimum_graph_bandwidth --lib
cargo test example_db --features example-db --lib
```

Expected: all model-side tests remain green with the canonical example wired through example-db.

**Step 3: Commit**

```bash
git add src/unit_tests/models/graph/minimum_graph_bandwidth.rs
git commit -m "Expand MinimumGraphBandwidth tests"
```

## Batch 2: Paper entry and final verification

### Task 5: Document the model in the paper and align the paper-example test

**Files:**
- Modify: `docs/paper/reductions.typ`
- Modify: `docs/paper/references.bib`
- Modify: `src/unit_tests/models/graph/minimum_graph_bandwidth.rs`

**Step 1: Write the failing paper-example test**

Add `test_minimumgraphbandwidth_paper_example` that uses the exact paper/example-db grid instance and asserts:
- the column-major labeling gives `SolutionSize::Valid(2)`
- brute force confirms the optimum is `2`
- the row-major labeling is strictly worse with value `3`

**Step 2: Run the test to verify RED**

Run:

```bash
cargo test minimum_graph_bandwidth_paper_example --lib
```

Expected: failure until the final example wording and instance are aligned.

**Step 3: Implement the paper entry**

In `docs/paper/reductions.typ`:
- add `"MinimumGraphBandwidth": [Minimum Graph Bandwidth],` to the display-name dictionary
- add a `problem-def("MinimumGraphBandwidth")[...][...]` section near the other graph problems
- use the canonical grid example from the issue/example-db and explain:
  - permutation labeling semantics
  - bandwidth as the maximum edge span
  - why the column-major order achieves `2`
  - why row-major gives `3`
- cite the exact-algorithm and NP-completeness claims with bibliography keys, not prose-only references

In `docs/paper/references.bib`, add any missing bibliography entries needed by the new section, likely including:
- Papadimitriou (1976), *The NP-Completeness of the Bandwidth Minimization Problem*
- Cygan & Pilipczuk (2010), *Exact and approximate bandwidth*
- Chvátalová (1975) only if the text explicitly cites the grid formula

Keep the paper example fully synchronized with the example-db JSON and the unit test.

**Step 4: Run the paper and example verification**

Run:

```bash
cargo test minimum_graph_bandwidth_paper_example --lib
make paper
```

Expected: the paper builds cleanly and the example test passes.

**Step 5: Commit**

```bash
git add docs/paper/reductions.typ docs/paper/references.bib src/unit_tests/models/graph/minimum_graph_bandwidth.rs
git commit -m "Document MinimumGraphBandwidth in the paper"
```

### Task 6: Run final repository verification and clean up

**Files:**
- Modify only if verification exposes real breakage

**Step 1: Run the final verification suite**

Run:

```bash
make test
make clippy
```

If the repo’s paper/export flow changed tracked files that belong in the feature, stage them explicitly. Ignore generated files under ignored paths such as `docs/src/reductions/`.

**Step 2: Inspect the tree**

Run:

```bash
git status --short
```

Expected: only intended tracked files are present.

**Step 3: Commit any last verification-driven fixes**

```bash
git add -A
git commit -m "Polish MinimumGraphBandwidth integration"
```

Only do this if verification required real follow-up edits; otherwise leave the earlier commits as the final stack.
