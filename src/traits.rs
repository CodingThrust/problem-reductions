//! Core traits for problem definitions.

/// Failure while evaluating one configuration of a valid problem instance.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum EvaluationError {
    #[error("invalid configuration: {0}")]
    InvalidConfiguration(String),
    #[error("integer overflow while {0}")]
    IntegerOverflow(String),
    #[error("inexact integer-to-float conversion while {0}")]
    InexactFloatConversion(String),
    #[error("non-finite floating-point result while {0}")]
    NonFiniteResult(String),
}

/// Minimal problem trait — a problem maps a configuration to a value or an
/// evaluation error.
///
/// This trait defines the interface for computational problems that can be
/// solved by enumeration or reduction to other problems.
pub trait Problem: Clone {
    /// Base name of this problem type (e.g., "MaximumIndependentSet").
    const NAME: &'static str;
    /// The evaluation value type.
    type Value: Clone;
    /// Configuration space dimensions. Each entry is the cardinality of that variable.
    fn dims(&self) -> Vec<usize>;
    /// Evaluate the problem on a configuration.
    fn evaluate(&self, config: &[usize]) -> Result<Self::Value, EvaluationError>;
    /// Number of variables (derived from dims).
    fn num_variables(&self) -> usize {
        self.dims().len()
    }
    /// Returns variant attributes derived from type parameters.
    ///
    /// Used for generating variant IDs in the reduction graph schema.
    /// Returns pairs like `[("graph", "SimpleGraph"), ("weight", "i64")]`.
    fn variant() -> Vec<(&'static str, &'static str)>;

    /// Look up this problem's catalog entry.
    ///
    /// Returns the full [`crate::registry::ProblemType`] metadata from the catalog registry.
    /// The default implementation uses `Self::NAME` to perform the lookup.
    fn problem_type() -> crate::registry::ProblemType {
        crate::registry::find_problem_type(Self::NAME)
            .unwrap_or_else(|| panic!("no catalog entry for Problem::NAME = {:?}", Self::NAME))
    }
}

/// Marker trait for explicitly declared problem variants.
///
/// Implemented automatically by `declare_variants!` for each concrete type.
/// The [`#[reduction]`] proc macro checks this trait at compile time to ensure
/// all reduction source/target types have been declared.
pub trait DeclaredVariant {}

#[cfg(test)]
#[path = "unit_tests/traits.rs"]
mod tests;
