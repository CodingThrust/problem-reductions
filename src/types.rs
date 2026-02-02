//! Common types used across the problemreductions library.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Marker trait for numeric weight types.
///
/// Weight subsumption uses Rust's `From` trait:
/// - `i32 → f64` is valid (From<i32> for f64 exists)
/// - `f64 → i32` is invalid (no lossless conversion)
pub trait NumericWeight:
    Clone + Default + PartialOrd + num_traits::Num + num_traits::Zero + std::ops::AddAssign + 'static
{
}

// Blanket implementation for any type satisfying the bounds
impl<T> NumericWeight for T where
    T: Clone
        + Default
        + PartialOrd
        + num_traits::Num
        + num_traits::Zero
        + std::ops::AddAssign
        + 'static
{
}

/// Marker type for unweighted problems.
///
/// Similar to Julia's `UnitWeight`, this type indicates that a problem
/// has uniform weights (all equal to 1). Used to distinguish unweighted
/// problem variants from weighted ones in the type system.
///
/// # Example
///
/// ```
/// use problemreductions::types::Unweighted;
///
/// // Problems can be parameterized by weight type:
/// // - `IndependentSet<Unweighted>` - unweighted (default)
/// // - `IndependentSet<i32>` - weighted with integer weights
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct Unweighted;

impl Unweighted {
    /// Returns 1 for any index (all weights are unit).
    pub fn get(&self, _index: usize) -> i32 {
        1
    }
}

impl std::fmt::Display for Unweighted {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unweighted")
    }
}

/// Specifies whether larger or smaller objective values are better.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnergyMode {
    /// Larger objective values are better (maximization).
    LargerSizeIsBetter,
    /// Smaller objective values are better (minimization).
    SmallerSizeIsBetter,
}

impl EnergyMode {
    /// Returns true if this mode prefers larger values.
    pub fn is_maximization(&self) -> bool {
        matches!(self, EnergyMode::LargerSizeIsBetter)
    }

    /// Returns true if this mode prefers smaller values.
    pub fn is_minimization(&self) -> bool {
        matches!(self, EnergyMode::SmallerSizeIsBetter)
    }

    /// Compare two values according to this energy mode.
    /// Returns true if `a` is better than `b`.
    pub fn is_better<T: PartialOrd>(&self, a: &T, b: &T) -> bool {
        match self {
            EnergyMode::LargerSizeIsBetter => a > b,
            EnergyMode::SmallerSizeIsBetter => a < b,
        }
    }

    /// Compare two values according to this energy mode.
    /// Returns true if `a` is better than or equal to `b`.
    pub fn is_better_or_equal<T: PartialOrd>(&self, a: &T, b: &T) -> bool {
        match self {
            EnergyMode::LargerSizeIsBetter => a >= b,
            EnergyMode::SmallerSizeIsBetter => a <= b,
        }
    }
}

/// The result of evaluating a solution's size/energy.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SolutionSize<T> {
    /// The objective value of the solution.
    pub size: T,
    /// Whether the solution satisfies all constraints.
    pub is_valid: bool,
}

impl<T> SolutionSize<T> {
    /// Create a new valid solution size.
    pub fn valid(size: T) -> Self {
        Self {
            size,
            is_valid: true,
        }
    }

    /// Create a new invalid solution size.
    pub fn invalid(size: T) -> Self {
        Self {
            size,
            is_valid: false,
        }
    }

    /// Create a new solution size with explicit validity.
    pub fn new(size: T, is_valid: bool) -> Self {
        Self { size, is_valid }
    }
}

impl<T: fmt::Display> fmt::Display for SolutionSize<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SolutionSize({}, {})",
            self.size,
            if self.is_valid { "valid" } else { "invalid" }
        )
    }
}

/// Problem size metadata (varies by problem type).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProblemSize {
    /// Named size components.
    pub components: Vec<(String, usize)>,
}

impl ProblemSize {
    /// Create a new problem size with named components.
    pub fn new(components: Vec<(&str, usize)>) -> Self {
        Self {
            components: components
                .into_iter()
                .map(|(k, v)| (k.to_string(), v))
                .collect(),
        }
    }

    /// Get a size component by name.
    pub fn get(&self, name: &str) -> Option<usize> {
        self.components
            .iter()
            .find(|(k, _)| k == name)
            .map(|(_, v)| *v)
    }
}

impl fmt::Display for ProblemSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ProblemSize{{")?;
        for (i, (name, value)) in self.components.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", name, value)?;
        }
        write!(f, "}}")
    }
}

/// A local constraint on a subset of variables.
///
/// The constraint specifies which configurations of the variables are valid.
/// The `spec` vector is indexed by the configuration value (treating variables
/// as digits in a base-`num_flavors` number).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalConstraint {
    /// Number of flavors (domain size) for each variable.
    pub num_flavors: usize,
    /// Indices of variables involved in this constraint.
    pub variables: Vec<usize>,
    /// Specification vector: `spec[config]` = true if config is valid.
    /// Length must be num_flavors^variables.len().
    pub spec: Vec<bool>,
}

impl LocalConstraint {
    /// Create a new local constraint.
    pub fn new(num_flavors: usize, variables: Vec<usize>, spec: Vec<bool>) -> Self {
        debug_assert_eq!(
            spec.len(),
            num_flavors.pow(variables.len() as u32),
            "spec length must be num_flavors^num_variables"
        );
        Self {
            num_flavors,
            variables,
            spec,
        }
    }

    /// Check if a configuration satisfies this constraint.
    pub fn is_satisfied(&self, config: &[usize]) -> bool {
        let index = self.config_to_index(config);
        self.spec.get(index).copied().unwrap_or(false)
    }

    /// Convert a full configuration to an index into the spec vector.
    fn config_to_index(&self, config: &[usize]) -> usize {
        let mut index = 0;
        for (i, &var) in self.variables.iter().enumerate() {
            let value = config.get(var).copied().unwrap_or(0);
            index += value * self.num_flavors.pow((self.variables.len() - 1 - i) as u32);
        }
        index
    }

    /// Get the number of variables in this constraint.
    pub fn num_variables(&self) -> usize {
        self.variables.len()
    }
}

/// A local contribution to the solution size from a subset of variables.
///
/// Similar to LocalConstraint but stores objective values instead of validity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocalSolutionSize<T> {
    /// Number of flavors (domain size) for each variable.
    pub num_flavors: usize,
    /// Indices of variables involved.
    pub variables: Vec<usize>,
    /// Specification vector: `spec[config]` = contribution for that config.
    /// Length must be num_flavors^variables.len().
    pub spec: Vec<T>,
}

impl<T: Clone> LocalSolutionSize<T> {
    /// Create a new local solution size.
    pub fn new(num_flavors: usize, variables: Vec<usize>, spec: Vec<T>) -> Self {
        debug_assert_eq!(
            spec.len(),
            num_flavors.pow(variables.len() as u32),
            "spec length must be num_flavors^num_variables"
        );
        Self {
            num_flavors,
            variables,
            spec,
        }
    }

    /// Get the contribution from a configuration.
    pub fn evaluate(&self, config: &[usize]) -> T {
        let index = self.config_to_index(config);
        self.spec[index].clone()
    }

    /// Convert a full configuration to an index into the spec vector.
    fn config_to_index(&self, config: &[usize]) -> usize {
        let mut index = 0;
        for (i, &var) in self.variables.iter().enumerate() {
            let value = config.get(var).copied().unwrap_or(0);
            index += value * self.num_flavors.pow((self.variables.len() - 1 - i) as u32);
        }
        index
    }

    /// Get the number of variables in this local objective.
    pub fn num_variables(&self) -> usize {
        self.variables.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energy_mode() {
        let max_mode = EnergyMode::LargerSizeIsBetter;
        let min_mode = EnergyMode::SmallerSizeIsBetter;

        assert!(max_mode.is_maximization());
        assert!(!max_mode.is_minimization());
        assert!(!min_mode.is_maximization());
        assert!(min_mode.is_minimization());

        assert!(max_mode.is_better(&10, &5));
        assert!(!max_mode.is_better(&5, &10));
        assert!(min_mode.is_better(&5, &10));
        assert!(!min_mode.is_better(&10, &5));

        assert!(max_mode.is_better_or_equal(&10, &10));
        assert!(min_mode.is_better_or_equal(&10, &10));
    }

    #[test]
    fn test_solution_size() {
        let valid = SolutionSize::valid(42);
        assert_eq!(valid.size, 42);
        assert!(valid.is_valid);

        let invalid = SolutionSize::invalid(0);
        assert!(!invalid.is_valid);

        let custom = SolutionSize::new(100, false);
        assert_eq!(custom.size, 100);
        assert!(!custom.is_valid);
    }

    #[test]
    fn test_solution_size_display() {
        let valid = SolutionSize::valid(42);
        assert_eq!(format!("{}", valid), "SolutionSize(42, valid)");

        let invalid = SolutionSize::invalid(0);
        assert_eq!(format!("{}", invalid), "SolutionSize(0, invalid)");
    }

    #[test]
    fn test_problem_size() {
        let ps = ProblemSize::new(vec![("vertices", 10), ("edges", 20)]);
        assert_eq!(ps.get("vertices"), Some(10));
        assert_eq!(ps.get("edges"), Some(20));
        assert_eq!(ps.get("unknown"), None);
    }

    #[test]
    fn test_problem_size_display() {
        let ps = ProblemSize::new(vec![("vertices", 10), ("edges", 20)]);
        assert_eq!(format!("{}", ps), "ProblemSize{vertices: 10, edges: 20}");

        let empty = ProblemSize::new(vec![]);
        assert_eq!(format!("{}", empty), "ProblemSize{}");

        let single = ProblemSize::new(vec![("n", 5)]);
        assert_eq!(format!("{}", single), "ProblemSize{n: 5}");
    }

    #[test]
    fn test_local_constraint() {
        // Binary constraint on 2 variables: only (0,0) and (1,1) are valid
        let constraint = LocalConstraint::new(2, vec![0, 1], vec![true, false, false, true]);

        assert!(constraint.is_satisfied(&[0, 0]));
        assert!(!constraint.is_satisfied(&[0, 1]));
        assert!(!constraint.is_satisfied(&[1, 0]));
        assert!(constraint.is_satisfied(&[1, 1]));
        assert_eq!(constraint.num_variables(), 2);
    }

    #[test]
    fn test_local_constraint_out_of_bounds() {
        let constraint = LocalConstraint::new(2, vec![5, 6], vec![true, false, false, true]);
        // Test with config that doesn't have indices 5 and 6 - defaults to 0
        assert!(constraint.is_satisfied(&[0, 0, 0]));
    }

    #[test]
    fn test_local_solution_size() {
        // Binary objective on 1 variable: weight 0 for 0, weight 5 for 1
        let objective = LocalSolutionSize::new(2, vec![0], vec![0, 5]);

        assert_eq!(objective.evaluate(&[0]), 0);
        assert_eq!(objective.evaluate(&[1]), 5);
        assert_eq!(objective.num_variables(), 1);
    }

    #[test]
    fn test_local_solution_size_multi_variable() {
        // Binary objective on 2 variables
        let objective = LocalSolutionSize::new(2, vec![0, 1], vec![0, 1, 2, 3]);
        assert_eq!(objective.evaluate(&[0, 0]), 0);
        assert_eq!(objective.evaluate(&[0, 1]), 1);
        assert_eq!(objective.evaluate(&[1, 0]), 2);
        assert_eq!(objective.evaluate(&[1, 1]), 3);
    }

    #[test]
    fn test_numeric_weight_impls() {
        fn assert_numeric_weight<T: NumericWeight>() {}

        assert_numeric_weight::<i32>();
        assert_numeric_weight::<f64>();
        assert_numeric_weight::<i64>();
        assert_numeric_weight::<f32>();
    }
}
