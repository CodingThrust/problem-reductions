//! Core traits for problem definitions.

use crate::types::ProblemParameters;

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

/// Minimal problem trait — a problem maps a solution to a value or an
/// evaluation error.
///
/// This trait defines the interface for computational problems that can be
/// evaluated or reduced to other problems.
pub trait Problem: Clone {
    /// Base name of this problem type (e.g., "MaximumIndependentSet").
    const NAME: &'static str;
    /// Mathematical witness type for this problem.
    type Solution;
    /// The evaluation value type.
    type Value: Clone;
    /// Canonical parameter names for this problem model.
    fn parameter_names() -> &'static [&'static str];
    /// Measure the complete canonical parameters of this concrete instance.
    fn parameters(&self) -> ProblemParameters;
    /// Evaluate the problem on a solution.
    fn evaluate(&self, solution: &Self::Solution) -> Result<Self::Value, EvaluationError>;
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

/// Define a problem's canonical parameters from inherent getter methods.
#[macro_export]
macro_rules! problem_parameters {
    ($(($name:literal, $getter:ident)),+ $(,)?) => {
        fn parameter_names() -> &'static [&'static str] {
            &[$($name),+]
        }

        fn parameters(&self) -> $crate::types::ProblemParameters {
            $crate::types::ProblemParameters::new(vec![
                $(($name, u64::try_from(self.$getter()).expect(concat!(
                    "parameter getter `", $name, "` violated its u64 invariant"
                )))),+
            ])
        }
    };
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
