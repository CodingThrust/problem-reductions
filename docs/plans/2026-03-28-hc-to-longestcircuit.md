# Plan: HamiltonianCircuit -> LongestCircuit Reduction

**Issue:** #358
**Skill:** [add-rule](../../.claude/skills/add-rule/SKILL.md)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND28, p.213

## Information Checklist

| # | Item | Value |
|---|------|-------|
| 1 | Source problem | `HamiltonianCircuit<SimpleGraph>` |
| 2 | Target problem | `LongestCircuit<SimpleGraph, i32>` |
| 3 | Reduction algorithm | Copy graph unchanged, assign unit weight (1) to every edge |
| 4 | Solution extraction | Convert binary edge selection to vertex permutation via `edges_to_cycle_order` |
| 5 | Correctness | HC exists iff optimal LC length = |V| (unit weights, max n edges in simple circuit on n vertices) |
| 6 | Size overhead | `num_vertices = "num_vertices"`, `num_edges = "num_edges"` |
| 7 | Example | 4-cycle: HC witness [0,1,2,3] -> LC selects all 4 edges with total length 4 = |V| |
| 8 | Solving strategy | BruteForce on LongestCircuit (already supported) |
| 9 | Reference | Garey & Johnson ND28 |

## Batch 1: Implementation (Steps 1-4)

### Step 1: Implement the reduction

Create `src/rules/hamiltoniancircuit_longestcircuit.rs`:

- **ReductionResult struct** `ReductionHamiltonianCircuitToLongestCircuit` holding the target `LongestCircuit<SimpleGraph, i32>`
- **ReductionResult impl**: `extract_solution` uses `graph_helpers::edges_to_cycle_order` to convert binary edge selection back to vertex permutation (same as HC->TSP)
- **ReduceTo impl** with `#[reduction]` macro:
  - Copy the graph as-is
  - Assign weight `1i32` to every edge
  - Overhead: `num_vertices = "num_vertices"`, `num_edges = "num_edges"`

Key detail: HC config = vertex permutation (each position is a vertex index), LC config = binary edge vector (0/1 per edge). The `extract_solution` must convert from edge-selection back to vertex ordering.

### Step 2: Register in mod.rs

Add `pub(crate) mod hamiltoniancircuit_longestcircuit;` to `src/rules/mod.rs`.

### Step 3: Write unit tests

Create `src/unit_tests/rules/hamiltoniancircuit_longestcircuit.rs`:

1. **Closed-loop test** using `assert_satisfaction_round_trip_from_optimization_target` (feasibility source, optimization target - same pattern as HC->TSP)
2. **Structure test**: verify target has same vertices/edges, all weights are 1
3. **Non-Hamiltonian test**: star graph (no HC), verify optimal LC length < |V|
4. **Extract solution test**: manually construct edge selection for known HC, verify extraction

### Step 4: Add canonical example to example_db

Add builder in `src/example_db/rule_builders.rs`:
- Source: `HamiltonianCircuit::new(SimpleGraph::cycle(4))` (4-cycle, same as HC->TSP)
- Solution: HC witness `[0,1,2,3]`, LC selects all 4 edges `[1,1,1,1]`

Register in `canonical_rule_example_specs()` and `build_rule_examples()`.

## Batch 2: Paper + Exports (Steps 5-6)

### Step 5: Document in paper

Add `reduction-rule("HamiltonianCircuit", "LongestCircuit", ...)` in `docs/paper/reductions.typ` near the existing HC reductions.

- Load example data from fixtures
- Theorem: O(|E|) reduction, copies graph, assigns unit weights
- Proof: construction, correctness (forward/reverse), solution extraction
- Worked example with concrete numbers from fixture data

### Step 6: Regenerate exports and verify

```bash
cargo run --example export_graph
cargo run --example export_schemas
make regenerate-fixtures
make test clippy
make paper
```
