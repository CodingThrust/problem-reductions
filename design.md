# Variant-Aware Reduction Paths

## Problem

The `ReductionGraph` performs path-finding at the name level (e.g., `"KSatisfiability" -> "QUBO"`), but problems have **variants** -- parameterized by graph type, weight type, or constants like `k`. This causes two concrete issues:

1. **Overhead ambiguity**: `lookup_overhead("KSatisfiability", "QUBO")` returns whichever `ReductionEntry` inventory iterates first. But `KSatisfiability<2> -> QUBO` and `KSatisfiability<3> -> QUBO` have different overheads (the k=3 path introduces auxiliary variables via Rosenberg quadratization). Callers get the wrong overhead silently.

2. **Natural edge gap**: The JSON export infers natural edges (e.g., `MIS{GridGraph} -> MIS{SimpleGraph}`) from the graph/weight subtype hierarchy, but these edges had no runtime backing -- no `ReduceTo` impl existed for most of them. The documentation showed edges that couldn't be executed.

## Solution: Two-Phase Path Resolution

The design separates path-finding into two phases:

### Phase 1: Name-Level Path Discovery (unchanged)

Existing APIs (`find_paths`, `find_cheapest_path`, `find_shortest_path`) continue to operate on base problem names. The internal `petgraph` has one node per problem name and one edge per registered reduction. This is fast and sufficient for topology.

### Phase 2: Variant-Level Resolution (new)

A new `resolve_path` method lifts a name-level `ReductionPath` into a `ResolvedPath` that carries full variant information at every node:

```rust
pub fn resolve_path(
    &self,
    path: &ReductionPath,                           // name-level plan
    source_variant: &BTreeMap<String, String>,       // caller's concrete variant
    target_variant: &BTreeMap<String, String>,       // desired target variant
) -> Option<ResolvedPath>
```

The resolver walks the name-level path, threading variant state through each step:

1. **Find candidates** -- all `ReductionEntry` items matching `(src_name, dst_name)`.
2. **Filter compatible** -- keep entries where the current variant is equal-or-more-specific than the entry's source variant on every axis (graph, weight, k).
3. **Pick most specific** -- among compatible entries, choose the tightest fit.
4. **Insert natural cast** -- if the current variant is more specific than the chosen entry's source, emit a `NaturalCast` edge to relax the variant.
5. **Advance** -- update current variant to the entry's target variant, emit a `Reduction` edge carrying the correct overhead.

### Data Model

```
ResolvedPath
  steps: Vec<ReductionStep>    // (name, variant) at each node
  edges: Vec<EdgeKind>         // Reduction{overhead} | NaturalCast between steps
```

Example -- resolving `MIS(GridGraph, i32) -> MinimumVertexCover(SimpleGraph, i32)`:

```
steps:  MIS{GridGraph,i32}  --NaturalCast-->  MIS{SimpleGraph,i32}  --Reduction-->  VC{SimpleGraph,i32}
edges:  [NaturalCast,  Reduction{overhead: ...}]
```

### Subtype Hierarchies

Two hierarchies drive variant compatibility:

- **Graph**: `GridGraph <: UnitDiskGraph <: PlanarGraph <: SimpleGraph <: HyperGraph` (and `Triangular <: SimpleGraph`, `BipartiteGraph <: SimpleGraph`)
- **Weight**: `One <: i32 <: f64`
- **Constants** (k): a specific value like `"3"` is a subtype of `"N"` (generic)

Both are built from `GraphSubtypeEntry` / `WeightSubtypeEntry` inventory registrations with transitive closure computed at construction time.

### `find_best_entry` -- Variant-Aware Rule Matching

A new public method selects the best `ReductionEntry` for a `(source_name, target_name)` pair given a caller's current variant:

```rust
pub fn find_best_entry(
    &self,
    source_name: &str,
    target_name: &str,
    current_variant: &BTreeMap<String, String>,
) -> Option<(source_variant, target_variant, overhead)>
```

This resolves the KSat ambiguity: given `k=3`, it filters out the `k=2` entry and returns the `k=3`-specific overhead.

## Changes

### New Types (`src/rules/graph.rs`)

| Type | Purpose |
|------|---------|
| `ReductionStep` | `(name, variant)` node in a resolved path |
| `EdgeKind` | `Reduction{overhead}` or `NaturalCast` |
| `ResolvedPath` | Fully resolved variant-level path with helper methods (`len`, `num_reductions`, `num_casts`) |

### New Methods (`ReductionGraph`)

| Method | Purpose |
|--------|---------|
| `resolve_path` | Lift name-level path to variant-level |
| `find_best_entry` | Find most-specific compatible ReductionEntry for a variant |
| `is_variant_reducible` | Check if variant A is strictly more restrictive than B |
| `is_weight_subtype` | Weight hierarchy check (analogous to existing `is_graph_subtype`) |
| `weight_hierarchy` | Expose weight hierarchy for inspection |

### Deprecations (`src/export.rs`)

| Function | Replacement |
|----------|-------------|
| `lookup_overhead` | `resolve_path` -> extract overhead from `EdgeKind::Reduction` |
| `lookup_overhead_or_empty` | Same |

### Natural Reductions (`src/rules/natural.rs`)

The explicit `impl_natural_reduction!` invocation was removed. Natural casts are now computed implicitly by `resolve_path` from the subtype hierarchies. The `impl_natural_reduction!` macro itself is retained for callers who need concrete `ReduceTo` impls for specific subtype pairs.

### Example Migration

`examples/reduction_ksatisfiability_to_qubo.rs` was migrated from `lookup_overhead` to the new `resolve_path` API, demonstrating the correct pattern for obtaining variant-specific overhead.

All other examples received `#[allow(deprecated)]` annotations as a temporary measure; they still use `lookup_overhead` and will be migrated incrementally.

## Design Decisions

1. **Name-level graph is kept** -- variant-level Dijkstra would explode the node count (every `(name, graph, weight, k)` combination). Two-phase resolution keeps path discovery fast.

2. **No execution engine** -- `ResolvedPath` is a *plan*. Callers dispatch `ReduceTo` for `Reduction` edges and `GraphCast::cast_graph()` for `NaturalCast` edges themselves. This avoids type-erasure complexity.

3. **Natural edges stay in JSON export** -- `to_json()` continues to infer natural edges from subtype hierarchies for documentation. The resolved path makes them executable at runtime.

4. **Backward compatible** -- all existing `find_paths` / `find_cheapest_path` / `has_direct_reduction` APIs are unchanged. `lookup_overhead` is deprecated, not removed.
