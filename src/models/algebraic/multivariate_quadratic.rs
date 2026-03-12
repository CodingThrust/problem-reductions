//! Multivariate Quadratic (MQ) problem implementation.
//!
//! Satisfy a system of multivariate quadratic equations over a finite field F_q.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{Problem, SatisfactionProblem};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MultivariateQuadratic",
        module_path: module_path!(),
        description: "Satisfy a system of multivariate quadratic equations over a finite field",
        fields: &[
            FieldInfo { name: "field_size", type_name: "usize", description: "Size of finite field q" },
            FieldInfo { name: "num_variables", type_name: "usize", description: "Number of variables n" },
            FieldInfo { name: "equations", type_name: "Vec<QuadraticPoly>", description: "System of quadratic polynomials" },
        ],
    }
}

/// A single quadratic polynomial over F_q.
///
/// Represents a polynomial of the form:
///   Σ_{j≤k} a_{jk} x_j x_k + Σ_j b_j x_j + c
///
/// where all arithmetic is performed modulo the field size q.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuadraticPoly {
    /// Coefficients for quadratic terms x_j * x_k (with j ≤ k).
    pub quadratic_terms: Vec<((usize, usize), u64)>,
    /// Coefficients for linear terms x_j.
    pub linear_terms: Vec<(usize, u64)>,
    /// Constant term.
    pub constant: u64,
}

impl QuadraticPoly {
    /// Evaluate the polynomial at the given configuration modulo `field_size`.
    pub fn evaluate(&self, config: &[usize], field_size: usize) -> u64 {
        let q = field_size as u64;
        let mut result = self.constant % q;
        for &((j, k), coeff) in &self.quadratic_terms {
            result =
                (result + coeff % q * (config[j] as u64 % q) % q * (config[k] as u64 % q) % q) % q;
        }
        for &(j, coeff) in &self.linear_terms {
            result = (result + coeff % q * (config[j] as u64 % q) % q) % q;
        }
        result
    }
}

/// The Multivariate Quadratic (MQ) problem.
///
/// Given a finite field F_q, n variables x_1, ..., x_n ∈ F_q, and m quadratic
/// polynomials f_1, ..., f_m, find an assignment such that all f_i evaluate to 0.
///
/// This is a fundamental problem in post-quantum cryptography and is NP-hard
/// even over F_2.
///
/// # Example
///
/// ```
/// use problemreductions::models::algebraic::{MultivariateQuadratic, QuadraticPoly};
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Over F_2: f1 = x0*x1 + x2, f2 = x1*x2 + x0
/// let eq1 = QuadraticPoly {
///     quadratic_terms: vec![((0, 1), 1)],
///     linear_terms: vec![(2, 1)],
///     constant: 0,
/// };
/// let eq2 = QuadraticPoly {
///     quadratic_terms: vec![((1, 2), 1)],
///     linear_terms: vec![(0, 1)],
///     constant: 0,
/// };
/// let problem = MultivariateQuadratic::new(2, 3, vec![eq1, eq2]);
///
/// let solver = BruteForce::new();
/// let solution = solver.find_satisfying(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultivariateQuadratic {
    /// Size of the finite field (e.g., 2 for F_2).
    field_size: usize,
    /// Number of variables n.
    num_variables: usize,
    /// System of quadratic polynomials.
    equations: Vec<QuadraticPoly>,
}

impl MultivariateQuadratic {
    /// Create a new MQ problem instance.
    pub fn new(field_size: usize, num_variables: usize, equations: Vec<QuadraticPoly>) -> Self {
        Self {
            field_size,
            num_variables,
            equations,
        }
    }

    /// Returns the field size q.
    pub fn field_size(&self) -> usize {
        self.field_size
    }

    /// Returns the number of variables n.
    pub fn num_vars(&self) -> usize {
        self.num_variables
    }

    /// Returns the number of equations m.
    pub fn num_equations(&self) -> usize {
        self.equations.len()
    }

    /// Returns the equations.
    pub fn equations(&self) -> &[QuadraticPoly] {
        &self.equations
    }
}

impl Problem for MultivariateQuadratic {
    const NAME: &'static str = "MultivariateQuadratic";
    type Metric = bool;

    fn dims(&self) -> Vec<usize> {
        vec![self.field_size; self.num_variables]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        self.equations
            .iter()
            .all(|eq| eq.evaluate(config, self.field_size) == 0)
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

impl SatisfactionProblem for MultivariateQuadratic {}

crate::declare_variants! {
    MultivariateQuadratic => "field_size^num_vars",
}

#[cfg(test)]
#[path = "../../unit_tests/models/algebraic/multivariate_quadratic.rs"]
mod tests;
