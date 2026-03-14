//! Subset Sum problem implementation.
//!
//! Given a set of positive integers and a target value, the problem asks whether
//! any subset sums to exactly the target. One of Karp's original 21 NP-complete
//! problems (1972).
//!
//! This implementation uses arbitrary-precision integers (`BigUint`) so
//! reductions can construct large instances without fixed-width overflow.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{Problem, SatisfactionProblem};
use num_bigint::{BigUint, ToBigUint};
use num_traits::Zero;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "SubsetSum",
        display_name: "",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Find a subset of positive integers that sums to exactly a target value",
        fields: &[
            FieldInfo { name: "sizes", type_name: "Vec<BigUint>", description: "Positive integer sizes s(a) for each element" },
            FieldInfo { name: "target", type_name: "BigUint", description: "Target sum B" },
        ],
    }
}

/// The Subset Sum problem.
///
/// Given a set of `n` positive integers and a target `B`, determine whether
/// there exists a subset whose elements sum to exactly `B`.
///
/// # Representation
///
/// Each element has a binary variable: `x_i = 1` if element `i` is selected,
/// `0` otherwise. The problem is satisfiable iff `∑_{i: x_i=1} sizes[i] == target`.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::SubsetSum;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// let problem = SubsetSum::new(vec![3u32, 7, 1, 8, 2, 4], 11u32);
/// let solver = BruteForce::new();
/// let solution = solver.find_satisfying(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubsetSum {
    #[serde(with = "decimal_biguint_vec")]
    sizes: Vec<BigUint>,
    #[serde(with = "decimal_biguint")]
    target: BigUint,
}

impl SubsetSum {
    /// Create a new SubsetSum instance.
    ///
    /// # Panics
    ///
    /// Panics if any size is not positive (must be > 0).
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
            .expect("SubsetSum target must be nonnegative");
        Self { sizes, target }
    }

    /// Create a new SubsetSum instance without validating sizes.
    ///
    /// This is intended for reductions that produce SubsetSum instances
    /// where positivity is guaranteed by construction.
    pub(crate) fn new_unchecked(sizes: Vec<BigUint>, target: BigUint) -> Self {
        Self { sizes, target }
    }

    /// Returns the element sizes.
    pub fn sizes(&self) -> &[BigUint] {
        &self.sizes
    }

    /// Returns the target sum.
    pub fn target(&self) -> &BigUint {
        &self.target
    }

    /// Returns the number of elements.
    pub fn num_elements(&self) -> usize {
        self.sizes.len()
    }
}

impl Problem for SubsetSum {
    const NAME: &'static str = "SubsetSum";
    type Metric = bool;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.num_elements()]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        if config.len() != self.num_elements() {
            return false;
        }
        if config.iter().any(|&v| v >= 2) {
            return false;
        }
        let mut total = BigUint::zero();
        for (i, &x) in config.iter().enumerate() {
            if x == 1 {
                total += &self.sizes[i];
            }
        }
        total == self.target
    }
}

impl SatisfactionProblem for SubsetSum {}

crate::declare_variants! {
    default sat SubsetSum => "2^(num_elements / 2)",
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

#[cfg(test)]
#[path = "../../unit_tests/models/misc/subset_sum.rs"]
mod tests;
