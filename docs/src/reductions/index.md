# Reductions

Problem reductions allow transforming one NP-hard problem into another while preserving solution structure.

## Why Reductions?

- **Solve problems indirectly**: Transform to a problem with a better solver
- **Prove equivalence**: Show problems have the same computational complexity
- **Hardware mapping**: Transform to problems supported by quantum hardware

## Available Reductions

| Source | Target | Notes |
|--------|--------|-------|
| IndependentSet | VertexCovering | Complement relationship |
| VertexCovering | IndependentSet | Complement relationship |
| IndependentSet | SetPacking | Intersection graph |
| SetPacking | IndependentSet | Intersection graph |
| VertexCovering | SetCovering | Edge covering |
| Matching | SetPacking | Edge-to-set mapping |
| SpinGlass | QUBO | Variable substitution |
| QUBO | SpinGlass | Variable substitution |
| SpinGlass | MaxCut | Direct mapping (may add ancilla) |
| MaxCut | SpinGlass | Direct mapping |
| Satisfiability | KSatisfiability | Clause splitting/padding |
| KSatisfiability | Satisfiability | Direct embedding |
| Satisfiability | IndependentSet | Gadget construction |
| Satisfiability | Coloring | OR-gadget construction |
| Satisfiability | DominatingSet | Variable-clause graph |
| CircuitSAT | SpinGlass | Gate gadgets |
| Factoring | CircuitSAT | Multiplier circuit |

## Reduction Traits

```rust
pub trait ReduceTo<T: Problem>: Problem {
    type Result: ReductionResult<Source = Self, Target = T>;
    fn reduce_to(&self) -> Self::Result;
}

pub trait ReductionResult: Clone {
    type Source: Problem;
    type Target: Problem;

    fn target_problem(&self) -> &Self::Target;
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize>;
}
```

See:
- [Using Reductions](./using.md)
- [Available Reductions](./available.md)
- [Reduction Graph](./graph.md)
