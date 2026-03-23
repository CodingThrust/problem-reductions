//! Common types used across the problemreductions library.

use serde::de::{self, DeserializeOwned, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
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
    /// Whether this is the unit weight type (`One`).
    const IS_UNIT: bool;
    /// Convert this weight element to the sum type.
    fn to_sum(&self) -> Self::Sum;
}

impl WeightElement for i32 {
    type Sum = i32;
    const IS_UNIT: bool = false;
    fn to_sum(&self) -> i32 {
        *self
    }
}

impl WeightElement for f64 {
    type Sum = f64;
    const IS_UNIT: bool = false;
    fn to_sum(&self) -> f64 {
        *self
    }
}

/// The constant 1. Unit weight for unweighted problems.
///
/// When used as the weight type parameter `W`, indicates that all weights
/// are uniformly 1. `One::to_sum()` returns `1i32`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct One;

impl Serialize for One {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(1)
    }
}

impl<'de> Deserialize<'de> for One {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OneVisitor;

        impl<'de> Visitor<'de> for OneVisitor {
            type Value = One;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("the unit weight `One` encoded as 1 or unit/null")
            }

            fn visit_i64<E>(self, value: i64) -> Result<One, E>
            where
                E: de::Error,
            {
                if value == 1 {
                    Ok(One)
                } else {
                    Err(E::custom(format!("expected 1 for One, got {value}")))
                }
            }

            fn visit_u64<E>(self, value: u64) -> Result<One, E>
            where
                E: de::Error,
            {
                if value == 1 {
                    Ok(One)
                } else {
                    Err(E::custom(format!("expected 1 for One, got {value}")))
                }
            }

            fn visit_unit<E>(self) -> Result<One, E>
            where
                E: de::Error,
            {
                Ok(One)
            }

            fn visit_none<E>(self) -> Result<One, E>
            where
                E: de::Error,
            {
                Ok(One)
            }

            fn visit_str<E>(self, value: &str) -> Result<One, E>
            where
                E: de::Error,
            {
                if value == "One" {
                    Ok(One)
                } else {
                    Err(E::custom(format!("expected \"One\" for One, got {value}")))
                }
            }
        }

        deserializer.deserialize_any(OneVisitor)
    }
}

impl WeightElement for One {
    type Sum = i32;
    const IS_UNIT: bool = true;
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

/// Foldable aggregate values for enumerating a problem's configuration space.
pub trait Aggregate: Clone + fmt::Debug + Serialize + DeserializeOwned {
    /// Neutral element for folding.
    fn identity() -> Self;

    /// Associative combine operation.
    fn combine(self, other: Self) -> Self;

    /// Whether this aggregate admits representative witness configurations.
    fn supports_witnesses() -> bool {
        false
    }

    /// Whether a configuration-level value belongs to the witness set
    /// for the final aggregate value.
    fn contributes_to_witnesses(_config_value: &Self, _total: &Self) -> bool {
        false
    }
}

/// Maximum aggregate over feasible values.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Max<V>(pub Option<V>);

impl<V: fmt::Debug + PartialOrd + Clone + Serialize + DeserializeOwned> Aggregate for Max<V> {
    fn identity() -> Self {
        Max(None)
    }

    fn combine(self, other: Self) -> Self {
        use std::cmp::Ordering;

        match (self.0, other.0) {
            (None, rhs) => Max(rhs),
            (lhs, None) => Max(lhs),
            (Some(lhs), Some(rhs)) => {
                let ord = lhs.partial_cmp(&rhs).expect("cannot compare values (NaN?)");
                match ord {
                    Ordering::Less => Max(Some(rhs)),
                    Ordering::Equal | Ordering::Greater => Max(Some(lhs)),
                }
            }
        }
    }

    fn supports_witnesses() -> bool {
        true
    }

    fn contributes_to_witnesses(config_value: &Self, total: &Self) -> bool {
        matches!((config_value, total), (Max(Some(value)), Max(Some(best))) if value == best)
    }
}

impl<V: fmt::Display> fmt::Display for Max<V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Some(value) => write!(f, "Max({value})"),
            None => write!(f, "Max(None)"),
        }
    }
}

/// Minimum aggregate over feasible values.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Min<V>(pub Option<V>);

impl<V: fmt::Debug + PartialOrd + Clone + Serialize + DeserializeOwned> Aggregate for Min<V> {
    fn identity() -> Self {
        Min(None)
    }

    fn combine(self, other: Self) -> Self {
        use std::cmp::Ordering;

        match (self.0, other.0) {
            (None, rhs) => Min(rhs),
            (lhs, None) => Min(lhs),
            (Some(lhs), Some(rhs)) => {
                let ord = lhs.partial_cmp(&rhs).expect("cannot compare values (NaN?)");
                match ord {
                    Ordering::Greater => Min(Some(rhs)),
                    Ordering::Equal | Ordering::Less => Min(Some(lhs)),
                }
            }
        }
    }

    fn supports_witnesses() -> bool {
        true
    }

    fn contributes_to_witnesses(config_value: &Self, total: &Self) -> bool {
        matches!((config_value, total), (Min(Some(value)), Min(Some(best))) if value == best)
    }
}

impl<V: fmt::Display> fmt::Display for Min<V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Some(value) => write!(f, "Min({value})"),
            None => write!(f, "Min(None)"),
        }
    }
}

/// Sum aggregate for value-only problems.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Sum<W>(pub W);

impl<W: fmt::Debug + NumericSize + Serialize + DeserializeOwned> Aggregate for Sum<W> {
    fn identity() -> Self {
        Sum(W::zero())
    }

    fn combine(self, other: Self) -> Self {
        let mut total = self.0;
        total += other.0;
        Sum(total)
    }
}

impl<W: fmt::Display> fmt::Display for Sum<W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Sum({})", self.0)
    }
}

/// Disjunction aggregate for existential satisfaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Or(pub bool);

impl Aggregate for Or {
    fn identity() -> Self {
        Or(false)
    }

    fn combine(self, other: Self) -> Self {
        Or(self.0 || other.0)
    }

    fn supports_witnesses() -> bool {
        true
    }

    fn contributes_to_witnesses(config_value: &Self, total: &Self) -> bool {
        config_value.0 && total.0
    }
}

impl fmt::Display for Or {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Or({})", self.0)
    }
}

/// Conjunction aggregate for universal satisfaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct And(pub bool);

impl Aggregate for And {
    fn identity() -> Self {
        And(true)
    }

    fn combine(self, other: Self) -> Self {
        And(self.0 && other.0)
    }
}

impl fmt::Display for And {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "And({})", self.0)
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

use crate::impl_variant_param;

impl_variant_param!(f64, "weight");
impl_variant_param!(i32, "weight", parent: f64, cast: |w| *w as f64);
impl_variant_param!(One, "weight", parent: i32, cast: |_| 1i32);

#[cfg(test)]
#[path = "unit_tests/types.rs"]
mod tests;
