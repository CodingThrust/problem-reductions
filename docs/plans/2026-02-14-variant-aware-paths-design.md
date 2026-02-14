# Variant-Aware Reduction Paths

**Goal:** Make reduction paths variant-level so that (a) variant-specific reductions are disambiguated (issue 2) and (b) natural cast steps are computed automatically from subtype hierarchies (issue 5).

## Background

The runtime `ReductionGraph` uses name-only nodes. `ReductionPath` is `Vec<&'static str>` — it carries no variant information. This causes two problems:

1. **Overhead lookup ambiguity (issue 2):** `lookup_overhead("KSatisfiability", "QUBO")` returns the first hit from inventory. KSatisfiability<2>→QUBO and KSatisfiability<3>→QUBO have different overheads, but the caller can't distinguish them.

2. **Natural edge inconsistency (issue 5):** The JSON export infers 8 natural edges (e.g., MIS GridGraph→SimpleGraph) from subtype hierarchies, but only 1 has a backing `ReduceTo` impl. Users see edges in documentation that aren't executable.

## Design

### 1. `ResolvedPath` Data Model

A variant-level path where each node carries `(name, variant)` and each edge is typed:

```rust
/// A node in a variant-level reduction path.
#[derive(Debug, Clone, Serialize)]
pub struct ReductionStep {
    /// Problem name (e.g., "MaximumIndependentSet").
    pub name: String,
    /// Variant at this point (e.g., {"graph": "GridGraph", "weight": "i32"}).
    pub variant: BTreeMap<String, String>,
}

/// The kind of transition between adjacent steps.
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
#[derive(Debug, Clone, Serialize)]
pub struct ResolvedPath {
    /// Sequence of (name, variant) nodes.
    pub steps: Vec<ReductionStep>,
    /// Edge kinds between adjacent steps. Length = steps.len() - 1.
    pub edges: Vec<EdgeKind>,
}
```

Example — resolving `MIS(GridGraph, i32) → QUBO(f64)` through name-path `["MIS", "QUBO"]`:

```
steps:
  [0] MIS  {graph: "GridGraph",   weight: "i32"}   ← source
  [1] MIS  {graph: "SimpleGraph", weight: "i32"}   ← natural cast
  [2] QUBO {weight: "f64"}                         ← reduction target

edges:
  [0] NaturalCast                                  ← GridGraph <: SimpleGraph
  [1] Reduction { overhead: ... }                  ← MIS→QUBO rule
```

### 2. Resolution Algorithm

```rust
impl ReductionGraph {
    pub fn resolve_path(
        &self,
        path: &ReductionPath,
        source_variant: &BTreeMap<String, String>,
        target_variant: &BTreeMap<String, String>,
    ) -> Option<ResolvedPath> { ... }
}
```

Algorithm:

```
current_variant = source_variant
steps = [ Step(path[0], current_variant) ]
edges = []

for each edge (src_name → dst_name) in the name-level path:

    1. FIND CANDIDATES
       Collect all ReductionEntry where
         entry.source_name == src_name AND entry.target_name == dst_name

    2. FILTER COMPATIBLE
       Keep entries where current_variant is reducible to entry.source_variant
       (current is equal-or-more-specific on every variant axis)

    3. PICK MOST SPECIFIC
       Among compatible entries, pick the one whose source_variant is the
       tightest supertype of current_variant.
       If none compatible → return None.

    4. INSERT NATURAL CAST (if needed)
       If current_variant ≠ best_rule.source_variant:
         steps.push( Step(src_name, best_rule.source_variant) )
         edges.push( NaturalCast )

    5. ADVANCE
       current_variant = best_rule.target_variant
       steps.push( Step(dst_name, current_variant) )
       edges.push( Reduction { overhead: best_rule.overhead() } )

// Trailing natural cast if final variant differs from target
if current_variant ≠ target_variant
   AND is_variant_reducible(current_variant, target_variant):
     steps.push( Step(last_name, target_variant) )
     edges.push( NaturalCast )

return ResolvedPath { steps, edges }
```

### 3. KSat Disambiguation Example

Resolving `KSat(k=3) → QUBO` via name-path `["KSatisfiability", "QUBO"]`:

```
FIND CANDIDATES:
  - KSat<2>→QUBO  (source_variant: {k:"2"}, overhead: num_vars)
  - KSat<3>→QUBO  (source_variant: {k:"3"}, overhead: num_vars + num_clauses)

FILTER COMPATIBLE with current k=3:
  - KSat<2>: k=3 reducible to k=2? No (3 is not a subtype of 2)
  - KSat<3>: k=3 == k=3? Yes ✓

PICK: KSat<3>→QUBO with correct overhead.
```

Overhead ambiguity is resolved by construction — the resolver picks the exact matching entry.

### 4. Natural Edges Become Implicit

With `resolve_path`, natural casts are **computed from subtype hierarchies**, not registered as `ReduceTo` impls.

**Removed:**
- `impl_natural_reduction!` macro invocations (the one in `natural.rs` and any future ones)
- Natural edges no longer need `ReductionEntry` registration via inventory

**Kept:**
- `GraphSubtypeEntry` / `WeightSubtypeEntry` — source of truth for subtype relationships
- Inference logic in `to_json()` — unchanged, still produces natural edges in JSON export
- `GraphCast` trait — still needed for actual execution by callers

**Callers execute natural steps** using `GraphCast::cast_graph()` (or equivalent weight cast) directly, guided by the `EdgeKind::NaturalCast` marker in the resolved path. No `ReduceTo` dispatch needed.

### 5. `lookup_overhead` Deprecated

`lookup_overhead(source_name, target_name)` is replaced by per-step overhead in `ResolvedPath`:

```rust
impl ResolvedPath {
    /// Total overhead for the entire path (composed across all steps).
    pub fn total_overhead(&self) -> ReductionOverhead { ... }

    /// Number of reduction steps (excludes natural casts).
    pub fn num_reductions(&self) -> usize { ... }

    /// Number of natural cast steps.
    pub fn num_casts(&self) -> usize { ... }
}
```

Examples migrate from `lookup_overhead("A", "B")` to using the resolved path's overhead.

### 6. Backward Compatibility

| API | Change |
|-----|--------|
| `ReductionPath` | Unchanged — still returned by `find_paths`, `find_cheapest_path` |
| `find_paths`, `find_paths_by_name` | Unchanged |
| `find_cheapest_path` | Unchanged (name-level planning) |
| `has_direct_reduction` | Unchanged |
| `resolve_path` | **New** — lifts name-level path to variant-level |
| `ResolvedPath` | **New** |
| `lookup_overhead` | **Deprecated** — kept for one release, then removed |
| `lookup_overhead_or_empty` | **Deprecated** |
| `impl_natural_reduction!` | **Removed** after migration |

Existing code using `find_paths` + `lookup_overhead` continues working. New code should use `find_paths` + `resolve_path` for variant-correct results.

### 7. Files Changed

| File | Change |
|------|--------|
| `src/rules/graph.rs` | Add `ResolvedPath`, `ReductionStep`, `EdgeKind`, `resolve_path()` method |
| `src/export.rs` | Deprecate `lookup_overhead`, `lookup_overhead_or_empty` |
| `src/rules/natural.rs` | Remove `impl_natural_reduction!` invocation |
| `src/rules/mod.rs` | Keep `impl_natural_reduction!` macro (optional convenience), remove from prelude |
| `examples/reduction_ksatisfiability_to_qubo.rs` | Migrate from `lookup_overhead` to `resolve_path` |
| `examples/*.rs` | Migrate remaining examples (can be incremental) |
| `src/unit_tests/rules/graph.rs` | Add tests for `resolve_path` |
| `src/unit_tests/rules/natural.rs` | Update or remove natural reduction tests |

### 8. Non-Goals

- Runtime graph does not become variant-level (stays name-only for path discovery)
- No execution engine — `ResolvedPath` is a plan; callers dispatch `ReduceTo` and `GraphCast` themselves
- No changes to `to_json()` natural edge inference (it already works correctly)
- No changes to `#[reduction]` macro
