//! Sum of Squares Partition problem implementation.
//!
//! Given a finite set of positive integers and K groups, find a partition
//! into K groups that minimizes the sum of squared group sums.
//! NP-hard in the strong sense (Garey & Johnson, SP19).

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "SumOfSquaresPartition",
        display_name: "Sum of Squares Partition",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Partition positive integers into K groups minimizing the sum of squared group sums",
        fields: &[
            FieldInfo { name: "sizes", type_name: "Vec<i64>", description: "Positive integer size s(a) for each element a in A" },
            FieldInfo { name: "num_groups", type_name: "usize", description: "Number of groups K in the partition" },
        ],
    }
}

/// The Sum of Squares Partition problem (Garey & Johnson SP19).
///
/// Given a finite set `A` with sizes `s(a) ∈ Z⁺` for each `a ∈ A`
/// and a positive integer `K ≤ |A|` (number of groups), find a
/// partition of `A` into `K` disjoint sets `A_1, ..., A_K` that
/// minimizes:
///
/// `∑_{i=1}^{K} (∑_{a ∈ A_i} s(a))²`
///
/// # Representation
///
/// Each element has a variable in `{0, ..., K-1}` representing its
/// group assignment. The value is the sum of squared group sums.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::SumOfSquaresPartition;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // 6 elements with sizes [5, 3, 8, 2, 7, 1], K=3 groups
/// let problem = SumOfSquaresPartition::new(vec![5, 3, 8, 2, 7, 1], 3);
/// let solver = BruteForce::new();
/// let solution = solver.find_witness(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct SumOfSquaresPartition {
    /// Positive integer sizes for each element.
    sizes: Vec<i64>,
    /// Number of groups K.
    num_groups: usize,
}

impl SumOfSquaresPartition {
    fn validate_inputs(sizes: &[i64], num_groups: usize) -> Result<(), String> {
        if sizes.iter().any(|&size| size <= 0) {
            return Err("All sizes must be positive (> 0)".to_string());
        }
        if num_groups == 0 {
            return Err("Number of groups must be positive".to_string());
        }
        if num_groups > sizes.len() {
            return Err("Number of groups must not exceed number of elements".to_string());
        }
        Ok(())
    }

    /// Create a new SumOfSquaresPartition instance, returning validation errors.
    pub fn try_new(sizes: Vec<i64>, num_groups: usize) -> Result<Self, String> {
        Self::validate_inputs(&sizes, num_groups)?;
        Ok(Self { sizes, num_groups })
    }

    /// Create a new SumOfSquaresPartition instance.
    ///
    /// # Panics
    ///
    /// Panics if any size is not positive (must be > 0), if `num_groups` is 0,
    /// or if `num_groups` exceeds the number of elements.
    pub fn new(sizes: Vec<i64>, num_groups: usize) -> Self {
        Self::try_new(sizes, num_groups).unwrap_or_else(|message| panic!("{message}"))
    }

    /// Returns the element sizes.
    pub fn sizes(&self) -> &[i64] {
        &self.sizes
    }

    /// Returns the number of groups K.
    pub fn num_groups(&self) -> usize {
        self.num_groups
    }

    /// Returns the number of elements |A|.
    pub fn num_elements(&self) -> usize {
        self.sizes.len()
    }

    /// Compute the sum of squared group sums for a given configuration.
    ///
    /// Returns `None` if the configuration is invalid (wrong length or
    /// out-of-range group index), or if arithmetic overflows `i64`.
    pub fn sum_of_squares(&self, config: &[usize]) -> Option<i64> {
        if config.len() != self.sizes.len() {
            return None;
        }
        let mut group_sums = vec![0i128; self.num_groups];
        for (i, &g) in config.iter().enumerate() {
            if g >= self.num_groups {
                return None;
            }
            group_sums[g] = group_sums[g].checked_add(i128::from(self.sizes[i]))?;
        }
        group_sums
            .into_iter()
            .try_fold(0i128, |total, group_sum| {
                let square = group_sum.checked_mul(group_sum)?;
                total.checked_add(square)
            })
            .and_then(|total| i64::try_from(total).ok())
    }
}

#[derive(Deserialize)]
struct SumOfSquaresPartitionData {
    sizes: Vec<i64>,
    num_groups: usize,
}

impl<'de> Deserialize<'de> for SumOfSquaresPartition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = SumOfSquaresPartitionData::deserialize(deserializer)?;
        Self::try_new(data.sizes, data.num_groups).map_err(D::Error::custom)
    }
}

impl Problem for SumOfSquaresPartition {
    const NAME: &'static str = "SumOfSquaresPartition";
    type Value = Min<i64>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![self.num_groups; self.sizes.len()]
    }

    fn evaluate(&self, config: &[usize]) -> Min<i64> {
        Min(self.sum_of_squares(config))
    }
}

crate::declare_variants! {
    default SumOfSquaresPartition => "num_groups^num_elements",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "sum_of_squares_partition",
        // sizes=[5,3,8,2,7,1], K=3
        // Optimal: groups {8,1},{5,2},{3,7} -> sums 9,7,10 -> 81+49+100=230
        instance: Box::new(SumOfSquaresPartition::new(vec![5, 3, 8, 2, 7, 1], 3)),
        optimal_config: vec![1, 2, 0, 1, 2, 0],
        optimal_value: serde_json::json!(230),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/sum_of_squares_partition.rs"]
mod tests;
