//! Problem type catalog: runtime lookup by name, alias, and variant validation.

use super::schema::{ProblemSchemaEntry, VariantDimension};
use super::FieldInfo;
use std::collections::BTreeMap;

/// A runtime view of a registered problem type from the catalog.
#[derive(Debug, Clone)]
pub struct ProblemType {
    /// Canonical problem name (e.g., `"MaximumIndependentSet"`).
    pub canonical_name: &'static str,
    /// Human-readable display name (e.g., `"Maximum Independent Set"`).
    pub display_name: &'static str,
    /// Short aliases (e.g., `["MIS"]`).
    pub aliases: &'static [&'static str],
    /// Declared variant dimensions with defaults and allowed values.
    pub dimensions: &'static [VariantDimension],
    /// Human-readable description.
    pub description: &'static str,
    /// Struct fields.
    pub fields: &'static [FieldInfo],
}

impl ProblemType {
    /// Build a `ProblemType` view from a schema entry.
    fn from_entry(entry: &'static ProblemSchemaEntry) -> Self {
        Self {
            canonical_name: entry.name,
            display_name: entry.display_name,
            aliases: entry.aliases,
            dimensions: entry.dimensions,
            description: entry.description,
            fields: entry.fields,
        }
    }

    /// Get the default variant map (each dimension set to its default value).
    pub fn default_variant(&self) -> BTreeMap<String, String> {
        self.dimensions
            .iter()
            .map(|d| (d.key.to_string(), d.default_value.to_string()))
            .collect()
    }
}

/// Find a problem type by exact canonical name.
pub fn find_problem_type(name: &str) -> Option<ProblemType> {
    inventory::iter::<ProblemSchemaEntry>
        .into_iter()
        .find(|entry| entry.name == name)
        .map(ProblemType::from_entry)
}

/// Find a problem type by alias (case-insensitive).
///
/// Searches both canonical names and declared aliases.
pub fn find_problem_type_by_alias(input: &str) -> Option<ProblemType> {
    let lower = input.to_lowercase();
    inventory::iter::<ProblemSchemaEntry>
        .into_iter()
        .find(|entry| {
            entry.name.to_lowercase() == lower
                || entry.aliases.iter().any(|a| a.to_lowercase() == lower)
        })
        .map(ProblemType::from_entry)
}

/// Return all registered problem types.
pub fn problem_types() -> Vec<ProblemType> {
    let mut types: Vec<ProblemType> = inventory::iter::<ProblemSchemaEntry>
        .into_iter()
        .map(ProblemType::from_entry)
        .collect();
    types.sort_by_key(|t| t.canonical_name);
    types
}

#[cfg(test)]
#[path = "../unit_tests/registry/problem_type.rs"]
mod tests;
