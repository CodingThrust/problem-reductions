# SolutionSize Enum Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add explicit `SolutionSize<T>` enum for validity tracking in optimization problems, replacing magic MIN/MAX values.

**Architecture:** Introduce `SolutionSize::Valid(T)` and `SolutionSize::Invalid` enum. Optimization problems return `SolutionSize<W>` as their Metric. The `OptimizationProblem` trait provides `is_better(&self, a, b) -> bool` for direction-aware comparison. Satisfaction problems keep `Metric = bool` unchanged.

**Tech Stack:** Rust, serde (for serialization)

---

## Task 1: Add SolutionSize enum to types.rs

**Files:**
- Modify: `src/types.rs`
- Test: `src/unit_tests/types.rs`

**Step 1: Write the failing test**

Add to `src/unit_tests/types.rs`:

```rust
#[test]
fn test_solution_size_valid() {
    let size: SolutionSize<i32> = SolutionSize::Valid(42);
    assert!(size.is_valid());
    assert_eq!(size.size(), Some(&42));
}

#[test]
fn test_solution_size_invalid() {
    let size: SolutionSize<i32> = SolutionSize::Invalid;
    assert!(!size.is_valid());
    assert_eq!(size.size(), None);
}

#[test]
fn test_solution_size_unwrap() {
    let valid: SolutionSize<i32> = SolutionSize::Valid(10);
    assert_eq!(valid.unwrap(), 10);
}

#[test]
#[should_panic(expected = "called unwrap on Invalid")]
fn test_solution_size_unwrap_panics() {
    let invalid: SolutionSize<i32> = SolutionSize::Invalid;
    invalid.unwrap();
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_solution_size --lib`
Expected: FAIL with "cannot find type `SolutionSize`"

**Step 3: Write the implementation**

Add to `src/types.rs`:

```rust
/// Result of evaluating a constrained optimization problem.
///
/// For optimization problems with constraints (like MaximumIndependentSet),
/// configurations may be infeasible. This enum explicitly represents validity.
///
/// # Example
///
/// ```
/// use problemreductions::types::SolutionSize;
///
/// let valid = SolutionSize::Valid(42);
/// assert!(valid.is_valid());
/// assert_eq!(valid.size(), Some(&42));
///
/// let invalid: SolutionSize<i32> = SolutionSize::Invalid;
/// assert!(!invalid.is_valid());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SolutionSize<T> {
    /// A valid (feasible) solution with the given objective value.
    Valid(T),
    /// An invalid (infeasible) solution that violates constraints.
    Invalid,
}

impl<T> SolutionSize<T> {
    /// Returns true if this is a valid solution.
    pub fn is_valid(&self) -> bool {
        matches!(self, SolutionSize::Valid(_))
    }

    /// Returns the size if valid, None if invalid.
    pub fn size(&self) -> Option<&T> {
        match self {
            SolutionSize::Valid(t) => Some(t),
            SolutionSize::Invalid => None,
        }
    }

    /// Unwraps the size, panicking if invalid.
    pub fn unwrap(self) -> T {
        match self {
            SolutionSize::Valid(t) => t,
            SolutionSize::Invalid => panic!("called unwrap on Invalid SolutionSize"),
        }
    }

    /// Maps the inner value if valid.
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> SolutionSize<U> {
        match self {
            SolutionSize::Valid(t) => SolutionSize::Valid(f(t)),
            SolutionSize::Invalid => SolutionSize::Invalid,
        }
    }
}

impl<T: Default> Default for SolutionSize<T> {
    fn default() -> Self {
        SolutionSize::Invalid
    }
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test test_solution_size --lib`
Expected: PASS

**Step 5: Commit**

```bash
git add src/types.rs src/unit_tests/types.rs
git commit -m "feat: add SolutionSize enum for explicit validity tracking"
```

---

## Task 2: Add is_better method to OptimizationProblem trait

**Files:**
- Modify: `src/traits.rs`
- Modify: `src/types.rs` (export SolutionSize)
- Test: `src/unit_tests/traits.rs`

**Step 1: Write the failing test**

Add to `src/unit_tests/traits.rs`:

```rust
use crate::types::{Direction, SolutionSize};

#[test]
fn test_is_better_maximize_valid_vs_valid() {
    // For maximization: larger is better
    let a = SolutionSize::Valid(10);
    let b = SolutionSize::Valid(5);
    assert!(is_better(&a, &b, Direction::Maximize));
    assert!(!is_better(&b, &a, Direction::Maximize));
}

#[test]
fn test_is_better_minimize_valid_vs_valid() {
    // For minimization: smaller is better
    let a = SolutionSize::Valid(5);
    let b = SolutionSize::Valid(10);
    assert!(is_better(&a, &b, Direction::Minimize));
    assert!(!is_better(&b, &a, Direction::Minimize));
}

#[test]
fn test_is_better_valid_vs_invalid() {
    // Valid is always better than invalid
    let valid = SolutionSize::Valid(0);
    let invalid: SolutionSize<i32> = SolutionSize::Invalid;
    assert!(is_better(&valid, &invalid, Direction::Maximize));
    assert!(is_better(&valid, &invalid, Direction::Minimize));
    assert!(!is_better(&invalid, &valid, Direction::Maximize));
    assert!(!is_better(&invalid, &valid, Direction::Minimize));
}

#[test]
fn test_is_better_invalid_vs_invalid() {
    // Neither invalid is better
    let a: SolutionSize<i32> = SolutionSize::Invalid;
    let b: SolutionSize<i32> = SolutionSize::Invalid;
    assert!(!is_better(&a, &b, Direction::Maximize));
    assert!(!is_better(&a, &b, Direction::Minimize));
}

#[test]
fn test_is_better_equal_valid() {
    // Equal values: neither is better
    let a = SolutionSize::Valid(5);
    let b = SolutionSize::Valid(5);
    assert!(!is_better(&a, &b, Direction::Maximize));
    assert!(!is_better(&a, &b, Direction::Minimize));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_is_better --lib`
Expected: FAIL with "cannot find function `is_better`"

**Step 3: Write the implementation**

Add to `src/types.rs`:

```rust
impl<T: Ord> SolutionSize<T> {
    /// Returns true if self is a better solution than other for the given direction.
    ///
    /// - For maximization: larger values are better
    /// - For minimization: smaller values are better
    /// - Valid solutions are always better than invalid ones
    /// - Two invalid solutions are equally bad (neither is better)
    pub fn is_better(&self, other: &Self, direction: Direction) -> bool {
        match (self, other) {
            (SolutionSize::Valid(a), SolutionSize::Valid(b)) => match direction {
                Direction::Maximize => a > b,
                Direction::Minimize => a < b,
            },
            (SolutionSize::Valid(_), SolutionSize::Invalid) => true,
            (SolutionSize::Invalid, SolutionSize::Valid(_)) => false,
            (SolutionSize::Invalid, SolutionSize::Invalid) => false,
        }
    }
}
```

Update test to use method:

```rust
fn is_better<T: Ord>(a: &SolutionSize<T>, b: &SolutionSize<T>, dir: Direction) -> bool {
    a.is_better(b, dir)
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test test_is_better --lib`
Expected: PASS

**Step 5: Update exports**

In `src/lib.rs`, add `SolutionSize` to prelude and re-exports:

```rust
pub use types::{Direction, NumericSize, NumericSizeBounds, ProblemSize, SolutionSize, Unweighted, Weights};
```

In `src/prelude.rs` section of `src/lib.rs`:

```rust
pub use crate::types::{Direction, NumericSize, NumericSizeBounds, NumericWeight, ProblemSize, SolutionSize, Unweighted, Weights};
```

**Step 6: Commit**

```bash
git add src/types.rs src/traits.rs src/lib.rs src/unit_tests/traits.rs
git commit -m "feat: add is_better method to SolutionSize for direction-aware comparison"
```

---

## Task 3: Update Solver trait and BruteForce implementation

**Files:**
- Modify: `src/solvers/mod.rs`
- Modify: `src/solvers/brute_force.rs`
- Test: `src/unit_tests/solvers/brute_force.rs`

**Step 1: Update Solver trait**

In `src/solvers/mod.rs`, change `find_best` signature:

```rust
use crate::traits::{OptimizationProblem, Problem};
use crate::types::SolutionSize;

/// Trait for problem solvers.
pub trait Solver {
    /// Find best solution(s) for an optimization problem.
    ///
    /// Returns all configurations that achieve the optimal metric value.
    /// Returns empty vec if all configurations are invalid.
    fn find_best<P>(&self, problem: &P) -> Vec<Vec<usize>>
    where
        P: OptimizationProblem,
        P::Metric: Clone;

    /// Find any satisfying solution for a satisfaction problem (Metric = bool).
    fn find_satisfying<P: Problem<Metric = bool>>(&self, problem: &P) -> Option<Vec<usize>>;
}
```

Note: Remove `find_all_satisfying` from the trait (internal only).

**Step 2: Update BruteForce implementation**

In `src/solvers/brute_force.rs`:

```rust
use crate::config::DimsIterator;
use crate::solvers::Solver;
use crate::traits::{OptimizationProblem, Problem};
use crate::types::SolutionSize;

impl Solver for BruteForce {
    fn find_best<P>(&self, problem: &P) -> Vec<Vec<usize>>
    where
        P: OptimizationProblem,
        P::Metric: Clone,
    {
        self.find_all_best(problem)
    }

    fn find_satisfying<P: Problem<Metric = bool>>(&self, problem: &P) -> Option<Vec<usize>> {
        let dims = problem.dims();
        if dims.is_empty() {
            return None;
        }
        DimsIterator::new(dims).find(|config| problem.evaluate(config))
    }
}

impl BruteForce {
    /// Internal: find all optimal solutions.
    fn find_all_best<P>(&self, problem: &P) -> Vec<Vec<usize>>
    where
        P: OptimizationProblem,
        P::Metric: Clone,
    {
        let dims = problem.dims();
        if dims.is_empty() {
            return vec![];
        }

        let iter = DimsIterator::new(dims);
        let mut best_solutions: Vec<Vec<usize>> = vec![];
        let mut best_metric: Option<P::Metric> = None;

        for config in iter {
            let metric = problem.evaluate(&config);

            let dominated = match &best_metric {
                None => false,
                Some(current_best) => problem.is_better(current_best, &metric),
            };

            if dominated {
                continue;
            }

            let dominates = match &best_metric {
                None => true,
                Some(current_best) => problem.is_better(&metric, current_best),
            };

            if dominates {
                best_metric = Some(metric);
                best_solutions.clear();
                best_solutions.push(config);
            } else if best_metric.is_some() {
                // Equal quality - add to solutions
                best_solutions.push(config);
            }
        }

        best_solutions
    }

    /// Find all satisfying solutions (internal, used for testing).
    pub(crate) fn find_all_satisfying<P: Problem<Metric = bool>>(
        &self,
        problem: &P,
    ) -> Vec<Vec<usize>> {
        let dims = problem.dims();
        if dims.is_empty() {
            return vec![];
        }
        DimsIterator::new(dims)
            .filter(|config| problem.evaluate(config))
            .collect()
    }
}
```

**Step 3: Run tests**

Run: `cargo test --lib`
Expected: Many failures (models still use old Metric type)

**Step 4: Commit intermediate progress**

```bash
git add src/solvers/mod.rs src/solvers/brute_force.rs
git commit -m "refactor: update Solver trait for SolutionSize-based metrics"
```

---

## Task 4: Add is_better to OptimizationProblem trait

**Files:**
- Modify: `src/traits.rs`

**Step 1: Update OptimizationProblem trait**

```rust
/// Extension for problems with a numeric objective to optimize.
pub trait OptimizationProblem: Problem {
    /// Whether to maximize or minimize the metric.
    fn direction(&self) -> crate::types::Direction;

    /// Returns true if metric `a` is better than metric `b` for this problem.
    fn is_better(&self, a: &Self::Metric, b: &Self::Metric) -> bool;
}
```

**Step 2: Commit**

```bash
git add src/traits.rs
git commit -m "feat: add is_better method to OptimizationProblem trait"
```

---

## Task 5: Update MaximumIndependentSet model

**Files:**
- Modify: `src/models/graph/maximum_independent_set.rs`
- Test: `src/unit_tests/models/graph/maximum_independent_set.rs`

**Step 1: Update imports and Problem impl**

```rust
use crate::types::{Direction, SolutionSize};

impl<G, W> Problem for MaximumIndependentSet<G, W>
where
    G: Graph,
    W: Clone + Default + PartialOrd + Ord + num_traits::Num + num_traits::Zero + std::ops::AddAssign + 'static,
{
    const NAME: &'static str = "MaximumIndependentSet";
    type Metric = SolutionSize<W>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("graph", crate::variant::short_type_name::<G>()),
            ("weight", crate::variant::short_type_name::<W>()),
        ]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_vertices()]
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<W> {
        if !is_independent_set_config(&self.graph, config) {
            return SolutionSize::Invalid;
        }
        let mut total = W::zero();
        for (i, &selected) in config.iter().enumerate() {
            if selected == 1 {
                total += self.weights[i].clone();
            }
        }
        SolutionSize::Valid(total)
    }
}

impl<G, W> OptimizationProblem for MaximumIndependentSet<G, W>
where
    G: Graph,
    W: Clone + Default + PartialOrd + Ord + num_traits::Num + num_traits::Zero + std::ops::AddAssign + 'static,
{
    fn direction(&self) -> Direction {
        Direction::Maximize
    }

    fn is_better(&self, a: &Self::Metric, b: &Self::Metric) -> bool {
        a.is_better(b, self.direction())
    }
}
```

**Step 2: Update unit tests**

In `src/unit_tests/models/graph/maximum_independent_set.rs`, update tests to use `SolutionSize`:

```rust
use crate::types::SolutionSize;

#[test]
fn test_evaluate_valid() {
    let problem = MaximumIndependentSet::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
    // Select vertex 2 only (not adjacent to anything selected)
    let config = vec![0, 0, 1];
    assert_eq!(problem.evaluate(&config), SolutionSize::Valid(1));
}

#[test]
fn test_evaluate_invalid() {
    let problem = MaximumIndependentSet::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
    // Select both 0 and 1 (adjacent - invalid)
    let config = vec![1, 1, 0];
    assert_eq!(problem.evaluate(&config), SolutionSize::Invalid);
}
```

**Step 3: Run tests**

Run: `cargo test maximum_independent_set --lib`
Expected: PASS

**Step 4: Commit**

```bash
git add src/models/graph/maximum_independent_set.rs src/unit_tests/models/graph/maximum_independent_set.rs
git commit -m "refactor: update MaximumIndependentSet to use SolutionSize"
```

---

## Task 6: Update MinimumVertexCover model

**Files:**
- Modify: `src/models/graph/minimum_vertex_cover.rs`
- Test: `src/unit_tests/models/graph/minimum_vertex_cover.rs`

**Step 1: Update Problem and OptimizationProblem impl**

Same pattern as Task 5, but:
- `evaluate` returns `SolutionSize::Invalid` when not a valid cover
- `direction()` returns `Direction::Minimize`

```rust
fn evaluate(&self, config: &[usize]) -> SolutionSize<W> {
    if !is_vertex_cover_config(&self.graph, config) {
        return SolutionSize::Invalid;
    }
    let mut total = W::zero();
    for (i, &selected) in config.iter().enumerate() {
        if selected == 1 {
            total += self.weights[i].clone();
        }
    }
    SolutionSize::Valid(total)
}
```

**Step 2: Update tests, run, commit**

```bash
git add src/models/graph/minimum_vertex_cover.rs src/unit_tests/models/graph/minimum_vertex_cover.rs
git commit -m "refactor: update MinimumVertexCover to use SolutionSize"
```

---

## Task 7: Update remaining graph models

**Files:**
- Modify: `src/models/graph/max_cut.rs`
- Modify: `src/models/graph/minimum_dominating_set.rs`
- Modify: `src/models/graph/maximal_is.rs`
- Modify: `src/models/graph/maximum_matching.rs`
- Modify: `src/models/graph/maximum_clique.rs`

For each model:
1. Change `type Metric = W` to `type Metric = SolutionSize<W>`
2. Update `evaluate` to return `SolutionSize::Valid(value)` or `SolutionSize::Invalid`
3. Add `is_better` method to `OptimizationProblem` impl
4. Update corresponding unit tests

**Note:** MaxCut may have no invalid configurations (all cuts are valid), so it always returns `SolutionSize::Valid(cut_value)`.

**Commit after each model:**

```bash
git commit -m "refactor: update <ModelName> to use SolutionSize"
```

---

## Task 8: Update set models

**Files:**
- Modify: `src/models/set/maximum_set_packing.rs`
- Modify: `src/models/set/minimum_set_covering.rs`

Same pattern as graph models.

---

## Task 9: Update optimization models

**Files:**
- Modify: `src/models/optimization/spin_glass.rs`
- Modify: `src/models/optimization/qubo.rs`
- Modify: `src/models/optimization/ilp.rs`

**Note:** SpinGlass and QUBO are unconstrained - they always return `SolutionSize::Valid(energy)`. ILP has constraints.

---

## Task 10: Update specialized models

**Files:**
- Modify: `src/models/specialized/paintshop.rs`
- Modify: `src/models/specialized/bmf.rs`
- Modify: `src/models/specialized/biclique_cover.rs`

Skip satisfaction models (Factoring, CircuitSAT) - they keep `Metric = bool`.

---

## Task 11: Update reduction rules

**Files:**
- All files in `src/rules/` that use `evaluate()` or compare metrics

Key changes:
- Update `extract_solution` methods if they check validity
- Update any code that compares metric values directly

---

## Task 12: Update examples

**Files:**
- All files in `examples/`

Update patterns:
- Change `problem.evaluate(&config) > i32::MIN` to `problem.evaluate(&config).is_valid()`
- Change `metric.is_min_bound()` to `!metric.is_valid()`
- Use `metric.unwrap()` or `metric.size()` to get the value

---

## Task 13: Update integration tests

**Files:**
- `tests/suites/integration.rs`
- `tests/suites/reductions.rs`

Same patterns as examples.

---

## Task 14: Update benchmarks

**Files:**
- `benches/solver_benchmarks.rs`

Ensure benchmarks compile with new API.

---

## Task 15: Final verification

**Step 1: Run all tests**

```bash
make test
```
Expected: All pass

**Step 2: Run clippy**

```bash
make clippy
```
Expected: No warnings

**Step 3: Run examples**

```bash
cargo run --example reduction_maximumindependentset_to_qubo
```
Expected: Success, JSON output unchanged

**Step 4: Final commit**

```bash
git add -A
git commit -m "feat: complete SolutionSize migration for explicit validity tracking"
```

---

## Summary

| Task | Description | Files |
|------|-------------|-------|
| 1 | Add SolutionSize enum | types.rs |
| 2 | Add is_better method | types.rs, traits.rs |
| 3 | Update Solver trait | solvers/*.rs |
| 4 | Update OptimizationProblem | traits.rs |
| 5-6 | Update MIS, MVC | graph/*.rs |
| 7 | Update other graph models | graph/*.rs |
| 8 | Update set models | set/*.rs |
| 9 | Update optimization models | optimization/*.rs |
| 10 | Update specialized models | specialized/*.rs |
| 11 | Update reduction rules | rules/*.rs |
| 12-14 | Update examples, tests, benchmarks | examples/, tests/, benches/ |
| 15 | Final verification | - |
