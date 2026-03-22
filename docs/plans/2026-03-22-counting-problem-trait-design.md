# CountingProblem Trait — Supporting #P and PP-Complete Problems

**Date:** 2026-03-22
**Status:** Approved design, pending implementation

## Problem

The current trait hierarchy supports two problem families:

- `OptimizationProblem` (`Metric = SolutionSize<V>`) — find a config that maximizes/minimizes an objective
- `SatisfactionProblem` (`Metric = bool`) — find a config satisfying all constraints

8 issues are blocked because they model problems where the answer depends on **aggregating over the entire configuration space** — counting feasible configs or summing weighted probabilities. These are #P-complete or PP-complete problems (not known to be in NP) that don't fit either existing trait.

### Blocked issues

**Models:** #235 NetworkReliability, #237 NetworkSurvivability, #404 KthLargestSubset, #405 KthLargestMTuple

**Rules (blocked on models above):** #256 SteinerTree → NetworkReliability, #257 VertexCover → NetworkSurvivability, #394 SubsetSum → KthLargestSubset, #395 SubsetSum → KthLargestMTuple

## Design

### New type: `Weight<W>`

A newtype wrapper for per-configuration weights, parallel to `SolutionSize<V>` for optimization problems. Infeasible configs have weight zero — no separate `Infeasible` variant needed (unlike `SolutionSize::Invalid`) because a zero-weight config contributes nothing to the sum.

```rust
// src/types.rs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Weight<W>(pub W);
```

### New trait: `CountingProblem`

A marker trait parallel to `SatisfactionProblem`, binding `Metric = Weight<Self::Value>`:

```rust
// src/traits.rs
pub trait CountingProblem: Problem<Metric = Weight<Self::Value>> {
    /// The inner weight type (e.g., `u64` for unweighted counting, `f64` for probabilities).
    type Value: Clone + AddAssign + Zero + PartialOrd + fmt::Debug + Serialize + DeserializeOwned;
}
```

The `evaluate(config) -> Weight<V>` method (inherited from `Problem`) returns the weight of a single configuration. The "answer" to the problem is the sum of weights over all configurations. This is computed by the solver, not by `evaluate`.

### Trait hierarchy (updated)

```
Problem (Metric: Clone)
├── OptimizationProblem  (Metric = SolutionSize<V>)   — existing, unchanged
├── SatisfactionProblem  (Metric = bool)               — existing, unchanged
└── CountingProblem      (Metric = Weight<V>)          — NEW
```

### Solver extension

Add a separate `CountingSolver` trait (parallel to how problem families have distinct traits) rather than extending the existing `Solver` trait. This avoids forcing `ILPSolver` to implement a meaningless `count` method:

```rust
// src/solvers/mod.rs (existing Solver trait unchanged)

/// Solver trait for counting problems.
pub trait CountingSolver {
    /// Compute the total weight (sum of evaluate over all configs).
    fn count<P: CountingProblem>(&self, problem: &P) -> P::Value;
}

// src/solvers/brute_force.rs
impl CountingSolver for BruteForce {
    fn count<P: CountingProblem>(&self, problem: &P) -> P::Value {
        let mut total = P::Value::zero();
        for config in DimsIterator::new(problem.dims()) {
            total += problem.evaluate(&config).0;
        }
        total
    }
}
```

`BruteForce` also gets a convenience method for testing:
```rust
/// Return all feasible configs and their weights alongside the total count.
pub fn count_with_configs<P: CountingProblem>(&self, problem: &P)
    -> (P::Value, Vec<(Vec<usize>, P::Value)>);
```

### Reduction support

Counting reductions preserve aggregate counts, not individual solutions. New traits parallel to `ReductionResult` / `ReduceTo<T>`:

```rust
// src/rules/traits.rs

pub trait CountingReductionResult {
    type Source: CountingProblem;
    type Target: CountingProblem;

    /// Get a reference to the target problem.
    fn target_problem(&self) -> &Self::Target;

    /// Transform the target's aggregate count back to the source's count.
    ///
    /// For parsimonious reductions (1-to-1 config mapping), this is identity.
    /// For non-parsimonious reductions, this applies a correction factor
    /// (e.g., divide by 2 if the reduction doubles feasible configs).
    fn extract_count(
        &self,
        target_count: <Self::Target as CountingProblem>::Value,
    ) -> <Self::Source as CountingProblem>::Value;
}

pub trait ReduceToCount<T: CountingProblem>: CountingProblem {
    type Result: CountingReductionResult<Source = Self, Target = T>;
    fn reduce_to_count(&self) -> Self::Result;
}
```

### Registry and CLI integration

#### `declare_variants!` macro

Gets a new `count` keyword. The macro generates a `CountSolveFn` (instead of `SolveFn`) that calls `BruteForce::count()` and formats the result:

```rust
crate::declare_variants! {
    default count NetworkReliability => "2^num_edges * num_vertices",
}
```

The `count` keyword generates:
- A new `SolverKind::Count` variant in the proc macro's internal `SolverKind` enum (alongside existing `Opt` and `Sat`)
- A `solver_kind` field on `VariantEntry` to distinguish problem families at runtime (enum with `Optimization`, `Satisfaction`, `Counting` variants)
- A `count_fn: Option<CountSolveFn>` field on `VariantEntry` where `CountSolveFn = fn(&dyn Any) -> String`
- The generated function downcasts `&dyn Any` to the concrete type, calls `BruteForce.count(&problem)`, and formats the result
- The existing `ProblemType` struct (which holds problem metadata, not a classification enum) is unchanged

#### `LoadedDynProblem`

Gets a new method:
```rust
pub fn solve_counting(&self) -> Option<String> {
    (self.count_fn?)(self.inner.as_any())
}
```

The existing `solve_brute_force` remains unchanged for opt/sat problems.

#### `pred solve` CLI

The solve command checks `VariantEntry::solver_kind` to determine the dispatch path:
- `SolverKind::Optimization` / `SolverKind::Satisfaction` → existing `solve_brute_force()`
- `SolverKind::Counting` → new `solve_counting()`, displays `Total weight: <value>`

#### `#[reduction]` proc macro

The existing macro hardcodes `ReduceTo` trait detection. It must be extended to also recognize `ReduceToCount`:

- When the macro sees `impl ReduceToCount<Target> for Source`, it generates a `reduce_count_fn` field on `ReductionEntry`
- The generated function returns a `Box<dyn DynCountingReductionResult>` (new type-erased trait for counting reductions)
- `overhead` attribute works identically — overhead expressions are about problem size, not about solution type

#### `ReductionEntry` changes

`ReductionEntry` (in `src/rules/registry.rs`) gains new optional fields for counting reductions. A given entry has either `reduce_fn` (opt/sat) or `reduce_count_fn` (counting), never both:

```rust
pub struct ReductionEntry {
    // ... existing fields unchanged ...
    pub reduce_fn: Option<ReduceFn>,              // existing: opt/sat reductions
    pub reduce_count_fn: Option<CountReduceFn>,    // NEW: counting reductions
}
```

Where `CountReduceFn = fn(&dyn Any) -> Box<dyn DynCountingReductionResult>`.

#### Reduction graph integration

Counting edges and opt/sat edges coexist in the same `ReductionGraph`. The graph is about problem reachability — edge type doesn't affect pathfinding. The distinction matters only at solve time:

- `ReductionEdgeData` gains an `edge_kind: EdgeKind` field (`enum EdgeKind { Standard, Counting }`)
- `reduce_along_path` checks edge kinds: a path must be homogeneous (all-standard or all-counting); mixed paths are invalid
- For all-counting paths, the runtime builds a `CountingReductionChain` instead of a `ReductionChain`

#### Counting reduction chains

For multi-hop counting paths (A →count→ B →count→ C):

```rust
pub trait DynCountingReductionResult {
    fn target_problem_any(&self) -> &dyn Any;
    /// Transform target count to source count using serde_json::Value for type erasure.
    fn extract_count_dyn(&self, target_count: serde_json::Value) -> serde_json::Value;
}
```

`CountingReductionChain` composes these: reduce A→B→C, solve C to get count as `serde_json::Value`, then call `extract_count_dyn` backwards through the chain. This parallels `ReductionChain` for opt/sat reductions.

**Note on cross-type reductions:** When source and target have different `Value` types (e.g., `u64` → `f64`), the `extract_count` implementation is responsible for the type conversion. The `serde_json::Value` type erasure in `DynCountingReductionResult` handles this naturally at the runtime dispatch level.

#### Exports

Add to prelude and `lib.rs`:
- `CountingProblem`, `Weight`, `CountingSolver` traits/types
- `ReduceToCount`, `CountingReductionResult` traits

### Concrete models

All models store **only the counting problem data** — no decision thresholds (`k`, `q`). The threshold is part of the GJ decision formulation but not part of the counting problem we model.

| Model | Value type | Fields | evaluate returns |
|---|---|---|---|
| `NetworkReliability` | `f64` | `graph`, `terminals`, `failure_probs` | `Weight(Π p_e^{x_e} · (1-p_e)^{1-x_e})` if terminals connected, else `Weight(0.0)` |
| `NetworkSurvivability` | `f64` | `graph`, `terminals`, `failure_probs` | Same pattern for survivability |
| `KthLargestSubset` | `u64` | `sizes`, `bound` | `Weight(1)` if subset sum ≤ bound, else `Weight(0)` |
| `KthLargestMTuple` | `u64` | `sizes`, `bound` | `Weight(1)` if m-tuple condition met, else `Weight(0)` |

### `Weight<W>` utility impls

For ergonomics, `Weight<W>` implements:
- `PartialOrd` where `W: PartialOrd` — delegates to inner value
- `Eq` where `W: Eq`, `Hash` where `W: Hash` — conditional impls (works for `u64`, not `f64`)
- `Add<Output = Weight<W>>` and `std::iter::Sum` where `W: Add` — enables `configs.map(evaluate).sum()`
- `Display` where `W: Display` — prints the inner value directly (e.g., `0.9832` not `Weight(0.9832)`)

### What is NOT changed

- `OptimizationProblem`, `SatisfactionProblem` — untouched
- `ReduceTo<T>`, `ReductionResult` — untouched
- All existing models and rules — untouched
- Existing `Solver` trait — untouched (new `CountingSolver` is separate)

## Alternatives considered

1. **Generalized metric aggregation** (#737) — replace all three leaf traits with a single `Aggregation` enum. Elegant but large breaking refactor with no immediate payoff. Filed for future consideration.

2. **Two-level trait** (`is_feasible` + `weight` methods) — more explicit but adds unnecessary surface area and boilerplate for unweighted counting.

3. **`Metric = f64` without wrapper** — works but loses type safety. `Weight<W>` follows the `SolutionSize<V>` pattern and makes intent explicit.

## Related issues

- #737 — Generalized metric aggregation (future architecture)
- #748 — DefaultSolver per problem (future architecture)
