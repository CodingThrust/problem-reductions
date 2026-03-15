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

    /// Look up this problem's catalog entry.
    ///
    /// Returns the full [`ProblemType`] metadata from the catalog registry.
    /// The default implementation uses `Self::NAME` to perform the lookup.
    fn problem_type() -> crate::registry::ProblemType {
        crate::registry::find_problem_type(Self::NAME)
            .unwrap_or_else(|| panic!("no catalog entry for Problem::NAME = {:?}", Self::NAME))
    }
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

/// Marker trait for explicitly declared problem variants.
///
/// Implemented automatically by [`declare_variants!`] for each concrete type.
/// The [`#[reduction]`] proc macro checks this trait at compile time to ensure
/// all reduction source/target types have been declared.
pub trait DeclaredVariant {}

#[cfg(test)]
#[path = "unit_tests/traits.rs"]
mod tests;
