# Testing Requirements

**Reference implementations — read these first:**
- **Reduction test:** `src/unit_tests/rules/minimumvertexcover_maximumindependentset.rs` — closed-loop pattern
- **Model test:** `src/unit_tests/models/graph/maximum_independent_set.rs` — evaluation, serialization
- **Solver test:** `src/unit_tests/solvers/brute_force.rs` — `find_best` + `find_satisfying`
- **Trait definitions:** `src/traits.rs` (`Problem`, `OptimizationProblem`), `src/solvers/mod.rs` (`Solver`)

## Coverage

New code must have >95% test coverage. Run `make coverage` to check.

## Naming

- Reduction tests: `test_<source>_to_<target>_closed_loop`
- Model tests: `test_<model>_basic`, `test_<model>_serialization`
- Solver tests: `test_<solver>_<problem>`

## Key Testing Patterns

Follow the reference files above for exact API usage. Summary:

- `solver.find_best(&problem)` → `Option<Vec<usize>>` — one optimal solution for optimization problems
- `solver.find_satisfying(&problem)` → `Option<Vec<usize>>` — one satisfying assignment
- `solver.find_all_best(&problem)` → `Vec<Vec<usize>>` — all optimal solutions (BruteForce only)
- `solver.find_all_satisfying(&problem)` → `Vec<Vec<usize>>` — all satisfying assignments (BruteForce only)
- `problem.evaluate(&config)` — returns `SolutionSize::Valid(value)` / `SolutionSize::Invalid` for optimization, `bool` for satisfaction

## File Organization

Unit tests live in `src/unit_tests/`, mirroring `src/` structure. Source files reference them via `#[path]`:

```rust
// In src/rules/foo_bar.rs:
#[cfg(test)]
#[path = "../unit_tests/rules/foo_bar.rs"]
mod tests;
```

Integration tests are in `tests/suites/`, consolidated through `tests/main.rs`.

## Example Tests

**Reference:** `tests/suites/examples.rs` — macro-based test harness

Example programs (`examples/reduction_*.rs`) are tested via `include!` in `tests/suites/examples.rs` — each example is compiled directly into the test binary (no subprocess overhead). Each example must expose a `pub fn run()` entry point. See any existing example (e.g., `examples/reduction_minimumvertexcover_to_maximumindependentset.rs`) for the pattern:

- `pub fn run()` with logic + `fn main() { run() }`
- Regular comments (`//`) not inner doc comments (`//!`)
- Hardcoded example name, not `env!("CARGO_BIN_NAME")`

The test harness auto-registers each example as a separate `#[test]`, so `cargo test` runs them in parallel.

## Before PR

```bash
make test clippy
```
