//! Closest Vector Problem (CVP) implementation.
//!
//! Given a lattice basis B and target vector t, find integer coefficients x
//! minimizing ‖Bx - t‖₂.

use crate::registry::{ConstructionError, CreateSpec, ProblemSchemaEntry, VariantDimension};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

/// Coordinate types supported by [`ClosestVectorProblem`].
pub trait ClosestVectorCoordinate: Clone {
    fn to_exact_f64(&self) -> Result<f64, crate::traits::EvaluationError>;
    fn validate(&self, context: &str) -> Result<(), ConstructionError>;
}

impl ClosestVectorCoordinate for i64 {
    fn to_exact_f64(&self) -> Result<f64, crate::traits::EvaluationError> {
        crate::types::i64_to_exact_f64(*self).map_err(|error| {
            crate::traits::EvaluationError::InexactFloatConversion(error.to_string())
        })
    }
    fn validate(&self, _context: &str) -> Result<(), ConstructionError> {
        Ok(())
    }
}

impl ClosestVectorCoordinate for f64 {
    fn to_exact_f64(&self) -> Result<f64, crate::traits::EvaluationError> {
        self.is_finite().then_some(*self).ok_or_else(|| {
            crate::traits::EvaluationError::NonFiniteResult(
                "reading a closest-vector basis coordinate".to_string(),
            )
        })
    }
    fn validate(&self, context: &str) -> Result<(), ConstructionError> {
        if self.is_finite() {
            Ok(())
        } else {
            Err(ConstructionError::NonFiniteFloat(format!(
                "{context} must be finite"
            )))
        }
    }
}

inventory::submit! {
    ProblemSchemaEntry {
        name: "ClosestVectorProblem",
        display_name: "Closest Vector Problem",
        aliases: &["CVP"],
        dimensions: &[VariantDimension::new("weight", "i64", &["i64", "f64"])],
        category: crate::registry::ProblemCategory::Algebraic,
        module_path: module_path!(),
        description: "Find the closest lattice point to a target vector",
        fields: ClosestVectorProblemI64CreateSpec::FIELDS,
    }
}

/// Variable bounds (None = unbounded in that direction).
///
/// Represents the lower and upper bounds for an integer variable.
/// A value of `None` indicates the variable is unbounded in that direction.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct VarBounds {
    /// Lower bound (None = -infinity).
    pub lower: Option<i64>,
    /// Upper bound (None = +infinity).
    pub upper: Option<i64>,
}

impl VarBounds {
    /// Create bounds for a binary variable: 0 <= x <= 1.
    pub fn binary() -> Self {
        Self {
            lower: Some(0),
            upper: Some(1),
        }
    }

    /// Create bounds for a non-negative variable: x >= 0.
    pub fn non_negative() -> Self {
        Self {
            lower: Some(0),
            upper: None,
        }
    }

    /// Create unbounded variable: -infinity < x < +infinity.
    pub fn unbounded() -> Self {
        Self {
            lower: None,
            upper: None,
        }
    }

    /// Create bounds with explicit lower and upper: lo <= x <= hi.
    pub fn bounded(lo: i64, hi: i64) -> Self {
        Self {
            lower: Some(lo),
            upper: Some(hi),
        }
    }

    /// Check if a value satisfies these bounds.
    pub fn contains(&self, value: i64) -> bool {
        if let Some(lo) = self.lower {
            if value < lo {
                return false;
            }
        }
        if let Some(hi) = self.upper {
            if value > hi {
                return false;
            }
        }
        true
    }

    /// Get the number of integer values in this bound range.
    /// Returns None if unbounded in either direction.
    pub fn num_values(&self) -> Option<usize> {
        match (self.lower, self.upper) {
            (Some(lo), Some(hi)) => {
                if hi >= lo {
                    let count = i128::from(hi) - i128::from(lo) + 1;
                    usize::try_from(count).ok()
                } else {
                    Some(0)
                }
            }
            _ => None,
        }
    }

    fn validate_enumerable(&self, index: usize) -> Result<(), ConstructionError> {
        let (Some(lower), Some(upper)) = (self.lower, self.upper) else {
            return Err(ConstructionError::Conversion(format!(
                "bounds at index {index} must be finite"
            )));
        };
        if upper < lower {
            return Err(ConstructionError::Conversion(format!(
                "upper bound at index {index} must not be less than its lower bound"
            )));
        }
        if self.num_values().is_none() {
            return Err(ConstructionError::IntegerOverflow(format!(
                "integer range at index {index} is too large to enumerate"
            )));
        }
        Ok(())
    }

    /// Returns an exact bounded binary basis for offsets in this range.
    ///
    /// For a bounded variable with offsets `0..=hi-lo`, the returned weights
    /// ensure that every bit-pattern reconstructs an in-range offset. Low-order
    /// weights use powers of two; the final weight is capped so the maximum
    /// reachable offset is exactly `hi-lo`.
    pub(crate) fn exact_encoding_weights(&self) -> Result<Vec<i64>, ConstructionError> {
        let Some(num_values) = self.num_values() else {
            return Err(ConstructionError::IntegerOverflow(
                "CVP QUBO encoding requires finite variable bounds".to_string(),
            ));
        };
        if num_values <= 1 {
            return Ok(Vec::new());
        }

        let max_offset = i64::try_from(num_values - 1).map_err(|_| {
            ConstructionError::IntegerOverflow(
                "CVP QUBO encoding offset cannot be represented as i64".to_string(),
            )
        })?;
        let num_bits = (usize::BITS - (num_values - 1).leading_zeros()) as usize;
        let mut weights = Vec::with_capacity(num_bits);

        for bit in 0..num_bits.saturating_sub(1) {
            weights.push(1_i64 << bit);
        }

        let covered_by_lower_bits = if num_bits <= 1 {
            0
        } else {
            (1_i64 << (num_bits - 1)) - 1
        };
        weights.push(max_offset - covered_by_lower_bits);
        Ok(weights)
    }

    /// Returns the number of encoding bits needed for the exact bounded basis.
    pub(crate) fn num_encoding_bits(&self) -> usize {
        self.num_values()
            .filter(|&num_values| num_values > 1)
            .map(|num_values| (usize::BITS - (num_values - 1).leading_zeros()) as usize)
            .unwrap_or(0)
    }
}

/// Closest Vector Problem (CVP).
///
/// Given a lattice basis B ∈ R^{m×n} and target t ∈ R^m,
/// find integer x ∈ Z^n minimizing ‖Bx - t‖₂.
///
/// Variables are integer coefficients with explicit bounds for enumeration.
/// The configuration encoding follows ILP: `config[i]` is an offset from `bounds[i].lower`.
#[derive(Debug, Clone, Serialize)]
pub struct ClosestVectorProblem<T> {
    /// Basis matrix B stored as n column vectors, each of dimension m.
    basis: Vec<Vec<T>>,
    /// Target vector t ∈ R^m.
    target: Vec<f64>,
    /// Integer bounds per variable for enumeration.
    bounds: Vec<VarBounds>,
}

macro_rules! cvp_create_spec {
    ($name:ident, $element:ty) => {
        #[derive(Debug, Deserialize, crate::CreateSpec)]
        struct $name {
            /// Basis matrix as semicolon-separated column vectors.
            #[create(codec = "semicolon-separated")]
            basis: Vec<Vec<$element>>,
            /// Target vector.
            #[create(name = "target_vec", codec = "comma-separated")]
            target: Vec<f64>,
            /// Shared lower and upper coefficient bounds.
            #[create(codec = "comma-separated")]
            bounds: Option<Vec<i64>>,
        }

        impl TryFrom<$name> for ClosestVectorProblem<$element> {
            type Error = ConstructionError;

            fn try_from(spec: $name) -> Result<Self, Self::Error> {
                let limits = spec.bounds.unwrap_or_else(|| vec![-10, 10]);
                if limits.len() != 2 {
                    return Err(ConstructionError::Conversion(
                        "bounds expects exactly lower,upper".to_string(),
                    ));
                }
                let bounds = vec![VarBounds::bounded(limits[0], limits[1]); spec.basis.len()];
                ClosestVectorProblem::new(spec.basis, spec.target, bounds)
            }
        }
    };
}

cvp_create_spec!(ClosestVectorProblemI64CreateSpec, i64);
cvp_create_spec!(ClosestVectorProblemF64CreateSpec, f64);

impl<T: ClosestVectorCoordinate> ClosestVectorProblem<T> {
    /// Create a new CVP instance.
    ///
    /// # Arguments
    /// * `basis` - n column vectors of dimension m
    /// * `target` - target vector of dimension m
    /// * `bounds` - integer bounds per variable (length n)
    ///
    pub fn new(
        basis: Vec<Vec<T>>,
        target: Vec<f64>,
        bounds: Vec<VarBounds>,
    ) -> Result<Self, ConstructionError> {
        let n = basis.len();
        if bounds.len() != n {
            return Err(ConstructionError::Conversion(
                "bounds length must match number of basis vectors".to_string(),
            ));
        }
        let m = target.len();
        for (row, value) in target.iter().enumerate() {
            if !value.is_finite() {
                return Err(ConstructionError::NonFiniteFloat(format!(
                    "target coordinate at index {row} must be finite"
                )));
            }
        }
        for (i, col) in basis.iter().enumerate() {
            if col.len() != m {
                return Err(ConstructionError::Conversion(format!(
                    "basis vector {i} has length {}, expected {m}",
                    col.len()
                )));
            }
            for (row, coordinate) in col.iter().enumerate() {
                coordinate.validate(&format!("basis coordinate at column {i}, row {row}"))?;
            }
        }
        let mut total_encoding_bits = 0usize;
        for (index, bound) in bounds.iter().enumerate() {
            bound.validate_enumerable(index)?;
            total_encoding_bits = total_encoding_bits
                .checked_add(bound.num_encoding_bits())
                .ok_or_else(|| {
                    ConstructionError::IntegerOverflow(
                        "computing the total number of encoding bits".to_string(),
                    )
                })?;
        }
        Ok(Self {
            basis,
            target,
            bounds,
        })
    }

    /// Number of basis vectors (lattice dimension n).
    pub fn num_basis_vectors(&self) -> usize {
        self.basis.len()
    }

    /// Dimension of the ambient space (m).
    pub fn ambient_dimension(&self) -> usize {
        self.target.len()
    }

    /// Access the basis matrix.
    pub fn basis(&self) -> &[Vec<T>] {
        &self.basis
    }

    /// Access the target vector.
    pub fn target(&self) -> &[f64] {
        &self.target
    }

    /// Access the variable bounds.
    pub fn bounds(&self) -> &[VarBounds] {
        &self.bounds
    }

    /// Returns the total number of bounded-encoding bits used by the QUBO form.
    pub fn num_encoding_bits(&self) -> usize {
        self.bounds.iter().map(VarBounds::num_encoding_bits).sum()
    }

    /// Convert a configuration (offsets from lower bounds) to integer values.
    fn config_to_values(
        &self,
        config: &[usize],
    ) -> Result<Vec<i64>, crate::traits::EvaluationError> {
        if config.len() != self.bounds.len() {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                format!(
                    "expected {} closest-vector coefficients, got {}",
                    self.bounds.len(),
                    config.len()
                ),
            ));
        }
        config
            .iter()
            .enumerate()
            .map(|(i, &c)| {
                let bound = &self.bounds[i];
                let dimension = bound.num_values().expect("validated finite CVP bounds");
                if c >= dimension {
                    return Err(crate::traits::EvaluationError::InvalidConfiguration(
                        format!("coefficient at index {i} is outside its encoded range"),
                    ));
                }
                let offset = i64::try_from(c).map_err(|_| {
                    crate::traits::EvaluationError::IntegerOverflow(
                        "converting a closest-vector configuration offset to i64".into(),
                    )
                })?;
                bound
                    .lower
                    .expect("validated finite CVP lower bound")
                    .checked_add(offset)
                    .ok_or_else(|| {
                        crate::traits::EvaluationError::IntegerOverflow(
                            "adding a closest-vector configuration offset".into(),
                        )
                    })
            })
            .collect()
    }
}

impl<'de, T> Deserialize<'de> for ClosestVectorProblem<T>
where
    T: ClosestVectorCoordinate + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Raw<T> {
            basis: Vec<Vec<T>>,
            target: Vec<f64>,
            bounds: Vec<VarBounds>,
        }

        let raw = Raw::deserialize(deserializer)?;
        Self::new(raw.basis, raw.target, raw.bounds).map_err(serde::de::Error::custom)
    }
}

impl<T> Problem for ClosestVectorProblem<T>
where
    T: ClosestVectorCoordinate
        + crate::variant::VariantParam
        + Serialize
        + for<'de> Deserialize<'de>
        + std::fmt::Debug
        + 'static,
{
    const NAME: &'static str = "ClosestVectorProblem";
    type Value = Min<f64>;

    fn dims(&self) -> Vec<usize> {
        self.bounds
            .iter()
            .map(|b| {
                b.num_values().expect(
                    "CVP brute-force enumeration requires all variables to have finite bounds",
                )
            })
            .collect()
    }

    fn evaluate(&self, config: &[usize]) -> Result<Min<f64>, crate::traits::EvaluationError> {
        Ok({
            let values = self.config_to_values(config)?;
            let m = self.ambient_dimension();
            let mut diff = vec![0.0f64; m];
            for (i, &x_i) in values.iter().enumerate() {
                let x_i = crate::types::i64_to_exact_f64(x_i).map_err(|error| {
                    crate::traits::EvaluationError::InexactFloatConversion(error.to_string())
                })?;
                for (j, b_ji) in self.basis[i].iter().enumerate() {
                    let term = x_i * b_ji.to_exact_f64()?;
                    let next = diff[j] + term;
                    if !term.is_finite() || !next.is_finite() {
                        return Err(crate::traits::EvaluationError::NonFiniteResult(
                            "computing closest-vector lattice point".to_string(),
                        ));
                    }
                    diff[j] = next;
                }
            }
            for (d, t) in diff.iter_mut().zip(self.target.iter()) {
                let next = *d - t;
                if !next.is_finite() {
                    return Err(crate::traits::EvaluationError::NonFiniteResult(
                        "computing closest-vector displacement".to_string(),
                    ));
                }
                *d = next;
            }
            let mut squared_norm = 0.0;
            for displacement in diff {
                let square = displacement * displacement;
                let next = squared_norm + square;
                if !square.is_finite() || !next.is_finite() {
                    return Err(crate::traits::EvaluationError::NonFiniteResult(
                        "computing closest-vector norm".to_string(),
                    ));
                }
                squared_norm = next;
            }
            let norm = squared_norm.sqrt();
            Min(Some(norm))
        })
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![T]
    }
}

crate::declare_variants! {
    default ClosestVectorProblem<i64> => "2^num_basis_vectors" create ClosestVectorProblemI64CreateSpec,
    ClosestVectorProblem<f64> => "2^num_basis_vectors" create ClosestVectorProblemF64CreateSpec,
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "closest_vector_problem_i64",
        instance: Box::new(
            ClosestVectorProblem::new(
                vec![vec![2, 0], vec![1, 2]],
                vec![2.8, 1.5],
                vec![VarBounds::bounded(-2, 4), VarBounds::bounded(-2, 4)],
            )
            .expect("canonical closest-vector instance must be valid"),
        ),
        optimal_config: vec![3, 3],
        optimal_value: serde_json::json!(0.5385164807134505),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/algebraic/closest_vector_problem.rs"]
mod tests;
