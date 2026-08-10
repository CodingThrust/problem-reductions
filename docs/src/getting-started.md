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

let problem = MaximumSetPacking::<i32>::new(vec![
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

For convenience, `ILPSolver::solve_reduced` combines reduce + solve + extract
in a single call:

```rust,ignore
let solution = ILPSolver::new()
    .solve_reduced::<bool, _>(&problem)
    .unwrap();
assert!(problem.evaluate(&solution).is_valid());
```

The ILP domain is explicit because a source type may provide more than one
direct ILP reduction. Both `bool` and `i32` are supported. `solve` and
`solve_reduced` return `ILPSolveError`, which distinguishes infeasibility,
timeout, unboundedness, unsupported dynamic input, and backend failure.

### Example 2: Reduction path search — integer factoring to spin glass

Real-world problems often require **chaining** multiple reductions. Here we factor the integer 6 by reducing `Factoring` through the reduction graph to `SpinGlass`, through automatic reduction path search. ([full source](https://github.com/CodingThrust/problem-reductions/blob/main/examples/chained_reduction_factoring_to_spinglass.rs))

Let's walk through each step.

#### Step 1 — Discover the reduction path

`ReductionGraph` holds every registered reduction. The example enumerates the
witness-capable simple paths and explicitly selects the documented
`Factoring -> CircuitSAT -> SpinGlass` route. Symbolic path discovery does not
rank paths; each route carries its strongest available per-field size relation.

```rust,ignore
{{#include ../../examples/chained_reduction_factoring_to_spinglass.rs:step1}}
```

```text
{{#include generated/factoring-path.txt}}
```

#### Step 2 — Create the Factoring problem

`Factoring::new(m, n, target)` creates a factoring instance: find two factors
`p` (m-bit) and `q` (n-bit) such that `p × q = target`. Here we factor **6**
with two 2-bit factors, expecting **2 × 3** or **3 × 2**.

```rust,ignore
{{#include ../../examples/chained_reduction_factoring_to_spinglass.rs:step2}}
```

#### Step 3 — Solve with ILPSolver

`solve_reduced` reduces the problem to ILP internally and solves it in one
call. It returns a configuration vector for the original problem — no manual
extraction needed. For small instances you can also use `BruteForce`, but
`ILPSolver` scales to much larger problems.

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

#### Step 5 — Inspect the size contract

Each reduction edge classifies every target-size field as exact, bound-only,
or unavailable with a reason. `compose_path_size_map` composes only exact
fields into an end-to-end map; a missing exact contract remains a typed error
and never falls back to a bound or asymptotic formula.

```rust,ignore
{{#include ../../examples/chained_reduction_factoring_to_spinglass.rs:size_contract}}
```

```text
{{#include generated/factoring-size-contract.txt}}
```

## Solvers

Three solvers are available:

| Solver | Use Case | Notes |
|--------|----------|-------|
| [`BruteForce`](api/problemreductions/solvers/struct.BruteForce.html) | Small instances (<20 variables) | Enumerates all configurations |
| [`ILPSolver`](api/problemreductions/solvers/ilp/struct.ILPSolver.html) | Larger instances | Enabled by default (`ilp` feature) |
| [`CustomizedSolver`](api/problemreductions/solvers/customized/struct.CustomizedSolver.html) | Structure-exploiting | Uses problem-specific exact algorithms |

ILP support is enabled by default. To disable it:

```bash
cargo add problemreductions --no-default-features
```

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
