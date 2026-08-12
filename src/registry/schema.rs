//! Problem schema registration via inventory.

use super::FieldInfo;
use serde::Serialize;
use std::fmt;
use std::str::FromStr;

/// Structural category used to organize problem implementations and catalog output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ProblemCategory {
    Algebraic,
    Formula,
    Graph,
    Misc,
    Set,
}

impl ProblemCategory {
    pub const ALL: [Self; 5] = [
        Self::Algebraic,
        Self::Formula,
        Self::Graph,
        Self::Misc,
        Self::Set,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Algebraic => "algebraic",
            Self::Formula => "formula",
            Self::Graph => "graph",
            Self::Misc => "misc",
            Self::Set => "set",
        }
    }
}

impl fmt::Display for ProblemCategory {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Error returned when a catalog category is not one of the five supported values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseProblemCategoryError(String);

impl fmt::Display for ParseProblemCategoryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let expected = ProblemCategory::ALL.map(ProblemCategory::as_str).join(", ");
        write!(
            formatter,
            "unknown problem category `{}`; expected one of: {expected}",
            self.0,
        )
    }
}

impl std::error::Error for ParseProblemCategoryError {}

impl FromStr for ProblemCategory {
    type Err = ParseProblemCategoryError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|category| category.as_str() == value)
            .ok_or_else(|| ParseProblemCategoryError(value.to_string()))
    }
}

/// A declared variant dimension for a problem type.
///
/// Describes one axis of variation (e.g., graph type, weight type) with
/// its default value and the set of allowed values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariantDimension {
    /// Dimension key (e.g., `"graph"`, `"weight"`, `"k"`).
    pub key: &'static str,
    /// Default value for this dimension (e.g., `"SimpleGraph"`).
    pub default_value: &'static str,
    /// All allowed values for this dimension.
    pub allowed_values: &'static [&'static str],
}

impl VariantDimension {
    /// Create a new variant dimension.
    pub const fn new(
        key: &'static str,
        default_value: &'static str,
        allowed_values: &'static [&'static str],
    ) -> Self {
        Self {
            key,
            default_value,
            allowed_values,
        }
    }
}

/// A registered problem schema entry for static inventory registration.
///
/// Category is required rather than inferred from source location:
///
/// ```compile_fail
/// use problemreductions::registry::ProblemSchemaEntry;
///
/// let _schema = ProblemSchemaEntry {
///     name: "Example",
///     display_name: "Example",
///     aliases: &[],
///     dimensions: &[],
///     module_path: module_path!(),
///     description: "Example schema",
///     fields: &[],
/// };
/// ```
pub struct ProblemSchemaEntry {
    /// Problem name (e.g., "MaximumIndependentSet").
    pub name: &'static str,
    /// Human-readable display name (e.g., "Maximum Independent Set").
    pub display_name: &'static str,
    /// Short aliases for CLI/MCP lookup (e.g., `&["MIS"]`).
    pub aliases: &'static [&'static str],
    /// Declared variant dimensions with defaults and allowed values.
    pub dimensions: &'static [VariantDimension],
    /// Explicit structural category shown in catalog output.
    pub category: ProblemCategory,
    /// Module path from `module_path!()` (e.g., "problemreductions::models::graph::maximum_independent_set").
    pub module_path: &'static str,
    /// Human-readable description.
    pub description: &'static str,
    /// Inputs accepted when constructing this problem.
    pub fields: &'static [FieldInfo],
}

inventory::collect!(ProblemSchemaEntry);

/// Optional static size-field metadata for problem types.
///
/// This is used when a problem has meaningful size fields even before it
/// participates in any reduction size expressions.
pub struct ProblemSizeFieldEntry {
    /// Problem name (e.g., "MaximumIndependentSet").
    pub name: &'static str,
    /// Size field names (e.g., `&["num_vertices", "num_edges"]`).
    pub fields: &'static [&'static str],
}

inventory::collect!(ProblemSizeFieldEntry);

/// JSON-serializable problem schema.
#[derive(Debug, Clone, Serialize)]
pub struct ProblemSchemaJson {
    /// Problem name.
    pub name: String,
    /// Problem description.
    pub description: String,
    /// Structural catalog category.
    pub category: ProblemCategory,
    /// Inputs accepted when constructing this problem.
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
            category: entry.category,
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

/// Collect explicitly declared size fields for a problem type.
pub fn declared_size_fields(name: &str) -> Vec<&'static str> {
    inventory::iter::<ProblemSizeFieldEntry>()
        .filter(|entry| entry.name == name)
        .flat_map(|entry| entry.fields.iter().copied())
        .collect()
}

#[cfg(test)]
#[path = "../unit_tests/registry/schema.rs"]
mod tests;
