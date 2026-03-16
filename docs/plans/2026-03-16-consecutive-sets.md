# Plan: Add ConsecutiveSets Model (#421)

## Context

**ConsecutiveSets** (Garey & Johnson A4 SR18): Given finite alphabet Σ (size `alphabet_size`), collection C = {Σ₁, ..., Σₙ} of subsets of Σ, and positive integer K (`bound_k`), decide if there exists a string w ∈ Σ* with |w| ≤ K such that for each i, the elements of Σᵢ occur in a consecutive block of |Σᵢ| symbols of w.

- **Type:** Satisfaction problem (Metric = bool)
- **NP-completeness:** Kou 1977, reduction from Hamiltonian Path
- **Category:** `set/` (universe + subsets input structure)
- **Complexity:** `"alphabet_size^bound_k * num_subsets"` (brute-force)

## Batch 1: Implementation (Steps 1–5.5)

### Task 1: Create model file `src/models/set/consecutive_sets.rs`

**Struct:**
```rust
pub struct ConsecutiveSets {
    alphabet_size: usize,
    subsets: Vec<Vec<usize>>,
    bound_k: usize,
}
```

**Constructor** `new(alphabet_size, subsets, bound_k)`:
- Validate: all subset elements < alphabet_size
- Validate: bound_k > 0
- Validate: no duplicate elements within individual subsets
- Sort each subset for canonical form

**Getters:** `alphabet_size()`, `num_subsets()`, `bound_k()`, `subsets()`

**Problem impl:**
- `NAME = "ConsecutiveSets"`
- `type Metric = bool`
- `dims()`: `vec![alphabet_size + 1; bound_k]` — values 0..alphabet_size-1 are symbols, alphabet_size = "unused"
- `evaluate(config)`:
  1. Validate config length == bound_k and all values <= alphabet_size
  2. Build string w: take symbols until trailing "unused" values; reject if any internal "unused"
  3. For each subset Σᵢ: scan all windows of length |Σᵢ| in w, check if window contains exactly the elements of Σᵢ (as a set). If no valid window found for any subset, return false.
  4. Return true if all subsets satisfied.
- `variant()`: `variant_params![]`

**SatisfactionProblem** marker trait impl.

**declare_variants!:**
```rust
default sat ConsecutiveSets => "alphabet_size^bound_k * num_subsets",
```

**ProblemSchemaEntry** with fields: `alphabet_size` (usize), `subsets` (Vec<Vec<usize>>), `bound_k` (usize).

**Canonical example spec** using YES instance from issue: alphabet_size=6, subsets=[{0,4},{2,4},{2,5},{1,5},{1,3}], bound_k=6, solution=[0,4,2,5,1,3].

### Task 2: Register model

- `src/models/set/mod.rs`: Add `pub(crate) mod consecutive_sets;`, `pub use`, and extend `canonical_model_example_specs()`
- `src/models/mod.rs`: Add `ConsecutiveSets` to `pub use set::` re-export

### Task 3: Write unit tests `src/unit_tests/models/set/consecutive_sets.rs`

Tests (reference: `exact_cover_by_3_sets.rs`):
- `test_consecutive_sets_creation` — dims, getters, num_variables
- `test_consecutive_sets_evaluation` — YES config [0,4,2,5,1,3] and NO configs
- `test_consecutive_sets_no_instance` — NO instance from issue (conflicting block constraints)
- `test_consecutive_sets_serialization` — serde round-trip
- `test_consecutive_sets_solver` — BruteForce finds satisfying configs, verify known solution present
- `test_consecutive_sets_empty` — edge case: empty subsets collection
- `test_consecutive_sets_invalid_constructor` — #[should_panic] for bad inputs

Link via `#[path]` in model file.

### Task 4: Verify build

Run `make check` (fmt + clippy + test). Fix any issues.

### Task 5: Run trait_consistency

Ensure the new model passes all trait consistency checks automatically via `declare_variants!`.

## Batch 2: Paper Entry (Step 6)

### Task 6: Write paper entry in `docs/paper/reductions.typ`

- Add `"ConsecutiveSets": [Consecutive Sets]` to `display-name` dict
- Add `#problem-def("ConsecutiveSets")[definition][body]` with:
  - Formal definition from Garey & Johnson
  - NP-completeness reference (Kou 1977)
  - Connection to consecutive ones property
  - Circular variant mention (Booth 1975)
- Run `make paper` to verify compilation
