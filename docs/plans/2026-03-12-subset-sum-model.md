# Plan: Add SubsetSum Model (#402)

## Overview
Add SubsetSum as a satisfaction problem in `src/models/misc/`. SubsetSum asks whether any subset of positive integers sums to a target value B.

## Steps

### 1. Model file: `src/models/misc/subset_sum.rs`
- Struct: `SubsetSum { sizes: Vec<u64>, target: u64 }`
- Getters: `sizes()`, `target()`, `num_items()`
- `Problem` impl: NAME = "SubsetSum", Metric = bool, dims = [2; n], evaluate checks if selected subset sums to target
- `SatisfactionProblem` impl (marker)
- `declare_variants!` with complexity `2^(num_items / 2)` (Horowitz-Sahni meet-in-the-middle)
- `inventory::submit!` for ProblemSchemaEntry
- Unit test path link

### 2. Register in `src/models/misc/mod.rs`
- Add `mod subset_sum;` and `pub use subset_sum::SubsetSum;`

### 3. Export from `src/lib.rs` prelude
- Add `SubsetSum` to the misc imports in prelude

### 4. Unit tests: `src/unit_tests/models/misc/subset_sum.rs`
- test_subset_sum_basic: field accessors, dims, NAME, variant
- test_subset_sum_evaluate_feasible: config that sums to target -> true
- test_subset_sum_evaluate_infeasible: config that doesn't sum -> false
- test_subset_sum_empty: empty set
- test_subset_sum_brute_force: find_satisfying
- test_subset_sum_serialization: round-trip JSON
- test_subset_sum_no_solution: impossible target
- test_subset_sum_all_selected: all items sum to target

### 5. CLI registration
- `problemreductions-cli/src/problem_name.rs`: add "subsetsum" alias
- `problemreductions-cli/src/dispatch.rs`: add deser_sat/try_ser for SubsetSum
- `problemreductions-cli/src/cli.rs`: add SubsetSum to help table
- `problemreductions-cli/src/commands/create.rs`: add create handler using --sizes and --target flags

### 6. CLI args
- Reuse `--sizes` flag from BinPacking
- Add `--target` usage note (already exists for Factoring, but Factoring uses u64 target)
- SubsetSum uses: `pred create SubsetSum --sizes 3,7,1,8,2,4 --target 11`

### 7. Regenerate exports
- `make export-schemas`
