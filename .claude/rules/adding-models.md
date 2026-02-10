---
paths:
  - "src/models/**/*.rs"
---

# Adding a Model (Problem Type)

## 1. Define the Model
Create `src/models/<category>/<name>.rs`:

```rust
use serde::{Deserialize, Serialize};
use crate::traits::Problem;
use crate::types::{EnergyMode, ProblemSize, SolutionSize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MyProblem<G, W> {
    graph: G,
    weights: Vec<W>,
}

impl<G: Graph, W: NumericWeight> Problem for MyProblem<G, W> {
    const NAME: &'static str = "MyProblem";
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", G::NAME), ("weight", W::NAME)]
    }
    type Size = W;

    fn num_variables(&self) -> usize { self.graph.num_vertices() }
    fn num_flavors(&self) -> usize { 2 }
    fn problem_size(&self) -> ProblemSize { ProblemSize::new(vec![("num_vertices", ...)]) }
    fn energy_mode(&self) -> EnergyMode { EnergyMode::LargerSizeIsBetter }
    fn solution_size(&self, config: &[usize]) -> SolutionSize<Self::Size> {
        // Compute objective value, check validity, return SolutionSize::new(value, is_valid)
    }
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
- `src/models/specialized/` - Factoring

## 4. Required Traits
- `Serialize`, `Deserialize` - JSON I/O support
- `Clone`, `Debug` - Standard Rust traits
- `Problem` - Core trait (see template above for required methods)
- Consider `ConstraintSatisfactionProblem` if applicable (adds `constraints()`, `objectives()`, `weights()`)

## 5. Naming
Use explicit optimization prefixes: `Maximum` for maximization, `Minimum` for minimization (e.g., `MaximumIndependentSet`, `MinimumVertexCover`).

## 6. Documentation
- Add entry to `display-name` dict in `docs/paper/reductions.typ`
- Add `#problem-def("ProblemName")[mathematical definition...]` in the paper

## Anti-patterns
- Don't create models without JSON serialization support
- Don't forget to implement `solution_size()` with correct validity checks
- Don't use concrete types when generic `W` and `G` are appropriate
