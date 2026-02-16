# Getting Started

## What This Library Does

**problem-reductions** transforms hard computational problems into forms that efficient solvers can handle. You define a problem, reduce it to another problem type (like QUBO or ILP), solve the reduced problem, and extract the solution back. The [interactive reduction graph](./introduction.html) shows all available problem types and transformations.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
problemreductions = "0.1"
```

## The Reduction Workflow

The core workflow is: **create** a problem, **reduce** it to a target, **solve** the target, and **extract** the solution back.

<div class="theme-light-only">

![Reduction Workflow](static/reduction-workflow.svg)

</div>
<div class="theme-dark-only">

![Reduction Workflow](static/reduction-workflow-dark.svg)

</div>

### Complete Example

```rust
use problemreductions::prelude::*;
use problemreductions::topology::SimpleGraph;

// 1. Create: Independent Set on a path graph (4 vertices)
let problem = MaximumIndependentSet::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);

// 2. Reduce: Transform to Minimum Vertex Cover
let reduction = ReduceTo::<MinimumVertexCover<SimpleGraph, i32>>::reduce_to(&problem);
let target = reduction.target_problem();

// 3. Solve: Find optimal solution to the target problem
let solver = BruteForce::new();
let target_solution = solver.find_best(target).unwrap();

// 4. Extract: Map solution back to original problem
let solution = reduction.extract_solution(&target_solution);

// Verify: solution is valid for the original problem
let metric = problem.evaluate(&solution);
assert!(metric.is_valid());
```

### Chaining Reductions

Reductions compose into multi-step chains. The `ReductionGraph` discovers
paths through the variant-level graph. Find a `ReductionPath` first, then
convert it to a typed `ExecutablePath<S, T>` via `make_executable()`. Call
`reduce()` once and get a `ChainedReduction` with `target_problem()` and
`extract_solution()`, just like a single-step reduction.

Here we solve a 3-SAT formula by chaining through Satisfiability
and MaximumIndependentSet:

```rust
use problemreductions::prelude::*;
use problemreductions::topology::SimpleGraph;
use problemreductions::rules::ReductionGraph;

let graph = ReductionGraph::new();
let rpath = graph
    .find_shortest_path::<KSatisfiability<K3>, MaximumIndependentSet<SimpleGraph, i32>>()
    .unwrap();
let path = graph
    .make_executable::<KSatisfiability<K3>, MaximumIndependentSet<SimpleGraph, i32>>(&rpath)
    .unwrap();

// Create: 3-SAT formula (a∨b∨¬c)∧(¬a∨¬b∨¬c)∧(¬a∨b∨c)∧(a∨¬b∨c)
let ksat = KSatisfiability::<K3>::new(3, vec![
    CNFClause::new(vec![1, 2, -3]),
    CNFClause::new(vec![-1, -2, -3]),
    CNFClause::new(vec![-1, 2, 3]),
    CNFClause::new(vec![1, -2, 3]),
]);

// Reduce: the executable path handles all intermediate steps
let reduction = path.reduce(&ksat);
let target = reduction.target_problem();

// Solve and extract back to original space
let solver = BruteForce::new();
let solution = solver.find_best(target).unwrap();
let original = reduction.extract_solution(&solution);

// Verify: satisfies the original 3-SAT formula
assert!(ksat.evaluate(&original));
```

The `ExecutablePath` handles variant casts (e.g., `K3` → `KN`) and
cross-problem reductions (e.g., SAT → MIS) uniformly. The `ChainedReduction`
extracts solutions back through the entire chain in one call.

## Solvers

Two solvers for testing purposes are available:

| Solver | Use Case | Notes |
|--------|----------|-------|
| [`BruteForce`](api/problemreductions/solvers/struct.BruteForce.html) | Small instances (<20 variables) | Enumerates all configurations |
| [`ILPSolver`](api/problemreductions/solvers/ilp/struct.ILPSolver.html) | Larger instances | Requires `ilp` feature flag |

Enable ILP support:

```toml
[dependencies]
problemreductions = { version = "0.1", features = ["ilp"] }
```

**Future:** Automated reduction path optimization will find the best route between any two connected problems.

## JSON Resources

The library exports machine-readable metadata useful for tooling and research:

- [reduction_graph.json](reductions/reduction_graph.json) lists all problem variants and reduction edges
- [problem_schemas.json](reductions/problem_schemas.json) lists field definitions for each problem type


## Next Steps

- Explore the [interactive reduction graph](./introduction.html) to discover available reductions
- Read the [Architecture](./arch.md) guide for implementation details
- Browse the [API Reference](./api.html) for full documentation
