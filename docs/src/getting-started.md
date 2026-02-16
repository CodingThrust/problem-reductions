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

### Example 1: Direct reduction

Reduce Maximum Independent Set to Minimum Vertex Cover on a 4-vertex path
graph, solve the target, and extract the solution back.

#### Step 1 — Create the source problem

A path graph `0–1–2–3` has 4 vertices and 3 edges.

```rust,ignore
use problemreductions::prelude::*;
use problemreductions::topology::SimpleGraph;

let problem = MaximumIndependentSet::new(
    SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
    vec![1i32; 4],
);
```

#### Step 2 — Reduce to Minimum Vertex Cover

`ReduceTo` applies a single-step reduction. The result holds the target
problem and knows how to map solutions back.

```rust,ignore
println!("Source: {} {:?}", MaximumIndependentSet::<SimpleGraph, i32>::NAME,
    MaximumIndependentSet::<SimpleGraph, i32>::variant());
let reduction = ReduceTo::<MinimumVertexCover<SimpleGraph, i32>>::reduce_to(&problem);
let target = reduction.target_problem();
println!("Target: {} {:?}, {} variables",
    MinimumVertexCover::<SimpleGraph, i32>::NAME,
    MinimumVertexCover::<SimpleGraph, i32>::variant(),
    target.num_variables());
```

```text
Source: MaximumIndependentSet [("graph", "SimpleGraph"), ("weight", "i32")]
Target: MinimumVertexCover [("graph", "SimpleGraph"), ("weight", "i32")], 4 variables
```

#### Step 3 — Solve the target problem

`BruteForce` enumerates all configurations and returns the optimal one.

```rust,ignore
let solver = BruteForce::new();
let target_solution = solver.find_best(target).unwrap();
println!("VC solution: {:?}", target_solution);
```

```text
VC solution: [1, 0, 1, 0]
```

#### Step 4 — Extract and verify

`extract_solution` maps the Vertex Cover solution back to an Independent Set
solution by complementing the configuration.

```rust,ignore
let solution = reduction.extract_solution(&target_solution);
let metric = problem.evaluate(&solution);
println!("IS solution: {:?} -> size {:?}", solution, metric);
assert!(metric.is_valid());
```

```text
IS solution: [0, 1, 0, 1] -> size Valid(2)
```

S ⊆ V is an independent set iff V \ S is a vertex cover, so the complement
maps optimality in one direction to optimality in the other.

### Example 2: Reduction path search — integer factoring to spin glass

Real-world problems often require **chaining** multiple reductions. Here we factor the integer 6 by reducing `Factoring` through the reduction graph to `SpinGlass`, through automatic reduction path search.

```rust,ignore
{{#include ../../examples/chained_reduction_factoring_to_spinglass.rs:imports}}

{{#include ../../examples/chained_reduction_factoring_to_spinglass.rs:example}}
```

Let's walk through each step.

#### Step 1 — Discover the reduction path

`ReductionGraph` holds every registered reduction. `find_cheapest_path`
searches for the shortest chain from a source problem variant to a target
variant.

```rust,ignore
{{#include ../../examples/chained_reduction_factoring_to_spinglass.rs:step1}}
```

```text
  Factoring → CircuitSAT → SpinGlass {graph: "SimpleGraph", weight: "i32"}
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
6 = 3 × 2
```

#### Step 5 — Inspect the overhead

Each reduction edge carries a polynomial overhead mapping source problem
sizes to target sizes. `path_overheads` returns the per-edge
polynomials, and `compose_path_overhead` composes them symbolically into a
single end-to-end formula.

```rust,ignore
{{#include ../../examples/chained_reduction_factoring_to_spinglass.rs:overhead}}
```

```text
Factoring → CircuitSAT:
  num_variables = num_bits_first * num_bits_second
  num_assignments = num_bits_first * num_bits_second
CircuitSAT → SpinGlass {graph: "SimpleGraph", weight: "i32"}:
  num_spins = num_assignments
  num_interactions = num_assignments
SpinGlass {graph: "SimpleGraph", weight: "i32"} → SpinGlass {graph: "SimpleGraph", weight: "f64"}:
  num_spins = num_spins
  num_interactions = num_interactions
Composed (source → target):
  num_spins = num_bits_first * num_bits_second
  num_interactions = num_bits_first * num_bits_second
```

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
