# Problem Variants in Reduction Diagram

**Date:** 2026-02-02
**Status:** Approved

## Overview

Show problem variants (different graph types, weighted/unweighted) as separate nodes in the reduction diagram. Variants are positioned directly below their parent problem.

## Naming Convention

- Base problem (SimpleGraph + Unweighted): `IndependentSet`
- Graph variant only: `IndependentSet/GridGraph`
- Weight variant only: `IndependentSet/Weighted`
- Both variants: `IndependentSet/GridGraph/Weighted`

Default types (SimpleGraph, Unweighted) are omitted from the ID.

## The `Unweighted` Marker Type

```rust
/// Marker type for unweighted problems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Unweighted;
```

Problems explicitly typed as:
- `IndependentSet<Unweighted>` - unweighted (default)
- `IndependentSet<i32>` - weighted with integer weights

## ReductionEntry Changes

```rust
pub struct ReductionEntry {
    pub source_name: &'static str,
    pub target_name: &'static str,
    pub source_graph: &'static str,
    pub target_graph: &'static str,
    pub source_weighted: bool,      // NEW
    pub target_weighted: bool,      // NEW
    pub overhead_fn: fn() -> ReductionOverhead,
}
```

Helper method generates variant IDs:
```rust
fn variant_id(name: &str, graph: &str, weighted: bool) -> String {
    let mut id = name.to_string();
    if graph != "SimpleGraph" && graph != "CNF" && graph != "SetSystem" {
        id.push('/');
        id.push_str(graph);
    }
    if weighted {
        id.push_str("/Weighted");
    }
    id
}
```

## JSON Schema Extension

```json
{
  "nodes": [
    {"id": "IndependentSet", "label": "IndependentSet", "category": "graph",
     "parent": null, "graph_type": "SimpleGraph", "weighted": false},
    {"id": "IndependentSet/GridGraph/Weighted", "label": "GridGraph/Weighted",
     "category": "graph", "parent": "IndependentSet",
     "graph_type": "GridGraph", "weighted": true}
  ]
}
```

## Diagram Layout

Variants positioned 0.5 units below parent, offset horizontally if multiple:
```
IndependentSet ←→ VertexCovering
      │
  GridGraph/
  Weighted
```

## Implementation Order

1. Add `Unweighted` type in `src/types.rs`
2. Extend `ReductionEntry` in `src/rules/registry.rs`
3. Update all reduction registrations (~12 files)
4. Update graph generation in `src/rules/graph.rs`
5. Update Typst diagram in `docs/paper/reduction-diagram.typ`
6. Regenerate with `make export-graph && make paper`

## Files to Modify

- `src/types.rs` - Add `Unweighted` struct
- `src/rules/registry.rs` - Extend `ReductionEntry`
- `src/rules/graph.rs` - Update JSON generation
- `src/rules/*.rs` - All reduction files (~12)
- `docs/paper/reduction-diagram.typ` - Auto-position variants
