//! Closest Vector Problem (CVP) implementation.
//!
//! Given a lattice basis B and target vector t, find integer coefficients x
//! minimizing ‖Bx - t‖₂.

use crate::models::optimization::VarBounds;
use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "ClosestVectorProblem",
        module_path: module_path!(),
        description: "Find the closest lattice point to a target vector",
        fields: &[
            FieldInfo { name: "basis", type_name: "Vec<Vec<T>>", description: "Basis matrix B as column vectors" },
            FieldInfo { name: "target", type_name: "Vec<f64>", description: "Target vector t" },
            FieldInfo { name: "bounds", type_name: "Vec<VarBounds>", description: "Integer bounds per variable" },
        ],
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
    type Metric = SolutionSize<f64>;

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

    fn evaluate(&self, config: &[usize]) -> SolutionSize<f64> {
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
        SolutionSize::Valid(norm)
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![T]
    }
}

impl<T> OptimizationProblem for ClosestVectorProblem<T>
where
    T: Clone
        + Into<f64>
        + crate::variant::VariantParam
        + Serialize
        + for<'de> Deserialize<'de>
        + std::fmt::Debug
        + 'static,
{
    type Value = f64;

    fn direction(&self) -> Direction {
        Direction::Minimize
    }
}

crate::declare_variants! {
    ClosestVectorProblem<i32> => "2^num_basis_vectors",
    ClosestVectorProblem<f64> => "2^num_basis_vectors",
}

#[cfg(test)]
#[path = "../../unit_tests/models/algebraic/closest_vector_problem.rs"]
mod tests;
