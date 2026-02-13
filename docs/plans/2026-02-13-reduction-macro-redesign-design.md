# Reduction Macro Redesign: Dynamic Variant Extraction

## Problem

The `#[reduction]` proc macro infers variant information (graph type, weight type) from type parameters using heuristic pattern matching. This breaks when problem types don't fit the expected `Problem<G, W>` pattern:

- `KColoring<K, G>` — const generic `K` parsed as type param, `SimpleGraph` misidentified as weight
- `CircuitSAT` — no type params, defaults work but are fragile
- `KSatisfiability<K>` — literal `3` works, generic `K` doesn't
- `QUBO<W>` — single-param, needs special-case detection

The macro duplicates logic that `Problem::variant()` already handles correctly, leading to bugs and maintenance burden.

## Solution: Dynamic Function Pointers

Replace static variant inference with function pointers that call `Problem::variant()` at runtime.

### Changes

#### 1. `ReductionEntry` (src/rules/registry.rs)

Replace static `&'static [(&'static str, &'static str)]` fields with `fn()` pointers:

```rust
// Before:
pub struct ReductionEntry {
    pub source_name: &'static str,
    pub target_name: &'static str,
    pub source_variant: &'static [(&'static str, &'static str)],
    pub target_variant: &'static [(&'static str, &'static str)],
    pub overhead_fn: fn() -> ReductionOverhead,
    pub module_path: &'static str,
}

// After:
pub struct ReductionEntry {
    pub source_name: &'static str,
    pub target_name: &'static str,
    pub source_variant_fn: fn() -> Vec<(&'static str, &'static str)>,
    pub target_variant_fn: fn() -> Vec<(&'static str, &'static str)>,
    pub overhead_fn: fn() -> ReductionOverhead,
    pub module_path: &'static str,
}
```

#### 2. Macro Simplification (problemreductions-macros/src/lib.rs)

**Remove** (~120 lines):
- `extract_graph_type()` — graph type inference from first param
- `extract_weight_type()` — weight type inference from second param
- `is_weight_type()` — weight type name list
- `get_weight_name()` — type-to-name conversion
- `source_graph`, `target_graph`, `source_weighted`, `target_weighted` attributes from `ReductionAttrs`

**Keep**:
- `extract_type_name()` — still needed for `source_name`/`target_name`
- `extract_target_from_trait()` — still needed to get target type from `ReduceTo<T>`
- `overhead` attribute parsing
- `ReductionAttrs` (with only `overhead` remaining)

**Add**:
- Const generic detection: scan impl generics for `const K: usize` patterns
- Type substitution: replace const generic idents with `usize::MAX` in variant_fn calls

**Generated code**:

For `impl ReduceTo<QUBO<f64>> for MaximumIndependentSet<SimpleGraph, i32>`:
```rust
inventory::submit! {
    crate::rules::registry::ReductionEntry {
        source_name: "MaximumIndependentSet",
        target_name: "QUBO",
        source_variant_fn: || <MaximumIndependentSet<SimpleGraph, i32> as Problem>::variant(),
        target_variant_fn: || <QUBO<f64> as Problem>::variant(),
        overhead_fn: || { /* overhead */ },
        module_path: module_path!(),
    }
}
```

For `impl<const K: usize> ReduceTo<QUBO<f64>> for KColoring<K, SimpleGraph>`:
```rust
// K is detected as const generic → substituted with usize::MAX
// const_usize_str::<{usize::MAX}>() returns "N" → variant becomes ("k", "N")
inventory::submit! {
    crate::rules::registry::ReductionEntry {
        source_name: "KColoring",
        target_name: "QUBO",
        source_variant_fn: || <KColoring<{usize::MAX}, SimpleGraph> as crate::traits::Problem>::variant(),
        target_variant_fn: || <QUBO<f64> as crate::traits::Problem>::variant(),
        overhead_fn: || { /* overhead */ },
        module_path: module_path!(),
    }
}
```

For `impl<W: NumericSize> ReduceTo<MinimumVertexCover<SimpleGraph, W>> for MaximumIndependentSet<SimpleGraph, W>`:
```rust
// W is a type generic (not const) → use Unweighted as default representative
inventory::submit! {
    crate::rules::registry::ReductionEntry {
        source_name: "MaximumIndependentSet",
        target_name: "MinimumVertexCover",
        source_variant_fn: || <MaximumIndependentSet<SimpleGraph, Unweighted> as crate::traits::Problem>::variant(),
        target_variant_fn: || <MinimumVertexCover<SimpleGraph, Unweighted> as crate::traits::Problem>::variant(),
        overhead_fn: || { /* overhead */ },
        module_path: module_path!(),
    }
}
```

#### 3. Manual `inventory::submit!` Updates

Update all 3 manual registrations to use new field names:
- `coloring_ilp.rs` — may become auto-generated (macro can now handle `KColoring<K, G>`)
- `factoring_ilp.rs` — keep manual (complex overhead logic)
- `sat_ksat.rs` — may become auto-generated

#### 4. Graph Builder Updates

Update `export_graph` example and any code that reads `ReductionEntry` to call the fn pointers:
```rust
// Before:
let source_variant = entry.source_variant;

// After:
let source_variant = (entry.source_variant_fn)();
```

### Const Generic Substitution Rules

When the macro encounters `impl<const K: usize>`, it:
1. Identifies `K` as a const generic parameter
2. In the generated variant_fn calls, replaces `K` with `usize::MAX`
3. `const_usize_str::<{usize::MAX}>()` → `"N"`, meaning "any K"

When the impl uses a literal const (e.g., `KSatisfiability<2>`):
- No substitution needed — `2` is already concrete
- `const_usize_str::<2>()` → `"2"`, preserving the specific value

### Type Generic Substitution Rules

When the macro encounters `impl<W: SomeBound>`, it:
1. Identifies `W` as a type generic parameter
2. In the generated variant_fn calls, replaces `W` with `Unweighted`
3. `Unweighted::variant_name()` or `short_type_name::<Unweighted>()` → `"Unweighted"`

This produces the "base" variant for generic reductions.

### What Doesn't Change

- `Problem::variant()` signature and implementations
- `ReduceTo` and `ReductionResult` traits
- Existing reduction rule logic (only the generated registration code changes)
- `variant()` implementations on all problem types
- `const_usize_str()` utility

### Benefits

- **Zero inference bugs** — variant info comes from the authoritative `Problem::variant()` implementation
- **Simpler macro** — ~120 lines of inference logic removed
- **No new traits** — no `VariantName` trait, no changes to `Problem`
- **Handles all type patterns** — `<G, W>`, `<K, G>`, `<W>`, no params, all work
- **Const generics handled automatically** — `usize::MAX` sentinel produces `"N"`
