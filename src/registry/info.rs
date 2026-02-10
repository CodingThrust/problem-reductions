//! Problem metadata and information types.
//!
//! This module provides types for describing problem characteristics:
//!
//! - [`ComplexityClass`] - Computational complexity (P, NP-complete, etc.)
//! - [`ProblemInfo`] - Rich metadata about a problem type
//! - [`ProblemMetadata`] - Trait for problems to provide their metadata
//!
//! # Example
//!
//! ```rust
//! use problemreductions::registry::{ProblemInfo, ComplexityClass};
//!
//! let info = ProblemInfo::new("3-SAT", "Satisfiability with 3-literal clauses")
//!     .with_aliases(&["3-CNF-SAT", "3SAT"])
//!     .with_complexity(ComplexityClass::NpComplete)
//!     .with_reduction_from("SAT");
//!
//! assert!(info.is_np_complete());
//! assert_eq!(info.all_names().len(), 3);
//! ```

use super::ProblemCategory;
use std::fmt;

/// Computational complexity class of a problem.
///
/// Used to classify problems by their computational difficulty.
/// Most problems in this library are [`NpComplete`](ComplexityClass::NpComplete).
///
/// # Example
///
/// ```rust
/// use problemreductions::registry::ComplexityClass;
///
/// let class = ComplexityClass::NpComplete;
/// assert!(class.is_hard());
/// assert_eq!(class.name(), "NP-complete");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComplexityClass {
    /// In P (polynomial time)
    P,
    /// NP-complete
    NpComplete,
    /// NP-hard (at least as hard as NP-complete)
    NpHard,
    /// PSPACE-complete
    PspaceComplete,
    /// Unknown or unclassified
    Unknown,
}

impl ComplexityClass {
    /// Get the complexity class name.
    pub fn name(&self) -> &'static str {
        match self {
            ComplexityClass::P => "P",
            ComplexityClass::NpComplete => "NP-complete",
            ComplexityClass::NpHard => "NP-hard",
            ComplexityClass::PspaceComplete => "PSPACE-complete",
            ComplexityClass::Unknown => "Unknown",
        }
    }

    /// Check if this problem is at least NP-hard.
    pub fn is_hard(&self) -> bool {
        matches!(
            self,
            ComplexityClass::NpComplete | ComplexityClass::NpHard | ComplexityClass::PspaceComplete
        )
    }
}

impl fmt::Display for ComplexityClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Metadata about a problem type.
///
/// Contains static information about a problem definition, including its name,
/// description, complexity class, and relationships to other problems.
/// Use the builder methods to construct instances.
///
/// # Example
///
/// ```rust
/// use problemreductions::registry::{ProblemInfo, ComplexityClass};
///
/// let info = ProblemInfo::new("Vertex Cover", "Find minimum vertices covering all edges")
///     .with_aliases(&["VC", "Minimum Vertex Cover"])
///     .with_complexity(ComplexityClass::NpComplete)
///     .with_reduction_from("Independent Set")
///     .with_reference("https://en.wikipedia.org/wiki/Vertex_cover");
///
/// println!("{}", info);  // "Vertex Cover (NP-complete)"
/// ```
///
/// # Builder Pattern
///
/// All builder methods are `const fn` and can be used in const contexts:
///
/// ```rust
/// use problemreductions::registry::{ProblemInfo, ComplexityClass};
///
/// const MY_PROBLEM_INFO: ProblemInfo = ProblemInfo::new("My Problem", "Description")
///     .with_complexity(ComplexityClass::NpComplete);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProblemInfo {
    /// The canonical name of the problem.
    pub name: &'static str,
    /// Alternative names for the problem.
    pub aliases: &'static [&'static str],
    /// A brief description of the problem.
    pub description: &'static str,
    /// The computational complexity class.
    pub complexity_class: ComplexityClass,
    /// Whether this has a decision version (yes/no answer).
    pub decision_version: bool,
    /// Whether this has an optimization version.
    pub optimization_version: bool,
    /// The canonical problem this reduces from (for NP-completeness proof).
    pub canonical_reduction_from: Option<&'static str>,
    /// Wikipedia or reference URL.
    pub reference_url: Option<&'static str>,
    /// Struct field descriptions for schema export.
    pub fields: &'static [FieldInfo],
}

impl ProblemInfo {
    /// Create a new ProblemInfo with minimal required fields.
    pub const fn new(name: &'static str, description: &'static str) -> Self {
        Self {
            name,
            aliases: &[],
            description,
            complexity_class: ComplexityClass::NpComplete,
            decision_version: true,
            optimization_version: true,
            canonical_reduction_from: None,
            reference_url: None,
            fields: &[],
        }
    }

    /// Builder method to add aliases.
    pub const fn with_aliases(mut self, aliases: &'static [&'static str]) -> Self {
        self.aliases = aliases;
        self
    }

    /// Builder method to set complexity class.
    pub const fn with_complexity(mut self, class: ComplexityClass) -> Self {
        self.complexity_class = class;
        self
    }

    /// Builder method to set decision version availability.
    pub const fn with_decision(mut self, has_decision: bool) -> Self {
        self.decision_version = has_decision;
        self
    }

    /// Builder method to set optimization version availability.
    pub const fn with_optimization(mut self, has_optimization: bool) -> Self {
        self.optimization_version = has_optimization;
        self
    }

    /// Builder method to set the canonical reduction source.
    pub const fn with_reduction_from(mut self, source: &'static str) -> Self {
        self.canonical_reduction_from = Some(source);
        self
    }

    /// Builder method to set reference URL.
    pub const fn with_reference(mut self, url: &'static str) -> Self {
        self.reference_url = Some(url);
        self
    }

    /// Builder method to set struct field descriptions.
    pub const fn with_fields(mut self, fields: &'static [FieldInfo]) -> Self {
        self.fields = fields;
        self
    }

    /// Check if this problem is NP-complete.
    pub fn is_np_complete(&self) -> bool {
        self.complexity_class == ComplexityClass::NpComplete
    }

    /// Get all names (canonical + aliases).
    pub fn all_names(&self) -> Vec<&'static str> {
        let mut names = vec![self.name];
        names.extend_from_slice(self.aliases);
        names
    }
}

impl fmt::Display for ProblemInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.complexity_class)
    }
}

/// Description of a struct field for JSON schema export.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldInfo {
    /// Field name as it appears in the Rust struct.
    pub name: &'static str,
    /// Type name (e.g., `Vec<W>`, `UnGraph<(), ()>`).
    pub type_name: &'static str,
    /// Human-readable description of what this field represents.
    pub description: &'static str,
}

/// Trait for problems that provide static metadata.
///
/// Implement this trait to enable introspection and discovery for problem types.
///
/// # Example
///
/// ```rust
/// use problemreductions::registry::{
///     ProblemMetadata, ProblemInfo, ProblemCategory,
///     GraphSubcategory, ComplexityClass
/// };
///
/// struct MyProblem;
///
/// impl ProblemMetadata for MyProblem {
///     fn problem_info() -> ProblemInfo {
///         ProblemInfo::new("My Problem", "Description")
///             .with_complexity(ComplexityClass::NpComplete)
///     }
///
///     fn category() -> ProblemCategory {
///         ProblemCategory::Graph(GraphSubcategory::Independent)
///     }
/// }
///
/// // Get problem metadata
/// let info = MyProblem::problem_info();
/// assert_eq!(info.name, "My Problem");
///
/// let category = MyProblem::category();
/// assert_eq!(category.path(), "graph/independent");
/// ```
///
/// # Implementing for Custom Problems
///
/// ```rust
/// use problemreductions::registry::{
///     ProblemMetadata, ProblemInfo, ProblemCategory,
///     GraphSubcategory, ComplexityClass
/// };
///
/// struct MyProblem;
///
/// impl ProblemMetadata for MyProblem {
///     fn problem_info() -> ProblemInfo {
///         ProblemInfo::new("My Problem", "Description of my problem")
///             .with_complexity(ComplexityClass::NpComplete)
///     }
///
///     fn category() -> ProblemCategory {
///         ProblemCategory::Graph(GraphSubcategory::Independent)
///     }
/// }
/// ```
pub trait ProblemMetadata {
    /// Returns the problem info for this problem type.
    ///
    /// This includes the problem name, description, aliases, complexity class,
    /// and known reductions.
    fn problem_info() -> ProblemInfo;

    /// Returns the problem category.
    ///
    /// This is a hierarchical classification like "graph/independent" or
    /// "satisfiability/sat".
    fn category() -> ProblemCategory;
}

#[cfg(test)]
#[path = "../unit_tests/registry/info.rs"]
mod tests;
