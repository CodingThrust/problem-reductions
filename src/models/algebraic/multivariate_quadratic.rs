//! Multivariate Quadratic (MQ) problem implementation.
//!
//! Satisfy a system of multivariate quadratic equations over F_2.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{Problem, SatisfactionProblem};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MultivariateQuadratic",
        module_path: module_path!(),
        description: "Satisfy a system of multivariate quadratic equations over F_2",
        fields: &[
            FieldInfo { name: "num_variables", type_name: "usize", description: "Number of variables n" },
            FieldInfo { name: "equations", type_name: "Vec<QuadraticPoly>", description: "System of quadratic polynomials over F_2" },
        ],
    }
}

/// A single quadratic polynomial over F_2.
///
/// Represents a polynomial of the form:
///   Σ_{j≤k} x_j x_k (for present pairs) + Σ_j x_j (for present indices) + c
///
/// where all arithmetic is performed in GF(2) (XOR for addition, AND for multiplication).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuadraticPoly {
    /// Variable pairs (j, k) with j <= k whose quadratic term x_j * x_k is present (coefficient 1).
    pub quadratic_terms: Vec<(usize, usize)>,
    /// Variable indices j whose linear term x_j is present (coefficient 1).
    pub linear_terms: Vec<usize>,
    /// Constant term: true means 1, false means 0.
    pub constant: bool,
}

impl QuadraticPoly {
    /// Evaluate the polynomial at the given configuration in F_2.
    ///
    /// Uses XOR for addition and AND for multiplication.
    pub fn evaluate(&self, config: &[usize]) -> bool {
        let mut result = self.constant;
        for &(j, k) in &self.quadratic_terms {
            // x_j AND x_k, then XOR into result
            if config[j] == 1 && config[k] == 1 {
                result = !result;
            }
        }
        for &j in &self.linear_terms {
            if config[j] == 1 {
                result = !result;
            }
        }
        // result == false means the polynomial evaluates to 0 (satisfied)
        // result == true means the polynomial evaluates to 1 (not satisfied)
        // Return whether polynomial == 0
        !result
    }
}

/// The Multivariate Quadratic (MQ) problem over F_2.
///
/// Given n binary variables x_0, ..., x_{n-1} in F_2 and m quadratic
/// polynomials f_1, ..., f_m, find an assignment such that all f_i evaluate to 0.
///
/// This is a fundamental problem in post-quantum cryptography and is NP-hard.
///
/// # Example
///
/// ```
/// use problemreductions::models::algebraic::{MultivariateQuadratic, QuadraticPoly};
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Over F_2: f1 = x0*x1 + x2, f2 = x1*x2 + x0
/// let eq1 = QuadraticPoly {
///     quadratic_terms: vec![(0, 1)],
///     linear_terms: vec![2],
///     constant: false,
/// };
/// let eq2 = QuadraticPoly {
///     quadratic_terms: vec![(1, 2)],
///     linear_terms: vec![0],
///     constant: false,
/// };
/// let problem = MultivariateQuadratic::new(3, vec![eq1, eq2]);
///
/// let solver = BruteForce::new();
/// let solution = solver.find_satisfying(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultivariateQuadratic {
    /// Number of variables n.
    num_variables: usize,
    /// System of quadratic polynomials over F_2.
    equations: Vec<QuadraticPoly>,
}

impl MultivariateQuadratic {
    /// Create a new MQ problem instance over F_2.
    pub fn new(num_variables: usize, equations: Vec<QuadraticPoly>) -> Self {
        Self {
            num_variables,
            equations,
        }
    }

    /// Returns the number of variables n.
    pub fn num_variables(&self) -> usize {
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
        vec![2; self.num_variables]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        self.equations.iter().all(|eq| eq.evaluate(config))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

impl SatisfactionProblem for MultivariateQuadratic {}

crate::declare_variants! {
    MultivariateQuadratic => "1.6181^num_variables",
}

#[cfg(test)]
#[path = "../../unit_tests/models/algebraic/multivariate_quadratic.rs"]
mod tests;
