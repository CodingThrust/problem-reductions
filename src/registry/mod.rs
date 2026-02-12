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

mod info;
mod schema;

pub use info::{ComplexityClass, FieldInfo, ProblemInfo, ProblemMetadata};
pub use schema::{collect_schemas, FieldInfoJson, ProblemSchemaEntry, ProblemSchemaJson};
