//! Quadratic Diophantine Equations problem implementation.
//!
//! Given positive integers a, b, c, determine whether there exist
//! positive integers x, y such that ax² + by = c.

use crate::registry::{FieldInfo, ProblemSchemaEntry, ProblemSizeFieldEntry};
use crate::traits::Problem;
use crate::types::Or;
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "QuadraticDiophantineEquations",
        display_name: "Quadratic Diophantine Equations",
        aliases: &["QDE"],
        dimensions: &[],
        module_path: module_path!(),
        description: "Decide whether ax² + by = c has a solution in positive integers x, y",
        fields: &[
            FieldInfo { name: "a", type_name: "u64", description: "Coefficient of x²" },
            FieldInfo { name: "b", type_name: "u64", description: "Coefficient of y" },
            FieldInfo { name: "c", type_name: "u64", description: "Right-hand side constant" },
        ],
    }
}

inventory::submit! {
    ProblemSizeFieldEntry {
        name: "QuadraticDiophantineEquations",
        fields: &["c"],
    }
}

/// Quadratic Diophantine Equations problem.
///
/// Given positive integers a, b, c, determine whether there exist
/// positive integers x, y such that ax² + by = c.
///
/// # Example
///
/// ```
/// use problemreductions::models::algebraic::QuadraticDiophantineEquations;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // a=3, b=5, c=53: x=1 gives y=10, x=4 gives y=1
/// let problem = QuadraticDiophantineEquations::new(3, 5, 53);
/// let solver = BruteForce::new();
/// let witness = solver.find_witness(&problem);
/// assert!(witness.is_some());
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct QuadraticDiophantineEquations {
    /// Coefficient of x².
    a: u64,
    /// Coefficient of y.
    b: u64,
    /// Right-hand side constant.
    c: u64,
}

impl QuadraticDiophantineEquations {
    fn validate_inputs(a: u64, b: u64, c: u64) -> Result<(), String> {
        if a == 0 {
            return Err("Coefficient a must be positive".to_string());
        }
        if b == 0 {
            return Err("Coefficient b must be positive".to_string());
        }
        if c == 0 {
            return Err("Right-hand side c must be positive".to_string());
        }
        Ok(())
    }

    /// Create a new QuadraticDiophantineEquations instance, returning an error
    /// instead of panicking when inputs are invalid.
    pub fn try_new(a: u64, b: u64, c: u64) -> Result<Self, String> {
        Self::validate_inputs(a, b, c)?;
        Ok(Self { a, b, c })
    }

    /// Create a new QuadraticDiophantineEquations instance.
    ///
    /// # Panics
    ///
    /// Panics if any of a, b, c is zero.
    pub fn new(a: u64, b: u64, c: u64) -> Self {
        Self::try_new(a, b, c).unwrap_or_else(|msg| panic!("{msg}"))
    }

    /// Get the coefficient a (coefficient of x²).
    pub fn a(&self) -> u64 {
        self.a
    }

    /// Get the coefficient b (coefficient of y).
    pub fn b(&self) -> u64 {
        self.b
    }

    /// Get the right-hand side constant c.
    pub fn c(&self) -> u64 {
        self.c
    }

    /// Compute the integer square root of n (floor(sqrt(n))).
    fn isqrt(n: u64) -> u64 {
        if n == 0 {
            return 0;
        }
        let mut x = (n as f64).sqrt() as u64;
        // Correct for floating-point imprecision.
        while x.saturating_mul(x) > n {
            x -= 1;
        }
        while (x + 1).saturating_mul(x + 1) <= n {
            x += 1;
        }
        x
    }

    /// Compute the maximum value of x (floor(sqrt(c/a))).
    /// Returns 0 if c < a (no positive x is possible since x >= 1 requires a*1 <= c).
    fn max_x(&self) -> u64 {
        if self.c < self.a {
            return 0;
        }
        Self::isqrt(self.c / self.a)
    }

    /// Check whether a given x yields a valid positive integer y.
    ///
    /// Returns Some(y) if y is a positive integer, None otherwise.
    pub fn check_x(&self, x: u64) -> Option<u64> {
        if x == 0 {
            return None;
        }
        let ax2 = self.a.checked_mul(x)?.checked_mul(x)?;
        if ax2 >= self.c {
            return None;
        }
        let remainder = self.c - ax2;
        if !remainder.is_multiple_of(self.b) {
            return None;
        }
        let y = remainder / self.b;
        if y == 0 {
            return None;
        }
        Some(y)
    }
}

#[derive(Deserialize)]
struct QuadraticDiophantineEquationsData {
    a: u64,
    b: u64,
    c: u64,
}

impl<'de> Deserialize<'de> for QuadraticDiophantineEquations {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = QuadraticDiophantineEquationsData::deserialize(deserializer)?;
        Self::try_new(data.a, data.b, data.c).map_err(D::Error::custom)
    }
}

impl Problem for QuadraticDiophantineEquations {
    const NAME: &'static str = "QuadraticDiophantineEquations";
    type Value = Or;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        let max_x = self.max_x() as usize;
        if max_x == 0 {
            // No valid x exists; return empty config space.
            return vec![0];
        }
        // One variable: x ranges over {1, ..., max_x}.
        // config[0] in {0, ..., max_x - 1} maps to x = config[0] + 1.
        vec![max_x]
    }

    fn evaluate(&self, config: &[usize]) -> Or {
        Or({
            if config.len() != 1 {
                return Or(false);
            }
            let x = (config[0] as u64) + 1; // 1-indexed
            self.check_x(x).is_some()
        })
    }
}

crate::declare_variants! {
    default QuadraticDiophantineEquations => "sqrt(c)",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "quadratic_diophantine_equations",
        instance: Box::new(QuadraticDiophantineEquations::new(3, 5, 53)),
        // x=1 (config[0]=0) gives y=10: 3*1 + 5*10 = 53
        optimal_config: vec![0],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/algebraic/quadratic_diophantine_equations.rs"]
mod tests;
