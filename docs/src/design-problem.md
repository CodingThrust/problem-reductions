# Problem contract

Every problem implements `Problem`. The associated `Value` type is the per-configuration aggregate returned by `evaluate()`. Solvers fold these values across the configuration space, and witness-capable aggregates can also recover representative configurations.

```rust,ignore
trait Problem {
    const NAME: &'static str;              // e.g., "MaximumIndependentSet"
    type Value: Clone;                     // e.g., Max<i32>, Or, Sum<i32>
    fn dims(&self) -> Vec<usize>;          // config space per variable
    fn evaluate(&self, config: &[usize]) -> Self::Value;
    fn variant() -> Vec<(&'static str, &'static str)>; // e.g., [("graph", "SimpleGraph"), ("weight", "i32")]
    fn num_variables(&self) -> usize;      // default: dims().len()
    fn problem_type() -> ProblemType;      // default: registry lookup by NAME
}
```

- **`Problem`** — the base trait. Every problem declares a `NAME` (e.g., `"MaximumIndependentSet"`). The solver explores the configuration space defined by `dims()` and scores each configuration with `evaluate()`. For example, a 4-vertex MIS has `dims() = [2, 2, 2, 2]` (each vertex is selected or not); `evaluate(&[1, 0, 1, 0])` returns `Max(Some(2))` if vertices 0 and 2 form an independent set, or `Max(None)` if they share an edge. Each problem also provides inherent getter methods (e.g., `num_vertices()`, `num_edges()`) used by reduction overhead expressions.
- **Witness-capable objective problems** — typically use `Max<V>`, `Min<V>`, or `Extremum<V>` as `Value`.
- **Witness-capable feasibility problems** — typically use `Or`.
- **Aggregate-only problems** — use fold values such as `Sum<W>` or `And`; these solve to a value but do not admit representative witness configurations.
- **Common aggregate wrappers** — `Max<V>`, `Min<V>`, `Sum<W>`, `Or`, `And`, `Extremum<V>`, `ExtremumSense`.

Next: [variant registration](design-variant-registration.md).
