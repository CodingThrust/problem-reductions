# Problem Types

This library implements 18+ NP-hard computational problems across several categories.

## Categories

### Graph Problems

Problems defined on graphs with vertices and edges:
- **IndependentSet**: Find maximum weight set of non-adjacent vertices
- **VertexCovering**: Find minimum weight set covering all edges
- **MaxCut**: Partition vertices to maximize cut weight
- **Coloring**: Assign colors minimizing conflicts
- **DominatingSet**: Find minimum set dominating all vertices
- **MaximalIS**: Find maximal independent sets
- **Matching**: Find maximum weight matching

[Learn more about Graph Problems](./graph.md)

### Satisfiability Problems

Boolean satisfaction problems:
- **Satisfiability (SAT)**: Find satisfying assignments for CNF formulas
- **CircuitSAT**: Satisfy boolean circuits
- **Factoring**: Factor integers (reducible to SAT)

[Learn more about Satisfiability Problems](./satisfiability.md)

### Optimization Problems

Continuous optimization on discrete domains:
- **SpinGlass**: Ising model Hamiltonian minimization
- **QUBO**: Quadratic unconstrained binary optimization

[Learn more about Optimization Problems](./optimization.md)

### Set Problems

Problems involving set collections:
- **SetCovering**: Minimum weight cover of universe
- **SetPacking**: Maximum weight packing of disjoint sets

[Learn more about Set Problems](./set.md)

### Specialized Problems

Domain-specific problems:
- **PaintShop**: Minimize color switches
- **BicliqueCover**: Cover bipartite graph with bicliques
- **BMF**: Boolean matrix factorization

[Learn more about Specialized Problems](./specialized.md)

## Common Interface

All problems implement the `Problem` trait:

```rust
pub trait Problem {
    type Size;

    fn num_variables(&self) -> usize;
    fn num_flavors(&self) -> usize;
    fn problem_size(&self) -> ProblemSize;
    fn solution_size(&self, config: &[usize]) -> SolutionSize<Self::Size>;
    fn energy_mode(&self) -> EnergyMode;
}
```
