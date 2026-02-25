# BinPacking Model Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add a `BinPacking` optimization problem model that assigns items with sizes to bins of fixed capacity, minimizing the number of bins used.

**Architecture:** `BinPacking<W>` struct parameterized by weight type `W` (for item sizes). Configuration space is `vec![n; n]` — each of `n` items maps to one of `n` possible bins. The objective (number of distinct bins) is always `i32` regardless of `W`. Follows the `MaximumSetPacking<W>` pattern (non-graph, weight-only variant).

**Tech Stack:** Rust, serde, inventory, num-traits

**Issue:** #95

---

### Task 1: Implement the BinPacking model

**Files:**
- Create: `src/models/optimization/bin_packing.rs`
- Modify: `src/models/optimization/mod.rs`
- Modify: `src/models/mod.rs`

**Step 1: Create `src/models/optimization/bin_packing.rs`**

```rust
//! Bin Packing problem implementation.
//!
//! The Bin Packing problem asks for an assignment of items to bins
//! that minimizes the number of bins used while respecting capacity constraints.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize, WeightElement};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "BinPacking",
        module_path: module_path!(),
        description: "Assign items to bins minimizing number of bins used, subject to capacity",
        fields: &[
            FieldInfo { name: "sizes", type_name: "Vec<W>", description: "Item sizes s_i for each item" },
            FieldInfo { name: "capacity", type_name: "W", description: "Bin capacity C" },
        ],
    }
}

/// The Bin Packing problem.
///
/// Given `n` items with sizes `s_1, ..., s_n` and bin capacity `C`,
/// find an assignment of items to bins such that:
/// - For each bin `j`, the total size of items assigned to `j` does not exceed `C`
/// - The number of bins used is minimized
///
/// # Representation
///
/// Each item has a variable in `{0, ..., n-1}` representing its bin assignment.
/// The worst case uses `n` bins (one item per bin).
///
/// # Type Parameters
///
/// * `W` - The weight type for sizes and capacity (e.g., `i32`, `f64`)
///
/// # Example
///
/// ```
/// use problemreductions::models::optimization::BinPacking;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // 4 items with sizes [3, 3, 2, 2], capacity 5
/// let problem = BinPacking::new(vec![3, 3, 2, 2], 5);
/// let solver = BruteForce::new();
/// let solution = solver.find_best(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinPacking<W> {
    /// Item sizes.
    sizes: Vec<W>,
    /// Bin capacity.
    capacity: W,
}

impl<W: Clone + Default> BinPacking<W> {
    /// Create a Bin Packing problem from item sizes and capacity.
    pub fn new(sizes: Vec<W>, capacity: W) -> Self {
        Self { sizes, capacity }
    }

    /// Get the item sizes.
    pub fn sizes(&self) -> &[W] {
        &self.sizes
    }

    /// Get the bin capacity.
    pub fn capacity(&self) -> &W {
        &self.capacity
    }

    /// Get the number of items.
    pub fn num_items(&self) -> usize {
        self.sizes.len()
    }
}

impl<W> Problem for BinPacking<W>
where
    W: WeightElement + crate::variant::VariantParam,
    W::Sum: PartialOrd,
{
    const NAME: &'static str = "BinPacking";
    type Metric = SolutionSize<i32>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![W]
    }

    fn dims(&self) -> Vec<usize> {
        let n = self.sizes.len();
        vec![n; n]
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<i32> {
        if !is_valid_packing(&self.sizes, &self.capacity, config) {
            return SolutionSize::Invalid;
        }
        let num_bins = count_bins(config);
        SolutionSize::Valid(num_bins as i32)
    }

    fn problem_size_names() -> &'static [&'static str] {
        &["num_items"]
    }
    fn problem_size_values(&self) -> Vec<usize> {
        vec![self.num_items()]
    }
}

impl<W> OptimizationProblem for BinPacking<W>
where
    W: WeightElement + crate::variant::VariantParam,
    W::Sum: PartialOrd,
{
    type Value = i32;

    fn direction(&self) -> Direction {
        Direction::Minimize
    }
}

/// Check if a configuration is a valid bin packing (all bins within capacity).
fn is_valid_packing<W: WeightElement>(sizes: &[W], capacity: &W, config: &[usize]) -> bool
where
    W::Sum: PartialOrd,
{
    if config.len() != sizes.len() {
        return false;
    }
    let n = sizes.len();
    // Check all bin indices are in range
    if config.iter().any(|&b| b >= n) {
        return false;
    }
    // Compute load per bin
    let cap_sum = capacity.to_sum();
    let mut bin_load: Vec<W::Sum> = vec![W::Sum::default(); n];
    for (i, &bin) in config.iter().enumerate() {
        bin_load[bin] += sizes[i].to_sum();
    }
    // Check capacity constraints
    bin_load.iter().all(|load| *load <= cap_sum)
}

/// Count the number of distinct bins used in a configuration.
fn count_bins(config: &[usize]) -> usize {
    let mut used = vec![false; config.len()];
    for &bin in config {
        if bin < used.len() {
            used[bin] = true;
        }
    }
    used.iter().filter(|&&u| u).count()
}

#[cfg(test)]
#[path = "../../unit_tests/models/optimization/bin_packing.rs"]
mod tests;
```

**Step 2: Register in `src/models/optimization/mod.rs`**

Add after the existing module declarations:
```rust
pub(crate) mod bin_packing;
```
Add to the public exports:
```rust
pub use bin_packing::BinPacking;
```

**Step 3: Register in `src/models/mod.rs`**

Add `BinPacking` to the `optimization` re-export line:
```rust
pub use optimization::{BinPacking, SpinGlass, ILP, QUBO};
```

**Step 4: Verify it compiles**

Run: `make build`
Expected: Compiles with no errors (tests will fail since test file doesn't exist yet).

**Step 5: Commit**

```bash
git add src/models/optimization/bin_packing.rs src/models/optimization/mod.rs src/models/mod.rs
git commit -m "feat: add BinPacking model (optimization, minimize bins)"
```

---

### Task 2: Write unit tests

**Files:**
- Create: `src/unit_tests/models/optimization/bin_packing.rs`

Ensure parent directory exists — check if `src/unit_tests/models/optimization/` already has files (it should, from QUBO/ILP/SpinGlass).

**Step 1: Create the test file**

Reference: `src/unit_tests/models/graph/maximum_independent_set.rs` and `src/unit_tests/models/set/maximum_set_packing.rs`.

```rust
use super::*;
use crate::solvers::BruteForce;
use crate::traits::{OptimizationProblem, Problem};
use crate::types::Direction;

#[test]
fn test_bin_packing_creation() {
    let problem = BinPacking::new(vec![6, 6, 5, 5, 4, 4], 10);
    assert_eq!(problem.num_items(), 6);
    assert_eq!(problem.sizes(), &[6, 6, 5, 5, 4, 4]);
    assert_eq!(*problem.capacity(), 10);
    assert_eq!(problem.dims().len(), 6);
    // Each variable has domain {0, ..., 5}
    assert!(problem.dims().iter().all(|&d| d == 6));
}

#[test]
fn test_bin_packing_direction() {
    let problem = BinPacking::new(vec![1, 2, 3], 5);
    assert_eq!(problem.direction(), Direction::Minimize);
}

#[test]
fn test_bin_packing_evaluate_valid() {
    // 6 items, capacity 10, sizes [6, 6, 5, 5, 4, 4]
    // Assignment: (0, 1, 2, 2, 0, 1) -> 3 bins
    // Bin 0: items 0,4 -> 6+4=10 OK
    // Bin 1: items 1,5 -> 6+4=10 OK
    // Bin 2: items 2,3 -> 5+5=10 OK
    let problem = BinPacking::new(vec![6, 6, 5, 5, 4, 4], 10);
    let result = problem.evaluate(&[0, 1, 2, 2, 0, 1]);
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 3);
}

#[test]
fn test_bin_packing_evaluate_invalid_overweight() {
    // Bin 0: items 0,1 -> 6+6=12 > 10
    let problem = BinPacking::new(vec![6, 6, 5, 5, 4, 4], 10);
    let result = problem.evaluate(&[0, 0, 1, 1, 2, 2]);
    assert!(!result.is_valid());
}

#[test]
fn test_bin_packing_evaluate_single_bin() {
    // All items fit in one bin
    let problem = BinPacking::new(vec![1, 2, 3], 10);
    let result = problem.evaluate(&[0, 0, 0]);
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 1);
}

#[test]
fn test_bin_packing_evaluate_all_separate() {
    // Each item in its own bin
    let problem = BinPacking::new(vec![3, 3, 3], 5);
    let result = problem.evaluate(&[0, 1, 2]);
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 3);
}

#[test]
fn test_bin_packing_problem_name() {
    assert_eq!(<BinPacking<i32> as Problem>::NAME, "BinPacking");
}

#[test]
fn test_bin_packing_brute_force_solver() {
    // 6 items, capacity 10, sizes [6, 6, 5, 5, 4, 4]
    // Optimal: 3 bins (lower bound ceil(30/10) = 3)
    let problem = BinPacking::new(vec![6, 6, 5, 5, 4, 4], 10);
    let solver = BruteForce::new();
    let solution = solver.find_best(&problem).expect("should find a solution");
    let metric = problem.evaluate(&solution);
    assert!(metric.is_valid());
    assert_eq!(metric.unwrap(), 3);
}

#[test]
fn test_bin_packing_brute_force_small() {
    // 3 items [3, 3, 4], capacity 7
    // Optimal: 2 bins (e.g., {3,4} + {3})
    let problem = BinPacking::new(vec![3, 3, 4], 7);
    let solver = BruteForce::new();
    let solution = solver.find_best(&problem).expect("should find a solution");
    let metric = problem.evaluate(&solution);
    assert!(metric.is_valid());
    assert_eq!(metric.unwrap(), 2);
}

#[test]
fn test_bin_packing_serialization() {
    let problem = BinPacking::new(vec![6, 6, 5, 5, 4, 4], 10);
    let json = serde_json::to_value(&problem).unwrap();
    let restored: BinPacking<i32> = serde_json::from_value(json).unwrap();
    assert_eq!(restored.sizes(), problem.sizes());
    assert_eq!(restored.capacity(), problem.capacity());
}
```

**Step 2: Check that the test directory exists**

Run: `ls src/unit_tests/models/optimization/`
If the directory doesn't exist, check existing test patterns under `src/unit_tests/models/` and ensure there's a `mod.rs` that includes `bin_packing`.

**Step 3: Run tests**

Run: `cargo test bin_packing -- --nocapture`
Expected: All tests PASS.

Note: The brute-force test with 6 items has search space 6^6 = 46656, which is tractable. If it's too slow, reduce to 4 items.

**Step 4: Commit**

```bash
git add src/unit_tests/models/optimization/bin_packing.rs
git commit -m "test: add BinPacking unit tests"
```

---

### Task 3: Register in CLI dispatch

**Files:**
- Modify: `problemreductions-cli/src/dispatch.rs`
- Modify: `problemreductions-cli/src/problem_name.rs`

**Step 1: Add match arms in `dispatch.rs`**

In `load_problem()` (around line 207), add before the `_ => bail!` fallthrough:
```rust
"BinPacking" => match variant.get("weight").map(|s| s.as_str()) {
    Some("f64") => deser_opt::<BinPacking<f64>>(data),
    _ => deser_opt::<BinPacking<i32>>(data),
},
```

In `serialize_any_problem()` (around line 257), add before the `_ => bail!` fallthrough:
```rust
"BinPacking" => match variant.get("weight").map(|s| s.as_str()) {
    Some("f64") => try_ser::<BinPacking<f64>>(any),
    _ => try_ser::<BinPacking<i32>>(any),
},
```

Add import at the top of `dispatch.rs` if not already covered by `prelude::*`:
```rust
use problemreductions::models::optimization::BinPacking;
```

**Step 2: Add alias in `problem_name.rs`**

In `resolve_alias()`, add:
```rust
"binpacking" => "BinPacking".to_string(),
```

Optionally add a short alias to the `ALIASES` array:
```rust
("BP", "BinPacking"),
```

**Step 3: Verify CLI builds**

Run: `make cli`
Expected: Builds successfully.

**Step 4: Commit**

```bash
git add problemreductions-cli/src/dispatch.rs problemreductions-cli/src/problem_name.rs
git commit -m "feat: register BinPacking in CLI dispatch"
```

---

### Task 4: Add problem definition to paper

**Files:**
- Modify: `docs/paper/reductions.typ`

**Step 1: Add to `display-name` dictionary**

Find the `display-name` dict (line ~28) and add:
```typst
"BinPacking": [Bin Packing],
```

**Step 2: Add `#problem-def` block**

Add after an appropriate location (e.g., after TravelingSalesman or at the end of the optimization section):
```typst
#problem-def("BinPacking")[
  Given $n$ items with sizes $s_1, dots, s_n in RR^+$ and bin capacity $C > 0$, find an assignment $x: {1, dots, n} -> {1, dots, n}$ minimizing $|{x(i) : i = 1, dots, n}|$ (number of distinct bins used) subject to $forall j: sum_(i: x(i) = j) s_i lt.eq C$.
]
```

**Step 3: Commit**

```bash
git add docs/paper/reductions.typ
git commit -m "docs: add BinPacking problem definition to paper"
```

---

### Task 5: Verify everything

**Step 1: Run full check**

Run: `make check`
Expected: fmt, clippy, and all tests pass.

**Step 2: Run review-implementation skill**

Use `/review-implementation` to verify structural and semantic completeness.

**Step 3: Final commit if any fixups needed**

---

## Summary of Files

| Action | File |
|--------|------|
| Create | `src/models/optimization/bin_packing.rs` |
| Create | `src/unit_tests/models/optimization/bin_packing.rs` |
| Modify | `src/models/optimization/mod.rs` |
| Modify | `src/models/mod.rs` |
| Modify | `problemreductions-cli/src/dispatch.rs` |
| Modify | `problemreductions-cli/src/problem_name.rs` |
| Modify | `docs/paper/reductions.typ` |

## Key Design Decisions

1. **Category:** `optimization/` — BinPacking is a core optimization problem.
2. **Type parameter:** `W` only (no graph). Follows `MaximumSetPacking<W>` pattern.
3. **Objective type:** `i32` always (bin count is integer), independent of `W`.
4. **Config space:** `vec![n; n]` — each of `n` items can be assigned to bins `0..n-1`.
5. **Feasibility:** Check per-bin load ≤ capacity. Out-of-range bin indices → invalid.
