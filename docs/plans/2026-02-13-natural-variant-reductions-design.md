# Natural Variant Reduction Edges

## Problem

The reduction graph export (`reduction_graph.json`) does not include "natural" reductions between variant nodes of the same problem. For example, `MaximumIndependentSet/GridGraph` should naturally reduce to `MaximumIndependentSet/SimpleGraph` because a GridGraph is a SimpleGraph. These edges should be auto-generated based on type hierarchies, not manually coded as `ReduceTo` impls.

## Design

### 1. Fix Graph Type Hierarchy (`src/graph_types.rs`)

**Bug fix**: Remove incorrect `UnitDiskGraph => PlanarGraph` relationship.

**Corrected hierarchy** (with all transitive relationships for compile-time trait bounds):

```
HyperGraph (most general)
└── SimpleGraph
    ├── PlanarGraph
    ├── BipartiteGraph
    └── UnitDiskGraph
        └── GridGraph
```

Declarations:
```rust
declare_graph_subtype!(GridGraph => UnitDiskGraph);
declare_graph_subtype!(GridGraph => SimpleGraph);       // transitive
declare_graph_subtype!(GridGraph => HyperGraph);         // transitive
declare_graph_subtype!(UnitDiskGraph => SimpleGraph);
declare_graph_subtype!(UnitDiskGraph => HyperGraph);     // transitive
declare_graph_subtype!(PlanarGraph => SimpleGraph);
declare_graph_subtype!(PlanarGraph => HyperGraph);       // transitive
declare_graph_subtype!(BipartiteGraph => SimpleGraph);
declare_graph_subtype!(BipartiteGraph => HyperGraph);    // transitive
declare_graph_subtype!(SimpleGraph => HyperGraph);
```

Add `GridGraph` and `HyperGraph` as `GraphMarker` types (marker structs + trait impls).

### 2. Weight Type Hierarchy (new)

Add `WeightSubtypeEntry` parallel to `GraphSubtypeEntry`:

```rust
pub struct WeightSubtypeEntry {
    pub subtype: &'static str,
    pub supertype: &'static str,
}
inventory::collect!(WeightSubtypeEntry);

macro_rules! declare_weight_subtype {
    ($sub:expr => $sup:expr) => {
        inventory::submit! {
            WeightSubtypeEntry {
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

`ReductionGraph` builds `weight_hierarchy` using the same transitive closure algorithm as `graph_hierarchy`.

### 3. Concrete Variant Registration (new)

Add `ConcreteVariantEntry` to register problem+variant combinations that exist as concrete types but have no explicit reduction rules:

```rust
pub struct ConcreteVariantEntry {
    pub name: &'static str,
    pub variant: &'static [(&'static str, &'static str)],
}
inventory::collect!(ConcreteVariantEntry);
```

Register concrete variants in `register_types!` (or a new `register_variants!` section):

```rust
// For each problem that supports non-SimpleGraph types:
submit_variant!("MaximumIndependentSet", &[("graph", "GridGraph"), ("weight", "Unweighted")]);
submit_variant!("MaximumIndependentSet", &[("graph", "UnitDiskGraph"), ("weight", "Unweighted")]);
submit_variant!("SpinGlass", &[("graph", "GridGraph"), ("weight", "f64")]);
// etc.
```

### 4. Auto-generate Natural Edges in `to_json()`

In `ReductionGraph::to_json()`, after collecting all nodes from reduction entries and concrete variant entries, add:

```
For every pair of nodes (A, B) with the same problem name:
  If A != B AND for EVERY variant field:
    field=="graph": A.graph is_subtype_of B.graph (using graph_hierarchy)
    field=="weight": A.weight is_subtype_of B.weight (using weight_hierarchy)
  Then emit a natural edge A -> B with:
    - overhead: identity mapping (each output field = same input field)
    - doc_path: "" (or a special marker like "natural")
```

### 5. Variant Comparison Rule

A variant A is "more restrictive" than B when ALL fields satisfy the subtype relationship:
- `graph`: checked against `graph_hierarchy`
- `weight`: checked against `weight_hierarchy`
- Other fields (e.g., `k`): must be equal (no hierarchy defined)

### 6. Files Changed

| File | Change |
|------|--------|
| `src/graph_types.rs` | Fix hierarchy, add GridGraph/HyperGraph markers, add WeightSubtypeEntry |
| `src/rules/graph.rs` | Build weight_hierarchy, generate natural edges in to_json(), collect ConcreteVariantEntry nodes |
| `src/rules/registry.rs` | Add ConcreteVariantEntry struct |
| `examples/export_graph.rs` | No changes needed (uses ReductionGraph::to_json()) |

### 7. Non-Goals

- No new `ReduceTo` impls for natural reductions (these are visualization-only edges in the JSON)
- No changes to runtime path finding (it already uses `rule_applicable` with graph hierarchy)
- No changes to the paper/Typst documentation initially
