# RootedTreeArrangement Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add `RootedTreeArrangement` as a graph-based satisfaction problem with the parent-array plus permutation encoding described in issue `#407`, including CLI/example-db support, tests, and a paper entry.

**Architecture:** Implement the model alongside [`OptimalLinearArrangement`](/Users/jinguomini/rcode/problem-reductions/.worktrees/issue-407/src/models/graph/optimal_linear_arrangement.rs) as a generic graph satisfaction problem over `G: Graph`. The core evaluator will parse a `2n`-variable configuration into a rooted-tree parent array and a bijection `f`, validate the rooted tree and mapping, then check the ancestor/descendant edge constraint and the total stretch bound `K`.

**Tech Stack:** Rust workspace, registry macros (`inventory::submit!`, `declare_variants!`), `pred` CLI, example-db exports, Typst paper, `cargo`/`make` verification.

---

## Issue-Specific Constraints

- Use the issue comment corrections as source of truth:
  - fixed-length encoding is `dims() = vec![n; 2 * n]`
  - expose `num_vertices()` and `num_edges()`
  - keep Example 2 consistent; use it as the canonical worked example
- Associated open rule issue exists: `#424 [Rule] Rooted Tree Arrangement to Rooted Tree Storage Assignment`, so this model is not an orphan.
- Reuse existing bibliography key `gareyJohnsonStockmeyer1976`; add missing keys for Gavril (1977) and Adolphson-Hu (1973) if they are not already in `docs/paper/references.bib`.

## Batch 1: add-model Steps 1-5.5

### Task 1: Add failing model tests first

**Files:**
- Create: `src/unit_tests/models/graph/rooted_tree_arrangement.rs`
- Reference: `src/unit_tests/models/graph/optimal_linear_arrangement.rs`
- Reference: `src/models/graph/optimal_linear_arrangement.rs`

**Step 1: Write the failing test file**

Add targeted tests for:
- basic getters and `dims()` on a 5-vertex issue example
- a valid YES witness using the chain tree plus identity mapping from issue Example 2
- rejection of invalid parent arrays (multiple roots / directed cycle)
- rejection of invalid bijections (duplicate or out-of-range image)
- rejection when an edge endpoint pair is not ancestor-comparable
- rejection when the total tree distance exceeds `bound`
- serde round-trip and brute-force sanity on a tiny instance

Use the issue encoding directly in the tests:
- parent array occupies `config[..n]`
- bijection occupies `config[n..]`

**Step 2: Run the targeted test to verify RED**

Run: `cargo test rooted_tree_arrangement --lib`

Expected: FAIL because `RootedTreeArrangement` does not exist yet.

**Step 3: Commit the red test scaffold**

```bash
git add src/unit_tests/models/graph/rooted_tree_arrangement.rs
git commit -m "test: add RootedTreeArrangement coverage"
```

### Task 2: Implement the model core

**Files:**
- Create: `src/models/graph/rooted_tree_arrangement.rs`
- Modify: `src/models/graph/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`

**Step 1: Implement the new graph model**

Follow repo-local `add-model` Steps 1-3 in this file:
- add `ProblemSchemaEntry` with `graph` dimension only
- define `RootedTreeArrangement<G>` with fields `graph: G` and `bound: usize`
- implement getters `graph()`, `bound()`, `num_vertices()`, `num_edges()`
- set `dims()` to `vec![n; 2 * n]`
- implement helpers to:
  - validate exactly one root in the parent array
  - reject self-parenting outside the root
  - reject cycles / disconnected parent forests
  - compute depth of every tree node
  - validate the bijection as a permutation of `0..n`
  - test whether two mapped vertices are ancestor-comparable
  - sum tree distances for graph edges
- expose `is_valid_solution()` and a stretch helper that returns `Option<usize>`
- implement `Problem` and `SatisfactionProblem`
- declare the default variant as `sat RootedTreeArrangement<SimpleGraph> => "2^num_vertices"`
- add `canonical_model_example_specs()` using the cleaned-up Example 2 instance from the issue
- link the new unit test file at the bottom of the module

**Step 2: Register exports**

Update:
- `src/models/graph/mod.rs`
- `src/models/mod.rs`
- `src/lib.rs`

Add the module declaration, public re-export, prelude export, and graph example-db registration chain entry.

**Step 3: Run the targeted tests to verify GREEN**

Run:
- `cargo test rooted_tree_arrangement --lib`

Expected: PASS for the new model tests.

**Step 4: Refactor lightly if needed**

Keep helpers local to the model file unless duplication appears.

**Step 5: Commit**

```bash
git add src/models/graph/rooted_tree_arrangement.rs src/models/graph/mod.rs src/models/mod.rs src/lib.rs
git commit -m "feat: add RootedTreeArrangement model"
```

### Task 3: Add CLI discovery and creation support

**Files:**
- Modify: `problemreductions-cli/src/problem_name.rs`
- Modify: `problemreductions-cli/src/commands/create.rs`
- Modify: `problemreductions-cli/src/cli.rs`

**Step 1: Extend CLI discovery**

In `problem_name.rs`, ensure the catalog alias path is sufficient; only add a manual alias if the registry alias is absent and the abbreviation is literature-backed.

**Step 2: Extend create command support**

In `create.rs`:
- add example/help strings for `RootedTreeArrangement`
- add the direct `pred create RootedTreeArrangement --graph ... --bound ...` match arm
- add the random graph generator arm mirroring `OptimalLinearArrangement`, defaulting `bound` to a safe satisfiable upper bound if omitted

**Step 3: Update CLI help text**

In `cli.rs`:
- add `RootedTreeArrangement` to the “Flags by problem type” table
- extend the `--bound` doc comment if the problem name list is maintained manually there

**Step 4: Verify**

Run:
- `cargo test -p problemreductions-cli create:: -- --nocapture`

If the CLI test filter is too broad or absent, run a targeted `cargo test -p problemreductions-cli rooted_tree_arrangement`.

**Step 5: Commit**

```bash
git add problemreductions-cli/src/problem_name.rs problemreductions-cli/src/commands/create.rs problemreductions-cli/src/cli.rs
git commit -m "feat: add RootedTreeArrangement CLI support"
```

### Task 4: Wire the canonical example and example-db coverage

**Files:**
- Modify: `src/models/graph/rooted_tree_arrangement.rs`
- Reference: `src/example_db/model_builders.rs`
- Reference: `src/unit_tests/example_db.rs`

**Step 1: Confirm example-db wiring**

Ensure the `canonical_model_example_specs()` entry added in Task 2 is reachable through `src/models/graph/mod.rs` so `src/example_db/model_builders.rs` picks it up automatically.

**Step 2: Add or extend tests if needed**

If the model tests do not already exercise the canonical example config, add one focused assertion that the example-db config evaluates to `true`.

**Step 3: Verify**

Run:
- `cargo test example_db`
- `cargo test rooted_tree_arrangement --features example-db`

**Step 4: Commit**

```bash
git add src/models/graph/rooted_tree_arrangement.rs src/models/graph/mod.rs
git commit -m "test: cover RootedTreeArrangement example-db wiring"
```

## Batch 2: add-model Step 6

### Task 5: Document the model in the paper

**Files:**
- Modify: `docs/paper/reductions.typ`
- Modify: `docs/paper/references.bib`

**Step 1: Add bibliography entries if missing**

Add citation keys for:
- Gavril (1977), the GT45 NP-completeness reference
- Adolphson and Hu (1973), the tree linear arrangement algorithm reference

Reuse existing `gareyJohnsonStockmeyer1976` for the `OptimalLinearArrangement` connection.

**Step 2: Add display-name and `problem-def`**

In `reductions.typ`:
- add `"RootedTreeArrangement": [Rooted Tree Arrangement],` to the display-name dictionary
- add a `problem-def("RootedTreeArrangement")` section near `OptimalLinearArrangement`
- explain the decision formulation, the rooted-tree witness, and the ancestor-path constraint
- use the canonical Example 2 instance from the model example export
- include a `pred-commands(...)` block using `pred create --example RootedTreeArrangement`

**Step 3: Build the paper**

Run:
- `make paper`

Expected: PASS with no Typst errors.

**Step 4: Commit**

```bash
git add docs/paper/reductions.typ docs/paper/references.bib
git commit -m "docs: document RootedTreeArrangement"
```

## Final Verification: add-model Step 7

### Task 6: Run focused and repo-level verification

**Files:**
- No code changes expected

**Step 1: Focused verification**

Run:
- `cargo test rooted_tree_arrangement`
- `cargo test optimal_linear_arrangement`
- `cargo test example_db --features example-db`

**Step 2: Project verification**

Run:
- `make fmt`
- `make check`

If `make check` is too slow or exposes unrelated failures, capture the exact failing command and stop rather than masking it.

**Step 3: Review git state**

Run:
- `git status --short`

Expected: clean working tree except for intentionally ignored/generated files.

**Step 4: Commit any remaining implementation work**

```bash
git add -A
git commit -m "Implement #407: add RootedTreeArrangement"
```
