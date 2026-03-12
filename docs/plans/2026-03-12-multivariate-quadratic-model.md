# Plan: Add MultivariateQuadratic Model (#129)

## Overview
Add the Multivariate Quadratic (MQ) satisfaction problem — a system of quadratic polynomial equations over finite field F_q. This is a core problem in post-quantum cryptography.

## Key Design Decisions
- **Category**: `algebraic/` — polynomial equations over finite fields are algebraic in nature
- **Problem type**: `SatisfactionProblem` (Metric = bool)
- **Domain**: Each variable ∈ F_q = {0, 1, ..., q-1}, so `dims()` = `vec![field_size; num_variables]`
- **Arithmetic**: All operations mod q (finite field)
- **No type parameters**: field_size is a runtime value, not a type parameter
- **Complexity**: `q^num_variables` (exhaustive search over F_q^n)

## Steps

### Step 1: Core Model Implementation
**File**: `src/models/algebraic/multivariate_quadratic.rs`

Implement:
- `QuadraticPoly` struct: quadratic_terms `Vec<((usize, usize), u64)>`, linear_terms `Vec<(usize, u64)>`, constant `u64`
- `MultivariateQuadratic` struct: field_size `usize`, num_variables `usize`, equations `Vec<QuadraticPoly>`
- Constructor `new(field_size, num_variables, equations)` with validation
- Getter methods: `field_size()`, `num_variables()`, `num_equations()`, `equations()`
- `QuadraticPoly::evaluate(config, field_size) -> u64` — evaluate polynomial mod field_size
- `Problem` impl: NAME = "MultivariateQuadratic", Metric = bool, dims = vec![field_size; num_variables], evaluate checks all equations = 0 mod q
- `SatisfactionProblem` marker impl
- `declare_variants!` with complexity `"field_size^num_variables"`
- `inventory::submit!` for ProblemSchemaEntry
- Test module link

**Dependencies**: None (independent)

### Step 2: Unit Tests
**File**: `src/unit_tests/models/algebraic/multivariate_quadratic.rs`

Tests:
- `test_multivariate_quadratic_basic`: creation, getters, dims
- `test_multivariate_quadratic_evaluate_f2`: F_2 example from issue (x₀·x₁ + x₂ = 0, x₁·x₂ + x₀ = 0)
- `test_multivariate_quadratic_evaluate_no_solution`: F_2 contradictory example from issue
- `test_multivariate_quadratic_brute_force`: use BruteForce::find_satisfying and find_all_satisfying
- `test_multivariate_quadratic_serialization`: serde round-trip
- `test_multivariate_quadratic_larger_field`: test with F_3 or F_5

**Dependencies**: Step 1

### Step 3: Module Registration
**Files**:
- `src/models/algebraic/mod.rs` — add `mod multivariate_quadratic; pub use multivariate_quadratic::*;`
- `src/models/mod.rs` — add `MultivariateQuadratic` to algebraic re-exports

**Dependencies**: Step 1

### Step 4: CLI Registration
**Files**:
- `problemreductions-cli/src/dispatch.rs` — add `deser_sat::<MultivariateQuadratic>` and `try_ser::<MultivariateQuadratic>`
- `problemreductions-cli/src/problem_name.rs` — add alias "MQ" -> "MultivariateQuadratic"
- `problemreductions-cli/src/commands/create.rs` — add creation logic with --field-size, --num-vars, --equations flags

**Dependencies**: Step 3

### Step 5: Example Program
**File**: `examples/multivariate_quadratic_f2.rs`

Demonstrate the F_2 example from the issue:
- Create MQ instance with 3 variables, 2 equations over F_2
- Show evaluation on satisfying and non-satisfying configs
- Use BruteForce to find all solutions
- Print JSON serialization

**Dependencies**: Steps 1-3

### Step 6: Regenerate Exports
Run `make export-schemas` to update problem schemas JSON.

**Dependencies**: Steps 1-4

### Step 7: Verify
Run `make check` (fmt + clippy + test) to verify everything compiles and passes.

**Dependencies**: All steps
