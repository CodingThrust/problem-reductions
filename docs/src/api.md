# API Reference

Full API documentation is available at [docs.rs/problemreductions](https://docs.rs/problemreductions) or in the generated rustdoc.

## Quick Links

### Core Traits

- [`Problem`](./api/problemreductions/traits/trait.Problem.html) - Base trait for all problems
- [`ConstraintSatisfactionProblem`](./api/problemreductions/traits/trait.ConstraintSatisfactionProblem.html) - CSP-specific trait
- [`ReduceTo`](./api/problemreductions/rules/trait.ReduceTo.html) - Reduction trait
- [`ReductionResult`](./api/problemreductions/rules/trait.ReductionResult.html) - Reduction result trait
- [`Solver`](./api/problemreductions/solvers/trait.Solver.html) - Solver trait

### Problem Types

#### Graph
- [`IndependentSet`](./api/problemreductions/models/graph/struct.IndependentSet.html)
- [`VertexCovering`](./api/problemreductions/models/graph/struct.VertexCovering.html)
- [`MaxCut`](./api/problemreductions/models/graph/struct.MaxCut.html)
- [`Coloring`](./api/problemreductions/models/graph/struct.Coloring.html)
- [`DominatingSet`](./api/problemreductions/models/graph/struct.DominatingSet.html)
- [`MaximalIS`](./api/problemreductions/models/graph/struct.MaximalIS.html)
- [`Matching`](./api/problemreductions/models/graph/struct.Matching.html)

#### Optimization
- [`SpinGlass`](./api/problemreductions/models/optimization/struct.SpinGlass.html)
- [`QUBO`](./api/problemreductions/models/optimization/struct.QUBO.html)

#### Set
- [`SetCovering`](./api/problemreductions/models/set/struct.SetCovering.html)
- [`SetPacking`](./api/problemreductions/models/set/struct.SetPacking.html)

#### Satisfiability
- [`Satisfiability`](./api/problemreductions/models/satisfiability/struct.Satisfiability.html)
- [`CircuitSAT`](./api/problemreductions/models/specialized/struct.CircuitSAT.html)
- [`Factoring`](./api/problemreductions/models/specialized/struct.Factoring.html)

### Utilities
- [`ReductionGraph`](./api/problemreductions/rules/struct.ReductionGraph.html)
- [`TruthTable`](./api/problemreductions/truth_table/struct.TruthTable.html)
- [`HyperGraph`](./api/problemreductions/topology/struct.HyperGraph.html)
- [`UnitDiskGraph`](./api/problemreductions/topology/struct.UnitDiskGraph.html)
