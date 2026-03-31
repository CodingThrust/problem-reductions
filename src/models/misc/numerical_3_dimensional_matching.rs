//! Numerical 3-Dimensional Matching (N3DM) problem implementation.
//!
//! Given disjoint sets W, X, Y each with m elements, sizes s(a) ∈ Z⁺ for
//! every element with B/4 < s(a) < B/2, and a bound B where the total sum
//! equals mB.  Decide whether W ∪ X ∪ Y can be partitioned into m triples,
//! each containing one element from W, X, and Y, with each triple summing
//! to exactly B.

use crate::registry::{FieldInfo, ProblemSchemaEntry, ProblemSizeFieldEntry};
use crate::traits::Problem;
use crate::types::Or;
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "Numerical3DimensionalMatching",
        display_name: "Numerical 3-Dimensional Matching",
        aliases: &["N3DM"],
        dimensions: &[],
        module_path: module_path!(),
        description: "Partition W∪X∪Y into m triples (one from each set) each summing to B",
        fields: &[
            FieldInfo { name: "sizes_w", type_name: "Vec<u64>", description: "Positive integer sizes for each element of W" },
            FieldInfo { name: "sizes_x", type_name: "Vec<u64>", description: "Positive integer sizes for each element of X" },
            FieldInfo { name: "sizes_y", type_name: "Vec<u64>", description: "Positive integer sizes for each element of Y" },
            FieldInfo { name: "bound", type_name: "u64", description: "Target sum B for each triple" },
        ],
    }
}

inventory::submit! {
    ProblemSizeFieldEntry {
        name: "Numerical3DimensionalMatching",
        fields: &["num_groups", "bound"],
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Numerical3DimensionalMatching {
    sizes_w: Vec<u64>,
    sizes_x: Vec<u64>,
    sizes_y: Vec<u64>,
    bound: u64,
}

impl Numerical3DimensionalMatching {
    fn validate_inputs(
        sizes_w: &[u64],
        sizes_x: &[u64],
        sizes_y: &[u64],
        bound: u64,
    ) -> Result<(), String> {
        let m = sizes_w.len();
        if m == 0 {
            return Err(
                "Numerical3DimensionalMatching requires at least one element per set".to_string(),
            );
        }
        if sizes_x.len() != m || sizes_y.len() != m {
            return Err(
                "Numerical3DimensionalMatching requires all three sets to have the same size"
                    .to_string(),
            );
        }
        if bound == 0 {
            return Err("Numerical3DimensionalMatching requires a positive bound".to_string());
        }

        let bound128 = u128::from(bound);
        for &size in sizes_w.iter().chain(sizes_x.iter()).chain(sizes_y.iter()) {
            if size == 0 {
                return Err("All sizes must be positive (> 0)".to_string());
            }
            let size128 = u128::from(size);
            if !(4 * size128 > bound128 && 2 * size128 < bound128) {
                return Err("Every size must lie strictly between B/4 and B/2".to_string());
            }
        }

        let total_sum: u128 = sizes_w
            .iter()
            .chain(sizes_x.iter())
            .chain(sizes_y.iter())
            .map(|&s| u128::from(s))
            .sum();
        let expected_sum = bound128 * (m as u128);
        if total_sum != expected_sum {
            return Err("Total sum of all sizes must equal m * bound".to_string());
        }
        if total_sum > u128::from(u64::MAX) {
            return Err("Total sum exceeds u64 range".to_string());
        }

        Ok(())
    }

    pub fn try_new(
        sizes_w: Vec<u64>,
        sizes_x: Vec<u64>,
        sizes_y: Vec<u64>,
        bound: u64,
    ) -> Result<Self, String> {
        Self::validate_inputs(&sizes_w, &sizes_x, &sizes_y, bound)?;
        Ok(Self {
            sizes_w,
            sizes_x,
            sizes_y,
            bound,
        })
    }

    /// Create a new Numerical 3-Dimensional Matching instance.
    ///
    /// # Panics
    ///
    /// Panics if the input violates the N3DM invariants.
    pub fn new(sizes_w: Vec<u64>, sizes_x: Vec<u64>, sizes_y: Vec<u64>, bound: u64) -> Self {
        Self::try_new(sizes_w, sizes_x, sizes_y, bound)
            .unwrap_or_else(|message| panic!("{message}"))
    }

    pub fn sizes_w(&self) -> &[u64] {
        &self.sizes_w
    }

    pub fn sizes_x(&self) -> &[u64] {
        &self.sizes_x
    }

    pub fn sizes_y(&self) -> &[u64] {
        &self.sizes_y
    }

    pub fn bound(&self) -> u64 {
        self.bound
    }

    pub fn num_groups(&self) -> usize {
        self.sizes_w.len()
    }
}

#[derive(Deserialize)]
struct Numerical3DimensionalMatchingData {
    sizes_w: Vec<u64>,
    sizes_x: Vec<u64>,
    sizes_y: Vec<u64>,
    bound: u64,
}

impl<'de> Deserialize<'de> for Numerical3DimensionalMatching {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = Numerical3DimensionalMatchingData::deserialize(deserializer)?;
        Self::try_new(data.sizes_w, data.sizes_x, data.sizes_y, data.bound)
            .map_err(D::Error::custom)
    }
}

impl Problem for Numerical3DimensionalMatching {
    const NAME: &'static str = "Numerical3DimensionalMatching";
    type Value = Or;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![self.num_groups(); 2 * self.num_groups()]
    }

    fn evaluate(&self, config: &[usize]) -> Or {
        Or({
            let m = self.num_groups();
            if config.len() != 2 * m {
                return Or(false);
            }

            // First m values: assignment of X-elements to W-elements (must be a permutation)
            let x_perm = &config[..m];
            // Second m values: assignment of Y-elements to W-elements (must be a permutation)
            let y_perm = &config[m..];

            // Check that both are valid permutations of 0..m
            let mut x_used = vec![false; m];
            let mut y_used = vec![false; m];

            for i in 0..m {
                if x_perm[i] >= m || y_perm[i] >= m {
                    return Or(false);
                }
                if x_used[x_perm[i]] || y_used[y_perm[i]] {
                    return Or(false);
                }
                x_used[x_perm[i]] = true;
                y_used[y_perm[i]] = true;
            }

            // Check that each triple sums to B
            let target = u128::from(self.bound);
            (0..m).all(|i| {
                let sum = u128::from(self.sizes_w[i])
                    + u128::from(self.sizes_x[x_perm[i]])
                    + u128::from(self.sizes_y[y_perm[i]]);
                sum == target
            })
        })
    }
}

crate::declare_variants! {
    default Numerical3DimensionalMatching => "num_groups^(2 * num_groups)",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "numerical_3_dimensional_matching",
        instance: Box::new(Numerical3DimensionalMatching::new(
            vec![4, 5],
            vec![4, 5],
            vec![5, 7],
            15,
        )),
        optimal_config: vec![0, 1, 1, 0],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/numerical_3_dimensional_matching.rs"]
mod tests;
