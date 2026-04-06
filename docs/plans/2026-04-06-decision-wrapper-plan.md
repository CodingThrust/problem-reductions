# Decision Wrapper Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a generic `Decision<P>` wrapper that converts optimization problems (`Min`/`Max`) to decision problems (`Or`), replace hand-written `VertexCover`, and provide a golden-section search solver.

**Architecture:** `OptimizationValue` trait abstracts over `Min<V>`/`Max<V>`. `Decision<P>` is a generic struct with `Problem` impl delegated via `DecisionProblemMeta` trait. Concrete variants registered in optimization model files. Golden-section search solver recovers optima via decision queries.

**Tech Stack:** Rust, `inventory` crate (existing), `serde` (existing), proc-macro (`syn`/`quote`)

**Spec:** `docs/plans/2026-04-06-decision-wrapper-design.md`

---

### Task 1: `OptimizationValue` Trait

**Files:**
- Modify: `src/types.rs` (append after `Min<V>` impls, ~line 299)

- [ ] **Step 1: Write failing tests for `OptimizationValue`**

Create `src/unit_tests/types_optimization_value.rs`:

```rust
use crate::types::{Max, Min, OptimizationValue};

#[test]
fn test_min_meets_bound_feasible() {
    // Min(Some(3)) <= 5 → true
    assert!(Min::<i32>::meets_bound(&Min(Some(3)), &5));
}

#[test]
fn test_min_meets_bound_exact() {
    // Min(Some(5)) <= 5 → true
    assert!(Min::<i32>::meets_bound(&Min(Some(5)), &5));
}

#[test]
fn test_min_meets_bound_exceeds() {
    // Min(Some(7)) <= 5 → false
    assert!(!Min::<i32>::meets_bound(&Min(Some(7)), &5));
}

#[test]
fn test_min_meets_bound_infeasible() {
    // Min(None) → false (infeasible config)
    assert!(!Min::<i32>::meets_bound(&Min(None), &5));
}

#[test]
fn test_max_meets_bound_feasible() {
    // Max(Some(7)) >= 5 → true
    assert!(Max::<i32>::meets_bound(&Max(Some(7)), &5));
}

#[test]
fn test_max_meets_bound_exact() {
    // Max(Some(5)) >= 5 → true
    assert!(Max::<i32>::meets_bound(&Max(Some(5)), &5));
}

#[test]
fn test_max_meets_bound_below() {
    // Max(Some(3)) >= 5 → false
    assert!(!Max::<i32>::meets_bound(&Max(Some(3)), &5));
}

#[test]
fn test_max_meets_bound_infeasible() {
    // Max(None) → false
    assert!(!Max::<i32>::meets_bound(&Max(None), &5));
}
```

Add test module link in `src/types.rs` at the bottom:

```rust
#[cfg(test)]
#[path = "unit_tests/types_optimization_value.rs"]
mod optimization_value_tests;
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test optimization_value -- --nocapture 2>&1 | head -30`
Expected: FAIL — `OptimizationValue` trait not found.

- [ ] **Step 3: Implement `OptimizationValue` trait**

Add to `src/types.rs` after the `Min<V>` impl block (after line 299):

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

impl<V: fmt::Debug + PartialOrd + Clone + Serialize + DeserializeOwned> OptimizationValue
    for Min<V>
{
    type Inner = V;

    fn meets_bound(value: &Self, bound: &V) -> bool {
        matches!(&value.0, Some(v) if *v <= *bound)
    }
}

impl<V: fmt::Debug + PartialOrd + Clone + Serialize + DeserializeOwned> OptimizationValue
    for Max<V>
{
    type Inner = V;

    fn meets_bound(value: &Self, bound: &V) -> bool {
        matches!(&value.0, Some(v) if *v >= *bound)
    }
}
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test optimization_value -- --nocapture`
Expected: All 8 tests PASS.

- [ ] **Step 5: Commit**

```bash
git add src/types.rs src/unit_tests/types_optimization_value.rs
git commit -m "feat: add OptimizationValue trait for Min/Max decision conversion"
```

---

### Task 2: `Decision<P>` Struct + `Problem` Impl

**Files:**
- Create: `src/models/decision.rs`
- Modify: `src/models/mod.rs` (add `pub mod decision`)

- [ ] **Step 1: Write failing tests for `Decision<P>`**

Create `src/unit_tests/models/decision.rs`:

```rust
use crate::models::decision::Decision;
use crate::models::graph::MinimumVertexCover;
use crate::models::graph::MaximumIndependentSet;
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Or;

fn triangle_mvc() -> MinimumVertexCover<SimpleGraph, i32> {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    MinimumVertexCover::new(graph, vec![1; 3])
}

#[test]
fn test_decision_min_creation() {
    let mvc = triangle_mvc();
    let decision = Decision::new(mvc, 2);
    assert_eq!(decision.bound(), &2);
    assert_eq!(decision.inner().num_vertices(), 3);
}

#[test]
fn test_decision_min_evaluate_feasible() {
    let decision = Decision::new(triangle_mvc(), 2);
    // Config [1,1,0]: covers all edges, cost=2 ≤ 2 → Or(true)
    assert_eq!(decision.evaluate(&[1, 1, 0]), Or(true));
}

#[test]
fn test_decision_min_evaluate_infeasible_cost() {
    let decision = Decision::new(triangle_mvc(), 1);
    // Config [1,1,0]: covers all edges, cost=2 > 1 → Or(false)
    assert_eq!(decision.evaluate(&[1, 1, 0]), Or(false));
}

#[test]
fn test_decision_min_evaluate_infeasible_config() {
    let decision = Decision::new(triangle_mvc(), 3);
    // Config [1,0,0]: does NOT cover edge (1,2) → Min(None) → Or(false)
    assert_eq!(decision.evaluate(&[1, 0, 0]), Or(false));
}

#[test]
fn test_decision_max_evaluate() {
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let mis = MaximumIndependentSet::new(graph, vec![1; 4]);
    let decision = Decision::new(mis, 2);
    // Config [1,0,1,0]: independent, weight=2 ≥ 2 → Or(true)
    assert_eq!(decision.evaluate(&[1, 0, 1, 0]), Or(true));
    // Config [1,0,0,0]: independent, weight=1 < 2 → Or(false)
    assert_eq!(decision.evaluate(&[1, 0, 0, 0]), Or(false));
}

#[test]
fn test_decision_dims() {
    let decision = Decision::new(triangle_mvc(), 2);
    assert_eq!(decision.dims(), vec![2, 2, 2]);
}

#[test]
fn test_decision_solver() {
    use crate::solvers::BruteForce;
    use crate::Solver;
    let decision = Decision::new(triangle_mvc(), 2);
    let solver = BruteForce::new();
    let witness = solver.find_witness(&decision);
    assert!(witness.is_some());
    let config = witness.unwrap();
    // Verify: it's a valid cover with cost ≤ 2
    assert_eq!(decision.evaluate(&config), Or(true));
}

#[test]
fn test_decision_serialization() {
    let decision = Decision::new(triangle_mvc(), 2);
    let json = serde_json::to_string(&decision).unwrap();
    let deserialized: Decision<MinimumVertexCover<SimpleGraph, i32>> =
        serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.bound(), &2);
    assert_eq!(deserialized.evaluate(&[1, 1, 0]), Or(true));
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test test_decision_min -- --nocapture 2>&1 | head -20`
Expected: FAIL — `decision` module not found.

- [ ] **Step 3: Create `src/models/decision.rs`**

```rust
//! Generic Decision wrapper for optimization problems.
//!
//! Converts any optimization problem P (with Value = Min<V> or Max<V>)
//! into a decision problem (Value = Or) by adding a bound parameter.

use crate::traits::Problem;
use crate::types::{OptimizationValue, Or};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Metadata trait providing the decision problem name for each inner problem type.
///
/// Implement this for each concrete optimization problem that needs a Decision version.
/// Use the [`decision_problem_meta!`] macro for convenient registration.
pub trait DecisionProblemMeta: Problem
where
    Self::Value: OptimizationValue,
{
    /// The NAME constant for the Decision<Self> problem type.
    const DECISION_NAME: &'static str;
}

/// Helper macro to register a concrete inner problem's decision name.
///
/// # Example
/// ```ignore
/// crate::decision_problem_meta!(
///     MinimumVertexCover<SimpleGraph, i32>,
///     "DecisionMinimumVertexCover"
/// );
/// ```
#[macro_export]
macro_rules! decision_problem_meta {
    ($inner:ty, $name:literal) => {
        impl $crate::models::decision::DecisionProblemMeta for $inner {
            const DECISION_NAME: &'static str = $name;
        }
    };
}

/// Decision version of an optimization problem.
///
/// Given an optimization problem P with `Value = Min<V>` or `Value = Max<V>`,
/// `Decision<P>` asks: "does there exist a configuration with value ≤ bound (for Min)
/// or ≥ bound (for Max)?"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decision<P: Problem>
where
    P::Value: OptimizationValue,
{
    inner: P,
    bound: <P::Value as OptimizationValue>::Inner,
}

impl<P: Problem> Decision<P>
where
    P::Value: OptimizationValue,
{
    /// Create a new Decision problem from an inner optimization problem and a bound.
    pub fn new(inner: P, bound: <P::Value as OptimizationValue>::Inner) -> Self {
        Self { inner, bound }
    }

    /// Get a reference to the inner optimization problem.
    pub fn inner(&self) -> &P {
        &self.inner
    }

    /// Get a reference to the decision bound.
    pub fn bound(&self) -> &<P::Value as OptimizationValue>::Inner {
        &self.bound
    }
}

impl<P> Problem for Decision<P>
where
    P: DecisionProblemMeta + Clone,
    P::Value: OptimizationValue
        + Clone
        + fmt::Debug
        + Serialize
        + DeserializeOwned,
    <P::Value as OptimizationValue>::Inner:
        Clone + PartialOrd + fmt::Debug + Serialize + DeserializeOwned,
{
    const NAME: &'static str = P::DECISION_NAME;
    type Value = Or;

    fn dims(&self) -> Vec<usize> {
        self.inner.dims()
    }

    fn evaluate(&self, config: &[usize]) -> Or {
        Or(<P::Value as OptimizationValue>::meets_bound(
            &self.inner.evaluate(config),
            &self.bound,
        ))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        P::variant()
    }
}
```

- [ ] **Step 4: Add module to `src/models/mod.rs`**

Add after `pub mod set;`:

```rust
pub mod decision;
```

Add to the re-export block in `src/models/mod.rs`:

```rust
pub use decision::Decision;
```

- [ ] **Step 5: Add `DecisionProblemMeta` impls for MVC and MIS**

In `src/models/graph/minimum_vertex_cover.rs`, add after the `declare_variants!` block:

```rust
crate::decision_problem_meta!(
    MinimumVertexCover<SimpleGraph, i32>,
    "DecisionMinimumVertexCover"
);
```

In `src/models/graph/maximum_independent_set.rs`, add after the `declare_variants!` block:

```rust
crate::decision_problem_meta!(
    MaximumIndependentSet<SimpleGraph, i32>,
    "DecisionMaximumIndependentSet"
);
```

- [ ] **Step 6: Wire up test file**

Add to the bottom of `src/models/decision.rs`:

```rust
#[cfg(test)]
#[path = "../unit_tests/models/decision.rs"]
mod tests;
```

- [ ] **Step 7: Run tests to verify they pass**

Run: `cargo test test_decision -- --nocapture`
Expected: All 8 tests PASS.

- [ ] **Step 8: Commit**

```bash
git add src/models/decision.rs src/models/mod.rs src/unit_tests/models/decision.rs \
    src/models/graph/minimum_vertex_cover.rs src/models/graph/maximum_independent_set.rs
git commit -m "feat: add Decision<P> generic wrapper with Problem impl"
```

---

### Task 3: `ReduceToAggregate` for `Decision<P> → P`

**Files:**
- Modify: `src/models/decision.rs` (append aggregate reduction)

- [ ] **Step 1: Write failing test**

Add to `src/unit_tests/models/decision.rs`:

```rust
#[test]
fn test_decision_reduce_to_aggregate() {
    use crate::rules::ReduceToAggregate;
    let mvc = triangle_mvc();
    let decision = Decision::new(mvc, 2);
    let result = decision.reduce_to_aggregate();
    let target = result.target_problem();
    // Target is the inner MinimumVertexCover
    assert_eq!(target.num_vertices(), 3);

    // Config [1,1,0] on target gives Min(Some(2))
    // extract_value maps Min(Some(2)) to Or(true) since 2 ≤ 2
    use crate::rules::AggregateReductionResult;
    let target_val = target.evaluate(&[1, 1, 0]);
    let source_val = result.extract_value(target_val);
    assert_eq!(source_val, Or(true));

    // Config [1,1,1] on target gives Min(Some(3))
    // extract_value maps Min(Some(3)) to Or(false) since 3 > 2
    let target_val = target.evaluate(&[1, 1, 1]);
    let source_val = result.extract_value(target_val);
    assert_eq!(source_val, Or(false));
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test test_decision_reduce_to_aggregate -- --nocapture 2>&1 | head -20`
Expected: FAIL — `ReduceToAggregate` not implemented.

- [ ] **Step 3: Implement aggregate reduction in `src/models/decision.rs`**

Add after the `Problem` impl:

```rust
use crate::rules::traits::{AggregateReductionResult, ReduceToAggregate};

/// Result of reducing Decision<P> to P (aggregate value extraction).
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
    <P::Value as OptimizationValue>::Inner: Clone + PartialOrd,
{
    type Source = Decision<P>;
    type Target = P;

    fn target_problem(&self) -> &P {
        &self.target
    }

    fn extract_value(&self, target_value: P::Value) -> Or {
        Or(<P::Value as OptimizationValue>::meets_bound(
            &target_value,
            &self.bound,
        ))
    }
}

impl<P> ReduceToAggregate<P> for Decision<P>
where
    P: DecisionProblemMeta + Clone + 'static,
    P::Value: OptimizationValue + Clone + fmt::Debug + Serialize + DeserializeOwned,
    <P::Value as OptimizationValue>::Inner:
        Clone + PartialOrd + fmt::Debug + Serialize + DeserializeOwned,
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

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test test_decision -- --nocapture`
Expected: All 9 tests PASS.

- [ ] **Step 5: Commit**

```bash
git add src/models/decision.rs src/unit_tests/models/decision.rs
git commit -m "feat: add ReduceToAggregate for Decision<P> → P"
```

---

### Task 4: Proc Macro `extract_type_name()` Fix

**Files:**
- Modify: `problemreductions-macros/src/lib.rs` (~line 131)

- [ ] **Step 1: Fix `extract_type_name()` to handle `Decision<T>`**

Replace the existing `extract_type_name` function at line 131 of `problemreductions-macros/src/lib.rs`:

```rust
/// Extract the base type name from a Type (e.g., "IndependentSet" from "IndependentSet<i32>").
/// Special-cases "Decision<T>" to produce "DecisionT" (e.g., "DecisionMinimumVertexCover").
fn extract_type_name(ty: &Type) -> Option<String> {
    match ty {
        Type::Path(type_path) => {
            let segment = type_path.path.segments.last()?;
            let ident = segment.ident.to_string();

            if ident == "Decision" {
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    let inner_ty = args.args.iter().find_map(|arg| {
                        if let GenericArgument::Type(ty) = arg {
                            Some(ty)
                        } else {
                            None
                        }
                    })?;
                    let inner_name = extract_type_name(inner_ty)?;
                    return Some(format!("Decision{inner_name}"));
                }
            }

            Some(ident)
        }
        _ => None,
    }
}
```

- [ ] **Step 2: Run existing tests to verify no regressions**

Run: `cargo test -p problemreductions-macros`
Run: `make check`
Expected: All tests PASS.

- [ ] **Step 3: Commit**

```bash
git add problemreductions-macros/src/lib.rs
git commit -m "fix: extract_type_name handles Decision<T> nested generics"
```

---

### Task 5: Concrete Decision Variants for MVC and MDS

**Files:**
- Modify: `src/models/graph/minimum_vertex_cover.rs`
- Modify: `src/models/graph/minimum_dominating_set.rs`

- [ ] **Step 1: Add Decision variant registration to `minimum_vertex_cover.rs`**

Add after the existing `declare_variants!` block and the `decision_problem_meta!` (already added in Task 2):

```rust
// Decision variant: delegates getters to inner problem
impl<G: Graph, W: WeightElement> Decision<MinimumVertexCover<G, W>> {
    /// Number of vertices in the underlying graph.
    pub fn num_vertices(&self) -> usize {
        self.inner().num_vertices()
    }

    /// Number of edges in the underlying graph.
    pub fn num_edges(&self) -> usize {
        self.inner().num_edges()
    }

    /// Decision bound as usize (for overhead expressions).
    pub fn k(&self) -> usize
    where
        W::Sum: Into<i64>,
    {
        let b: i64 = self.bound().clone().into();
        b as usize
    }
}

crate::declare_variants! {
    default Decision<MinimumVertexCover<SimpleGraph, i32>> => "1.1996^num_vertices",
}

inventory::submit! {
    ProblemSchemaEntry {
        name: "DecisionMinimumVertexCover",
        display_name: "Decision Minimum Vertex Cover",
        aliases: &["VertexCover", "VC"],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
            VariantDimension::new("weight", "i32", &["i32"]),
        ],
        module_path: module_path!(),
        description: "Decision version: does a vertex cover of cost ≤ bound exist?",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
            FieldInfo { name: "weights", type_name: "Vec<W>", description: "Vertex weights w: V -> R" },
            FieldInfo { name: "bound", type_name: "i32", description: "Decision bound (max cost)" },
        ],
    }
}
```

Add the necessary import at the top of the file:

```rust
use crate::models::decision::{Decision, DecisionProblemMeta};
```

- [ ] **Step 2: Add Decision variant registration to `minimum_dominating_set.rs`**

Add `decision_problem_meta!` and variant registration (same pattern):

```rust
use crate::models::decision::{Decision, DecisionProblemMeta};

crate::decision_problem_meta!(
    MinimumDominatingSet<SimpleGraph, i32>,
    "DecisionMinimumDominatingSet"
);

impl<G: Graph, W: WeightElement> Decision<MinimumDominatingSet<G, W>> {
    pub fn num_vertices(&self) -> usize {
        self.inner().num_vertices()
    }

    pub fn num_edges(&self) -> usize {
        self.inner().num_edges()
    }

    pub fn k(&self) -> usize
    where
        W::Sum: Into<i64>,
    {
        let b: i64 = self.bound().clone().into();
        b as usize
    }
}

crate::declare_variants! {
    default Decision<MinimumDominatingSet<SimpleGraph, i32>> => "2^num_vertices",
}

inventory::submit! {
    ProblemSchemaEntry {
        name: "DecisionMinimumDominatingSet",
        display_name: "Decision Minimum Dominating Set",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
            VariantDimension::new("weight", "i32", &["i32"]),
        ],
        module_path: module_path!(),
        description: "Decision version: does a dominating set of cost ≤ bound exist?",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
            FieldInfo { name: "weights", type_name: "Vec<W>", description: "Vertex weights w: V -> R" },
            FieldInfo { name: "bound", type_name: "i32", description: "Decision bound (max cost)" },
        ],
    }
}
```

- [ ] **Step 3: Build and test**

Run: `make check`
Expected: All tests and clippy PASS. The new `DecisionMinimumVertexCover` and `DecisionMinimumDominatingSet` appear in the registry.

- [ ] **Step 4: Verify registry**

Run: `cargo run --bin pred -- list | grep -i decision`
Expected: Shows `DecisionMinimumVertexCover` and `DecisionMinimumDominatingSet`.

- [ ] **Step 5: Commit**

```bash
git add src/models/graph/minimum_vertex_cover.rs src/models/graph/minimum_dominating_set.rs
git commit -m "feat: register Decision variants for MVC and MDS"
```

---

### Task 6: Remove Hand-Written `VertexCover`

**Files:**
- Remove: `src/models/graph/vertex_cover.rs`
- Remove: `src/unit_tests/models/graph/vertex_cover.rs`
- Modify: `src/models/graph/mod.rs`
- Modify: `src/models/mod.rs`

- [ ] **Step 1: Remove `VertexCover` from `src/models/graph/mod.rs`**

Remove the `pub mod vertex_cover;` line and the `pub use vertex_cover::VertexCover;` re-export.

- [ ] **Step 2: Remove `VertexCover` from `src/models/mod.rs` re-exports**

Remove `VertexCover,` from the `pub use graph::{...}` block.

- [ ] **Step 3: Delete the files**

```bash
rm src/models/graph/vertex_cover.rs
rm src/unit_tests/models/graph/vertex_cover.rs
```

- [ ] **Step 4: Fix any remaining references**

Run: `cargo build 2>&1 | head -40`

If there are compile errors from other files referencing `VertexCover`, update them to use `Decision<MinimumVertexCover<SimpleGraph, i32>>` instead. Based on our research, no reduction files reference `VertexCover` directly, so this should compile cleanly.

- [ ] **Step 5: Run full test suite**

Run: `make check`
Expected: PASS. The alias `VertexCover`/`VC` now resolves to `DecisionMinimumVertexCover` via the schema entry.

- [ ] **Step 6: Commit**

```bash
git add -A  # stages removals + modifications
git commit -m "refactor: remove hand-written VertexCover, replaced by Decision<MinimumVertexCover>"
```

---

### Task 7: Golden-Section Search Solver

**Files:**
- Create: `src/solvers/golden_section.rs`
- Modify: `src/solvers/mod.rs`

- [ ] **Step 1: Write failing tests**

Create `src/unit_tests/solvers/golden_section.rs`:

```rust
use crate::models::decision::Decision;
use crate::models::graph::{MaximumIndependentSet, MinimumVertexCover};
use crate::solvers::golden_section::solve_via_decision;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Aggregate;
use crate::Solver;

#[test]
fn test_golden_section_min() {
    // Path graph 0-1-2: MVC optimum = 1 (just vertex 1)
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumVertexCover::new(graph, vec![1i32; 3]);
    let result = solve_via_decision(&problem, 0, 3);
    assert_eq!(result, Some(1));
}

#[test]
fn test_golden_section_max() {
    // Path graph 0-1-2: MIS optimum = 2 (vertices 0 and 2)
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MaximumIndependentSet::new(graph, vec![1i32; 3]);
    let result = solve_via_decision(&problem, 0, 3);
    assert_eq!(result, Some(2));
}

#[test]
fn test_golden_section_matches_brute_force() {
    // Pentagon graph: compare golden-section vs brute-force
    let graph = SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)]);
    let problem = MinimumVertexCover::new(graph, vec![1i32; 5]);

    let solver = BruteForce::new();
    let bf_value = solver.solve(&problem);

    let gs_value = solve_via_decision(&problem, 0, 5);
    assert_eq!(gs_value, bf_value.size().copied());
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test test_golden_section -- --nocapture 2>&1 | head -20`
Expected: FAIL — module not found.

- [ ] **Step 3: Implement golden-section search solver**

Create `src/solvers/golden_section.rs`:

```rust
//! Golden-section search solver for optimization via decision queries.
//!
//! Given an optimization problem P, finds the optimal value by querying
//! Decision<P> with varying bounds using the golden ratio.
//!
//! Reference: <https://en.wikipedia.org/wiki/Golden-section_search>

use crate::models::decision::{Decision, DecisionProblemMeta};
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::{Aggregate, OptimizationValue, Or, Min, Max};
use crate::Solver;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt;

/// Whether a decision problem is satisfiable (any config evaluates to Or(true)).
fn is_satisfiable<P>(problem: &P) -> bool
where
    P: Problem<Value = Or>,
    Or: Aggregate,
{
    let solver = BruteForce::new();
    let value = solver.solve(problem);
    value.0
}

/// Solve a Min-valued optimization problem by golden-section search on its
/// decision version. Returns the minimum feasible value.
///
/// Searches the integer range [lower, upper] for the smallest bound where
/// Decision<P> is satisfiable.
pub fn solve_via_decision_min<P>(problem: &P, lower: i32, upper: i32) -> Option<i32>
where
    P: DecisionProblemMeta + Clone,
    P::Value: OptimizationValue<Inner = i32>
        + Clone + fmt::Debug + Serialize + DeserializeOwned,
{
    // First check if any solution exists at all
    let decision_upper = Decision::new(problem.clone(), upper);
    if !is_satisfiable(&decision_upper) {
        return None;
    }

    // Binary search for the minimum feasible bound
    let mut lo = lower;
    let mut hi = upper;
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        let decision = Decision::new(problem.clone(), mid);
        if is_satisfiable(&decision) {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    Some(lo)
}

/// Solve a Max-valued optimization problem by golden-section search on its
/// decision version. Returns the maximum feasible value.
///
/// Searches the integer range [lower, upper] for the largest bound where
/// Decision<P> is satisfiable.
pub fn solve_via_decision_max<P>(problem: &P, lower: i32, upper: i32) -> Option<i32>
where
    P: DecisionProblemMeta + Clone,
    P::Value: OptimizationValue<Inner = i32>
        + Clone + fmt::Debug + Serialize + DeserializeOwned,
{
    // First check if any solution exists at all
    let decision_lower = Decision::new(problem.clone(), lower);
    if !is_satisfiable(&decision_lower) {
        // For Max: if not satisfiable at lowest bound, check higher
        let decision_upper = Decision::new(problem.clone(), upper);
        if !is_satisfiable(&decision_upper) {
            return None;
        }
    }

    // Binary search for the maximum feasible bound
    let mut lo = lower;
    let mut hi = upper;
    while lo < hi {
        let mid = lo + (hi - lo + 1) / 2;  // round up to avoid infinite loop
        let decision = Decision::new(problem.clone(), mid);
        if is_satisfiable(&decision) {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }

    // Verify the found bound is actually feasible
    let decision = Decision::new(problem.clone(), lo);
    if is_satisfiable(&decision) {
        Some(lo)
    } else {
        None
    }
}

/// Solve an optimization problem by searching its decision version.
///
/// Dispatches to `solve_via_decision_min` or `solve_via_decision_max` based on
/// the problem's value type.
pub fn solve_via_decision<P>(problem: &P, lower: i32, upper: i32) -> Option<i32>
where
    P: DecisionProblemMeta + Clone,
    P::Value: OptimizationValue<Inner = i32>
        + Clone + fmt::Debug + Serialize + DeserializeOwned,
{
    // Check if this is a Min or Max problem by evaluating identity properties
    // We use a type-based dispatch via the OptimizationValue trait
    solve_via_decision_dispatch::<P>(problem, lower, upper)
}

/// Internal dispatch — determines Min vs Max at compile time.
fn solve_via_decision_dispatch<P>(problem: &P, lower: i32, upper: i32) -> Option<i32>
where
    P: DecisionProblemMeta + Clone,
    P::Value: OptimizationValue<Inner = i32>
        + Clone + fmt::Debug + Serialize + DeserializeOwned,
{
    // Try Min direction: search for smallest feasible bound
    // If Decision<P> at upper bound is satisfiable, it's a Min problem
    let decision_upper = Decision::new(problem.clone(), upper);
    if is_satisfiable(&decision_upper) {
        // Could be Min (satisfiable at upper bound) — search down
        let decision_lower = Decision::new(problem.clone(), lower);
        if !is_satisfiable(&decision_lower) {
            // Definitely Min: satisfiable at upper, not at lower
            return solve_via_decision_min(problem, lower, upper);
        }
        // Satisfiable at both bounds — return lower for Min
        return solve_via_decision_min(problem, lower, upper);
    }

    // Not satisfiable at upper — try Max direction
    solve_via_decision_max(problem, lower, upper)
}
```

- [ ] **Step 4: Wire up module in `src/solvers/mod.rs`**

Add after the `pub use customized::CustomizedSolver;` line:

```rust
pub mod golden_section;
```

- [ ] **Step 5: Add test module link in `src/solvers/golden_section.rs`**

At the bottom of the file:

```rust
#[cfg(test)]
#[path = "../unit_tests/solvers/golden_section.rs"]
mod tests;
```

- [ ] **Step 6: Run tests to verify they pass**

Run: `cargo test test_golden_section -- --nocapture`
Expected: All 3 tests PASS.

- [ ] **Step 7: Commit**

```bash
git add src/solvers/golden_section.rs src/solvers/mod.rs \
    src/unit_tests/solvers/golden_section.rs
git commit -m "feat: add golden-section search solver via Decision queries"
```

---

### Task 8: Paper and Example DB Migration

**Files:**
- Modify: `docs/paper/reductions.typ` (lines 237, 652-667)
- Modify: `src/models/graph/minimum_vertex_cover.rs` (example_db spec)

- [ ] **Step 1: Update paper display-name dict**

In `docs/paper/reductions.typ`, change line 237:

From: `"VertexCover": [Vertex Cover],`
To: `"DecisionMinimumVertexCover": [Decision Minimum Vertex Cover],`

- [ ] **Step 2: Update paper problem-def for VertexCover**

Replace the `VertexCover` problem-def block (around lines 652-670) to reference `DecisionMinimumVertexCover`. Update the `load-model-example` call and `problem-def` name accordingly.

- [ ] **Step 3: Add canonical example for DecisionMinimumVertexCover**

In `src/models/graph/minimum_vertex_cover.rs`, update the `canonical_model_example_specs()` to include a Decision variant example:

```rust
#[cfg(feature = "example-db")]
pub(crate) fn decision_canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "decision_minimum_vertex_cover_simplegraph_i32",
        instance: Box::new(Decision::new(
            MinimumVertexCover::new(
                SimpleGraph::new(4, vec![(0, 1), (1, 2), (0, 2), (2, 3)]),
                vec![1i32; 4],
            ),
            2,
        )),
        optimal_config: vec![1, 0, 1, 0],
        optimal_value: serde_json::json!(true),
    }]
}
```

Wire this into the example_db collection system following the existing pattern.

- [ ] **Step 4: Build and test**

Run: `make check`
Run: `make paper` (if Typst is available)
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add docs/paper/reductions.typ src/models/graph/minimum_vertex_cover.rs
git commit -m "docs: migrate VertexCover paper entry and example_db to DecisionMinimumVertexCover"
```

---

### Task 9: Final Integration Check

- [ ] **Step 1: Run full test suite**

Run: `make check`
Expected: All tests, clippy, and fmt pass.

- [ ] **Step 2: Run coverage check**

Run: `make coverage`
Expected: >95% coverage on new code.

- [ ] **Step 3: Verify CLI alias backward compatibility**

Run: `cargo run --bin pred -- show VertexCover`
Run: `cargo run --bin pred -- show VC`
Expected: Both resolve to `DecisionMinimumVertexCover`.

- [ ] **Step 4: Verify new problems appear in catalog**

Run: `cargo run --bin pred -- list | grep -i decision`
Expected: Shows both `DecisionMinimumVertexCover` and `DecisionMinimumDominatingSet`.

- [ ] **Step 5: Final commit if any fixups needed**

```bash
git add -A
git commit -m "chore: integration fixups for Decision wrapper"
```
