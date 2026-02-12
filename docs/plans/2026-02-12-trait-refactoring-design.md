# Trait System Refactoring Design

**Goal:** Simplify types and interfaces to lower the barrier for contributors.

**Approach:** Trait system redesign (Approach B) — addresses root causes of complexity without hiding them behind macros.

## 1. `NumericSize` Bound

Replace the repeated `where W: Clone + Default + PartialOrd + Num + Zero + AddAssign + 'static` with a single supertrait. Eliminates 15+ copy-pasted bound lists.

```rust
pub trait NumericSize:
    Clone + Default + PartialOrd + num_traits::Num + num_traits::Zero
    + std::ops::AddAssign + 'static
{}

// Blanket impl: any type meeting the bounds is automatically NumericSize.
impl<T> NumericSize for T
where
    T: Clone + Default + PartialOrd + num_traits::Num + num_traits::Zero
       + std::ops::AddAssign + 'static,
{}
```

Problems needing extra bounds add them locally: `W: Weights` where `W::Size: NumericSize + Mul<Output = W::Size>`.

## 2. `Weights` Trait

Replaces the current weight type parameter `W`. Separates two concepts that were conflated:
- **Weight storage** — how weights are stored (`Unweighted`, `Vec<i32>`, `Vec<f64>`)
- **Objective value type** — what type the metric is (`i32`, `f64`)

```rust
pub trait Weights: Clone + 'static {
    const NAME: &'static str;
    type Size: NumericSize;
    fn weight(&self, index: usize) -> Self::Size;
    fn len(&self) -> usize;
}
```

### Implementations

**`Unweighted`** — zero-data storage, every element has unit weight:

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Unweighted(pub usize);  // stores only the count

impl Weights for Unweighted {
    const NAME: &'static str = "Unweighted";
    type Size = i32;
    fn weight(&self, _index: usize) -> i32 { 1 }
    fn len(&self) -> usize { self.0 }
}
```

**`Vec<i32>` and `Vec<f64>`** — concrete weighted storage:

```rust
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

### Type-level distinction

The type reflects whether a problem is weighted:
- `MaximumIndependentSet<SimpleGraph, Unweighted>` — unweighted
- `MaximumIndependentSet<SimpleGraph, Vec<i32>>` — weighted with integers
- `MaximumIndependentSet<SimpleGraph, Vec<f64>>` — weighted with floats

Constructors make this ergonomic:
```rust
let mis = MaximumIndependentSet::new(graph);                        // -> MIS<_, Unweighted>
let mis = MaximumIndependentSet::with_weights(graph, vec![3, 1, 4]); // -> MIS<_, Vec<i32>>
```

## 3. `Problem` Trait (Minimal Function-like Object)

A problem is a function from configuration to metric. Two methods + one constant:

```rust
pub trait Problem: Clone {
    const NAME: &'static str;
    type Metric: Clone;

    /// Configuration space dimensions. Each entry is the cardinality
    /// of that variable (e.g., [2, 2, 2] = 3 binary variables).
    fn dims(&self) -> Vec<usize>;

    /// Evaluate the problem on a configuration.
    fn evaluate(&self, config: &[usize]) -> Self::Metric;
}
```

`num_variables()` is derived: `self.dims().len()`.

### `OptimizationProblem` extension

Optimization problems add a direction (maximize or minimize):

```rust
pub trait OptimizationProblem: Problem
where
    Self::Metric: NumericSize,
{
    fn direction(&self) -> Direction;
}

pub enum Direction {
    Maximize,
    Minimize,
}
```

### How problems implement this

**Satisfaction problems** — metric is `bool`:
```rust
impl<W: Weights> Problem for Satisfiability<W> {
    type Metric = bool;
    fn evaluate(&self, config: &[usize]) -> bool {
        self.clauses.iter().all(|c| c.is_satisfied(config))
    }
}
```

**Optimization problems** — metric is numeric, invalid configs return worst value:
```rust
impl<G: Graph, W: Weights> Problem for MaximumIndependentSet<G, W> {
    type Metric = W::Size;
    fn evaluate(&self, config: &[usize]) -> W::Size {
        if !self.is_independent(config) {
            return f64::NEG_INFINITY;  // not favored by maximize
        }
        self.total_weight(config)
    }
}
```

**All-valid problems** — every config is feasible:
```rust
impl<W: Weights> Problem for QUBO<W> {
    type Metric = W::Size;
    fn evaluate(&self, config: &[usize]) -> W::Size {
        self.compute_energy(config)
    }
}
```

### Problem categorization

| Problem | `Metric` | `OptimizationProblem` | Invalid handling |
|---------|----------|----------------------|-----------------|
| SAT | `bool` | No | N/A (all configs valid) |
| KColoring | `bool` | No | N/A (all configs valid) |
| MIS | `W::Size` | Yes (Maximize) | `-inf` |
| VertexCover | `W::Size` | Yes (Minimize) | `+inf` |
| QUBO | `W::Size` | Yes (Minimize) | N/A (all configs valid) |
| SpinGlass | `W::Size` | Yes (Minimize) | N/A (all configs valid) |
| MaxCut | `W::Size` | Yes (Maximize) | N/A (all configs valid) |
| MAX-SAT | `W::Size` | Yes (Maximize) | N/A (all configs valid) |

## 4. Standardized Type Parameters

| Category | Pattern | Example |
|----------|---------|---------|
| Graph + weighted | `<G: Graph, W: Weights>` | `MaximumIndependentSet<G, W>` |
| Non-graph + weighted | `<W: Weights>` | `QUBO<W>`, `Satisfiability<W>` |
| Decision (no weight) | `<G: Graph>` | `KColoring<G>` (k is runtime field) |

KColoring's const generic `K` becomes a runtime field `k: usize`.

## 5. `ReductionResult` and `ReduceTo` (Simplified)

```rust
pub trait ReductionResult: Clone {
    type Source: Problem;
    type Target: Problem;

    /// The reduced problem instance.
    fn target_problem(&self) -> &Self::Target;

    /// Map a target solution back to a source solution.
    fn extract_solution(&self, target_config: &[usize]) -> Vec<usize>;
}

pub trait ReduceTo<T: Problem>: Problem {
    type Result: ReductionResult<Source = Self, Target = T>;
    fn reduce_to(&self) -> Self::Result;
}
```

Removed `source_size()` and `target_size()`. Overhead is tracked in the `#[reduction]` macro attribute. Instance sizes available via `dims()`.

## 6. `#[reduction]` Macro (Trait-bound Extraction)

The macro identifies graph/weight types by inspecting **trait bounds**, not parameter positions:
- `G: Graph` bound → graph type, uses `Graph::NAME`
- `W: Weights` bound → weights type, uses `Weights::NAME`
- Source/target names: extracted from type signature (`ReduceTo<Target> for Source`)

No heuristics, no hardcoded type name lists, no silent fallbacks.

```rust
#[reduction(overhead = { ... })]
impl<G: Graph, W: Weights> ReduceTo<MinimumVertexCover<G, W>>
    for MaximumIndependentSet<G, W>
{
    type Result = ReductionMISToVC<G, W>;
    fn reduce_to(&self) -> Self::Result { ... }
}
```

Only the `overhead` attribute is required. Everything else is derived from types.

Variant IDs are constructed in the registry from `Graph::NAME` and `Weights::NAME`:
```
"MaximumIndependentSet"                    // SimpleGraph + Unweighted (defaults)
"MaximumIndependentSet/GridGraph"           // non-default graph
"MaximumIndependentSet/Weighted"            // non-default weight
"MaximumIndependentSet/GridGraph/Weighted"   // both non-default
```

## 7. What's Removed

| Removed | Replaced by |
|---------|------------|
| `Unweighted` marker struct | `Unweighted(usize)` real weight vector |
| `EnergyMode` enum | `Direction` on `OptimizationProblem` |
| `SolutionSize<T>` struct | `evaluate()` return value directly |
| `ConstraintSatisfactionProblem` trait | Removed entirely |
| `variant()` method | Derived from `Graph::NAME` + `Weights::NAME` |
| `solution_size()` | `evaluate()` |
| `is_valid()` | Folded into `evaluate()` (returns -inf/+inf) |
| `num_flavors()` | `dims()` (per-variable) |
| `num_variables()` | `dims().len()` |
| `problem_size()` on core trait | Removed |
| `set_weights()` / `is_weighted()` | Removed |
| `source_size()` / `target_size()` on `ReductionResult` | Removed, use `dims()` |
| Hardcoded weight type list in macro | Trait-bound inspection |
| Position-based type param inference in macro | Trait-bound inspection |

## 8. Contributor Experience After Refactoring

### Adding a new problem (2 methods + 1 constant)

```rust
pub struct MyProblem<G: Graph, W: Weights> {
    graph: G,
    weights: W,
}

impl<G: Graph, W: Weights> Problem for MyProblem<G, W> {
    const NAME: &'static str = "MyProblem";
    type Metric = W::Size;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_vertices()]
    }

    fn evaluate(&self, config: &[usize]) -> W::Size {
        // compute objective, return -inf for invalid if maximizing
    }
}

impl<G: Graph, W: Weights> OptimizationProblem for MyProblem<G, W> {
    fn direction(&self) -> Direction { Direction::Maximize }
}
```

### Adding a new reduction (2 methods)

```rust
#[derive(Clone)]
pub struct ReductionAToB<W: Weights> {
    target: ProblemB<W>,
}

impl<W: Weights> ReductionResult for ReductionAToB<W> {
    type Source = ProblemA<W>;
    type Target = ProblemB<W>;
    fn target_problem(&self) -> &Self::Target { &self.target }
    fn extract_solution(&self, target_config: &[usize]) -> Vec<usize> { /* ... */ }
}

#[reduction(overhead = { ... })]
impl<W: Weights> ReduceTo<ProblemB<W>> for ProblemA<W> {
    type Result = ReductionAToB<W>;
    fn reduce_to(&self) -> Self::Result { /* ... */ }
}
```
