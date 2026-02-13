# Natural Variant Reduction Edges — Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Auto-generate natural reduction edges in the JSON export between variant nodes of the same problem type when all variant fields of the source are transitively more restrictive than the target.

**Architecture:** Fix the graph type hierarchy (add GridGraph, HyperGraph; remove wrong UnitDiskGraph=>PlanarGraph). Add a parallel weight type hierarchy. Register concrete variant nodes. In `to_json()`, detect transitive reducibility between same-name nodes and emit natural edges.

**Tech Stack:** Rust, inventory crate for compile-time collection, serde_json for export.

---

### Task 1: Fix Graph Type Hierarchy

**Files:**
- Modify: `src/graph_types.rs`
- Modify: `src/unit_tests/graph_types.rs`

**Step 1: Write failing tests for the corrected hierarchy**

Add to `src/unit_tests/graph_types.rs`:

```rust
#[test]
fn test_gridgraph_subtypes() {
    fn assert_subtype<A: GraphSubtype<B>, B: GraphMarker>() {}

    assert_subtype::<GridGraph, UnitDiskGraph>();
    assert_subtype::<GridGraph, SimpleGraph>();
    assert_subtype::<GridGraph, HyperGraph>();
}

#[test]
fn test_hypergraph_subtypes() {
    fn assert_subtype<A: GraphSubtype<B>, B: GraphMarker>() {}

    assert_subtype::<SimpleGraph, HyperGraph>();
    assert_subtype::<PlanarGraph, HyperGraph>();
    assert_subtype::<UnitDiskGraph, HyperGraph>();
    assert_subtype::<BipartiteGraph, HyperGraph>();
    assert_subtype::<GridGraph, HyperGraph>();
}

#[test]
fn test_gridgraph_entries_registered() {
    let entries: Vec<_> = inventory::iter::<GraphSubtypeEntry>().collect();
    assert!(entries
        .iter()
        .any(|e| e.subtype == "GridGraph" && e.supertype == "UnitDiskGraph"));
}

#[test]
fn test_hypergraph_entries_registered() {
    let entries: Vec<_> = inventory::iter::<GraphSubtypeEntry>().collect();
    assert!(entries
        .iter()
        .any(|e| e.subtype == "SimpleGraph" && e.supertype == "HyperGraph"));
}
```

Also update existing `test_declared_subtypes` to remove the `UnitDiskGraph, PlanarGraph` assertion and add new ones. Update `test_unit_disk_to_planar_registered` to assert it is NOT registered.

**Step 2: Run tests to verify they fail**

Run: `cargo test --lib graph_types`
Expected: FAIL — `GridGraph` and `HyperGraph` not found as marker types.

**Step 3: Implement the hierarchy changes**

In `src/graph_types.rs`:

1. Add `GridGraph` and `HyperGraph` marker types:
```rust
/// Grid graph - vertices on a grid, edges to neighbors.
#[derive(Debug, Clone, Copy, Default)]
pub struct GridGraph;

impl GraphMarker for GridGraph {}

/// Hypergraph - most general graph type. Edges can connect any number of vertices.
#[derive(Debug, Clone, Copy, Default)]
pub struct HyperGraph;

impl GraphMarker for HyperGraph {}
```

2. Replace the hierarchy declarations:
```rust
// Corrected hierarchy:
//   HyperGraph (most general)
//   └── SimpleGraph
//       ├── PlanarGraph
//       ├── BipartiteGraph
//       └── UnitDiskGraph
//           └── GridGraph
declare_graph_subtype!(GridGraph => UnitDiskGraph);
declare_graph_subtype!(GridGraph => SimpleGraph);        // transitive
declare_graph_subtype!(GridGraph => HyperGraph);          // transitive
declare_graph_subtype!(UnitDiskGraph => SimpleGraph);
declare_graph_subtype!(UnitDiskGraph => HyperGraph);      // transitive
declare_graph_subtype!(PlanarGraph => SimpleGraph);
declare_graph_subtype!(PlanarGraph => HyperGraph);        // transitive
declare_graph_subtype!(BipartiteGraph => SimpleGraph);
declare_graph_subtype!(BipartiteGraph => HyperGraph);     // transitive
declare_graph_subtype!(SimpleGraph => HyperGraph);
```

**Step 4: Run tests to verify they pass**

Run: `cargo test --lib graph_types`
Expected: PASS

**Step 5: Commit**

```bash
git add src/graph_types.rs src/unit_tests/graph_types.rs
git commit -m "fix: correct graph type hierarchy, add GridGraph and HyperGraph markers"
```

---

### Task 2: Add Weight Type Hierarchy

**Files:**
- Modify: `src/graph_types.rs`
- Modify: `src/unit_tests/graph_types.rs`
- Modify: `src/rules/graph.rs` (build weight_hierarchy)
- Modify: `src/unit_tests/rules/graph.rs`

**Step 1: Write failing tests**

In `src/unit_tests/graph_types.rs`:
```rust
#[test]
fn test_weight_subtype_entries_registered() {
    let entries: Vec<_> = inventory::iter::<WeightSubtypeEntry>().collect();
    assert!(entries
        .iter()
        .any(|e| e.subtype == "Unweighted" && e.supertype == "i32"));
    assert!(entries
        .iter()
        .any(|e| e.subtype == "i32" && e.supertype == "f64"));
    assert!(entries
        .iter()
        .any(|e| e.subtype == "Unweighted" && e.supertype == "f64"));
}
```

In `src/unit_tests/rules/graph.rs`:
```rust
#[test]
fn test_weight_hierarchy_built() {
    let graph = ReductionGraph::new();
    let hierarchy = graph.weight_hierarchy();
    assert!(
        hierarchy
            .get("Unweighted")
            .map(|s| s.contains("i32"))
            .unwrap_or(false),
        "Unweighted should have i32 as supertype"
    );
    assert!(
        hierarchy
            .get("i32")
            .map(|s| s.contains("f64"))
            .unwrap_or(false),
        "i32 should have f64 as supertype"
    );
    assert!(
        hierarchy
            .get("Unweighted")
            .map(|s| s.contains("f64"))
            .unwrap_or(false),
        "Unweighted should transitively have f64 as supertype"
    );
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test --lib weight_subtype weight_hierarchy`
Expected: FAIL — `WeightSubtypeEntry` does not exist.

**Step 3: Implement weight hierarchy**

In `src/graph_types.rs`, add:
```rust
/// Runtime registration of weight subtype relationships.
pub struct WeightSubtypeEntry {
    pub subtype: &'static str,
    pub supertype: &'static str,
}

inventory::collect!(WeightSubtypeEntry);

/// Macro to declare weight subtype relationships (runtime only, no compile-time trait).
#[macro_export]
macro_rules! declare_weight_subtype {
    ($sub:expr => $sup:expr) => {
        ::inventory::submit! {
            $crate::graph_types::WeightSubtypeEntry {
                subtype: $sub,
                supertype: $sup,
            }
        }
    };
}

declare_weight_subtype!("Unweighted" => "i32");
declare_weight_subtype!("Unweighted" => "f64");  // transitive
declare_weight_subtype!("i32" => "f64");
```

In `src/rules/graph.rs`, add `weight_hierarchy` field to `ReductionGraph` and build it in `new()`:
```rust
// In ReductionGraph struct:
weight_hierarchy: HashMap<&'static str, HashSet<&'static str>>,

// In new():
let weight_hierarchy = Self::build_weight_hierarchy();

// New method:
fn build_weight_hierarchy() -> HashMap<&'static str, HashSet<&'static str>> {
    let mut supertypes: HashMap<&'static str, HashSet<&'static str>> = HashMap::new();
    for entry in inventory::iter::<WeightSubtypeEntry> {
        supertypes.entry(entry.subtype).or_default().insert(entry.supertype);
    }
    // Same transitive closure as build_graph_hierarchy
    loop {
        let mut changed = false;
        let types: Vec<_> = supertypes.keys().copied().collect();
        for sub in &types {
            let current: Vec<_> = supertypes.get(sub).map(|s| s.iter().copied().collect()).unwrap_or_default();
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

// Public accessor:
pub fn weight_hierarchy(&self) -> &HashMap<&'static str, HashSet<&'static str>> {
    &self.weight_hierarchy
}

// Weight subtype check:
pub fn is_weight_subtype(&self, sub: &str, sup: &str) -> bool {
    sub == sup
        || self.weight_hierarchy.get(sub).map(|s| s.contains(sup)).unwrap_or(false)
}
```

**Step 4: Run tests to verify they pass**

Run: `cargo test --lib weight_subtype weight_hierarchy`
Expected: PASS

**Step 5: Commit**

```bash
git add src/graph_types.rs src/unit_tests/graph_types.rs src/rules/graph.rs src/unit_tests/rules/graph.rs
git commit -m "feat: add weight type hierarchy (Unweighted => i32 => f64)"
```

---

### Task 3: Add Concrete Variant Registration

**Files:**
- Modify: `src/rules/registry.rs`
- Modify: `src/rules/graph.rs` (register_types and to_json)
- Modify: `src/unit_tests/rules/graph.rs`

**Step 1: Write failing test**

In `src/unit_tests/rules/graph.rs`:
```rust
#[test]
fn test_concrete_variant_nodes_in_json() {
    let graph = ReductionGraph::new();
    let json = graph.to_json();

    // GridGraph variants should appear as nodes
    let gridgraph_node = json.nodes.iter().any(|n| {
        n.name == "MaximumIndependentSet"
            && n.variant.get("graph") == Some(&"GridGraph".to_string())
    });
    assert!(gridgraph_node, "MIS/GridGraph node should exist");
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test --lib test_concrete_variant_nodes`
Expected: FAIL — no GridGraph variant node exists.

**Step 3: Implement ConcreteVariantEntry and registrations**

In `src/rules/registry.rs`, add:
```rust
/// A registered concrete problem variant (for JSON export nodes).
/// Variants registered here appear as nodes even without explicit reduction rules.
pub struct ConcreteVariantEntry {
    pub name: &'static str,
    pub variant: &'static [(&'static str, &'static str)],
}

inventory::collect!(ConcreteVariantEntry);
```

In `src/rules/graph.rs`, in `register_types()`, add variant registrations after the existing `register!` block. Use `inventory::submit!` for each concrete variant that should appear:

```rust
fn register_variants() {
    // These are registered via inventory::submit! at module level, not inside a function.
    // See the submit! blocks below register_types.
}
```

Actually, add the variant submissions at module level in `src/rules/graph.rs` (outside functions):

```rust
// Register concrete variants for graph problems that support non-SimpleGraph types.
// These generate nodes in the JSON export.
inventory::submit! { ConcreteVariantEntry { name: "MaximumIndependentSet", variant: &[("graph", "GridGraph"), ("weight", "Unweighted")] } }
inventory::submit! { ConcreteVariantEntry { name: "MaximumIndependentSet", variant: &[("graph", "UnitDiskGraph"), ("weight", "Unweighted")] } }
inventory::submit! { ConcreteVariantEntry { name: "MaxCut", variant: &[("graph", "GridGraph"), ("weight", "Unweighted")] } }
inventory::submit! { ConcreteVariantEntry { name: "SpinGlass", variant: &[("graph", "GridGraph"), ("weight", "f64")] } }
```

In `to_json()`, collect nodes from `ConcreteVariantEntry` in addition to `ReductionEntry`:

```rust
// After collecting from ReductionEntry, also collect from ConcreteVariantEntry
for entry in inventory::iter::<ConcreteVariantEntry> {
    node_set.insert((
        entry.name.to_string(),
        Self::variant_to_map(entry.variant),
    ));
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test --lib test_concrete_variant_nodes`
Expected: PASS

**Step 5: Commit**

```bash
git add src/rules/registry.rs src/rules/graph.rs src/unit_tests/rules/graph.rs
git commit -m "feat: add ConcreteVariantEntry for non-SimpleGraph variant nodes"
```

---

### Task 4: Auto-Generate Natural Edges in `to_json()`

**Files:**
- Modify: `src/rules/graph.rs` (to_json method)
- Modify: `src/unit_tests/rules/graph.rs`

**Step 1: Write failing tests for natural edges**

In `src/unit_tests/rules/graph.rs`:
```rust
#[test]
fn test_natural_edge_graph_relaxation() {
    let graph = ReductionGraph::new();
    let json = graph.to_json();

    // MIS/GridGraph -> MIS/SimpleGraph should exist (graph type relaxation)
    let has_edge = json.edges.iter().any(|e| {
        e.source.name == "MaximumIndependentSet"
            && e.target.name == "MaximumIndependentSet"
            && e.source.variant.get("graph") == Some(&"GridGraph".to_string())
            && e.target.variant.get("graph") == Some(&"SimpleGraph".to_string())
            && e.source.variant.get("weight") == e.target.variant.get("weight")
    });
    assert!(has_edge, "Natural edge MIS/GridGraph -> MIS/SimpleGraph should exist");
}

#[test]
fn test_natural_edge_weight_promotion() {
    let graph = ReductionGraph::new();
    let json = graph.to_json();

    // MIS{SimpleGraph, Unweighted} -> MIS{SimpleGraph, i32} should exist
    let has_edge = json.edges.iter().any(|e| {
        e.source.name == "MaximumIndependentSet"
            && e.target.name == "MaximumIndependentSet"
            && e.source.variant.get("graph") == Some(&"SimpleGraph".to_string())
            && e.target.variant.get("graph") == Some(&"SimpleGraph".to_string())
            && e.source.variant.get("weight") == Some(&"Unweighted".to_string())
            && e.target.variant.get("weight") == Some(&"i32".to_string())
    });
    assert!(has_edge, "Natural edge MIS/Unweighted -> MIS/i32 should exist");
}

#[test]
fn test_no_natural_edge_wrong_direction() {
    let graph = ReductionGraph::new();
    let json = graph.to_json();

    // MIS/SimpleGraph -> MIS/GridGraph should NOT exist (wrong direction)
    let has_edge = json.edges.iter().any(|e| {
        e.source.name == "MaximumIndependentSet"
            && e.target.name == "MaximumIndependentSet"
            && e.source.variant.get("graph") == Some(&"SimpleGraph".to_string())
            && e.target.variant.get("graph") == Some(&"GridGraph".to_string())
    });
    assert!(!has_edge, "Should NOT have MIS/SimpleGraph -> MIS/GridGraph");
}

#[test]
fn test_no_natural_self_edge() {
    let graph = ReductionGraph::new();
    let json = graph.to_json();

    // No self-edges (same node to same node)
    for edge in &json.edges {
        assert!(
            edge.source != edge.target,
            "Should not have self-edge: {} {:?}",
            edge.source.name,
            edge.source.variant
        );
    }
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test --lib test_natural_edge`
Expected: FAIL — no natural edges exist yet.

**Step 3: Implement natural edge generation**

In `src/rules/graph.rs`, add a method to check if a variant is transitively reducible to another:

```rust
/// Check if variant A is strictly more restrictive than variant B (same problem name).
/// Returns true if every field of A is a subtype of (or equal to) the corresponding field in B,
/// and at least one field is strictly more restrictive.
fn is_variant_reducible(
    &self,
    a: &std::collections::BTreeMap<String, String>,
    b: &std::collections::BTreeMap<String, String>,
) -> bool {
    if a == b {
        return false; // No self-reduction
    }

    let mut all_compatible = true;
    let mut any_strict = false;

    // Check all fields in both variants
    let all_keys: std::collections::BTreeSet<_> = a.keys().chain(b.keys()).collect();

    for key in all_keys {
        let a_val = a.get(key.as_str()).map(|s| s.as_str()).unwrap_or("");
        let b_val = b.get(key.as_str()).map(|s| s.as_str()).unwrap_or("");

        if a_val == b_val {
            continue; // Equal on this field
        }

        // Check subtype relationship based on field type
        let is_sub = match key.as_str() {
            "graph" => self.is_graph_subtype(a_val, b_val),
            "weight" => self.is_weight_subtype(a_val, b_val),
            _ => false, // Unknown fields must be equal
        };

        if is_sub {
            any_strict = true;
        } else {
            all_compatible = false;
            break;
        }
    }

    all_compatible && any_strict
}
```

In `to_json()`, after building `edges`, add natural edge generation:

```rust
// Auto-generate natural edges between same-name variants
// Group nodes by name
let mut nodes_by_name: HashMap<&str, Vec<&std::collections::BTreeMap<String, String>>> = HashMap::new();
for (name, variant) in &node_set {
    if !variant.is_empty() {
        nodes_by_name.entry(name.as_str()).or_default().push(variant);
    }
}

// For each pair of same-name nodes, check transitive reducibility
for (name, variants) in &nodes_by_name {
    for a in variants {
        for b in variants {
            if self.is_variant_reducible(a, b) {
                let src_ref = VariantRef { name: name.to_string(), variant: (*a).clone() };
                let dst_ref = VariantRef { name: name.to_string(), variant: (*b).clone() };
                let key = (src_ref.clone(), dst_ref.clone());
                if edge_set.insert(key) {
                    // Identity overhead: each output field = same input field, p(x) = x
                    let overhead: Vec<OverheadFieldJson> = a.iter()
                        .filter(|(k, _)| *k != "graph" && *k != "weight")
                        .map(|(k, _)| OverheadFieldJson { field: k.clone(), formula: k.clone() })
                        .collect();
                    // For graph problems, carry through standard size fields
                    let overhead = if overhead.is_empty() {
                        // Infer from existing edges of same problem
                        // Fallback: emit common fields as identity
                        vec![]  // Will be populated from source node's known fields
                    } else {
                        overhead
                    };
                    edges.push(EdgeJson {
                        source: src_ref,
                        target: dst_ref,
                        overhead,
                        doc_path: String::new(), // No module path — natural reduction
                    });
                }
            }
        }
    }
}

// Re-sort edges after adding natural ones
edges.sort_by(|a, b| { ... });
```

**Step 4: Run tests to verify they pass**

Run: `cargo test --lib test_natural_edge`
Expected: PASS

**Step 5: Run full test suite**

Run: `cargo test`
Expected: PASS (existing tests may need minor adjustments for changed node/edge counts)

**Step 6: Commit**

```bash
git add src/rules/graph.rs src/unit_tests/rules/graph.rs
git commit -m "feat: auto-generate natural variant reduction edges in JSON export"
```

---

### Task 5: Update Existing Tests and Regenerate Graph

**Files:**
- Modify: `src/unit_tests/rules/graph.rs` (fix any broken count assertions)
- Regenerate: `docs/book/reductions/reduction_graph.json`

**Step 1: Run full test suite and fix any failures**

Run: `cargo test`

Likely fixes:
- `test_to_json`: node count assertion `json.nodes.len() >= 10` may need updating
- `test_to_json`: edge count assertion `json.edges.len() >= 10` may need updating
- `test_graph_hierarchy_built`: remove assertion about `UnitDiskGraph` having `PlanarGraph` as supertype (if present)
- `test_is_graph_subtype_direct`: remove `UnitDiskGraph, PlanarGraph` assertion
- `test_is_graph_subtype_transitive`: update comment and assertions
- `test_subtype_entries_registered`: count assertion `entries.len() >= 4` may need updating

**Step 2: Regenerate reduction graph JSON**

Run: `cargo run --example export_graph`

**Step 3: Verify the generated JSON contains natural edges**

Run: `cargo test`
Expected: PASS

**Step 4: Commit**

```bash
git add src/unit_tests/rules/graph.rs docs/book/reductions/reduction_graph.json
git commit -m "fix: update tests for corrected hierarchy, regenerate reduction graph"
```

---

### Task 6: Run clippy and full verification

**Files:** None (verification only)

**Step 1: Run clippy**

Run: `cargo clippy -- -D warnings`
Expected: PASS

**Step 2: Run full test suite**

Run: `cargo test`
Expected: PASS

**Step 3: Run fmt check**

Run: `cargo fmt --check`
Expected: PASS (or fix formatting)
