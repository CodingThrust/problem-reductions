# Testing Requirements

## Coverage Requirement
New code must have >95% test coverage.

```bash
# Check coverage for specific module
cargo tarpaulin --features ilp --skip-clean --ignore-tests -- <module_name>

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
    let reduction = problem.reduce_to::<TargetProblem>();
    let target = reduction.target_problem();

    // 3. Solve target
    let solver = BruteForce::new();
    let solutions = solver.find_best(target);

    // 4. Extract and verify
    for sol in solutions {
        let extracted = reduction.extract_solution(&sol);
        assert!(problem.is_valid_solution(&extracted));
    }
}
```

## Before Submitting PR
```bash
make test      # All tests pass
make clippy    # No warnings
make coverage  # >95% for new code
```

## Anti-patterns
- Don't skip closed-loop tests for reductions
- Don't test only happy paths - include edge cases
- Don't ignore clippy warnings
