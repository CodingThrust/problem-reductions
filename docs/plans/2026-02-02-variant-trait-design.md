# Variant Trait Design

## Overview

Replace `type GraphType` and `type Weight` associated types in `Problem` trait with a single `fn variant()` method that returns extensible key-value attributes.

## Motivation

- Current design has fixed `GraphType` and `Weight` associated types
- Not extensible for other variant attributes (e.g., `k` for k-SAT, density)
- `variant()` method provides uniform, extensible interface

## Design

### Core Trait Changes

```rust
// src/traits.rs

pub trait Problem: Clone {
    /// Base name of this problem type (e.g., "IndependentSet").
    const NAME: &'static str;

    /// The type used for objective/size values.
    type Size: Clone + PartialOrd + Num + Zero + AddAssign;

    /// Returns attributes describing this problem variant.
    /// Each (key, value) pair describes a variant dimension.
    /// Common keys: "graph", "weight"
    fn variant() -> Vec<(&'static str, &'static str)>;

    fn num_variables(&self) -> usize;
    fn num_flavors(&self) -> usize;
    fn problem_size(&self) -> ProblemSize;
    fn energy_mode(&self) -> EnergyMode;
    fn solution_size(&self, config: &[usize]) -> SolutionSize<Self::Size>;
    // ... default methods unchanged
}
```

### Helper Function

```rust
// src/variant.rs

use std::any::type_name;

/// Extract short type name from full path.
/// e.g., "problemreductions::graph_types::SimpleGraph" -> "SimpleGraph"
pub fn short_type_name<T: 'static>() -> &'static str {
    let full = type_name::<T>();
    full.rsplit("::").next().unwrap_or(full)
}
```

### Problem Implementation Pattern

```rust
// src/models/graph/independent_set.rs

use crate::variant::short_type_name;

pub struct IndependentSet<W = Unweighted, G = SimpleGraph> {
    graph: UnGraph<(), ()>,
    weights: Vec<W>,
    _phantom: PhantomData<G>,
}

impl<W, G> Problem for IndependentSet<W, G>
where
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign + 'static,
    G: GraphMarker,
{
    const NAME: &'static str = "IndependentSet";
    type Size = W;

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("graph", short_type_name::<G>()),
            ("weight", short_type_name::<W>()),
        ]
    }

    // ... other methods unchanged
}
```

### Registry Updates

```rust
// src/rules/registry.rs

pub struct ReductionEntry {
    /// Base name of source problem (e.g., "IndependentSet").
    pub source_name: &'static str,
    /// Base name of target problem (e.g., "VertexCovering").
    pub target_name: &'static str,
    /// Source variant attributes.
    pub source_variant: &'static [(&'static str, &'static str)],
    /// Target variant attributes.
    pub target_variant: &'static [(&'static str, &'static str)],
    /// Function to create overhead information.
    pub overhead_fn: fn() -> ReductionOverhead,
}
```

### Reduction Graph JSON Format

```json
{
  "nodes": [
    {
      "name": "IndependentSet",
      "variant": {
        "graph": "SimpleGraph",
        "weight": "Unweighted"
      }
    },
    {
      "name": "VertexCovering",
      "variant": {
        "graph": "SimpleGraph",
        "weight": "Unweighted"
      }
    }
  ],
  "edges": [
    {
      "source": {
        "name": "IndependentSet",
        "variant": { "graph": "SimpleGraph", "weight": "Unweighted" }
      },
      "target": {
        "name": "VertexCovering",
        "variant": { "graph": "SimpleGraph", "weight": "Unweighted" }
      }
    }
  ]
}
```

## Changes Summary

### Remove
- `type GraphType: GraphMarker` from `Problem` trait
- `type Weight: NumericWeight` from `Problem` trait
- `GraphMarker::NAME` constant
- `variant_id()` function
- `source_graph`, `target_graph`, `source_weighted`, `target_weighted` from `ReductionEntry`

### Add
- `fn variant() -> Vec<(&'static str, &'static str)>` to `Problem` trait
- `short_type_name<T>()` helper function in `src/variant.rs`
- `source_variant`, `target_variant` fields in `ReductionEntry`

### Keep
- `GraphMarker` trait (for subtype relationships and type bounds)
- `NumericWeight` trait (for type bounds)
- Type parameters on structs (e.g., `IndependentSet<W, G>`)

### Update
- All `Problem` implementations to add `variant()` method
- Reduction graph JSON to use structured variant dict
- Graph building code in `src/rules/graph.rs`
- `#[reduction]` macro in `problemreductions-macros`

## Files Affected

1. `src/traits.rs` - Remove associated types, add `variant()` method
2. `src/variant.rs` - New file with `short_type_name()` helper
3. `src/graph_types.rs` - Remove `NAME` from `GraphMarker`
4. `src/rules/registry.rs` - Update `ReductionEntry` structure
5. `src/rules/graph.rs` - Update graph building to use structured variants
6. `src/models/**/*.rs` - Update all Problem implementations (~20 files)
7. `problemreductions-macros/src/lib.rs` - Update reduction macro
8. `docs/paper/reduction_graph.json` - Regenerate with new format
