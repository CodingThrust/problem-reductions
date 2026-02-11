//! Typed variant dimensions for problem identity.

use serde::{Deserialize, Serialize};

/// Typed key for variant dimensions.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VariantKey {
    /// Graph topology type (e.g., `SimpleGraph`).
    Graph,
    /// Weight/objective value type (e.g., `i32`, `f64`).
    Weight,
    /// Const generic parameter key (e.g., `k`).
    ConstParam(String),
    /// Domain-specific dimension key.
    Domain(String),
    /// Arbitrary custom key for forward compatibility.
    Custom(String),
}

impl VariantKey {
    /// Legacy string key used by existing exports.
    pub fn legacy_key(&self) -> &str {
        match self {
            VariantKey::Graph => "graph",
            VariantKey::Weight => "weight",
            VariantKey::ConstParam(key) => key.as_str(),
            VariantKey::Domain(key) => key.as_str(),
            VariantKey::Custom(key) => key.as_str(),
        }
    }

    /// Build a typed key from a legacy string key.
    pub fn from_legacy_key(key: &str) -> Self {
        match key {
            "graph" => VariantKey::Graph,
            "weight" => VariantKey::Weight,
            other => VariantKey::Custom(other.to_string()),
        }
    }
}

/// One typed variant dimension.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VariantDimension {
    /// Dimension key.
    pub key: VariantKey,
    /// Dimension value.
    pub value: String,
}

impl VariantDimension {
    /// Create a dimension with explicit key.
    pub fn new(key: VariantKey, value: impl Into<String>) -> Self {
        Self {
            key,
            value: value.into(),
        }
    }

    /// Create a graph dimension.
    pub fn graph(value: impl Into<String>) -> Self {
        Self::new(VariantKey::Graph, value)
    }

    /// Create a weight dimension.
    pub fn weight(value: impl Into<String>) -> Self {
        Self::new(VariantKey::Weight, value)
    }
}

/// Convert legacy variant pairs to typed dimensions.
pub fn from_legacy_variant(legacy: &[(&str, &str)]) -> Vec<VariantDimension> {
    legacy
        .iter()
        .map(|(k, v)| VariantDimension::new(VariantKey::from_legacy_key(k), *v))
        .collect()
}
