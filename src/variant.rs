//! Variant system for type-level problem parameterization.
//!
//! Types declare their variant category and value via `VariantParam`.
//! The `impl_variant_param!` macro registers types with the trait.
//! The `variant_params!` macro composes `Problem::variant()` bodies from type parameter names.

/// A type that participates in the variant system.
///
/// Declares its category (e.g., `"graph"`) and value (e.g., `"SimpleGraph"`).
pub trait VariantParam: 'static {
    /// Category name (e.g., `"graph"`, `"weight"`, `"k"`).
    const CATEGORY: &'static str;
    /// Type name within the category (e.g., `"SimpleGraph"`, `"i64"`).
    const VALUE: &'static str;
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

/// Implement `VariantParam` and optionally `KValue` for a type.
///
/// # Usage
///
/// ```text
/// // Variant parameter:
/// impl_variant_param!(SimpleGraph, "graph");
///
/// // Generic K value:
/// impl_variant_param!(KN, "k", k: None);
///
/// // Concrete K value:
/// impl_variant_param!(K3, "k", k: Some(3));
/// ```
#[macro_export]
macro_rules! impl_variant_param {
    ($ty:ty, $cat:expr) => {
        impl $crate::variant::VariantParam for $ty {
            const CATEGORY: &'static str = $cat;
            const VALUE: &'static str = stringify!($ty);
        }
    };
    ($ty:ty, $cat:expr, k: $k:expr) => {
        $crate::impl_variant_param!($ty, $cat);
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
/// ```text
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

/// K=5 (e.g., 5-coloring).
#[derive(Clone, Copy, Debug, Default)]
pub struct K5;

/// Generic K (any value). Used for reductions that apply to all K.
#[derive(Clone, Copy, Debug, Default)]
pub struct KN;

impl_variant_param!(KN, "k", k: None);
impl_variant_param!(K5, "k", k: Some(5));
impl_variant_param!(K4, "k", k: Some(4));
impl_variant_param!(K3, "k", k: Some(3));
impl_variant_param!(K2, "k", k: Some(2));
impl_variant_param!(K1, "k", k: Some(1));

// --- VariantSpec: canonical runtime representation of a problem variant ---

use std::collections::BTreeMap;

/// Canonical runtime representation of a problem variant.
///
/// Unlike raw `BTreeMap<String, String>`, construction from pairs rejects
/// duplicate dimensions.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VariantSpec {
    dims: BTreeMap<String, String>,
}

impl VariantSpec {
    /// Create a `VariantSpec` from key-value pairs, rejecting duplicate dimensions.
    ///
    /// Returns an error if the same dimension key appears more than once.
    pub fn try_from_pairs<I, K, V>(
        pairs: I,
    ) -> std::result::Result<Self, crate::registry::ConstructionError>
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        let mut dims = BTreeMap::new();
        for (k, v) in pairs {
            let key = k.into();
            let val = v.into();
            if dims.insert(key.clone(), val).is_some() {
                return Err(format!("duplicate dimension: {}", key).into());
            }
        }
        Ok(Self { dims })
    }

    /// Create a `VariantSpec` from an existing `BTreeMap`.
    pub fn try_from_map(
        map: BTreeMap<String, String>,
    ) -> std::result::Result<Self, crate::registry::ConstructionError> {
        Ok(Self { dims: map })
    }

    /// View the dimensions as a map.
    pub fn as_map(&self) -> &BTreeMap<String, String> {
        &self.dims
    }

    /// Consume this `VariantSpec` and return the underlying map.
    pub fn into_map(self) -> BTreeMap<String, String> {
        self.dims
    }

    /// Update or add a single dimension.
    pub fn update_dimension(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.dims.insert(key.into(), value.into());
    }
}

#[cfg(test)]
#[path = "unit_tests/variant.rs"]
mod tests;
