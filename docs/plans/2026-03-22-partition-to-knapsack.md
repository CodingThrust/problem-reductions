# Plan: Add Partition -> Knapsack Reduction

**Issue:** #202 — [Rule] Partition to Knapsack
**Skill:** add-rule

## Information Checklist

| # | Item | Value |
|---|------|-------|
| 1 | Source problem | `Partition` |
| 2 | Target problem | `Knapsack` |
| 3 | Reduction algorithm | Map each Partition element `s_i` to one Knapsack item with `weight_i = value_i = s_i`, cast from `u64` to `i64`, and set capacity to `floor(total_sum / 2)` |
| 4 | Solution extraction | Identity on the binary configuration vector: selected knapsack items are exactly the chosen partition subset |
| 5 | Correctness argument | A balanced partition exists iff the constructed knapsack optimum reaches `total_sum / 2`; when `weight = value`, maximizing value under capacity `floor(total_sum / 2)` is equivalent to finding the largest achievable subset sum up to that bound |
| 6 | Size overhead | `num_items = "num_elements"` |
| 7 | Concrete example | Partition sizes `[3, 1, 1, 2, 2, 1]` map to a Knapsack instance with identical weights/values and capacity `5`; witness config `[1, 0, 0, 1, 0, 0]` works on both sides |
| 8 | Solving strategy | Solve the target with `BruteForce::find_all_best()` / `find_best()`; use `assert_satisfaction_round_trip_from_optimization_target` on satisfiable instances |
| 9 | Reference | Garey & Johnson, *Computers and Intractability* (1979), Appendix A6 / MP9 (“Transformation from PARTITION”) |

## Design Decisions

### Problem-type bridge

`Partition` is a satisfaction problem while `Knapsack` is an optimization problem in this codebase. The rule will therefore interpret the target optimum semantically: a source instance is satisfiable exactly when the best knapsack value is `total_sum / 2`. The reduction itself still stores only the target instance plus the identity config mapping.

### Numeric conversion

`Partition` stores sizes as `u64`, but `Knapsack` uses `i64`. The reduction should use explicit checked casts (`i64::try_from`) with a rule-specific panic message so overflow is caught deterministically instead of silently truncating.

### Overhead scope

Keep the `#[reduction(overhead = ...)]` metadata to the stable target getter that matters here: `num_items = "num_elements"`. `Knapsack::num_slack_bits()` depends on capacity with a zero-capacity special case, so forcing it into the symbolic overhead expression would complicate the rule without improving the reduction graph.

### Repo-specific example registration

Although the generic `add-rule` skill mentions `src/example_db/rule_builders.rs`, this repo’s current pattern is to place `canonical_rule_example_specs()` in the rule module itself and aggregate it from `src/rules/mod.rs`. Follow the in-repo pattern, not the older wording.

## Execution Batches

### Batch 1: Implement the rule and regenerate exported data

#### Step 1: Add the rule module (`src/rules/partition_knapsack.rs`)

1. Create `ReductionPartitionToKnapsack` with a `target: Knapsack` field.
2. Add a small checked-cast helper for `u64 -> i64` with an explicit panic string such as `"Partition -> Knapsack requires all sizes and total_sum / 2 to fit in i64"`.
3. Implement `ReductionResult`:
   - `target_problem()` returns the stored `Knapsack`.
   - `extract_solution()` returns `target_solution.to_vec()` because both problems use the same binary selection vector.
4. Implement `ReduceTo<Knapsack> for Partition` with:
   - `weights = sizes.map(cast)`
   - `values = weights.clone()`
   - `capacity = cast(total_sum / 2)`
   - `#[reduction(overhead = { num_items = "num_elements" })]`
5. Add `#[cfg(feature = "example-db")] canonical_rule_example_specs()` in this file using the issue example and a single canonical witness.
6. Link the unit tests with `#[cfg(test)] #[path = "../unit_tests/rules/partition_knapsack.rs"] mod tests;`

#### Step 2: Register the rule (`src/rules/mod.rs`)

1. Add `pub(crate) mod partition_knapsack;`
2. Extend `canonical_rule_example_specs()` with `partition_knapsack::canonical_rule_example_specs()`

#### Step 3: Add unit tests (`src/unit_tests/rules/partition_knapsack.rs`)

1. Add a satisfiable closed-loop test using `Partition::new(vec![3, 1, 1, 2, 2, 1])` and `assert_satisfaction_round_trip_from_optimization_target(...)`.
2. Add a structure test that checks:
   - target weights equal `[3, 1, 1, 2, 2, 1]`
   - target values equal `[3, 1, 1, 2, 2, 1]`
   - target capacity equals `5`
   - `num_items()` matches `num_elements()`
3. Add an unsatisfiable/odd-total test (for example `[2, 4, 5]`) that:
   - solves the target with `BruteForce::find_best()`
   - extracts the source config
   - asserts `source.evaluate(&extracted)` is `false`
4. Add a checked-cast panic test with a value larger than `i64::MAX`.

#### Step 4: Regenerate the exports needed by the paper

Run these after the rule and canonical example compile:

```bash
cargo run --features "example-db" --example export_examples
cargo run --example export_graph
cargo run --example export_schemas
```

These commands must succeed before the paper step so `load-example("Partition", "Knapsack")` has backing JSON data.

### Batch 2: Write the paper entry and run final verification

#### Step 5: Document the rule in `docs/paper/reductions.typ`

1. Add a `#let partition_knapsack = load-example("Partition", "Knapsack")` block near the other reduction examples.
2. Add a `#reduction-rule("Partition", "Knapsack", ...)` entry that:
   - cites Garey & Johnson for the textbook reduction
   - states that the reduction uses one knapsack item per partition element
   - explains the satisfaction-to-optimization interpretation explicitly
3. In the proof body, include:
   - `_Construction._` with `w_i = v_i = s_i` and `C = floor(sum_i s_i / 2)`
   - `_Correctness._` proving `Partition` is satisfiable iff the optimum knapsack value is exactly `sum_i s_i / 2`
   - `_Solution extraction._` explaining the identity mapping back to the partition subset
4. In the `extra:` tutorial block, use the canonical example JSON data rather than hardcoded values:
   - `pred create --example Partition -o partition.json`
   - `pred reduce partition.json --to "Knapsack" -o bundle.json`
   - `pred solve bundle.json`
   - `pred evaluate partition.json --config ...`
5. Note witness semantics clearly: the fixture stores one canonical balanced subset even if multiple optimal knapsack witnesses exist.

#### Step 6: Verify the full change

Run:

```bash
make fmt-check
make clippy
make test
make paper
```

If `make paper` regenerates ignored outputs, leave ignored/generated files unstaged unless the repo already tracks them. The required committed changes are the rule/test/source files plus any tracked export fixture updates produced by the commands above.
