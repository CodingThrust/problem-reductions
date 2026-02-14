# Robust Variant System Redesign — Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.
> **Save location:** Copy this plan to `docs/plans/2026-02-14-variant-system-redesign.md` before starting implementation.

**Goal:** Replace the ad-hoc variant system with a unified `VariantParam` trait where graph types, weight types, and K values all self-declare their category, hierarchy position, and parent cast — eliminating hardcoded key matching, dead code, and const generic special-casing.

**Architecture:** Three new traits (`VariantParam`, `CastToParent`, `KValue`), two `macro_rules!` macros (`impl_variant_param!`, `variant_params!`), and two new graph types (`KingsSubgraph`, `TriangularSubgraph`). The `ReductionGraph` discovers the full hierarchy from `VariantTypeEntry` inventory registrations at runtime.

**Tech Stack:** Rust, `inventory` crate (already a dependency), `macro_rules!` (no proc macro changes needed beyond cleanup)

---

## Design Decisions

| Decision | Choice |
|----------|--------|
| Macro approach | `macro_rules!` only (no proc macro) |
| VariantParam coupling | Standalone impls (NOT supertrait of Graph/WeightElement) |
| Type parameter renaming | Skip — keys come from `VariantParam::CATEGORY` |
| Variant keys | Keep current lowercase: `"graph"`, `"weight"`, `"k"` |
| Hierarchy shape | Single-parent tree (transitivity computed by walking chain) |
| GridGraph/Triangular | Internal-only (not in public variant hierarchy) |
| New public graph types | `KingsSubgraph`, `TriangularSubgraph` (non-generic, subtype UnitDiskGraph) |
| Runtime casts | Required — `CastToParent` trait declared alongside hierarchy |
| VALUE derivation | From `stringify!($ty)` — no explicit VALUE argument needed |
| K values | Replace `const K: usize` with `K: KValue` type parameter (K2, K3, KN) |

---

## Task 1: Core Variant Infrastructure

**Files:**
- Modify: `src/variant.rs`
- Test: `src/unit_tests/variant.rs`

### Step 1: Write failing tests for VariantParam trait and macros

Add to `src/unit_tests/variant.rs`:

```rust
use crate::variant::{CastToParent, VariantParam, VariantTypeEntry};

// Test types for the new system
#[derive(Clone, Debug)]
struct TestRoot;
#[derive(Clone, Debug)]
struct TestChild;

impl_variant_param!(TestRoot, "test_cat");
impl_variant_param!(TestChild, "test_cat", parent: TestRoot, cast: |_| TestRoot);

#[test]
fn test_variant_param_root() {
    assert_eq!(TestRoot::CATEGORY, "test_cat");
    assert_eq!(TestRoot::VALUE, "TestRoot");
    assert_eq!(TestRoot::PARENT_VALUE, None);
}

#[test]
fn test_variant_param_child() {
    assert_eq!(TestChild::CATEGORY, "test_cat");
    assert_eq!(TestChild::VALUE, "TestChild");
    assert_eq!(TestChild::PARENT_VALUE, Some("TestRoot"));
}

#[test]
fn test_cast_to_parent() {
    let child = TestChild;
    let _parent: TestRoot = child.cast_to_parent();
}

#[test]
fn test_variant_type_entry_registered() {
    let entries: Vec<_> = inventory::iter::<VariantTypeEntry>()
        .filter(|e| e.category == "test_cat")
        .collect();
    assert!(entries.iter().any(|e| e.value == "TestRoot" && e.parent.is_none()));
    assert!(entries.iter().any(|e| e.value == "TestChild" && e.parent == Some("TestRoot")));
}

#[derive(Clone, Debug)]
struct TestKRoot;
#[derive(Clone, Debug)]
struct TestKChild;

impl_variant_param!(TestKRoot, "test_k", k: None);
impl_variant_param!(TestKChild, "test_k", parent: TestKRoot, cast: |_| TestKRoot, k: Some(3));

#[test]
fn test_kvalue_via_macro_root() {
    assert_eq!(TestKRoot::CATEGORY, "test_k");
    assert_eq!(TestKRoot::VALUE, "TestKRoot");
    assert_eq!(TestKRoot::PARENT_VALUE, None);
    assert_eq!(TestKRoot::K, None);
}

#[test]
fn test_kvalue_via_macro_child() {
    assert_eq!(TestKChild::CATEGORY, "test_k");
    assert_eq!(TestKChild::VALUE, "TestKChild");
    assert_eq!(TestKChild::PARENT_VALUE, Some("TestKRoot"));
    assert_eq!(TestKChild::K, Some(3));
}

#[test]
fn test_variant_params_macro_empty() {
    let v: Vec<(&str, &str)> = variant_params![];
    assert!(v.is_empty());
}

#[test]
fn test_variant_params_macro_single() {
    fn check<T: VariantParam>() -> Vec<(&'static str, &'static str)> {
        variant_params![T]
    }
    let v = check::<TestRoot>();
    assert_eq!(v, vec![("test_cat", "TestRoot")]);
}

#[test]
fn test_variant_params_macro_multiple() {
    fn check<A: VariantParam, B: VariantParam>() -> Vec<(&'static str, &'static str)> {
        variant_params![A, B]
    }
    let v = check::<TestRoot, TestChild>();
    assert_eq!(v, vec![("test_cat", "TestRoot"), ("test_cat", "TestChild")]);
}
```

### Step 2: Run tests to verify they fail

Run: `cargo test --lib variant -- --test-output`
Expected: Compilation errors — `VariantParam`, `CastToParent`, `VariantTypeEntry`, `impl_variant_param!`, `variant_params!` don't exist yet.

### Step 3: Implement VariantParam, CastToParent, VariantTypeEntry, and macros

Replace `src/variant.rs` contents with:

```rust
//! Variant system for type-level problem parameterization.
//!
//! Types declare their variant category, value, and parent via `VariantParam`.
//! The `impl_variant_param!` macro registers types with both the trait and
//! the runtime `VariantTypeEntry` inventory. The `variant_params!` macro
//! composes `Problem::variant()` bodies from type parameter names.

/// A type that participates in the variant system.
///
/// Declares its category (e.g., `"graph"`), value (e.g., `"SimpleGraph"`),
/// and optional parent in the subtype hierarchy.
pub trait VariantParam: 'static {
    /// Category name (e.g., `"graph"`, `"weight"`, `"k"`).
    const CATEGORY: &'static str;
    /// Type name within the category (e.g., `"SimpleGraph"`, `"i32"`).
    const VALUE: &'static str;
    /// Parent type name in the subtype hierarchy, or `None` for root types.
    const PARENT_VALUE: Option<&'static str>;
}

/// Types that can convert themselves to their parent in the variant hierarchy.
pub trait CastToParent: VariantParam {
    /// The parent type.
    type Parent: VariantParam;
    /// Convert this value to its parent type.
    fn cast_to_parent(&self) -> Self::Parent;
}

/// Runtime-discoverable variant type registration.
///
/// Built by `impl_variant_param!` macro, collected by `inventory`.
pub struct VariantTypeEntry {
    pub category: &'static str,
    pub value: &'static str,
    pub parent: Option<&'static str>,
}

inventory::collect!(VariantTypeEntry);

/// Implement `VariantParam` (and optionally `CastToParent`) for a type,
/// and register a `VariantTypeEntry` with inventory.
///
/// # Usage
///
/// ```rust,ignore
/// // Root type (no parent):
/// impl_variant_param!(SimpleGraph, "graph");
///
/// // Type with parent — cast closure required:
/// impl_variant_param!(UnitDiskGraph, "graph", parent: SimpleGraph,
///     cast: |g| SimpleGraph::new(g.num_vertices(), g.edges()));
/// ```
#[macro_export]
macro_rules! impl_variant_param {
    // Root type (no parent, no cast)
    ($ty:ty, $cat:expr) => {
        impl $crate::variant::VariantParam for $ty {
            const CATEGORY: &'static str = $cat;
            const VALUE: &'static str = stringify!($ty);
            const PARENT_VALUE: Option<&'static str> = None;
        }
        ::inventory::submit! {
            $crate::variant::VariantTypeEntry {
                category: $cat,
                value: stringify!($ty),
                parent: None,
            }
        }
    };
    // Type with parent + cast closure
    ($ty:ty, $cat:expr, parent: $parent:ty, cast: $cast:expr) => {
        impl $crate::variant::VariantParam for $ty {
            const CATEGORY: &'static str = $cat;
            const VALUE: &'static str = stringify!($ty);
            const PARENT_VALUE: Option<&'static str> = Some(stringify!($parent));
        }
        impl $crate::variant::CastToParent for $ty {
            type Parent = $parent;
            fn cast_to_parent(&self) -> $parent {
                let f: fn(&$ty) -> $parent = $cast;
                f(self)
            }
        }
        ::inventory::submit! {
            $crate::variant::VariantTypeEntry {
                category: $cat,
                value: stringify!($ty),
                parent: Some(stringify!($parent)),
            }
        }
    };
    // KValue root type (no parent, with k value)
    ($ty:ty, $cat:expr, k: $k:expr) => {
        $crate::impl_variant_param!($ty, $cat);
        impl $crate::variant::KValue for $ty {
            const K: Option<usize> = $k;
        }
    };
    // KValue type with parent + cast + k value
    ($ty:ty, $cat:expr, parent: $parent:ty, cast: $cast:expr, k: $k:expr) => {
        $crate::impl_variant_param!($ty, $cat, parent: $parent, cast: $cast);
        impl $crate::variant::KValue for $ty {
            const K: Option<usize> = $k;
        }
    };
}

/// Compose a `Problem::variant()` body from type parameter names.
///
/// All variant dimensions must be types implementing `VariantParam`.
///
/// # Usage
///
/// ```rust,ignore
/// variant_params![]           // → vec![]
/// variant_params![G, W]       // → vec![(G::CATEGORY, G::VALUE), ...]
/// ```
#[macro_export]
macro_rules! variant_params {
    () => { vec![] };
    ($($T:ident),+) => {
        vec![$((<$T as $crate::variant::VariantParam>::CATEGORY,
              <$T as $crate::variant::VariantParam>::VALUE)),+]
    };
}

#[cfg(test)]
#[path = "unit_tests/variant.rs"]
mod tests;
```

### Step 4: Run tests to verify they pass

Run: `cargo test --lib variant`
Expected: All new tests PASS. Old tests for `short_type_name` and `const_usize_str` still pass (we haven't removed them yet).

### Step 5: Commit

```bash
git add src/variant.rs src/unit_tests/variant.rs
git commit -m "feat: add VariantParam trait, CastToParent, impl_variant_param!, variant_params! macros"
```

---

## Task 2: KValue Types

**Files:**
- Modify: `src/variant.rs`
- Test: `src/unit_tests/variant.rs`

### Step 1: Write failing tests for KValue types

Add to `src/unit_tests/variant.rs`:

```rust
use crate::variant::{K2, K3, KN, KValue};

#[test]
fn test_kvalue_k2() {
    assert_eq!(K2::CATEGORY, "k");
    assert_eq!(K2::VALUE, "K2");
    assert_eq!(K2::PARENT_VALUE, Some("K3"));
    assert_eq!(K2::K, Some(2));
}

#[test]
fn test_kvalue_k3() {
    assert_eq!(K3::CATEGORY, "k");
    assert_eq!(K3::VALUE, "K3");
    assert_eq!(K3::PARENT_VALUE, Some("KN"));
    assert_eq!(K3::K, Some(3));
}

#[test]
fn test_kvalue_kn() {
    assert_eq!(KN::CATEGORY, "k");
    assert_eq!(KN::VALUE, "KN");
    assert_eq!(KN::PARENT_VALUE, None);
    assert_eq!(KN::K, None);
}

#[test]
fn test_kvalue_cast_chain() {
    let k2 = K2;
    let k3: K3 = k2.cast_to_parent();
    let kn: KN = k3.cast_to_parent();
    assert_eq!(KN::K, None);
    let _ = kn; // use it
}

#[test]
fn test_kvalue_variant_entries() {
    let entries: Vec<_> = inventory::iter::<VariantTypeEntry>()
        .filter(|e| e.category == "k")
        .collect();
    assert!(entries.iter().any(|e| e.value == "KN" && e.parent.is_none()));
    assert!(entries.iter().any(|e| e.value == "K3" && e.parent == Some("KN")));
    assert!(entries.iter().any(|e| e.value == "K2" && e.parent == Some("K3")));
}
```

### Step 2: Run tests to verify they fail

Run: `cargo test --lib variant::tests::test_kvalue`
Expected: Compilation errors — `KValue`, `K2`, `K3`, `KN` don't exist yet.

### Step 3: Implement KValue trait and types

Add to `src/variant.rs` (before the `#[cfg(test)]` block):

```rust
/// Trait for K-value types used in KSatisfiability and KColoring.
///
/// Each type represents a specific K value (K2=2, K3=3, etc.) or
/// the generic case (KN = any K). Hierarchy: K2 < K3 < KN.
///
/// Use `impl_variant_param!` with the `k:` argument to implement this trait:
/// ```rust,ignore
/// impl_variant_param!(K3, "k", parent: KN, cast: |_| KN, k: Some(3));
/// ```
pub trait KValue: VariantParam + Clone + 'static {
    /// The concrete K value, or `None` for the generic case (KN).
    const K: Option<usize>;
}

/// K=2 (e.g., 2-SAT, 2-coloring).
#[derive(Clone, Copy, Debug, Default)]
pub struct K2;

/// K=3 (e.g., 3-SAT, 3-coloring).
#[derive(Clone, Copy, Debug, Default)]
pub struct K3;

/// Generic K (any value). Used for reductions that apply to all K.
#[derive(Clone, Copy, Debug, Default)]
pub struct KN;

impl_variant_param!(KN, "k", k: None);
impl_variant_param!(K3, "k", parent: KN, cast: |_| KN, k: Some(3));
impl_variant_param!(K2, "k", parent: K3, cast: |_| K3, k: Some(2));
```

### Step 4: Run tests to verify they pass

Run: `cargo test --lib variant`
Expected: All KValue tests PASS.

### Step 5: Commit

```bash
git add src/variant.rs src/unit_tests/variant.rs
git commit -m "feat: add KValue trait with K2, K3, KN types for type-level K values"
```

---

## Task 3: Register Graph Types with VariantParam

**Files:**
- Modify: `src/topology/graph.rs` (SimpleGraph)
- Modify: `src/topology/unit_disk_graph.rs` (UnitDiskGraph)
- Modify: `src/topology/hypergraph.rs` (HyperGraph)
- Test: `src/unit_tests/variant.rs`

### Step 1: Write failing tests

Add to `src/unit_tests/variant.rs`:

```rust
use crate::topology::{Graph, SimpleGraph, UnitDiskGraph};
use crate::topology::HyperGraph;

#[test]
fn test_simple_graph_variant_param() {
    assert_eq!(SimpleGraph::CATEGORY, "graph");
    assert_eq!(SimpleGraph::VALUE, "SimpleGraph");
    assert_eq!(SimpleGraph::PARENT_VALUE, Some("HyperGraph"));
}

#[test]
fn test_unit_disk_graph_variant_param() {
    assert_eq!(UnitDiskGraph::CATEGORY, "graph");
    assert_eq!(UnitDiskGraph::VALUE, "UnitDiskGraph");
    assert_eq!(UnitDiskGraph::PARENT_VALUE, Some("SimpleGraph"));
}

#[test]
fn test_hyper_graph_variant_param() {
    assert_eq!(HyperGraph::CATEGORY, "graph");
    assert_eq!(HyperGraph::VALUE, "HyperGraph");
    assert_eq!(HyperGraph::PARENT_VALUE, None);
}

#[test]
fn test_graph_variant_entries() {
    let entries: Vec<_> = inventory::iter::<VariantTypeEntry>()
        .filter(|e| e.category == "graph")
        .collect();
    assert!(entries.iter().any(|e| e.value == "HyperGraph" && e.parent.is_none()));
    assert!(entries.iter().any(|e| e.value == "SimpleGraph" && e.parent == Some("HyperGraph")));
    assert!(entries.iter().any(|e| e.value == "UnitDiskGraph" && e.parent == Some("SimpleGraph")));
}

#[test]
fn test_simple_graph_cast_to_parent() {
    let sg = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let hg: HyperGraph = sg.cast_to_parent();
    assert_eq!(hg.num_vertices(), 3);
    assert_eq!(hg.num_edges(), 2);
}

#[test]
fn test_udg_cast_to_parent() {
    let udg = UnitDiskGraph::new(vec![(0.0, 0.0), (0.5, 0.0), (2.0, 0.0)], 1.0);
    let sg: SimpleGraph = udg.cast_to_parent();
    assert_eq!(sg.num_vertices(), 3);
    // Only the first two points are within distance 1.0
    assert!(sg.has_edge(0, 1));
    assert!(!sg.has_edge(0, 2));
}
```

### Step 2: Run tests to verify they fail

Run: `cargo test --lib variant::tests::test_simple_graph`
Expected: Compilation errors — graph types don't implement `VariantParam`.

### Step 3: Add impl_variant_param! to each graph type file

In `src/topology/hypergraph.rs`, add at the end (before any `#[cfg(test)]`):
```rust
use crate::impl_variant_param;
impl_variant_param!(HyperGraph, "graph");
```

In `src/topology/graph.rs`, add after the `SimpleGraph` impl of `Graph`:
```rust
use crate::impl_variant_param;
impl_variant_param!(SimpleGraph, "graph", parent: HyperGraph,
    cast: |g| HyperGraph::from_graph_edges(g.num_vertices(), g.edges()));
```

In `src/topology/unit_disk_graph.rs`, add after the `UnitDiskGraph` impl of `Graph`:
```rust
use crate::impl_variant_param;
impl_variant_param!(UnitDiskGraph, "graph", parent: SimpleGraph,
    cast: |g| SimpleGraph::new(g.num_vertices(), g.edges()));
```

Note: `HyperGraph::from_graph_edges` may need to be implemented (or use existing constructor). Check HyperGraph API and adapt the cast closure.

### Step 4: Run tests to verify they pass

Run: `cargo test --lib variant`
Expected: All graph variant tests PASS.

### Step 5: Commit

```bash
git add src/topology/graph.rs src/topology/unit_disk_graph.rs src/topology/hypergraph.rs src/unit_tests/variant.rs
git commit -m "feat: register SimpleGraph, UnitDiskGraph, HyperGraph with VariantParam"
```

---

## Task 4: Register Weight Types with VariantParam

**Files:**
- Modify: `src/types.rs`
- Test: `src/unit_tests/variant.rs`

### Step 1: Write failing tests

Add to `src/unit_tests/variant.rs`:

```rust
use crate::types::One;

#[test]
fn test_weight_f64_variant_param() {
    assert_eq!(<f64 as VariantParam>::CATEGORY, "weight");
    assert_eq!(<f64 as VariantParam>::VALUE, "f64");
    assert_eq!(<f64 as VariantParam>::PARENT_VALUE, None);
}

#[test]
fn test_weight_i32_variant_param() {
    assert_eq!(<i32 as VariantParam>::CATEGORY, "weight");
    assert_eq!(<i32 as VariantParam>::VALUE, "i32");
    assert_eq!(<i32 as VariantParam>::PARENT_VALUE, Some("f64"));
}

#[test]
fn test_weight_one_variant_param() {
    assert_eq!(One::CATEGORY, "weight");
    assert_eq!(One::VALUE, "One");
    assert_eq!(One::PARENT_VALUE, Some("i32"));
}

#[test]
fn test_weight_cast_chain() {
    let one = One;
    let i: i32 = one.cast_to_parent();
    assert_eq!(i, 1);
    let f: f64 = i.cast_to_parent();
    assert_eq!(f, 1.0);
}

#[test]
fn test_weight_variant_entries() {
    let entries: Vec<_> = inventory::iter::<VariantTypeEntry>()
        .filter(|e| e.category == "weight")
        .collect();
    assert!(entries.iter().any(|e| e.value == "f64" && e.parent.is_none()));
    assert!(entries.iter().any(|e| e.value == "i32" && e.parent == Some("f64")));
    assert!(entries.iter().any(|e| e.value == "One" && e.parent == Some("i32")));
}
```

### Step 2: Implement in `src/types.rs`

Add at end of `src/types.rs`:

```rust
use crate::impl_variant_param;

impl_variant_param!(f64, "weight");
impl_variant_param!(i32, "weight", parent: f64, cast: |w| *w as f64);
impl_variant_param!(One, "weight", parent: i32, cast: |_| 1i32);
```

### Step 3: Run tests

Run: `cargo test --lib variant`
Expected: All weight variant tests PASS.

### Step 4: Commit

```bash
git add src/types.rs src/unit_tests/variant.rs
git commit -m "feat: register One, i32, f64 with VariantParam"
```

---

## Task 5: Migrate KSatisfiability from const K to KValue

**Files:**
- Modify: `src/models/satisfiability/ksat.rs`
- Modify: `src/unit_tests/models/satisfiability/ksat.rs`
- Modify: `src/rules/sat_ksat.rs`
- Modify: `src/rules/ksatisfiability_qubo.rs`
- Modify: `src/prelude.rs` (in `src/lib.rs`)
- Test: existing ksat tests

### Step 1: Migrate KSatisfiability struct

In `src/models/satisfiability/ksat.rs`, change:

```rust
// Before:
pub struct KSatisfiability<const K: usize> { ... }

// After:
use crate::variant::{KValue, VariantParam};

pub struct KSatisfiability<K: KValue> {
    num_vars: usize,
    clauses: Vec<CNFClause>,
    _phantom: std::marker::PhantomData<K>,
}
```

Update all methods: replace `K` (const value) with `K::K.expect("KN cannot be instantiated")` or appropriate access. The `new()` method validates clause length using `K::K.unwrap()`.

Update `Problem` impl:

```rust
impl<K: KValue> Problem for KSatisfiability<K> {
    const NAME: &'static str = "KSatisfiability";
    type Metric = bool;

    fn variant() -> Vec<(&'static str, &'static str)> {
        variant_params![K]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.num_vars]
    }
    // ... evaluate unchanged (uses self.clauses)
}
```

### Step 2: Update all KSatisfiability usages

Search-and-replace across the codebase:
- `KSatisfiability::<3>` → `KSatisfiability::<K3>`
- `KSatisfiability::<2>` → `KSatisfiability::<K2>`
- `KSatisfiability<3>` → `KSatisfiability<K3>`
- `KSatisfiability<2>` → `KSatisfiability<K2>`
- `const K: usize` in sat_ksat.rs reduction structs → `K: KValue`

Add `use crate::variant::{K2, K3, KN};` where needed.

### Step 3: Update reduction rules

In `src/rules/sat_ksat.rs`:
- `ReductionSATToKSAT<const K: usize>` → `ReductionSATToKSAT<K: KValue>`
- `ReductionKSATToSAT<const K: usize>` → `ReductionKSATToSAT<K: KValue>`
- `impl_sat_to_ksat!(3)` → concrete impl for K3

In `src/rules/ksatisfiability_qubo.rs`:
- Update similarly

### Step 4: Update prelude in `src/lib.rs`

Add variant system exports to the prelude:
```rust
pub use crate::variant::{CastToParent, KValue, VariantParam, K2, K3, KN};
```
Note: `impl_variant_param!` and `variant_params!` are `#[macro_export]` so they're automatically available at the crate root (`problemreductions::impl_variant_param!`).

### Step 5: Run tests

Run: `cargo test --lib models::satisfiability::ksat && cargo test --lib rules::sat_ksat`
Expected: All existing ksat tests PASS with new type parameter syntax.

### Step 6: Commit

```bash
git add src/models/satisfiability/ksat.rs src/rules/sat_ksat.rs src/rules/ksatisfiability_qubo.rs src/lib.rs src/unit_tests/
git commit -m "refactor: migrate KSatisfiability from const K to KValue type parameter"
```

---

## Task 6: Migrate KColoring from const K to KValue

**Files:**
- Modify: `src/models/graph/kcoloring.rs`
- Modify: `src/unit_tests/models/graph/kcoloring.rs`
- Modify: `src/rules/sat_coloring.rs`
- Modify: `src/rules/coloring_qubo.rs`

### Step 1: Migrate KColoring struct

Same pattern as Task 5. Change `KColoring<const K: usize, G>` to `KColoring<K: KValue, G>`.

```rust
impl<K: KValue, G> Problem for KColoring<K, G>
where G: Graph + VariantParam {
    fn variant() -> Vec<(&'static str, &'static str)> {
        variant_params![K, G]
    }
    fn dims(&self) -> Vec<usize> {
        vec![K::K.expect("KN cannot be used as problem instance"); self.num_vertices()]
    }
}
```

### Step 2: Update all KColoring usages

- `KColoring::<3, SimpleGraph>` → `KColoring::<K3, SimpleGraph>`
- `KColoring::<2, SimpleGraph>` → `KColoring::<K2, SimpleGraph>`
- `KColoring::<4, SimpleGraph>` → add K4 type if needed, or use KN
- `KColoring::<1, SimpleGraph>` → add K1 type if needed

Note: Tests use K=1, K=2, K=3, K=4. Add `K1` and `K4` to `src/variant.rs`:

```rust
#[derive(Clone, Copy, Debug, Default)]
pub struct K1;
#[derive(Clone, Copy, Debug, Default)]
pub struct K4;

impl_variant_param!(K1, "k", parent: K2, cast: |_| K2, k: Some(1));
impl_variant_param!(K4, "k", parent: KN, cast: |_| KN, k: Some(4));
```

Update hierarchy: K1 < K2 < K3 < K4 < KN. Adjust K3's parent to K4.

### Step 3: Run tests

Run: `cargo test --lib models::graph::kcoloring && cargo test --lib rules::sat_coloring`
Expected: All PASS.

### Step 4: Commit

```bash
git add src/models/graph/kcoloring.rs src/rules/sat_coloring.rs src/rules/coloring_qubo.rs src/variant.rs src/unit_tests/
git commit -m "refactor: migrate KColoring from const K to KValue type parameter"
```

---

## Task 7: Apply variant_params! to All Problem Impls

**Files:**
- Modify: 21 model files in `src/models/**/*.rs`
- Modify: 10 test Problem types in `src/unit_tests/`

### Step 1: Add VariantParam bounds and variant_params! to graph+weight problems (9 files)

For each of: `maximum_independent_set.rs`, `minimum_vertex_cover.rs`, `minimum_dominating_set.rs`, `maximum_clique.rs`, `maximum_matching.rs`, `max_cut.rs`, `maximal_is.rs`, `traveling_salesman.rs`, `spin_glass.rs`:

```rust
// Before:
impl<G, W> Problem for MaximumIndependentSet<G, W>
where G: Graph, W: WeightElement {
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", G::NAME), ("weight", crate::variant::short_type_name::<W>())]
    }
}

// After:
use crate::variant::VariantParam;

impl<G, W> Problem for MaximumIndependentSet<G, W>
where G: Graph + VariantParam, W: WeightElement + VariantParam {
    fn variant() -> Vec<(&'static str, &'static str)> {
        variant_params![G, W]
    }
}
```

### Step 2: Weight-only problems (3 files)

For `qubo.rs`, `maximum_set_packing.rs`, `minimum_set_covering.rs`:

```rust
where W: WeightElement + VariantParam {
    fn variant() -> ... { variant_params![W] }
}
```

### Step 3: No-generic problems (7 files)

For `ilp.rs`, `sat.rs`, `circuit.rs`, `factoring.rs`, `biclique_cover.rs`, `bmf.rs`, `paintshop.rs`:

```rust
fn variant() -> ... { variant_params![] }
```

### Step 4: Update test Problem types

In `src/unit_tests/traits.rs`, `src/unit_tests/solvers/brute_force.rs`, `src/unit_tests/rules/traits.rs`:

Add `impl VariantParam` for each test-only Problem type. Since they use hardcoded variants, keep them as direct impls:

```rust
impl VariantParam for TestSatProblem {
    const CATEGORY: &'static str = "test";
    const VALUE: &'static str = "TestSatProblem";
    const PARENT_VALUE: Option<&'static str> = None;
}
```

Or use `variant_params![]` if the test doesn't need specific variant values.

### Step 5: Run all tests

Run: `make test`
Expected: All tests PASS.

### Step 6: Commit

```bash
git add src/models/ src/unit_tests/
git commit -m "refactor: apply variant_params! macro to all Problem implementations"
```

---

## Task 8: New Public Graph Types (KingsSubgraph, TriangularSubgraph)

**Files:**
- Create: `src/topology/kings_subgraph.rs`
- Create: `src/topology/triangular_subgraph.rs`
- Create: `src/unit_tests/topology/kings_subgraph.rs`
- Create: `src/unit_tests/topology/triangular_subgraph.rs`
- Modify: `src/topology/mod.rs`

### Step 1: Implement KingsSubgraph

Create `src/topology/kings_subgraph.rs`:

```rust
//! KingsSubgraph: a non-generic square-grid unit disk subgraph.

use super::graph::{Graph, SimpleGraph};
use crate::impl_variant_param;
use serde::{Deserialize, Serialize};

/// Non-generic graph for square-grid unit disk subgraphs.
///
/// Stores node positions and precomputed edges. Weights are NOT stored
/// here — they belong to the Problem that uses this graph.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KingsSubgraph {
    positions: Vec<(i32, i32)>,
    size: (usize, usize),
    radius: f64,
    edges: Vec<(usize, usize)>,
}

impl KingsSubgraph {
    /// Create from node positions and radius, computing edges.
    pub fn new(positions: Vec<(i32, i32)>, size: (usize, usize), radius: f64) -> Self {
        let mut edges = Vec::new();
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let (r1, c1) = positions[i];
                let (r2, c2) = positions[j];
                let dist = (((r1 - r2) as f64).powi(2) + ((c1 - c2) as f64).powi(2)).sqrt();
                if dist < radius {
                    edges.push((i, j));
                }
            }
        }
        Self { positions, size, radius, edges }
    }

    pub fn positions(&self) -> &[(i32, i32)] { &self.positions }
    pub fn grid_size(&self) -> (usize, usize) { self.size }
    pub fn radius(&self) -> f64 { self.radius }
}

impl Graph for KingsSubgraph {
    const NAME: &'static str = "KingsSubgraph";

    fn num_vertices(&self) -> usize { self.positions.len() }
    fn num_edges(&self) -> usize { self.edges.len() }
    fn edges(&self) -> Vec<(usize, usize)> { self.edges.clone() }

    fn has_edge(&self, u: usize, v: usize) -> bool {
        let (a, b) = if u < v { (u, v) } else { (v, u) };
        self.edges.iter().any(|&(x, y)| x == a && y == b)
    }

    fn neighbors(&self, v: usize) -> Vec<usize> {
        self.edges.iter().filter_map(|&(a, b)| {
            if a == v { Some(b) } else if b == v { Some(a) } else { None }
        }).collect()
    }
}

impl_variant_param!(KingsSubgraph, "graph", parent: UnitDiskGraph,
    cast: |g| {
        use super::unit_disk_graph::UnitDiskGraph;
        let positions: Vec<(f64, f64)> = g.positions().iter()
            .map(|&(r, c)| (r as f64, c as f64))
            .collect();
        UnitDiskGraph::new(positions, g.radius())
    });
```

### Step 2: Implement TriangularSubgraph

Same pattern as KingsSubgraph but with triangular coordinate system. Create `src/topology/triangular_subgraph.rs`.

### Step 3: Update topology module exports

In `src/topology/mod.rs`:

```rust
mod kings_subgraph;
mod triangular_subgraph;

pub use kings_subgraph::KingsSubgraph;
pub use triangular_subgraph::TriangularSubgraph;
```

### Step 4: Write tests, run, commit

Run: `cargo test --lib topology`
Expected: PASS.

```bash
git add src/topology/kings_subgraph.rs src/topology/triangular_subgraph.rs src/topology/mod.rs src/unit_tests/topology/
git commit -m "feat: add KingsSubgraph and TriangularSubgraph graph types"
```

---

## Task 9: Internal-ize GridGraph/Triangular and Restructure Reductions

**Files:**
- Modify: `src/rules/maximumindependentset_gridgraph.rs`
- Modify: `src/rules/maximumindependentset_triangular.rs`
- Modify: `src/topology/mod.rs`
- Modify: `src/graph_types.rs`

### Step 1: Remove `#[reduction]` from GridGraph/Triangular reduction files

The reduction code stays but is no longer registered in the variant graph. Internal unitdiskmapping still uses GridGraph/Triangular.

### Step 2: Add new registered reductions for KingsSubgraph/TriangularSubgraph

Restructure reduction files to output `MIS<KingsSubgraph, i32>` and `MIS<TriangularSubgraph, i32>` instead, converting from internal GridGraph results.

### Step 3: Make GridGraph/Triangular `pub(crate)` in topology

Update `src/topology/mod.rs` to use `pub(crate) use` for GridGraph and Triangular.

### Step 4: Remove GridGraph/Triangular from graph_types.rs markers

Remove `declare_graph_subtype!` entries for GridGraph and Triangular.

### Step 5: Run tests, commit

Run: `make test clippy`

```bash
git commit -m "refactor: internal-ize GridGraph/Triangular, add KingsSubgraph/TriangularSubgraph reductions"
```

---

## Task 10: Unify Hierarchy in ReductionGraph

**Files:**
- Modify: `src/rules/graph.rs`
- Modify: `src/unit_tests/rules/graph.rs`

### Step 1: Replace graph_hierarchy/weight_hierarchy with variant_hierarchy

In `src/rules/graph.rs`, replace the two separate hierarchy fields with:

```rust
variant_hierarchy: HashMap<String, HashMap<String, Option<String>>>,
```

Build from `VariantTypeEntry` inventory.

### Step 2: Generalize is_variant_reducible()

Replace the hardcoded match on key names with generic parent-chain walk:

```rust
fn is_subtype_in(&self, category_types: &HashMap<String, Option<String>>, a: &str, b: &str) -> bool {
    if a == b { return true; }
    let mut current = a;
    loop {
        match category_types.get(current) {
            Some(Some(parent)) => {
                if parent == b { return true; }
                current = parent;
            }
            _ => return false,
        }
    }
}
```

### Step 3: Remove old hierarchy code

Delete `is_graph_subtype`, `is_weight_subtype`, `is_const_subtype`, `graph_hierarchy`, `weight_hierarchy`.

### Step 4: Run tests, commit

Run: `cargo test --lib rules::graph`

```bash
git commit -m "refactor: unify variant hierarchy in ReductionGraph using VariantTypeEntry"
```

---

## Task 11: Remove Old Hierarchy System

**Files:**
- Modify: `src/graph_types.rs`
- Modify: `src/unit_tests/graph_types.rs`

### Step 1: Remove from graph_types.rs

Delete:
- `GraphSubtypeEntry`, `WeightSubtypeEntry` structs + inventory collections
- `declare_graph_subtype!`, `declare_weight_subtype!` macros + all invocations
- `GraphSubtype<G>` trait
- `GraphMarker` trait (verify no other usages first)

Keep: ZST marker structs (SimpleGraph, UnitDiskGraph, etc.) if used elsewhere, OR remove if superseded by topology types.

### Step 2: Update tests

Remove or rewrite `src/unit_tests/graph_types.rs` tests that reference deleted types.

### Step 3: Run tests, commit

Run: `make test clippy`

```bash
git commit -m "refactor: remove old GraphSubtypeEntry/WeightSubtypeEntry hierarchy system"
```

---

## Task 12: Cleanup and Remove Deprecated Code

**Files:**
- Modify: `src/variant.rs` — remove `const_usize_str`, `short_type_name`
- Modify: `problemreductions-macros/src/lib.rs` — remove const generic rewriting logic
- Modify: `src/rules/graph.rs` — remove `is_const_subtype`
- Modify: `src/unit_tests/variant.rs` — remove old tests for deleted functions

### Step 1: Remove const_usize_str and short_type_name

These are replaced by `KValue` types and `VariantParam::VALUE`.

### Step 2: Remove const generic rewriting from proc macro

In `problemreductions-macros/src/lib.rs`:
- Remove `collect_const_generic_names()`
- Remove `rewrite_const_generics()`
- Simplify `make_variant_fn_body()` — no more const generic handling needed since K is now a type param

### Step 3: Run full test suite

Run: `make test clippy`

```bash
git commit -m "refactor: remove deprecated const_usize_str, short_type_name, const generic rewriting"
```

---

## Task 13: Update Examples and Downstream

**Files:**
- Modify: `examples/reduction_*.rs` (files referencing KSatisfiability or KColoring)
- Modify: `src/lib.rs` (prelude updates)

### Step 1: Update example files

- `examples/reduction_ksatisfiability_to_qubo.rs` — `KSatisfiability::<3>` → `KSatisfiability::<K3>`
- `examples/reduction_kcoloring_to_qubo.rs` — `KColoring::<3, SimpleGraph>` → `KColoring::<K3, SimpleGraph>`
- `examples/reduction_kcoloring_to_ilp.rs` — similar
- `examples/reduction_satisfiability_to_ksatisfiability.rs` — `KSatisfiability<3>` → `KSatisfiability<K3>`
- `examples/reduction_satisfiability_to_kcoloring.rs` — `KColoring<3, SimpleGraph>` → `KColoring<K3, SimpleGraph>`

### Step 2: Update integration tests

- `tests/suites/integration.rs` — KColoring<3, ...> → KColoring<K3, ...>
- `tests/suites/reductions.rs` — similar

### Step 3: Run full suite

Run: `make test`

```bash
git commit -m "refactor: update examples and integration tests for KValue type parameters"
```

---

## Task 14: Final Verification

### Step 1: Format and lint

```bash
make fmt
make clippy
```

### Step 2: Run full test suite

```bash
make test
```

### Step 3: Check coverage

```bash
make coverage  # Must remain >95%
```

### Step 4: Regenerate artifacts

```bash
make rust-export
make examples
make doc
```

### Step 5: Verify no regressions

```bash
make compare  # Compare exported JSON
```

### Step 6: Final commit if needed

```bash
git commit -m "chore: regenerate artifacts after variant system redesign"
```

---

## Key Files Summary

| File | Change |
|------|--------|
| `src/variant.rs` | VariantParam, CastToParent, VariantTypeEntry, impl_variant_param! (4 arms: root, parent, k-root, k-parent), variant_params!, KValue, K1-K4/KN |
| `src/topology/kings_subgraph.rs` | NEW |
| `src/topology/triangular_subgraph.rs` | NEW |
| `src/topology/graph.rs` | impl_variant_param! for SimpleGraph |
| `src/topology/unit_disk_graph.rs` | impl_variant_param! for UnitDiskGraph |
| `src/topology/hypergraph.rs` | impl_variant_param! for HyperGraph |
| `src/topology/mod.rs` | Export new types, pub(crate) old ones |
| `src/types.rs` | impl_variant_param! for One, i32, f64 |
| `src/graph_types.rs` | Remove old hierarchy system |
| `src/models/satisfiability/ksat.rs` | const K → K: KValue |
| `src/models/graph/kcoloring.rs` | const K → K: KValue |
| `src/models/**/*.rs` | + VariantParam bounds, variant_params! (21 files) |
| `src/rules/graph.rs` | Unified variant_hierarchy |
| `src/rules/maximumindependentset_gridgraph.rs` | Restructure for KingsSubgraph |
| `src/rules/maximumindependentset_triangular.rs` | Restructure for TriangularSubgraph |
| `problemreductions-macros/src/lib.rs` | Remove const generic rewriting |
