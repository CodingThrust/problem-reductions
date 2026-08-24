//! Partition problem implementation.
//!
//! Given a finite set of positive integers, determine whether it can be
//! partitioned into two subsets of equal sum. One of Karp's original 21
//! NP-complete problems (1972), Garey & Johnson SP12.

use crate::registry::{ConstructionError, FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "Partition",
        display_name: "Partition",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Misc,
        module_path: module_path!(),
        description: "Determine whether a multiset of positive integers can be partitioned into two subsets of equal sum",
        fields: &[
            FieldInfo { name: "sizes", type_name: "Vec<i64>", description: "Positive integer size for each element" },
        ],
    }
}

/// The Partition problem.
///
/// Given a finite set `A` with `n` positive integer sizes, determine whether
/// there exists a subset `A' ⊆ A` such that `∑_{a ∈ A'} s(a) = ∑_{a ∈ A\A'} s(a)`.
///
/// # Representation
///
/// Each element has a binary variable: `x_i = 1` if element `i` is in the
/// second subset, `0` if in the first. The problem is satisfiable iff
/// `∑_{i: x_i=1} sizes[i] = total_sum / 2`.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::Partition;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// let problem = Partition::new(vec![3, 1, 1, 2, 2, 1]).unwrap();
/// let solver = BruteForce::new();
/// let solution = solver.find_witness(&problem).unwrap();
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct Partition {
    sizes: Vec<i64>,
}

impl Partition {
    /// Create a new Partition instance.
    ///
    pub fn new(sizes: Vec<i64>) -> Result<Self, ConstructionError> {
        if sizes.is_empty() {
            return Err(ConstructionError::Conversion(
                "Partition requires at least one element".into(),
            ));
        }
        if sizes.iter().any(|&size| size <= 0) {
            return Err(ConstructionError::Conversion(
                "all Partition sizes must be positive".into(),
            ));
        }
        sizes
            .iter()
            .try_fold(0i64, |sum, &size| sum.checked_add(size))
            .ok_or_else(|| ConstructionError::IntegerOverflow("summing Partition sizes".into()))?;
        Ok(Self { sizes })
    }

    /// Returns the element sizes.
    pub fn sizes(&self) -> &[i64] {
        &self.sizes
    }

    /// Returns the number of elements.
    pub fn num_elements(&self) -> usize {
        self.sizes.len()
    }

    /// Returns the total sum of all sizes.
    pub fn total_sum(&self) -> i64 {
        self.sizes.iter().sum()
    }
}

impl<'de> Deserialize<'de> for Partition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Raw {
            sizes: Vec<i64>,
        }

        let raw = Raw::deserialize(deserializer)?;
        Self::new(raw.sizes).map_err(serde::de::Error::custom)
    }
}

impl Problem for Partition {
    const NAME: &'static str = "Partition";
    type Value = crate::types::Or;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.num_elements()]
    }

    fn evaluate(
        &self,
        config: &[usize],
    ) -> Result<crate::types::Or, crate::traits::EvaluationError> {
        Ok({
            crate::types::Or({
                if config.len() != self.num_elements() {
                    return Ok(crate::types::Or(false));
                }
                if config.iter().any(|&v| v >= 2) {
                    return Ok(crate::types::Or(false));
                }
                let selected_sum: i64 = config
                    .iter()
                    .enumerate()
                    .filter(|(_, &x)| x == 1)
                    .map(|(i, _)| self.sizes[i])
                    .sum();
                selected_sum == self.total_sum() - selected_sum
            })
        })
    }
}

crate::declare_variants! {
    default Partition => "2^(num_elements / 2)",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "partition",
        instance: Box::new(Partition::new(vec![3, 1, 1, 2, 2, 1]).unwrap()),
        optimal_config: vec![1, 0, 0, 1, 0, 0],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/partition.rs"]
mod tests;
