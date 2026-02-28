# Plan: Add BinPacking Model

**Issue:** #95 — [Model] BinPacking
**Skill:** add-model (Steps 1–7)

## Overview

Add a `BinPacking` optimization model: given items with sizes and a bin capacity, minimize the number of bins used to pack all items such that no bin exceeds capacity.

## Design Decisions

- **Category:** `specialized` — BinPacking is a domain-specific packing/scheduling problem. It doesn't fit `graph/` (no graph), `set/` (not subset selection), `optimization/` (reserved for generic formulations like QUBO/ILP), or `satisfiability/`.
- **Struct:** `BinPacking<W = i32>` with fields `sizes: Vec<W>` and `capacity: W`. Generic over weight type W for integer or real-valued sizes.
- **dims():** `vec![n; n]` where n = number of items. Each variable is a bin index in {0, ..., n−1}. This is the first non-binary configuration space in the codebase.
- **Objective:** Minimize the count of distinct bin indices used (always `i32`, regardless of W). So `Metric = SolutionSize<i32>`, `Value = i32`.
- **Feasibility:** For each bin j, the sum of sizes of items assigned to j must not exceed capacity. Uses `WeightElement::to_sum()` for size summation and capacity comparison.
- **variant():** `variant_params![W]` — exposes weight type (i32, f64).
- **Solver:** BruteForce (existing) — enumerates all n^n assignments. No ILP reduction in this PR.

## Steps

### Step 1: Determine category
Category: `specialized/`

### Step 2: Implement the model
Create `src/models/specialized/bin_packing.rs`:

```rust
// Structure:
// 1. inventory::submit! for ProblemSchemaEntry
// 2. BinPacking<W> struct with sizes: Vec<W>, capacity: W
// 3. Constructor: new(sizes, capacity), with_unit_sizes(sizes, capacity) if W: From<i32>
// 4. Accessors: sizes(), capacity(), num_items()
// 5. Problem impl: NAME="BinPacking", Metric=SolutionSize<i32>, dims()=vec![n;n]
// 6. evaluate(): check bin capacities, count distinct bins
// 7. OptimizationProblem impl: Value=i32, direction=Minimize
// 8. #[cfg(test)] #[path] link
```

Key implementation details for `evaluate()`:
```
1. Group items by assigned bin index
2. For each bin, sum sizes via to_sum() and compare with capacity.to_sum()
3. If any bin exceeds capacity → SolutionSize::Invalid
4. Otherwise → SolutionSize::Valid(num_distinct_bins as i32)
```

### Step 3: Register the model
1. `src/models/specialized/mod.rs` — add `pub(crate) mod bin_packing;` and `pub use bin_packing::BinPacking;`
2. `src/models/mod.rs` — add `BinPacking` to the `specialized` re-export line

### Step 4: Register in CLI
1. `problemreductions-cli/src/dispatch.rs`:
   - `load_problem()`: add `"BinPacking" => deser_opt::<BinPacking<i32>>(data)`
   - `serialize_any_problem()`: add `"BinPacking" => try_ser::<BinPacking<i32>>(any)`
2. `problemreductions-cli/src/problem_name.rs`:
   - `resolve_alias()`: add `"binpacking" => "BinPacking".to_string()`
   - Optionally add `("BP", "BinPacking")` to `ALIASES`

### Step 5: Write unit tests
Create `src/unit_tests/models/specialized/bin_packing.rs`:

Tests:
- `test_binpacking_creation` — construct instance, verify num_items, dims
- `test_binpacking_evaluation_valid` — valid packing returns SolutionSize::Valid(num_bins)
- `test_binpacking_evaluation_invalid` — overloaded bin returns SolutionSize::Invalid
- `test_binpacking_direction` — verify Direction::Minimize
- `test_binpacking_solver` — BruteForce finds optimal 3-bin solution for the example instance (6 items, sizes [6,6,5,5,4,4], capacity 10)
- `test_binpacking_serialization` — round-trip serde test

Example instance from issue:
- 6 items, capacity C = 10, sizes = [6, 6, 5, 5, 4, 4]
- Optimal: 3 bins, e.g., x = (0, 1, 2, 2, 0, 1)

### Step 6: Document in paper
Update `docs/paper/reductions.typ`:
1. Add to `display-name` dictionary: `"BinPacking": [Bin Packing]`
2. Add `#problem-def("BinPacking")[...]` block with mathematical definition

### Step 7: Verify
```bash
make check  # fmt + clippy + test
```
Then run `/review-implementation` to verify completeness.

## Files Changed

| File | Action |
|------|--------|
| `src/models/specialized/bin_packing.rs` | **Create** — model implementation |
| `src/unit_tests/models/specialized/bin_packing.rs` | **Create** — unit tests |
| `src/models/specialized/mod.rs` | **Edit** — register module |
| `src/models/mod.rs` | **Edit** — add re-export |
| `problemreductions-cli/src/dispatch.rs` | **Edit** — CLI dispatch |
| `problemreductions-cli/src/problem_name.rs` | **Edit** — alias |
| `docs/paper/reductions.typ` | **Edit** — paper definition |
