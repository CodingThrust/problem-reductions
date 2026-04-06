# Decision Wrapper Design Spec

**Issue:** #998
**Date:** 2026-04-06
**Status:** Draft (revised after Codex review)

## Motivation

Many classical NP-completeness reductions (Garey & Johnson) operate between decision
versions of problems, but the codebase models are optimization problems. This blocks
reductions where the source is `Min<V>` or `Max<V>` but the target expects `Or`.

Blocked rules (case A — optimization-to-decision only):
- #379: MinimumDominatingSet → MinMaxMulticenter
- #198: MinimumVertexCover → HamiltonianCircuit
- #894: MinimumVertexCover → PartialFeedbackEdgeSet

Out of scope: case (B) cross-sense (`Max → Min`) and case (C) hidden-parameter
reductions from issue #998.

## Design Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Scope | Case (A) only | Cases B and C are independent problems |
| Location | `src/models/decision.rs` (generic struct); concrete variants in optimization model files | Keeps decision + optimization together |
| Comparison sense | Inferred from `P::Value` via `OptimizationValue` trait | No invalid states; `Min` always means ≤, `Max` always means ≥ |
| NAME resolution | `DecisionProblemMeta` trait + `decision_problem_meta!` macro | `const_format::concatcp!` doesn't support generic associated consts; this avoids the dependency while keeping shared generic logic |
| Registry | Explicit `declare_variants!` per concrete type, in the optimization model file | Consistent with existing patterns |
| Naming | `"Decision"` prefix (e.g., `"DecisionMinimumVertexCover"`) | Clear, no collision |
| Hand-written decision models | Replace with `Decision<P>` | Eliminates duplication; `VertexCover` → `Decision<MinimumVertexCover>` |
| Alias migration | Register `VertexCover`/`VC` as aliases on `DecisionMinimumVertexCover` schema entry | Preserves CLI/catalog backward compatibility |
| Opt→Decision reduction | `Decision<P> → P` as `ReduceToAggregate` only | Solve inner optimization, compare to bound — fits one-shot aggregate model |
| Decision→Opt solver | Golden-section search utility in `src/solvers/` | Multi-query algorithm, not a `ReduceTo` edge; included for testing |
| Initial concrete types | `Decision<MinimumVertexCover<SimpleGraph, i32>>`, `Decision<MinimumDominatingSet<SimpleGraph, i32>>` | Minimum to unblock #379, #198, #894 |

## Architecture

### 1. `OptimizationValue` Trait (`src/types.rs`)

```rust
/// Trait for aggregate values that represent optimization objectives (Min or Max).
/// Enables generic conversion to decision problems via a bound parameter.
pub trait OptimizationValue: Aggregate {
    /// The inner numeric type (e.g., `i32` for `Min<i32>`).
    type Inner: Clone + PartialOrd + fmt::Debug + Serialize + DeserializeOwned;

    /// Does this evaluation result satisfy the decision bound?
    /// - For `Min<V>`: true iff value is Some(v) where v ≤ bound
    /// - For `Max<V>`: true iff value is Some(v) where v ≥ bound
    fn meets_bound(value: &Self, bound: &Self::Inner) -> bool;
}
```

Implementations for `Min<V>` (checks `≤`) and `Max<V>` (checks `≥`).
`Min(None)` and `Max(None)` (infeasible configs) always return `false`.

### 2. `DecisionProblemMeta` Trait + `Decision<P>` Struct (`src/models/decision.rs`)

`const_format::concatcp!` does not support generic associated consts (`P::NAME`).
Instead, use a metadata trait that each concrete inner problem implements:

```rust
/// Metadata trait providing the decision problem name for each inner problem type.
pub trait DecisionProblemMeta: Problem
where
    Self::Value: OptimizationValue,
{
    const DECISION_NAME: &'static str;
}

/// Helper macro to register a concrete inner problem's decision name.
#[macro_export]
macro_rules! decision_problem_meta {
    ($inner:ty, $name:literal) => {
        impl crate::models::decision::DecisionProblemMeta for $inner {
            const DECISION_NAME: &'static str = $name;
        }
    };
}
```

The generic struct and `Problem` impl:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decision<P: Problem>
where
    P::Value: OptimizationValue,
{
    inner: P,
    bound: <P::Value as OptimizationValue>::Inner,
}

impl<P> Problem for Decision<P>
where
    P: DecisionProblemMeta,
    P::Value: OptimizationValue,
{
    const NAME: &'static str = P::DECISION_NAME;
    type Value = Or;

    fn dims(&self) -> Vec<usize> { self.inner.dims() }

    fn evaluate(&self, config: &[usize]) -> Or {
        Or(<P::Value as OptimizationValue>::meets_bound(
            &self.inner.evaluate(config),
            &self.bound,
        ))
    }

    fn variant() -> Vec<(&'static str, &'static str)> { P::variant() }
}
```

Accessor methods: `new()`, `inner()`, `bound()`.

### 3. `Decision<P> → P` Aggregate Reduction (`src/models/decision.rs`)

Only the `Decision<P> → P` direction is a valid reduction (solve inner optimization,
compare to bound). The reverse (bisection) is not a one-shot reduction — it requires
multiple adaptive queries and belongs as a separate solver utility (out of scope).

```rust
#[derive(Debug, Clone)]
pub struct DecisionToOptimizationResult<P>
where
    P: Problem,
    P::Value: OptimizationValue,
{
    target: P,
    bound: <P::Value as OptimizationValue>::Inner,
}

impl<P> AggregateReductionResult for DecisionToOptimizationResult<P>
where
    P: Problem + 'static,
    P::Value: OptimizationValue + Serialize + DeserializeOwned,
{
    type Source = Decision<P>;
    type Target = P;

    fn target_problem(&self) -> &P { &self.target }

    fn extract_value(&self, target_value: P::Value) -> Or {
        Or(<P::Value as OptimizationValue>::meets_bound(&target_value, &self.bound))
    }
}

impl<P> ReduceToAggregate<P> for Decision<P>
where
    P: DecisionProblemMeta + Clone + 'static,
    P::Value: OptimizationValue + Serialize + DeserializeOwned,
{
    type Result = DecisionToOptimizationResult<P>;

    fn reduce_to_aggregate(&self) -> Self::Result {
        DecisionToOptimizationResult {
            target: self.inner().clone(),
            bound: self.bound().clone(),
        }
    }
}
```

**Note:** `#[reduction]` currently only populates `reduce_fn` (witness), not
`reduce_aggregate_fn`. Concrete aggregate edges must be registered manually via
`inventory::submit!(ReductionEntry { ... })` in the optimization model files.

### 4. Golden-Section Search Solver (`src/solvers/golden_section.rs`)

Finds the optimal value of an optimization problem `P` by querying its decision
version `Decision<P>`. Uses the [golden-section search](https://en.wikipedia.org/wiki/Golden-section_search)
algorithm — not a reduction (it requires multiple adaptive queries), but a solver
utility that exercises the `Decision<P>` wrapper end-to-end.

```rust
/// Solve an optimization problem by golden-section search on its decision version.
///
/// Given an optimization problem P with Value = Min<V> or Max<V>, constructs
/// Decision<P> instances with varying bounds and narrows the search interval
/// using the golden ratio φ = (1 + √5) / 2.
///
/// For Min<V>: searches for the smallest bound where Decision<P> is satisfiable.
/// For Max<V>: searches for the largest bound where Decision<P> is satisfiable.
pub fn solve_via_decision<P>(problem: &P, lower: V, upper: V) -> Option<V>
where
    P: DecisionProblemMeta + Clone,
    P::Value: OptimizationValue<Inner = V>,
    V: /* numeric bounds */,
{
    // Golden ratio narrowing:
    // 1. Evaluate Decision<P> at two interior probe points
    // 2. Narrow interval based on which probe is feasible/infeasible
    // 3. Repeat until convergence (for f64) or interval width ≤ 1 (for integers)
}
```

**Integer specialization:** For `V = i32` (discrete), golden-section search degrades
to a narrowing strategy that converges in O(log n) decision queries, where n is the
value range. Each query constructs a `Decision<P>` and solves it with `BruteForce`.

**Purpose:** Primarily a testing utility — validates that `Decision<P>` correctly
wraps the optimization problem by recovering the same optimum that `BruteForce`
finds directly. Also demonstrates the decision↔optimization duality.

### 5. Proc Macro: `extract_type_name()` Fix (`problemreductions-macros/src/lib.rs`)


The current `extract_type_name()` takes only the last path segment identifier.
For `Decision<MinimumVertexCover<SimpleGraph, i32>>`, it produces `"Decision"` —
losing the inner type. All `Decision<...>` variants would collapse to one name.

**Fix:** Special-case `"Decision"` and recurse into the first type argument:

```rust
fn extract_type_name(ty: &syn::Type) -> Option<String> {
    match ty {
        syn::Type::Path(type_path) => {
            let segment = type_path.path.segments.last()?;
            let ident = segment.ident.to_string();

            if ident == "Decision" {
                let syn::PathArguments::AngleBracketed(args) = &segment.arguments else {
                    return Some(ident);
                };
                let inner_ty = args.args.iter().find_map(|arg| match arg {
                    syn::GenericArgument::Type(ty) => Some(ty),
                    _ => None,
                })?;
                let inner_name = extract_type_name(inner_ty)?;
                return Some(format!("Decision{inner_name}"));
            }

            Some(ident)
        }
        _ => None,
    }
}
```

After this change:
- `MinimumVertexCover<SimpleGraph, i32>` → `"MinimumVertexCover"` (unchanged)
- `Decision<MinimumVertexCover<SimpleGraph, i32>>` → `"DecisionMinimumVertexCover"`

This affects both `#[reduction]` name extraction and `declare_variants!` default grouping.

### 5. Concrete Variants (in optimization model files)

Each optimization model that needs a decision version adds to its own file:

```rust
// In src/models/graph/minimum_vertex_cover.rs

// --- Decision name registration ---
crate::decision_problem_meta!(
    MinimumVertexCover<SimpleGraph, i32>,
    "DecisionMinimumVertexCover"
);

// --- Delegated getters for overhead expressions ---
impl<G: Graph, W: WeightElement> Decision<MinimumVertexCover<G, W>> {
    pub fn num_vertices(&self) -> usize { self.inner().num_vertices() }
    pub fn num_edges(&self) -> usize { self.inner().num_edges() }
    pub fn k(&self) -> usize { self.bound() as usize }
}

// --- Variant registration ---
declare_variants! {
    default Decision<MinimumVertexCover<SimpleGraph, i32>> => "1.1996^num_vertices",
}

// --- Manual aggregate reduction registration ---
// inventory::submit!(ReductionEntry { ... }) for Decision<MVC> → MVC aggregate edge
```

Same pattern for `minimum_dominating_set.rs`.

### 6. Migration: Remove Hand-Written Decision Models

**Remove:**
- `src/models/graph/vertex_cover.rs` — replaced by `Decision<MinimumVertexCover>`
- `src/unit_tests/models/graph/vertex_cover.rs`

**Alias migration:** Register `VertexCover` and `VC` as aliases on the new
`DecisionMinimumVertexCover` `ProblemSchemaEntry`, so `pred show VertexCover`
and `pred show VC` continue to work.

**No existing reductions reference `VertexCover`** — all reduction files use
`MinimumVertexCover` (the optimization version). Zero reduction file changes needed.

**Additional files to update:**
- `src/models/graph/mod.rs` — remove `VertexCover` export
- `src/example_db/model_builders.rs` — update or remove VertexCover canonical example
- `src/example_db/specs.rs` — migrate VertexCover spec to DecisionMinimumVertexCover
- `docs/paper/reductions.typ` — update `problem-def("VertexCover")` entry and `display-name` dict

### 7. Testing

**Generic tests** in `src/unit_tests/models/decision.rs`:
1. `test_decision_min_creation` — construct, verify accessors
2. `test_decision_min_evaluate_feasible` — cost ≤ bound → `Or(true)`
3. `test_decision_min_evaluate_infeasible_cost` — cost > bound → `Or(false)`
4. `test_decision_min_evaluate_infeasible_config` — invalid config → `Or(false)`
5. `test_decision_max_evaluate` — ≥ bound semantics with a Max-valued problem
6. `test_decision_solver` — BruteForce witness recovery
7. `test_decision_serialization` — round-trip serde
8. `test_decision_dims` — delegates to inner

**Golden-section search tests** in `src/unit_tests/solvers/golden_section.rs`:
9. `test_golden_section_min` — recover MinimumVertexCover optimum via Decision queries
10. `test_golden_section_max` — recover MaximumIndependentSet optimum via Decision queries
11. `test_golden_section_matches_brute_force` — verify golden-section result equals BruteForce result

**Per-problem tests** in existing model test files gain decision variant coverage.

## File Changes Summary

| File | Change |
|------|--------|
| `src/types.rs` | Add `OptimizationValue` trait + `Min`/`Max` impls |
| `src/models/decision.rs` | **New** — `DecisionProblemMeta` trait, `decision_problem_meta!` macro, `Decision<P>` struct, generic `Problem` impl, `ReduceToAggregate` impl |
| `src/solvers/golden_section.rs` | **New** — golden-section search solver: finds optimum by querying `Decision<P>` |
| `src/models/mod.rs` | Add `pub mod decision` |
| `src/models/graph/minimum_vertex_cover.rs` | Add `decision_problem_meta!`, decision getters, `declare_variants!`, `ProblemSchemaEntry` (with VC/VertexCover aliases), manual aggregate `ReductionEntry` |
| `src/models/graph/minimum_dominating_set.rs` | Same pattern |
| `src/models/graph/vertex_cover.rs` | **Remove** |
| `src/models/graph/mod.rs` | Remove `VertexCover` export |
| `problemreductions-macros/src/lib.rs` | Fix `extract_type_name()` to recurse into `Decision<T>` |
| `src/example_db/model_builders.rs` | Migrate VertexCover example → DecisionMinimumVertexCover |
| `src/example_db/specs.rs` | Migrate VertexCover spec |
| `docs/paper/reductions.typ` | Update VertexCover problem-def and display-name |
| `src/unit_tests/models/decision.rs` | **New** — generic decision tests |
| `src/unit_tests/models/graph/vertex_cover.rs` | **Remove** |

## Not In Scope

- Implementing blocked reductions (#379, #198, #894) — separate PRs
- Cases (B) cross-sense and (C) hidden-parameter from issue #998
- CLI `pred create` support for Decision types
- `const_format` dependency (replaced by `DecisionProblemMeta` trait)
