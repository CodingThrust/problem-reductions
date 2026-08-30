//! Common types used across the problemreductions library.

use serde::de::{self, DeserializeOwned, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

/// Largest integer magnitude represented exactly by an IEEE 754 `f64`.
pub const MAX_EXACT_F64_INTEGER: i64 = (1_i64 << 53) - 1;

/// An `i64` cannot cross an exact-integer `f64` boundary without precision loss.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error(
    "integer {value} is outside the exactly representable f64 range [{min}, {max}]",
    min = -MAX_EXACT_F64_INTEGER,
    max = MAX_EXACT_F64_INTEGER
)]
pub struct ExactI64ToF64Error {
    pub value: i64,
}

/// Failure while performing checked arithmetic on a numeric value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum NumericArithmeticError {
    /// An exact integer result is outside the numeric type's range.
    #[error("integer overflow")]
    IntegerOverflow,
    /// A floating-point result is not finite.
    #[error("non-finite floating-point result")]
    NonFiniteResult,
}

/// Convert an `i64` to `f64` only when the integer value remains exact.
pub fn i64_to_exact_f64(value: i64) -> Result<f64, ExactI64ToF64Error> {
    if (-MAX_EXACT_F64_INTEGER..=MAX_EXACT_F64_INTEGER).contains(&value) {
        Ok(value as f64)
    } else {
        Err(ExactI64ToF64Error { value })
    }
}

/// Bound for objective value types (i64, f64, etc.)
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
    /// Add two values when the exact result remains representable and finite.
    fn checked_add_value(self, other: Self) -> Result<Self, NumericArithmeticError>;
    /// Multiply two values when the exact result remains representable and finite.
    fn checked_mul_value(self, other: Self) -> Result<Self, NumericArithmeticError>;
}

macro_rules! impl_integer_numeric_size {
    ($($type:ty),* $(,)?) => {
        $(
            impl NumericSize for $type {
                fn checked_add_value(self, other: Self) -> Result<Self, NumericArithmeticError> {
                    self.checked_add(other).ok_or(NumericArithmeticError::IntegerOverflow)
                }

                fn checked_mul_value(self, other: Self) -> Result<Self, NumericArithmeticError> {
                    self.checked_mul(other).ok_or(NumericArithmeticError::IntegerOverflow)
                }
            }
        )*
    };
}

impl_integer_numeric_size!(i64, u64, usize);

impl NumericSize for f64 {
    fn checked_add_value(self, other: Self) -> Result<Self, NumericArithmeticError> {
        let result = self + other;
        result
            .is_finite()
            .then_some(result)
            .ok_or(NumericArithmeticError::NonFiniteResult)
    }

    fn checked_mul_value(self, other: Self) -> Result<Self, NumericArithmeticError> {
        let result = self * other;
        result
            .is_finite()
            .then_some(result)
            .ok_or(NumericArithmeticError::NonFiniteResult)
    }
}

fn evaluation_arithmetic_error(
    error: NumericArithmeticError,
    context: &str,
) -> crate::traits::EvaluationError {
    match error {
        NumericArithmeticError::IntegerOverflow => {
            crate::traits::EvaluationError::IntegerOverflow(context.to_string())
        }
        NumericArithmeticError::NonFiniteResult => {
            crate::traits::EvaluationError::NonFiniteResult(context.to_string())
        }
    }
}

/// Maps a weight element to its sum/metric type.
///
/// This decouples the per-element weight type from the accumulation type.
/// Exact integer weights use a wider accumulation type: `i64` and the unit
/// weight [`One`] both use `i64`. Approximate `f64` weights continue to sum
/// into `f64`.
pub trait WeightElement: Clone + Default + 'static {
    /// The numeric type used for sums and comparisons.
    type Sum: NumericSize;
    /// Whether this is the unit weight type (`One`).
    const IS_UNIT: bool;
    /// Construct the multiplicative unit weight.
    fn unit() -> Self;
    /// Validate that an element belongs to the public weight domain.
    fn validate_element(&self, context: &str) -> Result<(), crate::registry::ConstructionError>;
    /// Convert this weight element to the sum type.
    fn to_sum(&self) -> Self::Sum;
    /// Add one element to an evaluated objective without overflowing or producing a non-finite value.
    fn checked_add_to_sum(
        total: Self::Sum,
        value: Self::Sum,
        context: &str,
    ) -> Result<Self::Sum, crate::traits::EvaluationError> {
        total
            .checked_add_value(value)
            .map_err(|error| evaluation_arithmetic_error(error, context))
    }
    /// Multiply evaluated quantities without overflowing or producing a non-finite value.
    fn checked_mul_sum(
        left: Self::Sum,
        right: Self::Sum,
        context: &str,
    ) -> Result<Self::Sum, crate::traits::EvaluationError> {
        left.checked_mul_value(right)
            .map_err(|error| evaluation_arithmetic_error(error, context))
    }
}

impl WeightElement for i64 {
    type Sum = i64;
    const IS_UNIT: bool = false;
    fn unit() -> Self {
        1
    }
    fn validate_element(&self, _context: &str) -> Result<(), crate::registry::ConstructionError> {
        Ok(())
    }
    fn to_sum(&self) -> i64 {
        *self
    }
}

impl WeightElement for f64 {
    type Sum = f64;
    const IS_UNIT: bool = false;
    fn unit() -> Self {
        1.0
    }
    fn validate_element(&self, context: &str) -> Result<(), crate::registry::ConstructionError> {
        if self.is_finite() {
            Ok(())
        } else {
            Err(crate::registry::ConstructionError::NonFiniteFloat(format!(
                "{context} must be finite"
            )))
        }
    }
    fn to_sum(&self) -> f64 {
        *self
    }
}

/// The constant 1. Unit weight for unweighted problems.
///
/// When used as the weight type parameter `W`, indicates that all weights
/// are uniformly 1. `One::to_sum()` returns `1i64`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct One;

impl Serialize for One {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(1)
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
    type Sum = i64;
    const IS_UNIT: bool = true;
    fn unit() -> Self {
        One
    }
    fn validate_element(&self, _context: &str) -> Result<(), crate::registry::ConstructionError> {
        Ok(())
    }
    fn to_sum(&self) -> i64 {
        1
    }
}

impl std::fmt::Display for One {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "One")
    }
}

/// Failure while combining configuration values during a solve.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum AggregationError {
    #[error("aggregate arithmetic overflow or non-finite result")]
    ArithmeticOverflow,
    #[error("aggregate values are not comparable")]
    UnorderedComparison,
    #[error("cannot combine extrema with different optimization senses")]
    IncompatibleExtremumSense,
}

/// Foldable aggregate values for enumerating a problem's configuration space.
pub trait Aggregate: Clone + fmt::Debug + Serialize + DeserializeOwned {
    /// Neutral element for folding.
    fn identity() -> Self;

    /// Associative combine operation.
    fn combine(self, other: Self) -> Result<Self, AggregationError>;

    /// Whether no further configuration can change this aggregate value.
    fn is_absorbing(&self) -> bool {
        false
    }
}

/// Aggregate value whose optimum identifies contributing solutions.
pub trait SolutionAggregate: Aggregate {
    /// Whether a solution-level value contributes to the final aggregate value.
    fn contributes_to_solution(value: &Self, total: &Self) -> bool;
}

/// Maximum aggregate over feasible values.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Max<V>(pub Option<V>);

impl<V: fmt::Debug + PartialOrd + Clone + Serialize + DeserializeOwned> Aggregate for Max<V> {
    fn identity() -> Self {
        Max(None)
    }

    fn combine(self, other: Self) -> Result<Self, AggregationError> {
        use std::cmp::Ordering;

        Ok(match (self.0, other.0) {
            (None, rhs) => Max(rhs),
            (lhs, None) => Max(lhs),
            (Some(lhs), Some(rhs)) => {
                let ord = lhs
                    .partial_cmp(&rhs)
                    .ok_or(AggregationError::UnorderedComparison)?;
                match ord {
                    Ordering::Less => Max(Some(rhs)),
                    Ordering::Equal | Ordering::Greater => Max(Some(lhs)),
                }
            }
        })
    }
}

impl<V: fmt::Debug + PartialOrd + Clone + Serialize + DeserializeOwned> SolutionAggregate
    for Max<V>
{
    fn contributes_to_solution(value: &Self, total: &Self) -> bool {
        matches!((value, total), (Max(Some(value)), Max(Some(best))) if value == best)
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

    fn combine(self, other: Self) -> Result<Self, AggregationError> {
        use std::cmp::Ordering;

        Ok(match (self.0, other.0) {
            (None, rhs) => Min(rhs),
            (lhs, None) => Min(lhs),
            (Some(lhs), Some(rhs)) => {
                let ord = lhs
                    .partial_cmp(&rhs)
                    .ok_or(AggregationError::UnorderedComparison)?;
                match ord {
                    Ordering::Greater => Min(Some(rhs)),
                    Ordering::Equal | Ordering::Less => Min(Some(lhs)),
                }
            }
        })
    }
}

impl<V: fmt::Debug + PartialOrd + Clone + Serialize + DeserializeOwned> SolutionAggregate
    for Min<V>
{
    fn contributes_to_solution(value: &Self, total: &Self) -> bool {
        matches!((value, total), (Min(Some(value)), Min(Some(best))) if value == best)
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

/// Trait for aggregate values that represent optimization objectives.
pub trait OptimizationValue: Aggregate {
    /// The inner numeric type used for comparisons with decision bounds.
    type Inner: Clone + PartialOrd + fmt::Debug + Serialize + DeserializeOwned;

    /// Whether this aggregate value satisfies the provided decision bound.
    fn meets_bound(value: &Self, bound: &Self::Inner) -> bool;
}

impl<V: fmt::Debug + PartialOrd + Clone + Serialize + DeserializeOwned> OptimizationValue
    for Min<V>
{
    type Inner = V;

    fn meets_bound(value: &Self, bound: &V) -> bool {
        matches!(&value.0, Some(v) if *v <= *bound)
    }
}

impl<V: fmt::Debug + PartialOrd + Clone + Serialize + DeserializeOwned> OptimizationValue
    for Max<V>
{
    type Inner = V;

    fn meets_bound(value: &Self, bound: &V) -> bool {
        matches!(&value.0, Some(v) if *v >= *bound)
    }
}

/// Additive fold value.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Sum<W>(pub W);

impl<W: fmt::Debug + NumericSize + Serialize + DeserializeOwned> Aggregate for Sum<W> {
    fn identity() -> Self {
        Sum(W::zero())
    }

    fn combine(self, other: Self) -> Result<Self, AggregationError> {
        self.0
            .checked_add_value(other.0)
            .map(Sum)
            .map_err(|_| AggregationError::ArithmeticOverflow)
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

    fn combine(self, other: Self) -> Result<Self, AggregationError> {
        Ok(Or(self.0 || other.0))
    }

    fn is_absorbing(&self) -> bool {
        self.0
    }
}

impl SolutionAggregate for Or {
    fn contributes_to_solution(value: &Self, total: &Self) -> bool {
        value.0 && total.0
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

    fn combine(self, other: Self) -> Result<Self, AggregationError> {
        Ok(And(self.0 && other.0))
    }

    fn is_absorbing(&self) -> bool {
        !self.0
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

    fn combine(self, other: Self) -> Result<Self, AggregationError> {
        use std::cmp::Ordering;

        Ok(match (self.value, other.value) {
            (None, rhs) => Self {
                sense: other.sense,
                value: rhs,
            },
            (lhs, None) => Self {
                sense: self.sense,
                value: lhs,
            },
            (Some(lhs), Some(rhs)) => {
                if self.sense != other.sense {
                    return Err(AggregationError::IncompatibleExtremumSense);
                }
                let ord = lhs
                    .partial_cmp(&rhs)
                    .ok_or(AggregationError::UnorderedComparison)?;
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
        })
    }
}

impl<V: fmt::Debug + PartialOrd + Clone + Serialize + DeserializeOwned> SolutionAggregate
    for Extremum<V>
{
    fn contributes_to_solution(candidate: &Self, total: &Self) -> bool {
        matches!(
            (candidate.value.as_ref(), total.value.as_ref()),
            (Some(value), Some(best)) if candidate.sense == total.sense && value == best
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

/// Canonical named parameters for one concrete problem instance.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProblemParameters {
    /// Named parameters in canonical declaration order.
    #[serde(deserialize_with = "deserialize_parameter_components")]
    pub(crate) components: Vec<(String, u64)>,
}

impl ProblemParameters {
    /// Create problem parameters in canonical declaration order.
    ///
    /// # Panics
    /// Panics if a parameter name occurs more than once.
    pub fn new(components: Vec<(&str, u64)>) -> Self {
        Self::from_owned(
            components
                .into_iter()
                .map(|(name, value)| (name.to_string(), value))
                .collect(),
        )
    }

    /// Create problem parameters from owned names.
    ///
    /// # Panics
    /// Panics if a parameter name occurs more than once.
    pub fn from_owned(components: Vec<(String, u64)>) -> Self {
        if let Some(name) = duplicate_parameter_name(&components) {
            panic!("duplicate problem parameter `{name}`");
        }
        Self { components }
    }

    /// Iterate over parameters in canonical declaration order.
    pub fn iter(&self) -> impl Iterator<Item = (&str, u64)> {
        self.components
            .iter()
            .map(|(name, value)| (name.as_str(), *value))
    }

    /// Get a parameter by name.
    pub fn get(&self, name: &str) -> Option<u64> {
        self.components
            .iter()
            .find(|(k, _)| k == name)
            .map(|(_, v)| *v)
    }
}

fn duplicate_parameter_name(components: &[(String, u64)]) -> Option<&str> {
    components
        .iter()
        .enumerate()
        .find_map(|(index, (name, _))| {
            components[..index]
                .iter()
                .any(|(previous, _)| previous == name)
                .then_some(name.as_str())
        })
}

fn deserialize_parameter_components<'de, D>(deserializer: D) -> Result<Vec<(String, u64)>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let components = Vec::<(String, u64)>::deserialize(deserializer)?;
    if let Some(name) = duplicate_parameter_name(&components) {
        return Err(serde::de::Error::custom(format!(
            "duplicate problem parameter `{name}`"
        )));
    }
    Ok(components)
}

impl fmt::Display for ProblemParameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ProblemParameters{{")?;
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
impl_variant_param!(i64, "weight", parent: f64);
impl_variant_param!(One, "weight", parent: i64, cast: |_| 1i64);

#[cfg(test)]
#[path = "unit_tests/types.rs"]
mod tests;

#[cfg(test)]
#[path = "unit_tests/types_optimization_value.rs"]
mod optimization_value_tests;
