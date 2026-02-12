//! Common types used across the problemreductions library.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Marker trait for numeric weight types.
///
/// Weight subsumption uses Rust's `From` trait:
/// - `i32 → f64` is valid (`From<i32>` for f64 exists)
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

/// Trait for weight storage. Separates weight storage from objective value type.
pub trait Weights: Clone + 'static {
    /// Name for variant metadata (e.g., "Unweighted", "`Weighted<i32>`").
    const NAME: &'static str;
    /// The objective/metric type derived from these weights.
    type Size: NumericSize;
    /// Get the weight at a given index.
    fn weight(&self, index: usize) -> Self::Size;
    /// Number of weights.
    fn len(&self) -> usize;
    /// Whether the weight vector is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Marker type for unweighted problems.
///
/// When constructed with `Unweighted(n)`, it represents `n` unit weights (all equal to 1).
/// When constructed with `Unweighted` (the zero-sized default), it serves as a type marker.
///
/// # Example
///
/// ```
/// use problemreductions::types::{Unweighted, Weights};
///
/// let w = Unweighted(5);
/// assert_eq!(w.len(), 5);
/// assert_eq!(w.weight(0), 1);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct Unweighted(pub usize);

impl Unweighted {
    /// Returns 1 for any index (all weights are unit).
    pub fn get(&self, _index: usize) -> i32 {
        1
    }
}

impl Weights for Unweighted {
    const NAME: &'static str = "Unweighted";
    type Size = i32;
    fn weight(&self, _index: usize) -> i32 {
        1
    }
    fn len(&self) -> usize {
        self.0
    }
}

impl Weights for Vec<i32> {
    const NAME: &'static str = "Weighted<i32>";
    type Size = i32;
    fn weight(&self, index: usize) -> i32 {
        self[index]
    }
    fn len(&self) -> usize {
        self.len()
    }
}

impl Weights for Vec<f64> {
    const NAME: &'static str = "Weighted<f64>";
    type Size = f64;
    fn weight(&self, index: usize) -> f64 {
        self[index]
    }
    fn len(&self) -> usize {
        self.len()
    }
}

impl std::fmt::Display for Unweighted {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unweighted")
    }
}

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
