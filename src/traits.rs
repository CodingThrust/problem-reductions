//! Core traits for problem definitions.

use crate::types::Direction;

/// Minimal problem trait — a problem is a function from configuration to metric.
///
/// This trait defines the interface for computational problems that can be
/// solved by enumeration or reduction to other problems.
pub trait Problem: Clone {
    /// Base name of this problem type (e.g., "MaximumIndependentSet").
    const NAME: &'static str;
    /// The evaluation metric type.
    type Metric: Clone;
    /// Configuration space dimensions. Each entry is the cardinality of that variable.
    fn dims(&self) -> Vec<usize>;
    /// Evaluate the problem on a configuration.
    fn evaluate(&self, config: &[usize]) -> Self::Metric;
    /// Number of variables (derived from dims).
    fn num_variables(&self) -> usize {
        self.dims().len()
    }
    /// Returns variant attributes derived from type parameters.
    ///
    /// Used for generating variant IDs in the reduction graph schema.
    /// Returns pairs like `[("graph", "SimpleGraph"), ("weight", "i32")]`.
    fn variant() -> Vec<(&'static str, &'static str)>;
}

/// Extension for problems with a numeric objective to optimize.
pub trait OptimizationProblem: Problem {
    /// Whether to maximize or minimize the metric.
    fn direction(&self) -> crate::types::Direction;

    /// Returns true if metric `a` is better than metric `b` for this problem.
    fn is_better(&self, a: &Self::Metric, b: &Self::Metric) -> bool;
}

#[cfg(test)]
#[path = "unit_tests/traits.rs"]
mod tests;
