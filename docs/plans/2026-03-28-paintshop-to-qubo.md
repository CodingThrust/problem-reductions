# Plan: PaintShop -> QUBO Reduction

**Issue:** #649
**Skill:** add-rule
**Reference:** Streif et al. (2021), Physical Review A 104, 012403

## Batch 1: Implementation (Steps 1-4)

### Step 1: Implement the reduction

Create `src/rules/paintshop_qubo.rs`:

**ReductionResult struct:** `ReductionPaintShopToQUBO` stores:
- `target: QUBO<f64>` — the constructed QUBO instance
- `num_cars: usize` — number of cars (= number of QUBO variables)

**Algorithm (reduce_to):**
1. Get `n = num_cars`, `sequence_indices`, `is_first` from the PaintShop instance.
2. Initialize an `n x n` matrix Q of zeros.
3. For each adjacent pair (j, j+1) in the sequence:
   - Let `a = sequence_indices[j]`, `b = sequence_indices[j+1]`.
   - If `a == b`: skip (always a color change — constant term).
   - If `a != b`:
     - Ensure `(a, b)` is ordered as `(min, max)` for upper-triangular storage.
     - If same parity (both `is_first` or both `!is_first`): color change when `x_a != x_b`.
       Add +1 to Q[a][a], +1 to Q[b][b], -2 to Q[min][max].
     - If different parity: color change when `x_a == x_b`.
       Add -1 to Q[a][a], -1 to Q[b][b], +2 to Q[min][max].
4. Construct `QUBO::from_matrix(matrix)`.

**Solution extraction:** Direct identity mapping — QUBO solution `(x_1, ..., x_n)` maps directly to PaintShop config where car `i` gets color `x_i` at its first occurrence and `1-x_i` at its second.

**Note on value correspondence:** The QUBO minimizes `x^T Q x`. The PaintShop minimum switches = QUBO_min + offset, where offset = number of different-parity adjacent pairs + number of same-car adjacent pairs. Since QUBO<f64> and PaintShop both use `Min<_>` values but with different scales, this is a **witness-preserving** reduction (optimal QUBO configs map to optimal PaintShop configs) but not value-preserving. We only need `extract_solution`, not value extraction.

**Public API additions to PaintShop:** Add `sequence_indices()` and `is_first()` getters to expose the internal fields needed by the reduction.

**Overhead:** `num_vars = "num_cars"`

### Step 2: Register in mod.rs

Add `pub(crate) mod paintshop_qubo;` to `src/rules/mod.rs` (alphabetical order, among the non-ILP rules).

### Step 3: Write unit tests

Create `src/unit_tests/rules/paintshop_qubo.rs`:

1. `test_paintshop_to_qubo_closed_loop` — use `assert_optimization_round_trip_from_optimization_target`
   - Test with the example from the issue: [A, B, C, A, D, B, D, C]
   - Verify optimal switches = 2

2. `test_paintshop_to_qubo_structure` — verify QUBO matrix dimensions and specific entries match the issue example.

3. `test_paintshop_to_qubo_small` — test with a minimal instance: [A, B, A, B] (2 cars).

4. `test_paintshop_to_qubo_trivial` — test with [A, A] (1 car, always 1 switch).

### Step 4: Add canonical example to example_db

Add `paintshop_to_qubo` builder in `src/example_db/rule_builders.rs` (via `canonical_rule_example_specs()` in the rule file).
- Source: PaintShop with sequence [A, B, C, A, D, B, D, C]
- Use `rule_example_with_witness` with pre-computed solution pair.

Register in `src/rules/mod.rs` `canonical_rule_example_specs()` function.

## Batch 2: Paper and exports (Steps 5-6)

### Step 5: Document in paper

Add `reduction-rule("PaintShop", "QUBO", ...)` entry in `docs/paper/reductions.typ`:
- Rule statement: O(n) reduction from Streif et al. 2021
- Construction: detail the Q matrix construction from adjacent pair parity analysis
- Correctness: prove witness preservation
- Worked example: [A, B, C, A, D, B, D, C] -> 4x4 QUBO matrix

### Step 6: Regenerate exports and verify

```bash
cargo run --example export_graph
cargo run --example export_schemas
make regenerate-fixtures   # needs ILP
make test clippy
```
