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

/// K-value marker trait for types that represent a const-generic K parameter.
///
/// Types implementing this trait declare an optional K value. `None` means
/// the type represents an arbitrary K (like KN), while `Some(k)` means
/// a specific value (like K2, K3).
pub trait KValue: VariantParam + Clone + 'static {
    /// The K value, or `None` for arbitrary K.
    const K: Option<usize>;
}

/// Runtime-discoverable variant type registration.
///
/// Built by `impl_variant_param!` macro, collected by `inventory`.
pub struct VariantTypeEntry {
    /// Category name (e.g., `"graph"`, `"weight"`, `"k"`).
    pub category: &'static str,
    /// Type name within the category (e.g., `"SimpleGraph"`, `"i32"`).
    pub value: &'static str,
    /// Parent type name in the subtype hierarchy, or `None` for root types.
    pub parent: Option<&'static str>,
}

inventory::collect!(VariantTypeEntry);

/// Implement `VariantParam` (and optionally `CastToParent` and/or `KValue`) for a type,
/// and register a `VariantTypeEntry` with inventory.
///
/// # Usage
///
/// ```rust,ignore
/// // Root type (no parent):
/// impl_variant_param!(SimpleGraph, "graph");
///
/// // Type with parent -- cast closure required:
/// impl_variant_param!(UnitDiskGraph, "graph", parent: SimpleGraph,
///     cast: |g| SimpleGraph::new(g.num_vertices(), g.edges()));
///
/// // Root K type (no parent, with K value):
/// impl_variant_param!(KN, "k", k: None);
///
/// // K type with parent + cast + K value:
/// impl_variant_param!(K3, "k", parent: KN, cast: |_| KN, k: Some(3));
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
/// variant_params![]           // -> vec![]
/// variant_params![G, W]       // -> vec![(G::CATEGORY, G::VALUE), ...]
/// ```
#[macro_export]
macro_rules! variant_params {
    () => { vec![] };
    ($($T:ident),+) => {
        vec![$((<$T as $crate::variant::VariantParam>::CATEGORY,
              <$T as $crate::variant::VariantParam>::VALUE)),+]
    };
}

// --- Concrete KValue types ---

/// K=1 (e.g., 1-coloring).
#[derive(Clone, Copy, Debug, Default)]
pub struct K1;

/// K=2 (e.g., 2-SAT, 2-coloring).
#[derive(Clone, Copy, Debug, Default)]
pub struct K2;

/// K=3 (e.g., 3-SAT, 3-coloring).
#[derive(Clone, Copy, Debug, Default)]
pub struct K3;

/// K=4 (e.g., 4-coloring).
#[derive(Clone, Copy, Debug, Default)]
pub struct K4;

/// Generic K (any value). Used for reductions that apply to all K.
#[derive(Clone, Copy, Debug, Default)]
pub struct KN;

impl_variant_param!(KN, "k", k: None);
impl_variant_param!(K4, "k", parent: KN, cast: |_| KN, k: Some(4));
impl_variant_param!(K3, "k", parent: K4, cast: |_| K4, k: Some(3));
impl_variant_param!(K2, "k", parent: K3, cast: |_| K3, k: Some(2));
impl_variant_param!(K1, "k", parent: K2, cast: |_| K2, k: Some(1));

#[cfg(test)]
#[path = "unit_tests/variant.rs"]
mod tests;
