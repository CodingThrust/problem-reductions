# Type System Cleanup Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Clean up the weight type system by introducing `WeightElement` trait and `One` type, add `SatisfactionProblem` marker trait, and delete dead abstractions.

**Architecture:** Three independent changes: (1) `WeightElement` trait decouples weight element type from accumulation type, enabling `One` as a unit weight; (2) `SatisfactionProblem` marker trait for `Metric = bool` problems; (3) merge `NumericWeight` into `NumericSize`.

**Tech Stack:** Rust, inventory crate for registry

**Design doc:** `docs/plans/2026-02-14-type-system-cleanup-design.md`

---

### Task 1: Add `WeightElement` trait and `One` type to `types.rs`

**Files:**
- Modify: `src/types.rs`
- Test: `src/unit_tests/types.rs`

**Step 1: Add `WeightElement` trait and implementations after `NumericSize`**

Add after the `NumericSize` blanket impl (after line 51):

```rust
/// Maps a weight element to its sum/metric type.
///
/// This decouples the per-element weight type from the accumulation type.
/// For concrete weights (`i32`, `f64`), `Sum` is the same type.
/// For the unit weight `One`, `Sum = i32`.
pub trait WeightElement: Clone + Default + 'static {
    /// The numeric type used for sums and comparisons.
    type Sum: NumericSize;
    /// Convert this weight element to the sum type.
    fn to_sum(&self) -> Self::Sum;
}

impl WeightElement for i32 {
    type Sum = i32;
    fn to_sum(&self) -> i32 {
        *self
    }
}

impl WeightElement for f64 {
    type Sum = f64;
    fn to_sum(&self) -> f64 {
        *self
    }
}
```

**Step 2: Replace `Unweighted` with `One`**

Replace the `Unweighted` struct, its methods, Display impl, and Weights impl with:

```rust
/// The constant 1. Unit weight for unweighted problems.
///
/// When used as the weight type parameter `W`, indicates that all weights
/// are uniformly 1. `One::to_sum()` returns `1i32`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct One;

impl WeightElement for One {
    type Sum = i32;
    fn to_sum(&self) -> i32 {
        1
    }
}

impl std::fmt::Display for One {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "One")
    }
}
```

**Step 3: Delete dead code**

- Delete `NumericWeight` trait and blanket impl (lines 11-26)
- Delete `Weights` trait (lines 53-67)
- Delete `Weights for Unweighted` impl (lines 93-102)
- Delete `Weights for Vec<i32>` impl (lines 104-113)
- Delete `Weights for Vec<f64>` impl (lines 115-124)

**Step 4: Update `src/unit_tests/types.rs`**

- Replace `test_unweighted` to test `One` instead
- Replace `test_unweighted_weights_trait` to test `WeightElement for One`
- Add tests for `WeightElement for i32` and `WeightElement for f64`

**Step 5: Run test to verify**

Run: `cargo test --lib types::tests`

**Step 6: Commit**

```
feat: add WeightElement trait and One type, remove Unweighted/Weights/NumericWeight
```

---

### Task 2: Update `lib.rs` exports

**Files:**
- Modify: `src/lib.rs`

**Step 1: Update prelude and crate-level re-exports**

In the `pub mod prelude` block (line 102), replace:
```rust
Direction, NumericSize, NumericWeight, ProblemSize, SolutionSize, Unweighted, Weights,
```
with:
```rust
Direction, NumericSize, One, ProblemSize, SolutionSize, WeightElement,
```

**Step 2: Run build to check**

Run: `cargo check --all-features`

**Step 3: Commit**

```
refactor: update lib.rs exports for WeightElement/One
```

---

### Task 3: Update graph problem `Problem` and `OptimizationProblem` impls

**Files (8 graph problems):**
- Modify: `src/models/graph/maximum_independent_set.rs`
- Modify: `src/models/graph/maximum_clique.rs`
- Modify: `src/models/graph/minimum_vertex_cover.rs`
- Modify: `src/models/graph/minimum_dominating_set.rs`
- Modify: `src/models/graph/maximal_is.rs`
- Modify: `src/models/graph/maximum_matching.rs`
- Modify: `src/models/graph/traveling_salesman.rs`
- Modify: `src/models/graph/max_cut.rs`

For each file, apply the same pattern:

**Step 1: Update `Problem` impl**

Change trait bounds from:
```rust
impl<G, W> Problem for ProblemType<G, W>
where
    G: Graph,
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign + 'static,
```
to:
```rust
impl<G, W> Problem for ProblemType<G, W>
where
    G: Graph,
    W: WeightElement,
```

Change `type Metric = SolutionSize<W>` to `type Metric = SolutionSize<W::Sum>`.

Change `evaluate()` body: replace `W::zero()` with `W::Sum::zero()`, replace `self.weights[i].clone()` with `self.weights[i].to_sum()` (in the accumulation `total +=` line).

**Step 2: Update `OptimizationProblem` impl**

Same bound changes. Change `type Value = W` to `type Value = W::Sum`.

**Step 3: Update constructors and helper methods**

In `new()` constructors that create default weights: `vec![W::from(1); n]` stays as-is since `One::default()` doesn't produce 1 — but actually these constructors are bounded on `W: From<i32>`, and `One` doesn't implement `From<i32>`. Instead, `new()` should use `W::default()` for `One` (but default of `One` is `One`, which is correct). Check each constructor individually — most use `W::from(1)` which needs `From<i32>` bound. Since `One` is constructed as the default and `from(1)` makes no sense for `One`, the `new()` constructor that creates unit weights should be specialized or use a `WeightElement`-specific helper.

**Alternative:** Add `From<i32> for One`:
```rust
impl From<i32> for One {
    fn from(_: i32) -> Self { One }
}
```
This allows `W::from(1)` to work for `One` — it ignores the value and returns `One`. This is mathematically sound: promoting any integer to the `One` type gives `One`.

Add this to `types.rs` in Task 1.

**Step 4: Update `ReductionResult` impls in the same files**

`ReductionResult` impls with generic `W` bounds need the same bound change from the long trait list to `W: WeightElement`.

**Step 5: Run tests**

Run: `cargo test --lib models::graph`

**Step 6: Commit**

```
refactor: update graph problem impls to use WeightElement
```

---

### Task 4: Update set and optimization problem impls

**Files (4 problems):**
- Modify: `src/models/set/maximum_set_packing.rs`
- Modify: `src/models/set/minimum_set_covering.rs`
- Modify: `src/models/optimization/qubo.rs`
- Modify: `src/models/optimization/spin_glass.rs`

Same pattern as Task 3: update `Problem` bounds, `Metric`, `Value`, and `evaluate()` body.

For `QUBO<W>` and `SpinGlass<G, W>`, the `W` parameter is already the numeric type (not a weight element in the vertex sense), so `WeightElement for f64` with `Sum = f64` should work directly. Verify that `W::zero()` still works via `NumericSize` bound on `W::Sum`.

**Step 1: Apply same changes as Task 3**

**Step 2: Run tests**

Run: `cargo test --lib models::set models::optimization`

**Step 3: Commit**

```
refactor: update set and optimization problem impls to use WeightElement
```

---

### Task 5: Update reduction rule files

**Files (~20 reduction files):**
- All files in `src/rules/` that have generic `W` bounds on `ReductionResult` impls

The concrete `ReduceTo` impls (from our previous work) don't need changes since they use `i32`/`f64` directly. But the generic `ReductionResult` impls need bounds updated.

**Step 1: For each reduction file with generic `ReductionResult` impls**

Replace:
```rust
where W: Clone + Default + PartialOrd + Num + Zero + Bounded + AddAssign + 'static
```
or similar long bound lists with:
```rust
where W: WeightElement
```

Add `use crate::types::WeightElement;` to imports if not already present. Remove unused `num_traits` imports.

**Step 2: Run tests**

Run: `cargo test --all-features`

**Step 3: Commit**

```
refactor: update reduction rule bounds to use WeightElement
```

---

### Task 6: Update variant metadata — `"Unweighted"` to `"One"`

**Files:**
- Modify: `src/rules/variants.rs` — replace all `"Unweighted"` with `"One"`
- Modify: `src/graph_types.rs` — replace `"Unweighted"` in weight subtype declarations
- Modify: `src/rules/registry.rs` — replace `"Unweighted"` in weight checking (if any)
- Modify: `docs/src/reductions/reduction_graph.json` — regenerated
- Modify: test files that assert on `"Unweighted"` string

**Step 1: Replace `"Unweighted"` with `"One"` in source files**

In `variants.rs`, `graph_types.rs`, and `registry.rs`.

**Step 2: Update test assertions**

In `unit_tests/rules/graph.rs`, `unit_tests/rules/registry.rs`, `unit_tests/graph_types.rs` — replace all `"Unweighted"` assertions with `"One"`.

**Step 3: Regenerate reduction graph JSON**

Run: `make rust-export`

**Step 4: Run tests**

Run: `cargo test --all-features`

**Step 5: Commit**

```
refactor: rename Unweighted to One in variant metadata
```

---

### Task 7: Add `SatisfactionProblem` marker trait

**Files:**
- Modify: `src/traits.rs`
- Modify: `src/models/satisfiability/sat.rs`
- Modify: `src/models/satisfiability/ksat.rs`
- Modify: `src/models/specialized/circuit.rs`
- Modify: `src/models/graph/kcoloring.rs`
- Modify: `src/lib.rs` (re-export)

**Step 1: Add trait to `src/traits.rs`**

After the `OptimizationProblem` trait:
```rust
/// Marker trait for satisfaction (decision) problems.
///
/// Satisfaction problems evaluate configurations to `bool`:
/// `true` if the configuration satisfies all constraints, `false` otherwise.
pub trait SatisfactionProblem: Problem<Metric = bool> {}
```

**Step 2: Implement for each satisfaction problem**

In each file, add after the `Problem` impl:
```rust
impl SatisfactionProblem for Satisfiability {}
impl<const K: usize> SatisfactionProblem for KSatisfiability<K> {}
impl SatisfactionProblem for CircuitSAT {}
impl<const K: usize, G: Graph> SatisfactionProblem for KColoring<K, G> {}
```

**Step 3: Add re-export in `lib.rs`**

Add `SatisfactionProblem` to the traits re-export.

**Step 4: Optionally update solver bounds**

In `src/solvers/brute_force.rs` and `src/solvers/mod.rs`, change `P: Problem<Metric = bool>` to `P: SatisfactionProblem`. This is optional — the existing bound still works.

**Step 5: Run tests**

Run: `cargo test --all-features`

**Step 6: Commit**

```
feat: add SatisfactionProblem marker trait
```

---

### Task 8: Final verification and cleanup

**Step 1: Run full test suite**

Run: `make test clippy`

**Step 2: Check for any remaining `Unweighted` or `NumericWeight` references**

Run: `rg "Unweighted|NumericWeight" src/`

Any remaining references should be in comments/docs only — update those too.

**Step 3: Update paper if needed**

Check `docs/paper/reductions.typ` for `Unweighted` references.

**Step 4: Run doc build**

Run: `make doc`

**Step 5: Final commit**

```
chore: cleanup remaining Unweighted/NumericWeight references
```
