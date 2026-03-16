# Plan: [Model] TwoDimensionalConsecutiveSets (#422)

## Problem Summary

**Name:** `TwoDimensionalConsecutiveSets`
**Canonical name:** 2-DIMENSIONAL CONSECUTIVE SETS (Garey & Johnson A4 SR19)
**Type:** Satisfaction (Metric = bool)
**Category:** `set`
**Type parameters:** None
**Associated rule:** #437 (Graph 3-Colorability → 2-Dimensional Consecutive Sets)

**Definition:** Given a finite alphabet Σ = {0, ..., alphabet_size-1} and a collection C = {Σ₁, ..., Σₙ} of subsets of Σ, determine whether Σ can be partitioned into disjoint sets X₁, X₂, ..., Xₖ such that:
1. Each Xᵢ has at most one element in common with each Σⱼ (intersection constraint)
2. For each Σⱼ ∈ C, there exists an index l(j) such that Σⱼ ⊆ X_{l(j)} ∪ X_{l(j)+1} ∪ ... ∪ X_{l(j)+|Σⱼ|-1} (consecutiveness constraint)

**Fields:** `alphabet_size: usize`, `subsets: Vec<Vec<usize>>`
**Getters:** `alphabet_size()`, `num_subsets()`, `subsets()`
**Complexity:** `alphabet_size^alphabet_size` (brute force)
**Reference:** Lipski Jr. (1977), "Two NP-complete problems related to information retrieval," FCT 1977, LNCS 56

## Batch 1: Implementation (Steps 1–5.5)

### Task 1.1: Create model file `src/models/set/two_dimensional_consecutive_sets.rs`

Follow `ExactCoverBy3Sets` pattern:

1. `inventory::submit!` with `ProblemSchemaEntry`:
   - name: "TwoDimensionalConsecutiveSets"
   - display_name: "2-Dimensional Consecutive Sets"
   - aliases: &[]
   - dimensions: &[]
   - fields: alphabet_size (usize), subsets (Vec<Vec<usize>>)

2. Struct:
   ```rust
   #[derive(Debug, Clone, Serialize, Deserialize)]
   pub struct TwoDimensionalConsecutiveSets {
       alphabet_size: usize,
       subsets: Vec<Vec<usize>>,
   }
   ```

3. Constructor `new(alphabet_size, subsets)`:
   - Validate: alphabet_size > 0
   - Validate: all elements in subsets are < alphabet_size
   - Validate: no duplicate elements within a subset
   - Sort each subset

4. Getters: `alphabet_size()`, `num_subsets()`, `subsets()`

5. `Problem` impl:
   - `NAME = "TwoDimensionalConsecutiveSets"`
   - `Metric = bool`
   - `dims()` = `vec![alphabet_size; alphabet_size]` — each of the `alphabet_size` symbols is assigned to a group index in {0, ..., alphabet_size-1}
   - `evaluate(config)`:
     - Check config length == alphabet_size
     - Check all values < alphabet_size
     - Build partition: for each group index g, collect symbols assigned to g
     - For each subset Σⱼ:
       - Check intersection constraint: each group has at most 1 element from Σⱼ
       - Check consecutiveness: the group indices of Σⱼ's elements form a contiguous range of length |Σⱼ|
     - Return true iff all subsets pass both checks
   - `variant()` = `crate::variant_params![]`

6. `SatisfactionProblem` impl (marker)

7. `declare_variants!`:
   ```rust
   crate::declare_variants! {
       default sat TwoDimensionalConsecutiveSets => "alphabet_size^alphabet_size",
   }
   ```

8. Canonical example (feature-gated `example-db`):
   - Use the YES instance from the issue: alphabet_size=6, subsets=[[0,1,2],[3,4,5],[1,3],[2,4],[0,5]]
   - Sample config: the known valid partition [0, 1, 1, 2, 3, 1] (or whatever maps to X₁={0}, X₂={1,5}, X₃={2,3}, X₄={4})

9. `#[cfg(test)] #[path]` link to test file

### Task 1.2: Register module in `src/models/set/mod.rs`

- Add `pub(crate) mod two_dimensional_consecutive_sets;`
- Add `pub use two_dimensional_consecutive_sets::TwoDimensionalConsecutiveSets;`
- Add `specs.extend(two_dimensional_consecutive_sets::canonical_model_example_specs());` in `canonical_model_example_specs()`

### Task 1.3: Re-export in `src/models/mod.rs`

Add `TwoDimensionalConsecutiveSets` to the `pub use set::` line.

### Task 1.4: CLI discovery — `problemreductions-cli/src/problem_name.rs`

Alias resolution is now registry-backed. No manual alias needed (no well-known abbreviation). The `ProblemSchemaEntry` handles it.

### Task 1.5: CLI create support — `problemreductions-cli/src/commands/create.rs`

Add a match arm for `"TwoDimensionalConsecutiveSets"`:
- Parse `--universe` (alphabet_size) and `--sets` (subsets)
- Construct `TwoDimensionalConsecutiveSets::new(universe, sets)`
- Add example hint in `example_hint()`
- Pattern: similar to `SetBasis` arm but without `--k`

### Task 1.6: CLI help table — `problemreductions-cli/src/cli.rs`

Add entry:
```
TwoDimensionalConsecutiveSets   --universe, --sets
```
Add example:
```
pred create TwoDimensionalConsecutiveSets --universe 6 --sets "0,1,2;3,4,5;1,3;2,4;0,5"
```

### Task 1.7: Create test file `src/unit_tests/models/set/two_dimensional_consecutive_sets.rs`

Tests:
- `test_two_dimensional_consecutive_sets_creation` — constructor, dims, num_variables
- `test_two_dimensional_consecutive_sets_evaluation` — YES and NO configs
- `test_two_dimensional_consecutive_sets_no_instance` — unsatisfiable instance (from issue Example 2)
- `test_two_dimensional_consecutive_sets_solver` — BruteForce finds satisfying solutions
- `test_two_dimensional_consecutive_sets_serialization` — JSON round-trip
- `test_two_dimensional_consecutive_sets_paper_example` — verify paper instance (written after Step 6)
- Panic tests: out-of-range elements, duplicate elements, zero alphabet_size

### Task 1.8: Add trait_consistency entry

In `src/unit_tests/trait_consistency.rs`:
- Add `check_problem_trait(&TwoDimensionalConsecutiveSets::new(...), "TwoDimensionalConsecutiveSets")` with a small instance

### Task 1.9: Verify

```bash
make test clippy
```

## Batch 2: Paper Entry (Step 6)

### Task 2.1: Add display name in `docs/paper/reductions.typ`

```typst
"TwoDimensionalConsecutiveSets": [2-Dimensional Consecutive Sets],
```

### Task 2.2: Write problem-def entry

```typst
#problem-def("TwoDimensionalConsecutiveSets")[
  Given a finite alphabet $Sigma = {0, 1, ..., n-1}$ and a collection $cal(C) = {Sigma_1, ..., Sigma_m}$ of subsets of $Sigma$, determine whether $Sigma$ can be partitioned into disjoint sets $X_1, X_2, ..., X_k$ such that each $X_i$ has at most one element in common with each $Sigma_j$, and for each $Sigma_j in cal(C)$, there is an index $l(j)$ such that $Sigma_j subset.eq X_(l(j)) union X_(l(j)+1) union dots union X_(l(j)+|Sigma_j|-1)$.
][
  Background, example with CeTZ diagram, evaluation.
]
```

### Task 2.3: Build and verify

```bash
make paper
```
