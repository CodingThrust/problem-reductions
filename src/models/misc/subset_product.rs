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
    #[serde(with = "super::biguint_serde::decimal_biguint_vec")]
    sizes: Vec<BigUint>,
    #[serde(with = "super::biguint_serde::decimal_biguint")]
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

    /// Create a SubsetProduct without validating sizes (for testing edge cases).
    #[cfg(test)]
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
                    if product > self.target {
                        return crate::types::Or(false);
                    }
                }
            }
            product == self.target
        })
    }
}

crate::declare_variants! {
    default SubsetProduct => "2^num_elements",
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
