# Knapsack Model Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the Knapsack (0-1) problem model to the codebase as a new `misc/` category model.

**Architecture:** Knapsack is a maximization problem with binary variables (select/don't select each item). It has no graph or weight type parameters — all fields are concrete `i64`. The struct stores `weights`, `values`, and `capacity`. Feasibility requires total weight ≤ capacity; objective is total value. Solved via existing BruteForce solver.

**Tech Stack:** Rust, serde, inventory registration, `Problem`/`OptimizationProblem` traits.

**Reference:** Follow `add-model` skill Steps 1–7. Reference files: `src/models/misc/bin_packing.rs`, `src/unit_tests/models/misc/bin_packing.rs`.

---

### Task 1: Create the Knapsack model file

**Files:**
- Create: `src/models/misc/knapsack.rs`
- Modify: `src/models/misc/mod.rs`

**Step 1: Create `src/models/misc/knapsack.rs`**

```rust
//! Knapsack problem implementation.
//!
//! The 0-1 Knapsack problem asks for a subset of items that maximizes
//! total value while respecting a weight capacity constraint.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "Knapsack",
        module_path: module_path!(),
        description: "Select items to maximize total value subject to weight capacity constraint",
        fields: &[
            FieldInfo { name: "weights", type_name: "Vec<i64>", description: "Item weights w_i" },
            FieldInfo { name: "values", type_name: "Vec<i64>", description: "Item values v_i" },
            FieldInfo { name: "capacity", type_name: "i64", description: "Knapsack capacity C" },
        ],
    }
}

/// The 0-1 Knapsack problem.
///
/// Given `n` items, each with weight `w_i` and value `v_i`, and a capacity `C`,
/// find a subset `S ⊆ {0, ..., n-1}` such that `∑_{i∈S} w_i ≤ C`,
/// maximizing `∑_{i∈S} v_i`.
///
/// # Representation
///
/// Each item has a binary variable: `x_i = 1` if item `i` is selected, `0` otherwise.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::Knapsack;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// let problem = Knapsack::new(vec![2, 3, 4, 5], vec![3, 4, 5, 7], 7);
/// let solver = BruteForce::new();
/// let solution = solver.find_best(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Knapsack {
    weights: Vec<i64>,
    values: Vec<i64>,
    capacity: i64,
}

impl Knapsack {
    /// Create a new Knapsack instance.
    ///
    /// # Panics
    /// Panics if `weights` and `values` have different lengths.
    pub fn new(weights: Vec<i64>, values: Vec<i64>, capacity: i64) -> Self {
        assert_eq!(
            weights.len(),
            values.len(),
            "weights and values must have the same length"
        );
        Self {
            weights,
            values,
            capacity,
        }
    }

    /// Returns the item weights.
    pub fn weights(&self) -> &[i64] {
        &self.weights
    }

    /// Returns the item values.
    pub fn values(&self) -> &[i64] {
        &self.values
    }

    /// Returns the knapsack capacity.
    pub fn capacity(&self) -> i64 {
        self.capacity
    }

    /// Returns the number of items.
    pub fn num_items(&self) -> usize {
        self.weights.len()
    }
}

impl Problem for Knapsack {
    const NAME: &'static str = "Knapsack";
    type Metric = SolutionSize<i64>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.num_items()]
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<i64> {
        if config.len() != self.num_items() {
            return SolutionSize::Invalid;
        }
        if config.iter().any(|&v| v >= 2) {
            return SolutionSize::Invalid;
        }
        let total_weight: i64 = config
            .iter()
            .enumerate()
            .filter(|(_, &x)| x == 1)
            .map(|(i, _)| self.weights[i])
            .sum();
        if total_weight > self.capacity {
            return SolutionSize::Invalid;
        }
        let total_value: i64 = config
            .iter()
            .enumerate()
            .filter(|(_, &x)| x == 1)
            .map(|(i, _)| self.values[i])
            .sum();
        SolutionSize::Valid(total_value)
    }
}

impl OptimizationProblem for Knapsack {
    type Value = i64;

    fn direction(&self) -> Direction {
        Direction::Maximize
    }
}

crate::declare_variants! {
    Knapsack => "2^(num_items / 2)",
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/knapsack.rs"]
mod tests;
```

**Step 2: Register in `src/models/misc/mod.rs`**

Add `mod knapsack;` and `pub use knapsack::Knapsack;` following the existing pattern (alphabetical order).

**Step 3: Verify it compiles**

Run: `make build`
Expected: SUCCESS

**Step 4: Commit**

```bash
git add src/models/misc/knapsack.rs src/models/misc/mod.rs
git commit -m "feat: add Knapsack model struct and Problem impl"
```

---

### Task 2: Write unit tests for Knapsack

**Files:**
- Create: `src/unit_tests/models/misc/knapsack.rs`
- Modify: `src/unit_tests/models/misc/mod.rs` (if it exists)

**Step 1: Create `src/unit_tests/models/misc/knapsack.rs`**

```rust
use crate::models::misc::Knapsack;
use crate::solvers::BruteForce;
use crate::traits::{OptimizationProblem, Problem, Solver};
use crate::types::{Direction, SolutionSize};

#[test]
fn test_knapsack_basic() {
    let problem = Knapsack::new(vec![2, 3, 4, 5], vec![3, 4, 5, 7], 7);
    assert_eq!(problem.num_items(), 4);
    assert_eq!(problem.weights(), &[2, 3, 4, 5]);
    assert_eq!(problem.values(), &[3, 4, 5, 7]);
    assert_eq!(problem.capacity(), 7);
    assert_eq!(problem.dims(), vec![2; 4]);
    assert_eq!(problem.direction(), Direction::Maximize);
    assert_eq!(<Knapsack as Problem>::NAME, "Knapsack");
    assert_eq!(<Knapsack as Problem>::variant(), vec![]);
}

#[test]
fn test_knapsack_evaluate_optimal() {
    // Items: w=[2,3,4,5], v=[3,4,5,7], C=7
    // Select items 0 and 3: weight=2+5=7, value=3+7=10
    let problem = Knapsack::new(vec![2, 3, 4, 5], vec![3, 4, 5, 7], 7);
    assert_eq!(problem.evaluate(&[1, 0, 0, 1]), SolutionSize::Valid(10));
}

#[test]
fn test_knapsack_evaluate_feasible() {
    let problem = Knapsack::new(vec![2, 3, 4, 5], vec![3, 4, 5, 7], 7);
    // Select items 0 and 1: weight=2+3=5, value=3+4=7
    assert_eq!(problem.evaluate(&[1, 1, 0, 0]), SolutionSize::Valid(7));
}

#[test]
fn test_knapsack_evaluate_overweight() {
    let problem = Knapsack::new(vec![2, 3, 4, 5], vec![3, 4, 5, 7], 7);
    // Select items 2 and 3: weight=4+5=9 > 7
    assert_eq!(problem.evaluate(&[0, 0, 1, 1]), SolutionSize::Invalid);
}

#[test]
fn test_knapsack_evaluate_empty() {
    let problem = Knapsack::new(vec![2, 3, 4, 5], vec![3, 4, 5, 7], 7);
    // Select nothing: weight=0, value=0
    assert_eq!(problem.evaluate(&[0, 0, 0, 0]), SolutionSize::Valid(0));
}

#[test]
fn test_knapsack_evaluate_all_selected() {
    let problem = Knapsack::new(vec![1, 1, 1], vec![10, 20, 30], 5);
    // Select all: weight=3 <= 5, value=60
    assert_eq!(problem.evaluate(&[1, 1, 1]), SolutionSize::Valid(60));
}

#[test]
fn test_knapsack_evaluate_wrong_config_length() {
    let problem = Knapsack::new(vec![2, 3], vec![3, 4], 5);
    assert_eq!(problem.evaluate(&[1]), SolutionSize::Invalid);
    assert_eq!(problem.evaluate(&[1, 0, 0]), SolutionSize::Invalid);
}

#[test]
fn test_knapsack_evaluate_invalid_variable_value() {
    let problem = Knapsack::new(vec![2, 3], vec![3, 4], 5);
    assert_eq!(problem.evaluate(&[2, 0]), SolutionSize::Invalid);
}

#[test]
fn test_knapsack_empty_instance() {
    let problem = Knapsack::new(vec![], vec![], 10);
    assert_eq!(problem.num_items(), 0);
    assert_eq!(problem.dims(), Vec::<usize>::new());
    assert_eq!(problem.evaluate(&[]), SolutionSize::Valid(0));
}

#[test]
fn test_knapsack_brute_force() {
    let problem = Knapsack::new(vec![2, 3, 4, 5], vec![3, 4, 5, 7], 7);
    let solver = BruteForce::new();
    let solution = solver.find_best(&problem).expect("should find a solution");
    let metric = problem.evaluate(&solution);
    assert_eq!(metric, SolutionSize::Valid(10));
}

#[test]
fn test_knapsack_serialization() {
    let problem = Knapsack::new(vec![2, 3, 4, 5], vec![3, 4, 5, 7], 7);
    let json = serde_json::to_value(&problem).unwrap();
    let restored: Knapsack = serde_json::from_value(json).unwrap();
    assert_eq!(restored.weights(), problem.weights());
    assert_eq!(restored.values(), problem.values());
    assert_eq!(restored.capacity(), problem.capacity());
}

#[test]
#[should_panic(expected = "weights and values must have the same length")]
fn test_knapsack_mismatched_lengths() {
    Knapsack::new(vec![1, 2], vec![3], 5);
}
```

**Step 2: Register test module in `src/unit_tests/models/misc/mod.rs`**

Add `mod knapsack;` if the mod file exists and uses explicit module declarations.

**Step 3: Run tests**

Run: `make test`
Expected: All tests pass

**Step 4: Commit**

```bash
git add src/unit_tests/models/misc/knapsack.rs src/unit_tests/models/misc/mod.rs
git commit -m "test: add Knapsack unit tests"
```

---

### Task 3: Register Knapsack in CLI dispatch

**Files:**
- Modify: `problemreductions-cli/src/dispatch.rs`
- Modify: `problemreductions-cli/src/problem_name.rs`

**Step 1: Add import in `dispatch.rs`**

Add `use problemreductions::models::misc::Knapsack;` to the imports.

**Step 2: Add to `load_problem()` match**

```rust
"Knapsack" => deser_opt::<Knapsack>(data),
```

**Step 3: Add to `serialize_any_problem()` match**

```rust
"Knapsack" => try_ser::<Knapsack>(any),
```

**Step 4: Add CLI alias in `problem_name.rs`**

Add to the `ALIASES` array:
```rust
("KS", "Knapsack"),
```

Add to `resolve_alias()` match:
```rust
"ks" | "knapsack" => "Knapsack".to_string(),
```

**Step 5: Run CLI tests**

Run: `make cli && make cli-demo`
Expected: SUCCESS

**Step 6: Commit**

```bash
git add problemreductions-cli/src/dispatch.rs problemreductions-cli/src/problem_name.rs
git commit -m "feat: register Knapsack in CLI dispatch and aliases"
```

---

### Task 4: Add Knapsack to the paper

**Files:**
- Modify: `docs/paper/reductions.typ`

**Step 1: Add display name**

In the `display-name` dictionary, add:
```typst
"Knapsack": [Knapsack],
```

**Step 2: Add problem definition**

Use `#problem-def("Knapsack")[...]` with the formal definition from the issue. Include:
- Mathematical formulation (items, weights, values, capacity)
- Configuration variables (binary selection)
- Objective and constraint
- Background on Karp's 21 NP-complete problems, pseudo-polynomial DP, FPTAS

**Step 3: Verify paper builds**

Run: `make paper` (or just `make examples && make export-schemas` if paper build requires full toolchain)

**Step 4: Commit**

```bash
git add docs/paper/reductions.typ
git commit -m "docs: add Knapsack problem definition to paper"
```

---

### Task 5: Run full verification

**Step 1: Run formatting and linting**

Run: `make fmt && make clippy`

**Step 2: Run all tests**

Run: `make test`

**Step 3: Export schemas**

Run: `make export-schemas`

**Step 4: Run check**

Run: `make check`
Expected: All pass

**Step 5: Commit any generated files**

```bash
git add -A
git commit -m "chore: regenerate schemas after Knapsack addition"
```
