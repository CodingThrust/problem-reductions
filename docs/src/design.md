# Design

The Rust library holds problem definitions, executable reductions, and their registry metadata. The CLI exposes that core to tools and agents.

| Location | Responsibility |
|---|---|
| `src/models/` | Models grouped by graph, formula, set, algebraic, or miscellaneous input |
| `src/rules/` | Reduction implementations and solution/value mappings |
| `src/registry/` | Concrete variant metadata and dynamic dispatch |
| `src/solvers/` | Exhaustive, ILP, specialized, and decision-search solvers |
| `src/example_db/` | Canonical model and rule examples |
| `src/unit_tests/` | Tests mirroring the source tree |
| `problemreductions-cli/` | The `pred` CLI |

## Module map

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

## Problem contract

Every problem implements `Problem`. `evaluate()` returns the associated `Value` for one configuration; solvers fold those values across the configuration space defined by `dims()`.

```rust,ignore
trait Problem {
    const NAME: &'static str;              // e.g., "MaximumIndependentSet"
    type Value: Clone;                     // e.g., Max<i32>, Or, Sum<i32>
    fn dims(&self) -> Vec<usize>;          // configuration space per variable
    fn evaluate(&self, config: &[usize]) -> Self::Value;
    fn variant() -> Vec<(&'static str, &'static str)>; // e.g., [("graph", "SimpleGraph"), ("weight", "i32")]
    fn num_variables(&self) -> usize;      // default: dims().len()
    fn problem_type() -> ProblemType;      // default: registry lookup by NAME
}
```

A four-vertex independent set problem has `dims() = [2, 2, 2, 2]`; `evaluate(&[1, 0, 1, 0])` returns `Max(Some(2))` if vertices 0 and 2 are non-adjacent and `Max(None)` otherwise. Witness-capable objective problems use `Max<V>`, `Min<V>`, or `Extremum<V>`; feasibility problems use `Or`; aggregate-only problems such as counting use `Sum<W>` or `And` and solve to a value without a representative configuration. Each problem also provides inherent getters such as `num_vertices()` that reduction overhead expressions reference.

## Variants

One problem name can have several **variants**: weights on vertices, or a restricted topology such as a king's subgraph. Variants form a subtype hierarchy, and the reduction from a more specific variant to a less specific one is a **variant cast**, an identity mapping that preserves indices.

<div class="theme-light-only">

![Variant Hierarchy](static/variant-hierarchy.svg)

</div>
<div class="theme-dark-only">

![Variant Hierarchy](static/variant-hierarchy-dark.svg)

</div>

Variant parameters fall into three categories: graph type (`SimpleGraph` at the root, then `PlanarGraph`, `BipartiteGraph`, `UnitDiskGraph`, `KingsSubgraph`, `TriangularSubgraph`), weight type (`One`, `i32`, `f64`), and K value (`K3` for 3-SAT, `KN` for arbitrary K).

<div class="theme-light-only">

![Lattices](static/lattices.svg)

</div>
<div class="theme-dark-only">

![Lattices](static/lattices-dark.svg)

</div>

Each parameter type implements `VariantParam`, declaring its category, value, and optional parent; types with a parent also implement `CastToParent` for the runtime conversion. `Problem::variant()` is composed from the type parameters with `variant_params![G, W]`. The macros `impl_variant_param!`, `impl_variant_reduction!`, and `declare_variants!` register parameter types, explicit variant casts, and concrete variants with their load, serialize, and solve metadata. Their current contract is documented in the [repository instructions](https://github.com/CodingThrust/problem-reductions/blob/main/.claude/CLAUDE.md) and visible in any model file.

## Reductions

A reduction connects exact source and target variants. Its capability determines how a result is recovered:

| Capability | Contract | Example |
|---|---|---|
| Witness | `ReduceTo<T>` and `ReductionResult::extract_solution` | Solve a target, recover a source configuration |
| Aggregate | `ReduceToAggregate<T>` and `AggregateReductionResult::extract_value` | Solve a target value, recover a source value |
| Turing | Multiple target queries | Optimize by querying a decision problem at several bounds |

Graph search defaults to witness mode; `ReductionMode::Aggregate` and `ReductionMode::Turing` select the others. A witness reduction is registered with the `#[reduction]` attribute, whose `overhead` block is required:

```rust,ignore
#[reduction(overhead = {
    num_vertices = "num_vertices",
    num_edges = "num_edges",
})]
impl ReduceTo<MinimumVertexCover<SimpleGraph, i32>>
    for MaximumIndependentSet<SimpleGraph, i32>
{
    // Provide Result and reduce_to(); the result owns the target
    // and maps a vertex-cover witness to its independent-set complement.
}
```

See a [complete implementation](https://github.com/CodingThrust/problem-reductions/blob/main/src/rules/maximumindependentset_minimumvertexcover.rs) for the result type. Aggregate and Turing edges use manual `ReductionEntry` registration. Keep one primitive registration per exact endpoint pair. Correctness needs a proof that construction and extraction preserve the required result; a closed-loop test on a small example is evidence, not proof.

## Path costs and overhead

`ReductionGraph` searches a directed graph of exact `(name, variant)` pairs. Registered reductions carry capabilities; natural variant connections follow the graph and weight subtype relations.

| Cost | Purpose |
|---|---|
| `MinimizeSteps` | Fewest reduction steps |
| `Minimize("field")` | Cost based on an output size field |
| `CustomCost(closure)` | User-defined edge cost from overhead and current size |

`find_cheapest_path` takes source and target variant maps, an input `ProblemSize`, and a cost; `find_all_paths` enumerates simple paths with a bound. Overhead expressions refer to getters on the source type and are validated at compile time. They describe scaling bounds, not exact target counts: for a concrete instance inspect the constructed target, and for a chain use `path_overheads` and `compose_path_overhead` for an end-to-end bound. A shorter route can produce a harder target, so compare target sizes and solver measurements as well as hop counts.

## JSON serialization

```rust,ignore
use problemreductions::io::{to_json, from_json};

let json: String = to_json(&problem)?;
let restored: MaximumIndependentSet<SimpleGraph, i32> = from_json(&json)?;
```

These helpers serialize typed problem data. The CLI additionally wraps data with `type` and `variant` for dynamic loading; keep that wrapper when passing files between commands.

## Contributing

Propose a model or reduction through the `propose` [skill](skills.md) or the [issue templates](https://github.com/CodingThrust/problem-reductions/issues/new/choose). A useful rule includes exact endpoint variants, a construction, solution or value extraction, a correctness argument, and overhead metadata. The `add-model` and `add-rule` skills list the required code, tests, examples, and paper changes; `.claude/CLAUDE.md` holds the conventions.
