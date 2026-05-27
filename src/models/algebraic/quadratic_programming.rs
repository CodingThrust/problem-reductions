//! Bounded Integer Quadratic Programming (QP) problem implementation.
//!
//! Minimizes a quadratic objective over bounded integer variables subject to
//! linear inequality constraints. The variable domain is `{-K, ..., K}` for a
//! fixed bound `K >= 1`, so `dims()` returns a finite product of discrete sets.
//!
//! The continuous / rational case of QP (Garey & Johnson MP2) is out of scope
//! for this library because `Problem::dims()` requires finite discrete domains.
//! NP-hardness of the bounded integer variant follows from Sahni (1974)'s
//! PARTITION → QP reduction, which lands in `{0, 1}^m` and therefore remains
//! valid for any `K >= 1`.

use crate::models::algebraic::ilp::LinearConstraint;
use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "QuadraticProgramming",
        display_name: "Quadratic Programming",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Minimize a quadratic objective over bounded integer variables subject to linear inequalities",
        fields: &[
            FieldInfo { name: "num_vars", type_name: "usize", description: "Number of integer variables m" },
            FieldInfo { name: "bound", type_name: "usize", description: "Per-variable domain bound K; each y_i in {-K, ..., K}" },
            FieldInfo { name: "constraints", type_name: "Vec<LinearConstraint>", description: "Linear inequality constraints (each (x, b) means x . y <= b)" },
            FieldInfo { name: "quad_coeffs", type_name: "Vec<f64>", description: "Quadratic coefficients c_i for the objective" },
            FieldInfo { name: "lin_coeffs", type_name: "Vec<f64>", description: "Linear coefficients d_i for the objective" },
        ],
    }
}

/// Bounded Integer Quadratic Programming.
///
/// Minimize `sum_i (c_i * y_i^2 + d_i * y_i)` subject to `x . y <= b` for every
/// constraint `(x, b)`, with `y_i in {-K, ..., K}` for the fixed per-variable
/// bound `K = bound`.
///
/// # Example
///
/// ```
/// use problemreductions::models::algebraic::{LinearConstraint, QuadraticProgramming};
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // PARTITION over a = (1, 1, 2) with target sum 2. The optimum lands at
/// // y = (1, 1, 0) with objective 0.
/// let qp = QuadraticProgramming::new(
///     3,
///     1,
///     vec![
///         LinearConstraint::le(vec![(0, -1.0)], 0.0),
///         LinearConstraint::le(vec![(1, -1.0)], 0.0),
///         LinearConstraint::le(vec![(2, -1.0)], 0.0),
///         LinearConstraint::le(vec![(0, 1.0), (1, 1.0), (2, 2.0)], 2.0),
///         LinearConstraint::le(vec![(0, -1.0), (1, -1.0), (2, -2.0)], -2.0),
///     ],
///     vec![-1.0, -1.0, -1.0],
///     vec![1.0, 1.0, 2.0],
/// );
///
/// let solver = BruteForce::new();
/// let best = solver.find_witness(&qp).unwrap();
/// // Optimal config maps to y = (1, 1, 0): config = (1+K, 1+K, 0+K) = (2, 2, 1)
/// assert_eq!(Problem::evaluate(&qp, &best).0, Some(0.0));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuadraticProgramming {
    /// Number of integer variables `m`.
    pub num_vars: usize,
    /// Per-variable domain bound `K`; each `y_i in {-K, ..., K}`.
    pub bound: usize,
    /// Linear inequality constraints `x . y <= b`.
    pub constraints: Vec<LinearConstraint>,
    /// Quadratic coefficients `c_i` (one per variable).
    pub quad_coeffs: Vec<f64>,
    /// Linear coefficients `d_i` (one per variable).
    pub lin_coeffs: Vec<f64>,
}

impl QuadraticProgramming {
    /// Create a new bounded integer QP instance.
    ///
    /// # Panics
    /// Panics if `bound == 0`, or if `quad_coeffs.len() != num_vars` or
    /// `lin_coeffs.len() != num_vars`.
    pub fn new(
        num_vars: usize,
        bound: usize,
        constraints: Vec<LinearConstraint>,
        quad_coeffs: Vec<f64>,
        lin_coeffs: Vec<f64>,
    ) -> Self {
        assert!(bound >= 1, "QuadraticProgramming bound K must be >= 1");
        assert_eq!(
            quad_coeffs.len(),
            num_vars,
            "quad_coeffs length must equal num_vars"
        );
        assert_eq!(
            lin_coeffs.len(),
            num_vars,
            "lin_coeffs length must equal num_vars"
        );
        Self {
            num_vars,
            bound,
            constraints,
            quad_coeffs,
            lin_coeffs,
        }
    }

    /// Number of variables `m`.
    pub fn num_vars(&self) -> usize {
        self.num_vars
    }

    /// Per-variable bound `K`.
    pub fn bound(&self) -> usize {
        self.bound
    }

    /// Number of linear inequality constraints.
    pub fn num_constraints(&self) -> usize {
        self.constraints.len()
    }

    /// Decode a configuration `c[i] in {0, ..., 2K}` to integer values
    /// `y_i = c[i] - K`.
    pub fn config_to_values(&self, config: &[usize]) -> Vec<i64> {
        let k = self.bound as i64;
        config.iter().map(|&c| c as i64 - k).collect()
    }

    /// Check whether all linear constraints are satisfied for the given
    /// integer values.
    pub fn constraints_satisfied(&self, values: &[i64]) -> bool {
        self.constraints.iter().all(|c| c.is_satisfied(values))
    }

    /// Evaluate the quadratic objective `sum_i (c_i * y_i^2 + d_i * y_i)`
    /// for the given integer values.
    pub fn objective_value(&self, values: &[i64]) -> f64 {
        values
            .iter()
            .enumerate()
            .map(|(i, &y)| {
                let y = y as f64;
                self.quad_coeffs[i] * y * y + self.lin_coeffs[i] * y
            })
            .sum()
    }
}

impl Problem for QuadraticProgramming {
    const NAME: &'static str = "QuadraticProgramming";
    type Value = Min<f64>;

    fn dims(&self) -> Vec<usize> {
        vec![2 * self.bound + 1; self.num_vars]
    }

    fn evaluate(&self, config: &[usize]) -> Min<f64> {
        if config.len() != self.num_vars {
            return Min(None);
        }
        let values = self.config_to_values(config);
        if !self.constraints_satisfied(&values) {
            return Min(None);
        }
        Min(Some(self.objective_value(&values)))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

crate::declare_variants! {
    default QuadraticProgramming => "(2 * bound + 1) ^ num_vars",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "quadratic_programming",
        instance: Box::new(QuadraticProgramming::new(
            3,
            1,
            vec![
                LinearConstraint::le(vec![(0, -1.0)], 0.0),
                LinearConstraint::le(vec![(1, -1.0)], 0.0),
                LinearConstraint::le(vec![(2, -1.0)], 0.0),
                LinearConstraint::le(vec![(0, 1.0), (1, 1.0), (2, 2.0)], 2.0),
                LinearConstraint::le(vec![(0, -1.0), (1, -1.0), (2, -2.0)], -2.0),
            ],
            vec![-1.0, -1.0, -1.0],
            vec![1.0, 1.0, 2.0],
        )),
        // y = (1, 1, 0) encoded with bound K=1 means config = y + K = (2, 2, 1).
        optimal_config: vec![2, 2, 1],
        optimal_value: serde_json::json!(0.0),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/algebraic/quadratic_programming.rs"]
mod tests;
