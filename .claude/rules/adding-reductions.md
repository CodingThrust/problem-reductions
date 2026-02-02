---
paths:
  - "src/rules/**/*.rs"
---

# Adding a Reduction Rule (A → B)

## 1. Implementation
Create `src/rules/<source>_<target>.rs`:

```rust
// Register reduction for automatic discovery (adds edge + metadata)
inventory::submit! {
    ReductionEntry {
        source_name: "SourceProblem",
        target_name: "TargetProblem",
        source_graph: "SourceProblem",
        target_graph: "TargetProblem",
        overhead_fn: || ReductionOverhead::new(vec![
            ("num_vars", Polynomial { terms: vec![...] }),
            ("num_constraints", Polynomial { terms: vec![...] }),
        ]),
    }
}

impl ReduceTo<TargetProblem> for SourceProblem {
    type Result = ReductionSourceToTarget;
    fn reduce_to(&self) -> Self::Result { ... }
}
```

Register module in `src/rules/mod.rs`:
```rust
mod source_target;
pub use source_target::ReductionSourceToTarget;
```

## 2. Closed-Loop Test (Required)
```rust
#[test]
fn test_closed_loop() {
    // 1. Create small instance A
    let problem = SourceProblem::new(...);

    // 2. Reduce A to B
    let reduction = ReduceTo::<TargetProblem>::reduce_to(&problem);
    let target = reduction.target_problem();

    // 3. Solve B
    let solver = TargetSolver::new();
    let target_solution = solver.solve(target).unwrap();

    // 4. Extract solution of A
    let extracted = reduction.extract_solution(&target_solution);

    // 5. Verify solution
    assert!(problem.is_valid_solution(&extracted));
}
```

## 3. Documentation
Update `docs/paper/reductions.typ`:
- Add theorem + proof sketch
- Add code example (note feature requirements if any)
- Add to summary table with overhead and citation

Citations must be verifiable. Use `[Folklore]` or `—` for trivial reductions.

## 4. Regenerate Reduction Graph
```bash
make export-graph
```

## Anti-patterns
- Don't create reductions without closed-loop tests
- Don't forget `inventory::submit!` registration (reduction graph won't update)
- Don't hardcode weights - use generic `W` parameter
- Don't skip overhead polynomial specification
