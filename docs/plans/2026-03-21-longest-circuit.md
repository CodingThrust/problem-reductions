# LongestCircuit Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the `[Model] LongestCircuit` graph satisfaction model from issue `#287`, including registry/example-db/CLI integration and the paper entry needed for the companion rule issue `#358` (`HamiltonianCircuit -> LongestCircuit`).

**Architecture:** Model `LongestCircuit` as an edge-selection satisfaction problem on `SimpleGraph` with a generic edge-length type `W: WeightElement`, matching the repo's existing weighted graph satisfaction models. A configuration is valid iff the selected edges form one connected simple circuit and the selected edge-length sum is at least the positive bound `K`. Batch 1 handles code, tests, example-db, CLI, and MCP integration; Batch 2 adds the paper entry after the canonical example is stable.

**Tech Stack:** Rust workspace (`problemreductions`, `problemreductions-cli`), serde/inventory registry, `BruteForce` solver, Typst paper, GitHub issue workflow.

---

## Source-of-Truth Notes

- Issue: `#287 [Model] LongestCircuit`
- Companion rule already exists as issue `#358 [Rule] HAMILTONIAN CIRCUIT to LONGEST CIRCUIT`, so this model is not orphaned.
- Use the corrected issue comments as design input:
  - Keep the model as the decision version with positive edge lengths and positive bound `K`.
  - Avoid the misleading `1.657^n` Hamiltonicity claim for longest cycle.
  - Use the issue's 6-vertex, 10-edge weighted YES instance as the canonical example/paper figure.
- Canonical implementation choices:
  - Category: `src/models/graph/`
  - Struct: `LongestCircuit<G, W>`
  - Fields: `graph`, `edge_lengths`, `bound`
  - Getter size fields: `num_vertices()`, `num_edges()`
  - Config encoding: one binary variable per edge (`dims() == vec![2; num_edges]`)
  - Validity helper: selected edges induce exactly one connected 2-regular subgraph and total selected length is `>= bound`

## Batch 1: Model, Tests, Example-DB, CLI, MCP

### Task 1: Write the first failing model tests

**Files:**
- Create: `src/unit_tests/models/graph/longest_circuit.rs`
- Read for pattern: `src/unit_tests/models/graph/shortest_weight_constrained_path.rs`
- Read for pattern: `src/unit_tests/models/graph/rural_postman.rs`

**Step 1: Write the failing tests**

Add focused tests for:
- creation/accessors on the issue instance
- `evaluate()` on one valid circuit and a few invalid edge selections
- brute-force satisfiability on the issue YES instance

Use the issue's canonical graph and the expected satisfying circuit `0-1-2-3-4-5-0`.

**Step 2: Run the focused test to verify it fails**

Run: `cargo test longest_circuit --lib`

Expected: FAIL because `LongestCircuit` is not implemented or not exported yet.

**Step 3: Do not add behavior here**

Stop once the failure is the expected missing-model failure.

### Task 2: Implement the core model and register it

**Files:**
- Create: `src/models/graph/longest_circuit.rs`
- Modify: `src/models/graph/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`
- Modify: `src/unit_tests/models/graph/longest_circuit.rs`

**Step 1: Implement the minimal model to satisfy Task 1**

Add `LongestCircuit<G, W: WeightElement>` with:
- `ProblemSchemaEntry` for graph + weight dimensions
- constructor validation:
  - `edge_lengths.len() == graph.num_edges()`
  - all edge lengths positive
  - `bound > 0`
- accessors: `graph()`, `edge_lengths()`, `bound()`, `num_vertices()`, `num_edges()`, `is_weighted()`, optional `set_lengths(...)`
- `Problem` impl with `Metric = bool`
- `SatisfactionProblem` impl
- helper that accepts only one simple circuit:
  - config length matches `num_edges`
  - values are binary
  - selected edge count at least 3
  - each selected vertex has degree exactly 2
  - non-selected vertices have degree 0
  - selected subgraph is connected
  - total selected length `>= bound`
- `declare_variants!` for `LongestCircuit<SimpleGraph, i32>`
- `#[cfg(test)]` link to the new unit test file

Export the new model from the graph module, global model re-export, and prelude.

**Step 2: Run the focused model tests**

Run: `cargo test longest_circuit --lib`

Expected: PASS for the new model tests.

**Step 3: Refactor only if needed**

If the circuit-validation helper is noisy, extract a small private helper for degree/connectivity counting, but keep the representation edge-based for compatibility with issue `#358`.

### Task 3: Add canonical example-db support before CLI work

**Files:**
- Modify: `src/models/graph/longest_circuit.rs`
- Modify: `src/models/graph/mod.rs`
- Read for pattern: `src/models/graph/hamiltonian_circuit.rs`
- Read for pattern: `src/example_db/model_builders.rs`

**Step 1: Add the canonical model example spec**

Inside `src/models/graph/longest_circuit.rs`, add `canonical_model_example_specs()` gated by `example-db` using the issue's YES instance:
- graph edges in issue order
- edge lengths `[3, 2, 4, 1, 5, 2, 3, 2, 1, 2]`
- bound `17`
- optimal/satisfying config selecting the 6-cycle `0-1-2-3-4-5-0`

Register the new example chain in `src/models/graph/mod.rs`.

**Step 2: Run the example-db tests that should cover lookup/build**

Run: `cargo test example_db --features example-db`

Expected: PASS, and the new model example appears in the built example DB.

### Task 4: Add CLI create support and CLI tests

**Files:**
- Modify: `problemreductions-cli/src/commands/create.rs`
- Modify: `problemreductions-cli/src/cli.rs`
- Modify: `problemreductions-cli/tests/cli_tests.rs`
- Optionally modify: `problemreductions-cli/src/problem_name.rs` only if exact-canonical-name resolution is insufficient

**Step 1: Write failing CLI tests first**

Add tests for:
- explicit creation:
  - `pred create LongestCircuit --graph ... --edge-weights ... --bound 17`
- rejection of missing `--edge-weights`
- rejection of non-positive `--bound`
- help text / usage string mentions the right flags
- random creation:
  - `pred create LongestCircuit --random --num-vertices 6 --bound 4`

Use `--edge-weights` as the CLI surface for the single edge-length vector, following `RuralPostman` and other one-vector edge-weight graph problems.

**Step 2: Run the new CLI tests to verify they fail**

Run: `cargo test -p problemreductions-cli longest_circuit`

Expected: FAIL because the create command and help text do not know `LongestCircuit` yet.

**Step 3: Implement the minimal CLI support**

In `problemreductions-cli/src/commands/create.rs`:
- add an explicit create arm for `LongestCircuit`
- parse `graph`, `edge_weights`, and `bound`
- validate positive edge weights and positive bound
- serialize `LongestCircuit::new(graph, edge_weights, bound)`
- add a random-create arm that generates a random graph, unit edge lengths, and a reasonable positive default bound (for example `max(3, num_vertices / 2)`)

In `problemreductions-cli/src/cli.rs`:
- add `LongestCircuit` to the "Flags by problem type" help table
- update any help text mentioning supported `--bound` users if needed

**Step 4: Re-run the focused CLI tests**

Run: `cargo test -p problemreductions-cli longest_circuit`

Expected: PASS.

### Task 5: Add MCP creation support and focused regression checks

**Files:**
- Modify: `problemreductions-cli/src/mcp/tools.rs`
- Read for pattern: existing `MaxCut` / `TravelingSalesman` / `MinimumSumMulticenter` MCP branches

**Step 1: Write or reuse a focused failing regression if there is already MCP coverage**

If an MCP test file already covers graph creation routing, add a failing test there for `LongestCircuit`. If there is no practical focused test harness, document that and use code review plus compile/test verification.

**Step 2: Implement MCP support**

Update the MCP creation helpers so `LongestCircuit` can be created from MCP params in both normal and random flows:
- parse graph params
- parse one edge-weight vector as the circuit lengths
- parse positive `bound`
- serialize with the `SimpleGraph/i32` variant

**Step 3: Run the tightest relevant verification**

Run one of:
- `cargo test -p problemreductions-cli mcp`
- or, if the test names are more granular, the narrowest matching MCP test selection

Expected: PASS.

### Task 6: Add final model-level tests and run Batch 1 verification

**Files:**
- Modify: `src/unit_tests/models/graph/longest_circuit.rs`

**Step 1: Expand the test file to cover the final model contract**

Ensure the test file includes at least:
- creation/accessors
- `evaluate()` valid/invalid circuit selections
- brute-force satisfiable and unsatisfiable cases
- serialization round-trip
- paper/canonical example test using the issue's YES instance

Add at least one invalid case for each structural failure mode:
- disconnected selected cycles
- degree-1 or degree-3 selection
- length below bound

**Step 2: Run focused model tests**

Run: `cargo test longest_circuit --lib`

Expected: PASS.

**Step 3: Run Batch 1 workspace checks**

Run:
- `cargo test longest_circuit`
- `cargo test -p problemreductions-cli longest_circuit`

Expected: PASS for all targeted checks before moving to the paper batch.

## Batch 2: Paper Entry and End-to-End Verification

### Task 7: Add the paper entry and align it with the canonical example

**Files:**
- Modify: `docs/paper/reductions.typ`
- Read for pattern: `problem-def("HamiltonianCircuit")`
- Read for pattern: `problem-def("TravelingSalesman")`

**Step 1: Write the failing paper-adjacent test first**

If the model test file does not already assert the canonical example from the paper, add or tighten `test_longest_circuit_paper_example` before editing the Typst entry so the model-side example is pinned.

Run: `cargo test longest_circuit_paper_example --lib`

Expected: PASS or FAIL only for model/example mismatch. Fix the model/example first, not Typst.

**Step 2: Implement the Typst entry**

Add:
- display-name dictionary entry for `LongestCircuit`
- `problem-def("LongestCircuit")` with:
  - formal definition using positive edge lengths and threshold `K`
  - brief background and corrected algorithm notes
  - a worked example using the canonical YES instance
  - `pred-commands(...)` using the example-db-backed pattern, not a hand-written fragile spec
- a small circuit visualization that highlights the satisfying cycle

Keep the algorithm discussion conservative:
- state the classical exact baseline carefully
- do not repeat the misleading Hamiltonicity-only `1.657^n` claim as the longest-cycle complexity

**Step 3: Run paper verification**

Run: `make paper`

Expected: PASS.

### Task 8: Run full verification, then prepare the implementation summary

**Files:**
- No new files expected

**Step 1: Run the repo verification required before closing the issue-to-pr execution**

Run:
- `make test`
- `make clippy`
- `git status --short`

Expected:
- tests pass
- clippy passes
- only intended tracked files are modified

**Step 2: Prepare commit boundaries**

Create small coherent commits, for example:
- model/tests/example-db integration
- CLI/MCP integration
- paper entry

**Step 3: Record deviations if any**

If implementation differs from this plan, note the difference for the PR summary comment before push.
