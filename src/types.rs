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

impl<V> Max<V> {
    pub fn is_valid(&self) -> bool {
        self.0.is_some()
    }

    pub fn size(&self) -> Option<&V> {
        self.0.as_ref()
    }

    pub fn unwrap(self) -> V {
        self.0.expect("called unwrap on invalid Max value")
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

impl<V> Min<V> {
    pub fn is_valid(&self) -> bool {
        self.0.is_some()
    }

    pub fn size(&self) -> Option<&V> {
        self.0.as_ref()
    }

    pub fn unwrap(self) -> V {
        self.0.expect("called unwrap on invalid Min value")
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

impl Or {
    pub fn is_valid(&self) -> bool {
        self.0
    }

    pub fn unwrap(self) -> bool {
        self.0
    }
}

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

impl std::ops::Not for Or {
    type Output = bool;

    fn not(self) -> Self::Output {
        !self.0
    }
}

impl PartialEq<bool> for Or {
    fn eq(&self, other: &bool) -> bool {
        self.0 == *other
    }
}

impl PartialEq<Or> for bool {
    fn eq(&self, other: &Or) -> bool {
        *self == other.0
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExtremumSense {
    Maximize,
    Minimize,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Extremum<V> {
    pub sense: ExtremumSense,
    pub value: Option<V>,
}

impl<V> Extremum<V> {
    pub fn maximize(value: Option<V>) -> Self {
        Self {
            sense: ExtremumSense::Maximize,
            value,
        }
    }

    pub fn minimize(value: Option<V>) -> Self {
        Self {
            sense: ExtremumSense::Minimize,
            value,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.value.is_some()
    }

    pub fn size(&self) -> Option<&V> {
        self.value.as_ref()
    }

    pub fn unwrap(self) -> V {
        self.value.expect("called unwrap on invalid Extremum value")
    }
}

impl<V: fmt::Debug + PartialOrd + Clone + Serialize + DeserializeOwned> Aggregate for Extremum<V> {
    fn identity() -> Self {
        Self::maximize(None)
    }

    fn combine(self, other: Self) -> Self {
        use std::cmp::Ordering;

        match (self.value, other.value) {
            (None, rhs) => Self {
                sense: other.sense,
                value: rhs,
            },
            (lhs, None) => Self {
                sense: self.sense,
                value: lhs,
            },
            (Some(lhs), Some(rhs)) => {
                assert_eq!(
                    self.sense, other.sense,
                    "cannot combine Extremum values with different senses"
                );
                let ord = lhs.partial_cmp(&rhs).expect("cannot compare values (NaN?)");
                let keep_self = match self.sense {
                    ExtremumSense::Maximize => matches!(ord, Ordering::Equal | Ordering::Greater),
                    ExtremumSense::Minimize => matches!(ord, Ordering::Equal | Ordering::Less),
                };
                if keep_self {
                    Self {
                        sense: self.sense,
                        value: Some(lhs),
                    }
                } else {
                    Self {
                        sense: other.sense,
                        value: Some(rhs),
                    }
                }
            }
        }
    }

    fn supports_witnesses() -> bool {
        true
    }

    fn contributes_to_witnesses(config_value: &Self, total: &Self) -> bool {
        matches!(
            (config_value.value.as_ref(), total.value.as_ref()),
            (Some(value), Some(best)) if config_value.sense == total.sense && value == best
        )
    }
}

impl<V: fmt::Display> fmt::Display for Extremum<V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (&self.sense, &self.value) {
            (ExtremumSense::Maximize, Some(value)) => write!(f, "Max({value})"),
            (ExtremumSense::Maximize, None) => write!(f, "Max(None)"),
            (ExtremumSense::Minimize, Some(value)) => write!(f, "Min({value})"),
            (ExtremumSense::Minimize, None) => write!(f, "Min(None)"),
        }
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

use crate::impl_variant_param;

impl_variant_param!(f64, "weight");
impl_variant_param!(i32, "weight", parent: f64, cast: |w| *w as f64);
impl_variant_param!(One, "weight", parent: i32, cast: |_| 1i32);

#[cfg(test)]
#[path = "unit_tests/types.rs"]
mod tests;
