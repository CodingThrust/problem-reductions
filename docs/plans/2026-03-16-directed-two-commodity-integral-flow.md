# Plan: Add DirectedTwoCommodityIntegralFlow Model

**Issue:** #295
**Type:** [Model]
**Category:** `graph/` (directed graph input)

## Overview

Implement the Directed Two-Commodity Integral Flow problem (Garey & Johnson A2 ND38). This is a satisfaction problem: given a directed graph with arc capacities and two source-sink pairs with flow requirements, determine whether two integral flow functions exist that satisfy joint capacity constraints, flow conservation, and requirement thresholds.

## Batch 1: Implementation (Steps 1-5.5)

### Step 1: Category = `graph/`

The problem takes a directed graph as primary input, so it belongs in `src/models/graph/`.

### Step 1.5: Size getters

From the complexity expression `(max_capacity + 1)^(2 * num_arcs)`:
- `num_vertices()` -> from `graph.num_vertices()`
- `num_arcs()` -> from `graph.num_arcs()`
- `max_capacity()` -> from `capacities.iter().max().copied().unwrap_or(0)`

### Step 2: Implement the model

Create `src/models/graph/directed_two_commodity_integral_flow.rs`:

**Struct fields:**
- `graph: DirectedGraph` — directed graph G = (V, A)
- `capacities: Vec<u64>` — capacity c(a) for each arc
- `source_1: usize` — source vertex s_1
- `sink_1: usize` — sink vertex t_1
- `source_2: usize` — source vertex s_2
- `sink_2: usize` — sink vertex t_2
- `requirement_1: u64` — flow requirement R_1
- `requirement_2: u64` — flow requirement R_2

**Problem trait:**
- `NAME = "DirectedTwoCommodityIntegralFlow"`
- `Metric = bool` (satisfaction problem)
- `dims()` = for each arc a, the domain for commodity i's flow is {0, ..., c(a)}. Configuration is a flat vector of 2*|A| variables: first |A| for commodity 1, next |A| for commodity 2. `dims()` returns `capacities.iter().chain(capacities.iter()).map(|&c| (c as usize) + 1).collect()`
- `evaluate()`: check (1) joint capacity f_1(a)+f_2(a) <= c(a) for all arcs, (2) flow conservation for each commodity at non-terminal vertices, (3) net flow into sink >= requirement for each commodity
- `variant()` = `variant_params![]` (no type parameters)

**Implement `SatisfactionProblem` (marker trait).**

**`inventory::submit!` for `ProblemSchemaEntry`** — fields: graph, capacities, source_1, sink_1, source_2, sink_2, requirement_1, requirement_2. No aliases (name is too long for a standard abbreviation, and none exists in the literature). No dimensions.

### Step 2.5: Register variant complexity

```rust
crate::declare_variants! {
    default sat DirectedTwoCommodityIntegralFlow => "(max_capacity + 1)^(2 * num_arcs)",
}
```

### Step 3: Register the model

1. `src/models/graph/mod.rs` — add `pub(crate) mod directed_two_commodity_integral_flow;` and `pub use directed_two_commodity_integral_flow::DirectedTwoCommodityIntegralFlow;`
2. `src/models/mod.rs` — add `DirectedTwoCommodityIntegralFlow` to the `graph::` re-export line

### Step 4: CLI discovery

Add lowercase alias mapping in `problemreductions-cli/src/problem_name.rs` `resolve_alias()` — no action needed since aliases are now catalog-driven via `ProblemSchemaEntry`. The `ProblemSchemaEntry` with no aliases is sufficient.

### Step 4.5: CLI create support

Add a match arm in `problemreductions-cli/src/commands/create.rs` for `"DirectedTwoCommodityIntegralFlow"`:
- Requires: `--arcs`, `--capacities`, `--source-1`, `--sink-1`, `--source-2`, `--sink-2`, `--requirement-1`, `--requirement-2`
- Add new CLI flags to `CreateArgs` in `problemreductions-cli/src/cli.rs`: `capacities`, `source_1`, `sink_1`, `source_2`, `sink_2`, `requirement_1`, `requirement_2`
- Update `all_data_flags_empty()` to include the new flags
- Add entry to "Flags by problem type" help table

### Step 4.6: Canonical model example

Add builder function in `src/example_db/model_builders.rs` — use the YES instance from the issue: 6 vertices, 8 arcs (all capacity 1), source_1=0, sink_1=4, source_2=1, sink_2=5, R_1=1, R_2=1. Known satisfying solution: commodity 1 path 0->2->4, commodity 2 path 1->3->5.

Register in `src/models/graph/mod.rs` `canonical_model_example_specs()`.

### Step 5: Unit tests

Create `src/unit_tests/models/graph/directed_two_commodity_integral_flow.rs`:

- `test_directed_two_commodity_integral_flow_creation` — construct the 6-vertex instance, verify dims
- `test_directed_two_commodity_integral_flow_evaluation_satisfying` — verify the YES instance evaluates to `true`
- `test_directed_two_commodity_integral_flow_evaluation_unsatisfying` — verify the NO instance evaluates to `false`
- `test_directed_two_commodity_integral_flow_solver` — use BruteForce::find_satisfying on the YES instance
- `test_directed_two_commodity_integral_flow_no_solution` — verify BruteForce::find_satisfying returns None on the NO instance
- `test_directed_two_commodity_integral_flow_serialization` — round-trip serde
- `test_directed_two_commodity_integral_flow_paper_example` — verify example from paper (same as YES instance), check solution count

Link via `#[cfg(test)] #[path = "..."] mod tests;`

### Step 5.5: Trait consistency

Add `check_problem_trait(...)` call in `src/unit_tests/trait_consistency.rs` for `DirectedTwoCommodityIntegralFlow`.
No `test_direction` entry needed (satisfaction problem).

## Batch 2: Paper Entry (Step 6)

### Step 6: Document in paper

Add to `docs/paper/reductions.typ`:
1. Display name: `"DirectedTwoCommodityIntegralFlow": [Directed Two-Commodity Integral Flow]`
2. `problem-def("DirectedTwoCommodityIntegralFlow")` with formal definition and body (background, example with CeTZ diagram showing the 6-vertex network with two commodity paths highlighted)

Run `make paper` to verify compilation.
