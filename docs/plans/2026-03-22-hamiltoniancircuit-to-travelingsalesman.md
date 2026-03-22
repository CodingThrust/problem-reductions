# HamiltonianCircuit to TravelingSalesman Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement issue #258 by adding the primitive reduction `HamiltonianCircuit<SimpleGraph> -> TravelingSalesman<SimpleGraph, i32>`, plus tests, canonical example data, paper documentation, and regenerated exports/fixtures.

**Architecture:** The reduction should build the complete graph on the same vertex set, assign weight `1` to source edges and weight `2` to non-edges, and rely on the existing optimization-style `TravelingSalesman` model. `ReductionResult::extract_solution()` must turn the selected TSP cycle edges back into a Hamiltonian-circuit vertex permutation by traversing the unique degree-2 cycle. Use a small 4-cycle source instance for the canonical rule example and paper walkthrough so the transformed complete graph stays readable.

**Tech Stack:** Rust workspace, `#[reduction]` registry macro, `BruteForce`, `example-db`, Typst paper, export helpers.

---

## Information Checklist

| # | Item | Value |
|---|------|-------|
| 1 | Source problem | `HamiltonianCircuit<SimpleGraph>` |
| 2 | Target problem | `TravelingSalesman<SimpleGraph, i32>` |
| 3 | Reduction algorithm | Build `K_n` on the same vertices; weight each edge `1` if it exists in the source graph and `2` otherwise |
| 4 | Solution extraction | Traverse the selected TSP cycle and emit a Hamiltonian-circuit permutation in source vertex IDs |
| 5 | Correctness argument | TSP uses exactly `n` edges; total weight `n` is achievable iff every selected edge has weight `1`, i.e. iff the tour is a Hamiltonian circuit in the source graph |
| 6 | Size overhead | `num_vertices = "num_vertices"`, `num_edges = "num_vertices * (num_vertices - 1) / 2"` |
| 7 | Concrete example | 4-cycle `(0,1),(1,2),(2,3),(3,0)` mapped to weighted `K_4` with cycle edges weight `1` and diagonals weight `2` |
| 8 | Solving strategy | `BruteForce::find_all_best()` on the target TSP instance |
| 9 | Reference | Garey & Johnson ND22; the issue comments also confirm the sat-to-opt framing already expected by this codebase |

## Batch 1: Rule, Tests, Example Data

### Task 1: Lock the target behavior with focused rule tests

**Files:**
- Create: `src/unit_tests/rules/hamiltoniancircuit_travelingsalesman.rs`
- Modify: `src/rules/mod.rs`
- Reference: `src/unit_tests/rules/minimumvertexcover_maximumindependentset.rs`
- Reference: `src/rules/test_helpers.rs`

**Step 1: Write the failing tests**

Add tests that cover:
- `test_hamiltoniancircuit_to_travelingsalesman_closed_loop` on a YES instance using `assert_satisfaction_round_trip_from_optimization_target`
- `test_hamiltoniancircuit_to_travelingsalesman_structure` to verify the target graph is complete and weights are `1/2` according to source-edge membership
- `test_hamiltoniancircuit_to_travelingsalesman_nonhamiltonian_cost_gap` on the Petersen graph, asserting the optimal TSP value is strictly greater than `num_vertices`
- `test_hamiltoniancircuit_to_travelingsalesman_extract_solution_cycle` to verify `extract_solution()` returns a valid Hamiltonian-circuit permutation for a known target witness

**Step 2: Wire the module so the test can compile once the rule exists**

Add `pub(crate) mod hamiltoniancircuit_travelingsalesman;` to `src/rules/mod.rs`.

**Step 3: Run the focused test to verify it fails**

Run: `cargo test hamiltoniancircuit_to_travelingsalesman --lib`

Expected: compile failure or unresolved `ReduceTo<TravelingSalesman<SimpleGraph, i32>> for HamiltonianCircuit<SimpleGraph>` until the rule file is implemented.

### Task 2: Implement the primitive reduction

**Files:**
- Create: `src/rules/hamiltoniancircuit_travelingsalesman.rs`
- Modify: `src/rules/mod.rs`
- Reference: `src/models/graph/hamiltonian_circuit.rs`
- Reference: `src/models/graph/traveling_salesman.rs`
- Reference: `src/rules/sat_maximumindependentset.rs`

**Step 1: Add the reduction result type**

Define a `ReductionHamiltonianCircuitToTravelingSalesman` struct that stores the target `TravelingSalesman<SimpleGraph, i32>` instance.

**Step 2: Implement `ReductionResult`**

Implement:
- `target_problem()` as a simple reference getter
- `extract_solution()` by:
  - collecting the selected target edges
  - building degree-2 adjacency from those selected edges
  - traversing the single cycle to recover a permutation of all vertices
  - returning that permutation in source vertex order

Use the target graph’s edge order directly; do not add extra index mapping state unless traversal proves impossible without it.

**Step 3: Implement `ReduceTo<TravelingSalesman<SimpleGraph, i32>>`**

Build the complete graph on the source vertex set and weight edges with:
- `1` if `self.graph().has_edge(u, v)`
- `2` otherwise

Register the reduction with:

```rust
#[reduction(overhead = {
    num_vertices = "num_vertices",
    num_edges = "num_vertices * (num_vertices - 1) / 2",
})]
```

**Step 4: Link the new test module**

Add:

```rust
#[cfg(test)]
#[path = "../unit_tests/rules/hamiltoniancircuit_travelingsalesman.rs"]
mod tests;
```

to the bottom of the new rule file.

**Step 5: Run the focused test to verify it passes**

Run: `cargo test hamiltoniancircuit_to_travelingsalesman --lib`

Expected: all new rule tests pass.

### Task 3: Add canonical rule example data

**Files:**
- Modify: `src/rules/hamiltoniancircuit_travelingsalesman.rs`
- Modify: `src/rules/mod.rs`
- Reference: `src/rules/minimumvertexcover_maximumindependentset.rs`

**Step 1: Add `canonical_rule_example_specs()` to the rule file**

Use a 4-cycle source graph:
- source config: `[0, 1, 2, 3]`
- target graph: weighted `K_4`
- target config: select the four cycle edges that correspond to the source circuit

Construct the example with `rule_example_with_witness::<_, TravelingSalesman<SimpleGraph, i32>>()`.

**Step 2: Register the example spec in `src/rules/mod.rs`**

Extend `canonical_rule_example_specs()` with `hamiltoniancircuit_travelingsalesman::canonical_rule_example_specs()`.

**Step 3: Regenerate the exports needed by docs and fixtures**

Run:
- `cargo run --example export_graph`
- `cargo run --example export_schemas`

Expected: updated reduction graph and problem schema exports include the new rule.

### Task 4: Validate batch-1 behavior before touching paper

**Files:**
- No new files

**Step 1: Run the focused rule/example checks**

Run:
- `cargo test hamiltoniancircuit_to_travelingsalesman --lib`
- `cargo test example_db --lib`

Expected: the new rule behaves correctly and the canonical witness round-trips.

**Step 2: Commit the implementation batch**

Run:

```bash
git add src/rules/mod.rs \
        src/rules/hamiltoniancircuit_travelingsalesman.rs \
        src/unit_tests/rules/hamiltoniancircuit_travelingsalesman.rs
git commit -m "Add HamiltonianCircuit to TravelingSalesman reduction"
```

Do not commit `docs/plans/...` beyond the initial plan-only PR commit; that file is removed later by the pipeline.

## Batch 2: Paper Entry and Full Verification

### Task 5: Document the rule in the paper

**Files:**
- Modify: `docs/paper/reductions.typ`
- Reference: `docs/paper/reductions.typ` (`TravelingSalesman -> ILP`, `TravelingSalesman -> QUBO`)

**Step 1: Add the theorem entry**

Write a new `#reduction-rule("HamiltonianCircuit", "TravelingSalesman", ...)` entry that:
- states the `O(n^2)` construction on the complete graph
- explains the sat-to-opt correctness criterion (`opt = n iff HC exists`)
- describes solution extraction from selected TSP edges back to a Hamiltonian-circuit permutation

**Step 2: Add the worked example block**

Use the canonical 4-cycle fixture and start the `extra:` block with:
- `pred create --example HamiltonianCircuit -o hc.json`
- `pred reduce hc.json --to ... -o bundle.json`
- `pred solve bundle.json`
- `pred evaluate hc.json --config ...`

The example text should explicitly call out the two diagonal non-edges having weight `2`.

### Task 6: Regenerate fixtures and run the repo verification set

**Files:**
- Generated: exports and example fixtures

**Step 1: Regenerate example fixtures**

Run: `make regenerate-fixtures`

Expected: `src/example_db/fixtures/examples.json` gains the new rule example.

**Step 2: Build the paper**

Run: `make paper`

Expected: Typst succeeds and the new theorem/example renders without missing fixture data.

**Step 3: Run final verification**

Run:
- `make test`
- `make clippy`
- `make fmt-check`

Expected: all checks pass.

**Step 4: Commit the documentation/export batch**

Run:

```bash
git add docs/paper/reductions.typ \
        src/example_db/fixtures/examples.json
git commit -m "Document HamiltonianCircuit to TravelingSalesman reduction"
```

Include any tracked export updates from `cargo run --example export_graph` / `cargo run --example export_schemas` if they changed.

## Implementation Notes

- Favor the existing `SimpleGraph::new(n, edges)` edge ordering and build tests around `graph.edges()` rather than hard-coding index assumptions from manual combinatorics.
- In the non-Hamiltonian test, assert the target optimum is `> n`; do not require a specific gap.
- Keep `extract_solution()` tolerant of either tour orientation. It only needs to return a valid Hamiltonian-circuit permutation, not a canonical rotation.
- This rule is a single primitive endpoint pair. Do not add extra cast rules or variant registrations.
