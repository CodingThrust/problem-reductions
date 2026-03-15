# StrongConnectivityAugmentation Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the `StrongConnectivityAugmentation` model from issue #233 as a directed-graph satisfaction problem, with registry/example-db/CLI integration, paper documentation, and verification.

**Architecture:** Reuse the repo's existing `DirectedGraph` wrapper for the base digraph, store augmentable weighted arcs explicitly, and treat each binary variable as "add this candidate arc". Evaluation should accept exactly those configurations whose selected candidate arcs stay within the bound and make the augmented digraph strongly connected. Keep the paper/example writing in a separate batch after the model, tests, exports, and fixtures are complete.

**Tech Stack:** Rust workspace, `inventory` schema registry, `declare_variants!`, `DirectedGraph`, `BruteForce`, `pred create`, example-db fixtures, Typst paper, `make` verification targets.

---

## Context And Required Decisions

- Issue: `#233` (`[Model] StrongConnectivityAugmentation`)
- Associated rule issue exists: `#254` (`[Rule] HAMILTONIAN CIRCUIT to STRONG CONNECTIVITY AUGMENTATION`)
- Preflight guard already passed: `Good` label present
- Use the issue's 6-vertex directed example as the canonical paper/example-db instance
- Deliberate design deviation from the issue comment: use the repo-standard `DirectedGraph` wrapper instead of exposing raw `petgraph::DiGraph` in the public model schema
- CLI shape for this model:
  - Base digraph: `--arcs "u>v,..."`
  - Candidate weighted augmenting arcs: `--candidate-arcs "u>v:w,..."`
  - Budget: reuse `--bound`
- Complexity registration: `default sat StrongConnectivityAugmentation<i32> => "2^num_potential_arcs"`

## Batch Structure

- **Batch 1:** Tasks 1-4
  - Implement topology primitive, model, registrations, fixtures, CLI/tests
- **Batch 2:** Task 5
  - Paper entry only, after Batch 1 outputs exist
- **Batch 3:** Task 6
  - Final verification, review, and cleanup checks

### Task 1: Add Directed Strong-Connectivity Primitive

**Files:**
- Modify: `src/topology/directed_graph.rs`
- Test: `src/unit_tests/topology/directed_graph.rs`

**Step 1: Write the failing topology tests**

Add focused tests in `src/unit_tests/topology/directed_graph.rs` for:
- a directed 3-cycle is strongly connected
- a one-way path is not strongly connected
- a single-vertex digraph is strongly connected
- an empty digraph is strongly connected (vacuous case)

Use test names like:
- `test_is_strongly_connected_cycle`
- `test_is_strongly_connected_path`
- `test_is_strongly_connected_single_vertex`
- `test_is_strongly_connected_empty`

**Step 2: Run the topology tests to verify RED**

Run:
```bash
cargo test --features "ilp-highs example-db" test_is_strongly_connected --lib
```

Expected:
- FAIL because `DirectedGraph::is_strongly_connected()` does not exist yet

**Step 3: Write the minimal implementation**

In `src/topology/directed_graph.rs`:
- add `pub fn is_strongly_connected(&self) -> bool`
- treat `0` and `1` vertices as strongly connected
- implement with two traversals from vertex `0`:
  - forward traversal via `successors()`
  - reverse traversal via `predecessors()`
- return `true` only if both traversals visit every vertex

Do not introduce a new graph wrapper or expose `inner`.

**Step 4: Run the topology tests to verify GREEN**

Run:
```bash
cargo test --features "ilp-highs example-db" test_is_strongly_connected --lib
```

Expected:
- PASS

**Step 5: Commit**

```bash
git add src/topology/directed_graph.rs src/unit_tests/topology/directed_graph.rs
git commit -m "feat: add directed strong connectivity check"
```

### Task 2: Implement The StrongConnectivityAugmentation Model

**Files:**
- Create: `src/models/graph/strong_connectivity_augmentation.rs`
- Test: `src/unit_tests/models/graph/strong_connectivity_augmentation.rs`

**Step 1: Write the failing model tests**

Create `src/unit_tests/models/graph/strong_connectivity_augmentation.rs` with tests covering:
- creation/dims/getters
- valid issue example (`bound = 1`, exactly the `(5,2,1)` candidate chosen)
- invalid issue example (`bound = 0` or all-zero config)
- wrong-length config returns `false`
- already-strongly-connected base graph accepts the all-zero config
- serialization round-trip
- brute-force solver returns one satisfying configuration for the canonical example
- `variant()` matches `[("weight", "i32")]`
- paper-example parity test using the exact canonical instance

Use the issue's candidate-arc order verbatim so the witness config is stable:
```text
(3,0,5), (3,1,3), (3,2,4),
(4,0,6), (4,1,2), (4,2,7),
(5,0,4), (5,1,3), (5,2,1),
(0,3,8), (0,4,3), (0,5,2),
(1,3,6), (1,4,4), (1,5,5),
(2,4,3), (2,5,7), (1,0,2)
```

Witness config for the YES instance:
```text
[0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0]
```

**Step 2: Run the new model tests to verify RED**

Run:
```bash
cargo test --features "ilp-highs example-db" strong_connectivity_augmentation --lib
```

Expected:
- FAIL because the model file and type do not exist yet

**Step 3: Write the minimal model implementation**

Create `src/models/graph/strong_connectivity_augmentation.rs` with:
- `inventory::submit!` schema entry
- `#[derive(Debug, Clone, Serialize, Deserialize)]`
- public struct:
  - `graph: DirectedGraph`
  - `candidate_arcs: Vec<(usize, usize, W)>`
  - `bound: W`
- constructor validations:
  - every candidate arc endpoint is in range
  - no candidate arc duplicates an existing graph arc
  - candidate arc pairs are unique
- getters/helpers:
  - `graph()`
  - `candidate_arcs()`
  - `bound()`
  - `num_vertices()`
  - `num_arcs()`
  - `num_potential_arcs()`
  - `is_weighted()`
  - `is_valid_solution()`
- `Problem` impl:
  - `NAME = "StrongConnectivityAugmentation"`
  - `Metric = bool`
  - `variant() = crate::variant_params![W]`
  - `dims() = vec![2; self.candidate_arcs.len()]`
  - `evaluate()`:
    - reject wrong-length configs
    - sum selected candidate-arc weights and require `<= self.bound.to_sum()`
    - build the augmented arc list from base arcs plus selected candidates
    - return whether the augmented digraph is strongly connected
- `impl SatisfactionProblem`
- `declare_variants!` with the default `i32` variant
- `#[cfg(feature = "example-db")] canonical_model_example_specs()` using the issue example and `satisfaction_example(...)`
- test module link at the bottom

Keep the base graph type as `DirectedGraph`; do not use raw `DiGraph` in the public schema.

**Step 4: Run the model tests to verify GREEN**

Run:
```bash
cargo test --features "ilp-highs example-db" strong_connectivity_augmentation --lib
```

Expected:
- PASS

**Step 5: Commit**

```bash
git add src/models/graph/strong_connectivity_augmentation.rs src/unit_tests/models/graph/strong_connectivity_augmentation.rs
git commit -m "feat: add strong connectivity augmentation model"
```

### Task 3: Register The Model, Example DB, And Trait Checks

**Files:**
- Modify: `src/models/graph/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`
- Modify: `src/unit_tests/trait_consistency.rs`
- Modify: `src/unit_tests/example_db.rs`
- Modify: `src/example_db/fixtures/examples.json`

**Step 1: Write or extend failing registration checks**

Add or extend assertions so the new model is exercised by existing infrastructure:
- in `src/unit_tests/trait_consistency.rs`, add a `check_problem_trait(...)` case using a tiny directed example
- in `src/unit_tests/example_db.rs`, add a `find_model_example(...)` test for `StrongConnectivityAugmentation/i32`

**Step 2: Run the focused registration tests to verify RED**

Run:
```bash
cargo test --features "ilp-highs example-db" trait_consistency
cargo test --features "ilp-highs example-db" test_find_model_example_strong_connectivity_augmentation
```

Expected:
- FAIL because the model is not exported/registered everywhere yet

**Step 3: Register the model and canonical example**

Update:
- `src/models/graph/mod.rs`
  - add module/export entries
  - append `strong_connectivity_augmentation::canonical_model_example_specs()`
- `src/models/mod.rs`
  - re-export `StrongConnectivityAugmentation`
- `src/lib.rs`
  - add to `prelude`
- `src/unit_tests/trait_consistency.rs`
  - add the new satisfaction problem instance
- `src/unit_tests/example_db.rs`
  - add a lookup/assertion for the canonical example

**Step 4: Regenerate fixtures**

Run:
```bash
make regenerate-fixtures
```

Expected:
- `src/example_db/fixtures/examples.json` updates to include the new canonical model example

**Step 5: Run the focused registration tests to verify GREEN**

Run:
```bash
cargo test --features "ilp-highs example-db" trait_consistency
cargo test --features "ilp-highs example-db" test_find_model_example_strong_connectivity_augmentation
```

Expected:
- PASS

**Step 6: Commit**

```bash
git add src/models/graph/mod.rs src/models/mod.rs src/lib.rs src/unit_tests/trait_consistency.rs src/unit_tests/example_db.rs src/example_db/fixtures/examples.json
git commit -m "feat: register strong connectivity augmentation"
```

### Task 4: Add `pred create` Support And CLI Smoke Coverage

**Files:**
- Modify: `problemreductions-cli/src/cli.rs`
- Modify: `problemreductions-cli/src/commands/create.rs`
- Test: `problemreductions-cli/tests/cli_tests.rs`

**Step 1: Write the failing CLI smoke test**

Add a new test to `problemreductions-cli/tests/cli_tests.rs` that runs something like:
```bash
pred create StrongConnectivityAugmentation \
  --arcs "0>1,1>2,2>0,3>4,4>3,2>3,4>5,5>3" \
  --candidate-arcs "3>0:5,3>1:3,3>2:4,4>0:6,4>1:2,4>2:7,5>0:4,5>1:3,5>2:1,0>3:8,0>4:3,0>5:2,1>3:6,1>4:4,1>5:5,2>4:3,2>5:7,1>0:2" \
  --bound 1
```

Assert that:
- the command succeeds
- the JSON `type` is `StrongConnectivityAugmentation`
- the emitted data contains `graph`, `candidate_arcs`, and `bound`

**Step 2: Run the CLI test to verify RED**

Run:
```bash
cargo test -p problemreductions-cli test_create_problem_strong_connectivity_augmentation
```

Expected:
- FAIL because the new CLI flag/parser path does not exist yet

**Step 3: Implement CLI support**

In `problemreductions-cli/src/cli.rs`:
- add `candidate_arcs: Option<String>` to `CreateArgs`
- include it in `all_data_flags_empty()`
- update the `Flags by problem type` help block

In `problemreductions-cli/src/commands/create.rs`:
- add `type_format_hint()` support for the candidate-arc field format
- add `example_for("StrongConnectivityAugmentation", ...)`
- add a parser for `--candidate-arcs "u>v:w,..."`
- add a new `create()` match arm for `StrongConnectivityAugmentation`
- reuse `parse_directed_graph()` for the base graph and `--bound` for the budget

Do not add a short alias unless there is a literature-standard abbreviation.

**Step 4: Run the CLI test to verify GREEN**

Run:
```bash
cargo test -p problemreductions-cli test_create_problem_strong_connectivity_augmentation
```

Expected:
- PASS

**Step 5: Commit**

```bash
git add problemreductions-cli/src/cli.rs problemreductions-cli/src/commands/create.rs problemreductions-cli/tests/cli_tests.rs
git commit -m "feat: add CLI support for strong connectivity augmentation"
```

### Task 5: Document The Model In The Paper

**Files:**
- Modify: `docs/paper/reductions.typ`

**Step 1: Write the paper entry using the canonical exported example**

Add:
- `"StrongConnectivityAugmentation": [Strong Connectivity Augmentation],` to the `display-name` dictionary
- a new `#problem-def("StrongConnectivityAugmentation")[...][...]` block

Use the exported canonical example, not a hardcoded duplicate. The body should include:
- formal decision version with bound `B`
- short historical/application background citing Eswaran-Tarjan and Garey-Johnson
- brute-force complexity statement `2^num_potential_arcs` plus a note that no stronger exact bound is being claimed here
- the 6-vertex worked example with the single selected augmenting arc `(5,2)`
- a directed-graph figure in the style of `MinimumFeedbackVertexSet`

**Step 2: Run the paper build to verify RED/GREEN as needed**

Run:
```bash
make paper
```

Expected:
- initially FAIL until the display-name/problem-def entry is complete
- finally PASS once the paper entry is correct

**Step 3: Commit**

```bash
git add docs/paper/reductions.typ
git commit -m "docs: add strong connectivity augmentation to paper"
```

### Task 6: Final Verification And Review

**Files:**
- No new files expected; fix anything verification or review finds

**Step 1: Run focused model and CLI checks**

Run:
```bash
cargo test --features "ilp-highs example-db" strong_connectivity_augmentation
cargo test -p problemreductions-cli test_create_problem_strong_connectivity_augmentation
```

Expected:
- PASS

**Step 2: Run repo verification**

Run:
```bash
make test
make clippy
```

Expected:
- PASS

**Step 3: Run implementation review**

Use the repo-local review skill directly:
```text
.claude/skills/review-implementation/SKILL.md
```

Auto-fix any actionable findings before moving on.

**Step 4: If review fixes were needed, rerun verification**

Run:
```bash
make test
make clippy
```

Expected:
- PASS

**Step 5: Commit review-driven fixes if needed**

```bash
git add -A
git commit -m "fix: address strong connectivity augmentation review findings"
```

Only make this commit if review or verification required additional changes.
