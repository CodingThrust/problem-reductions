//! Algebraic Equations over GF(2) problem implementation.
//!
//! Given m multilinear polynomials over GF(2) in n variables, determine whether
//! there exists an assignment of the variables making all polynomials evaluate
//! to 0 (mod 2).

use crate::registry::{FieldInfo, ProblemSchemaEntry, ProblemSizeFieldEntry};
use crate::traits::Problem;
use crate::types::Or;
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "AlgebraicEquationsOverGF2",
        display_name: "Algebraic Equations over GF(2)",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Find assignment satisfying multilinear polynomial equations over GF(2)",
        fields: &[
            FieldInfo { name: "num_variables", type_name: "usize", description: "Number of Boolean variables" },
            FieldInfo { name: "equations", type_name: "Vec<Vec<Vec<usize>>>", description: "Equations: list of polynomials, each a list of monomials, each a sorted list of variable indices" },
        ],
    }
}

inventory::submit! {
    ProblemSizeFieldEntry {
        name: "AlgebraicEquationsOverGF2",
        fields: &["num_variables", "num_equations"],
    }
}

/// Algebraic Equations over GF(2).
///
/// Given m multilinear polynomials over GF(2) in n variables, determine whether
/// there exists an assignment of the variables making all polynomials evaluate
/// to 0 (mod 2).
///
/// Each equation is a list of monomials. Each monomial is a sorted list of
/// variable indices (0-indexed). An empty monomial represents the constant 1.
/// A polynomial evaluates to 0 when the XOR (sum mod 2) of all its monomial
/// values equals 0.
///
/// # Example
///
/// ```
/// use problemreductions::models::algebraic::AlgebraicEquationsOverGF2;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Two equations in 3 variables:
/// //   x0*x1 + x2 = 0 (mod 2)
/// //   x0 + 1 = 0 (mod 2)
/// let problem = AlgebraicEquationsOverGF2::new(
///     3,
///     vec![
///         vec![vec![0, 1], vec![2]],   // x0*x1 XOR x2
///         vec![vec![0], vec![]],        // x0 XOR 1
///     ],
/// ).unwrap();
///
/// let solver = BruteForce::new();
/// let witness = solver.find_witness(&problem);
/// assert!(witness.is_some());
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct AlgebraicEquationsOverGF2 {
    /// Number of variables.
    num_variables: usize,
    /// Equations: each equation is a list of monomials;
    /// each monomial is a sorted list of variable indices.
    equations: Vec<Vec<Vec<usize>>>,
}

impl AlgebraicEquationsOverGF2 {
    fn validate(num_variables: usize, equations: &[Vec<Vec<usize>>]) -> Result<(), String> {
        for (eq_idx, equation) in equations.iter().enumerate() {
            for (mono_idx, monomial) in equation.iter().enumerate() {
                // Check variable indices are in range
                for &var in monomial {
                    if var >= num_variables {
                        return Err(format!(
                            "Variable index {var} in equation {eq_idx}, monomial {mono_idx} \
                             is out of range (num_variables = {num_variables})"
                        ));
                    }
                }
                // Check monomial is sorted and has no duplicates
                for w in monomial.windows(2) {
                    if w[0] >= w[1] {
                        return Err(format!(
                            "Monomial {mono_idx} in equation {eq_idx} is not strictly sorted: \
                             found {} >= {}",
                            w[0], w[1]
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    /// Create a new `AlgebraicEquationsOverGF2` instance.
    ///
    /// Returns an error if any variable index is out of range or any monomial
    /// is not strictly sorted.
    pub fn new(num_variables: usize, equations: Vec<Vec<Vec<usize>>>) -> Result<Self, String> {
        Self::validate(num_variables, &equations)?;
        Ok(Self {
            num_variables,
            equations,
        })
    }

    /// Get the number of variables.
    pub fn num_variables(&self) -> usize {
        self.num_variables
    }

    /// Get the number of equations.
    pub fn num_equations(&self) -> usize {
        self.equations.len()
    }

    /// Get the equations.
    pub fn equations(&self) -> &[Vec<Vec<usize>>] {
        &self.equations
    }

    /// Evaluate a single monomial given a binary assignment.
    ///
    /// An empty monomial is the constant 1.
    /// A non-empty monomial is the product (AND) of the indicated variables.
    fn evaluate_monomial(monomial: &[usize], assignment: &[usize]) -> usize {
        if monomial.is_empty() {
            return 1;
        }
        for &var in monomial {
            if assignment[var] == 0 {
                return 0;
            }
        }
        1
    }

    /// Evaluate a single equation (polynomial) given a binary assignment.
    ///
    /// Returns true if the polynomial evaluates to 0 (mod 2).
    fn evaluate_equation(equation: &[Vec<usize>], assignment: &[usize]) -> bool {
        let sum: usize = equation
            .iter()
            .map(|mono| Self::evaluate_monomial(mono, assignment))
            .sum();
        sum.is_multiple_of(2)
    }
}

#[derive(Deserialize)]
struct AlgebraicEquationsOverGF2Data {
    num_variables: usize,
    equations: Vec<Vec<Vec<usize>>>,
}

impl<'de> Deserialize<'de> for AlgebraicEquationsOverGF2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = AlgebraicEquationsOverGF2Data::deserialize(deserializer)?;
        Self::new(data.num_variables, data.equations).map_err(D::Error::custom)
    }
}

impl Problem for AlgebraicEquationsOverGF2 {
    const NAME: &'static str = "AlgebraicEquationsOverGF2";
    type Value = Or;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.num_variables]
    }

    fn evaluate(&self, config: &[usize]) -> Or {
        Or(self
            .equations
            .iter()
            .all(|eq| Self::evaluate_equation(eq, config)))
    }
}

crate::declare_variants! {
    default AlgebraicEquationsOverGF2 => "2^(0.6943 * num_variables)",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "algebraic_equations_over_gf2",
        instance: Box::new(
            AlgebraicEquationsOverGF2::new(
                3,
                vec![
                    // x0*x1 + x2 = 0
                    vec![vec![0, 1], vec![2]],
                    // x1*x2 + x0 + 1 = 0
                    vec![vec![1, 2], vec![0], vec![]],
                    // x0 + x1 + x2 + 1 = 0
                    vec![vec![0], vec![1], vec![2], vec![]],
                ],
            )
            .unwrap(),
        ),
        // config [1,0,0]: eq1: 0*0+0=0 ✓, eq2: 0*0+1+1=0 ✓, eq3: 1+0+0+1=0 ✓
        optimal_config: vec![1, 0, 0],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/algebraic/algebraic_equations_over_gf2.rs"]
mod tests;
