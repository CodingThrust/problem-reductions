# KClique Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the `KClique` decision model end-to-end so issue #714 can unblock the pending rule issues that target or source Clique as a satisfaction problem.

**Architecture:** Implement `KClique<G>` as a graph-based `SatisfactionProblem` with one binary variable per vertex and a runtime threshold `k: usize`. Reuse the same clique-validity semantics as `MaximumClique`, but expose a boolean metric, graph-only registry variants, and constructor-facing schema/CLI fields `graph` and `k`. Use the issue’s 5-vertex house-graph instance with witness `[0, 0, 1, 1, 1]` and `k = 3` as the canonical example for tests, `pred create --example`, and the paper.

**Tech Stack:** Rust, serde, inventory registry, Clap CLI, MCP create helpers, Typst paper, `make test`, `make clippy`, `make paper`.

---

## Issue Context

- **Issue:** #714 `[Model] KClique`
- **Good label:** present
- **Existing PR:** none (`action = create-pr`)
- **Associated rule issues verified from the issue + open rule search:** #229, #231, #201, #206
- **References to honor:** `@garey1979`, `@karp1972`, `@xiao2017`
- **Important modeling decision:** this is a dedicated decision problem, not a variant of `MaximumClique`, because the metric is `bool` and the instance carries a threshold `k`

## Required Information Checklist

| # | Item | Value |
|---|------|-------|
| 1 | Problem name | `KClique` |
| 2 | Mathematical definition | Given an undirected graph `G = (V, E)` and integer `k`, determine whether there exists `V' ⊆ V` with `|V'| ≥ k` such that every distinct pair in `V'` is adjacent |
| 3 | Problem type | Satisfaction (`Metric = bool`) |
| 4 | Type parameters | `G: Graph` |
| 5 | Struct fields | `graph: G`, `k: usize` |
| 6 | Configuration space | `vec![2; graph.num_vertices()]` |
| 7 | Feasibility check | Selected vertices must all be pairwise adjacent and the selected set size must be at least `k` |
| 8 | Objective function | `bool` only: `true` iff the configuration is a clique witness of size at least `k` |
| 9 | Best known exact algorithm | `O*(1.1996^n)` via complement to MIS / maximum clique, encoded as `"1.1996^num_vertices"` |
| 10 | Solving strategy | Brute force already works through `SatisfactionProblem`; no ILP work in this issue |
| 11 | Category | `src/models/graph/` |
| 12 | Expected outcome | On the issue’s 5-vertex graph with edges `{0,1},{0,2},{1,3},{2,3},{2,4},{3,4}` and `k = 3`, witness `[0, 0, 1, 1, 1]` is satisfying and is the unique satisfying assignment |

## Batch 1: Model, Registry, CLI, Example DB, Tests

### Task 1: Write the model tests first

**Files:**
- Create: `src/unit_tests/models/graph/kclique.rs`

**Step 1: Write the failing tests**

Add focused tests that encode the issue semantics before any implementation exists:
- `test_kclique_creation` for constructor/getters/dims
- `test_kclique_evaluate_yes_instance` using the issue witness
- `test_kclique_evaluate_rejects_non_clique`
- `test_kclique_evaluate_rejects_too_small_clique`
- `test_kclique_solver_finds_unique_witness`
- `test_kclique_serialization_round_trip`
- `test_kclique_paper_example`

Use helper builders in the test file for the issue graph and witness so the same instance is reused across tests.

**Step 2: Run the focused test to verify RED**

Run: `cargo test kclique --lib`

Expected: fail because `KClique` and its test module do not exist yet.

### Task 2: Implement the core `KClique` model

**Files:**
- Create: `src/models/graph/kclique.rs`
- Modify: `src/models/graph/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`

**Step 1: Implement the model file**

Create `src/models/graph/kclique.rs` with:
- `inventory::submit!` schema entry
- `KClique<G>` struct deriving `Debug, Clone, Serialize, Deserialize`
- constructor `new(graph, k)` with a guard that `k <= num_vertices`
- getters `graph()`, `k()`, `num_vertices()`, `num_edges()`
- `is_valid_solution(&self, config: &[usize]) -> bool`
- internal clique-check helper shared by `evaluate`
- `Problem` impl with `NAME = "KClique"`, `Metric = bool`, graph-only `variant()`, binary `dims()`, and boolean `evaluate()`
- `SatisfactionProblem` impl
- `declare_variants! { default sat KClique<SimpleGraph> => "1.1996^num_vertices" }`
- `#[cfg(feature = "example-db")] canonical_model_example_specs()` using the issue instance
- linked test module

Do not add a weight dimension. `k` is instance data, not a variant.

**Step 2: Register exports**

Wire the new model through:
- `src/models/graph/mod.rs`
- `src/models/mod.rs`
- `src/lib.rs` prelude exports

Also add `canonical_model_example_specs()` into the graph example chain in `src/models/graph/mod.rs`.

**Step 3: Run the focused tests to verify GREEN**

Run: `cargo test kclique --lib`

Expected: the new `kclique` model tests pass.

### Task 3: Add CLI and MCP creation support

**Files:**
- Modify: `problemreductions-cli/src/commands/create.rs`
- Modify: `problemreductions-cli/src/cli.rs`
- Modify: `problemreductions-cli/src/mcp/tools.rs`

**Step 1: Add failing create-path tests**

In the existing `problemreductions-cli/src/commands/create.rs` test module, add at least:
- a positive `pred create KClique --graph ... --k 3` test that deserializes to `KClique<SimpleGraph>`
- a negative test that rejects missing `--k` or `k > |V|`

Prefer using the same issue example graph for the positive case.

**Step 2: Run the focused create tests to verify RED**

Run: `cargo test create_kclique --package problemreductions-cli`

Expected: fail because create support does not exist yet.

**Step 3: Implement create support**

Update `problemreductions-cli/src/commands/create.rs` to:
- add `KClique` to the problem help examples table
- accept `KClique` in the graph problem dispatch
- parse `--graph` plus required `--k`
- construct `KClique::new(graph, k)` for normal create
- support random graph creation with `--random --num-vertices ... --k ...`

Update `problemreductions-cli/src/cli.rs` help text so `KClique` appears in the “Flags by problem type” list with `--graph, --k`.

Update `problemreductions-cli/src/mcp/tools.rs` in the mirrored create paths and supported-problem text so the MCP interface can also create `KClique` instances.

**Step 4: Run focused CLI verification**

Run:
- `cargo test create_kclique --package problemreductions-cli`
- `cargo test kclique --package problemreductions-cli`

Expected: the new create-path tests pass.

### Task 4: Verify example-db and registry integration

**Files:**
- No new files beyond the ones above unless a small central regression test proves necessary

**Step 1: Run focused integration checks**

Run:
- `cargo test example_db --lib`
- `cargo test schema --lib`
- `cargo test problem_size --lib`

If one of these suites exposes a missing assertion for `KClique`, add the minimal regression in the existing central test file that failed. Do not preemptively edit unrelated test files unless the failure requires it.

**Step 2: Run batch-1 verification**

Run:
- `cargo test kclique --all-targets`
- `cargo test create_kclique --package problemreductions-cli`
- `cargo test example_db --lib`

Expected: all implementation-batch tests pass before touching the paper.

## Batch 2: Paper Entry and Paper-Example Consistency

### Task 5: Add the Typst paper entry after the model is stable

**Files:**
- Modify: `docs/paper/reductions.typ`

**Step 1: Add the display name**

Register:
- `"KClique": [$k$-Clique],`

near the `display-name` dictionary.

**Step 2: Add the `problem-def("KClique")` entry**

Place it near the other graph problem definitions. The entry should:
- use the example-db fixture via `load-model-example("KClique")`
- introduce `G = (V, E)` and threshold `k`
- explain the decision semantics (`|K| >= k`)
- cite the historical context with `@karp1972` / `@garey1979`
- cite the exact-algorithm bound with `@xiao2017`
- present the issue’s house-graph example and witness `[0,0,1,1,1]`
- explicitly state why the witness is satisfying and why no 4-clique exists in that graph

Avoid adding the chordal-graph claim unless you also add a proper bibliography entry and use it consistently.

**Step 3: Re-run the paper-specific model test**

Run: `cargo test kclique_paper_example --lib`

Expected: pass against the same instance used in the paper entry.

### Task 6: Final verification

**Files:**
- No new files

**Step 1: Run repository verification commands**

Run:
- `cargo test kclique --all-targets`
- `cargo test create_kclique --package problemreductions-cli`
- `make paper`
- `make test`
- `make clippy`

If `make paper` regenerates ignored exports under `docs/src/reductions/`, leave them unstaged.

**Step 2: Inspect git status**

Run: `git status --short`

Expected: only the intended tracked source/doc changes remain.

**Step 3: Implementation summary points for the PR comment**

Capture for the final PR comment:
- model file(s) added and registrations updated
- CLI/MCP create support added
- canonical example + paper entry added
- any deviations from the plan, or `None`
