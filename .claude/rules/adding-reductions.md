---
paths:
  - "src/rules/**/*.rs"
---

# Adding a Reduction Rule (A → B)

## 0. Brainstorm & Generate Test Data First

Before writing any Rust code, follow this workflow:

1. **Brainstorm the reduction** — use `superpowers:brainstorming` to discuss with the user:
   - Research the mathematical formulation (paper, textbook, or derive it)
   - Understand the variable mapping and constraint encoding
   - Discuss implementation approach: penalty values, matrix construction, solution extraction
   - Read reference implementations in the codebase (e.g., `src/rules/spinglass_qubo.rs`) to understand conventions
   - Agree on scope (weighted vs unweighted, specific graph types, const generics)
2. **Generate ground truth test data** — use an existing library (e.g., Python with qubogen, qubovert, or networkx) to create small instances, reduce them, brute-force solve both sides, and export as JSON to `tests/data/<target>/`. It is recommended to download the relevant package and check the existing tests to understand how to construct tests. To generate the test data, you can use the following command:
   ```bash
   # Example: generate QUBO test data
   cd scripts && uv run python generate_qubo_tests.py
   ```
3. **Create a practical example** — design a small, explainable instance for `examples/` (e.g., "wireless tower placement" for IndependentSet, "map coloring" for Coloring). This example will also appear in the `docs/paper/reductions.typ`.
4. **Write the implementation plan** — save to `docs/plans/` using `superpowers:writing-plans`. The plan must include implementation details from the brainstorming session (formulas, penalty terms, matrix construction, variable indexing).

## 1. Implementation

Create `src/rules/<source>_<target>.rs` following the pattern in `src/rules/spinglass_qubo.rs`:

```rust
use crate::reduction;

#[derive(Debug, Clone)]
pub struct ReductionSourceToTarget {
    target: TargetProblem<...>,
    source_size: ProblemSize,
    // + any metadata needed for extract_solution
}

impl ReductionResult for ReductionSourceToTarget {
    type Source = SourceProblem<...>;
    type Target = TargetProblem<...>;

    fn target_problem(&self) -> &Self::Target { &self.target }
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> { ... }
    fn source_size(&self) -> ProblemSize { self.source_size.clone() }
    fn target_size(&self) -> ProblemSize { self.target.problem_size() }
}

#[reduction(
    overhead = { ReductionOverhead::new(vec![...]) }
)]
impl ReduceTo<TargetProblem<...>> for SourceProblem<...> {
    type Result = ReductionSourceToTarget;
    fn reduce_to(&self) -> Self::Result { ... }
}

#[cfg(test)]
#[path = "../unit_tests/rules/<source>_<target>.rs"]
mod tests;
```

The `#[reduction]` macro auto-generates the `inventory::submit!` call. Optional attributes: `source_graph`, `target_graph`, `source_weighted`, `target_weighted`.

Register module in `src/rules/mod.rs`:
```rust
mod source_target;
pub use source_target::ReductionSourceToTarget;
```

## 2. Tests (Required)

- **Unit tests** in `src/unit_tests/rules/<source>_<target>.rs` — closed-loop + edge cases. See `rules/testing.md`.
- **Integration tests** in `tests/suites/reductions.rs` — compare against JSON ground truth from step 0.
- Test name: `test_<source>_to_<target>_closed_loop`

## 3. Example Program

Add a round-trip demo to `examples/` showing a practical, explainable instance:
1. Create source problem with a real-world story
2. Reduce to target, solve, extract solution
3. Print human-readable explanation

## 4. Documentation

Update `docs/paper/reductions.typ` (see `rules/documentation.md` for the pattern):
- Add theorem + proof sketch
- Add Rust code example from the example program
- Add to summary table with overhead and citation

The goal is to 1. prove the correctness of the reduction to human beings. 2. provide a minimal working example to the readers.

Citations must be verifiable. Use `[Folklore]` or `—` for trivial reductions.

## 5. Regenerate Reduction Graph
```bash
make export-graph
```

## Anti-patterns
- Don't write Rust code before understanding the math and having test data
- Don't create reductions without closed-loop tests
- Don't forget `inventory::submit!` registration (reduction graph won't update)
- Don't hardcode weights - use generic `W` parameter
- Don't skip overhead polynomial specification
- Don't skip the example program — every reduction needs an explainable demo
