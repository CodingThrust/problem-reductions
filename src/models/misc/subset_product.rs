//! Subset Product problem implementation.
//!
//! Given a multiset of positive integers and a target value, determine whether
//! some submultiset has product exactly equal to the target.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Or;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "SubsetProduct",
        display_name: "Subset Product",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Find a subset of positive integers whose product equals a target value",
        fields: &[
            FieldInfo { name: "values", type_name: "Vec<u64>", description: "Positive integer values in the multiset" },
            FieldInfo { name: "target", type_name: "u64", description: "Target product" },
        ],
    }
}

/// The Subset Product problem.
///
/// Given positive integers `a_1, ..., a_n` and a target `B`, determine whether
/// there exists a subset whose product is exactly `B`.
///
/// Each element has a binary decision variable: `x_i = 1` if value `i` is
/// selected, `0` otherwise.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubsetProduct {
    values: Vec<u64>,
    target: u64,
}

impl SubsetProduct {
    /// Create a new SubsetProduct instance.
    ///
    /// # Panics
    ///
    /// Panics if any value is zero.
    pub fn new(values: Vec<u64>, target: u64) -> Self {
        assert!(
            values.iter().all(|&value| value > 0),
            "All values must be positive (> 0)"
        );
        Self { values, target }
    }

    /// Returns the multiset values.
    pub fn values(&self) -> &[u64] {
        &self.values
    }

    /// Returns the target product.
    pub fn target(&self) -> u64 {
        self.target
    }

    /// Returns the number of elements.
    pub fn num_elements(&self) -> usize {
        self.values.len()
    }
}

impl Problem for SubsetProduct {
    const NAME: &'static str = "SubsetProduct";
    type Value = Or;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.values.len()]
    }

    fn evaluate(&self, config: &[usize]) -> Or {
        if config.len() != self.values.len() || config.iter().any(|&value| value > 1) {
            return Or(false);
        }

        let mut product = 1u64;
        for (index, &selected) in config.iter().enumerate() {
            if selected == 1 {
                let Some(next_product) = product.checked_mul(self.values[index]) else {
                    return Or(false);
                };
                product = next_product;
                if product > self.target {
                    return Or(false);
                }
            }
        }

        Or(product == self.target)
    }
}

crate::declare_variants! {
    default SubsetProduct => "2^num_elements",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "subset_product",
        instance: Box::new(SubsetProduct::new(vec![2, 3, 5, 7], 30)),
        optimal_config: vec![1, 1, 1, 0],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/subset_product.rs"]
mod tests;
