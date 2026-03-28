# Plan: SubsetSum -> CapacityAssignment Reduction

**Issue:** #426 — [Rule] Subset Sum to Capacity Assignment
**Skill:** add-rule Steps 1–6

## Summary

Reduce SubsetSum (Or, feasibility) to CapacityAssignment (Min<u128>, optimization).
Each element becomes a communication link with 2 capacities. High capacity = include element (cost = a_i, delay = 0). Low capacity = exclude element (cost = 0, delay = a_i). Delay budget J = S - B forces the optimizer to select elements totaling at least B. The minimum cost equals B iff a valid subset exists.

## Batch 1: Implementation (add-rule Steps 1–4)

### Step 1: Implement the reduction

File: `src/rules/subsetsum_capacityassignment.rs`

- **ReductionResult struct:** `ReductionSubsetSumToCapacityAssignment` holding the target `CapacityAssignment`.
- **ReductionResult impl:** `extract_solution` maps capacity index 1 (high) -> 1 (include), capacity index 0 (low) -> 0 (exclude). Identity mapping since SubsetSum config[i]=1 means include and CapacityAssignment config[i]=1 means high capacity.
- **ReduceTo impl with `#[reduction]`:**
  - Overhead: `num_links = "num_elements"`, `num_capacities = "2"`
  - Construction:
    1. capacities = [1, 2]
    2. For each element a_i: cost = [0, a_i_u64], delay = [a_i_u64, 0]
    3. delay_budget = S - B (where S = sum of all sizes, B = target)
  - BigUint -> u64 conversion with panic on overflow.

### Step 2: Register in mod.rs

Add `pub(crate) mod subsetsum_capacityassignment;` to `src/rules/mod.rs`.

### Step 3: Write unit tests

File: `src/unit_tests/rules/subsetsum_capacityassignment.rs`

Tests:
1. `test_subsetsum_to_capacityassignment_closed_loop` — YES instance {3,7,1,8,4,12} target 15, use `assert_satisfaction_round_trip_from_optimization_target`
2. `test_subsetsum_to_capacityassignment_structure` — verify target dimensions, cost/delay matrices, delay_budget
3. `test_subsetsum_to_capacityassignment_no_instance` — A={1,5,11,6} B=4, verify min cost > B (no subset sums to 4)
4. `test_subsetsum_to_capacityassignment_panics_on_overflow` — BigUint too large for u64

### Step 4: Add canonical example to example_db

In `src/example_db/rule_builders.rs`:
- Builder function for `subsetsum_to_capacityassignment`
- Source: SubsetSum::new(vec![3u32, 7, 1, 8, 4, 12], 15u32)
- Solution pair: source_config [1,0,0,1,1,0] (elements {3,8,4}), target_config [1,0,0,1,1,0] (same indices get high capacity)

Register in `build_rule_examples()` and `src/rules/mod.rs` canonical specs collection.

## Batch 2: Paper and exports (add-rule Steps 5–6)

### Step 5: Document in paper

In `docs/paper/reductions.typ`, add a `#reduction-rule("SubsetSum", "CapacityAssignment", ...)` entry near the existing SubsetSum section.

### Step 6: Regenerate exports and verify

```bash
cargo run --example export_graph
cargo run --example export_schemas
make regenerate-fixtures
make test clippy
```
