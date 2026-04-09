//! Quadratic Diophantine Equations problem implementation.
//!
//! Given positive integers `a`, `b`, and `c`, determine whether there exist
//! positive integers `x`, `y` such that `a x^2 + b y = c`.
//!
//! The witness integer `x` is encoded as a little-endian binary vector so the
//! model can represent large reductions without fixed-width overflow.

use crate::registry::{FieldInfo, ProblemSchemaEntry, ProblemSizeFieldEntry};
use crate::traits::Problem;
use crate::types::Or;
use num_bigint::{BigUint, ToBigUint};
use num_traits::{One, Zero};
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "QuadraticDiophantineEquations",
        display_name: "Quadratic Diophantine Equations",
        aliases: &["QDE"],
        dimensions: &[],
        module_path: module_path!(),
        description: "Decide whether ax^2 + by = c has a solution in positive integers x, y",
        fields: &[
            FieldInfo { name: "a", type_name: "BigUint", description: "Coefficient of x^2" },
            FieldInfo { name: "b", type_name: "BigUint", description: "Coefficient of y" },
            FieldInfo { name: "c", type_name: "BigUint", description: "Right-hand side constant" },
        ],
    }
}

inventory::submit! {
    ProblemSizeFieldEntry {
        name: "QuadraticDiophantineEquations",
        fields: &["bit_length_a", "bit_length_b", "bit_length_c"],
    }
}

/// Quadratic Diophantine Equations problem.
///
/// Given positive integers `a`, `b`, and `c`, determine whether there exist
/// positive integers `x`, `y` such that `a x^2 + b y = c`.
///
/// The configuration encodes `x` in little-endian binary:
/// `config[i] in {0,1}` is the coefficient of `2^i`.
#[derive(Debug, Clone, Serialize)]
pub struct QuadraticDiophantineEquations {
    /// Coefficient of x^2.
    #[serde(with = "crate::models::misc::biguint_serde::decimal_biguint")]
    a: BigUint,
    /// Coefficient of y.
    #[serde(with = "crate::models::misc::biguint_serde::decimal_biguint")]
    b: BigUint,
    /// Right-hand side constant.
    #[serde(with = "crate::models::misc::biguint_serde::decimal_biguint")]
    c: BigUint,
}

fn bit_length(value: &BigUint) -> usize {
    if value.is_zero() {
        0
    } else {
        let bytes = value.to_bytes_be();
        let msb = *bytes.first().expect("nonzero BigUint has bytes");
        8 * (bytes.len() - 1) + (8 - msb.leading_zeros() as usize)
    }
}

impl QuadraticDiophantineEquations {
    fn validate_inputs(a: &BigUint, b: &BigUint, c: &BigUint) -> Result<(), String> {
        if a.is_zero() {
            return Err("Coefficient a must be positive".to_string());
        }
        if b.is_zero() {
            return Err("Coefficient b must be positive".to_string());
        }
        if c.is_zero() {
            return Err("Right-hand side c must be positive".to_string());
        }
        Ok(())
    }

    fn isqrt(n: &BigUint) -> BigUint {
        if n.is_zero() {
            return BigUint::zero();
        }

        let mut low = BigUint::zero();
        let mut high = BigUint::one() << ((bit_length(n) + 1) / 2);

        while low < high {
            let mid = (&low + &high + BigUint::one()) >> 1usize;
            if &mid * &mid <= *n {
                low = mid;
            } else {
                high = mid - BigUint::one();
            }
        }

        low
    }

    /// Create a new QuadraticDiophantineEquations instance, returning an error
    /// instead of panicking when inputs are invalid.
    pub fn try_new<A, B, C>(a: A, b: B, c: C) -> Result<Self, String>
    where
        A: ToBigUint,
        B: ToBigUint,
        C: ToBigUint,
    {
        let a = a
            .to_biguint()
            .ok_or_else(|| "Coefficient a must be nonnegative".to_string())?;
        let b = b
            .to_biguint()
            .ok_or_else(|| "Coefficient b must be nonnegative".to_string())?;
        let c = c
            .to_biguint()
            .ok_or_else(|| "Right-hand side c must be nonnegative".to_string())?;
        Self::validate_inputs(&a, &b, &c)?;
        Ok(Self { a, b, c })
    }

    /// Create a new QuadraticDiophantineEquations instance.
    ///
    /// # Panics
    ///
    /// Panics if any of `a`, `b`, `c` is zero.
    pub fn new<A, B, C>(a: A, b: B, c: C) -> Self
    where
        A: ToBigUint,
        B: ToBigUint,
        C: ToBigUint,
    {
        Self::try_new(a, b, c).unwrap_or_else(|msg| panic!("{msg}"))
    }

    /// Get the coefficient a (coefficient of x^2).
    pub fn a(&self) -> &BigUint {
        &self.a
    }

    /// Get the coefficient b (coefficient of y).
    pub fn b(&self) -> &BigUint {
        &self.b
    }

    /// Get the right-hand side constant c.
    pub fn c(&self) -> &BigUint {
        &self.c
    }

    /// Number of bits needed to encode the coefficient a.
    pub fn bit_length_a(&self) -> usize {
        bit_length(&self.a)
    }

    /// Number of bits needed to encode the coefficient b.
    pub fn bit_length_b(&self) -> usize {
        bit_length(&self.b)
    }

    /// Number of bits needed to encode the constant c.
    pub fn bit_length_c(&self) -> usize {
        bit_length(&self.c)
    }

    fn max_x(&self) -> BigUint {
        if self.c < self.a {
            return BigUint::zero();
        }
        Self::isqrt(&(&self.c / &self.a))
    }

    fn witness_bit_length(&self) -> usize {
        let max_x = self.max_x();
        if max_x.is_zero() {
            0
        } else {
            bit_length(&max_x)
        }
    }

    /// Encode a candidate witness integer `x` as a little-endian binary configuration.
    pub fn encode_witness(&self, x: &BigUint) -> Option<Vec<usize>> {
        if x.is_zero() || x > &self.max_x() {
            return None;
        }

        let num_bits = self.witness_bit_length();
        let mut remaining = x.clone();
        let mut config = Vec::with_capacity(num_bits);

        for _ in 0..num_bits {
            config.push(if (&remaining & BigUint::one()).is_zero() {
                0
            } else {
                1
            });
            remaining >>= 1usize;
        }

        if remaining.is_zero() {
            Some(config)
        } else {
            None
        }
    }

    /// Decode a little-endian binary configuration into its candidate witness `x`.
    pub fn decode_witness(&self, config: &[usize]) -> Option<BigUint> {
        if config.len() != self.witness_bit_length() || config.iter().any(|&digit| digit > 1) {
            return None;
        }

        let mut value = BigUint::zero();
        let mut weight = BigUint::one();
        for &digit in config {
            if digit == 1 {
                value += &weight;
            }
            weight <<= 1usize;
        }
        Some(value)
    }

    /// Check whether a given x yields a valid positive integer y.
    ///
    /// Returns `Some(y)` if `y` is a positive integer, `None` otherwise.
    pub fn check_x(&self, x: &BigUint) -> Option<BigUint> {
        if x.is_zero() {
            return None;
        }

        let ax2 = &self.a * x * x;
        if ax2 >= self.c {
            return None;
        }

        let remainder = &self.c - ax2;
        if (&remainder % &self.b) != BigUint::zero() {
            return None;
        }

        let y = remainder / &self.b;
        if y.is_zero() {
            return None;
        }

        Some(y)
    }
}

#[derive(Deserialize)]
struct QuadraticDiophantineEquationsData {
    #[serde(with = "crate::models::misc::biguint_serde::decimal_biguint")]
    a: BigUint,
    #[serde(with = "crate::models::misc::biguint_serde::decimal_biguint")]
    b: BigUint,
    #[serde(with = "crate::models::misc::biguint_serde::decimal_biguint")]
    c: BigUint,
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
        let num_bits = self.witness_bit_length();
        if num_bits == 0 {
            Vec::new()
        } else {
            vec![2; num_bits]
        }
    }

    fn evaluate(&self, config: &[usize]) -> Or {
        let Some(x) = self.decode_witness(config) else {
            return Or(false);
        };

        if x.is_zero() || x > self.max_x() {
            return Or(false);
        }

        Or(self.check_x(&x).is_some())
    }
}

crate::declare_variants! {
    default QuadraticDiophantineEquations => "2^bit_length_c",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    let instance = QuadraticDiophantineEquations::new(3u32, 5u32, 53u32);
    let optimal_config = instance
        .encode_witness(&BigUint::from(1u32))
        .expect("x=1 should be a valid canonical witness");

    vec![crate::example_db::specs::ModelExampleSpec {
        id: "quadratic_diophantine_equations",
        instance: Box::new(instance),
        optimal_config,
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/algebraic/quadratic_diophantine_equations.rs"]
mod tests;
