# Design

This guide covers the library internals for contributors.

## Module Overview

<div class="theme-light-only">

![Module Overview](static/module-overview.svg)

</div>
<div class="theme-dark-only">

![Module Overview](static/module-overview-dark.svg)

</div>

| Module | Purpose |
|--------|---------|
| [`src/models/`](#problem-model) | Problem type implementations (SAT, Graph, Set, Optimization) |
| [`src/rules/`](#reduction-rules) | Reduction rules with `ReduceTo` implementations |
| [`src/registry/`](#reduction-graph) | Compile-time reduction graph metadata |
| [`src/solvers/`](#solvers) | BruteForce and ILP solvers |
| `src/traits.rs` | Core `Problem` and `OptimizationProblem` traits (see [Problem Model](#problem-model)) |
| `src/types.rs` | Shared types: `SolutionSize`, `Direction`, `ProblemSize` (see [Problem Model](#problem-model)) |
| `src/variant.rs` | Variant parameter system (see [Variant System](#variant-system)) |

## Problem Model

Every problem implements `Problem`. Optimization problems additionally implement `OptimizationProblem`; satisfaction problems implement `SatisfactionProblem`.

- **`Problem`** — the base trait. Every problem declares a `NAME` (e.g., `"MaximumIndependentSet"`). The solver explores the configuration space defined by `dims()` and scores each configuration with `evaluate()`. For example, a 4-vertex MIS has `dims() = [2, 2, 2, 2]` (each vertex is selected or not); `evaluate(&[1, 0, 1, 0])` returns `Valid(2)` if vertices 0 and 2 form an independent set, or `Invalid` if they share an edge.
- **`OptimizationProblem`** — extends `Problem` with a comparable `Value` type and a `direction()` (`Maximize` or `Minimize`).
- **`SatisfactionProblem`** — constrains `Metric = bool`: `true` if all constraints are satisfied, `false` otherwise.

<div class="theme-light-only">

![Trait Hierarchy](static/trait-hierarchy.svg)

</div>
<div class="theme-dark-only">

![Trait Hierarchy](static/trait-hierarchy-dark.svg)

</div>

## Variant System

A single problem name like `MaximumIndependentSet` can have multiple **variants** — carrying weights on vertices, or defined on a restricted topology (e.g., king's subgraph). Variants form a subtype hierarchy: independent sets on king's subgraphs are a subset of independent sets on unit-disk graphs. The reduction from a more specific variant to a less specific one is a **variant cast** — an identity mapping where indices are preserved.

<div class="theme-light-only">

![Variant Hierarchy](static/variant-hierarchy.svg)

</div>
<div class="theme-dark-only">

![Variant Hierarchy](static/variant-hierarchy-dark.svg)

</div>

Variant types fall into three categories:

- **Graph type** — `HyperGraph` (root), `SimpleGraph`, `PlanarGraph`, `BipartiteGraph`, `UnitDiskGraph`, `KingsSubgraph`, `TriangularSubgraph`.
- **Weight type** — `One` (unweighted), `i32`, `f64`.
- **K value** — e.g., `K3` for 3-SAT, `KN` for arbitrary K.

<div class="theme-light-only">

![Lattices](static/lattices.svg)

</div>
<div class="theme-dark-only">

![Lattices](static/lattices-dark.svg)

</div>

<details>
<summary>Implementation details: VariantParam trait and macros</summary>

### VariantParam trait

Each variant parameter type implements `VariantParam`, which declares its category, value, and optional parent:

```rust
pub trait VariantParam: 'static {
    const CATEGORY: &'static str;     // e.g., "graph", "weight", "k"
    const VALUE: &'static str;        // e.g., "SimpleGraph", "i32"
    const PARENT_VALUE: Option<&'static str>;  // None for root types
}
```

Types with a parent also implement `CastToParent`, providing the runtime conversion for variant casts:

```rust
pub trait CastToParent: VariantParam {
    type Parent: VariantParam;
    fn cast_to_parent(&self) -> Self::Parent;
}
```

### Registration with `impl_variant_param!`

The `impl_variant_param!` macro implements `VariantParam` (and optionally `CastToParent` / `KValue`) for a type:

```rust
// Root type (no parent):
impl_variant_param!(HyperGraph, "graph");

// Type with parent (cast closure required):
impl_variant_param!(SimpleGraph, "graph", parent: HyperGraph,
    cast: |g| {
        let edges: Vec<Vec<usize>> = g.edges().into_iter().map(|(u, v)| vec![u, v]).collect();
        HyperGraph::new(g.num_vertices(), edges)
    });

// K root (arbitrary K):
impl_variant_param!(KN, "k", k: None);

// Specific K with parent:
impl_variant_param!(K3, "k", parent: KN, cast: |_| KN, k: Some(3));
```

### Variant cast reductions with `impl_variant_reduction!`

When a more specific variant needs to be treated as a less specific one, an explicit variant cast reduction is declared:

```rust
impl_variant_reduction!(
    MaximumIndependentSet,
    <KingsSubgraph, i32> => <UnitDiskGraph, i32>,
    fields: [num_vertices, num_edges],
    |src| MaximumIndependentSet::from_graph(
        src.graph().cast_to_parent(), src.weights())
);
```

### Composing `Problem::variant()`

The `variant_params!` macro composes the `Problem::variant()` body from type parameter names:

```rust
// MaximumIndependentSet<G: VariantParam, W: VariantParam>
fn variant() -> Vec<(&'static str, &'static str)> {
    crate::variant_params![G, W]
    // e.g., MaximumIndependentSet<UnitDiskGraph, One>
    //     -> vec![("graph", "UnitDiskGraph"), ("weight", "One")]
}
```

</details>

## Reduction Rules

A reduction requires two pieces: a **result struct** and a **`ReduceTo<T>` impl**.

The result struct holds the target problem and the logic to map solutions back:

```rust
#[derive(Debug, Clone)]
pub struct ReductionISToVC<W> {
    target: MinimumVertexCover<SimpleGraph, W>,
}

impl<W: WeightElement + VariantParam> ReductionResult for ReductionISToVC<W> {
    type Source = MaximumIndependentSet<SimpleGraph, W>;
    type Target = MinimumVertexCover<SimpleGraph, W>;

    fn target_problem(&self) -> &Self::Target { &self.target }
    fn extract_solution(&self, target_sol: &[usize]) -> Vec<usize> {
        target_sol.iter().map(|&x| 1 - x).collect()  // complement
    }
}
```

The `#[reduction]` macro on the `ReduceTo<T>` impl registers the reduction in the compile-time graph:

```rust
#[reduction(
    overhead = {
        ReductionOverhead::new(vec![
            ("num_vertices", poly!(num_vertices)),
            ("num_edges", poly!(num_edges)),
        ])
    }
)]
impl ReduceTo<MinimumVertexCover<SimpleGraph, i32>>
    for MaximumIndependentSet<SimpleGraph, i32>
{
    type Result = ReductionISToVC<i32>;
    fn reduce_to(&self) -> Self::Result { /* ... */ }
}
```

<details>
<summary>What the <code>#[reduction]</code> macro generates</summary>

The `#[reduction]` attribute expands to the original `impl` block plus an `inventory::submit!` call:

```rust
inventory::submit! {
    ReductionEntry {
        source_name: "MaximumIndependentSet",
        target_name: "MinimumVertexCover",
        source_variant_fn: || <MaximumIndependentSet<SimpleGraph, i32> as Problem>::variant(),
        target_variant_fn: || <MinimumVertexCover<SimpleGraph, i32> as Problem>::variant(),
        overhead_fn: || ReductionOverhead::new(vec![
            ("num_vertices", poly!(num_vertices)),
            ("num_edges", poly!(num_edges)),
        ]),
        module_path: module_path!(),
        reduce_fn: |src: &dyn Any| -> Box<dyn DynReductionResult> {
            let src = src.downcast_ref::<MaximumIndependentSet<SimpleGraph, i32>>().unwrap();
            Box::new(ReduceTo::<MinimumVertexCover<SimpleGraph, i32>>::reduce_to(src))
        },
    }
}
```

This `ReductionEntry` is collected at compile time by `inventory`, making the reduction discoverable by the `ReductionGraph` without any manual registration. The `reduce_fn` field provides a type-erased executor that enables runtime-discovered paths to chain reductions automatically.

</details>

## Reduction Graph

`ReductionGraph::new()` scans `inventory::iter::<ReductionEntry>` and builds a variant-level directed graph:

- **Nodes** are unique `(problem_name, variant)` pairs — e.g., `("MaximumIndependentSet", {graph: "KingsSubgraph", weight: "i32"})`.
- **Edges** come exclusively from `#[reduction]` registrations — both cross-problem reductions and variant casts. There are no auto-generated edges.

### Path finding

All path-finding operates on **exact variant nodes**. Use `ReductionGraph::variant_to_map(&T::variant())` to convert a `Problem::variant()` into the required `BTreeMap<String, String>`.

| Method | Algorithm | Use case |
|--------|-----------|----------|
| `find_cheapest_path(src, src_var, dst, dst_var, input_size, cost_fn)` | Dijkstra | Optimal path under a cost function |
| `find_all_paths(src, src_var, dst, dst_var)` | All simple paths | Enumerate every route |

Use `find_cheapest_path` with `MinimizeSteps` for fewest-hops search.

The `PathCostFn` trait (used by `find_cheapest_path`) computes edge cost from overhead and current problem size:

| Cost function | Strategy |
|--------------|----------|
| `MinimizeSteps` | Minimize number of hops (unit edge cost) |
| `Minimize("field")` | Minimize a single output field |
| `CustomCost(closure)` | User-defined cost function |

**Example:** Finding a path from `MIS{KingsSubgraph, i32}` to `VC{SimpleGraph, i32}`:

```
MIS{KingsSubgraph,i32} -> MIS{UnitDiskGraph,i32} -> MIS{SimpleGraph,i32} -> VC{SimpleGraph,i32}
     variant cast              variant cast                reduction
```

### Executable paths

Convert a `ReductionPath` into a typed `ExecutablePath<S, T>` via `make_executable()`, then call `reduce()`:

```rust
let rpath = graph.find_cheapest_path("Factoring", &src_var,
    "SpinGlass", &dst_var, &ProblemSize::new(vec![]), &MinimizeSteps).unwrap();
let path = graph.make_executable::<Factoring, SpinGlass<SimpleGraph, f64>>(&rpath).unwrap();

let reduction = path.reduce(&factoring_instance);
let target = reduction.target_problem();
let solution = reduction.extract_solution(&target_solution);
```

Internally, `ExecutablePath` holds a type-erased executor per edge. `ChainedReduction` stores intermediate results and extracts solutions back through the chain in reverse.

For full type control, you can also chain `ReduceTo::reduce_to()` calls manually at each step.

<details>
<summary>Overhead evaluation</summary>

Each reduction declares how the output problem size relates to the input, expressed as polynomials. The `poly!` macro provides concise syntax:

```rust
poly!(num_vertices)              // p(x) = num_vertices
poly!(num_vertices ^ 2)          // p(x) = num_vertices²
poly!(3 * num_edges)             // p(x) = 3 × num_edges
poly!(num_vertices * num_edges)  // p(x) = num_vertices × num_edges
```

A `ReductionOverhead` pairs output field names with their polynomials:

```rust
ReductionOverhead::new(vec![
    ("num_vars", poly!(num_vertices) + poly!(num_edges)),
    ("num_clauses", poly!(3 * num_edges)),
])
```

`evaluate_output_size(input)` substitutes input values:

```
Input:  ProblemSize { num_vertices: 10, num_edges: 15 }
Output: ProblemSize { num_vars: 25, num_clauses: 45 }
```

For multi-step paths, overhead composes: the output of step N becomes the input of step N+1. Variant cast edges use `ReductionOverhead::identity()`, passing through all fields unchanged.

</details>

## Solvers

Solvers implement the `Solver` trait:

```rust
pub trait Solver {
    fn find_best<P: OptimizationProblem>(&self, problem: &P) -> Option<Vec<usize>>;
    fn find_satisfying<P: Problem<Metric = bool>>(&self, problem: &P) -> Option<Vec<usize>>;
}
```

| Solver | Description |
|--------|-------------|
| **BruteForce** | Enumerates all configurations. Also provides `find_all_best()` and `find_all_satisfying()`. Used for testing and verification. |
| **ILPSolver** | Feature-gated (`ilp`). Uses HiGHS via `good_lp`. Also provides `solve_reduced()` for problems that implement `ReduceTo<ILP>`. |

## JSON Serialization

All problem types support JSON serialization via serde:

```rust
use problemreductions::io::{to_json, from_json};

let json = to_json(&problem)?;
let restored: MaximumIndependentSet<SimpleGraph, i32> = from_json(&json)?;
```

Exported files:

- [reduction_graph.json](reductions/reduction_graph.json) — all problem variants and reduction edges
- [problem_schemas.json](reductions/problem_schemas.json) — field definitions for each problem type

Regenerate with `cargo run --example export_graph` and `cargo run --example export_schemas`.

## Contributing

See [Call for Contributions](./introduction.md#call-for-contributions) for the recommended issue-based workflow (no coding required).
