# Plan: Add MinimumCardinalityKey Model (Issue #444)

## Information Checklist

| # | Item | Value |
|---|------|-------|
| 1 | Problem name | `MinimumCardinalityKey` |
| 2 | Mathematical definition | Given attribute set A, functional dependencies F, and bound M, is there a candidate key K ⊆ A with \|K\| ≤ M? A key K must: (1) determine all of A under closure F*, and (2) be minimal (no proper subset also determines A). |
| 3 | Problem type | Satisfaction (`Metric = bool`) |
| 4 | Type parameters | None |
| 5 | Struct fields | `num_attributes: usize`, `dependencies: Vec<(Vec<usize>, Vec<usize>)>`, `bound_k: usize` |
| 6 | Configuration space | `vec![2; num_attributes]` — binary per attribute (1 = included in K) |
| 7 | Feasibility check | K = selected attributes; compute closure(K) under F; check closure = A, \|K\| ≤ bound_k, and K is minimal |
| 8 | Objective function | N/A (satisfaction) — returns `true` iff valid candidate key of bounded size |
| 9 | Best known exact algorithm | O(2^num_attributes) brute-force enumeration (Lucchesi & Osborn 1978) |
| 10 | Solving strategy | BruteForce (enumerate subsets, check key property) |
| 11 | Category | `set/` (attribute sets + functional dependencies) |
| 12 | Expected outcome | Instance 1 (YES): A={0..5}, F={({0,1}→{2}), ({0,2}→{3}), ({1,3}→{4}), ({2,4}→{5})}, M=2. Key {0,1} determines all of A. Instance 2 (NO): same A, F={({0,1,2}→{3}), ({3,4}→{5})}, M=2. No 2-element key exists. |

## Associated Rules

- R120: Vertex Cover → MinimumCardinalityKey (this model is target)
- R122: MinimumCardinalityKey → PrimeAttributeName (this model is source)

## Batch 1: Implementation (Steps 1–5.5)

All tasks in this batch are independent and can run in parallel.

### Task 1: Implement model + register + declare variants

**Files:** `src/models/set/minimum_cardinality_key.rs`, `src/models/set/mod.rs`, `src/models/mod.rs`

Follow `SetBasis` as the template (satisfaction problem, no type parameters, set/ category).

1. Create `src/models/set/minimum_cardinality_key.rs`:
   - `inventory::submit!` with `ProblemSchemaEntry` (name: "MinimumCardinalityKey", display_name: "Minimum Cardinality Key", aliases: &[], fields: num_attributes/usize, dependencies/Vec<(Vec<usize>, Vec<usize>)>, bound_k/usize)
   - Struct `MinimumCardinalityKey` with `#[derive(Debug, Clone, Serialize, Deserialize)]`
   - Fields: `num_attributes: usize`, `dependencies: Vec<(Vec<usize>, Vec<usize>)>`, `bound_k: usize`
   - Constructor `new(num_attributes, dependencies, bound_k)` — validate all attribute indices in dependencies are < num_attributes
   - Getter methods: `num_attributes()`, `num_dependencies()`, `bound_k()`, `dependencies()`
   - Helper: `compute_closure(attrs: &[bool]) -> Vec<bool>` — iteratively apply FDs until fixpoint
   - Helper: `is_minimal_key(config: &[usize]) -> bool` — for each selected attribute, check removing it breaks the key property
   - `Problem` impl: NAME = "MinimumCardinalityKey", Metric = bool, dims = vec![2; num_attributes], variant = variant_params![]
   - `evaluate()`: (a) K = selected attrs, (b) |K| ≤ bound_k, (c) closure(K) = A, (d) K is minimal
   - `SatisfactionProblem` impl (marker)
   - `declare_variants! { default sat MinimumCardinalityKey => "2^num_attributes" }`
   - `#[cfg(test)] #[path]` link to unit tests

2. Update `src/models/set/mod.rs`:
   - Add `pub(crate) mod minimum_cardinality_key;`
   - Add `pub use minimum_cardinality_key::MinimumCardinalityKey;`
   - Add `specs.extend(minimum_cardinality_key::canonical_model_example_specs());`

3. Update `src/models/mod.rs`:
   - Add `MinimumCardinalityKey` to the set re-export line

### Task 2: CLI registration

**Files:** `problemreductions-cli/src/problem_name.rs`, `problemreductions-cli/src/commands/create.rs`, `problemreductions-cli/src/cli.rs`

1. `problem_name.rs`: Add `"minimumcardinalitykey" => "MinimumCardinalityKey"` to `resolve_alias()`
2. `commands/create.rs`: Add match arm for "MinimumCardinalityKey" that parses `--num-attributes`, `--dependencies` (JSON array of [lhs, rhs] pairs), `--bound-k`
3. `cli.rs`: Add new CLI flags if needed (`--dependencies`, `--bound-k`) and update `CreateArgs` help table

### Task 3: Canonical example in example_db

**File:** `src/example_db/model_builders.rs` (aggregator) + `src/models/set/minimum_cardinality_key.rs` (specs function)

Add `canonical_model_example_specs()` function in the model file returning a `ModelExampleSpec` with id "minimum_cardinality_key". Use Instance 1 from the issue (num_attributes=6, the 4 FDs, bound_k=2) and a known satisfying solution [1,1,0,0,0,0] (K={0,1}).

### Task 4: Unit tests

**File:** `src/unit_tests/models/set/minimum_cardinality_key.rs`

Tests (≥3 required):
1. `test_minimum_cardinality_key_creation` — constructor, getters, dims
2. `test_minimum_cardinality_key_evaluation_yes` — Instance 1: config [1,1,0,0,0,0] evaluates to true
3. `test_minimum_cardinality_key_evaluation_no` — Instance 2: no config of size ≤2 is a key
4. `test_minimum_cardinality_key_non_minimal_rejected` — a superset of a key (e.g., {0,1,2} when {0,1} suffices) returns false
5. `test_minimum_cardinality_key_solver` — BruteForce finds all satisfying configs for Instance 1
6. `test_minimum_cardinality_key_serialization` — serde roundtrip
7. `test_minimum_cardinality_key_closure_computation` — verify closure works correctly
8. `test_minimum_cardinality_key_invalid_config` — wrong length, out-of-range values
9. `test_minimum_cardinality_key_empty_dependencies` — edge case: no FDs, only trivial keys
10. `test_minimum_cardinality_key_paper_example` — (placeholder, finalized after Step 6)

Also update `src/unit_tests/models/set/mod.rs` to include the new test module.

### Task 5: Build verification

Run `make test clippy` to verify everything compiles and passes.

## Batch 2: Paper Entry (Step 6)

Depends on Batch 1 completion (needs working model for exports).

### Task 6: Write paper entry in docs/paper/reductions.typ

1. Add display name: `"MinimumCardinalityKey": [Minimum Cardinality Key],`
2. Write `#problem-def("MinimumCardinalityKey")[def][body]`:
   - **Definition:** Given a set A of attribute names, functional dependencies F on A, and positive integer M, determine whether there exists a candidate key K ⊆ A with |K| ≤ M such that (K,A) ∈ F* and K is minimal.
   - **Body:** Background on relational database theory and Armstrong's axioms. NP-completeness via Vertex Cover (Lucchesi & Osborn 1978). Best known: brute-force O(2^|A|). CeTZ diagram showing the example instance with attributes and FD arrows.
3. Run `make paper` to verify compilation.
