//! Subset Product problem implementation.
//!
//! Given a set of positive integers and a target value, the problem asks whether
//! any subset's product equals exactly the target. A multiplicative analogue of
//! Subset Sum; NP-complete (see e.g. Garey & Johnson, 1979).
//!
//! This implementation uses arbitrary-precision integers (`BigUint`) so
//! reductions can construct large instances without fixed-width overflow.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use num_bigint::{BigUint, ToBigUint};
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "SubsetProduct",
        display_name: "Subset Product",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Find a subset of positive integers whose product equals exactly a target value",
        fields: &[
            FieldInfo { name: "sizes", type_name: "Vec<BigUint>", description: "Positive integer sizes s(a) for each element" },
            FieldInfo { name: "target", type_name: "BigUint", description: "Target product B" },
        ],
    }
}

/// The Subset Product problem.
///
/// Given a set of `n` positive integers and a target `B`, determine whether
/// there exists a subset whose elements multiply to exactly `B`.
///
/// # Representation
///
/// Each element has a binary variable: `x_i = 1` if element `i` is selected,
/// `0` otherwise. The problem is satisfiable iff `∏_{i: x_i=1} sizes[i] == target`.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::SubsetProduct;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// let problem = SubsetProduct::new(vec![2u32, 3, 5, 7, 6, 10], 210u32);
/// let solver = BruteForce::new();
/// let solution = solver.find_witness(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubsetProduct {
    #[serde(with = "decimal_biguint_vec")]
    sizes: Vec<BigUint>,
    #[serde(with = "decimal_biguint")]
    target: BigUint,
}

impl SubsetProduct {
    /// Create a new SubsetProduct instance.
    ///
    /// # Panics
    ///
    /// Panics if any size is not positive (must be > 0) or if target is zero.
    pub fn new<S, T>(sizes: Vec<S>, target: T) -> Self
    where
        S: ToBigUint,
        T: ToBigUint,
    {
        let sizes: Vec<BigUint> = sizes
            .into_iter()
            .map(|s| s.to_biguint().expect("All sizes must be positive (> 0)"))
            .collect();
        assert!(
            sizes.iter().all(|s| !s.is_zero()),
            "All sizes must be positive (> 0)"
        );
        let target = target
            .to_biguint()
            .expect("SubsetProduct target must be nonnegative");
        assert!(!target.is_zero(), "SubsetProduct target must be positive");
        Self { sizes, target }
    }

    /// Create a new SubsetProduct instance without validating sizes.
    ///
    /// This is intended for reductions that produce SubsetProduct instances
    /// where positivity is guaranteed by construction.
    #[allow(dead_code)]
    pub(crate) fn new_unchecked(sizes: Vec<BigUint>, target: BigUint) -> Self {
        Self { sizes, target }
    }

    /// Returns the element sizes.
    pub fn sizes(&self) -> &[BigUint] {
        &self.sizes
    }

    /// Returns the target product.
    pub fn target(&self) -> &BigUint {
        &self.target
    }

    /// Returns the number of elements.
    pub fn num_elements(&self) -> usize {
        self.sizes.len()
    }
}

impl Problem for SubsetProduct {
    const NAME: &'static str = "SubsetProduct";
    type Value = crate::types::Or;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.num_elements()]
    }

    fn evaluate(&self, config: &[usize]) -> crate::types::Or {
        crate::types::Or({
            if config.len() != self.num_elements() {
                return crate::types::Or(false);
            }
            if config.iter().any(|&v| v >= 2) {
                return crate::types::Or(false);
            }
            let mut product = BigUint::one();
            for (i, &x) in config.iter().enumerate() {
                if x == 1 {
                    product *= &self.sizes[i];
                }
            }
            product == self.target
        })
    }
}

crate::declare_variants! {
    default SubsetProduct => "2^(num_elements / 2)",
}

mod decimal_biguint {
    use super::BigUint;
    use serde::de::Error;
    use serde::{Deserialize, Deserializer, Serializer};

    #[derive(Deserialize)]
    #[serde(untagged)]
    pub(super) enum Repr {
        String(String),
        U64(u64),
        I64(i64),
    }

    pub(super) fn parse_repr<E: Error>(value: Repr) -> Result<BigUint, E> {
        match value {
            Repr::String(s) => BigUint::parse_bytes(s.as_bytes(), 10)
                .ok_or_else(|| E::custom(format!("invalid decimal integer: {s}"))),
            Repr::U64(n) => Ok(BigUint::from(n)),
            Repr::I64(n) if n >= 0 => Ok(BigUint::from(n as u64)),
            Repr::I64(n) => Err(E::custom(format!("expected nonnegative integer, got {n}"))),
        }
    }

    pub fn serialize<S>(value: &BigUint, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&value.to_str_radix(10))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<BigUint, D::Error>
    where
        D: Deserializer<'de>,
    {
        parse_repr(Repr::deserialize(deserializer)?)
    }
}

mod decimal_biguint_vec {
    use super::BigUint;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(values: &[BigUint], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let strings: Vec<String> = values.iter().map(ToString::to_string).collect();
        strings.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<BigUint>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let values = Vec::<super::decimal_biguint::Repr>::deserialize(deserializer)?;
        values
            .into_iter()
            .map(super::decimal_biguint::parse_repr::<D::Error>)
            .collect()
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // 6 elements [2,3,5,7,6,10], target 210 → select {2,3,5,7}
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "subset_product",
        instance: Box::new(SubsetProduct::new(vec![2u32, 3, 5, 7, 6, 10], 210u32)),
        optimal_config: vec![1, 1, 1, 1, 0, 0],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/subset_product.rs"]
mod tests;
