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

Every problem implements `Problem`. The associated `Value` type is the per-configuration aggregate returned by `evaluate()`. The brute-force solver folds these values across the configuration space and uses its `SolutionAggregate` capability to select corresponding witnesses. Specialized solvers and ILP backends return their solutions directly; model evaluation does not require that selection capability.

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
- **`BruteForceProblem`** — the reference-solver capability for registered variants with a finite Cartesian coordinate space. Its fallible `num_variables()` and `dimension(variable)` methods describe coordinates without allocating their vector. These methods and the Cartesian iterator belong to the brute-force solver, not to the mathematical `Problem` contract.
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

### Construction and deserialization

Model constructors own instance checks and normalization. Fallible constructors
return `Result<Self, ConstructionError>`; Serde calls those constructors through
`#[serde(try_from = "...")]` or a manual `Deserialize` implementation. A model's
`CreateSpec` handles its input names, defaults, and inference, then calls the same
constructor. Nested graphs validate their own endpoints.

For example, `MaximumIndependentSet::new(SimpleGraph::path(3), vec![1, 1])`
returns an error because three vertices require three weights. Loading the same
instance from JSON also fails during construction, before evaluation or solving.
Supplying three weights creates the same mathematical instance through either
entry point. A reduction propagates target construction failures with
`ReduceTo::target_construction`, preserving the source and target model types.

Cached dimensions, adjacency lists, and other derived fields are rebuilt from
validated inputs during deserialization. Persisted cache values do not override
those computations. Setters that can violate an instance invariant check their
replacement data before assigning it; failure leaves the instance unchanged.

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

Supported weight variants are `One`, `i64`, and `f64`.

### Responsibility boundaries

| Layer | Contract |
|-------|----------|
| Model (`Problem`) | Defines instances, witnesses, feasibility, and objectives in its declared mathematical representation. Evaluation is independent of backend tolerances, statuses, and enumeration capacity. |
| Reduction (`ReduceTo`, `ReductionResult`) | Constructs the target within the rule's mathematical domain and maps target witnesses satisfying the stated preconditions to source witnesses. It owns coefficient arithmetic, parameter relationships, and mapping correctness. |
| Backend adapter | Encodes the target, executes the backend, interprets statuses, decodes numerical results, and validates the returned witness against the original target model. |
| Solver orchestration | Executes registered capabilities and reduction chains, interprets aggregate results, and extracts source witnesses under the reduction contracts. |
| CLI / MCP | Uses public construction, evaluation, and solving APIs and presents their results. |

Models and rules do not repair backend results, change constraints to make a
solver succeed, or independently prove a backend's global optimality. Invalid
returned witnesses and operational failures must be explicit errors. A backend's
numerical limitations do not justify a package-wide certificate system or
downgrading every successful result.

Search-space cardinalities belong to the solver capability, not the mathematical
model. Actual model storage and witness representation constraints still apply.

### Witness and aggregate reductions

`ReductionResult::extract_solution()` maps `Target::Solution` to
`Source::Solution`; it does not require equal `Problem::Value` types. Resolve
concrete associated types from the implementation, then check the mathematical
mapping and its Rust implementation rather than applying a wrapper-pair whitelist.

For an optimization reduction, explain why target optima map to source optima.
Opposite directions are valid when the objective relationship reverses order:
independent-set size `k` corresponds to vertex-cover size `n-k` by complementing
the witness. Different numeric value types do not require conversion of an
objective that the extractor never converts. Check the domain and arithmetic of
conversions the construction or mapping actually performs.

Value-only operations use `ReduceToAggregate` / `AggregateReductionResult` and
must justify their actual `extract_value()` relationship. Multi-query algorithms
use the existing Turing reduction capability. A feasibility witness alone does
not establish an optimization result without the required mathematical argument.

`Problem::evaluate()` defines feasibility as well as objective values. A successful
call can return an infeasible value such as `Or(false)` or `Max(None)`; absence
of an `EvaluationError` does not imply a valid witness. The adapter validates
backend output before returning it. Both typed extraction and `pred extract`
assume witnesses satisfying the reduction's documented premises; neither checks
feasibility or optimality. JSON parsing and type conversion remain at the transport
boundary. Evaluation may supply requested display values without acting as an
acceptance gate. Solver orchestration interprets aggregate mappings to determine
source outcomes before invoking witness mappings.

### Executed reduction lifecycle

A witness reduction is one algorithm with construction and reverse mapping.
`reduce_to()` returns the target and all mapping state in one result. Each
executed chain step constructs that result once. Its witness and optional
aggregate `Rc` views share one allocation; obtaining another view does not
reconstruct or copy the target. `Decision<P> -> P` stores the bound with that
same result.

For every rule, document its instance domain, required target witness quality
and conditions, source guarantee, and treatment of source infeasibility.
The guarantee applies to every qualifying witness, including tied optima.
A witness-capable edge alone does not establish a complete-solving procedure:
composition must establish the preceding edge's witness premise.

| Example | Required recovery |
|---|---|
| MVC -> MIS | Complement a maximum independent set to obtain a minimum cover |
| SAT -> MIS | With `m` clauses, optimum size `m` permits witness extraction; an optimum below `m` means UNSAT |
| Binary ILP -> QUBO | Use the constructed energy relationship to obtain a source optimum or source infeasibility; a QUBO optimum alone does not establish ILP feasibility |
| MVC -> MIS -> SetPacking -> ILP | Apply the stored ILP-to-packing and packing-to-MIS mappings, then the complement mapping |
| TSP -> QUBO | Shift signed edge costs uniformly; the energy threshold distinguishes source infeasibility, and the stored offset recovers tour cost |
| Discrete inverse kinematics -> QUBO | Restore omitted constants and compare against the gap between feasible distance and constraint penalties before decoding orientations |
| MultiwayCut -> QUBO | Always delete negative edges; optimize nonnegative cut cost and decode an optimal terminal partition |
| Aggregate-only operation | Map the final value without selecting any witness, including `Sum` |

The mathematical thresholds and objective relationships belong to the rule.
Solver completion invokes the executed step's concrete `interpret_optimum`
operation before its witness mapping. This operation shares the constructed
result and does not query the model registry. Ordinary extraction uses only the
witness mapping. Typed chain, executed path, and JSON extraction share the same
reverse traversal; dynamic/JSON methods perform necessary representation
conversion rather than introducing another extraction contract.

`SolutionAggregate` is defined in `solvers/brute_force.rs` and exported through
`solvers` for enumeration clients. It compares candidate and aggregate values;
it is not a model-feasibility interface. Concrete variant declarations generate
`DynProblem` transport implementations using the value's own `is_valid`
semantics, without aggregation or solver-registration requirements. A concrete
hand-registered dynamic type can use `impl_dyn_problem!` directly.

Witness and aggregate describe what can be recovered. Turing describes a
potentially adaptive query procedure. Exact witness recovery does not establish
approximation or counting preservation; those require their own proofs.

### Arithmetic

- Integer models and reductions preserve integer values in their declared
  representation. Report actual arithmetic overflow explicitly; do not wrap,
  saturate, or silently approximate. Reuse an existing exact representation
  when the mathematical model requires it.
- Floating-point models and rules use ordinary finite `f64` arithmetic and its
  rounding. Check non-finite results and do not deliberately discard nonzero
  coefficients. Backend feasibility tolerances must not expand the model's
  feasible set. A declared input convention, such as checking probability sums,
  is distinct from accepting a solver's returned assignment.
- CVP evaluates squared distance as `Min<BigRational>` through its
  `squared_distance()` method. Integer coordinates enter exact integer arithmetic;
  finite `f64` targets retain their stored binary rational values. For example,
  the zero lattice point and target `(3, 4)` have objective `25`. The customized
  solver uses the same coordinate conversion. SubsetSum compares squared distance
  with its integer item count. JSON evaluation uses the dependency's rational
  serialization; CLI display uses fractions such as `Min(9/16)`.
- `i64_to_exact_f64()` accepts integers in `[-(2^53-1), 2^53-1]` and rejects
  everything outside that supported conversion range. This is a conservative
  interface limit, not the set of all exactly representable f64 integers. Ordinary conversion
  into a floating-point model and backend transport are separate
  responsibilities. Neither a lossless scalar conversion nor `transform = exact`
  proves error-free floating-point evaluation or backend optimality; the latter
  describes parameter relationships only.
- Preserve real construction and witness-structure checks, including bounds
  derived by the reduction and adjacency preservation in geometric mappings.
  Do not add exact arithmetic solely to audit a floating-point backend or reject
  a mathematical reduction because that backend may struggle to solve it.

### Boundaries

- Use value-preserving conversions where possible and checked conversions for
  range/sign changes. A floating-point model's declared rounding is not a
  lossless-conversion requirement. Reuse `i64_to_exact_f64` where lossless scalar
  conversion is actually required; backend input acceptance belongs to the
  adapter and must not narrow integer model domains.
- Converting a registered parameter getter from `usize` to `u64` is an internal
  invariant of `Problem::parameters()`, not a recoverable construction error.
  Check it to prevent silent truncation.
- Symbolic parameter evaluation may use arbitrary-precision intermediates, but
  materialized `ProblemParameters` must fit `u64`.
- A lattice-to-`UnitDiskGraph` reduction must reject a stored geometry that
  changes source adjacency. This is a mathematical reduction requirement.
- Rust constructors retain their declared integer fields. Existing serde JSON
  serialization can emit i64 integer values beyond the consecutive-integer range
  of f64. Describe the actual codec and consumer representation; do not impose
  a universal f64 gate on Rust models or claim one exists in CLI/MCP.

### Validation evidence

Model tests check definitions and direct evaluation. Reduction tests check
construction, witness mappings, objective relationships, and parameter formulas
using explicit witnesses or small exhaustive enumeration. Choose cases that can
expose a concrete defect; there is no minimum vertex, assertion, test-function,
or generated-check count that establishes correctness.

Keep representative solver integration tests and report whether failures occur
in construction, solving, extraction, or source validation. Backend timeout or
numerical failure is not evidence that a reduction theorem is false. Test backend
decoding and transport boundaries once in their shared implementation, not in
every rule. Retain arithmetic regressions that detect actual coefficient loss or
incorrect mappings. Do not enlarge tolerances to make a failing test pass.

### Search representation

`BruteForceProblem::num_variables()` and `dimension(variable)` return
`Result<usize, SolveError>`. The caller supplies an index below the coordinate
count. Derived counts and cardinalities use checked arithmetic. Shared
`cartesian_dimensions()` materializes these values with fallible allocation for
registered solving and inspection; models do not call it during evaluation.

The Cartesian iterator advances coordinates until mixed-radix exhaustion. Its
complete search count need not fit `usize`, and it does not implement
`ExactSizeIterator`. An empty product has one empty candidate; any zero-sized
coordinate makes the product empty. Native masks and dense tables retain their
actual representation limits and report errors before overflowing or allocating
an unrepresentable table. These are implementation limits, not difficulty budgets.

`TruthTable` construction and deserialization share checked row-count and shape
validation. Variable-arity constructors return `ConstructionError` for unsupported
row counts or allocation failures. Valid tables retain the same JSON format.

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
    aggregate: identity,
    |src| {
        let construction_error = ReductionError::construction::<
            MaximumIndependentSet<UnitDiskGraph, i64>,
            MaximumIndependentSet<SimpleGraph, i64>,
        >;
        let graph = SimpleGraph::new(src.num_vertices(), Graph::edges(src.graph()))
            .map_err(construction_error)?;
        MaximumIndependentSet::new(graph, src.weights().to_vec())
            .map_err(construction_error)?
    }
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
        Ok(target_sol.iter().map(|&x| !x).collect())
    }
}
```

### Solution extraction contract

`ReductionResult::extract_solution` maps a complete target solution satisfying
the rule's mathematical premises into a source solution. The adapter establishes
target validity for internal solves. External callers supply witnesses under the
same contract. Rules requiring optimal target solutions document that requirement.
Source YES/NO and optimization outcomes are interpreted by solver orchestration,
not by the extraction chain. Invalid external witnesses have no mapping-correctness
guarantee.

Do not repeat checks implied by target constraints or successful construction.
Do not truncate or pad input, substitute values for missing data, retry another
mapping, or add runtime acceptance checks to compensate for a rule defect.
Keep actual mathematical case distinctions and representation errors that can
occur for inputs satisfying the mapping's premises.

Zero and sentinel values remain valid when the source model explicitly gives
them meaning. For example, `MaximumCommonEdgeSubgraph` includes an "unmapped"
sentinel in its source dimensions. Missing target data must never be
interpreted as that sentinel.

Each conditional in an extractor should implement a case in the reduction's
mathematics or report an error that remains reachable under its premises.
The external boundary handles parsing and type conversion; extraction does not
accumulate feasibility checks, compatibility branches, or fallbacks.

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
| **ILPSolver** | Executes a problem's registered ILP pipeline. Each pipeline terminates at a native `ILP<V, C>` with bool/i64 variables and i64/f64 coefficients, solved by the shared HiGHS adapter through its native Rust bindings. |

### ILP execution boundary

`ILPSolver::solve<P>() -> Result<P::Solution, ILPSolveError>` is the typed entry
point. Adapter failures retain their classified errors. Registry lookup,
concrete-terminal dispatch, aggregate interpretation,
and reduction-chain extraction belong to orchestration. Integer pipelines end
at native integer ILPs; they do not need a float-coefficient cast edge to execute.
Explicit coefficient-conversion rules retain their own mathematical contracts.

The shared internal `HighsAdapter` borrows an `ILP<V, C>` and returns its existing
`Vec<i64>` witness representation. It encodes the backend model, executes it,
interprets termination, checks returned integer values, and validates constraints
and objective arithmetic against the original ILP. It does not inspect source
model names, query reduction registrations, or extract source witnesses.
Unsupported transport produces `InexactTransport`; a rejected returned witness
produces `InvalidSolution`. A validation failure must not relax model constraints.

Optimality and infeasibility are backend conclusions under HiGHS's numerical
contract, not independent mathematical certificates. An accepted optimum requires
both an optimal backend termination and successful witness validation. Timeouts,
non-optimal termination, and invalid results are errors, not infeasibility.
Variable decoding tolerances belong to the adapter; they do not define source or
target feasibility, nor a universal objective-error allowance for tests.

After accepting a target optimum, orchestration must apply the reduction's
aggregate mapping to interpret a source decision threshold. If that optimum
cannot meet the threshold, the source answer is NO. A merely feasible witness
or failed solve is insufficient for that conclusion. Typed solving, dynamic
solving, and explicit CLI bundles must share the same interpretation and witness
mapping.

Fixed pipelines and explicit CLI bundles reuse the executed `ReductionChain`
and the solver completion path. Aggregate mappings interpret an accepted target
optimum before witness extraction. Source evaluation computes requested output
values and propagates evaluation errors; it is not another feasibility gate.

## JSON Serialization

All problem types support JSON serialization via serde:

```rust,ignore
use problemreductions::io::{to_json, from_json};

let json: String = to_json(&problem)?;
let restored: MaximumIndependentSet<SimpleGraph, i64> = from_json(&json)?;
```

## Contributing

See [Call for Contributions](./open-problems.md) for the recommended issue-based workflow (no coding required).

### QUBO coefficient storage

QUBO stores coefficients in `sprs::CsMat<W>` using CSR order. Construction from
linear/quadratic terms preserves last-assignment semantics; reductions accumulate
coefficients with their existing checked arithmetic before compression. Exact
zeros need no stored entry. Evaluation visits the upper triangle in row/column
order, retaining checked integer addition and floating-point summation order.

`QUBO::from_sparse` accepts a square CSR or CSC matrix; `matrix()` returns the
CSR matrix and `get(i, j)` returns an owned coefficient, including zero for an
unstored in-bounds entry. `from_matrix` and CLI `--matrix` accept dense input.
Persisted QUBO JSON stores the `sprs` matrix object (`storage`, `nrows`, `ncols`,
`indptr`, `indices`, `data`); variable count comes from the matrix dimensions.
Rules, numeric casts, and solver reductions consume sparse coefficients directly.
