//! Quadratic Congruences problem implementation.
//!
//! Given non-negative integers `a`, `b`, and `c` with `b > 0` and `a < b`,
//! determine whether there exists a positive integer `x < c` such that
//! `x² ≡ a (mod b)`.
//!
//! The witness integer `x` is encoded as a little-endian binary vector so the
//! model can represent arbitrarily large instances while still fitting the
//! crate's `Vec<usize>` configuration interface.

use crate::registry::{FieldInfo, ProblemSchemaEntry, ProblemSizeFieldEntry};
use crate::traits::Problem;
use crate::types::Or;
use num_bigint::{BigUint, ToBigUint};
use num_traits::{One, Zero};
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "QuadraticCongruences",
        display_name: "Quadratic Congruences",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Decide whether x² ≡ a (mod b) has a solution for x in {1, ..., c-1}",
        fields: &[
            FieldInfo { name: "a", type_name: "BigUint", description: "a" },
            FieldInfo { name: "b", type_name: "BigUint", description: "b" },
            FieldInfo { name: "c", type_name: "BigUint", description: "c" },
        ],
    }
}

inventory::submit! {
    ProblemSizeFieldEntry {
        name: "QuadraticCongruences",
        fields: &["bit_length_a", "bit_length_b", "bit_length_c"],
    }
}

/// Quadratic Congruences problem.
///
/// Given non-negative integers `a`, `b`, `c` with `b > 0` and `a < b`,
/// determine whether there exists a positive integer `x < c` such that
/// `x² ≡ a (mod b)`.
///
/// The configuration encodes `x` in little-endian binary:
/// `config[i] ∈ {0,1}` is the coefficient of `2^i`.
#[derive(Debug, Clone, Serialize)]
pub struct QuadraticCongruences {
    /// Quadratic residue target.
    #[serde(with = "crate::models::misc::biguint_serde::decimal_biguint")]
    a: BigUint,
    /// Modulus.
    #[serde(with = "crate::models::misc::biguint_serde::decimal_biguint")]
    b: BigUint,
    /// Search-space bound; feasible witnesses satisfy `1 <= x < c`.
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

impl QuadraticCongruences {
    fn validate_inputs(a: &BigUint, b: &BigUint, c: &BigUint) -> Result<(), String> {
        if b.is_zero() {
            return Err("Modulus b must be positive".to_string());
        }
        if c.is_zero() {
            return Err("Bound c must be positive".to_string());
        }
        if a >= b {
            return Err(format!("Residue a ({a}) must be less than modulus b ({b})"));
        }
        Ok(())
    }

    /// Create a new QuadraticCongruences instance, returning an error instead of
    /// panicking when the inputs are invalid.
    pub fn try_new<A, B, C>(a: A, b: B, c: C) -> Result<Self, String>
    where
        A: ToBigUint,
        B: ToBigUint,
        C: ToBigUint,
    {
        let a = a
            .to_biguint()
            .ok_or_else(|| "Residue a must be nonnegative".to_string())?;
        let b = b
            .to_biguint()
            .ok_or_else(|| "Modulus b must be nonnegative".to_string())?;
        let c = c
            .to_biguint()
            .ok_or_else(|| "Bound c must be nonnegative".to_string())?;
        Self::validate_inputs(&a, &b, &c)?;
        Ok(Self { a, b, c })
    }

    /// Create a new QuadraticCongruences instance.
    ///
    /// # Panics
    ///
    /// Panics if `b == 0`, `c == 0`, or `a >= b`.
    pub fn new<A, B, C>(a: A, b: B, c: C) -> Self
    where
        A: ToBigUint,
        B: ToBigUint,
        C: ToBigUint,
    {
        Self::try_new(a, b, c).unwrap_or_else(|msg| panic!("{msg}"))
    }

    /// Get the quadratic residue target `a`.
    pub fn a(&self) -> &BigUint {
        &self.a
    }

    /// Get the modulus `b`.
    pub fn b(&self) -> &BigUint {
        &self.b
    }

    /// Get the search-space bound `c`.
    pub fn c(&self) -> &BigUint {
        &self.c
    }

    /// Number of bits needed to encode the residue target.
    pub fn bit_length_a(&self) -> usize {
        bit_length(&self.a)
    }

    /// Number of bits needed to encode the modulus.
    pub fn bit_length_b(&self) -> usize {
        bit_length(&self.b)
    }

    /// Number of bits needed to encode the search bound.
    pub fn bit_length_c(&self) -> usize {
        bit_length(&self.c)
    }

    fn witness_bit_length(&self) -> usize {
        if self.c <= BigUint::one() {
            0
        } else {
            bit_length(&(&self.c - BigUint::one()))
        }
    }

    /// Encode a witness integer `x` as a little-endian binary configuration.
    pub fn encode_witness(&self, x: &BigUint) -> Option<Vec<usize>> {
        if x.is_zero() || x >= &self.c {
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

    /// Decode a little-endian binary configuration into its witness integer `x`.
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
}

#[derive(Deserialize)]
struct QuadraticCongruencesData {
    #[serde(with = "crate::models::misc::biguint_serde::decimal_biguint")]
    a: BigUint,
    #[serde(with = "crate::models::misc::biguint_serde::decimal_biguint")]
    b: BigUint,
    #[serde(with = "crate::models::misc::biguint_serde::decimal_biguint")]
    c: BigUint,
}

impl<'de> Deserialize<'de> for QuadraticCongruences {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = QuadraticCongruencesData::deserialize(deserializer)?;
        Self::try_new(data.a, data.b, data.c).map_err(D::Error::custom)
    }
}

impl Problem for QuadraticCongruences {
    const NAME: &'static str = "QuadraticCongruences";
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

        if x.is_zero() || x >= *self.c() {
            return Or(false);
        }

        let satisfies = (&x * &x) % self.b() == self.a().clone();
        Or(satisfies)
    }
}

crate::declare_variants! {
    default QuadraticCongruences => "2^bit_length_c",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    let instance = QuadraticCongruences::new(4u32, 15u32, 10u32);
    let optimal_config = instance
        .encode_witness(&BigUint::from(2u32))
        .expect("x=2 should be a valid canonical witness");

    vec![crate::example_db::specs::ModelExampleSpec {
        id: "quadratic_congruences",
        instance: Box::new(instance),
        optimal_config,
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/algebraic/quadratic_congruences.rs"]
mod tests;
