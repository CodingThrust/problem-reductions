---
paths:
  - "src/models/**/*.rs"
---

# Adding a Model (Problem Type)

## 1. Define the Model
Create `src/models/<category>/<name>.rs`:

```rust
use serde::{Deserialize, Serialize};
use crate::traits::{Problem, ProblemSize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MyProblem<W> {
    // Problem data fields
    pub size: usize,
    pub weights: Vec<W>,
    // ...
}

impl<W: Clone> Problem for MyProblem<W> {
    fn num_variables(&self) -> usize { ... }
    fn problem_size(&self) -> ProblemSize { ... }
    fn is_valid_solution(&self, solution: &[usize]) -> bool { ... }
}
```

## 2. Register in Module
Add to `src/models/<category>/mod.rs`:
```rust
mod my_problem;
pub use my_problem::MyProblem;
```

## 3. Categories
Place models in appropriate category:
- `src/models/satisfiability/` - Satisfiability, KSatisfiability, CircuitSAT
- `src/models/graph/` - MaximumIndependentSet, MinimumVertexCover, KColoring, etc.
- `src/models/set/` - MinimumSetCovering, MaximumSetPacking
- `src/models/optimization/` - SpinGlass, QUBO, ILP

## 4. Required Traits
- `Serialize`, `Deserialize` - JSON I/O support
- `Clone`, `Debug` - Standard Rust traits
- `Problem` - Core trait with `num_variables()`, `problem_size()`, `is_valid_solution()`
- Consider `ConstraintSatisfactionProblem` if applicable

## 5. Naming
Use explicit optimization prefixes: `Maximum` for maximization, `Minimum` for minimization (e.g., `MaximumIndependentSet`, `MinimumVertexCover`).

## 6. Documentation
Document in `docs/paper/reductions.typ` using `#problem-def("ProblemName", "Display Title")[...]`

## Anti-patterns
- Don't create models without JSON serialization support
- Don't forget to implement `is_valid_solution()` correctly
- Don't use concrete types when generic `W` is appropriate
