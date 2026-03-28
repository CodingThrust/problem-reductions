# Plan: Partition -> SubsetSum Reduction

**Issue:** #387 — [Rule] PARTITION to SUBSET SUM
**Skill:** add-rule Steps 1-6
**Reference:** Karp (1972), Garey & Johnson SP13 p.223

## Summary

Implement the textbook-canonical reduction from Partition to SubsetSum. The reduction
re-interprets a Partition instance as a SubsetSum instance with target B = S/2.
When the total sum S is odd, return a trivially infeasible SubsetSum instance
(sizes = [], target = 1).

Both models already exist: `src/models/misc/partition.rs` and `src/models/misc/subset_sum.rs`.

## Batch 1: Implementation (add-rule Steps 1-4, 6)

### Step 1: Implement the reduction

Create `src/rules/partition_subsetsum.rs`:

- **ReductionResult struct:** `ReductionPartitionToSubsetSum { target: SubsetSum }`
- **extract_solution:** Identity — `target_solution.to_vec()`
  - SubsetSum has same number of binary variables as Partition. Both use `x_i = 1` to indicate element `i` is selected.
  - If odd-sum case (trivially infeasible SubsetSum with empty sizes), return empty vec.
- **reduce_to:**
  1. Compute S = total_sum()
  2. If S is odd: return SubsetSum with sizes = [] (via `new_unchecked`), target = 1
  3. If S is even: return SubsetSum with same sizes (converted to BigUint), target = S/2
- **overhead:** `num_elements = "num_elements"`

### Step 2: Register in mod.rs

Add `pub(crate) mod partition_subsetsum;` to `src/rules/mod.rs` (alphabetical order, near existing `partition_*` entries).

### Step 3: Write unit tests

Create `src/unit_tests/rules/partition_subsetsum.rs`:

1. `test_partition_to_subsetsum_closed_loop` — standard closed-loop with [3,1,1,2,2,1]
2. `test_partition_to_subsetsum_structure` — verify target sizes and target value
3. `test_partition_to_subsetsum_odd_sum` — verify odd total sum produces infeasible instance
4. `test_partition_to_subsetsum_all_equal_even` — all-equal elements with even count

### Step 4: Add canonical example to example_db

Add builder function in `src/example_db/rule_builders.rs`:
- Source: Partition::new(vec![5, 3, 8, 2, 7, 1, 4]) (from issue example)
- Solution: A' = {5, 8, 2} -> config [1, 0, 1, 1, 0, 0, 0] (indices 0,2,3 selected)
  - Verify: 5+8+2 = 15 = 30/2

Register in `build_rule_examples()` (via `canonical_rule_example_specs()` in mod.rs).

### Step 6: Regenerate exports and verify

```bash
cargo run --example export_graph
cargo run --example export_schemas
make regenerate-fixtures
make test clippy
```

## Batch 2: Paper entry (add-rule Step 5)

### Step 5: Document in paper

Add `reduction-rule("Partition", "SubsetSum", ...)` entry in `docs/paper/reductions.typ`,
near the existing Partition section.

- **Theorem body:** O(n) reduction, keeps same elements, sets target B = S/2.
- **Proof:** Construction, correctness (biconditional), solution extraction.
- **Worked example:** Use fixture data from the canonical example.

```bash
make paper
```
