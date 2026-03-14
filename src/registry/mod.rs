//! Problem registry and metadata types.
//!
//! This module provides types for problem introspection and discovery.
//!
//! # Overview
//!
//! - [`ProblemInfo`] - Rich metadata (name, description, complexity, reductions)
//! - [`ProblemMetadata`] - Trait for problems to provide their own metadata
//! - [`ComplexityClass`] - Computational complexity classification
//!
//! # Example
//!
//! ```rust
//! use problemreductions::registry::{ProblemInfo, ComplexityClass};
//!
//! // Create problem metadata
//! let info = ProblemInfo::new("Independent Set", "Find maximum non-adjacent vertices")
//!     .with_aliases(&["MIS", "Stable Set"])
//!     .with_complexity(ComplexityClass::NpComplete)
//!     .with_reduction_from("3-SAT");
//!
//! assert!(info.is_np_complete());
//! ```
//!
//! # Implementing for Custom Problems
//!
//! Problems can implement [`ProblemMetadata`] to provide introspection:
//!
//! ```rust
//! use problemreductions::registry::{
//!     ProblemMetadata, ProblemInfo, ComplexityClass
//! };
//!
//! struct MyProblem;
//!
//! impl ProblemMetadata for MyProblem {
//!     fn problem_info() -> ProblemInfo {
//!         ProblemInfo::new("My Problem", "Description")
//!             .with_complexity(ComplexityClass::NpComplete)
//!     }
//! }
//!
//! let info = MyProblem::problem_info();
//! println!("Problem: {}", info.name);
//! ```

mod dyn_problem;
mod info;
pub mod problem_ref;
pub mod problem_type;
mod schema;
pub mod variant;

pub use dyn_problem::{DynProblem, LoadedDynProblem, SolveFn};
pub use info::{ComplexityClass, FieldInfo, ProblemInfo, ProblemMetadata};
pub use problem_ref::{parse_catalog_problem_ref, require_graph_variant, ProblemRef};
pub use problem_type::{find_problem_type, find_problem_type_by_alias, problem_types, ProblemType};
pub use schema::{
    collect_schemas, FieldInfoJson, ProblemSchemaEntry, ProblemSchemaJson, VariantDimension,
};
pub use variant::{find_variant_entry, VariantEntry};

use std::any::Any;
use std::collections::BTreeMap;

/// Load a problem from JSON by exact problem name and exact variant map.
///
/// No alias resolution or default fallback. Returns `Err` if the entry is not found.
pub fn load_dyn(
    name: &str,
    variant: &BTreeMap<String, String>,
    data: serde_json::Value,
) -> Result<LoadedDynProblem, String> {
    let entry = find_variant_entry(name, variant).ok_or_else(|| {
        format!(
            "No registered variant for `{name}` with variant {:?}",
            variant
        )
    })?;

    let inner =
        (entry.factory)(data).map_err(|e| format!("Failed to deserialize `{name}`: {e}"))?;
    Ok(LoadedDynProblem::new(inner, entry.solve_fn))
}

/// Serialize a `&dyn Any` by exact problem name and exact variant map.
///
/// Returns `None` if the entry is not found or the downcast fails.
pub fn serialize_any(
    name: &str,
    variant: &BTreeMap<String, String>,
    any: &dyn Any,
) -> Option<serde_json::Value> {
    let entry = find_variant_entry(name, variant)?;
    (entry.serialize_fn)(any)
}

#[cfg(test)]
#[path = "../unit_tests/registry/dispatch.rs"]
mod dispatch_tests;
