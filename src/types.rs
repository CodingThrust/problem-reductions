//! Common types used across the problemreductions library.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Bound for objective value types (i32, f64, etc.)
pub trait NumericSize:
    Clone
    + Default
    + PartialOrd
    + num_traits::Num
    + num_traits::Zero
    + num_traits::Bounded
    + std::ops::AddAssign
    + 'static
{
}

impl<T> NumericSize for T where
    T: Clone
        + Default
        + PartialOrd
        + num_traits::Num
        + num_traits::Zero
        + num_traits::Bounded
        + std::ops::AddAssign
        + 'static
{
}

/// Maps a weight element to its sum/metric type.
///
/// This decouples the per-element weight type from the accumulation type.
/// For concrete weights (`i32`, `f64`), `Sum` is the same type.
/// For the unit weight `One`, `Sum = i32`.
pub trait WeightElement: Clone + Default + 'static {
    /// The numeric type used for sums and comparisons.
    type Sum: NumericSize;
    /// Convert this weight element to the sum type.
    fn to_sum(&self) -> Self::Sum;
}

impl WeightElement for i32 {
    type Sum = i32;
    fn to_sum(&self) -> i32 {
        *self
    }
}

impl WeightElement for f64 {
    type Sum = f64;
    fn to_sum(&self) -> f64 {
        *self
    }
}

/// The constant 1. Unit weight for unweighted problems.
///
/// When used as the weight type parameter `W`, indicates that all weights
/// are uniformly 1. `One::to_sum()` returns `1i32`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct One;

impl WeightElement for One {
    type Sum = i32;
    fn to_sum(&self) -> i32 {
        1
    }
}

impl std::fmt::Display for One {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "One")
    }
}

impl From<i32> for One {
    fn from(_: i32) -> Self {
        One
    }
}

/// Backward-compatible alias for `One`.
pub type Unweighted = One;

/// Result of evaluating a constrained optimization problem.
///
/// For optimization problems with constraints (like MaximumIndependentSet),
/// configurations may be infeasible. This enum explicitly represents validity.
///
/// # Example
///
/// ```
/// use problemreductions::types::SolutionSize;
///
/// let valid = SolutionSize::Valid(42);
/// assert!(valid.is_valid());
/// assert_eq!(valid.size(), Some(&42));
///
/// let invalid: SolutionSize<i32> = SolutionSize::Invalid;
/// assert!(!invalid.is_valid());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum SolutionSize<T> {
    /// A valid (feasible) solution with the given objective value.
    Valid(T),
    /// An invalid (infeasible) solution that violates constraints.
    #[default]
    Invalid,
}

impl<T> SolutionSize<T> {
    /// Returns true if this is a valid solution.
    pub fn is_valid(&self) -> bool {
        matches!(self, SolutionSize::Valid(_))
    }

    /// Returns the size if valid, None if invalid.
    pub fn size(&self) -> Option<&T> {
        match self {
            SolutionSize::Valid(t) => Some(t),
            SolutionSize::Invalid => None,
        }
    }

    /// Unwraps the size, panicking if invalid.
    pub fn unwrap(self) -> T {
        match self {
            SolutionSize::Valid(t) => t,
            SolutionSize::Invalid => panic!("called unwrap on Invalid SolutionSize"),
        }
    }

    /// Maps the inner value if valid.
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> SolutionSize<U> {
        match self {
            SolutionSize::Valid(t) => SolutionSize::Valid(f(t)),
            SolutionSize::Invalid => SolutionSize::Invalid,
        }
    }
}

impl<T: PartialOrd> SolutionSize<T> {
    /// Returns true if self is a better solution than other for the given direction.
    ///
    /// - For maximization: larger values are better
    /// - For minimization: smaller values are better
    /// - Valid solutions are always better than invalid ones
    /// - Two invalid solutions are equally bad (neither is better)
    ///
    /// # Panics
    ///
    /// Panics if comparing two valid values that are not comparable (e.g., NaN for f64).
    pub fn is_better(&self, other: &Self, direction: Direction) -> bool {
        match (self, other) {
            (SolutionSize::Valid(a), SolutionSize::Valid(b)) => {
                use std::cmp::Ordering;
                let ord = a.partial_cmp(b).expect("cannot compare values (NaN?)");
                match direction {
                    Direction::Maximize => ord == Ordering::Greater,
                    Direction::Minimize => ord == Ordering::Less,
                }
            }
            (SolutionSize::Valid(_), SolutionSize::Invalid) => true,
            (SolutionSize::Invalid, SolutionSize::Valid(_)) => false,
            (SolutionSize::Invalid, SolutionSize::Invalid) => false,
        }
    }
}

/// Optimization direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Direction {
    /// Maximize the objective value.
    Maximize,
    /// Minimize the objective value.
    Minimize,
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

#[cfg(test)]
#[path = "unit_tests/types.rs"]
mod tests;
