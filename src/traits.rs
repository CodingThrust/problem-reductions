//! Core traits for problem definitions.

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
    /// Named size components for overhead estimation in reduction path-finding.
    fn problem_size(&self) -> crate::types::ProblemSize;
}

/// Extension for problems with a numeric objective to optimize.
///
/// The supertrait bound guarantees `Metric = SolutionSize<Self::Value>`,
/// so the solver can call `metric.is_valid()` and `metric.is_better()`
/// directly — no per-problem customization needed.
pub trait OptimizationProblem: Problem<Metric = crate::types::SolutionSize<Self::Value>> {
    /// The inner objective value type (e.g., `i32`, `f64`).
    type Value: PartialOrd + Clone;
    /// Whether to maximize or minimize the metric.
    fn direction(&self) -> crate::types::Direction;
}

/// Marker trait for satisfaction (decision) problems.
///
/// Satisfaction problems evaluate configurations to `bool`:
/// `true` if the configuration satisfies all constraints, `false` otherwise.
pub trait SatisfactionProblem: Problem<Metric = bool> {}

#[cfg(test)]
#[path = "unit_tests/traits.rs"]
mod tests;
