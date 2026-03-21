# MixedChinesePostman Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the `MixedChinesePostman` model, including a new `MixedGraph` topology, exact orientation-based evaluation, CLI/example-db integration, and paper documentation for issue #242.

**Architecture:** Represent a mixed graph as directed arcs plus undirected edges in a reusable topology type. Model `MixedChinesePostman<W>` as a satisfaction problem with one binary variable per undirected edge, orient each edge according to the config, reject orientations that do not yield a strongly connected directed graph, then compute the minimum extra traversal cost needed to balance the directed graph by combining all-pairs shortest paths with a minimum-cost assignment over degree imbalances.

**Tech Stack:** Rust, petgraph, serde, inventory, existing `BruteForce` solver, existing CLI create/example pipeline, Typst paper exports

---

## Skill Alignment

- **Primary repo skill:** `.claude/skills/add-model/SKILL.md`
- **Execution mode:** `superpowers:subagent-driven-development`
- **Testing discipline:** `superpowers:test-driven-development`

## Issue Packet Summary

- **Issue:** #242 — `[Model] MixedChinesePostman`
- **Kind:** model
- **Labels:** `model`, `Good`
- **Associated rule:** #260 — `[Rule] 3-SATISFIABILITY to CHINESE POSTMAN FOR MIXED GRAPHS`
- **Approved design corrections from comments:**
  - Rename to `MixedChinesePostman`
  - Use `|E|` binary orientation variables for undirected edges
  - Add a reusable `MixedGraph` topology
  - Expose size getters `num_vertices`, `num_arcs`, `num_edges`, `bound`
  - Use complexity metadata `2^num_edges * num_vertices^3`
  - Use the fixed YES/NO examples from the issue comments as the source of truth

## Information Checklist

| # | Item | Value |
|---|------|-------|
| 1 | Problem name | `MixedChinesePostman` |
| 2 | Mathematical definition | Given a mixed graph `G = (V, A, E)` with nonnegative lengths on arcs and undirected edges and a bound `B`, determine whether there exists a closed walk of total length at most `B` that traverses every directed arc in its given direction and every undirected edge in at least one direction |
| 3 | Problem type | Satisfaction (`Metric = bool`) |
| 4 | Type parameters | Weight parameter `W` only; topology is a concrete `MixedGraph` |
| 5 | Struct fields | `graph: MixedGraph`, `arc_weights: Vec<W>`, `edge_weights: Vec<W>`, `bound: W::Sum` |
| 6 | Configuration space | `vec![2; num_edges]` — one binary orientation choice per undirected edge |
| 7 | Feasibility check | Orient each undirected edge, require the resulting digraph to be strongly connected, compute the cheapest added traversal cost needed to balance in/out degrees, and accept iff base cost + added cost `<= bound` |
| 8 | Objective function | `bool` |
| 9 | Best known exact algorithm | Brute-force over all edge orientations with polynomial subproblem solve; metadata string: `"2^num_edges * num_vertices^3"` |
| 10 | Solving strategy | `BruteForce` over orientations; `evaluate()` solves the oriented directed-postman subproblem exactly via shortest-path costs plus minimum-cost assignment over imbalances |
| 11 | Category | `graph` |
| 12 | Expected outcome | Use the corrected YES/NO examples from issue #242 comments, including the nontrivial YES example requiring duplicated traversals |

## Batch Structure

- **Batch 1:** Steps 1-5.5 from `add-model` — topology, model, registration, CLI, example-db, tests
- **Batch 2:** Step 6 from `add-model` — paper entry and paper/example alignment

## Batch 1

### Task 1: Add failing topology tests for `MixedGraph`

**Files:**
- Create: `src/unit_tests/topology/mixed_graph.rs`
- Modify: `src/topology/mod.rs`

**Steps:**
1. Write tests that define the expected `MixedGraph` surface:
   - constructor preserves `num_vertices`, arcs, and undirected edges
   - accessors report `num_arcs`, `num_edges`, `arcs()`, `edges()`
   - adjacency helpers and degree helpers behave correctly for both arc and edge views
   - serde round-trip preserves the mixed structure
2. Run the focused tests and verify they fail because `MixedGraph` does not exist yet:
   - `cargo test mixed_graph --lib`

### Task 2: Implement `MixedGraph` and register it as a topology

**Files:**
- Create: `src/topology/mixed_graph.rs`
- Modify: `src/topology/mod.rs`
- Modify: `src/lib.rs`

**Steps:**
1. Implement `MixedGraph` with:
   - `num_vertices`
   - directed `arcs: Vec<(usize, usize)>`
   - undirected `edges: Vec<(usize, usize)>`
   - constructor validation for vertex bounds
   - accessors for counts and edge lists
   - helpers needed by the model (`num_vertices`, `num_arcs`, `num_edges`)
2. Add serde support and `impl_variant_param!(MixedGraph, "graph")`.
3. Export the new topology from `src/topology/mod.rs` and the public prelude in `src/lib.rs`.
4. Re-run the focused topology tests until green:
   - `cargo test mixed_graph --lib`

### Task 3: Add failing model tests for `MixedChinesePostman`

**Files:**
- Create: `src/unit_tests/models/graph/mixed_chinese_postman.rs`
- Reference: `src/unit_tests/models/graph/directed_two_commodity_integral_flow.rs`
- Reference: `src/unit_tests/models/graph/strong_connectivity_augmentation.rs`

**Steps:**
1. Write tests for:
   - construction, dimensions, and accessors
   - corrected YES issue example evaluates to `true`
   - corrected NO issue example evaluates to `false`
   - a disconnected or not-strongly-connected orientation evaluates to `false`
   - serialization round-trip
   - `BruteForce::find_satisfying()` finds a witness for the YES instance
   - paper/example invariant test placeholder using the issue example
2. Run the focused model tests and verify they fail because the model is missing:
   - `cargo test mixed_chinese_postman --lib`

### Task 4: Implement `MixedChinesePostman` with exact oriented evaluation

**Files:**
- Create: `src/models/graph/mixed_chinese_postman.rs`
- Modify: `src/models/graph/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`

**Steps:**
1. Add `ProblemSchemaEntry` metadata with display name, aliases (if any), fields, and dimensions.
2. Implement the struct and constructor validation:
   - `arc_weights.len() == graph.num_arcs()`
   - `edge_weights.len() == graph.num_edges()`
3. Add size/accessor methods:
   - `num_vertices()`
   - `num_arcs()`
   - `num_edges()`
   - `bound()`
   - `graph()`, `arc_weights()`, `edge_weights()`
4. Implement `Problem` / `SatisfactionProblem`:
   - `NAME = "MixedChinesePostman"`
   - `dims() = vec![2; num_edges]`
   - `variant() = crate::variant_params![W]`
5. Implement `evaluate()` in small, testable helpers:
   - orient each undirected edge according to the config
   - compute the oriented base cost
   - reject if the resulting directed graph is not strongly connected
   - compute all-pairs shortest path distances on the weighted oriented digraph
   - derive vertex imbalances `out_degree - in_degree`
   - solve the balancing transportation problem exactly with minimum-cost assignment over expanded imbalance copies
   - reject if any required shortest path is unreachable
   - accept iff `base_cost + balancing_cost <= bound`
6. Register concrete variants with `declare_variants!`:
   - `MixedChinesePostman<i32>`
   - default `MixedChinesePostman<One>` if unit weights are supported cleanly by the implementation
   - use complexity metadata string `"2^num_edges * num_vertices^3"`
7. Link the new unit test file and export the model from the graph modules and prelude.
8. Re-run the focused model tests until green:
   - `cargo test mixed_chinese_postman --lib`

### Task 5: Add canonical example-db coverage

**Files:**
- Modify: `src/models/graph/mixed_chinese_postman.rs`
- Modify: `src/models/graph/mod.rs`
- Modify: `src/example_db/model_builders.rs`

**Steps:**
1. Add `canonical_model_example_specs()` in the model file.
2. Use the corrected YES example from issue #242 comments as the canonical example.
3. Register the spec in the graph example chain so `pred create --example MixedChinesePostman/...` works.
4. Add or update any example-db assertions needed by existing tests.
5. Run focused example-db coverage if available:
   - `cargo test example_db --lib --features example-db`

### Task 6: Add CLI discovery and create support

**Files:**
- Modify: `problemreductions-cli/src/problem_name.rs`
- Modify: `problemreductions-cli/src/commands/create.rs`
- Modify: `problemreductions-cli/src/cli.rs`
- Modify: `problemreductions-cli/tests/cli_tests.rs`

**Steps:**
1. Add alias resolution for `MixedChinesePostman`.
2. Wire `pred create MixedChinesePostman` using:
   - `--arcs` for directed arcs
   - `--graph` for undirected edges
   - `--arc-costs` for arc lengths
   - `--weights` or `--edge-weights` for undirected edge lengths, depending on the existing CLI convention that best fits the repo
   - `--bound`
   - optional `--num-vertices`
3. Update help text and `all_data_flags_empty()` if a new flag is required for edge weights.
4. Add CLI tests for:
   - successful creation
   - missing required flags
   - weight-count mismatch errors
   - `pred show` / alias resolution if needed
5. Run the focused CLI tests and verify the new path:
   - `cargo test -p problemreductions-cli mixed_chinese_postman`

### Task 7: Run integration verification for Batch 1

**Files:**
- No code changes expected

**Steps:**
1. Run the targeted Rust checks:
   - `cargo test mixed_graph --lib`
   - `cargo test mixed_chinese_postman --lib`
   - `cargo test -p problemreductions-cli mixed_chinese_postman`
2. Run broader checks once the targeted suite is green:
   - `make test`
   - `make clippy`
3. If any failures expose missing registration or schema issues, fix them before moving to Batch 2.

## Batch 2

### Task 8: Add paper entry for `MixedChinesePostman`

**Files:**
- Modify: `docs/paper/reductions.typ`

**Steps:**
1. Add the display-name dictionary entry for `MixedChinesePostman`.
2. Add a `problem-def("MixedChinesePostman")` entry with:
   - formal mixed-graph definition
   - background and references
   - best-known algorithm prose with citations
   - a worked example aligned with the canonical YES instance
   - `pred-commands()` wired to the canonical example data
3. Keep the paper example consistent with the model test and example-db fixture.
4. Run:
   - `make paper`

### Task 9: Final verification and implementation handoff

**Files:**
- No code changes expected

**Steps:**
1. Re-run the full verification commands fresh:
   - `make test`
   - `make clippy`
   - `make fmt-check`
   - `make paper`
2. Inspect `git status --short` and confirm only intentional tracked changes remain.
3. Summarize any deviations from this plan before the implementation summary comment is posted on the PR.
