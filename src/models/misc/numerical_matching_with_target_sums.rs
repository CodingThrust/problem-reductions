//! Numerical Matching with Target Sums problem implementation.
//!
//! Given two disjoint sets X and Y each with m elements, integer sizes
//! s(x) for x ∈ X and s(y) for y ∈ Y, and a multiset of m target values
//! B_1, …, B_m, decide whether X ∪ Y can be partitioned into m pairs,
//! each containing one element from X and one from Y, such that the
//! multiset of pair sums {s(x_i) + s(y_{π(i)})} equals the target multiset.

use crate::registry::{FieldInfo, ProblemSchemaEntry, ProblemSizeFieldEntry};
use crate::traits::Problem;
use crate::types::Or;
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "NumericalMatchingWithTargetSums",
        display_name: "Numerical Matching with Target Sums",
        aliases: &["NMTS"],
        dimensions: &[],
        module_path: module_path!(),
        description: "Partition X∪Y into m pairs (one from X, one from Y) with pair sums matching targets",
        fields: &[
            FieldInfo { name: "sizes_x", type_name: "Vec<i64>", description: "Integer sizes for each element of X" },
            FieldInfo { name: "sizes_y", type_name: "Vec<i64>", description: "Integer sizes for each element of Y" },
            FieldInfo { name: "targets", type_name: "Vec<i64>", description: "Target sums for each pair" },
        ],
    }
}

inventory::submit! {
    ProblemSizeFieldEntry {
        name: "NumericalMatchingWithTargetSums",
        fields: &["num_pairs"],
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct NumericalMatchingWithTargetSums {
    sizes_x: Vec<i64>,
    sizes_y: Vec<i64>,
    targets: Vec<i64>,
}

impl NumericalMatchingWithTargetSums {
    fn validate_inputs(sizes_x: &[i64], sizes_y: &[i64], targets: &[i64]) -> Result<(), String> {
        let m = sizes_x.len();
        if m == 0 {
            return Err(
                "NumericalMatchingWithTargetSums requires at least one element per set".to_string(),
            );
        }
        if sizes_y.len() != m {
            return Err(
                "NumericalMatchingWithTargetSums requires sizes_x and sizes_y to have the same length"
                    .to_string(),
            );
        }
        if targets.len() != m {
            return Err(
                "NumericalMatchingWithTargetSums requires targets to have the same length as sizes_x"
                    .to_string(),
            );
        }
        Ok(())
    }

    pub fn try_new(
        sizes_x: Vec<i64>,
        sizes_y: Vec<i64>,
        targets: Vec<i64>,
    ) -> Result<Self, String> {
        Self::validate_inputs(&sizes_x, &sizes_y, &targets)?;
        Ok(Self {
            sizes_x,
            sizes_y,
            targets,
        })
    }

    /// Create a new Numerical Matching with Target Sums instance.
    ///
    /// # Panics
    ///
    /// Panics if the input violates the NMTS invariants.
    pub fn new(sizes_x: Vec<i64>, sizes_y: Vec<i64>, targets: Vec<i64>) -> Self {
        Self::try_new(sizes_x, sizes_y, targets).unwrap_or_else(|message| panic!("{message}"))
    }

    /// Number of pairs (m).
    pub fn num_pairs(&self) -> usize {
        self.sizes_x.len()
    }

    /// Integer sizes for each element of X.
    pub fn sizes_x(&self) -> &[i64] {
        &self.sizes_x
    }

    /// Integer sizes for each element of Y.
    pub fn sizes_y(&self) -> &[i64] {
        &self.sizes_y
    }

    /// Target sums for each pair.
    pub fn targets(&self) -> &[i64] {
        &self.targets
    }
}

#[derive(Deserialize)]
struct NumericalMatchingWithTargetSumsData {
    sizes_x: Vec<i64>,
    sizes_y: Vec<i64>,
    targets: Vec<i64>,
}

impl<'de> Deserialize<'de> for NumericalMatchingWithTargetSums {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = NumericalMatchingWithTargetSumsData::deserialize(deserializer)?;
        Self::try_new(data.sizes_x, data.sizes_y, data.targets).map_err(D::Error::custom)
    }
}

impl Problem for NumericalMatchingWithTargetSums {
    const NAME: &'static str = "NumericalMatchingWithTargetSums";
    type Value = Or;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        let m = self.num_pairs();
        vec![m; m]
    }

    fn evaluate(&self, config: &[usize]) -> Or {
        Or({
            let m = self.num_pairs();
            if config.len() != m {
                return Or(false);
            }

            // Check config is valid permutation of 0..m
            let mut used = vec![false; m];
            for &idx in config {
                if idx >= m || used[idx] {
                    return Or(false);
                }
                used[idx] = true;
            }

            // Compute pair sums and compare multisets
            let mut pair_sums: Vec<i64> = (0..m)
                .map(|i| self.sizes_x[i] + self.sizes_y[config[i]])
                .collect();
            let mut sorted_targets = self.targets.clone();
            pair_sums.sort();
            sorted_targets.sort();
            pair_sums == sorted_targets
        })
    }
}

crate::declare_variants! {
    default NumericalMatchingWithTargetSums => "2^num_pairs",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "numerical_matching_with_target_sums",
        instance: Box::new(NumericalMatchingWithTargetSums::new(
            vec![1, 4, 7],
            vec![2, 5, 3],
            vec![3, 7, 12],
        )),
        optimal_config: vec![0, 2, 1],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/numerical_matching_with_target_sums.rs"]
mod tests;
