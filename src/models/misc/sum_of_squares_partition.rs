//! Sum of Squares Partition problem implementation.
//!
//! Given a finite set of positive integers, K groups, and a bound J,
//! determine whether the set can be partitioned into K groups such that
//! the sum of squared group sums is at most J.
//! NP-complete in the strong sense (Garey & Johnson, SP19).

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{Problem, SatisfactionProblem};
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "SumOfSquaresPartition",
        display_name: "Sum of Squares Partition",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Partition positive integers into K groups minimizing sum of squared group sums, subject to bound J",
        fields: &[
            FieldInfo { name: "sizes", type_name: "Vec<i64>", description: "Positive integer size s(a) for each element a in A" },
            FieldInfo { name: "num_groups", type_name: "usize", description: "Number of groups K in the partition" },
            FieldInfo { name: "bound", type_name: "i64", description: "Upper bound J on the sum of squared group sums" },
        ],
    }
}

/// The Sum of Squares Partition problem (Garey & Johnson SP19).
///
/// Given a finite set `A` with sizes `s(a) ∈ Z⁺` for each `a ∈ A`,
/// a positive integer `K ≤ |A|` (number of groups), and a positive
/// integer `J` (bound), determine whether `A` can be partitioned into
/// `K` disjoint sets `A_1, ..., A_K` such that:
///
/// `∑_{i=1}^{K} (∑_{a ∈ A_i} s(a))² ≤ J`
///
/// # Representation
///
/// Each element has a variable in `{0, ..., K-1}` representing its
/// group assignment. A configuration is satisfying if the sum of
/// squared group sums does not exceed `J`.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::SumOfSquaresPartition;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // 6 elements with sizes [5, 3, 8, 2, 7, 1], K=3 groups, bound J=240
/// let problem = SumOfSquaresPartition::new(vec![5, 3, 8, 2, 7, 1], 3, 240);
/// let solver = BruteForce::new();
/// let solution = solver.find_satisfying(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct SumOfSquaresPartition {
    /// Positive integer sizes for each element.
    sizes: Vec<i64>,
    /// Number of groups K.
    num_groups: usize,
    /// Upper bound J on the sum of squared group sums.
    bound: i64,
}

impl SumOfSquaresPartition {
    fn validate_inputs(sizes: &[i64], num_groups: usize, bound: i64) -> Result<(), String> {
        if sizes.iter().any(|&size| size <= 0) {
            return Err("All sizes must be positive (> 0)".to_string());
        }
        if num_groups == 0 {
            return Err("Number of groups must be positive".to_string());
        }
        if num_groups > sizes.len() {
            return Err("Number of groups must not exceed number of elements".to_string());
        }
        if bound < 0 {
            return Err("Bound must be nonnegative".to_string());
        }
        Ok(())
    }

    fn try_new(sizes: Vec<i64>, num_groups: usize, bound: i64) -> Result<Self, String> {
        Self::validate_inputs(&sizes, num_groups, bound)?;
        Ok(Self {
            sizes,
            num_groups,
            bound,
        })
    }

    /// Create a new SumOfSquaresPartition instance.
    ///
    /// # Panics
    ///
    /// Panics if any size is not positive (must be > 0), if `num_groups` is 0,
    /// if `num_groups` exceeds the number of elements, or if `bound` is negative.
    pub fn new(sizes: Vec<i64>, num_groups: usize, bound: i64) -> Self {
        Self::try_new(sizes, num_groups, bound).unwrap_or_else(|message| panic!("{message}"))
    }

    /// Returns the element sizes.
    pub fn sizes(&self) -> &[i64] {
        &self.sizes
    }

    /// Returns the number of groups K.
    pub fn num_groups(&self) -> usize {
        self.num_groups
    }

    /// Returns the bound J.
    pub fn bound(&self) -> i64 {
        self.bound
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
    bound: i64,
}

impl<'de> Deserialize<'de> for SumOfSquaresPartition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = SumOfSquaresPartitionData::deserialize(deserializer)?;
        Self::try_new(data.sizes, data.num_groups, data.bound).map_err(D::Error::custom)
    }
}

impl Problem for SumOfSquaresPartition {
    const NAME: &'static str = "SumOfSquaresPartition";
    type Metric = bool;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![self.num_groups; self.sizes.len()]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        match self.sum_of_squares(config) {
            Some(sos) => sos <= self.bound,
            None => false,
        }
    }
}

impl SatisfactionProblem for SumOfSquaresPartition {}

crate::declare_variants! {
    default sat SumOfSquaresPartition => "num_groups^num_elements",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "sum_of_squares_partition",
        build: || {
            // sizes=[5,3,8,2,7,1], K=3, J=240
            // Satisfying: groups {8,1},{5,2},{3,7} -> sums 9,7,10 -> 81+49+100=230 <= 240
            // Config: a0=5->group1, a1=3->group2, a2=8->group0, a3=2->group1, a4=7->group2, a5=1->group0
            let problem = SumOfSquaresPartition::new(vec![5, 3, 8, 2, 7, 1], 3, 240);
            crate::example_db::specs::satisfaction_example(problem, vec![vec![1, 2, 0, 1, 2, 0]])
        },
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/sum_of_squares_partition.rs"]
mod tests;
