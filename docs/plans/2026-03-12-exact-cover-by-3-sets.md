# Plan: Add ExactCoverBy3Sets Model

**Issue:** #491 [Model] ExactCoverBy3Sets(x3c)
**Skill:** add-model

## Summary

Add the Exact Cover by 3-Sets (X3C) problem as a satisfaction problem. X3C asks: given a universe X with |X| = 3q elements and a collection C of 3-element subsets of X, does C contain a subcollection of q disjoint triples that cover every element exactly once?

## Steps

### Step 1: Create model file `src/models/set/exact_cover_by_3_sets.rs`

- Struct `ExactCoverBy3Sets` with fields: `universe_size: usize`, `subsets: Vec<[usize; 3]>`
- Implement `Problem` with `Metric = bool` (satisfaction problem)
- Implement `SatisfactionProblem` marker trait
- Getters: `universe_size()`, `num_subsets()`, `subsets()`
- `dims()` returns `vec![2; num_subsets]` (binary: include/exclude each subset)
- `evaluate()`: check that selected subsets are pairwise disjoint and their union = X
- `variant()`: `crate::variant_params![]` (no type parameters)
- `declare_variants! { ExactCoverBy3Sets => "2^num_subsets" }`
- `inventory::submit!` for `ProblemSchemaEntry`
- `#[cfg(test)] #[path]` link to unit tests

### Step 2: Register model

- `src/models/set/mod.rs`: add `pub(crate) mod exact_cover_by_3_sets;` and `pub use`
- `src/models/mod.rs`: add `ExactCoverBy3Sets` to set re-exports

### Step 3: Register in CLI

- `problemreductions-cli/src/dispatch.rs`: add `deser_sat::<ExactCoverBy3Sets>` in `load_problem()` and `try_ser::<ExactCoverBy3Sets>` in `serialize_any_problem()`
- `problemreductions-cli/src/problem_name.rs`: add `"exactcoverby3sets" | "x3c"` alias in `resolve_alias()` and add `("X3C", "ExactCoverBy3Sets")` to `ALIASES`
- `problemreductions-cli/src/commands/create.rs`: add creation handler parsing `--universe` and `--sets` (validate each set has exactly 3 elements)
- `problemreductions-cli/src/cli.rs`: add to help table

### Step 4: Write unit tests `src/unit_tests/models/set/exact_cover_by_3_sets.rs`

- `test_exact_cover_by_3_sets_creation`: verify dimensions and accessors
- `test_exact_cover_by_3_sets_evaluation`: test valid/invalid configs
- `test_exact_cover_by_3_sets_solver`: use BruteForce to find satisfying assignments
- `test_exact_cover_by_3_sets_serialization`: round-trip serde test
- `test_exact_cover_by_3_sets_no_solution`: test unsatisfiable instance
- `test_exact_cover_by_3_sets_is_valid_solution`: test helper method

### Step 5: Add paper entry in `docs/paper/reductions.typ`

- Add display-name entry
- Add `problem-def("ExactCoverBy3Sets")` with formal definition

### Step 6: Verify

- `make fmt`
- `make clippy`
- `make test`
