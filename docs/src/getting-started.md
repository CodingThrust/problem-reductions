# Getting Started

## What This Library Does

**problem-reductions** transforms hard computational problems into forms that efficient solvers can handle. You define a problem, reduce it to another problem type (like QUBO or ILP), solve the reduced problem, and extract the solution back. The [interactive reduction graph](./introduction.html) shows all available problem types and transformations.

## Installation

```bash
cargo add problemreductions
```

## The Reduction Workflow

The core workflow is: **create** a problem, **reduce** it to a target, **solve** the target, and **extract** the solution back.

<div class="theme-light-only">

![Reduction Workflow](static/reduction-workflow.svg)

</div>
<div class="theme-dark-only">

![Reduction Workflow](static/reduction-workflow-dark.svg)

</div>

### Example 1: Direct reduction — Set Packing to ILP

Reduce Maximum Set Packing to Integer Linear Programming (ILP), solve with the
ILP solver, and extract the solution back.

#### Step 1 — Create the source problem

A small set system with pairwise overlaps gives a direct binary ILP.

```rust,ignore
use problemreductions::prelude::*;
use problemreductions::models::algebraic::ILP;
use problemreductions::solvers::ILPSolver;

let problem = MaximumSetPacking::<i64>::new(vec![
    vec![0, 1],
    vec![1, 2],
    vec![2, 3],
    vec![4, 5],
]);
```

#### Step 2 — Reduce to ILP

`ReduceTo` applies a single-step reduction. The result holds the target
problem and knows how to map solutions back. The ILP formulation introduces
binary variable x_i for each set, constraint x_i + x_j ≤ 1 for each
overlapping pair, and maximizes the weighted sum.

```rust,ignore
let reduction = ReduceTo::<ILP>::reduce_to(&problem);
let ilp = reduction.target_problem();
println!("ILP: {} variables, {} constraints", ilp.num_vars, ilp.constraints.len());
```

```text
ILP: 4 variables, 2 constraints
```

#### Step 3 — Solve the ILP

`ILPSolver` uses the HiGHS solver to find optimal solutions efficiently.
For small instances you can also use `BruteForce`, but `ILPSolver` scales
to much larger problems.

```rust,ignore
let solver = ILPSolver::new();
let ilp_solution = solver.solve(ilp).unwrap();
println!("ILP solution: {:?}", ilp_solution);
```

```text
ILP solution: [1, 0, 1, 0]
```

#### Step 4 — Extract and verify

`extract_solution` maps the ILP solution back to the original problem's
configuration space.

```rust,ignore
let solution = reduction.extract_solution(&ilp_solution);
let metric = problem.evaluate(&solution);
println!("Packing solution: {:?} -> size {}", solution, metric);
assert!(metric.is_valid());
```

```text
Packing solution: [1, 0, 1, 1] -> size Max(3)
```

`ILPSolver::solve` executes the problem's registered ILP pipeline, including
all reductions and reverse witness extraction:

```rust,ignore
let solution = ILPSolver::new()
    .solve(&problem)
    .unwrap();
assert!(problem.evaluate(&solution).is_valid());
```

The registered path determines the ILP variable domain. Every path ends at an
`ILP<V, f64>` terminal accepted by the HiGHS backend. `solve` returns
`ILPSolveError`, which distinguishes infeasibility, timeout, unboundedness,
missing pipelines, unsupported dynamic input, and backend failure.

### Example 2: Reduction path search — integer factoring to spin glass

Real-world problems often require **chaining** multiple reductions. Here we factor the integer 6 by reducing `Factoring` through the reduction graph to `SpinGlass`, through automatic reduction path search. ([full source](https://github.com/CodingThrust/problem-reductions/blob/main/examples/chained_reduction_factoring_to_spinglass.rs))

Let's walk through each step.

#### Step 1 — Discover the reduction path

`ReductionGraph` holds every registered reduction. The example enumerates the
witness-capable simple paths and explicitly selects the documented
`Factoring -> CircuitSAT -> SpinGlass` route. Path discovery does not rank or
automatically select a route.

```rust,ignore
{{#include ../../examples/chained_reduction_factoring_to_spinglass.rs:step1}}
```

```text
{{#include generated/factoring-path.txt}}
```

#### Step 2 — Create the Factoring problem

`Factoring::new(target)` derives safe factor-width bounds from the target.
`Factoring::with_factor_bits(target, m, n)` overrides them when a fixed-width
multiplier is required. Here we factor **6** with explicit 2-bit bounds,
returning the canonical pair **2 × 3**.

```rust,ignore
{{#include ../../examples/chained_reduction_factoring_to_spinglass.rs:step2}}
```

#### Step 3 — Solve with ILPSolver

`solve` executes the registered ILP pipeline and returns a configuration for
the original problem — no manual extraction needed. For small instances you
can also use `BruteForce`, but `ILPSolver` scales to much larger problems.

```rust,ignore
{{#include ../../examples/chained_reduction_factoring_to_spinglass.rs:step3}}
```

#### Step 4 — Read and verify the factors

`read_factors` decodes the binary configuration back into the two integer
factors.

```rust,ignore
{{#include ../../examples/chained_reduction_factoring_to_spinglass.rs:step4}}
```

```text
{{#include generated/factoring-result.txt}}
```

## Solvers

Three solvers are available:

| Solver | Use Case | Notes |
|--------|----------|-------|
| [`BruteForce`](api/problemreductions/solvers/struct.BruteForce.html) | Small instances (<20 variables) | Enumerates all configurations |
| [`ILPSolver`](api/problemreductions/solvers/ilp/struct.ILPSolver.html) | Larger instances | Uses the bundled HiGHS backend |
| **Customized backend** | Structure-exploiting | Uses problem-specific exact algorithms registered for exact problem variants |

ILP support through HiGHS is part of the library and is always available.

## JSON Resources

The library exports machine-readable metadata useful for tooling and research:

These files are generated when you build the docs locally.
- [reduction_graph.json](reductions/reduction_graph.json) lists all problem variants and reduction edges
- [problem_schemas.json](reductions/problem_schemas.json) lists field definitions for each problem type


## Next Steps

- Try the [CLI tool](./cli.md) to explore problems and reduction paths from your terminal
- Explore the [interactive reduction graph](./introduction.html) to discover available reductions
- Read the [Design](./design.md) guide for implementation details
- Browse the [API Reference](./api.html) for full documentation
