//! Problem registry and metadata types.
//!
//! This module provides types for problem classification, introspection, and discovery.
//! It enables organizing 100+ NP-complete problems into a hierarchical category system
//! and provides rich metadata for each problem type.
//!
//! # Overview
//!
//! - [`ProblemCategory`] - Hierarchical categorization (e.g., `graph/independent`)
//! - [`ProblemInfo`] - Rich metadata (name, description, complexity, reductions)
//! - [`ProblemMetadata`] - Trait for problems to provide their own metadata
//! - [`ComplexityClass`] - Computational complexity classification
//!
//! # Example
//!
//! ```rust
//! use problemreductions::registry::{ProblemCategory, GraphSubcategory, ProblemInfo, ComplexityClass};
//!
//! // Create a category path
//! let category = ProblemCategory::Graph(GraphSubcategory::Independent);
//! assert_eq!(category.path(), "graph/independent");
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
//!     ProblemMetadata, ProblemInfo, ProblemCategory,
//!     GraphSubcategory, ComplexityClass
//! };
//!
//! struct MyProblem;
//!
//! impl ProblemMetadata for MyProblem {
//!     fn problem_info() -> ProblemInfo {
//!         ProblemInfo::new("My Problem", "Description")
//!             .with_complexity(ComplexityClass::NpComplete)
//!     }
//!
//!     fn category() -> ProblemCategory {
//!         ProblemCategory::Graph(GraphSubcategory::Independent)
//!     }
//! }
//!
//! let info = MyProblem::problem_info();
//! let category = MyProblem::category();
//! println!("Problem: {} ({})", info.name, category.path());
//! ```

mod category;
mod info;
mod schema;

pub use category::{
    GraphSubcategory, NetworkSubcategory, OptimizationSubcategory, ProblemCategory,
    SatisfiabilitySubcategory, SchedulingSubcategory, SetSubcategory, SpecializedSubcategory,
    StringSubcategory,
};
pub use info::{ComplexityClass, FieldInfo, ProblemInfo, ProblemMetadata};
pub use schema::{collect_schemas, FieldInfoJson, ProblemSchemaEntry, ProblemSchemaJson};
