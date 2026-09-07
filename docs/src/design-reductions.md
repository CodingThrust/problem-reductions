# Reduction contracts

A reduction connects exact source and target variants. Its contract determines how a result can be recovered.

| Capability | Contract | Example |
|---|---|---|
| Witness | `ReduceTo<T>` and `ReductionResult::extract_solution` | Solve a target, recover a source configuration |
| Aggregate | `ReduceToAggregate<T>` and `AggregateReductionResult::extract_value` | Solve a target value, recover a source value |
| Turing | Multiple target queries | Optimize by querying a decision problem at several bounds |

Search defaults to witness mode. Use `ReductionMode::Aggregate` or `ReductionMode::Turing` for the corresponding graph queries; do not replay those edges as a single witness mapping.

## Register a witness reduction

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

This is a schematic declaration. Read a [complete implementation](https://github.com/CodingThrust/problem-reductions/blob/main/src/rules/maximumindependentset_minimumvertexcover.rs) for the result type and constructor.

The attribute requires overhead metadata and registers witness/configuration reductions. Aggregate and Turing edges currently use manual `ReductionEntry` registration. Keep one primitive registration for each exact endpoint pair.

## Check correctness

Prove that the construction and extraction preserve the required result. For a small witness example, solve source and target independently, extract the target solution, and compare its source evaluation with the direct optimum.

Next: [overhead semantics](design-paths.md) or [implementation workflow](agent-pipeline.md).
