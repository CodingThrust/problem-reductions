# Plan: KSatisfiability to SubsetSum Reduction (#126)

## Overview

Add the classical Karp reduction from 3-SAT (KSatisfiability<K3>) to SubsetSum. This requires first implementing the SubsetSum model (since it doesn't exist in the codebase), then the reduction rule.

**Source:** KSatisfiability<K3> (3-SAT) — satisfaction problem
**Target:** SubsetSum — satisfaction problem (does a subset sum to exactly B?)
**Reference:** Karp 1972; Sipser Theorem 7.56; CLRS §34.5.5

## Phase 1: SubsetSum Model (add-model)

### Step 1: Create `src/models/misc/subset_sum.rs`

SubsetSum is a **satisfaction problem** (Metric = bool).

```rust
struct SubsetSum {
    sizes: Vec<i64>,   // positive integer sizes s(a) for each element
    target: i64,       // target sum B
}
```

- `Problem` impl: `NAME = "SubsetSum"`, `Metric = bool`, `dims = vec![2; n]`
- `evaluate`: check if selected elements sum to exactly `target`
- `SatisfactionProblem` impl (marker trait)
- Getters: `num_elements()`, `sizes()`, `target()`
- `declare_variants!`: `SubsetSum => "2^(num_elements / 2)"` (Horowitz-Sahni meet-in-the-middle)
- `inventory::submit!` with `ProblemSchemaEntry`
- `#[cfg(test)] #[path]` link to unit tests

### Step 2: Register SubsetSum

- `src/models/misc/mod.rs`: add `mod subset_sum; pub use subset_sum::SubsetSum;`
- `src/models/mod.rs`: add `SubsetSum` to `pub use misc::{...}`
- `src/lib.rs`: add `SubsetSum` to prelude re-export

### Step 3: SubsetSum Unit Tests

Create `src/unit_tests/models/misc/subset_sum.rs`:
- `test_subsetsum_basic`: construct instance, verify dims, evaluate satisfying/unsatisfying configs
- `test_subsetsum_serialization`: serde round-trip
- `test_subsetsum_solver`: BruteForce finds satisfying assignment

### Step 4: CLI Registration

- `problemreductions-cli/src/dispatch.rs`: add `"SubsetSum" => deser_sat::<SubsetSum>(data)` to `load_problem()` and `"SubsetSum" => try_ser::<SubsetSum>(any)` to `serialize_any_problem()`
- Add import: `use problemreductions::models::misc::SubsetSum;` (update existing import line)
- `problemreductions-cli/src/problem_name.rs`: add `"subsetsum" => "SubsetSum"` to `resolve_alias()`
- CLI help table: add SubsetSum row to the problems table

## Phase 2: KSatisfiability<K3> → SubsetSum Reduction Rule (add-rule)

### Step 5: Create `src/rules/ksatisfiability_subsetsum.rs`

Reduction algorithm (base-10 digit encoding):
1. For each variable x_i (i=1..n), create two integers y_i, z_i with (n+m) digits:
   - y_i: digit i = 1, digit n+j = 1 if x_i ∈ C_j
   - z_i: digit i = 1, digit n+j = 1 if ¬x_i ∈ C_j
2. For each clause C_j (j=1..m), create two slack integers:
   - g_j: digit n+j = 1
   - h_j: digit n+j = 2
3. Target T: digits 1..n = 1, digits n+1..n+m = 4

**ReductionResult struct:** `Reduction3SATToSubsetSum`
- `target: SubsetSum`
- `source_num_vars: usize`
- `extract_solution`: for each i, if y_i is selected → x_i = 1, if z_i is selected → x_i = 0

**Overhead:**
```rust
#[reduction(overhead = {
    num_elements = "2 * num_vars + 2 * num_clauses",
})]
```

### Step 6: Register Rule

- `src/rules/mod.rs`: add `mod ksatisfiability_subsetsum;`

### Step 7: Rule Unit Tests

Create `src/unit_tests/rules/ksatisfiability_subsetsum.rs`:
- `test_ksatisfiability_to_subsetsum_closed_loop`: 3 vars, 2 clauses (issue example), verify satisfying assignment maps correctly
- `test_ksatisfiability_to_subsetsum_unsatisfiable`: verify unsatisfiable formula yields no SubsetSum solution
- `test_ksatisfiability_to_subsetsum_single_clause`: single clause, verify all 7/8 satisfying assignments work
- `test_ksatisfiability_to_subsetsum_structure`: verify num_elements = 2n + 2m

### Step 8: Example Program

Create `examples/reduction_ksatisfiability_to_subsetsum.rs`:
- Use the issue's example: (x₁∨x₂∨x₃) ∧ (¬x₁∨¬x₂∨x₃), n=3, m=2
- Reduce, solve with BruteForce, extract solution, verify
- Must have `pub fn run()` + `fn main() { run() }`

### Step 9: Register Example in Tests

- `tests/suites/examples.rs`: add `example_test!` and `example_fn!` entries

## Phase 3: Paper Entries

### Step 10: Paper Updates

In `docs/paper/reductions.typ`:
- Add `"SubsetSum": [Subset Sum]` to `display-name` dict
- Add `#problem-def("SubsetSum")[...]` with formal definition
- Add `#reduction-rule("KSatisfiability", "SubsetSum", example: true, ...)` with proof and example

## Phase 4: Verification

### Step 11: Build and Test

- `make fmt` — format code
- `make clippy` — lint
- `make test` — run all tests
- `make export-schemas` — regenerate schemas JSON
- Run example: `cargo run --example reduction_ksatisfiability_to_subsetsum`

## Parallelization

**Independent tasks (can run in parallel):**
- Phase 1 Steps 1-4 (model) — must complete before Phase 2
- Phase 2 Steps 5-9 (rule) — depends on Phase 1
- Phase 3 Step 10 (paper) — can run in parallel with Phase 2

**Sequential dependencies:**
- Step 1 → Step 2 → Step 3 → Step 4 (model pipeline)
- Step 5 → Step 6 → Step 7 → Step 8 → Step 9 (rule pipeline, depends on model)
- Step 11 (verification) runs last
