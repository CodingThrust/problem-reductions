# Type System Cleanup Design

## Problem

The weight and trait system has several mathematical inconsistencies:

1. **Weight dual role**: The type parameter `W` serves as both the per-element weight type and the accumulation/metric type. This prevents using a unit-weight type (`One`) because `One + One` can't produce `2` within the same type.

2. **Dead abstractions**: `Unweighted(usize)` is never used as a type parameter. The `Weights` trait is implemented but never used outside its own tests. `NumericWeight` and `NumericSize` are nearly identical traits.

3. **Missing satisfaction trait**: Satisfaction problems (SAT, CircuitSAT, KColoring, Factoring) use `Metric = bool` but have no shared trait. The `BruteForce::find_satisfying()` method uses `Problem<Metric = bool>` inline.

## Design

### 1. `WeightElement` trait + `One` type

Introduce a trait that maps weight element types to their accumulation type:

```rust
/// Maps a weight element to its sum/metric type.
pub trait WeightElement: Clone + Default + 'static {
    /// The numeric type used for sums and comparisons.
    type Sum: NumericSize;
    /// Convert this weight element to the sum type.
    fn to_sum(&self) -> Self::Sum;
}
```

Implementations:

```rust
/// The constant 1. Unit weight for unweighted problems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct One;

impl WeightElement for One {
    type Sum = i32;
    fn to_sum(&self) -> i32 { 1 }
}

impl WeightElement for i32 {
    type Sum = i32;
    fn to_sum(&self) -> i32 { *self }
}

impl WeightElement for f64 {
    type Sum = f64;
    fn to_sum(&self) -> f64 { *self }
}
```

**Impact on problems:**

Before:
```rust
impl<G, W> Problem for MaximumIndependentSet<G, W>
where W: Clone + Default + PartialOrd + Num + Zero + AddAssign + 'static
{
    type Metric = SolutionSize<W>;
    fn evaluate(&self, config: &[usize]) -> SolutionSize<W> {
        let mut total = W::zero();
        for (i, &sel) in config.iter().enumerate() {
            if sel == 1 { total += self.weights[i].clone(); }
        }
        SolutionSize::Valid(total)
    }
}

impl<G, W> OptimizationProblem for MaximumIndependentSet<G, W> {
    type Value = W;
}
```

After:
```rust
impl<G, W: WeightElement> Problem for MaximumIndependentSet<G, W>
where W::Sum: PartialOrd
{
    type Metric = SolutionSize<W::Sum>;
    fn evaluate(&self, config: &[usize]) -> SolutionSize<W::Sum> {
        let mut total = W::Sum::zero();
        for (i, &sel) in config.iter().enumerate() {
            if sel == 1 { total += self.weights[i].to_sum(); }
        }
        SolutionSize::Valid(total)
    }
}

impl<G, W: WeightElement> OptimizationProblem for MaximumIndependentSet<G, W> {
    type Value = W::Sum;
}
```

**Variant output:** `variant()` uses `short_type_name::<W>()` which returns `"One"`, `"i32"`, or `"f64"`. The variant label changes from `"Unweighted"` to `"One"`.

### 2. `SatisfactionProblem` marker trait

```rust
/// Marker trait for satisfaction (decision) problems.
pub trait SatisfactionProblem: Problem<Metric = bool> {}
```

Implemented by: `Satisfiability`, `KSatisfiability`, `CircuitSAT`, `KColoring`, `Factoring`.

No new methods. Makes the problem category explicit in the type system. `BruteForce::find_satisfying()` can use `P: SatisfactionProblem` as its bound.

### 3. Merge `NumericWeight` / `NumericSize`

Delete `NumericWeight`. Keep `NumericSize` as the sole numeric bound trait:

```rust
pub trait NumericSize:
    Clone + Default + PartialOrd + Num + Zero + Bounded + AddAssign + 'static
{}
```

This is the bound on `WeightElement::Sum`. The extra `Bounded` requirement (vs the old `NumericWeight`) is needed for solver penalty calculations and is satisfied by `i32` and `f64`.

### Removals

- `Unweighted` struct (replaced by `One`)
- `Weights` trait (unused, subsumed by `WeightElement`)
- `NumericWeight` trait (merged into `NumericSize`)

### Reduction impact

Concrete `ReduceTo` impls change `Unweighted` references to `One`. The `ConcreteVariantEntry` registrations in `variants.rs` change `"Unweighted"` to `"One"`. The natural edge system (weight subtype hierarchy) adds `One` as a subtype of `i32`.

### Variant impact

The `variant()` output for unweighted problems changes from `("weight", "Unweighted")` to `("weight", "One")`. The reduction graph JSON, paper, and JavaScript visualization update accordingly.
