# Plan: Add Partition Model

**Issue:** #210 — [Model] Partition
**Skill:** add-model (Steps 1–7)

## Overview

Add a `Partition` satisfaction model: given a finite set A of positive integers, determine whether A can be partitioned into two subsets with equal sum. This is a classic NP-complete problem (Karp #20, Garey & Johnson A3 SP12), though only weakly NP-hard due to the pseudo-polynomial DP algorithm.

## Design Decisions

- **Category:** `misc/` — Partition takes a plain list of positive integers as input. It does not fit `graph/` (no graph), `formula/` (no boolean formula), `set/` (no universe/subset structure), or `algebraic/` (no matrix/lattice).
- **Struct:** `Partition` with a single field `sizes: Vec<u64>`. No type parameters — sizes are always positive integers (matching the GJ definition: s(a) in Z+).
- **Problem type:** Satisfaction (`Metric = bool`, implements `SatisfactionProblem`).
- **dims():** `vec![2; n]` where n = number of elements. Each binary variable x_i indicates which subset element i belongs to.
- **evaluate():** Compute sum of sizes where x_i = 0 and sum where x_i = 1. Return `true` iff the two sums are equal (equivalently, each equals total_sum / 2).
- **variant():** `variant_params![]` — no type parameters.
- **Getter methods:** `num_elements()` returns `sizes.len()` — used in the complexity expression.
- **Complexity:** `2^(num_elements / 2)` from the Schroeppel-Shamir meet-in-the-middle algorithm (SIAM J. Comput. 10(4):456-464, 1981). This solves SUBSET SUM (and hence PARTITION) in O*(2^(n/2)) time and O*(2^(n/4)) space.
- **Solver:** BruteForce (existing) — enumerates all 2^n binary configurations, checking feasibility via `evaluate()`. Also solvable via ILP (binary constraint: sum x_i * s_i = total/2).

## Steps

### Step 1: Determine category
Category: `misc/`

### Step 1.5: Infer problem size getters
From complexity O*(2^(n/2)) where n = |A|:
- `num_elements()` -> `usize` (number of elements in A)

### Step 2: Implement the model
Create `src/models/misc/partition.rs`:

```rust
// Structure:
// 1. inventory::submit! for ProblemSchemaEntry
//    - name: "Partition"
//    - fields: [sizes: Vec<u64>]
// 2. Partition struct with sizes: Vec<u64>
// 3. Constructor: new(sizes: Vec<u64>) — panics if any size is 0
// 4. Accessors: sizes(), num_elements(), total_sum()
// 5. Problem impl: NAME="Partition", Metric=bool, dims()=vec![2; n]
// 6. evaluate(): sum sizes in each partition, return sums_equal
// 7. SatisfactionProblem impl (marker trait, empty)
// 8. declare_variants! { Partition => "2^(num_elements / 2)" }
// 9. #[cfg(test)] #[path] link
```

Key implementation details for `evaluate()`:
```
1. Convert config to partition assignment: x_i = 0 -> subset S0, x_i = 1 -> subset S1
2. Compute sum_0 = sum of sizes[i] where config[i] == 0
3. Compute sum_1 = sum of sizes[i] where config[i] == 1
4. Return sum_0 == sum_1
```

Note: If total_sum is odd, no valid partition exists — evaluate() will always return false (which is correct behavior; no need for special-casing).

### Step 2.5: Register variant complexity
```rust
crate::declare_variants! {
    Partition => "2^(num_elements / 2)",
}
```

### Step 3: Register the model
1. `src/models/misc/mod.rs` — add `mod partition;` and `pub use partition::Partition;`
2. `src/models/mod.rs` — add `Partition` to the `misc` re-export line

### Step 4: Register in CLI
1. `problemreductions-cli/src/dispatch.rs`:
   - `load_problem()`: add `"Partition" => deser_sat::<Partition>(data)`
   - `serialize_any_problem()`: add `"Partition" => try_ser::<Partition>(any)`
2. `problemreductions-cli/src/problem_name.rs`:
   - `resolve_alias()`: add `"partition" => "Partition".to_string()`
   - No short alias — "PART" is not a well-established abbreviation

### Step 4.5: Add CLI creation support
Update `problemreductions-cli/src/commands/create.rs`:
- Add a match arm for `"Partition"` that parses a `--sizes` flag (comma-separated u64 values)
- Update `cli.rs` if a new flag is needed (or reuse an existing mechanism)
- Update help text in `CreateArgs`

### Step 5: Write unit tests
Create `src/unit_tests/models/misc/partition.rs`:

Tests:
- `test_partition_creation` — construct instance, verify num_elements, dims
- `test_partition_evaluation_feasible` — A = {3, 1, 1, 2, 2, 1}, config assigning {3,2} to one side returns true
- `test_partition_evaluation_infeasible` — wrong partition returns false
- `test_partition_odd_sum` — A = {1, 2, 4}, total=7 (odd), no valid partition exists
- `test_partition_solver` — BruteForce finds satisfying assignment for the example instance
- `test_partition_serialization` — round-trip serde test

Example instance from issue:
- A = {3, 1, 1, 2, 2, 1}, n = 6, total_sum = 10, target = 5
- Feasible: A' = {a_1, a_4} = {3, 2}, sum = 5; complement = {1, 1, 2, 1}, sum = 5
- Config: [0, 1, 1, 0, 1, 1] (0 = in A', 1 = in complement) — evaluate() returns true

### Step 6: Document in paper
Update `docs/paper/reductions.typ`:
1. Add to `display-name` dictionary: `"Partition": [Partition]`
2. Add `#problem-def("Partition")[...]` block with:
   - Formal definition from GJ A3 SP12
   - Reference to Karp 1972 and Garey & Johnson
   - Note on weak NP-hardness and pseudo-polynomial DP
   - Example visualization showing the partition of {3, 1, 1, 2, 2, 1}
   - Algorithm list: brute-force, Schroeppel-Shamir meet-in-the-middle, pseudo-polynomial DP

### Step 7: Verify
```bash
make check  # fmt + clippy + test
```
Then run `/review-implementation` to verify completeness.

## Files Changed

| File | Action |
|------|--------|
| `src/models/misc/partition.rs` | **Create** — model implementation |
| `src/unit_tests/models/misc/partition.rs` | **Create** — unit tests |
| `src/models/misc/mod.rs` | **Edit** — register module |
| `src/models/mod.rs` | **Edit** — add re-export |
| `problemreductions-cli/src/dispatch.rs` | **Edit** — CLI dispatch |
| `problemreductions-cli/src/problem_name.rs` | **Edit** — alias |
| `problemreductions-cli/src/commands/create.rs` | **Edit** — CLI create support |
| `problemreductions-cli/src/cli.rs` | **Edit** — CLI flags (if needed) |
| `docs/paper/reductions.typ` | **Edit** — paper definition |
