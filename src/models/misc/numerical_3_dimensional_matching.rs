//! Numerical 3-Dimensional Matching (N3DM) problem implementation.
//!
//! Given disjoint sets W, X, Y each with m elements, sizes s(a) ∈ Z⁺ for
//! every element with B/4 < s(a) < B/2, and a bound B where the total sum
//! equals mB.  Decide whether W ∪ X ∪ Y can be partitioned into m triples,
//! each containing one element from W, X, and Y, with each triple summing
//! to exactly B.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
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
        category: crate::registry::ProblemCategory::Misc,
        module_path: module_path!(),
        description: "Partition W∪X∪Y into m triples (one from each set) each summing to B",
        fields: &[
            FieldInfo { name: "sizes_w", type_name: "Vec<i64>", description: "Positive integer sizes for each element of W" },
            FieldInfo { name: "sizes_x", type_name: "Vec<i64>", description: "Positive integer sizes for each element of X" },
            FieldInfo { name: "sizes_y", type_name: "Vec<i64>", description: "Positive integer sizes for each element of Y" },
            FieldInfo { name: "bound", type_name: "i64", description: "Target sum B for each triple" },
        ],
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Numerical3DimensionalMatching {
    sizes_w: Vec<i64>,
    sizes_x: Vec<i64>,
    sizes_y: Vec<i64>,
    bound: i64,
}

impl Numerical3DimensionalMatching {
    fn validate_inputs(
        sizes_w: &[i64],
        sizes_x: &[i64],
        sizes_y: &[i64],
        bound: i64,
    ) -> Result<(), crate::registry::ConstructionError> {
        let m = sizes_w.len();
        if m == 0 {
            return Err(
                "Numerical3DimensionalMatching requires at least one element per set".into(),
            );
        }
        if sizes_x.len() != m || sizes_y.len() != m {
            return Err(
                "Numerical3DimensionalMatching requires all three sets to have the same size"
                    .into(),
            );
        }
        if bound == 0 {
            return Err("Numerical3DimensionalMatching requires a positive bound"
                .to_string()
                .into());
        }

        for &size in sizes_w.iter().chain(sizes_x.iter()).chain(sizes_y.iter()) {
            if size == 0 {
                return Err("All sizes must be positive (> 0)".to_string().into());
            }
            let four_times_size = size
                .checked_mul(4)
                .ok_or("four times a size exceeds i64 range")?;
            let two_times_size = size
                .checked_mul(2)
                .ok_or("two times a size exceeds i64 range")?;
            if !(four_times_size > bound && two_times_size < bound) {
                return Err("Every size must lie strictly between B/4 and B/2"
                    .to_string()
                    .into());
            }
        }

        let total_sum = sizes_w
            .iter()
            .chain(sizes_x.iter())
            .chain(sizes_y.iter())
            .try_fold(0_i64, |total, &size| total.checked_add(size))
            .ok_or("total size sum exceeds i64 range")?;
        let group_count = i64::try_from(m).map_err(|_| "group count exceeds i64 range")?;
        let expected_sum = bound
            .checked_mul(group_count)
            .ok_or("m * bound exceeds i64 range")?;
        if total_sum != expected_sum {
            return Err("Total sum of all sizes must equal m * bound"
                .to_string()
                .into());
        }
        Ok(())
    }

    pub fn try_new(
        sizes_w: Vec<i64>,
        sizes_x: Vec<i64>,
        sizes_y: Vec<i64>,
        bound: i64,
    ) -> Result<Self, crate::registry::ConstructionError> {
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
    pub fn new(sizes_w: Vec<i64>, sizes_x: Vec<i64>, sizes_y: Vec<i64>, bound: i64) -> Self {
        Self::try_new(sizes_w, sizes_x, sizes_y, bound)
            .unwrap_or_else(|message| panic!("{message}"))
    }

    pub fn sizes_w(&self) -> &[i64] {
        &self.sizes_w
    }

    pub fn sizes_x(&self) -> &[i64] {
        &self.sizes_x
    }

    pub fn sizes_y(&self) -> &[i64] {
        &self.sizes_y
    }

    pub fn bound(&self) -> i64 {
        self.bound
    }

    pub fn num_groups(&self) -> usize {
        self.sizes_w.len()
    }
}

#[derive(Deserialize)]
struct Numerical3DimensionalMatchingData {
    sizes_w: Vec<i64>,
    sizes_x: Vec<i64>,
    sizes_y: Vec<i64>,
    bound: i64,
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
    type Solution = Vec<usize>;
    type Value = Or;

    crate::problem_parameters![("bound", bound), ("num_groups", num_groups),];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn evaluate(&self, config: &Self::Solution) -> Result<Or, crate::traits::EvaluationError> {
        Ok({
            Or({
                let m = self.num_groups();
                if config.len() != 2 * m {
                    return Err(crate::traits::EvaluationError::InvalidConfiguration(
                        "matching permutation length does not match the instance".into(),
                    ));
                }

                if config.iter().any(|&index| index >= m) {
                    return Err(crate::traits::EvaluationError::InvalidConfiguration(
                        "matching permutation contains an out-of-range index".into(),
                    ));
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
                        return Ok(Or(false));
                    }
                    if x_used[x_perm[i]] || y_used[y_perm[i]] {
                        return Ok(Or(false));
                    }
                    x_used[x_perm[i]] = true;
                    y_used[y_perm[i]] = true;
                }

                // Check that each triple sums to B
                for i in 0..m {
                    let sum = self.sizes_w[i]
                        .checked_add(self.sizes_x[x_perm[i]])
                        .and_then(|sum| sum.checked_add(self.sizes_y[y_perm[i]]))
                        .ok_or_else(|| {
                            crate::traits::EvaluationError::IntegerOverflow(
                                "summing numerical three-dimensional matching triple".into(),
                            )
                        })?;
                    if sum != self.bound {
                        return Ok(Or(false));
                    }
                }
                true
            })
        })
    }
}

impl crate::solvers::BruteForceProblem for Numerical3DimensionalMatching {
    fn dimensions(&self) -> Vec<usize> {
        vec![self.num_groups(); 2 * self.num_groups()]
    }
}

crate::declare_variants! {
    default Numerical3DimensionalMatching => "num_groups^(2 * num_groups)",
}

crate::register_brute_force! {
    Numerical3DimensionalMatching,
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
        optimal_config: serde_json::json!(vec![0, 1, 1, 0]),
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/numerical_3_dimensional_matching.rs"]
mod tests;
