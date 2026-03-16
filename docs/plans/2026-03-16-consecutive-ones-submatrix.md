# Plan: Add ConsecutiveOnesSubmatrix Model (#417)

## Problem Summary

**ConsecutiveOnesSubmatrix** is a satisfaction (decision) problem from Garey & Johnson A4 SR14.

**Definition:** Given an m×n binary matrix A and a positive integer K ≤ n, is there an m×K submatrix B (formed by selecting K columns) that has the "consecutive ones property" — i.e., the columns of B can be permuted so that in each row all 1's occur consecutively?

**Category:** `algebraic/` (binary matrix input)
**Type:** Satisfaction (`Metric = bool`, `SatisfactionProblem`)
**Parameters:** None (no graph/weight type params)
**Fields:** `matrix: Vec<Vec<bool>>`, `bound_k: usize`
**Getters:** `num_rows()` → m, `num_cols()` → n, `bound_k()` → K
**Dims:** `vec![2; num_cols]` (binary: include column or not)
**Complexity:** `"2^(num_cols) * num_rows"` (brute-force column subset enumeration with C1P test)

**Evaluate logic:** For a configuration selecting exactly K columns, try all K! permutations of those columns and check if every row's 1-entries are consecutive in at least one permutation. Return true if such a permutation exists.

**Associated rules:** R108 (HamiltonianPath → ConsecutiveOnesSubmatrix), blocked until HamiltonianPath model exists.

## Batch 1: Implementation (Steps 1–5.5)

All tasks in this batch are independent except where noted.

### Task 1: Create model file `src/models/algebraic/consecutive_ones_submatrix.rs`

Follow `SubsetSum` and `BMF` patterns. Reference: `src/models/misc/subset_sum.rs`.

```rust
// 1. ProblemSchemaEntry via inventory::submit!
//    name: "ConsecutiveOnesSubmatrix"
//    display_name: "Consecutive Ones Submatrix"
//    aliases: &[]
//    dimensions: &[]
//    fields: [
//      { name: "matrix", type: "Vec<Vec<bool>>", desc: "m×n binary matrix A" },
//      { name: "bound_k", type: "usize", desc: "Required number of columns K" },
//    ]

// 2. Struct: ConsecutiveOnesSubmatrix { matrix, bound_k }
//    - new(matrix, bound_k): validate rows same length, bound_k ≤ n
//    - matrix() -> &[Vec<bool>]
//    - bound_k() -> usize
//    - num_rows() -> usize (matrix.len())
//    - num_cols() -> usize (matrix[0].len() or 0)

// 3. Problem trait:
//    - NAME = "ConsecutiveOnesSubmatrix"
//    - Metric = bool
//    - variant() = variant_params![]
//    - dims() = vec![2; num_cols]
//    - evaluate(config):
//      a. Check config.len() == num_cols, all values < 2
//      b. Collect selected column indices where config[j] == 1
//      c. If count != bound_k, return false
//      d. Try all permutations of selected columns
//      e. For each permutation, check if all rows have consecutive 1's
//      f. Return true if any permutation passes

// 4. SatisfactionProblem marker trait

// 5. declare_variants! { default sat ConsecutiveOnesSubmatrix => "2^(num_cols) * num_rows" }

// 6. canonical_model_example_specs() with satisfaction_example
//    Use Tucker matrix example (3×4, K=3):
//    matrix = [[1,1,0,1],[1,0,1,1],[0,1,1,0]], bound_k = 3
//    Solution: select cols {0,1,3} → permutation [1,0,3] gives C1P

// 7. #[cfg(test)] #[path = "../../unit_tests/models/algebraic/consecutive_ones_submatrix.rs"] mod tests;
```

**Helper function `has_consecutive_ones_property`:** Given selected column indices and a permutation, check if every row's 1-entries in the reordered columns are consecutive. This can be a private helper on the struct.

### Task 2: Register model in module tree

1. `src/models/algebraic/mod.rs`: Add `pub(crate) mod consecutive_ones_submatrix;` and `pub use consecutive_ones_submatrix::ConsecutiveOnesSubmatrix;`. Add to `canonical_model_example_specs()`.
2. `src/models/mod.rs`: Add `ConsecutiveOnesSubmatrix` to the `algebraic` re-export line.

### Task 3: CLI integration

1. **`problemreductions-cli/src/problem_name.rs`**: Add lowercase alias `"consecutiveonesubmatrix" | "consecutiveones" | "c1s"` → nope, only add the lowercase form `"consecutiveonessubmatrix"` to `resolve_alias()`. Per conventions, do NOT invent short aliases.

2. **`problemreductions-cli/src/commands/create.rs`**:
   - Import `ConsecutiveOnesSubmatrix` from `problemreductions::models::algebraic`
   - Add match arm for `"ConsecutiveOnesSubmatrix"`:
     - Parse `--matrix` (bool matrix, reuse `parse_bool_matrix()` helper from BMF)
     - Parse `--bound-k` (new flag, or reuse existing suitable flag)
     - Construct `ConsecutiveOnesSubmatrix::new(matrix, bound_k)`
   - Need a `--bound-k` flag or reuse `--k` if it exists. Check existing flags.

3. **`problemreductions-cli/src/cli.rs`**:
   - Add `--bound-k` flag to `CreateArgs` if not already present (check if `--k` exists)
   - Update `all_data_flags_empty()` if adding new flag
   - Add entry to "Flags by problem type" table: `ConsecutiveOnesSubmatrix  --matrix (0/1), --bound-k`

### Task 4: Unit tests `src/unit_tests/models/algebraic/consecutive_ones_submatrix.rs`

```rust
// test_consecutive_ones_submatrix_basic
//   - Construct 3×4 Tucker matrix instance with K=3
//   - Assert num_rows, num_cols, bound_k, dims, NAME, variant

// test_consecutive_ones_submatrix_evaluate_satisfying
//   - Tucker matrix K=3: select cols {0,1,3} (config [1,1,0,1]) → true

// test_consecutive_ones_submatrix_evaluate_unsatisfying
//   - Tucker matrix K=4: select all cols [1,1,1,1] → false (Tucker obstruction)
//   - Wrong count: [1,0,0,0] with K=3 → false

// test_consecutive_ones_submatrix_evaluate_wrong_config
//   - Wrong length, out-of-range values

// test_consecutive_ones_submatrix_brute_force
//   - Tucker matrix K=3: find_satisfying returns valid solution
//   - find_all_satisfying: all solutions valid

// test_consecutive_ones_submatrix_unsatisfiable
//   - Tucker matrix K=4: no permutation works → find_satisfying returns None

// test_consecutive_ones_submatrix_serialization
//   - Round-trip serde JSON test

// test_consecutive_ones_submatrix_trivial_c1p
//   - Identity-like matrix where full matrix has C1P (K=n): should be YES

// test_consecutive_ones_submatrix_paper_example
//   - Same instance as paper, verify exact solution and count
```

### Task 5: trait_consistency entry

`src/unit_tests/trait_consistency.rs`:
- Add `check_problem_trait(&ConsecutiveOnesSubmatrix::new(...), "ConsecutiveOnesSubmatrix")` in `test_all_problems_implement_trait_correctly`.
- No direction test needed (satisfaction, not optimization).

### Task 6: Verify batch 1

```bash
make test clippy
```

## Batch 2: Paper Entry (Step 6)

Depends on Batch 1 (needs working model for exports).

### Task 7: Write paper entry in `docs/paper/reductions.typ`

1. **Display name:** Add `"ConsecutiveOnesSubmatrix": [Consecutive Ones Submatrix],` to `display-name` dict.

2. **Problem definition:**
   ```typst
   #problem-def("ConsecutiveOnesSubmatrix")[
     Given an $m times n$ binary matrix $A$ and a positive integer $K <= n$,
     determine whether there exists a subset of $K$ columns of $A$ whose columns
     can be permuted so that in each row all 1's occur consecutively.
   ][
     // Background, algorithms, example with CeTZ diagram
   ]
   ```

3. **Example:** Use Tucker 3×4 matrix with K=3. Show the matrix, the selected columns, the permutation, and verify C1P visually with a small table or diagram.

4. **Verify:** `make paper`
