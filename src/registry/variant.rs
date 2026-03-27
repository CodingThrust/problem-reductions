//! Explicit variant registration via inventory.

use std::any::Any;
use std::collections::BTreeMap;

use crate::registry::dyn_problem::{DynProblem, SolveValueFn, SolveWitnessFn};

/// A registered problem variant entry.
///
/// Submitted by [`declare_variants!`] for each concrete problem type.
/// The reduction graph uses these entries to build nodes with complexity metadata.
pub struct VariantEntry {
    /// Problem name (from `Problem::NAME`).
    pub name: &'static str,
    /// Function returning variant key-value pairs (from `Problem::variant()`).
    pub variant_fn: fn() -> Vec<(&'static str, &'static str)>,
    /// Worst-case time complexity expression (e.g., `"2^num_vertices"`).
    pub complexity: &'static str,
    /// Compiled complexity evaluation function.
    /// Takes a `&dyn Any` (must be `&ProblemType`), calls getter methods directly,
    /// and returns the estimated worst-case time as f64.
    pub complexity_eval_fn: fn(&dyn Any) -> f64,
    /// Whether this entry is the declared default variant for its problem.
    pub is_default: bool,
    /// Factory: deserialize JSON into a boxed dynamic problem.
    pub factory: fn(serde_json::Value) -> Result<Box<dyn DynProblem>, serde_json::Error>,
    /// Serialize: downcast `&dyn Any` and serialize to JSON.
    pub serialize_fn: fn(&dyn Any) -> Option<serde_json::Value>,
    /// Solve value: downcast `&dyn Any` and brute-force solve to an aggregate string.
    pub solve_value_fn: SolveValueFn,
    /// Solve witness: downcast `&dyn Any` and brute-force recover a witness when available.
    pub solve_witness_fn: SolveWitnessFn,
}

impl VariantEntry {
    /// Get the variant by calling the function.
    pub fn variant(&self) -> Vec<(&'static str, &'static str)> {
        (self.variant_fn)()
    }

    /// Get the variant as a `BTreeMap<String, String>`.
    pub fn variant_map(&self) -> BTreeMap<String, String> {
        self.variant()
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }
}

/// Find a variant entry by exact problem name and exact variant map.
///
/// No alias resolution or default fallback. Both `name` and `variant` must match exactly.
pub fn find_variant_entry(
    name: &str,
    variant: &BTreeMap<String, String>,
) -> Option<&'static VariantEntry> {
    inventory::iter::<VariantEntry>()
        .find(|entry| entry.name == name && entry.variant_map() == *variant)
}

impl std::fmt::Debug for VariantEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VariantEntry")
            .field("name", &self.name)
            .field("variant", &self.variant())
            .field("complexity", &self.complexity)
            .finish()
    }
}

inventory::collect!(VariantEntry);
