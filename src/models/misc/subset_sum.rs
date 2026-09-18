//! Subset Sum problem implementation.
//!
//! Given a set of positive integers and a target value, the problem asks whether
//! any subset sums to exactly the target. One of Karp's original 21 NP-complete
//! problems (1972).
//!
//! This implementation uses arbitrary-precision integers (`BigUint`) so
//! reductions can construct large instances without fixed-width overflow.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use num_bigint::{BigUint, ToBigUint};
use num_traits::Zero;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "SubsetSum",
        display_name: "Subset Sum",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Misc,
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
/// use problemreductions::{Problem, BruteForce};
///
/// let problem = SubsetSum::new(vec![3u32, 7, 1, 8, 2, 4], 11u32);
/// let solver = BruteForce::new();
/// let solution = solver.solve(&problem).unwrap();
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "SubsetSumData")]
pub struct SubsetSum {
    #[serde(with = "super::biguint_serde::decimal_biguint_vec")]
    sizes: Vec<BigUint>,
    #[serde(with = "super::biguint_serde::decimal_biguint")]
    target: BigUint,
}

#[derive(Deserialize)]
struct SubsetSumData {
    #[serde(with = "super::biguint_serde::decimal_biguint_vec")]
    sizes: Vec<BigUint>,
    #[serde(with = "super::biguint_serde::decimal_biguint")]
    target: BigUint,
}

impl TryFrom<SubsetSumData> for SubsetSum {
    type Error = crate::registry::ConstructionError;
    fn try_from(data: SubsetSumData) -> Result<Self, Self::Error> {
        if data.sizes.iter().any(BigUint::is_zero) {
            return Err("all sizes must be positive (> 0)".into());
        }
        Ok(Self {
            sizes: data.sizes,
            target: data.target,
        })
    }
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
        Self::try_new(sizes, target).unwrap_or_else(|error| panic!("{error}"))
    }

    fn try_new<S, T>(sizes: Vec<S>, target: T) -> Result<Self, crate::registry::ConstructionError>
    where
        S: ToBigUint,
        T: ToBigUint,
    {
        let sizes = sizes
            .into_iter()
            .map(|size| size.to_biguint().ok_or("all sizes must be positive (> 0)"))
            .collect::<Result<Vec<_>, _>>()?;
        let target = target
            .to_biguint()
            .ok_or("SubsetSum target must be nonnegative")?;
        SubsetSumData { sizes, target }.try_into()
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
    type Solution = Vec<bool>;
    type Value = crate::types::Or;

    crate::problem_parameters![("num_elements", num_elements),];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<crate::types::Or, crate::traits::EvaluationError> {
        Ok({
            crate::types::Or({
                if config.len() != self.num_elements() {
                    return Err(crate::traits::EvaluationError::InvalidConfiguration(
                        "subset-selection length does not match the elements".into(),
                    ));
                }
                let mut total = BigUint::zero();
                for (i, &x) in config.iter().enumerate() {
                    if x {
                        total += &self.sizes[i];
                    }
                }
                total == self.target
            })
        })
    }
}

impl crate::solvers::BruteForceProblem for SubsetSum {
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.num_elements()]
    }
}

crate::declare_variants! {
    default SubsetSum => "2^(num_elements / 2)",
}

crate::register_brute_force! {
    SubsetSum decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // 6 elements [3,7,1,8,2,4], target 11 → select {3,8}
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "subset_sum",
        instance: Box::new(SubsetSum::new(vec![3u32, 7, 1, 8, 2, 4], 11u32)),
        optimal_config: serde_json::json!(vec![true, false, false, true, false, false]),
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/subset_sum.rs"]
mod tests;
