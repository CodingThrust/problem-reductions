//! Problem schema registration via inventory.

use super::FieldInfo;
use serde::Serialize;

/// A registered problem schema entry for static inventory registration.
pub struct ProblemSchemaEntry {
    /// Problem name (e.g., "MaximumIndependentSet").
    pub name: &'static str,
    /// Human-readable description.
    pub description: &'static str,
    /// Struct fields.
    pub fields: &'static [FieldInfo],
}

inventory::collect!(ProblemSchemaEntry);

/// JSON-serializable problem schema.
#[derive(Debug, Clone, Serialize)]
pub struct ProblemSchemaJson {
    /// Problem name.
    pub name: String,
    /// Problem description.
    pub description: String,
    /// Struct fields.
    pub fields: Vec<FieldInfoJson>,
}

/// JSON-serializable field info.
#[derive(Debug, Clone, Serialize)]
pub struct FieldInfoJson {
    /// Field name.
    pub name: String,
    /// Field type.
    pub type_name: String,
    /// Field description.
    pub description: String,
}

/// Collect all registered problem schemas into JSON-serializable form.
pub fn collect_schemas() -> Vec<ProblemSchemaJson> {
    let mut schemas: Vec<ProblemSchemaJson> = inventory::iter::<ProblemSchemaEntry>
        .into_iter()
        .map(|entry| ProblemSchemaJson {
            name: entry.name.to_string(),
            description: entry.description.to_string(),
            fields: entry
                .fields
                .iter()
                .map(|f| FieldInfoJson {
                    name: f.name.to_string(),
                    type_name: f.type_name.to_string(),
                    description: f.description.to_string(),
                })
                .collect(),
        })
        .collect();
    schemas.sort_by(|a, b| a.name.cmp(&b.name));
    schemas
}

#[cfg(test)]
#[path = "../unit_tests/registry/schema.rs"]
mod tests;
