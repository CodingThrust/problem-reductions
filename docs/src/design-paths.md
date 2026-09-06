# Path costs and overhead

`ReductionGraph` searches a directed graph of exact `(name, variant)` pairs. Registered reductions carry capabilities; natural variant connections reflect graph and weight subtype relations.

## Choose a cost

| Cost | Purpose |
|---|---|
| `MinimizeSteps` | Fewest reduction steps |
| `Minimize("field")` | Cost based on an output size field |
| `CustomCost(closure)` | User-defined edge cost from overhead and current size |

`find_cheapest_path` accepts source/target variant maps, an input `ProblemSize`, and a cost function. `find_all_paths` enumerates simple paths. Use bounded enumeration when exploring a large graph.

## Interpret an overhead

```rust,ignore
#[reduction(overhead = {
    num_vars = "num_vertices + num_edges",
    num_clauses = "3 * num_edges",
})]
```

Expressions refer to getters on the source type. The macro validates those names at compile time. The metadata describes scaling bounds; it does not promise exact target counts for each input.

For a concrete instance, inspect the constructed target. For a chain, `path_overheads` returns each edge's expressions and `compose_path_overhead` substitutes them to obtain an end-to-end bound.

## Keep costs separate from solve time

A shorter route can produce a harder target. Compare target sizes and solver measurements as well as hop counts; do not present symbolic overheads as measured runtime.

Next: [execute a path](rust-paths.md) or [download the graph](reductions/reduction_graph.json).
