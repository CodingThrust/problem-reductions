# Plan: Add SumOfSquaresPartition Model (#403)

## Summary

Implement the SumOfSquaresPartition satisfaction problem (Garey & Johnson SP19).
Given a finite set A of positive integers, K groups, and bound J, determine whether A
can be partitioned into K disjoint groups such that the sum of squared group sums <= J.

## Batch 1: Implementation (add-model Steps 1-5.5)

### Step 1: Category = `misc`
- Input is a set of positive integers + bounds, not graph/formula/set/algebraic
- File: `src/models/misc/sum_of_squares_partition.rs`

### Step 1.5: Size getters
- `num_elements()` -> |A| (number of elements)
- `num_groups()` -> K (number of groups)
- Complexity: `num_groups^num_elements` (brute-force K^n)

### Step 2: Implement the model
- Struct: `SumOfSquaresPartition { sizes: Vec<i64>, num_groups: usize, bound: i64 }`
- No type parameters (concrete i64 per issue comments)
- `Problem` trait: NAME = "SumOfSquaresPartition", Metric = bool
- `dims()`: `vec![num_groups; sizes.len()]` — each element assigned to group 0..K-1
- `evaluate()`: partition elements by group, compute sum of each group, square and sum,
  check <= bound
- `SatisfactionProblem` marker trait
- `variant()`: `variant_params![]` (no type params)
- Constructor validates sizes are positive (> 0)
- ProblemSchemaEntry with fields: sizes (Vec<i64>), num_groups (usize), bound (i64)

### Step 2.5: Register variant complexity
```rust
crate::declare_variants! {
    default sat SumOfSquaresPartition => "num_groups^num_elements",
}
```

### Step 3: Register the model
- `src/models/misc/mod.rs`: add `mod sum_of_squares_partition;` and `pub use`
- `src/models/mod.rs`: add `SumOfSquaresPartition` to misc re-exports

### Step 4: CLI discovery
- `problemreductions-cli/src/problem_name.rs`: add lowercase alias in resolve_alias()
  ("sumofsquarespartition" => "SumOfSquaresPartition")
- No short alias (not well-established in literature)

### Step 4.5: CLI creation support
- Add `--num-groups` flag to CreateArgs in `cli.rs`
- Add `all_data_flags_empty` check for `num_groups`
- Add match arm in `create.rs` for "SumOfSquaresPartition":
  requires --sizes, --num-groups, --bound
- Add to help table in cli.rs
- Add hint string in `hint_for_problem()`

### Step 4.6: Canonical model example
- Add builder in `src/example_db/model_builders.rs` via `canonical_model_example_specs()`
  in the misc module's `sum_of_squares_partition.rs`
- Instance: sizes=[5,3,8,2,7,1], num_groups=3, bound=240
- Satisfying config: [0,2,0,1,2,1] -> groups {5,8},{2,1},{3,7} -> sums 13,3,10 -> 169+9+100=278 <= 240? No.
  Actually need to verify: partition {8,1},{5,2},{3,7} = groups with sums 9,7,10 -> 81+49+100=230 <= 240.
  Config mapping: element indices [0..5] for sizes [5,3,8,2,7,1]:
  - a0=5 -> group 1 (A2={5,2}), a1=3 -> group 2 (A3={3,7}), a2=8 -> group 0 (A1={8,1}),
    a3=2 -> group 1, a4=7 -> group 2, a5=1 -> group 0
  - Config: [1, 2, 0, 1, 2, 0]
- Sample eval with this satisfying config

### Step 5: Unit tests
- `src/unit_tests/models/misc/sum_of_squares_partition.rs`
- Tests: creation, evaluation (satisfying/unsatisfying), wrong config, brute force, serialization
- Paper example test

### Step 5.5: trait_consistency
- Add `check_problem_trait(SumOfSquaresPartition::new(...))` entry
- No test_direction entry (satisfaction problem, not optimization)

## Batch 2: Paper entry (add-model Step 6)

### Step 6: Document in paper
- Add display name: `"SumOfSquaresPartition": [Sum of Squares Partition]`
- Add `problem-def("SumOfSquaresPartition")` with:
  - Formal definition (satisfaction)
  - Background (NP-complete in the strong sense, G&J SP19)
  - Example with the canonical instance
- Run `make paper` to verify
