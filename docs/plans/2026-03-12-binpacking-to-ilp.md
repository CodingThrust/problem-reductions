# Plan: BinPacking to ILP Reduction (Issue #97)

## Overview
Add a reduction rule from BinPacking to ILP (Integer Linear Programming) using the standard assignment-based formulation from Martello & Toth (1990).

## Reduction Algorithm
- **Variables:** `x_{ij}` (item i assigned to bin j) + `y_j` (bin j is used), all binary. Total: `n^2 + n` variables.
- **Variable ordering:** `x_{00}, x_{01}, ..., x_{0,n-1}, x_{10}, ..., x_{n-1,n-1}, y_0, ..., y_{n-1}`
- **Constraints:**
  1. Assignment: for each item i, `sum_j x_{ij} = 1` (n constraints)
  2. Capacity+linking: for each bin j, `sum_i w_i * x_{ij} <= C * y_j` (n constraints)
- **Objective:** minimize `sum_j y_j`
- **Solution extraction:** For each item i, find unique j with `x_{ij} = 1`; return bin assignment vector.

## Overhead
- `num_vars = num_items * num_items + num_items`
- `num_constraints = 2 * num_items`

## Implementation Steps

### Step 1: Create reduction rule file
File: `src/rules/binpacking_ilp.rs`
- Implement `ReduceTo<ILP<bool>> for BinPacking<i32>`
- Create `ReductionBPToILP` result struct with `target` and `n` fields
- Use `#[reduction(overhead = { num_vars = "num_items * num_items + num_items", num_constraints = "2 * num_items" })]`
- Solution extraction: for each item i (0..n), scan x_{ij} variables to find which bin j has value 1

### Step 2: Register module in `src/rules/mod.rs`
Add `mod binpacking_ilp;` under the `#[cfg(feature = "ilp-solver")]` block.

### Step 3: Create unit tests
File: `src/unit_tests/rules/binpacking_ilp.rs`
- `test_binpacking_to_ilp_closed_loop`: solve with BruteForce and ILP solver, compare objectives
- `test_reduction_creates_valid_ilp`: verify ILP structure (num_vars, num_constraints)
- `test_single_item`: edge case with 1 item
- `test_same_weight_items`: all items have same weight
- `test_exact_fill`: items that exactly fill bins
- `test_solution_extraction`: verify extract_solution returns valid packing

### Step 4: Create example file
File: `examples/reduction_binpacking_to_ilp.rs`
- Use the issue's example: 5 items with weights [6, 5, 5, 4, 3], capacity 10
- Export JSON to `docs/paper/examples/binpacking_to_ilp.json`

### Step 5: Register example in tests
Add `example_test!` and `example_fn!` entries to `tests/suites/examples.rs`.

### Step 6: Run `make check` to verify everything compiles and passes.
