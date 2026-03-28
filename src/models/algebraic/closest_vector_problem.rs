//! Closest Vector Problem (CVP) implementation.
//!
//! Given a lattice basis B and target vector t, find integer coefficients x
//! minimizing ‖Bx - t‖₂.

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "ClosestVectorProblem",
        display_name: "Closest Vector Problem",
        aliases: &["CVP"],
        dimensions: &[VariantDimension::new("weight", "i32", &["i32", "f64"])],
        module_path: module_path!(),
        description: "Find the closest lattice point to a target vector",
        fields: &[
            FieldInfo { name: "basis", type_name: "Vec<Vec<T>>", description: "Basis matrix B as column vectors" },
            FieldInfo { name: "target", type_name: "Vec<f64>", description: "Target vector t" },
            FieldInfo { name: "bounds", type_name: "Vec<VarBounds>", description: "Integer bounds per variable" },
        ],
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
                    Some((hi - lo + 1) as usize)
                } else {
                    Some(0)
                }
            }
            _ => None,
        }
    }

    /// Returns an exact bounded binary basis for offsets in this range.
    ///
    /// For a bounded variable with offsets `0..=hi-lo`, the returned weights
    /// ensure that every bit-pattern reconstructs an in-range offset. Low-order
    /// weights use powers of two; the final weight is capped so the maximum
    /// reachable offset is exactly `hi-lo`.
    pub(crate) fn exact_encoding_weights(&self) -> Vec<i64> {
        let Some(num_values) = self.num_values() else {
            panic!("CVP QUBO encoding requires finite variable bounds");
        };
        if num_values <= 1 {
            return Vec::new();
        }

        let max_offset = (num_values - 1) as i64;
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
        weights
    }

    /// Returns the number of encoding bits needed for the exact bounded basis.
    pub(crate) fn num_encoding_bits(&self) -> usize {
        self.exact_encoding_weights().len()
    }
}

/// Closest Vector Problem (CVP).
///
/// Given a lattice basis B ∈ R^{m×n} and target t ∈ R^m,
/// find integer x ∈ Z^n minimizing ‖Bx - t‖₂.
///
/// Variables are integer coefficients with explicit bounds for enumeration.
/// The configuration encoding follows ILP: config[i] is an offset from bounds[i].lower.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClosestVectorProblem<T> {
    /// Basis matrix B stored as n column vectors, each of dimension m.
    basis: Vec<Vec<T>>,
    /// Target vector t ∈ R^m.
    target: Vec<f64>,
    /// Integer bounds per variable for enumeration.
    bounds: Vec<VarBounds>,
}

impl<T> ClosestVectorProblem<T> {
    /// Create a new CVP instance.
    ///
    /// # Arguments
    /// * `basis` - n column vectors of dimension m
    /// * `target` - target vector of dimension m
    /// * `bounds` - integer bounds per variable (length n)
    ///
    /// # Panics
    /// Panics if basis/bounds lengths mismatch or dimensions are inconsistent.
    pub fn new(basis: Vec<Vec<T>>, target: Vec<f64>, bounds: Vec<VarBounds>) -> Self {
        let n = basis.len();
        assert_eq!(
            bounds.len(),
            n,
            "bounds length must match number of basis vectors"
        );
        let m = target.len();
        for (i, col) in basis.iter().enumerate() {
            assert_eq!(
                col.len(),
                m,
                "basis vector {i} has length {}, expected {m}",
                col.len()
            );
        }
        Self {
            basis,
            target,
            bounds,
        }
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
    fn config_to_values(&self, config: &[usize]) -> Vec<i64> {
        config
            .iter()
            .enumerate()
            .map(|(i, &c)| {
                let lo = self.bounds.get(i).and_then(|b| b.lower).unwrap_or(0);
                lo + c as i64
            })
            .collect()
    }
}

impl<T> Problem for ClosestVectorProblem<T>
where
    T: Clone
        + Into<f64>
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

    fn evaluate(&self, config: &[usize]) -> Min<f64> {
        let values = self.config_to_values(config);
        let m = self.ambient_dimension();
        let mut diff = vec![0.0f64; m];
        for (i, &x_i) in values.iter().enumerate() {
            for (j, b_ji) in self.basis[i].iter().enumerate() {
                diff[j] += x_i as f64 * b_ji.clone().into();
            }
        }
        for (d, t) in diff.iter_mut().zip(self.target.iter()) {
            *d -= t;
        }
        let norm = diff.iter().map(|d| d * d).sum::<f64>().sqrt();
        Min(Some(norm))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![T]
    }
}

crate::declare_variants! {
    default ClosestVectorProblem<i32> => "2^num_basis_vectors",
    ClosestVectorProblem<f64> => "2^num_basis_vectors",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "closest_vector_problem_i32",
        instance: Box::new(ClosestVectorProblem::new(
            vec![vec![2, 0], vec![1, 2]],
            vec![2.8, 1.5],
            vec![VarBounds::bounded(-2, 4), VarBounds::bounded(-2, 4)],
        )),
        optimal_config: vec![3, 3],
        optimal_value: serde_json::json!(0.5385164807134505),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/algebraic/closest_vector_problem.rs"]
mod tests;
