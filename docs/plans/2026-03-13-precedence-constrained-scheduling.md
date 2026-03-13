# Plan: Add PrecedenceConstrainedScheduling Model (#501)

## Overview

Add the Precedence Constrained Scheduling problem — a satisfaction problem from Garey & Johnson (A5 SS9). Given unit-length tasks with precedence constraints, m processors, and a deadline D, determine whether all tasks can be scheduled to meet D while respecting precedences.

This is a satisfaction problem (`Metric = bool`). No type parameters (all unit-length tasks, integer parameters).

## Steps

### Step 1: Create Model File

**File:** `src/models/misc/precedence_constrained_scheduling.rs`

**Struct:**
```rust
pub struct PrecedenceConstrainedScheduling {
    num_tasks: usize,
    num_processors: usize,
    deadline: usize,
    precedences: Vec<(usize, usize)>,
}
```

**Constructor:** `new(num_tasks, num_processors, deadline, precedences)` — validate that precedence indices are within bounds.

**Getters:** `num_tasks()`, `num_processors()`, `deadline()`, `precedences()`.

**Problem impl:**
- `NAME = "PrecedenceConstrainedScheduling"`
- `type Metric = bool`
- `dims()`: Each task is assigned a time slot in `{0, ..., deadline-1}`, so `vec![deadline; num_tasks]`
- `evaluate(config)`: Check (1) config length == num_tasks, (2) all values < deadline, (3) at most `num_processors` tasks per time slot, (4) for each precedence (i, j): config[j] >= config[i] + 1. Return true iff all constraints satisfied.
- `variant()`: `crate::variant_params![]`

**SatisfactionProblem:** empty impl.

**Complexity:** `declare_variants!{ PrecedenceConstrainedScheduling => "deadline ^ num_tasks" }` — brute force bound D^n.

**Schema registration:** `inventory::submit!` with fields: num_tasks, num_processors, deadline, precedences.

### Step 2: Register Model

**`src/models/misc/mod.rs`:** Add `mod precedence_constrained_scheduling;` and `pub use`.

**`src/models/mod.rs`:** Add to `misc::` re-export line.

### Step 3: CLI Registration

**`problemreductions-cli/src/dispatch.rs`:**
- `load_problem`: Add `"PrecedenceConstrainedScheduling" => deser_sat::<PrecedenceConstrainedScheduling>(data),`
- `serialize_any_problem`: Add `"PrecedenceConstrainedScheduling" => try_ser::<PrecedenceConstrainedScheduling>(any),`
- Add import for `PrecedenceConstrainedScheduling` from `problemreductions::models::misc`

**`problemreductions-cli/src/problem_name.rs`:**
- `resolve_alias`: Add `"precedenceconstrainedscheduling" => "PrecedenceConstrainedScheduling".to_string(),`

### Step 4: Unit Tests

**File:** `src/unit_tests/models/misc/precedence_constrained_scheduling.rs`

Tests (following SubsetSum pattern):
1. `test_precedence_constrained_scheduling_basic` — verify construction, getters, dims, NAME, variant
2. `test_precedence_constrained_scheduling_evaluate_valid` — valid schedule from issue example (8 tasks, 3 processors, deadline 4)
3. `test_precedence_constrained_scheduling_evaluate_invalid_precedence` — schedule violating a precedence constraint
4. `test_precedence_constrained_scheduling_evaluate_invalid_capacity` — schedule with too many tasks in one slot
5. `test_precedence_constrained_scheduling_evaluate_wrong_config_length` — wrong-length config
6. `test_precedence_constrained_scheduling_evaluate_invalid_variable_value` — value >= deadline
7. `test_precedence_constrained_scheduling_brute_force` — find_satisfying with small instance
8. `test_precedence_constrained_scheduling_brute_force_all` — find_all_satisfying
9. `test_precedence_constrained_scheduling_unsatisfiable` — instance with no valid schedule
10. `test_precedence_constrained_scheduling_serialization` — round-trip JSON
11. `test_precedence_constrained_scheduling_empty` — 0 tasks
12. `test_precedence_constrained_scheduling_no_precedences` — tasks with no ordering constraints

Link test module in model file with `#[cfg(test)] #[path = "..."] mod tests;`.

Also add entry in `src/unit_tests/models/misc/mod.rs` if it exists.

### Step 5: Verify

Run `make check` (fmt + clippy + test). Ensure all tests pass.
