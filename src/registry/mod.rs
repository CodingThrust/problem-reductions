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
//! # Using with Problems
//!
//! Problems that implement [`ProblemMetadata`] can be queried for their category and info:
//!
//! ```rust
//! use problemreductions::registry::ProblemMetadata;
//! use problemreductions::models::graph::IndependentSetT;
//!
//! let info = IndependentSetT::<i32>::problem_info();
//! let category = IndependentSetT::<i32>::category();
//!
//! println!("Problem: {} ({})", info.name, category.path());
//! ```

mod category;
mod info;

pub use category::{
    GraphSubcategory, NetworkSubcategory, OptimizationSubcategory, ProblemCategory,
    SatisfiabilitySubcategory, SchedulingSubcategory, SetSubcategory, SpecializedSubcategory,
    StringSubcategory,
};
pub use info::{ComplexityClass, ProblemInfo, ProblemMetadata};
