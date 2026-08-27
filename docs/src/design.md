# Design

This guide covers the library internals for contributors.

See [Numeric types and arithmetic](#numeric-types-and-arithmetic) before
choosing numeric fields or implementing arithmetic in a model or reduction.

## Module Architecture

<script src="https://unpkg.com/cytoscape@3.30.4/dist/cytoscape.min.js"></script>

<div id="module-graph"></div>
<div id="mg-controls">
  <div id="mg-legend">
    <span class="swatch" style="background:#c8f0c8;"></span>Core
    <span class="swatch" style="background:#c8c8f0;"></span>Models
    <span class="swatch" style="background:#f0d8b0;"></span>Rules
    <span class="swatch" style="background:#b0e0f0;"></span>Registry
    <span class="swatch" style="background:#d0f0d0;"></span>Solvers
    <span class="swatch" style="background:#e0e0e0;"></span>Utilities
  </div>
</div>
<div id="mg-help">
  Click a module to expand/collapse its public items.
  Double-click to open rustdoc.
</div>
<div id="mg-tooltip"></div>

## Problem Model

Every problem implements `Problem`. The associated `Value` type is the per-configuration aggregate returned by `evaluate()`. Solvers fold these values across the configuration space, and witness-capable aggregates can also recover representative configurations.

```rust,ignore
trait Problem: Clone {
    const NAME: &'static str;              // e.g., "MaximumIndependentSet"
    type Solution;                         // e.g., Vec<bool>, permutation, tuple
    type Value: Clone;                     // e.g., Max<i64>, Or, Sum<i64>
    fn size_parameter_names() -> &'static [&'static str];
    fn size(&self) -> ProblemSize;
    fn evaluate(&self, solution: &Self::Solution) -> Result<Self::Value, EvaluationError>;
    fn variant() -> Vec<(&'static str, &'static str)>; // e.g., [("graph", "SimpleGraph"), ("weight", "i64")]
    fn problem_type() -> ProblemType;      // default: registry lookup by NAME
}
```

- **`Problem`** — the base trait. Every problem declares a mathematical `Solution` type, evaluates that type directly, and measures its concrete instance size. For example, a 4-vertex MIS uses `Vec<bool>`; `evaluate(&[true, false, true, false])` returns `Ok(Max(Some(2)))` if vertices 0 and 2 form an independent set, or `Ok(Max(None))` if they share an edge. Inherent getters such as `num_vertices()` and `num_edges()` supply the named size parameters used by reduction expressions.
- **`BruteForceProblem`** — the reference-solver capability for registered variants with a finite Cartesian coordinate space. Its `dimensions()` method and the Cartesian iterator belong to the brute-force solver, not to the mathematical `Problem` contract.
- **Objective problems** — typically use `Max<V>`, `Min<V>`, or `Extremum<V>` as `Value`.
- **Feasibility problems** — typically use `Or`.
- **Solve contract** — a successful solve always returns the problem's `Solution`; a global count or statistic without a representative solution is not a `Problem` solve.
- **Common aggregate wrappers** — `Max<V>`, `Min<V>`, `Sum<W>`, `Or`, `And`, `Extremum<V>`, `ExtremumSense`.

## Numeric types and arithmetic

Numeric formats are selected by semantic role:

- `usize` represents in-memory indices, collection lengths, and brute-force
  dimensions;
- `u64` represents public problem size parameters and the input/output values
  of reduction size expressions;
- `i64` represents signed mathematical integers;
- `bool` represents Boolean variables; and
- finite `f64` represents real or rational values when an approximate
  representation is part of the model contract.

`usize` is not a portable serialized size format, and `u64` is not an index or
general-purpose replacement for a model's mathematical integer domain.

Another numeric format requires sufficient justification from the mathematical
problem or target schema. Required exceptions include `BigUint` in `Factoring`,
`SubsetSum`, `SubsetProduct`, `QuadraticCongruences`, and
`QuadraticDiophantineEquations`, where arbitrary precision is part of the
problem, and `One` in unweighted variants, where the type represents the
unit-weight domain. Implementation convenience is not sufficient justification.
There is no `i32` model or I/O numeric format.

This contract applies only at model, result, reduction-target, and external I/O
boundaries; implementation-local values are outside its scope. For example,
SpinGlass couplings and its objective result use `i64`, while the temporary
`{−1, +1}` spin values used inside `evaluate()` need not. A reduction's
temporary calculations are also outside the contract, but numeric fields
written into its target model must follow the target model's numeric format.

Weight variants are `One`, `i64`, and `f64`, with `One ⊂ i64 ⊂ f64`.
`i64 → f64` is a fallible reduction using a checked conversion in
`±(2^53-1)`, not `as f64`.

### Arithmetic

- Keep arithmetic in the declared type. Exact values use checked `i64`
  operations; approximate values use finite `f64` operations.
- Constructors and reductions reject an arithmetic step that would overflow
  `i64` when producing a stored field. They do not cap every magnitude at
  `2^53-1`. `evaluate()` never widens, wraps, saturates, or silently
  approximates.
- Do not promote an `i64` calculation to `i128`, `BigInt`, or `BigUint` to
  accept a larger instance.

### Boundaries

- Use `From` only for value-preserving conversions and `TryFrom` when range,
  sign, or domain can change. Do not use `as` for model-derived values.
- Converting a registered size getter from `usize` to `u64` is an internal
  invariant of `Problem::size()`, not a recoverable construction error. A valid
  instance's registered size parameters must already fit `u64`; the
  implementation checks this conversion to prevent silent truncation.
- Symbolic size evaluation may use arbitrary-precision integers for local
  intermediate arithmetic, but a materialized `ProblemSize` must fit `u64`.
- An `i64` to `f64` conversion is explicit and fallible: it succeeds only
  for `|value| ≤ 2^53-1`. Use one shared helper at weight casts, solver
  adapters, and other exact-to-float hubs.
- A lattice subtype converts to `UnitDiskGraph` through a fallible reduction,
  not an infallible parent cast. The reduction rejects a coordinate conversion
  if the stored `f64` geometry would change source adjacency.
- Rust constructors keep `i64` fields as `i64`. CLI and MCP JSON encoding
  of an `i64` with `|value| > 2^53-1` errors; there is no string encoding
  and no clamping.

## Variant System

A single problem name like `MaximumIndependentSet` can have multiple **variants** — carrying weights on vertices, or defined on a restricted topology (e.g., king's subgraph). Variants form a subtype hierarchy: independent sets on king's subgraphs are a subset of independent sets on unit-disk graphs. The reduction from a more specific variant to a less specific one is a **variant cast**: configuration indices are preserved, while target construction remains fallible when its numeric representation is narrower.

<div class="theme-light-only">

![Variant Hierarchy](static/variant-hierarchy.svg)

</div>
<div class="theme-dark-only">

![Variant Hierarchy](static/variant-hierarchy-dark.svg)

</div>

Variant types fall into three categories:

- **Graph type** — `SimpleGraph` (root), `PlanarGraph`, `BipartiteGraph`, `UnitDiskGraph`, `KingsSubgraph`, `TriangularSubgraph`.
- **Weight type** — `One` (unweighted), `i64`, `f64`.
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

```rust,ignore
pub trait VariantParam: 'static {
    const CATEGORY: &'static str;     // e.g., "graph", "weight", "k"
    const VALUE: &'static str;        // e.g., "SimpleGraph", "i64"
    const PARENT_VALUE: Option<&'static str>;  // None for root types
}
```

Types with a parent also implement `CastToParent`, providing the runtime conversion for variant casts:

```rust,ignore
pub trait CastToParent: VariantParam {
    type Parent: VariantParam;
    fn cast_to_parent(&self) -> Self::Parent;
}
```

### Registration with `impl_variant_param!`

The `impl_variant_param!` macro implements `VariantParam` (and optionally `CastToParent` / `KValue`) for a type:

```rust,ignore
// Root type (no parent):
impl_variant_param!(SimpleGraph, "graph");

// K root (arbitrary K):
impl_variant_param!(KN, "k", k: None);

// Specific K with parent:
impl_variant_param!(K3, "k", parent: KN, cast: |_| KN, k: Some(3));
```

### Variant cast reductions with `impl_variant_reduction!`

When a more specific variant needs to be treated as a less specific one, an explicit variant cast reduction is declared:

```rust,ignore
impl_variant_reduction!(
    MaximumIndependentSet,
    <UnitDiskGraph, i64> => <SimpleGraph, i64>,
    fields: [num_vertices, num_edges],
    |src| MaximumIndependentSet::new(
        src.graph().cast_to_parent(), src.weights().to_vec())
);
```

### Composing `Problem::variant()`

The `variant_params!` macro composes the `Problem::variant()` body from type parameter names:

```rust,ignore
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

```rust,ignore
#[derive(Debug, Clone)]
pub struct ReductionISToVC<W> {
    target: MinimumVertexCover<SimpleGraph, W>,
}

impl<W: WeightElement + VariantParam> ReductionResult for ReductionISToVC<W> {
    type Source = MaximumIndependentSet<SimpleGraph, W>;
    type Target = MinimumVertexCover<SimpleGraph, W>;

    fn target_problem(&self) -> &Self::Target { &self.target }
    fn extract_solution(
        &self,
        target_sol: &[usize],
    ) -> crate::rules::ExtractionResult<Vec<usize>> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_sol)?;
        Ok(target_sol.iter().map(|&x| 1 - x).collect())
    }
}
```

### Solution extraction contract

`ReductionResult::extract_solution` accepts one complete target configuration
and returns the source configuration defined by the reduction. Extraction is a
fallible boundary, not a recovery mechanism:

1. In every direct extractor, call `validate_target_solution()` once before
   indexing or decoding. Composed extractors delegate this check.
2. Validate any structure required by the inverse mapping, such as exactly-one
   blocks, permutations, paths, flows, or schedules.
3. Apply the reduction's mathematical inverse once and return a source
   configuration with the required length and domains.
4. Return `ExtractionError` when a precondition is not satisfied.

Do not truncate or pad input, substitute zero for missing data, select the
first of several invalid candidates, retry with another mapping, or panic on
caller-provided configuration data. Empty and singleton instances should flow
through the same mathematical mapping unless the reduction itself has a
genuine mathematical case distinction.

Zero and sentinel values remain valid when the source model explicitly gives
them meaning. For example, `MaximumCommonEdgeSubgraph` includes an "unmapped"
sentinel in its source dimensions. Missing target data must never be
interpreted as that sentinel.

Each conditional in an extractor should therefore either reject a named
invariant violation or implement a case in the reduction's mathematics. A
normal extractor has one validation phase followed by one decoding phase; it
does not accumulate compatibility or fallback branches.

The `#[reduction]` attribute on the `ReduceTo<T>` impl registers the reduction in the global registry (via `inventory`):

```rust,ignore
#[reduction(size = exact {
    num_vertices = "num_vertices",
    num_edges = "num_edges",
})]
impl ReduceTo<MinimumVertexCover<SimpleGraph, i64>>
    for MaximumIndependentSet<SimpleGraph, i64>
{
    type Result = ReductionISToVC<i64>;
    fn reduce_to(&self) -> Self::Result { /* ... */ }
}
```

<details>
<summary>What the <code>#[reduction]</code> macro generates</summary>

The `#[reduction]` attribute expands to the original `impl` block plus an `inventory::submit!` call:

```rust,ignore
inventory::submit! {
    ReductionEntry {
        source_name: "MaximumIndependentSet",
        target_name: "MinimumVertexCover",
        source_variant_fn: || <MaximumIndependentSet<SimpleGraph, i64> as Problem>::variant(),
        target_variant_fn: || <MinimumVertexCover<SimpleGraph, i64> as Problem>::variant(),
        size_declarations_fn: || ReductionSizeDeclarations {
            relation: Some(SizeRelation::Exact),
            fields: vec![
                ("num_vertices", Expr::Var("num_vertices")),
                ("num_edges", Expr::Var("num_edges")),
            ],
            unavailable: vec![],
        },
        module_path: module_path!(),
        reduce_fn: |src: &dyn Any| -> Box<dyn DynReductionResult> {
            let src = src.downcast_ref::<MaximumIndependentSet<SimpleGraph, i64>>().unwrap();
            Box::new(ReduceTo::<MinimumVertexCover<SimpleGraph, i64>>::reduce_to(src))
        },
    }
}
```

Each `ReductionEntry` is collected by `inventory` at link time and iterated at runtime, making every reduction discoverable by `ReductionGraph` without manual registration. The `reduce_fn` field provides a type-erased executor that enables dynamically discovered paths to chain reductions automatically.

</details>

## Reduction Graph

`ReductionGraph::new()` iterates all registered `ReductionEntry` items (via `inventory`) and builds a variant-level directed graph:

- **Nodes** are unique `(problem_name, variant)` pairs — e.g., `("MaximumIndependentSet", {graph: "KingsSubgraph", weight: "i64"})`.
- **Edges** come exclusively from `#[reduction]` registrations — both cross-problem reductions and variant casts. There are no auto-generated edges.

Exported files:

- [reduction_graph.json](reductions/reduction_graph.json) — all problem variants and reduction edges
- [problem_schemas.json](reductions/problem_schemas.json) — field definitions for each problem type

These JSON assets are generated during `make doc`, `make mdbook`, and `make paper`; they are build artifacts, not committed source files.
Generate them manually with `cargo run --example export_graph` and `cargo run --example export_schemas` when you need the raw exports locally.

### Path finding

All path-finding operates on **exact variant nodes**. Use `ReductionGraph::variant_to_map(&T::variant())` to convert a `Problem::variant()` into the required `BTreeMap<String, String>`.

| Method | Algorithm | Use case |
|--------|-----------|----------|
| `find_all_paths(src, src_var, dst, dst_var)` | All simple paths | Enumerate every route |
| `compose_path_size_transform(path)` | Symbolic composition | Compose each rule's exact or upper-bound size relation while preserving its promise |

A rule has one relation for all of its formulas: either an exact equality or an upper
bound. Composition keeps exact formulas exact only when every step is exact; every other
combination is an upper bound. Concrete-instance measurement remains a separate execution
API.

**Example:** Finding a path from `MIS{KingsSubgraph, i64}` to `VC{SimpleGraph, i64}`:

```
MIS{KingsSubgraph,i64} -> MIS{UnitDiskGraph,i64} -> MIS{SimpleGraph,i64} -> VC{SimpleGraph,i64}
     variant cast              variant cast                reduction
```

### Executable paths

Convert a `ReductionPath` into a typed `ExecutablePath<S, T>` via `make_executable()`, then call `reduce()`:

```rust,ignore
let paths = graph.find_all_paths_mode(
    "Factoring", &src_var, "SpinGlass", &dst_var, ReductionMode::Witness,
);
let rpath = paths.iter()
    .find(|path| path.type_names() == ["Factoring", "CircuitSAT", "SpinGlass"])
    .expect("required route");

// make_executable converts it into a typed, callable chain
let path = graph.make_executable::<Factoring, SpinGlass<SimpleGraph, f64>>(&rpath).unwrap();

// reduce() applies each step, returning a ChainedReduction
let reduction = path.reduce(&factoring_instance);
let target: &SpinGlass<SimpleGraph, f64> = reduction.target_problem();
let solution: Vec<usize> = reduction.extract_solution(&target_solution);
```

`ExecutablePath` holds a type-erased `ReduceFn` per edge. `reduce()` applies them sequentially, producing a `ChainedReduction` that stores each intermediate result. `extract_solution` maps the final solution back through the chain in reverse order.

For full type control, you can also chain `ReduceTo::reduce_to()` calls manually at each step.

<details>
<summary>Size contracts</summary>

Each reduction declares one relation for all represented target-size fields and may mark
other fields unavailable with a reason. The `#[reduction]` macro parses every formula into
the canonical `Expr` DAG at compile time:

```rust,ignore
#[reduction(
size = upper_bound {
    num_vars = "num_vertices + num_edges",
    num_clauses = "3 * num_edges",
},
unavailable = {
    encoding_bits = "coefficient magnitudes are not tracked",
},
})]
impl ReduceTo<Target> for Source { ... }
```

`SizeTransform` uses exact rational and arbitrary-precision integer arithmetic. Exact
relations must evaluate to non-negative integers, while upper-bound results round rational
values upward. Missing fields, negative or non-integral exact results, division by zero,
and explicit conversion outside `usize` are errors.

Transforms can be evaluated with an explicit source size:

```
Input:  ProblemSize { num_vertices: 10, num_edges: 15 }
Output: ProblemSize { num_vars: 25 }
```

For multi-step paths, `compose_path_size_transform` substitutes each step into the next.
When only upper bounds are known for the intermediate fields, a downstream polynomial is
first fully expanded and like monomials are combined; terms with non-positive coefficients
are then removed before substitution. For example, `m <= n^2` followed by `k = 10 - m`
produces the sound bound `k <= 10`, while
`e' = v(v - 1)/2 - e` produces `e' <= v^2/2`. A non-polynomial downstream formula cannot
propagate symbolic upper bounds and reports an error. Projection to `Growth` is a separate
terminal operation used for Big-O path comparison.

</details>

## Solvers

The reference solver exposes a direct typed operation:

```rust,ignore
BruteForce::solve(&problem) -> Result<Option<P::Solution>, SolveError>
```

`Some(solution)` is a successful exact solve, `None` means exhaustive search
proved infeasibility, and `Err` reports an operational failure.

| Solver | Description |
|--------|-------------|
| **BruteForce** | Enumerates a registered finite search space and returns an optimal or satisfying solution. Used for testing and verification. |
| **ILPSolver** | Solves `ILP<bool>` and `ILP<i64>` instances directly with HiGHS via `good_lp`. Also provides `solve_reduced::<V, _>()` for problems that implement `ReduceTo<ILP<V>>`. |

## JSON Serialization

All problem types support JSON serialization via serde:

```rust,ignore
use problemreductions::io::{to_json, from_json};

let json: String = to_json(&problem)?;
let restored: MaximumIndependentSet<SimpleGraph, i64> = from_json(&json)?;
```

## Contributing

See [Call for Contributions](./introduction.md#call-for-contributions) for the recommended issue-based workflow (no coding required).
