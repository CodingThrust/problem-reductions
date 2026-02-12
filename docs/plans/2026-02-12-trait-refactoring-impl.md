# Trait System Refactoring Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Refactor the core type system to simplify Problem/Reduction traits, making it easier for contributors to add problems and reductions.

**Architecture:** Replace the current 8-method `Problem` trait + `ConstraintSatisfactionProblem` with a minimal 2-method `Problem` trait (`dims` + `evaluate`) plus an `OptimizationProblem` extension. Introduce a `Weights` trait to separate weight storage from objective types. Simplify the proc macro to use trait-bound inspection instead of heuristics.

**Tech Stack:** Rust, proc-macro2/syn/quote (proc macro crate), inventory (static registration), serde, num-traits

**Design doc:** `docs/plans/2026-02-12-trait-refactoring-design.md`

---

## Task 1: Add `NumericSize` trait and `Weights` trait to `src/types.rs`

**Files:**
- Modify: `src/types.rs`

**Step 1: Write failing test**

Add to `src/unit_tests/types.rs`:

```rust
#[test]
fn test_numeric_size_blanket_impl() {
    fn assert_numeric_size<T: NumericSize>() {}
    assert_numeric_size::<i32>();
    assert_numeric_size::<i64>();
    assert_numeric_size::<f64>();
}

#[test]
fn test_unweighted_weights_trait() {
    let w = Unweighted(5);
    assert_eq!(w.len(), 5);
    assert_eq!(w.weight(0), 1);
    assert_eq!(w.weight(4), 1);
    assert_eq!(Unweighted::NAME, "Unweighted");
}

#[test]
fn test_vec_i32_weights_trait() {
    let w = vec![3, 1, 4];
    assert_eq!(w.len(), 3);
    assert_eq!(w.weight(0), 3);
    assert_eq!(w.weight(2), 4);
    assert_eq!(<Vec<i32> as Weights>::NAME, "Weighted<i32>");
}

#[test]
fn test_vec_f64_weights_trait() {
    let w = vec![1.5, 2.5];
    assert_eq!(w.len(), 2);
    assert_eq!(w.weight(1), 2.5);
    assert_eq!(<Vec<f64> as Weights>::NAME, "Weighted<f64>");
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test --lib test_numeric_size_blanket_impl test_unweighted_weights_trait test_vec_i32_weights_trait test_vec_f64_weights_trait`
Expected: FAIL — `NumericSize`, `Weights` not defined

**Step 3: Implement `NumericSize`, `Weights`, and refactored `Unweighted`**

In `src/types.rs`, add after the existing `NumericWeight` trait (we keep `NumericWeight` temporarily for backwards compat):

```rust
/// Bound for objective value types (i32, f64, etc.)
pub trait NumericSize:
    Clone + Default + PartialOrd + num_traits::Num + num_traits::Zero
    + std::ops::AddAssign + 'static
{}

impl<T> NumericSize for T
where
    T: Clone + Default + PartialOrd + num_traits::Num + num_traits::Zero
       + std::ops::AddAssign + 'static,
{}

/// Trait for weight storage. Separates weight storage from objective value type.
pub trait Weights: Clone + 'static {
    /// Name for variant metadata (e.g., "Unweighted", "Weighted<i32>").
    const NAME: &'static str;
    /// The objective/metric type derived from these weights.
    type Size: NumericSize;
    /// Get the weight at a given index.
    fn weight(&self, index: usize) -> Self::Size;
    /// Number of weights.
    fn len(&self) -> usize;
    /// Whether the weight vector is empty.
    fn is_empty(&self) -> bool { self.len() == 0 }
}
```

Change `Unweighted` from a zero-sized marker to a real weight vector:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct Unweighted(pub usize);

impl Weights for Unweighted {
    const NAME: &'static str = "Unweighted";
    type Size = i32;
    fn weight(&self, _index: usize) -> i32 { 1 }
    fn len(&self) -> usize { self.0 }
}

impl Weights for Vec<i32> {
    const NAME: &'static str = "Weighted<i32>";
    type Size = i32;
    fn weight(&self, index: usize) -> i32 { self[index] }
    fn len(&self) -> usize { self.len() }
}

impl Weights for Vec<f64> {
    const NAME: &'static str = "Weighted<f64>";
    type Size = f64;
    fn weight(&self, index: usize) -> f64 { self[index] }
    fn len(&self) -> usize { self.len() }
}
```

Keep `Unweighted::get()` method for backwards compat during migration.

**Step 4: Run test to verify it passes**

Run: `cargo test --lib test_numeric_size_blanket_impl test_unweighted_weights_trait test_vec_i32_weights_trait test_vec_f64_weights_trait`
Expected: PASS

**Step 5: Commit**

```bash
git add src/types.rs src/unit_tests/types.rs
git commit -m "feat: add NumericSize trait, Weights trait, and refactored Unweighted"
```

---

## Task 2: Add new `Problem` and `OptimizationProblem` traits to `src/traits.rs`

**Files:**
- Modify: `src/traits.rs`
- Modify: `src/unit_tests/traits.rs`

**Step 1: Write failing test**

Add to `src/unit_tests/traits.rs`:

```rust
use crate::types::{Direction, Weights};

#[derive(Clone)]
struct TestSatProblem {
    num_vars: usize,
    satisfying: Vec<Vec<usize>>,
}

impl crate::traits::ProblemV2 for TestSatProblem {
    const NAME: &'static str = "TestSat";
    type Metric = bool;
    fn dims(&self) -> Vec<usize> { vec![2; self.num_vars] }
    fn evaluate(&self, config: &[usize]) -> bool {
        self.satisfying.iter().any(|s| s == config)
    }
}

#[test]
fn test_problem_v2_sat() {
    let p = TestSatProblem {
        num_vars: 2,
        satisfying: vec![vec![1, 0], vec![0, 1]],
    };
    assert_eq!(p.dims(), vec![2, 2]);
    assert!(p.evaluate(&[1, 0]));
    assert!(!p.evaluate(&[0, 0]));
}

#[derive(Clone)]
struct TestOptProblem {
    weights: Vec<i32>,
}

impl crate::traits::ProblemV2 for TestOptProblem {
    const NAME: &'static str = "TestOpt";
    type Metric = i32;
    fn dims(&self) -> Vec<usize> { vec![2; self.weights.len()] }
    fn evaluate(&self, config: &[usize]) -> i32 {
        config.iter().enumerate()
            .map(|(i, &v)| if v == 1 { self.weights[i] } else { 0 })
            .sum()
    }
}

impl crate::traits::OptimizationProblemV2 for TestOptProblem {
    fn direction(&self) -> Direction { Direction::Maximize }
}

#[test]
fn test_optimization_problem_v2() {
    let p = TestOptProblem { weights: vec![3, 1, 4] };
    assert_eq!(p.evaluate(&[1, 0, 1]), 7);
    assert_eq!(p.direction(), Direction::Maximize);
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test --lib test_problem_v2_sat test_optimization_problem_v2`
Expected: FAIL — `ProblemV2`, `OptimizationProblemV2`, `Direction` not defined

**Step 3: Implement new traits**

In `src/types.rs`, add `Direction` enum:

```rust
/// Optimization direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Direction {
    /// Maximize the objective value.
    Maximize,
    /// Minimize the objective value.
    Minimize,
}
```

In `src/traits.rs`, add new traits (keep old ones during migration):

```rust
use crate::types::Direction;

/// Minimal problem trait — a problem is a function from configuration to metric.
pub trait ProblemV2: Clone {
    /// Base name of this problem type.
    const NAME: &'static str;
    /// The evaluation metric type.
    type Metric: Clone;
    /// Configuration space dimensions. Each entry is the cardinality of that variable.
    fn dims(&self) -> Vec<usize>;
    /// Evaluate the problem on a configuration.
    fn evaluate(&self, config: &[usize]) -> Self::Metric;
    /// Number of variables (derived from dims).
    fn num_variables(&self) -> usize { self.dims().len() }
    /// Returns variant attributes derived from type parameters.
    /// Used for generating variant IDs in the reduction graph schema.
    /// Returns pairs like `[("graph", "SimpleGraph"), ("weight", "i32")]`.
    fn variant() -> Vec<(&'static str, &'static str)>;
}

/// Extension for problems with a numeric objective to optimize.
pub trait OptimizationProblemV2: ProblemV2
where
    Self::Metric: crate::types::NumericSize,
{
    /// Whether to maximize or minimize the metric.
    fn direction(&self) -> Direction;
}
```

NOTE: We use `ProblemV2`/`OptimizationProblemV2` as temporary names. After all models are migrated (Task 6+), we rename to `Problem`/`OptimizationProblem` and remove old traits.

**Step 4: Run test to verify it passes**

Run: `cargo test --lib test_problem_v2_sat test_optimization_problem_v2`
Expected: PASS

**Step 5: Commit**

```bash
git add src/traits.rs src/types.rs src/unit_tests/traits.rs
git commit -m "feat: add ProblemV2, OptimizationProblemV2, and Direction"
```

---

## Task 3: Add new `ReductionResult` and `ReduceTo` traits in `src/rules/traits.rs`

**Files:**
- Modify: `src/rules/traits.rs`
- Modify: `src/unit_tests/rules/traits.rs`

**Step 1: Write failing test**

Add to `src/unit_tests/rules/traits.rs`:

```rust
use crate::traits::ProblemV2;
use crate::rules::traits::{ReductionResultV2, ReduceToV2};

#[derive(Clone)]
struct SourceProblem;
#[derive(Clone)]
struct TargetProblem;

impl ProblemV2 for SourceProblem {
    const NAME: &'static str = "Source";
    type Metric = i32;
    fn dims(&self) -> Vec<usize> { vec![2, 2] }
    fn evaluate(&self, config: &[usize]) -> i32 { (config[0] + config[1]) as i32 }
}

impl ProblemV2 for TargetProblem {
    const NAME: &'static str = "Target";
    type Metric = i32;
    fn dims(&self) -> Vec<usize> { vec![2, 2] }
    fn evaluate(&self, config: &[usize]) -> i32 { (config[0] + config[1]) as i32 }
}

#[derive(Clone)]
struct TestReduction { target: TargetProblem }

impl ReductionResultV2 for TestReduction {
    type Source = SourceProblem;
    type Target = TargetProblem;
    fn target_problem(&self) -> &TargetProblem { &self.target }
    fn extract_solution(&self, target_config: &[usize]) -> Vec<usize> {
        target_config.to_vec()
    }
}

impl ReduceToV2<TargetProblem> for SourceProblem {
    type Result = TestReduction;
    fn reduce_to(&self) -> TestReduction {
        TestReduction { target: TargetProblem }
    }
}

#[test]
fn test_reduction_v2() {
    let source = SourceProblem;
    let result = source.reduce_to();
    let target = result.target_problem();
    assert_eq!(target.evaluate(&[1, 1]), 2);
    assert_eq!(result.extract_solution(&[1, 0]), vec![1, 0]);
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test --lib test_reduction_v2`
Expected: FAIL — `ReductionResultV2`, `ReduceToV2` not defined

**Step 3: Implement new reduction traits**

In `src/rules/traits.rs`, add (keep old traits):

```rust
use crate::traits::ProblemV2;

/// Simplified reduction result — just target problem and solution extraction.
pub trait ReductionResultV2: Clone {
    type Source: ProblemV2;
    type Target: ProblemV2;
    fn target_problem(&self) -> &Self::Target;
    fn extract_solution(&self, target_config: &[usize]) -> Vec<usize>;
}

/// Simplified reduction trait.
pub trait ReduceToV2<T: ProblemV2>: ProblemV2 {
    type Result: ReductionResultV2<Source = Self, Target = T>;
    fn reduce_to(&self) -> Self::Result;
}
```

Update `src/rules/mod.rs` to also export new traits:

```rust
pub use traits::{ReduceTo, ReductionResult, ReduceToV2, ReductionResultV2};
```

**Step 4: Run test to verify it passes**

Run: `cargo test --lib test_reduction_v2`
Expected: PASS

**Step 5: Commit**

```bash
git add src/rules/traits.rs src/rules/mod.rs src/unit_tests/rules/traits.rs
git commit -m "feat: add ReductionResultV2 and ReduceToV2 traits"
```

---

## Task 4: Migrate one model as proof-of-concept — `MaximumIndependentSet`

This task validates the full migration pattern. All subsequent model migrations follow the same steps.

**Files:**
- Modify: `src/models/graph/maximum_independent_set.rs`
- Modify: `src/unit_tests/models/graph/maximum_independent_set.rs`

**Step 1: Write failing test for new trait impl**

Add to `src/unit_tests/models/graph/maximum_independent_set.rs`:

```rust
#[test]
fn test_mis_problem_v2() {
    use crate::traits::ProblemV2;
    use crate::types::Direction;

    // Triangle graph with unit weights
    let p = MaximumIndependentSet::<SimpleGraph, Vec<i32>>::with_weights(
        3, vec![(0, 1), (1, 2), (0, 2)], vec![1, 1, 1],
    );
    assert_eq!(p.dims(), vec![2, 2, 2]);
    // Valid IS: select vertex 0 only
    assert_eq!(p.evaluate(&[1, 0, 0]), 1);
    // Invalid IS: select adjacent 0,1 -> should return i32::MIN (neg inf for integers)
    assert_eq!(p.evaluate(&[1, 1, 0]), i32::MIN);
    assert_eq!(p.direction(), Direction::Maximize);
}

#[test]
fn test_mis_unweighted_v2() {
    use crate::traits::ProblemV2;
    use crate::types::Unweighted;

    let p = MaximumIndependentSet::<SimpleGraph, Unweighted>::new_unweighted(
        3, vec![(0, 1), (1, 2), (0, 2)],
    );
    assert_eq!(p.dims(), vec![2, 2, 2]);
    assert_eq!(p.evaluate(&[1, 0, 0]), 1);
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test --lib test_mis_problem_v2 test_mis_unweighted_v2`
Expected: FAIL

**Step 3: Add `ProblemV2` and `OptimizationProblemV2` impls to MIS**

In `src/models/graph/maximum_independent_set.rs`, add:

```rust
use crate::traits::{ProblemV2, OptimizationProblemV2};
use crate::types::{Direction, Weights};

impl MaximumIndependentSet<SimpleGraph, Unweighted> {
    pub fn new_unweighted(num_vertices: usize, edges: Vec<(usize, usize)>) -> Self {
        let graph = SimpleGraph::new(num_vertices, edges);
        Self { graph, weights: Unweighted(num_vertices) }
    }
}

impl<G, W> ProblemV2 for MaximumIndependentSet<G, W>
where
    G: Graph,
    W: Weights,
{
    const NAME: &'static str = "MaximumIndependentSet";
    type Metric = W::Size;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_vertices()]
    }

    fn evaluate(&self, config: &[usize]) -> W::Size {
        if !is_independent_set_config(&self.graph, config) {
            // Return worst value for maximization
            // For i32: i32::MIN, for f64: f64::NEG_INFINITY
            return <W::Size as num_traits::Bounded>::min_value();
        }
        let mut total = W::Size::zero();
        for (i, &selected) in config.iter().enumerate() {
            if selected == 1 {
                total += self.weights.weight(i);
            }
        }
        total
    }
}

impl<G, W> OptimizationProblemV2 for MaximumIndependentSet<G, W>
where
    G: Graph,
    W: Weights,
    W::Size: crate::types::NumericSize,
{
    fn direction(&self) -> Direction {
        Direction::Maximize
    }
}
```

NOTE: The "return worst value" pattern requires `num_traits::Bounded`. Add this bound to `NumericSize`:

In `src/types.rs`, update `NumericSize`:
```rust
pub trait NumericSize:
    Clone + Default + PartialOrd + num_traits::Num + num_traits::Zero
    + num_traits::Bounded + std::ops::AddAssign + 'static
{}
```

**Step 4: Run test to verify it passes**

Run: `cargo test --lib test_mis_problem_v2 test_mis_unweighted_v2`
Expected: PASS

**Step 5: Run full test suite to verify nothing is broken**

Run: `make test`
Expected: All existing tests still PASS (old traits untouched)

**Step 6: Commit**

```bash
git add src/types.rs src/traits.rs src/models/graph/maximum_independent_set.rs src/unit_tests/models/graph/maximum_independent_set.rs
git commit -m "feat: add ProblemV2 impl for MaximumIndependentSet (proof of concept)"
```

---

## Task 5: Migrate remaining graph models

Repeat the Task 4 pattern for each graph model. Each should implement `ProblemV2` (and `OptimizationProblemV2` where applicable).

**Files to modify (one per sub-step):**
- `src/models/graph/minimum_vertex_cover.rs` — `ProblemV2` + `OptimizationProblemV2` (Minimize)
- `src/models/graph/maximum_clique.rs` — `ProblemV2` + `OptimizationProblemV2` (Maximize)
- `src/models/graph/max_cut.rs` — `ProblemV2` + `OptimizationProblemV2` (Maximize)
- `src/models/graph/maximum_matching.rs` — `ProblemV2` + `OptimizationProblemV2` (Maximize)
- `src/models/graph/minimum_dominating_set.rs` — `ProblemV2` + `OptimizationProblemV2` (Minimize)
- `src/models/graph/maximal_is.rs` — `ProblemV2` + `OptimizationProblemV2` (Maximize)
- `src/models/graph/kcoloring.rs` — `ProblemV2` only (Metric = bool, no `OptimizationProblemV2`). Remove `PhantomData<W>`, change to `KColoring<G>` with runtime `k: usize` field.

For **KColoring** specifically, the struct changes to:

```rust
pub struct KColoring<G> {
    graph: G,
    k: usize,
}

impl<G: Graph> ProblemV2 for KColoring<G> {
    const NAME: &'static str = "KColoring";
    type Metric = bool;
    fn dims(&self) -> Vec<usize> { vec![self.k; self.graph.num_vertices()] }
    fn evaluate(&self, config: &[usize]) -> bool { self.is_valid_coloring(config) }
}
```

NOTE: KColoring changes its type signature (`<const K, G, W>` -> `<G>`), which breaks existing reductions that reference it. Keep the old struct as a type alias during migration:
```rust
pub type KColoringLegacy<const K: usize, G, W> = KColoring<G>;
```

**Commit after each model:** one commit per model file.

---

## Task 6: Migrate optimization models

**Files to modify:**
- `src/models/optimization/qubo.rs` — `ProblemV2` + `OptimizationProblemV2` (Minimize), `W: Weights` where `W::Size: Mul<Output = W::Size>`
- `src/models/optimization/spin_glass.rs` — `ProblemV2` + `OptimizationProblemV2` (Minimize)
- `src/models/optimization/ilp.rs` — `ProblemV2` + `OptimizationProblemV2` (uses `ObjectiveSense`)

**Commit after each model.**

---

## Task 7: Migrate satisfiability models

**Files to modify:**
- `src/models/satisfiability/sat.rs` — `ProblemV2` only (Metric = bool for SAT, or `W::Size` for MAX-SAT)
- `src/models/satisfiability/ksat.rs` — `ProblemV2` only (Metric = bool)
- `src/models/specialized/circuit.rs` — `ProblemV2` only (Metric = bool)
- `src/models/specialized/factoring.rs` — `ProblemV2` only (Metric = bool)

**Commit after each model.**

---

## Task 8: Migrate set models and remaining specialized models

**Files to modify:**
- `src/models/set/minimum_set_covering.rs` — `ProblemV2` + `OptimizationProblemV2` (Minimize)
- `src/models/set/maximum_set_packing.rs` — `ProblemV2` + `OptimizationProblemV2` (Maximize)
- `src/models/specialized/paintshop.rs` — `ProblemV2` + `OptimizationProblemV2` (Minimize)
- `src/models/specialized/biclique_cover.rs` — `ProblemV2` + `OptimizationProblemV2` (Minimize)
- `src/models/specialized/bmf.rs` — `ProblemV2` + `OptimizationProblemV2` (Minimize)

**Commit after each model.**

---

## Task 9: Update solvers to use new traits

**Files:**
- Modify: `src/solvers/mod.rs`
- Modify: `src/solvers/brute_force.rs`
- Modify: `src/unit_tests/solvers/brute_force.rs`

**Step 1: Add new `SolverV2` trait**

In `src/solvers/mod.rs`:

```rust
use crate::traits::{ProblemV2, OptimizationProblemV2};
use crate::types::Direction;

pub trait SolverV2 {
    /// Find best solution(s) for an optimization problem.
    fn find_best_optimization<P: OptimizationProblemV2>(
        &self, problem: &P,
    ) -> Vec<Vec<usize>>
    where P::Metric: crate::types::NumericSize;

    /// Find any satisfying solution for a satisfaction problem (Metric = bool).
    fn find_satisfying<P: ProblemV2<Metric = bool>>(
        &self, problem: &P,
    ) -> Option<Vec<usize>>;
}
```

**Step 2: Implement for `BruteForce`**

In `src/solvers/brute_force.rs`, add `SolverV2` impl that uses `evaluate()` and `direction()` instead of `solution_size()` and `energy_mode()`.

**Step 3: Test with new traits**

Add tests in `src/unit_tests/solvers/brute_force.rs` using `ProblemV2`-based problems.

**Step 4: Commit**

```bash
git add src/solvers/
git commit -m "feat: add SolverV2 using ProblemV2/OptimizationProblemV2"
```

---

## Task 10: Update proc macro for trait-bound inspection

**Files:**
- Modify: `problemreductions-macros/src/lib.rs`

**Step 1: Replace type extraction heuristics with trait-bound inspection**

Replace `extract_graph_type()`, `extract_weight_type()`, `is_weight_type()`, `get_weight_name()` with:

```rust
/// Inspect impl block's generic params and their bounds to identify roles.
fn extract_roles_from_bounds(impl_block: &ItemImpl) -> (Option<String>, Option<String>) {
    let mut graph_type = None;
    let mut weight_type = None;

    for param in &impl_block.generics.params {
        if let syn::GenericParam::Type(type_param) = param {
            for bound in &type_param.bounds {
                if let syn::TypeParamBound::Trait(trait_bound) = bound {
                    let trait_name = trait_bound.path.segments.last()
                        .map(|s| s.ident.to_string());
                    match trait_name.as_deref() {
                        Some("Graph") => graph_type = Some(type_param.ident.to_string()),
                        Some("Weights") => weight_type = Some(type_param.ident.to_string()),
                        _ => {}
                    }
                }
            }
        }
    }

    (graph_type, weight_type)
}
```

For concrete types in the signature (e.g., `SimpleGraph` in `ReduceTo<QUBO<f64>> for MIS<SimpleGraph, i32>`), match them against the type arguments and use string literals.

**Step 2: Update `generate_reduction_entry` to use new extraction**

Remove all the old `extract_graph_type`, `is_weight_type`, `get_weight_name` functions. The new logic:
1. Call `extract_roles_from_bounds()` to find which generic params are Graph/Weights
2. For generic params: use the trait's `NAME` constant at registration time
3. For concrete types in signatures: use literal strings
4. Emit compile error if structure is ambiguous

**Step 3: Test**

Run: `make test`
Expected: All existing reductions still compile and register correctly

**Step 4: Commit**

```bash
git add problemreductions-macros/src/lib.rs
git commit -m "refactor: replace macro heuristics with trait-bound inspection"
```

---

## Task 11: Swap old traits for new — the rename

Once all models, solvers, and reductions implement the V2 traits, perform the swap.

**Files:**
- Modify: `src/traits.rs` — rename `ProblemV2` -> `Problem`, `OptimizationProblemV2` -> `OptimizationProblem`, remove old `Problem` and `ConstraintSatisfactionProblem`. **Keep `fn variant() -> Vec<(&'static str, &'static str)>` in the Problem trait** for schema/registry variant ID generation.
- Modify: `src/rules/traits.rs` — rename `ReductionResultV2` -> `ReductionResult`, `ReduceToV2` -> `ReduceTo`, remove old traits
- Modify: `src/types.rs` — remove `EnergyMode`, `SolutionSize`, `LocalConstraint`, `LocalSolutionSize`, `NumericWeight`, old `Unweighted`. Remove `csp_solution_size()`. **Add `NumericSizeBounds` trait** for bound-checking in solvers.
- Modify: `src/lib.rs` — update prelude and re-exports
- Modify: `src/variant.rs` — **KEEP** `short_type_name` and `const_usize_str` (still used by `Problem::variant()` impls)
- Modify: ALL model files — remove old `Problem` / `CSP` impls, keep only new impls. **Each model must implement `fn variant()`** returning type parameter metadata.
- Modify: ALL rule files — update to use new traits
- Modify: ALL solver files — remove old `Solver` trait, keep `SolverV2` renamed to `Solver`
- Modify: ALL test files — update imports
- Modify: ALL example files — update to use new API (`solution_size` -> `evaluate`, keep `variant()` calls)

**This is the largest task.** Break it into sub-steps:

1. Rename traits in `src/traits.rs` and `src/rules/traits.rs`
2. Update `src/types.rs` (remove dead types)
3. Update `src/lib.rs` prelude
4. Update each model file (remove old impls)
5. Update each rule file
6. Update each solver file
7. Update each test file
8. Update each example file
9. Run `make test clippy` after each batch

**Commit frequently** — at minimum one commit per sub-step.

---

## Task 12: Clean up and verify

**Step 1: Run full test suite**

```bash
make test
```

**Step 2: Run clippy**

```bash
make clippy
```

**Step 3: Check formatting**

```bash
make fmt-check
```

**Step 4: Run coverage**

```bash
make coverage
```
Expected: >95% coverage

**Step 5: Regenerate reduction graph**

```bash
cargo run --example export_graph
```

**Step 6: Build docs**

```bash
make doc
```

**Step 7: Final commit**

```bash
git add -A
git commit -m "chore: cleanup after trait refactoring"
```

---

## Migration Strategy Summary

The key principle is **parallel existence**: new traits (`ProblemV2`, `OptimizationProblemV2`, `ReductionResultV2`, `ReduceToV2`) coexist with old traits throughout the migration. This means:

- The codebase compiles and all tests pass at every commit
- Models can be migrated one at a time
- The final rename (Task 11) is the only "big bang" change

**Dependency order:**
1. Types (`NumericSize`, `Weights`, `Direction`) — no dependencies
2. Traits (`ProblemV2`, `OptimizationProblemV2`) — depends on types
3. Reduction traits (`ReductionResultV2`, `ReduceToV2`) — depends on `ProblemV2`
4. Models — depends on all above
5. Solvers — depends on models + traits
6. Proc macro — independent (just registration metadata)
7. Rename — depends on everything being migrated
8. Cleanup — depends on rename
