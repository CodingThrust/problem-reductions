# Testing Requirements

## Coverage Requirement
New code must have >95% test coverage.

```bash
# Generate full HTML report
make coverage
```

## Test Naming Conventions
- Reduction tests: `test_<source>_to_<target>_closed_loop`
- Model tests: `test_<model>_basic`, `test_<model>_serialization`
- Solver tests: `test_<solver>_<problem>`

## Closed-Loop Test Pattern
Every reduction MUST have a closed-loop test:

```rust
#[test]
fn test_source_to_target_closed_loop() {
    // 1. Create small instance
    let problem = SourceProblem::new(...);

    // 2. Reduce
    let reduction = ReduceTo::<TargetProblem>::reduce_to(&problem);
    let target = reduction.target_problem();

    // 3. Solve target
    let solver = BruteForce::new();
    let solutions = solver.find_best(target);

    // 4. Extract and verify
    for sol in solutions {
        let extracted = reduction.extract_solution(&sol);
        assert!(problem.solution_size(&extracted).is_valid);
    }
}
```

## Before Submitting PR
```bash
make test      # All tests pass
make clippy    # No warnings
make coverage  # >95% for new code
```

## Test File Organization

Unit tests live in `src/unit_tests/`, mirroring `src/` structure. Source files reference them via `#[path]`:

```rust
// In src/rules/foo_bar.rs:
#[cfg(test)]
#[path = "../unit_tests/rules/foo_bar.rs"]
mod tests;
```

The `#[path]` is relative to the source file's directory. `use super::*` in the test file resolves to the parent module (same as inline tests).

Integration tests are consolidated into a single binary at `tests/main.rs`, with test modules in `tests/suites/`.

## Anti-patterns
- Don't skip closed-loop tests for reductions
- Don't test only happy paths - include edge cases
- Don't ignore clippy warnings
- Don't add inline `mod tests` blocks in `src/` — use `src/unit_tests/` with `#[path]`
