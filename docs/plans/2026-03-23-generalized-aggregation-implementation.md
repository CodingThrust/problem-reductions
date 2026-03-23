# Generalized Aggregation Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Replace the current optimization/satisfaction split with aggregate values, preserve witness workflows where they are meaningful, and make reduction-path search capability-aware via `EdgeCapabilities`.

**Architecture:** Implement the refactor in layers. First add aggregate primitives and generic solver semantics, then split dynamic value solve from witness solve, then extend the reduction graph with aggregate reductions and capability-aware path search. After the infrastructure is stable, migrate the existing model/test surface mechanically from `Metric` to `Value`.

**Tech Stack:** Rust, inventory-based registry dispatch, proc macros in `problemreductions-macros`, petgraph-based reduction graph, Cargo tests, repository `make` targets.

---

### Task 1: Add aggregate core types

**Files:**
- Modify: `src/types.rs`
- Test: `src/unit_tests/types.rs`

**Step 1: Write the failing tests**

Add focused tests for:

- `Max::<i32>::identity()` and `combine()`
- `Min::<i32>::identity()` and `combine()`
- `Sum::<u64>::identity()` and `combine()`
- `Or::identity()` / `combine()`
- `And::identity()` / `combine()`
- witness defaults for `Sum` and `And`
- witness hooks for `Max`, `Min`, and `Or`

Example test skeleton:

```rust
#[test]
fn test_max_identity_and_combine() {
    assert_eq!(Max::<i32>::identity(), Max(None));
    assert_eq!(Max(Some(7)).combine(Max(Some(3))), Max(Some(7)));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_max_identity_and_combine --lib`
Expected: FAIL with missing `Max` / `Aggregate` definitions

**Step 3: Write minimal implementation**

In `src/types.rs`:

- add `Aggregate`
- add `Max`, `Min`, `Sum`, `Or`, `And`
- add `Display` impls for user-facing formatting
- keep witness hooks on `Aggregate` with safe defaults

**Step 4: Run tests to verify they pass**

Run: `cargo test test_max_identity_and_combine --lib`
Expected: PASS

Run: `cargo test test_sum_identity_and_combine --lib`
Expected: PASS

**Step 5: Commit**

```bash
git add src/types.rs src/unit_tests/types.rs
git commit -m "refactor: add aggregate value primitives"
```

### Task 2: Rewrite `Problem` and `BruteForce` around aggregate values

**Files:**
- Modify: `src/traits.rs`
- Modify: `src/solvers/mod.rs`
- Modify: `src/solvers/brute_force.rs`
- Test: `src/unit_tests/traits.rs`
- Test: `src/unit_tests/solvers/brute_force.rs`

**Step 1: Write the failing tests**

Update the focused trait and solver tests to use:

- `type Value`
- `Solver::solve()`
- `BruteForce::find_witness()`
- `BruteForce::find_all_witnesses()`

Example solver test skeleton:

```rust
#[test]
fn test_solver_solves_max_value() {
    let solver = BruteForce::new();
    let total = solver.solve(&problem);
    assert_eq!(total, Max(Some(6)));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_solver_solves_max_value --lib`
Expected: FAIL because `Problem::Value` / `Solver::solve` do not exist yet

**Step 3: Write minimal implementation**

In the listed files:

- rename `Metric` to `Value`
- remove `OptimizationProblem` / `SatisfactionProblem`
- make `Solver` expose only `solve`
- implement witness helpers in `BruteForce` using `Aggregate::contributes_to_witnesses`

**Step 4: Run tests to verify they pass**

Run: `cargo test test_solver_solves_max_value --lib`
Expected: PASS

Run: `cargo test test_solver_find_witness --lib`
Expected: PASS

**Step 5: Commit**

```bash
git add src/traits.rs src/solvers/mod.rs src/solvers/brute_force.rs src/unit_tests/traits.rs src/unit_tests/solvers/brute_force.rs
git commit -m "refactor: unify core solving on aggregate values"
```

### Task 3: Split dynamic value solving from dynamic witness solving

**Files:**
- Modify: `src/registry/dyn_problem.rs`
- Modify: `src/registry/variant.rs`
- Modify: `problemreductions-macros/src/lib.rs`
- Test: `src/unit_tests/registry/dispatch.rs`

**Step 1: Write the failing tests**

Add or update tests to cover:

- `LoadedDynProblem::solve_brute_force_value()`
- `LoadedDynProblem::solve_brute_force_witness()`
- witness solve returns `None` for an aggregate-only dummy problem

Example test skeleton:

```rust
#[test]
fn loaded_dyn_problem_returns_none_for_aggregate_only_witness() {
    let loaded = LoadedDynProblem::new(Box::new(problem), solve_value, solve_witness);
    assert!(loaded.solve_brute_force_witness().is_none());
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test loaded_dyn_problem_returns_none_for_aggregate_only_witness --lib`
Expected: FAIL because `solve_brute_force_value` / `solve_brute_force_witness` do not exist

**Step 3: Write minimal implementation**

In the listed files:

- replace `SolveFn` with `SolveValueFn` and `SolveWitnessFn`
- store both function pointers on `VariantEntry`
- generate both closures in `declare_variants!`
- have the generated witness closure call `BruteForce::find_witness()`

**Step 4: Run tests to verify they pass**

Run: `cargo test loaded_dyn_problem_returns_none_for_aggregate_only_witness --lib`
Expected: PASS

Run: `cargo test test_load_problem_alias_uses_registry_dispatch --lib`
Expected: PASS

**Step 5: Commit**

```bash
git add src/registry/dyn_problem.rs src/registry/variant.rs problemreductions-macros/src/lib.rs src/unit_tests/registry/dispatch.rs
git commit -m "refactor: split dynamic value solve from witness solve"
```

### Task 4: Add aggregate reduction traits and runtime chain support

**Files:**
- Modify: `src/rules/traits.rs`
- Modify: `src/rules/registry.rs`
- Modify: `src/rules/graph.rs`
- Test: `src/unit_tests/rules/traits.rs`
- Test: `src/unit_tests/rules/registry.rs`
- Test: `src/unit_tests/rules/graph.rs`

**Step 1: Write the failing tests**

Add tests for:

- `AggregateReductionResult::extract_value`
- type-erased `DynAggregateReductionResult`
- aggregate chain execution over a tiny dummy two-step path

Example skeleton:

```rust
#[test]
fn test_aggregate_reduction_chain_extracts_value_backwards() {
    assert_eq!(chain.extract_value_dyn(json!(7)), json!(3));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_aggregate_reduction_chain_extracts_value_backwards --lib`
Expected: FAIL because aggregate reduction traits and chain types do not exist

**Step 3: Write minimal implementation**

In the listed files:

- add `AggregateReductionResult`
- add `ReduceToAggregate<T>`
- add `DynAggregateReductionResult`
- add aggregate-chain execution alongside the existing witness chain

**Step 4: Run tests to verify they pass**

Run: `cargo test test_aggregate_reduction_chain_extracts_value_backwards --lib`
Expected: PASS

**Step 5: Commit**

```bash
git add src/rules/traits.rs src/rules/registry.rs src/rules/graph.rs src/unit_tests/rules/traits.rs src/unit_tests/rules/registry.rs src/unit_tests/rules/graph.rs
git commit -m "refactor: add aggregate reduction execution"
```

### Task 5: Introduce `EdgeCapabilities` and capability-aware path search

**Files:**
- Modify: `src/rules/registry.rs`
- Modify: `src/rules/graph.rs`
- Test: `src/unit_tests/reduction_graph.rs`
- Test: `src/unit_tests/rules/graph.rs`

**Step 1: Write the failing tests**

Add pathfinding tests that prove:

- witness search ignores aggregate-only edges
- aggregate search ignores witness-only edges
- natural subtype edges remain usable in both modes

Example skeleton:

```rust
#[test]
fn witness_path_search_rejects_aggregate_only_edge() {
    assert!(graph.find_cheapest_path(..., ReductionMode::Witness, ...).is_none());
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test witness_path_search_rejects_aggregate_only_edge --lib`
Expected: FAIL because `EdgeCapabilities` / `ReductionMode` do not exist

**Step 3: Write minimal implementation**

In the listed files:

- add `EdgeCapabilities`
- store capabilities on `ReductionEntry` and edge data
- thread `ReductionMode` through path search and path execution
- mark natural subtype edges as `{ witness: true, aggregate: true }`

**Step 4: Run tests to verify they pass**

Run: `cargo test witness_path_search_rejects_aggregate_only_edge --lib`
Expected: PASS

Run: `cargo test natural_edge_supports_both_modes --lib`
Expected: PASS

**Step 5: Commit**

```bash
git add src/rules/registry.rs src/rules/graph.rs src/unit_tests/reduction_graph.rs src/unit_tests/rules/graph.rs
git commit -m "refactor: make reduction paths capability-aware"
```

### Task 6: Update CLI solve/reduce flows and ILP gating

**Files:**
- Modify: `problemreductions-cli/src/dispatch.rs`
- Modify: `problemreductions-cli/src/commands/solve.rs`
- Modify: `problemreductions-cli/src/commands/reduce.rs`
- Modify: `problemreductions-cli/src/mcp/tools.rs`
- Modify: `src/solvers/ilp/solver.rs`
- Test: `problemreductions-cli/tests/cli_tests.rs`
- Test: `problemreductions-cli/src/mcp/tests.rs`

**Step 1: Write the failing tests**

Add tests for:

- plain `pred solve` on a value-only problem prints evaluation without `Solution:`
- bundle solve rejects aggregate-only paths
- ILP solve rejects aggregate-only source problems with a clear error

Example CLI assertion skeleton:

```rust
assert!(stdout.contains("Evaluation: Sum("));
assert!(!stdout.contains("Solution:"));
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_solve_value_only_problem_omits_solution --package problemreductions-cli`
Expected: FAIL because CLI still assumes every solve returns a config

**Step 3: Write minimal implementation**

In the listed files:

- use `solve_brute_force_value()` for plain problem solves
- print `Solution` only when `solve_brute_force_witness()` succeeds
- make `pred reduce` / bundle solve witness-only
- keep ILP witness-only and improve the user-facing error

**Step 4: Run tests to verify they pass**

Run: `cargo test test_solve_value_only_problem_omits_solution --package problemreductions-cli`
Expected: PASS

Run: `cargo test test_solve_bundle --package problemreductions-cli`
Expected: PASS

**Step 5: Commit**

```bash
git add problemreductions-cli/src/dispatch.rs problemreductions-cli/src/commands/solve.rs problemreductions-cli/src/commands/reduce.rs problemreductions-cli/src/mcp/tools.rs src/solvers/ilp/solver.rs problemreductions-cli/tests/cli_tests.rs problemreductions-cli/src/mcp/tests.rs
git commit -m "refactor: separate value solve from witness workflows"
```

### Task 7: Mechanically migrate existing models, rules, and tests

**Files:**
- Modify: files returned by `rg -l 'type Metric|OptimizationProblem|SatisfactionProblem|SolutionSize|Direction|find_best|find_satisfying|find_all_best|find_all_satisfying' src tests problemreductions-cli`

**Step 1: Write the failing tests**

Pick one optimization file and one satisfaction file from the `rg` output first and update their adjacent tests before doing the bulk migration.

Suggested first pair:

- `src/models/graph/maximum_independent_set.rs`
- `src/models/formula/sat.rs`

Update their direct test files first:

- `src/unit_tests/models/graph/maximum_independent_set.rs`
- `src/unit_tests/models/formula/sat.rs`

**Step 2: Run test to verify it fails**

Run: `cargo test maximum_independent_set --lib`
Expected: FAIL until the model/test pair is migrated

Run: `cargo test formula::sat --lib`
Expected: FAIL until the model/test pair is migrated

**Step 3: Write minimal implementation**

For each migrated file:

- `type Metric = ...` -> `type Value = ...`
- `SolutionSize::Valid(x)` -> `Max(Some(x))` or `Min(Some(x))`
- `bool` satisfaction outputs -> `Or(...)`
- `find_best` / `find_all_best` -> `find_witness` / `find_all_witnesses` where appropriate
- `find_satisfying` / `find_all_satisfying` -> same witness helpers

Then repeat the same mechanical transformation across the remaining `rg` result set in small reviewable batches.

**Step 4: Run tests to verify they pass**

Run: `cargo test maximum_independent_set --lib`
Expected: PASS

Run: `cargo test sat --lib`
Expected: PASS

After each batch:

Run: `make test`
Expected: PASS

**Step 5: Commit**

```bash
git add src tests problemreductions-cli
git commit -m "refactor: migrate models and tests to aggregate values"
```

### Task 8: Final verification and cleanup

**Files:**
- Modify: any stragglers found by verification commands
- Verify: `docs/plans/2026-03-22-generalized-aggregation-design.md`

**Step 1: Write the failing checks**

Use grep-style sweeps to confirm the removed surface is really gone:

```bash
rg 'OptimizationProblem|SatisfactionProblem|SolutionSize|Direction|type Metric' src tests problemreductions-cli
```

**Step 2: Run checks to verify they fail before cleanup**

Run the command above before the last cleanup pass.
Expected: remaining hits identify unfinished migration work

**Step 3: Write minimal cleanup**

- remove the final stale references
- align docs/comments/examples with the new witness/value vocabulary
- verify graph export includes both `witness` and `aggregate` booleans

**Step 4: Run verification to verify it passes**

Run:

```bash
make fmt-check
make test
make clippy
rg 'OptimizationProblem|SatisfactionProblem|SolutionSize|Direction|type Metric' src tests problemreductions-cli
```

Expected:

- `make fmt-check` passes
- `make test` passes
- `make clippy` passes
- final `rg` finds no remaining production-code references

**Step 5: Commit**

```bash
git add src tests problemreductions-cli docs/plans/2026-03-22-generalized-aggregation-design.md
git commit -m "refactor: finish generalized aggregation migration"
```
