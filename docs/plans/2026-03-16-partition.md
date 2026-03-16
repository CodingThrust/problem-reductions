# Plan: Implement Partition Model (#210)

## Context

Issue #210 adds the **Partition** problem — a classical NP-complete satisfaction problem (Karp #20, Garey & Johnson SP12). Given a set of positive integers, decide whether it can be split into two subsets of equal sum.

- **Type:** Satisfaction problem (`Metric = bool`, `SatisfactionProblem`)
- **Schema:** `Partition { sizes: Vec<u64> }` — no type parameters
- **Getter:** `num_elements()` = `sizes.len()`
- **Complexity:** `2^(num_elements / 2)` — Schroeppel–Shamir (1981) meet-in-the-middle
- **Category:** `misc` (unique input structure, closest to SubsetSum)
- **Solvers:** BruteForce (enumerate 2^n subsets) + ILP (binary vars, equality constraint)

## Batch 1: Implementation (Steps 1–5.5)

### Step 1: Create model file `src/models/misc/partition.rs`

Follow `SubsetSum` as the primary reference (same category, satisfaction, integer sizes).

- `inventory::submit!` with `ProblemSchemaEntry` — name `"Partition"`, display name `"Partition"`, aliases `&[]`, fields: `sizes` (`Vec<u64>`)
- Struct: `#[derive(Debug, Clone, Serialize, Deserialize)] pub struct Partition { sizes: Vec<u64> }`
- Constructor: `pub fn new(sizes: Vec<u64>) -> Self` — assert all sizes > 0
- Getters: `pub fn sizes(&self) -> &[u64]`, `pub fn num_elements(&self) -> usize`
- Helper: `pub fn total_sum(&self) -> u64` — sum of all sizes (useful for the half-sum check)
- `Problem` impl: `NAME = "Partition"`, `Metric = bool`, `dims = vec![2; num_elements()]`, `variant = crate::variant_params![]`
- `evaluate`: return false if config length mismatch or any value >= 2; compute sum of `sizes[i]` where `config[i] == 1`; return `sum * 2 == total_sum` (avoids integer division)
- `SatisfactionProblem` marker impl
- `declare_variants! { default sat Partition => "2^(num_elements / 2)" }`
- Test path link: `#[cfg(test)] #[path = "../../unit_tests/models/misc/partition.rs"] mod tests;`

### Step 2: Register in module hierarchy

- `src/models/misc/mod.rs`: add `mod partition;` and `pub use partition::Partition;`
- `src/models/mod.rs`: add `Partition` to the `pub use misc::{...}` line

### Step 3: Add canonical example to `src/example_db/`

In the `Partition` model file, add a `canonical_model_example_specs()` function (gated on `example-db` feature):
- Example instance: `Partition::new(vec![3, 1, 1, 2, 2, 1])` — the issue's example (n=6, total=10, target half=5)
- Sample configs: `vec![1, 0, 0, 1, 0, 0]` (A'={3,2}, sum=5, satisfying) and `vec![0, 0, 0, 0, 0, 0]` (sum=0, not satisfying)
- Use `satisfaction_example()` helper

In `src/models/misc/mod.rs`: add `specs.extend(partition::canonical_model_example_specs());`

### Step 4: Write unit tests `src/unit_tests/models/misc/partition.rs`

Tests (>95% coverage):
- `test_partition_basic`: create instance, check `num_elements()`, `sizes()`, `total_sum()`, `dims()`
- `test_partition_evaluate_satisfying`: evaluate a known satisfying config
- `test_partition_evaluate_unsatisfying`: evaluate a non-satisfying config
- `test_partition_evaluate_wrong_length`: config with wrong length returns false
- `test_partition_evaluate_invalid_value`: config with value >= 2 returns false
- `test_partition_odd_total`: instance with odd total sum — no satisfying assignment exists
- `test_partition_solver`: BruteForce `find_satisfying` on satisfiable instance
- `test_partition_solver_all`: BruteForce `find_all_satisfying`, verify all solutions evaluate to true
- `test_partition_unsatisfiable`: instance with no solution (odd total or no equal split)
- `test_partition_serialization`: serde round-trip
- `test_partition_single_element`: edge case with 1 element (impossible to partition)
- `test_partition_two_elements`: edge case with 2 equal elements

### Step 5: Add trait_consistency entry

In `src/unit_tests/trait_consistency.rs`: add `check_problem_trait(&Partition::new(vec![3, 1, 1, 2, 2, 1]), "Partition");`

## Batch 2: Paper Entry (Step 6)

### Step 6: Write paper entry in `docs/paper/reductions.typ`

- Add `"Partition": [Partition]` to `display-name` dict
- Add `#problem-def("Partition")[def][body]`:
  - **Definition:** Given a finite set A with sizes s(a) ∈ Z⁺, determine whether ∃ A' ⊆ A such that Σ_{a∈A'} s(a) = Σ_{a∈A\A'} s(a)
  - **Body:** Karp #20, GJ SP12, weakly NP-hard (pseudo-polynomial DP in O(n·B)), best exact algorithm is O*(2^{n/2}) meet-in-the-middle. Related to SubsetSum (special case with target = total/2)
  - **Example:** A = {3,1,1,2,2,1}, partition A'={3,2} vs A\A'={1,1,2,1}
- Place after SubsetSum in the paper (related problems grouped together)
- Run `make paper` to verify compilation
- Run `make export-schemas` to regenerate JSON exports
