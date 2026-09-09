# Design

This guide covers the library internals for contributors.

See [Numeric types and arithmetic](#numeric-types-and-arithmetic) before
choosing numeric fields or implementing arithmetic in a model or reduction.

## Module Architecture

| Location | Responsibility | Depends on |
|---|---|---|
| `src/traits.rs`, `src/types.rs`, `src/variant.rs`, `src/topology/` | Core: the `Problem` trait, aggregate values, variant parameters, graph types | — |
| `src/models/` | Models grouped by graph, formula, set, algebraic, or miscellaneous input | Core |
| `src/rules/` | Reduction implementations and solution/value mappings | Models |
| `src/registry/` | Concrete variant metadata and dynamic dispatch | Rules |
| `src/solvers/` | Exhaustive, ILP, specialized, and decision-search solvers | Core |
| `src/io.rs`, `src/expr.rs` | JSON serialization and overhead expressions | Core |
| `src/example_db/` | Canonical model and rule examples | Models, rules |
| `src/unit_tests/` | Tests mirroring the source tree | Everything |
| `problemreductions-cli/` | The `pred` CLI | The library |

## Problem Model

Every problem implements `Problem`. The associated `Value` type is the per-configuration aggregate returned by `evaluate()`. Solvers fold these values across the configuration space, and witness-capable aggregates can also recover representative configurations.

```rust,ignore
trait Problem: Clone {
    const NAME: &'static str;              // e.g., "MaximumIndependentSet"
    type Solution;                         // e.g., Vec<bool>, permutation, tuple
    type Value: Clone;                     // e.g., Max<i64>, Or, Sum<i64>
    fn parameter_names() -> &'static [&'static str];
    fn parameters(&self) -> ProblemParameters;
    fn evaluate(&self, solution: &Self::Solution) -> Result<Self::Value, EvaluationError>;
    fn variant() -> Vec<(&'static str, &'static str)>; // e.g., [("graph", "SimpleGraph"), ("weight", "i64")]
    fn problem_type() -> ProblemType;      // default: registry lookup by NAME
}
```

- **`Problem`** — the base trait. Every problem declares a mathematical `Solution` type, evaluates that type directly, and reports its canonical instance parameters. For example, a 4-vertex MIS uses `Vec<bool>`; `evaluate(&[true, false, true, false])` returns `Ok(Max(Some(2)))` if vertices 0 and 2 form an independent set, or `Ok(Max(None))` if they share an edge. Inherent getters such as `num_vertices()` and `num_edges()` supply the named parameters used by reduction expressions.
- **`BruteForceProblem`** — the reference-solver capability for registered variants with a finite Cartesian coordinate space. Its `dimensions()` method and the Cartesian iterator belong to the brute-force solver, not to the mathematical `Problem` contract.
- **Objective problems** — typically use `Max<V>`, `Min<V>`, or `Extremum<V>` as `Value`.
- **Feasibility problems** — typically use `Or`.
- **Solve contract** — a successful solve always returns the problem's `Solution`; a global count or statistic without a representative solution is not a `Problem` solve.
- **Common aggregate wrappers** — `Max<V>`, `Min<V>`, `Sum<W>`, `Or`, `And`, `Extremum<V>`, `ExtremumSense`.

## Construction inputs

`VariantEntry::inputs()` describes the values a concrete constructor accepts.
Models with a separate construction specification supply `CreateSpec::inputs()`;
direct constructors use their declared fields. CLI creation, MCP creation, and
`pred show` use this contract. Model-level
catalog fields describe the model family; they are not a concrete variant's input
schema. `show` exposes concrete `inputs` in JSON and labels them **Inputs** in text.

Unit-valued data are implicit in `One` variants. For example, `MVC/One` accepts a
graph, while `MVC/i64` also accepts vertex weights. Constructors derive unit-vector
lengths from the graph, set family, or task deadlines. Internal `Vec<One>` storage
and persisted instance JSON remain independent of construction inputs. Supplying
an undeclared weight or length input is an error, even when every value is one.

`Decision<P>` composes the registered inputs of `P` with an objective `bound` and
calls `P`'s registered constructor before wrapping the result. It does not repeat
the inner input schema or deserialize construction inputs as persisted model JSON.

## Numeric types and arithmetic

Numeric formats are selected by semantic role:

- `usize` represents in-memory indices, collection lengths, and brute-force
  dimensions;
- `u64` represents public problem parameters and the input/output values
  of reduction parameter expressions;
- `i64` represents signed mathematical integers;
- `bool` represents Boolean variables; and
- finite `f64` represents real or rational values when an approximate
  representation is part of the model contract.

`usize` is not a portable serialized parameter format, and `u64` is not an index or
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
- Converting a registered parameter getter from `usize` to `u64` is an internal
  invariant of `Problem::parameters()`, not a recoverable construction error. A valid
  instance's registered parameters must already fit `u64`; the
  implementation checks this conversion to prevent silent truncation.
- Symbolic parameter evaluation may use arbitrary-precision integers for local
  intermediate arithmetic, but a materialized `ProblemParameters` must fit `u64`.
- An `i64` to `f64` conversion is explicit and fallible: it succeeds only
  for `|value| ≤ 2^53-1`. Use one shared helper at weight casts, solver
  adapters, and other exact-to-float hubs.
- A lattice-to-`UnitDiskGraph` reduction converts coordinates fallibly and
  rejects a stored `f64` geometry that would change source adjacency.
- Rust constructors keep `i64` fields as `i64`. CLI and MCP JSON encoding
  of an `i64` with `|value| > 2^53-1` errors; there is no string encoding
  and no clamping.

## Variant System

A single problem name like `MaximumIndependentSet` can have multiple
**variants**. Each variant is identified by dimension-value pairs such as
`{graph: "SimpleGraph", weight: "i64"}`. Concrete variants are registered
nodes in the reduction graph, and explicit reduction rules connect them.

<div class="theme-light-only">

![Variant Dimensions](static/variant-hierarchy.svg)

</div>
<div class="theme-dark-only">

![Variant Dimensions](static/variant-hierarchy-dark.svg)

</div>

Variant types fall into three categories:

- **Graph type** — `SimpleGraph`, `PlanarGraph`, `BipartiteGraph`, `UnitDiskGraph`, `KingsSubgraph`, `TriangularSubgraph`.
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

Each reusable variant parameter type implements `VariantParam`, which declares
its category and value:

```rust,ignore
pub trait VariantParam: 'static {
    const CATEGORY: &'static str;     // e.g., "graph", "weight", "k"
    const VALUE: &'static str;        // e.g., "SimpleGraph", "i64"
}
```

### Registration with `impl_variant_param!`

The `impl_variant_param!` macro implements `VariantParam` and optionally
`KValue` for a type:

```rust,ignore
impl_variant_param!(SimpleGraph, "graph");

impl_variant_param!(KN, "k", k: None);

impl_variant_param!(K3, "k", k: Some(3));
```

### Explicit variant reductions

`impl_variant_reduction!` registers a concrete same-model conversion with an
exact parameter transform and identity witness extraction:

```rust,ignore
impl_variant_reduction!(
    MaximumIndependentSet,
    <UnitDiskGraph, i64> => <SimpleGraph, i64>,
    fields: [num_vertices, num_edges],
    |src| MaximumIndependentSet::new(
        SimpleGraph::new(
            src.num_vertices(),
            Graph::edges(src.graph()),
        ),
        src.weights().to_vec())
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

### Querying one variant family

`ReductionGraph::variants_for(name)` returns every registered concrete variant
of a problem. `ReductionGraph::outgoing_reductions(name)` returns their outgoing
edges. Filtering those edges by `target_name == name` produces the directed
relations within that variant family.

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
        target_sol: &Vec<bool>,
    ) -> crate::rules::ExtractionResult<Vec<bool>> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_sol)?;
        Ok(target_sol.iter().map(|&x| !x).collect())
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
#[reduction(transform = exact {
    num_vertices = "num_vertices",
    num_edges = "num_edges",
})]
impl ReduceTo<MinimumVertexCover<SimpleGraph, i64>>
    for MaximumIndependentSet<SimpleGraph, i64>
{
    type Result = ReductionISToVC<i64>;
    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> { /* ... */ }
}
```

## Reduction Graph

`ReductionGraph::new()` iterates all registered `ReductionEntry` items (via `inventory`) and builds a variant-level directed graph:

- **Nodes** are unique `(problem_name, variant)` pairs — e.g., `("MaximumIndependentSet", {graph: "KingsSubgraph", weight: "i64"})`.
- **Edges** come from explicit `#[reduction]` registrations, including
  cross-problem and same-problem variant reductions.

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
| `compose_path_parameter_transform(path)` | Symbolic composition | Compose each rule's exact or upper-bound parameter relation while preserving its promise |

A rule has one relation for all of its formulas: either an exact equality or an upper
bound. Composition keeps exact formulas exact only when every step is exact; every other
combination is an upper bound. Concrete-instance measurement remains a separate execution
API.

**Example:** Finding a path from `MIS{KingsSubgraph, i64}` to `VC{SimpleGraph, i64}`:

```
MIS{KingsSubgraph,i64} -> MIS{UnitDiskGraph,i64} -> MIS{SimpleGraph,i64} -> VC{SimpleGraph,i64}
    variant reduction        variant reduction              reduction
```

### Executable paths

Execute an explicitly selected path with `ReductionGraph::reduce_along_path`:

```rust,ignore
let reduction = graph.reduce_along_path(rpath, &factoring_instance)?.unwrap();
let target: &SpinGlass<SimpleGraph, f64> = reduction.target_problem();
let source_solution = reduction.extract_solution(&target_solution)?;
```

The returned `ReductionChain` stores each intermediate reduction and extracts the source solution by applying the inverse mappings in reverse order. Construction returns `ReductionError`; extraction returns `ExtractionError`.

<details>
<summary>Parameter contracts</summary>

Each reduction declares one relation for all represented target-parameter fields and may mark
other fields unavailable with a reason. The `#[reduction]` macro parses every formula into
the canonical `Expr` DAG at compile time:

```rust,ignore
#[reduction(
transform = upper_bound {
    num_vars = "num_vertices + num_edges",
    num_clauses = "3 * num_edges",
},
unavailable = {
    encoding_bits = "coefficient magnitudes are not tracked",
},
})]
impl ReduceTo<Target> for Source { ... }
```

`ParameterTransform` uses exact rational and arbitrary-precision integer arithmetic. Exact
relations must evaluate to non-negative integers, while upper-bound results round rational
values upward. Missing fields, negative or non-integral exact results, division by zero,
and explicit conversion outside `u64` are errors.

Transforms can be evaluated with explicit source parameters:

```
Input:  ProblemParameters { num_vertices: 10, num_edges: 15 }
Output: ProblemParameters { num_vars: 25 }
```

For multi-step paths, `compose_path_parameter_transform` substitutes each step into the next.
When only upper bounds are known for the intermediate fields, a downstream polynomial is
first fully expanded and like monomials are combined; terms with non-positive coefficients
are then removed before substitution. For example, `m <= n^2` followed by `k = 10 - m`
produces the sound bound `k <= 10`, while
`e' = v(v - 1)/2 - e` produces `e' <= v^2/2`. A non-polynomial downstream formula cannot
propagate symbolic upper bounds and reports an error. Projection to `Growth` is a separate descriptive terminal operation used for
Big-O display; it does not rank or filter paths.

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
| **ILPSolver** | Executes a problem's registered ILP pipeline. Each pipeline terminates at a native `ILP<V, C>` with bool/i64 variables and i64/f64 coefficients, solved by the shared HiGHS adapter via `good_lp`. |

ILP results are optimal or infeasible according to HiGHS numerical tolerances;
zero MIP gaps do not imply mathematical exactness. Integer extraction rounds
variable assignments, validates the original constraints, and recomputes the
source objective with checked integer arithmetic. Floating-point objective
comparisons in numerical regression tests use an explicit acceptance policy
in source units (absolute and relative tolerances of `1e-7` for the QUBO solver
regression), separate from the `1e-6` variable-rounding tolerance. This test
policy is not a universal bound on backend objective error.

When an ILP target witness misses a source decision threshold, the solver
returns `ILPSolveError::UnresolvedDecision`, not infeasibility: the witness
alone cannot prove that no qualifying source solution exists.

### ILP execution boundary

`ILPSolver::solve<P>() -> Result<P::Solution, ILPSolveError>` remains the public
entry point. Registry lookup, concrete-terminal dispatch, and reduction-chain
extraction live in the orchestration layer. Integer pipelines stop at their
integer ILP instead of constructing a float-coefficient ILP as an extra step.
Existing explicit coefficient-conversion reductions remain available.

The internal `HighsAdapter` borrows an `ILP<V, C>` and returns its existing
`Vec<i64>` solution representation. It builds the backend model, executes it,
checks returned integer values, and validates constraints and objective
arithmetic against the original ILP. It does not inspect variant names, query
registrations, or extract solutions for source problems. Coefficient conversion
is an adapter-local capability; the public `ILPCoefficient` trait is unchanged.
The existing exact-integer transport limits and float-model tolerances remain
in effect. Validating a witness is not an independent optimality certificate;
solver-reported optimality retains its existing numerical contract.

Adapter errors remain internal and map to the existing public `ILPSolveError`
variants. Rust return types, solver configuration, and CLI/JSON/MCP outcome
formats remain unchanged; reported reduction paths now end at native ILPs.
An unsupported integer coefficient is now reported through the existing
`InexactTransport` error at the adapter boundary instead of a cast-reduction
error. Bounds use that same existing transport error. An integer assignment
that violates the original ILP is rejected as `InvalidSolution` by the adapter,
rather than failing later during coefficient-cast extraction.

## JSON Serialization

All problem types support JSON serialization via serde:

```rust,ignore
use problemreductions::io::{to_json, from_json};

let json: String = to_json(&problem)?;
let restored: MaximumIndependentSet<SimpleGraph, i64> = from_json(&json)?;
```

## Contributing

See [Call for Contributions](./open-problems.md) for the recommended issue-based workflow (no coding required).
