# Variant-Aware Reduction Paths — Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add `resolve_path()` to lift name-level reduction paths into variant-level paths with natural cast steps, fixing overhead disambiguation (issue 2) and natural edge inconsistency (issue 5).

**Architecture:** `ResolvedPath` is a new type layered on top of the existing `ReductionPath`. The resolver walks a name-level path, threads variant state through each edge, picks the most-specific matching `ReductionEntry`, and inserts `NaturalCast` steps where the caller's variant is more specific than what the rule expects. No changes to the name-level graph or path-finding algorithms.

**Tech Stack:** Rust, `inventory` crate (existing), `petgraph` (existing), `serde` (existing), `BTreeMap` for variant representation.

---

### Task 1: Add `ResolvedPath` data types

**Files:**
- Modify: `src/rules/graph.rs` (after `ReductionPath` impl block, ~line 132)

**Step 1: Write the failing test**

Add to `src/unit_tests/rules/graph.rs`:

```rust
#[test]
fn test_resolved_path_basic_structure() {
    use crate::rules::graph::{ResolvedPath, ReductionStep, EdgeKind};
    use std::collections::BTreeMap;

    let steps = vec![
        ReductionStep {
            name: "A".to_string(),
            variant: BTreeMap::from([("graph".to_string(), "SimpleGraph".to_string())]),
        },
        ReductionStep {
            name: "B".to_string(),
            variant: BTreeMap::from([("weight".to_string(), "f64".to_string())]),
        },
    ];
    let edges = vec![EdgeKind::Reduction {
        overhead: Default::default(),
    }];
    let path = ResolvedPath {
        steps: steps.clone(),
        edges,
    };

    assert_eq!(path.len(), 1);
    assert_eq!(path.num_reductions(), 1);
    assert_eq!(path.num_casts(), 0);
    assert_eq!(path.steps[0].name, "A");
    assert_eq!(path.steps[1].name, "B");
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_resolved_path_basic_structure -- --no-run 2>&1`
Expected: Compilation error — `ResolvedPath`, `ReductionStep`, `EdgeKind` not defined.

**Step 3: Write the types**

Add to `src/rules/graph.rs` after the `ReductionPath` impl block (after line 132):

```rust
/// A node in a variant-level reduction path.
#[derive(Debug, Clone, Serialize)]
pub struct ReductionStep {
    /// Problem name (e.g., "MaximumIndependentSet").
    pub name: String,
    /// Variant at this point (e.g., {"graph": "GridGraph", "weight": "i32"}).
    pub variant: std::collections::BTreeMap<String, String>,
}

/// The kind of transition between adjacent steps in a resolved path.
#[derive(Debug, Clone, Serialize)]
pub enum EdgeKind {
    /// A registered reduction (backed by a ReduceTo impl).
    Reduction {
        /// Overhead from the matching ReductionEntry.
        overhead: ReductionOverhead,
    },
    /// A natural cast via subtype relaxation. Identity overhead.
    NaturalCast,
}

/// A fully resolved reduction path with variant information at each node.
///
/// Created by [`ReductionGraph::resolve_path`] from a name-level [`ReductionPath`].
/// Each adjacent pair of steps is connected by an [`EdgeKind`]: either a registered
/// reduction or a natural cast (subtype relaxation with identity overhead).
#[derive(Debug, Clone, Serialize)]
pub struct ResolvedPath {
    /// Sequence of (name, variant) nodes.
    pub steps: Vec<ReductionStep>,
    /// Edge kinds between adjacent steps. Length = steps.len() - 1.
    pub edges: Vec<EdgeKind>,
}

impl ResolvedPath {
    /// Number of edges (reductions + casts) in the path.
    pub fn len(&self) -> usize {
        self.edges.len()
    }

    /// Whether the path is empty.
    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }

    /// Number of registered reduction steps (excludes natural casts).
    pub fn num_reductions(&self) -> usize {
        self.edges
            .iter()
            .filter(|e| matches!(e, EdgeKind::Reduction { .. }))
            .count()
    }

    /// Number of natural cast steps.
    pub fn num_casts(&self) -> usize {
        self.edges
            .iter()
            .filter(|e| matches!(e, EdgeKind::NaturalCast))
            .count()
    }
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test test_resolved_path_basic_structure`
Expected: PASS

**Step 5: Commit**

```bash
git add src/rules/graph.rs src/unit_tests/rules/graph.rs
git commit -m "feat: add ResolvedPath, ReductionStep, EdgeKind types"
```

---

### Task 2: Add helper to find matching ReductionEntry candidates

The resolver needs to iterate `inventory::iter::<ReductionEntry>` filtered by name pair, check variant compatibility, and pick the most specific match. Add this as a private helper on `ReductionGraph`.

**Files:**
- Modify: `src/rules/graph.rs` (inside the second `impl ReductionGraph` block that contains `is_variant_reducible`, after line 618)

**Step 1: Write the failing test**

Add to `src/unit_tests/rules/graph.rs`:

```rust
#[test]
fn test_find_matching_entry_ksat_k3() {
    let graph = ReductionGraph::new();
    let variant_k3: std::collections::BTreeMap<String, String> =
        [("k".to_string(), "3".to_string())].into();

    let entry = graph.find_best_entry("KSatisfiability", "QUBO", &variant_k3);
    assert!(entry.is_some());
    let (source_var, _target_var, overhead) = entry.unwrap();
    // K=3 overhead has num_clauses term; K=2 does not
    assert!(overhead
        .output_size
        .iter()
        .any(|(field, _)| *field == "num_vars"));
    // K=3 overhead: poly!(num_vars) + poly!(num_clauses) → two terms total
    let num_vars_poly = &overhead
        .output_size
        .iter()
        .find(|(f, _)| *f == "num_vars")
        .unwrap()
        .1;
    assert!(
        num_vars_poly.terms.len() >= 2,
        "K=3 overhead should have num_vars + num_clauses"
    );
}

#[test]
fn test_find_matching_entry_ksat_k2() {
    let graph = ReductionGraph::new();
    let variant_k2: std::collections::BTreeMap<String, String> =
        [("k".to_string(), "2".to_string())].into();

    let entry = graph.find_best_entry("KSatisfiability", "QUBO", &variant_k2);
    assert!(entry.is_some());
    let (_source_var, _target_var, overhead) = entry.unwrap();
    // K=2 overhead: just poly!(num_vars) → one term
    let num_vars_poly = &overhead
        .output_size
        .iter()
        .find(|(f, _)| *f == "num_vars")
        .unwrap()
        .1;
    assert_eq!(
        num_vars_poly.terms.len(),
        1,
        "K=2 overhead should have only num_vars"
    );
}

#[test]
fn test_find_matching_entry_no_match() {
    let graph = ReductionGraph::new();
    let variant: std::collections::BTreeMap<String, String> =
        [("k".to_string(), "99".to_string())].into();

    // k=99 is not a subtype of k=2 or k=3
    let entry = graph.find_best_entry("KSatisfiability", "QUBO", &variant);
    assert!(entry.is_none());
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_find_matching_entry -- --no-run 2>&1`
Expected: Compilation error — `find_best_entry` method not found.

**Step 3: Implement `find_best_entry`**

Add to `src/rules/graph.rs` in the `impl ReductionGraph` block that contains `is_variant_reducible` (after `is_variant_reducible` at ~line 618):

```rust
    /// Find the best matching `ReductionEntry` for a (source_name, target_name) pair
    /// given the caller's current source variant.
    ///
    /// "Best" means: compatible (current variant is reducible to the entry's source variant)
    /// and most specific (tightest fit among all compatible entries).
    ///
    /// Returns `(entry_source_variant, entry_target_variant, overhead)` or `None`.
    pub fn find_best_entry(
        &self,
        source_name: &str,
        target_name: &str,
        current_variant: &std::collections::BTreeMap<String, String>,
    ) -> Option<(
        std::collections::BTreeMap<String, String>,
        std::collections::BTreeMap<String, String>,
        ReductionOverhead,
    )> {
        use crate::rules::registry::ReductionEntry;

        let mut best: Option<(
            std::collections::BTreeMap<String, String>,
            std::collections::BTreeMap<String, String>,
            ReductionOverhead,
        )> = None;

        for entry in inventory::iter::<ReductionEntry> {
            if entry.source_name != source_name || entry.target_name != target_name {
                continue;
            }

            let entry_source = Self::variant_to_map(&entry.source_variant());
            let entry_target = Self::variant_to_map(&entry.target_variant());

            // Check: current_variant is reducible to entry's source variant
            // (current is equal-or-more-specific on every axis)
            if current_variant != &entry_source
                && !self.is_variant_reducible(current_variant, &entry_source)
            {
                continue;
            }

            // Pick the most specific: if we already have a best, prefer the one
            // whose source_variant is more specific (tighter fit)
            let dominated = if let Some((ref best_source, _, _)) = best {
                // New entry is more specific than current best?
                self.is_variant_reducible(&entry_source, best_source)
                    || entry_source == *current_variant
            } else {
                true
            };

            if dominated {
                best = Some((entry_source, entry_target, entry.overhead()));
            }
        }

        best
    }
```

**Step 4: Run tests to verify they pass**

Run: `cargo test test_find_matching_entry`
Expected: All 3 tests PASS.

**Step 5: Commit**

```bash
git add src/rules/graph.rs src/unit_tests/rules/graph.rs
git commit -m "feat: add find_best_entry for variant-aware ReductionEntry lookup"
```

---

### Task 3: Implement `resolve_path`

**Files:**
- Modify: `src/rules/graph.rs` (add method to `ReductionGraph`, near `find_best_entry`)

**Step 1: Write the failing tests**

Add to `src/unit_tests/rules/graph.rs`:

```rust
#[test]
fn test_resolve_path_direct_same_variant() {
    use std::collections::BTreeMap;
    let graph = ReductionGraph::new();

    // MIS(SimpleGraph, i32) → VC(SimpleGraph, i32) — no cast needed
    let name_path = graph
        .find_shortest_path::<
            MaximumIndependentSet<SimpleGraph, i32>,
            MinimumVertexCover<SimpleGraph, i32>,
        >()
        .unwrap();

    let source_variant = BTreeMap::from([
        ("graph".to_string(), "SimpleGraph".to_string()),
        ("weight".to_string(), "i32".to_string()),
    ]);
    let target_variant = BTreeMap::from([
        ("graph".to_string(), "SimpleGraph".to_string()),
        ("weight".to_string(), "i32".to_string()),
    ]);

    let resolved = graph
        .resolve_path(&name_path, &source_variant, &target_variant)
        .unwrap();

    assert_eq!(resolved.num_reductions(), 1);
    assert_eq!(resolved.num_casts(), 0);
    assert_eq!(resolved.steps.len(), 2);
    assert_eq!(resolved.steps[0].name, "MaximumIndependentSet");
    assert_eq!(resolved.steps[1].name, "MinimumVertexCover");
}

#[test]
fn test_resolve_path_with_natural_cast() {
    use std::collections::BTreeMap;
    use crate::topology::GridGraph;
    let graph = ReductionGraph::new();

    // MIS(GridGraph) → VC(SimpleGraph) — needs a natural cast MIS(GridGraph)→MIS(SimpleGraph)
    let name_path = graph
        .find_shortest_path::<
            MaximumIndependentSet<GridGraph, i32>,
            MinimumVertexCover<SimpleGraph, i32>,
        >()
        .unwrap();

    let source_variant = BTreeMap::from([
        ("graph".to_string(), "GridGraph".to_string()),
        ("weight".to_string(), "i32".to_string()),
    ]);
    let target_variant = BTreeMap::from([
        ("graph".to_string(), "SimpleGraph".to_string()),
        ("weight".to_string(), "i32".to_string()),
    ]);

    let resolved = graph
        .resolve_path(&name_path, &source_variant, &target_variant)
        .unwrap();

    // Should be: MIS(GridGraph) --NaturalCast--> MIS(SimpleGraph) --Reduction--> VC(SimpleGraph)
    assert_eq!(resolved.num_reductions(), 1);
    assert_eq!(resolved.num_casts(), 1);
    assert_eq!(resolved.steps.len(), 3);
    assert_eq!(resolved.steps[0].name, "MaximumIndependentSet");
    assert_eq!(
        resolved.steps[0].variant.get("graph").unwrap(),
        "GridGraph"
    );
    assert_eq!(resolved.steps[1].name, "MaximumIndependentSet");
    assert_eq!(
        resolved.steps[1].variant.get("graph").unwrap(),
        "SimpleGraph"
    );
    assert_eq!(resolved.steps[2].name, "MinimumVertexCover");
    assert!(matches!(resolved.edges[0], EdgeKind::NaturalCast));
    assert!(matches!(resolved.edges[1], EdgeKind::Reduction { .. }));
}

#[test]
fn test_resolve_path_ksat_disambiguates() {
    use std::collections::BTreeMap;
    use crate::rules::graph::EdgeKind;
    let graph = ReductionGraph::new();

    let name_path = graph
        .find_shortest_path_by_name("KSatisfiability", "QUBO")
        .unwrap();

    // Resolve with k=3
    let source_k3 = BTreeMap::from([("k".to_string(), "3".to_string())]);
    let target = BTreeMap::from([("weight".to_string(), "f64".to_string())]);

    let resolved_k3 = graph
        .resolve_path(&name_path, &source_k3, &target)
        .unwrap();
    assert_eq!(resolved_k3.num_reductions(), 1);

    // Extract overhead from the reduction edge
    let overhead_k3 = match &resolved_k3.edges.last().unwrap() {
        EdgeKind::Reduction { overhead } => overhead,
        _ => panic!("last edge should be Reduction"),
    };
    // K=3 overhead has 2 terms in num_vars polynomial
    let num_vars_poly_k3 = &overhead_k3
        .output_size
        .iter()
        .find(|(f, _)| *f == "num_vars")
        .unwrap()
        .1;
    assert!(num_vars_poly_k3.terms.len() >= 2);

    // Resolve with k=2
    let source_k2 = BTreeMap::from([("k".to_string(), "2".to_string())]);
    let resolved_k2 = graph
        .resolve_path(&name_path, &source_k2, &target)
        .unwrap();
    let overhead_k2 = match &resolved_k2.edges.last().unwrap() {
        EdgeKind::Reduction { overhead } => overhead,
        _ => panic!("last edge should be Reduction"),
    };
    let num_vars_poly_k2 = &overhead_k2
        .output_size
        .iter()
        .find(|(f, _)| *f == "num_vars")
        .unwrap()
        .1;
    assert_eq!(num_vars_poly_k2.terms.len(), 1);
}

#[test]
fn test_resolve_path_incompatible_returns_none() {
    use std::collections::BTreeMap;
    let graph = ReductionGraph::new();

    let name_path = graph
        .find_shortest_path_by_name("KSatisfiability", "QUBO")
        .unwrap();

    // k=99 matches neither k=2 nor k=3
    let source = BTreeMap::from([("k".to_string(), "99".to_string())]);
    let target = BTreeMap::from([("weight".to_string(), "f64".to_string())]);

    let resolved = graph.resolve_path(&name_path, &source, &target);
    assert!(resolved.is_none());
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test test_resolve_path -- --no-run 2>&1`
Expected: Compilation error — `resolve_path` method not found.

**Step 3: Implement `resolve_path`**

Add to `src/rules/graph.rs` in the same `impl ReductionGraph` block, after `find_best_entry`:

```rust
    /// Resolve a name-level [`ReductionPath`] into a variant-level [`ResolvedPath`].
    ///
    /// Walks the name-level path, threading variant state through each edge.
    /// For each step, picks the most-specific compatible `ReductionEntry` and
    /// inserts `NaturalCast` steps where the caller's variant is more specific
    /// than the rule's expected source variant.
    ///
    /// Returns `None` if no compatible reduction entry exists for any step.
    pub fn resolve_path(
        &self,
        path: &ReductionPath,
        source_variant: &std::collections::BTreeMap<String, String>,
        target_variant: &std::collections::BTreeMap<String, String>,
    ) -> Option<ResolvedPath> {
        if path.type_names.len() < 2 {
            return None;
        }

        let mut current_variant = source_variant.clone();
        let mut steps = vec![ReductionStep {
            name: path.type_names[0].to_string(),
            variant: current_variant.clone(),
        }];
        let mut edges = Vec::new();

        for i in 0..path.type_names.len() - 1 {
            let src_name = path.type_names[i];
            let dst_name = path.type_names[i + 1];

            let (entry_source, entry_target, overhead) =
                self.find_best_entry(src_name, dst_name, &current_variant)?;

            // Insert natural cast if current variant differs from entry's source
            if current_variant != entry_source {
                steps.push(ReductionStep {
                    name: src_name.to_string(),
                    variant: entry_source,
                });
                edges.push(EdgeKind::NaturalCast);
            }

            // Advance through the reduction
            current_variant = entry_target;
            steps.push(ReductionStep {
                name: dst_name.to_string(),
                variant: current_variant.clone(),
            });
            edges.push(EdgeKind::Reduction { overhead });
        }

        // Trailing natural cast if final variant differs from requested target
        if current_variant != *target_variant
            && self.is_variant_reducible(&current_variant, target_variant)
        {
            let last_name = path.type_names.last().unwrap();
            steps.push(ReductionStep {
                name: last_name.to_string(),
                variant: target_variant.clone(),
            });
            edges.push(EdgeKind::NaturalCast);
        }

        Some(ResolvedPath { steps, edges })
    }
```

**Step 4: Run tests to verify they pass**

Run: `cargo test test_resolve_path`
Expected: All 4 tests PASS.

**Step 5: Commit**

```bash
git add src/rules/graph.rs src/unit_tests/rules/graph.rs
git commit -m "feat: add resolve_path for variant-level reduction paths"
```

---

### Task 4: Deprecate `lookup_overhead` and migrate KSat example

**Files:**
- Modify: `src/export.rs:91-98` (add deprecation)
- Modify: `src/export.rs:100-103` (add deprecation)
- Modify: `examples/reduction_ksatisfiability_to_qubo.rs:120-121` (migrate to resolve_path)

**Step 1: Add deprecation annotations**

In `src/export.rs`, add `#[deprecated]` to both functions:

```rust
#[deprecated(
    since = "0.2.0",
    note = "Use ReductionGraph::resolve_path() for variant-aware overhead lookup"
)]
pub fn lookup_overhead(source_name: &str, target_name: &str) -> Option<ReductionOverhead> {
    // ... unchanged body ...
}

#[deprecated(
    since = "0.2.0",
    note = "Use ReductionGraph::resolve_path() for variant-aware overhead lookup"
)]
pub fn lookup_overhead_or_empty(source_name: &str, target_name: &str) -> ReductionOverhead {
    lookup_overhead(source_name, target_name).unwrap_or_default()
}
```

**Step 2: Migrate the KSat example**

In `examples/reduction_ksatisfiability_to_qubo.rs`, replace the `lookup_overhead` call (line 120-121) with `resolve_path`:

```rust
    // Resolve variant-aware overhead via resolve_path
    let rg = problemreductions::rules::graph::ReductionGraph::new();
    let name_path = rg
        .find_shortest_path_by_name("KSatisfiability", "QUBO")
        .expect("KSatisfiability -> QUBO path not found");
    let source_variant = variant_to_map(KSatisfiability::<3>::variant())
        .into_iter()
        .map(|(k, v)| (k, v))
        .collect::<std::collections::BTreeMap<_, _>>();
    let target_variant = variant_to_map(QUBO::<f64>::variant())
        .into_iter()
        .map(|(k, v)| (k, v))
        .collect::<std::collections::BTreeMap<_, _>>();
    let resolved = rg
        .resolve_path(&name_path, &source_variant, &target_variant)
        .expect("Failed to resolve KSatisfiability -> QUBO path");
    // Extract overhead from the reduction edge
    let overhead = match resolved.edges.iter().find_map(|e| match e {
        problemreductions::rules::graph::EdgeKind::Reduction { overhead } => Some(overhead),
        _ => None,
    }) {
        Some(o) => o.clone(),
        None => panic!("Resolved path has no reduction edge"),
    };
```

**Step 3: Verify the example still compiles and runs**

Run: `cargo build --example reduction_ksatisfiability_to_qubo --features ilp`
Expected: Builds (deprecation warnings for other examples are OK).

Run: `cargo run --example reduction_ksatisfiability_to_qubo --features ilp`
Expected: Runs successfully, produces JSON output.

**Step 4: Commit**

```bash
git add src/export.rs examples/reduction_ksatisfiability_to_qubo.rs
git commit -m "refactor: deprecate lookup_overhead, migrate KSat example to resolve_path"
```

---

### Task 5: Remove `impl_natural_reduction!` invocation from `natural.rs`

Now that `resolve_path` inserts natural casts automatically, the explicit natural reduction registration is no longer needed for planning. Remove it and update the test.

**Files:**
- Modify: `src/rules/natural.rs` (remove invocation)
- Modify: `src/unit_tests/rules/natural.rs` (update test to use resolve_path instead)

**Step 1: Update the test to verify natural casts via resolve_path**

Replace `src/unit_tests/rules/natural.rs` contents:

```rust
use crate::models::graph::MaximumIndependentSet;
use crate::rules::graph::{EdgeKind, ReductionGraph};
use crate::topology::{SimpleGraph, Triangular};
use crate::traits::Problem;
use std::collections::BTreeMap;

#[test]
fn test_natural_cast_triangular_to_simple_via_resolve() {
    let graph = ReductionGraph::new();

    // Find any path from MIS to itself (via VC round-trip) to test natural cast insertion
    // Instead, directly test that resolve_path inserts a natural cast for MIS(Triangular)→VC(SimpleGraph)
    let name_path = graph
        .find_shortest_path::<
            MaximumIndependentSet<Triangular, i32>,
            crate::models::graph::MinimumVertexCover<SimpleGraph, i32>,
        >()
        .unwrap();

    let source_variant = BTreeMap::from([
        ("graph".to_string(), "Triangular".to_string()),
        ("weight".to_string(), "i32".to_string()),
    ]);
    let target_variant = BTreeMap::from([
        ("graph".to_string(), "SimpleGraph".to_string()),
        ("weight".to_string(), "i32".to_string()),
    ]);

    let resolved = graph
        .resolve_path(&name_path, &source_variant, &target_variant)
        .unwrap();

    // Path should be: MIS(Triangular) --NaturalCast--> MIS(SimpleGraph) --Reduction--> VC(SimpleGraph)
    assert_eq!(resolved.num_casts(), 1);
    assert_eq!(resolved.num_reductions(), 1);
    assert!(matches!(resolved.edges[0], EdgeKind::NaturalCast));
    assert!(matches!(resolved.edges[1], EdgeKind::Reduction { .. }));
    assert_eq!(
        resolved.steps[0].variant.get("graph").unwrap(),
        "Triangular"
    );
    assert_eq!(
        resolved.steps[1].variant.get("graph").unwrap(),
        "SimpleGraph"
    );
}
```

**Step 2: Remove the `impl_natural_reduction!` invocation**

Update `src/rules/natural.rs` to:

```rust
//! Natural-edge reductions via graph subtype relaxation.
//!
//! Natural reductions (e.g., a problem on `Triangular` solved as `SimpleGraph`)
//! are handled automatically by [`ReductionGraph::resolve_path`], which inserts
//! `NaturalCast` steps based on the registered graph/weight subtype hierarchies.
//!
//! No explicit `ReduceTo` impls are needed for natural edges — the resolver
//! computes them from `GraphSubtypeEntry` and `WeightSubtypeEntry` registrations.

#[cfg(test)]
#[path = "../unit_tests/rules/natural.rs"]
mod tests;
```

**Step 3: Run the test**

Run: `cargo test test_natural_cast_triangular_to_simple_via_resolve`
Expected: PASS

**Step 4: Run full test suite to check nothing broke**

Run: `make test clippy`
Expected: PASS (some deprecation warnings for examples still using `lookup_overhead` are OK)

**Step 5: Commit**

```bash
git add src/rules/natural.rs src/unit_tests/rules/natural.rs
git commit -m "refactor: remove explicit natural reduction, rely on resolve_path"
```

---

### Task 6: Run full verification

**Files:** None (verification only)

**Step 1: Run full check**

Run: `make check`
Expected: fmt, clippy, and all tests pass.

**Step 2: Run doc build**

Run: `cargo doc --no-deps -p problemreductions`
Expected: No warnings.

**Step 3: Run examples that use lookup_overhead (should still work via deprecation)**

Run: `cargo build --examples --features ilp`
Expected: Builds with deprecation warnings but no errors.

**Step 4: Commit any fixups if needed, then final commit message**

```bash
git add -A
git commit -m "chore: final cleanup for variant-aware reduction paths"
```
