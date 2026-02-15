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

Reductions compose into multi-step chains. The `ReductionGraph` finds paths
through the variant-level graph, where each edge is a registered reduction
(including variant casts with identity overhead).
Here we solve a 3-SAT formula by chaining through Satisfiability
and MaximumIndependentSet:

```rust
use problemreductions::prelude::*;
use problemreductions::topology::SimpleGraph;
use problemreductions::rules::ReductionGraph;
use problemreductions::solvers::ILPSolver;

// --- Plan: find a reduction path ---

let graph = ReductionGraph::new();
let path = graph.find_shortest_path_by_name("KSatisfiability", "MaximumIndependentSet").unwrap();

// The path: KSatisfiability → Satisfiability → MaximumIndependentSet
// With variant casts: K3 → KN (variant cast), then KN-SAT → SAT → MIS

// --- Execute: create, reduce, solve, extract ---

// Create: 3-SAT formula (a∨b∨¬c)∧(¬a∨¬b∨¬c)∧(¬a∨b∨c)∧(a∨¬b∨c)
let ksat = KSatisfiability::<K3>::new(3, vec![
    CNFClause::new(vec![1, 2, -3]),    // a ∨ b ∨ ¬c
    CNFClause::new(vec![-1, -2, -3]),  // ¬a ∨ ¬b ∨ ¬c
    CNFClause::new(vec![-1, 2, 3]),    // ¬a ∨ b ∨ c
    CNFClause::new(vec![1, -2, 3]),    // a ∨ ¬b ∨ c
]);

// Variant cast: 3-SAT → N-SAT (explicit reduction, KN accepts any clause size)
let r0 = ReduceTo::<KSatisfiability<KN>>::reduce_to(&ksat);

// Reduce: N-SAT → Satisfiability (trivial embedding)
let r1 = ReduceTo::<Satisfiability>::reduce_to(r0.target_problem());

// Reduce: Satisfiability → MaximumIndependentSet (Karp reduction)
let r2 = ReduceTo::<MaximumIndependentSet<SimpleGraph, i32>>::reduce_to(r1.target_problem());

// Solve: MIS via ILP (internally: MIS → ILP → solve → extract)
let ilp = ILPSolver::new();
let mis_solution = ilp.solve_reduced(r2.target_problem()).unwrap();

// Extract: trace back through the reduction chain
let sat_solution = r2.extract_solution(&mis_solution);
let nsat_solution = r1.extract_solution(&sat_solution);
let original_solution = r0.extract_solution(&nsat_solution);

// Verify: satisfies the original 3-SAT formula
assert!(ksat.evaluate(&original_solution));
```

The `ILPSolver::solve_reduced()` handles the final MIS → ILP reduction,
solve, and extraction internally. The caller traces back the explicit chain
with `extract_solution()` at each step, recovering a satisfying assignment
for the original formula.

> **Note:** `ILPSolver` requires the `ilp` feature flag (see [Solvers](#solvers)).

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
