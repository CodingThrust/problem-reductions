# Plan: Add ConsecutiveBlockMinimization Model (#420)

## Summary

Add the Consecutive Block Minimization satisfaction problem (Garey & Johnson SR17) to the `algebraic/` category. Given an m×n binary matrix A and positive integer K, decide whether there exists a column permutation yielding at most K maximal blocks of consecutive 1's across all rows.

## Issue Details

- **Problem**: ConsecutiveBlockMinimization
- **Type**: SatisfactionProblem (Metric = bool)
- **Category**: algebraic/ (matrix input)
- **No type parameters** (no graph type, no weight type)
- **NP-complete**: Kou (1977), reduction from Hamiltonian Path
- **Complexity**: O(n^n × m × n) brute-force

## Batch 1: Implementation (Steps 1-5.5)

### Step 1: Category

Place in `src/models/algebraic/` alongside BMF, QUBO, ILP, CVP.

### Step 1.5: Size Getters

Getter methods needed (from complexity expression and overhead expressions):
- `num_rows()` → m (matrix.len())
- `num_cols()` → n (matrix[0].len())
- `bound_k()` → K
- `num_variables()` → num_cols (override default)

### Step 2: Model File

Create `src/models/algebraic/consecutive_block_minimization.rs`:

**Schema entry** (inventory::submit!):
- name: "ConsecutiveBlockMinimization"
- display_name: "ConsecutiveBlockMinimization"
- aliases: &["CBM"]
- dimensions: &[] (no type params)
- fields: matrix (Vec<Vec<bool>>), bound_k (usize)

**Struct**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsecutiveBlockMinimization {
    matrix: Vec<Vec<bool>>,
    num_rows: usize,
    num_cols: usize,
    bound_k: usize,
}
```

**Constructor** `new(matrix, bound_k)`:
- Validate all rows have same length
- Store derived num_rows, num_cols

**Accessors**: `matrix()`, `num_rows()`, `num_cols()`, `bound_k()`

**Helper** `count_blocks(config)`:
- Interpret config as column permutation (config[i] = position of column i)
- Validate it's a valid permutation (all distinct, in 0..num_cols)
- For each row, count maximal runs of consecutive 1's after reordering
- Return Option<usize>: None if invalid permutation, Some(total) otherwise

**Problem impl**:
- `NAME = "ConsecutiveBlockMinimization"`
- `Metric = bool`
- `dims() = vec![num_cols; num_cols]`
- `evaluate(config)`: call count_blocks, return false if invalid permutation, otherwise total <= bound_k
- `variant() = crate::variant_params![]`
- Override `num_variables()` → num_cols

**SatisfactionProblem**: empty marker impl

### Step 2.5: declare_variants!

```rust
crate::declare_variants! {
    default sat ConsecutiveBlockMinimization => "num_cols^num_cols * num_rows * num_cols",
}
```

### Step 3: Register in Modules

1. `src/models/algebraic/mod.rs`:
   - Add `pub(crate) mod consecutive_block_minimization;`
   - Add `pub use consecutive_block_minimization::ConsecutiveBlockMinimization;`
   - Update module doc comment
   - Add to `canonical_model_example_specs()` aggregator

2. `src/models/mod.rs`:
   - Re-export `ConsecutiveBlockMinimization` from `algebraic`

### Step 4: CLI Registration

1. **Aliases**: Already handled via `declare_variants!` + ProblemSchemaEntry aliases field ("CBM")

2. **`problemreductions-cli/src/commands/create.rs`**:
   - Add match arm for "ConsecutiveBlockMinimization"
   - Parse `--matrix` (JSON 2D bool array) and `--bound-k` (integer)
   - Add to `after_help` flag table
   - Add to `all_data_flags_empty`
   - Add to `example_for` with a small example instance

3. **`problemreductions-cli/src/cli.rs`**:
   - Add `--matrix` flag: `Option<String>` for JSON-encoded matrix
   - Add `--bound-k` flag: `Option<usize>`

### Step 4.6: Example-DB

Add `canonical_model_example_specs()` in the model file:
- Use the YES instance from the issue (Instance 2: path graph adjacency matrix, K=6)
- Or use a simpler small instance that has a satisfying permutation
- Use `satisfaction_example(problem, vec![valid_config])`

### Step 5: Unit Tests

Create `src/unit_tests/models/algebraic/consecutive_block_minimization.rs`:

- `test_consecutive_block_minimization_creation`: constructor, getters
- `test_consecutive_block_minimization_evaluation`: test with known YES and NO configs
- `test_consecutive_block_minimization_invalid_permutation`: non-permutation configs return false
- `test_consecutive_block_minimization_serialization`: serde round-trip
- `test_consecutive_block_minimization_solver`: BruteForce::find_satisfying on small instance
- `test_consecutive_block_minimization_paper_example`: test with the issue's example instances
- `test_consecutive_block_minimization_empty_matrix`: edge case

Add `#[path]` link in model file:
```rust
#[cfg(test)]
#[path = "../../unit_tests/models/algebraic/consecutive_block_minimization.rs"]
mod tests;
```

### Step 5.5: Trait Consistency

Add entry in `src/unit_tests/trait_consistency.rs`:
```rust
check_problem_trait(
    &ConsecutiveBlockMinimization::new(matrix, bound_k),
    "ConsecutiveBlockMinimization",
);
```

## Batch 2: Paper Entry (Step 6)

### Step 6: Paper Documentation

In `docs/paper/reductions.typ`:

1. Add to `display-name` dict:
   ```typst
   "ConsecutiveBlockMinimization": [Consecutive Block Minimization],
   ```

2. Add `problem-def` block after related algebraic problems:
   - Formal definition: m×n binary matrix, positive integer K, column permutation, blocks
   - Background: GJ SR17, information retrieval, scheduling, glass cutting
   - Reference to Kou (1977), Booth (1975), Haddadi & Layouni (2008)
   - Connection to consecutive ones property (PQ-trees)

## Verification

After each batch, run:
```bash
make check   # fmt + clippy + test
make paper   # build paper
```
