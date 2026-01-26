# Set-Theoretic Reduction Path Finding - Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement parametric problem modeling with set-theoretic reduction path finding, automatic registration, and customizable cost functions.

**Architecture:** Problems carry `<G: GraphMarker, W: NumericWeight>` type parameters. Reductions are auto-registered via `inventory` crate with polynomial overhead metadata. Path finding uses Dijkstra with set-theoretic validation (A ⊆ C, D ⊆ B) and user-defined cost functions.

**Tech Stack:** Rust, petgraph, inventory, ordered-float

**Reference:** See `docs/plans/2026-01-26-set-theoretic-reductions-design.md` for full design rationale.

---

## Phase 1: Foundation Types

### Task 1.1: Add Dependencies

**Files:**
- Modify: `Cargo.toml`

**Step 1: Add inventory and ordered-float dependencies**

```toml
[dependencies]
# ... existing deps ...
inventory = "0.3"
ordered-float = "4.0"
```

**Step 2: Run cargo check**

```bash
cargo check
```

Expected: Compiles with new dependencies

**Step 3: Commit**

```bash
git add Cargo.toml
git commit -m "deps: Add inventory and ordered-float crates"
```

---

### Task 1.2: Create Graph Marker Traits

**Files:**
- Create: `src/graph_types.rs`
- Modify: `src/lib.rs`

**Step 1: Write the failing test**

Create `src/graph_types.rs`:

```rust
//! Graph type markers for parametric problem modeling.

/// Marker trait for graph types.
pub trait GraphMarker: 'static + Clone + Send + Sync {
    /// The name of this graph type for runtime queries.
    const NAME: &'static str;
}

/// Compile-time subtype relationship between graph types.
pub trait GraphSubtype<G: GraphMarker>: GraphMarker {}

// Reflexive: every type is a subtype of itself
impl<G: GraphMarker> GraphSubtype<G> for G {}

/// Simple (arbitrary) graph - the most general graph type.
#[derive(Debug, Clone, Copy, Default)]
pub struct SimpleGraph;

impl GraphMarker for SimpleGraph {
    const NAME: &'static str = "SimpleGraph";
}

/// Planar graph - can be drawn on a plane without edge crossings.
#[derive(Debug, Clone, Copy, Default)]
pub struct PlanarGraph;

impl GraphMarker for PlanarGraph {
    const NAME: &'static str = "PlanarGraph";
}

/// Unit disk graph - vertices are points, edges connect points within unit distance.
#[derive(Debug, Clone, Copy, Default)]
pub struct UnitDiskGraph;

impl GraphMarker for UnitDiskGraph {
    const NAME: &'static str = "UnitDiskGraph";
}

/// Bipartite graph - vertices can be partitioned into two sets with edges only between sets.
#[derive(Debug, Clone, Copy, Default)]
pub struct BipartiteGraph;

impl GraphMarker for BipartiteGraph {
    const NAME: &'static str = "BipartiteGraph";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_marker_names() {
        assert_eq!(SimpleGraph::NAME, "SimpleGraph");
        assert_eq!(PlanarGraph::NAME, "PlanarGraph");
        assert_eq!(UnitDiskGraph::NAME, "UnitDiskGraph");
        assert_eq!(BipartiteGraph::NAME, "BipartiteGraph");
    }

    #[test]
    fn test_reflexive_subtype() {
        fn assert_subtype<A: GraphSubtype<B>, B: GraphMarker>() {}

        // Every type is a subtype of itself
        assert_subtype::<SimpleGraph, SimpleGraph>();
        assert_subtype::<PlanarGraph, PlanarGraph>();
        assert_subtype::<UnitDiskGraph, UnitDiskGraph>();
    }
}
```

**Step 2: Run test to verify it compiles**

```bash
cargo test graph_types --lib
```

Expected: PASS

**Step 3: Add module to lib.rs**

In `src/lib.rs`, add:

```rust
pub mod graph_types;
```

**Step 4: Run tests**

```bash
cargo test graph_types --lib
```

Expected: PASS

**Step 5: Commit**

```bash
git add src/graph_types.rs src/lib.rs
git commit -m "feat: Add graph marker traits for parametric problems"
```

---

### Task 1.3: Register Graph Subtype Hierarchy

**Files:**
- Modify: `src/graph_types.rs`

**Step 1: Add inventory-based registration**

Add to `src/graph_types.rs`:

```rust
use inventory;

/// Runtime registration of graph subtype relationships.
pub struct GraphSubtypeEntry {
    pub subtype: &'static str,
    pub supertype: &'static str,
}

inventory::collect!(GraphSubtypeEntry);

/// Macro to declare both compile-time trait and runtime registration.
#[macro_export]
macro_rules! declare_graph_subtype {
    ($sub:ty => $sup:ty) => {
        impl $crate::graph_types::GraphSubtype<$sup> for $sub {}

        ::inventory::submit! {
            $crate::graph_types::GraphSubtypeEntry {
                subtype: <$sub as $crate::graph_types::GraphMarker>::NAME,
                supertype: <$sup as $crate::graph_types::GraphMarker>::NAME,
            }
        }
    };
}

// Declare the graph type hierarchy
declare_graph_subtype!(UnitDiskGraph => PlanarGraph);
declare_graph_subtype!(UnitDiskGraph => SimpleGraph);
declare_graph_subtype!(PlanarGraph => SimpleGraph);
declare_graph_subtype!(BipartiteGraph => SimpleGraph);
```

**Step 2: Add test for runtime hierarchy**

```rust
#[test]
fn test_subtype_entries_registered() {
    let entries: Vec<_> = inventory::iter::<GraphSubtypeEntry>.collect();

    // Should have at least 4 entries
    assert!(entries.len() >= 4);

    // Check specific relationships
    assert!(entries.iter().any(|e|
        e.subtype == "UnitDiskGraph" && e.supertype == "SimpleGraph"
    ));
    assert!(entries.iter().any(|e|
        e.subtype == "PlanarGraph" && e.supertype == "SimpleGraph"
    ));
}
```

**Step 3: Run tests**

```bash
cargo test graph_types --lib
```

Expected: PASS

**Step 4: Commit**

```bash
git add src/graph_types.rs
git commit -m "feat: Add inventory-based graph subtype registration"
```

---

### Task 1.4: Create NumericWeight Trait

**Files:**
- Modify: `src/types.rs`

**Step 1: Add NumericWeight trait**

Add to `src/types.rs`:

```rust
/// Marker trait for numeric weight types.
///
/// Weight subsumption uses Rust's `From` trait:
/// - `i32 → f64` is valid (From<i32> for f64 exists)
/// - `f64 → i32` is invalid (no lossless conversion)
pub trait NumericWeight: Clone + Default + PartialOrd + num_traits::Num + num_traits::Zero + std::ops::AddAssign + 'static {}

impl NumericWeight for bool {}
impl NumericWeight for i8 {}
impl NumericWeight for i16 {}
impl NumericWeight for i32 {}
impl NumericWeight for i64 {}
impl NumericWeight for f32 {}
impl NumericWeight for f64 {}
```

**Step 2: Add test**

```rust
#[test]
fn test_numeric_weight_impls() {
    fn assert_numeric_weight<T: NumericWeight>() {}

    assert_numeric_weight::<i32>();
    assert_numeric_weight::<f64>();
    assert_numeric_weight::<i64>();
}
```

**Step 3: Run tests**

```bash
cargo test numeric_weight --lib
```

Expected: PASS

**Step 4: Export in prelude**

In `src/lib.rs` prelude, add:

```rust
pub use crate::types::NumericWeight;
```

**Step 5: Commit**

```bash
git add src/types.rs src/lib.rs
git commit -m "feat: Add NumericWeight marker trait"
```

---

### Task 1.5: Create Polynomial Type

**Files:**
- Create: `src/polynomial.rs`
- Modify: `src/lib.rs`

**Step 1: Create polynomial module**

Create `src/polynomial.rs`:

```rust
//! Polynomial representation for reduction overhead.

use crate::types::ProblemSize;

/// A monomial: coefficient × Π(variable^exponent)
#[derive(Clone, Debug, PartialEq)]
pub struct Monomial {
    pub coefficient: f64,
    pub variables: Vec<(&'static str, u8)>,
}

impl Monomial {
    pub fn constant(c: f64) -> Self {
        Self { coefficient: c, variables: vec![] }
    }

    pub fn var(name: &'static str) -> Self {
        Self { coefficient: 1.0, variables: vec![(name, 1)] }
    }

    pub fn var_pow(name: &'static str, exp: u8) -> Self {
        Self { coefficient: 1.0, variables: vec![(name, exp)] }
    }

    pub fn scale(mut self, c: f64) -> Self {
        self.coefficient *= c;
        self
    }

    pub fn evaluate(&self, size: &ProblemSize) -> f64 {
        let var_product: f64 = self.variables.iter()
            .map(|(name, exp)| {
                let val = size.get(name).unwrap_or(0) as f64;
                val.powi(*exp as i32)
            })
            .product();
        self.coefficient * var_product
    }
}

/// A polynomial: Σ monomials
#[derive(Clone, Debug, PartialEq)]
pub struct Polynomial {
    pub terms: Vec<Monomial>,
}

impl Polynomial {
    pub fn zero() -> Self {
        Self { terms: vec![] }
    }

    pub fn constant(c: f64) -> Self {
        Self { terms: vec![Monomial::constant(c)] }
    }

    pub fn var(name: &'static str) -> Self {
        Self { terms: vec![Monomial::var(name)] }
    }

    pub fn var_pow(name: &'static str, exp: u8) -> Self {
        Self { terms: vec![Monomial::var_pow(name, exp)] }
    }

    pub fn scale(mut self, c: f64) -> Self {
        for term in &mut self.terms {
            term.coefficient *= c;
        }
        self
    }

    pub fn add(mut self, other: Self) -> Self {
        self.terms.extend(other.terms);
        self
    }

    pub fn evaluate(&self, size: &ProblemSize) -> f64 {
        self.terms.iter().map(|m| m.evaluate(size)).sum()
    }
}

/// Convenience macro for building polynomials.
#[macro_export]
macro_rules! poly {
    // Single variable: poly!(n)
    ($name:ident) => {
        $crate::polynomial::Polynomial::var(stringify!($name))
    };
    // Variable with exponent: poly!(n^2)
    ($name:ident ^ $exp:literal) => {
        $crate::polynomial::Polynomial::var_pow(stringify!($name), $exp)
    };
    // Constant: poly!(5)
    ($c:literal) => {
        $crate::polynomial::Polynomial::constant($c as f64)
    };
    // Scaled variable: poly!(3 * n)
    ($c:literal * $name:ident) => {
        $crate::polynomial::Polynomial::var(stringify!($name)).scale($c as f64)
    };
    // Scaled variable with exponent: poly!(9 * n^2)
    ($c:literal * $name:ident ^ $exp:literal) => {
        $crate::polynomial::Polynomial::var_pow(stringify!($name), $exp).scale($c as f64)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monomial_constant() {
        let m = Monomial::constant(5.0);
        let size = ProblemSize::new(vec![("n", 10)]);
        assert_eq!(m.evaluate(&size), 5.0);
    }

    #[test]
    fn test_monomial_variable() {
        let m = Monomial::var("n");
        let size = ProblemSize::new(vec![("n", 10)]);
        assert_eq!(m.evaluate(&size), 10.0);
    }

    #[test]
    fn test_monomial_var_pow() {
        let m = Monomial::var_pow("n", 2);
        let size = ProblemSize::new(vec![("n", 5)]);
        assert_eq!(m.evaluate(&size), 25.0);
    }

    #[test]
    fn test_polynomial_add() {
        // 3n + 2m
        let p = Polynomial::var("n").scale(3.0)
            .add(Polynomial::var("m").scale(2.0));

        let size = ProblemSize::new(vec![("n", 10), ("m", 5)]);
        assert_eq!(p.evaluate(&size), 40.0);  // 3*10 + 2*5
    }

    #[test]
    fn test_polynomial_complex() {
        // n^2 + 3m
        let p = Polynomial::var_pow("n", 2)
            .add(Polynomial::var("m").scale(3.0));

        let size = ProblemSize::new(vec![("n", 4), ("m", 2)]);
        assert_eq!(p.evaluate(&size), 22.0);  // 16 + 6
    }

    #[test]
    fn test_poly_macro() {
        let size = ProblemSize::new(vec![("n", 5), ("m", 3)]);

        assert_eq!(poly!(n).evaluate(&size), 5.0);
        assert_eq!(poly!(n^2).evaluate(&size), 25.0);
        assert_eq!(poly!(3 * n).evaluate(&size), 15.0);
        assert_eq!(poly!(2 * m^2).evaluate(&size), 18.0);
    }

    #[test]
    fn test_missing_variable() {
        let p = Polynomial::var("missing");
        let size = ProblemSize::new(vec![("n", 10)]);
        assert_eq!(p.evaluate(&size), 0.0);  // missing var = 0
    }
}
```

**Step 2: Add module to lib.rs**

```rust
pub mod polynomial;
```

**Step 3: Run tests**

```bash
cargo test polynomial --lib
```

Expected: PASS

**Step 4: Commit**

```bash
git add src/polynomial.rs src/lib.rs
git commit -m "feat: Add Polynomial type for reduction overhead"
```

---

## Phase 2: Reduction Registration

### Task 2.1: Create ReductionEntry and Registration

**Files:**
- Create: `src/rules/registry.rs`
- Modify: `src/rules/mod.rs`

**Step 1: Create registry module**

Create `src/rules/registry.rs`:

```rust
//! Automatic reduction registration via inventory.

use crate::polynomial::Polynomial;
use inventory;

/// Overhead specification for a reduction.
#[derive(Clone, Debug)]
pub struct ReductionOverhead {
    /// Output size as polynomials of input size variables.
    /// Each entry is (output_field_name, polynomial).
    pub output_size: Vec<(&'static str, Polynomial)>,
}

impl ReductionOverhead {
    pub fn new(output_size: Vec<(&'static str, Polynomial)>) -> Self {
        Self { output_size }
    }

    /// Evaluate output size given input size.
    pub fn evaluate_output_size(&self, input: &crate::types::ProblemSize) -> crate::types::ProblemSize {
        let fields: Vec<_> = self.output_size.iter()
            .map(|(name, poly)| (*name, poly.evaluate(input) as usize))
            .collect();
        crate::types::ProblemSize::new(fields)
    }
}

impl Default for ReductionOverhead {
    fn default() -> Self {
        Self { output_size: vec![] }
    }
}

/// A registered reduction entry.
pub struct ReductionEntry {
    /// Base name of source problem (e.g., "IndependentSet").
    pub source_name: &'static str,
    /// Base name of target problem (e.g., "VertexCovering").
    pub target_name: &'static str,
    /// Graph type of source problem (e.g., "SimpleGraph").
    pub source_graph: &'static str,
    /// Graph type of target problem.
    pub target_graph: &'static str,
    /// Overhead information.
    pub overhead: ReductionOverhead,
}

inventory::collect!(ReductionEntry);

/// Macro for registering a reduction alongside its impl.
#[macro_export]
macro_rules! register_reduction {
    (
        $source:ty => $target:ty,
        output: { $($out_name:ident : $out_poly:expr),* $(,)? }
    ) => {
        ::inventory::submit! {
            $crate::rules::registry::ReductionEntry {
                source_name: <$source as $crate::traits::Problem>::NAME,
                target_name: <$target as $crate::traits::Problem>::NAME,
                source_graph: <<$source as $crate::traits::Problem>::GraphType as $crate::graph_types::GraphMarker>::NAME,
                target_graph: <<$target as $crate::traits::Problem>::GraphType as $crate::graph_types::GraphMarker>::NAME,
                overhead: $crate::rules::registry::ReductionOverhead::new(vec![
                    $((stringify!($out_name), $out_poly)),*
                ]),
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ProblemSize;
    use crate::poly;

    #[test]
    fn test_reduction_overhead_evaluate() {
        let overhead = ReductionOverhead::new(vec![
            ("n", poly!(3 * m)),
            ("m", poly!(m^2)),
        ]);

        let input = ProblemSize::new(vec![("m", 4)]);
        let output = overhead.evaluate_output_size(&input);

        assert_eq!(output.get("n"), Some(12));  // 3 * 4
        assert_eq!(output.get("m"), Some(16));  // 4^2
    }
}
```

**Step 2: Add module to rules/mod.rs**

```rust
pub mod registry;
pub use registry::{ReductionEntry, ReductionOverhead};
```

**Step 3: Run tests**

```bash
cargo test registry --lib
```

Expected: PASS

**Step 4: Commit**

```bash
git add src/rules/registry.rs src/rules/mod.rs
git commit -m "feat: Add reduction registration with inventory"
```

---

### Task 2.2: Create Cost Function Traits

**Files:**
- Create: `src/rules/cost.rs`
- Modify: `src/rules/mod.rs`

**Step 1: Create cost module**

Create `src/rules/cost.rs`:

```rust
//! Cost functions for reduction path optimization.

use crate::rules::registry::ReductionOverhead;
use crate::types::ProblemSize;

/// User-defined cost function for path optimization.
pub trait PathCostFn {
    /// Compute cost of taking an edge given current problem size.
    fn edge_cost(&self, overhead: &ReductionOverhead, current_size: &ProblemSize) -> f64;
}

/// Minimize a single output field.
pub struct Minimize(pub &'static str);

impl PathCostFn for Minimize {
    fn edge_cost(&self, overhead: &ReductionOverhead, size: &ProblemSize) -> f64 {
        overhead.evaluate_output_size(size).get(self.0).unwrap_or(0) as f64
    }
}

/// Minimize weighted sum of output fields.
pub struct MinimizeWeighted(pub Vec<(&'static str, f64)>);

impl PathCostFn for MinimizeWeighted {
    fn edge_cost(&self, overhead: &ReductionOverhead, size: &ProblemSize) -> f64 {
        let output = overhead.evaluate_output_size(size);
        self.0.iter()
            .map(|(field, weight)| weight * output.get(field).unwrap_or(0) as f64)
            .sum()
    }
}

/// Minimize the maximum of specified fields.
pub struct MinimizeMax(pub Vec<&'static str>);

impl PathCostFn for MinimizeMax {
    fn edge_cost(&self, overhead: &ReductionOverhead, size: &ProblemSize) -> f64 {
        let output = overhead.evaluate_output_size(size);
        self.0.iter()
            .map(|field| output.get(field).unwrap_or(0) as f64)
            .fold(0.0, f64::max)
    }
}

/// Lexicographic: minimize first field, break ties with subsequent.
pub struct MinimizeLexicographic(pub Vec<&'static str>);

impl PathCostFn for MinimizeLexicographic {
    fn edge_cost(&self, overhead: &ReductionOverhead, size: &ProblemSize) -> f64 {
        let output = overhead.evaluate_output_size(size);
        let mut cost = 0.0;
        let mut scale = 1.0;
        for field in &self.0 {
            cost += scale * output.get(field).unwrap_or(0) as f64;
            scale *= 1e-10;
        }
        cost
    }
}

/// Minimize number of reduction steps.
pub struct MinimizeSteps;

impl PathCostFn for MinimizeSteps {
    fn edge_cost(&self, _overhead: &ReductionOverhead, _size: &ProblemSize) -> f64 {
        1.0
    }
}

/// Custom cost function from closure.
pub struct CustomCost<F>(pub F);

impl<F: Fn(&ReductionOverhead, &ProblemSize) -> f64> PathCostFn for CustomCost<F> {
    fn edge_cost(&self, overhead: &ReductionOverhead, size: &ProblemSize) -> f64 {
        (self.0)(overhead, size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::polynomial::Polynomial;

    fn test_overhead() -> ReductionOverhead {
        ReductionOverhead::new(vec![
            ("n", Polynomial::var("n").scale(2.0)),
            ("m", Polynomial::var("m")),
        ])
    }

    #[test]
    fn test_minimize_single() {
        let cost_fn = Minimize("n");
        let size = ProblemSize::new(vec![("n", 10), ("m", 5)]);
        let overhead = test_overhead();

        assert_eq!(cost_fn.edge_cost(&overhead, &size), 20.0);  // 2 * 10
    }

    #[test]
    fn test_minimize_weighted() {
        let cost_fn = MinimizeWeighted(vec![("n", 1.0), ("m", 2.0)]);
        let size = ProblemSize::new(vec![("n", 10), ("m", 5)]);
        let overhead = test_overhead();

        // output n = 20, output m = 5
        // cost = 1.0 * 20 + 2.0 * 5 = 30
        assert_eq!(cost_fn.edge_cost(&overhead, &size), 30.0);
    }

    #[test]
    fn test_minimize_steps() {
        let cost_fn = MinimizeSteps;
        let size = ProblemSize::new(vec![("n", 100)]);
        let overhead = test_overhead();

        assert_eq!(cost_fn.edge_cost(&overhead, &size), 1.0);
    }
}
```

**Step 2: Add module to rules/mod.rs**

```rust
pub mod cost;
pub use cost::{PathCostFn, Minimize, MinimizeWeighted, MinimizeMax, MinimizeLexicographic, MinimizeSteps, CustomCost};
```

**Step 3: Run tests**

```bash
cargo test cost --lib
```

Expected: PASS

**Step 4: Commit**

```bash
git add src/rules/cost.rs src/rules/mod.rs
git commit -m "feat: Add PathCostFn trait and built-in cost functions"
```

---

## Phase 3: Update Problem Trait

### Task 3.1: Add NAME and GraphType to Problem Trait

**Files:**
- Modify: `src/traits.rs`

**Step 1: Update Problem trait**

Add to `Problem` trait:

```rust
use crate::graph_types::{GraphMarker, SimpleGraph};
use crate::types::NumericWeight;

pub trait Problem: Clone {
    /// Base name of this problem type (e.g., "IndependentSet").
    const NAME: &'static str;

    /// The graph type this problem operates on.
    type GraphType: GraphMarker;

    /// The weight type for this problem.
    type Weight: NumericWeight;

    /// The type used for objective/size values.
    type Size: Clone + PartialOrd + Num + Zero + AddAssign;

    // ... existing methods ...
}
```

**Step 2: This will cause compilation errors - fix in next tasks**

Note: All problem implementations will need updating. Proceed to Phase 4.

---

## Phase 4: Update Problem Implementations

### Task 4.1: Update IndependentSet

**Files:**
- Modify: `src/models/graph/independent_set.rs`

**Step 1: Update struct definition**

```rust
use crate::graph_types::{GraphMarker, SimpleGraph};
use crate::types::NumericWeight;
use std::marker::PhantomData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndependentSet<G: GraphMarker = SimpleGraph, W: NumericWeight = i32> {
    graph: UnGraph<(), ()>,
    weights: Vec<W>,
    #[serde(skip)]
    _phantom: PhantomData<G>,
}
```

**Step 2: Update impl blocks**

```rust
impl<G: GraphMarker, W: NumericWeight + Default> IndependentSet<G, W> {
    pub fn new(num_vertices: usize, edges: Vec<(usize, usize)>) -> Self
    where
        W: From<i32>,
    {
        // ... existing implementation
        Self { graph, weights, _phantom: PhantomData }
    }
}

impl<G: GraphMarker, W: NumericWeight> Problem for IndependentSet<G, W> {
    const NAME: &'static str = "IndependentSet";
    type GraphType = G;
    type Weight = W;
    type Size = W;
    // ... rest of impl
}
```

**Step 3: Run tests**

```bash
cargo test independent_set --lib
```

Expected: May have compilation errors to fix

**Step 4: Fix any remaining issues and commit**

```bash
git add src/models/graph/independent_set.rs
git commit -m "refactor: Add GraphMarker parameter to IndependentSet"
```

---

### Task 4.2-4.10: Update Remaining Problems

Apply the same pattern to:
- `VertexCovering`
- `MaxCut`
- `Matching`
- `DominatingSet`
- `Coloring`
- `SpinGlass`
- `QUBO`
- `SetPacking`
- `SetCovering`
- `Satisfiability`
- `KSatisfiability`
- `CircuitSAT`
- `Factoring`

Each follows the same pattern:
1. Add `G: GraphMarker` parameter (default `SimpleGraph`)
2. Add `PhantomData<G>` field
3. Update `Problem` impl with `NAME`, `GraphType`, `Weight`
4. Test and commit

---

## Phase 5: Update ReductionGraph

### Task 5.1: Rewrite ReductionGraph with Set-Theoretic Path Finding

**Files:**
- Modify: `src/rules/graph.rs`

**Step 1: Add new imports and fields**

```rust
use crate::graph_types::GraphSubtypeEntry;
use crate::rules::registry::ReductionEntry;
use crate::rules::cost::PathCostFn;
use ordered_float::OrderedFloat;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
```

**Step 2: Add graph hierarchy**

```rust
pub struct ReductionGraph {
    graph: DiGraph<&'static str, ReductionEdge>,
    name_indices: HashMap<&'static str, NodeIndex>,
    type_to_name: HashMap<TypeId, &'static str>,
    graph_hierarchy: HashMap<&'static str, HashSet<&'static str>>,
}

pub struct ReductionEdge {
    pub source_graph: &'static str,
    pub target_graph: &'static str,
    pub overhead: ReductionOverhead,
}
```

**Step 3: Implement set-theoretic validation**

```rust
impl ReductionGraph {
    fn build_graph_hierarchy() -> HashMap<&'static str, HashSet<&'static str>> {
        let mut supertypes: HashMap<&'static str, HashSet<&'static str>> = HashMap::new();

        for entry in inventory::iter::<GraphSubtypeEntry> {
            supertypes.entry(entry.subtype)
                .or_default()
                .insert(entry.supertype);
        }

        // Compute transitive closure
        loop {
            let mut changed = false;
            let types: Vec<_> = supertypes.keys().copied().collect();

            for sub in &types {
                let current: Vec<_> = supertypes.get(sub)
                    .map(|s| s.iter().copied().collect())
                    .unwrap_or_default();

                for sup in current {
                    if let Some(sup_supers) = supertypes.get(sup).cloned() {
                        for ss in sup_supers {
                            if supertypes.entry(sub).or_default().insert(ss) {
                                changed = true;
                            }
                        }
                    }
                }
            }

            if !changed { break; }
        }

        supertypes
    }

    pub fn is_graph_subtype(&self, sub: &str, sup: &str) -> bool {
        sub == sup || self.graph_hierarchy
            .get(sub)
            .map(|s| s.contains(sup))
            .unwrap_or(false)
    }

    /// Check if reduction rule can be used: A ⊆ C and D ⊆ B
    pub fn rule_applicable(
        &self,
        want_source_graph: &str,
        want_target_graph: &str,
        rule_source_graph: &str,
        rule_target_graph: &str,
    ) -> bool {
        self.is_graph_subtype(want_source_graph, rule_source_graph)
            && self.is_graph_subtype(rule_target_graph, want_target_graph)
    }
}
```

**Step 4: Implement Dijkstra with custom cost**

```rust
impl ReductionGraph {
    pub fn find_cheapest_path<C: PathCostFn>(
        &self,
        source: (&str, &str),  // (problem_name, graph_type)
        target: (&str, &str),
        input_size: &ProblemSize,
        cost_fn: &C,
    ) -> Option<ReductionPath> {
        let src_idx = *self.name_indices.get(source.0)?;
        let dst_idx = *self.name_indices.get(target.0)?;

        let mut costs: HashMap<NodeIndex, f64> = HashMap::new();
        let mut sizes: HashMap<NodeIndex, ProblemSize> = HashMap::new();
        let mut prev: HashMap<NodeIndex, (NodeIndex, EdgeIndex)> = HashMap::new();
        let mut heap = BinaryHeap::new();

        costs.insert(src_idx, 0.0);
        sizes.insert(src_idx, input_size.clone());
        heap.push(Reverse((OrderedFloat(0.0), src_idx)));

        while let Some(Reverse((cost, node))) = heap.pop() {
            if node == dst_idx {
                return Some(self.reconstruct_path(&prev, src_idx, dst_idx));
            }

            if cost.0 > *costs.get(&node).unwrap_or(&f64::INFINITY) {
                continue;
            }

            let current_size = sizes.get(&node)?;

            for edge_idx in self.graph.edges(node) {
                let edge = &self.graph[edge_idx.id()];
                let next = edge_idx.target();

                if !self.rule_applicable(
                    source.1, target.1,
                    edge.source_graph, edge.target_graph,
                ) {
                    continue;
                }

                let edge_cost = cost_fn.edge_cost(&edge.overhead, current_size);
                let new_cost = cost.0 + edge_cost;
                let new_size = edge.overhead.evaluate_output_size(current_size);

                if new_cost < *costs.get(&next).unwrap_or(&f64::INFINITY) {
                    costs.insert(next, new_cost);
                    sizes.insert(next, new_size);
                    prev.insert(next, (node, edge_idx.id()));
                    heap.push(Reverse((OrderedFloat(new_cost), next)));
                }
            }
        }

        None
    }
}
```

**Step 5: Run tests**

```bash
cargo test graph --lib
```

Expected: PASS (may need to update tests)

**Step 6: Commit**

```bash
git add src/rules/graph.rs
git commit -m "refactor: Implement set-theoretic path finding with cost functions"
```

---

## Phase 6: Update Reduction Implementations

### Task 6.1: Update Reduction Implementations with Registration

For each reduction in `src/rules/`, add registration:

Example for `spinglass_maxcut.rs`:

```rust
use crate::{register_reduction, poly};

// After the impl ReduceTo block:
register_reduction!(
    MaxCut<SimpleGraph, i32> => SpinGlass<SimpleGraph, i32>,
    output: {
        n: poly!(n),
        m: poly!(m),
    }
);
```

Apply to all reduction files.

---

## Phase 7: Integration Tests

### Task 7.1: Add Integration Tests

**Files:**
- Create: `tests/set_theoretic_tests.rs`

```rust
use problemreductions::prelude::*;
use problemreductions::graph_types::*;
use problemreductions::rules::{ReductionGraph, Minimize, MinimizeSteps};

#[test]
fn test_unit_disk_to_simple_path() {
    let graph = ReductionGraph::new();

    // Unit disk IS should find path to Simple SpinGlass
    let path = graph.find_cheapest_path(
        ("IndependentSet", "UnitDiskGraph"),
        ("SpinGlass", "SimpleGraph"),
        &ProblemSize::new(vec![("n", 100), ("m", 200)]),
        &MinimizeSteps,
    );

    assert!(path.is_some());
}

#[test]
fn test_simple_cannot_use_unit_disk_rule() {
    let graph = ReductionGraph::new();

    // If only UnitDiskGraph rules exist, SimpleGraph source should fail
    // (This tests that A ⊆ C is enforced)
}

#[test]
fn test_weight_conversion() {
    // Test that i32 -> f64 works but f64 -> i32 doesn't
}
```

---

## Summary

**Total Tasks:** ~25 bite-sized tasks across 7 phases

**Key Files Changed:**
- `Cargo.toml` - dependencies
- `src/graph_types.rs` - new
- `src/polynomial.rs` - new
- `src/types.rs` - NumericWeight
- `src/traits.rs` - Problem trait updates
- `src/models/**/*.rs` - all problem types
- `src/rules/registry.rs` - new
- `src/rules/cost.rs` - new
- `src/rules/graph.rs` - major rewrite
- `src/rules/*.rs` - all reductions

**Commit Frequency:** After each task step 5 (approx. every 10-15 minutes of work)
