# Explicit Variant Declarations with Per-Variant Complexity

**Date:** 2026-02-27
**Status:** Approved

## Problem

Variants currently emerge implicitly from `#[reduction]` registrations. This means:
- A variant can't exist without a reduction
- There's no place to attach per-variant metadata (e.g., worst-case time complexity)
- No compile-time validation that reductions reference valid variants

## Design

### New types

**`DeclaredVariant` marker trait** (`src/traits.rs`):
```rust
pub trait DeclaredVariant {}
```

**`VariantEntry` inventory struct** (new file `src/registry/variant.rs`):
```rust
pub struct VariantEntry {
    pub name: &'static str,
    pub variant_fn: fn() -> Vec<(&'static str, &'static str)>,
    pub complexity: &'static str,  // worst-case time complexity, e.g., "2^num_vertices"
}
inventory::collect!(VariantEntry);
```

### `declare_variants!` macro

Declarative macro that generates both `DeclaredVariant` trait impls and `VariantEntry` inventory submissions:

```rust
macro_rules! declare_variants {
    ($($ty:ty => $complexity:expr),+ $(,)?) => {
        $(
            impl $crate::traits::DeclaredVariant for $ty {}

            inventory::submit! {
                $crate::registry::VariantEntry {
                    name: <$ty as $crate::traits::Problem>::NAME,
                    variant_fn: || <$ty as $crate::traits::Problem>::variant(),
                    complexity: $complexity,
                }
            }
        )+
    };
}
```

**Usage** (in each model file, e.g., `maximum_independent_set.rs`):
```rust
declare_variants! {
    MaximumIndependentSet<SimpleGraph, i32>    => "2^num_vertices",
    MaximumIndependentSet<KingsSubgraph, i32>  => "2^num_vertices",
    MaximumIndependentSet<UnitDiskGraph, i32>  => "2^num_vertices",
}
```

### Compile-time checking in `#[reduction]`

The `#[reduction]` proc macro generates a `DeclaredVariant` assertion after the impl block:

```rust
const _: () = {
    fn _assert<T: DeclaredVariant>() {}
    _assert::<SourceType>();
    _assert::<TargetType>();
};
```

This produces a compile error if either source or target variant is not declared via `declare_variants!`.

### Graph construction change

`ReductionGraph::new()` changes:
1. **First:** Build nodes from `VariantEntry` inventory (each entry becomes a node with complexity metadata)
2. **Then:** Build edges from `ReductionEntry` inventory (edges connect existing nodes)
3. Edges referencing undeclared variants would be caught at compile time by `#[reduction]`

### Display changes

- `pred show <problem>`: Shows complexity per variant in the variants list
- Graph JSON export: Adds `complexity` field per node
- `pred show` JSON output: Includes complexity in variant info

## Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Declaration location | Model file | All variants of a problem are visible in one place |
| Macro syntax | `declare_variants!` (macro_rules!) | Good balance of conciseness vs. complexity |
| Type specification | Concrete Rust types | Enables compile-time checking via trait bounds |
| Validation | Compile error | Strictest; catches mistakes early via `DeclaredVariant` trait |
| Complexity format | String expression (e.g., `"2^num_vertices"`) | Consistent with overhead expression syntax |

## Scope

Every model file that has variants needs a `declare_variants!` call. This touches all files in `src/models/`.
