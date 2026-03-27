//! Sequencing to Minimize Weighted Tardiness problem implementation.
//!
//! A classical NP-complete single-machine scheduling problem (SS5 from
//! Garey & Johnson, 1979) asking whether there exists a job order whose
//! total weighted tardiness is at most a given bound.
//! Corresponds to scheduling notation `1 || sum w_j T_j`.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "SequencingToMinimizeWeightedTardiness",
        display_name: "Sequencing to Minimize Weighted Tardiness",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Schedule jobs on one machine so total weighted tardiness is at most K",
        fields: &[
            FieldInfo { name: "lengths", type_name: "Vec<u64>", description: "Processing times l_j for each job" },
            FieldInfo { name: "weights", type_name: "Vec<u64>", description: "Tardiness weights w_j for each job" },
            FieldInfo { name: "deadlines", type_name: "Vec<u64>", description: "Deadlines d_j for each job" },
            FieldInfo { name: "bound", type_name: "u64", description: "Upper bound K on total weighted tardiness" },
        ],
    }
}

/// Sequencing to Minimize Weighted Tardiness.
///
/// Given jobs with processing times `l_j`, weights `w_j`, deadlines `d_j`,
/// and a bound `K`, determine whether there exists a permutation schedule on a
/// single machine whose total weighted tardiness
/// `sum_j w_j * max(0, C_j - d_j)` is at most `K`, where `C_j` is the
/// completion time of job `j`.
///
/// # Representation
///
/// Configurations use Lehmer code to encode permutations of the jobs.
/// Decoding yields the job order processed by the single machine.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::SequencingToMinimizeWeightedTardiness;
/// use problemreductions::{BruteForce, Problem, Solver};
///
/// let problem = SequencingToMinimizeWeightedTardiness::new(
///     vec![3, 4, 2, 5, 3],
///     vec![2, 3, 1, 4, 2],
///     vec![5, 8, 4, 15, 10],
///     13,
/// );
///
/// let solver = BruteForce::new();
/// assert!(solver.find_witness(&problem).is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequencingToMinimizeWeightedTardiness {
    lengths: Vec<u64>,
    weights: Vec<u64>,
    deadlines: Vec<u64>,
    bound: u64,
}

impl SequencingToMinimizeWeightedTardiness {
    /// Create a new weighted tardiness scheduling instance.
    ///
    /// # Panics
    ///
    /// Panics if the input vectors do not have the same length.
    pub fn new(lengths: Vec<u64>, weights: Vec<u64>, deadlines: Vec<u64>, bound: u64) -> Self {
        assert_eq!(
            lengths.len(),
            weights.len(),
            "weights length must equal lengths length"
        );
        assert_eq!(
            lengths.len(),
            deadlines.len(),
            "deadlines length must equal lengths length"
        );
        Self {
            lengths,
            weights,
            deadlines,
            bound,
        }
    }

    /// Returns the job lengths.
    pub fn lengths(&self) -> &[u64] {
        &self.lengths
    }

    /// Returns the tardiness weights.
    pub fn weights(&self) -> &[u64] {
        &self.weights
    }

    /// Returns the deadlines.
    pub fn deadlines(&self) -> &[u64] {
        &self.deadlines
    }

    /// Returns the weighted tardiness bound.
    pub fn bound(&self) -> u64 {
        self.bound
    }

    /// Returns the number of jobs.
    pub fn num_tasks(&self) -> usize {
        self.lengths.len()
    }

    fn decode_schedule(&self, config: &[usize]) -> Option<Vec<usize>> {
        super::decode_lehmer(config, self.num_tasks())
    }

    fn schedule_weighted_tardiness(&self, schedule: &[usize]) -> Option<u64> {
        let mut completion_time = 0u128;
        let mut total = 0u128;
        for &job in schedule {
            completion_time += u128::from(self.lengths[job]);
            let tardiness = completion_time.saturating_sub(u128::from(self.deadlines[job]));
            total += tardiness * u128::from(self.weights[job]);
        }
        u64::try_from(total).ok()
    }

    /// Compute the total weighted tardiness of a Lehmer-encoded schedule.
    ///
    /// Returns `None` if the configuration is not a valid Lehmer code or if
    /// the accumulated objective does not fit in `u64`.
    pub fn total_weighted_tardiness(&self, config: &[usize]) -> Option<u64> {
        let schedule = self.decode_schedule(config)?;
        self.schedule_weighted_tardiness(&schedule)
    }
}

impl Problem for SequencingToMinimizeWeightedTardiness {
    const NAME: &'static str = "SequencingToMinimizeWeightedTardiness";
    type Value = crate::types::Or;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        super::lehmer_dims(self.num_tasks())
    }

    fn evaluate(&self, config: &[usize]) -> crate::types::Or {
        crate::types::Or({
            self.total_weighted_tardiness(config)
                .is_some_and(|total| total <= self.bound)
        })
    }
}

crate::declare_variants! {
    default SequencingToMinimizeWeightedTardiness => "factorial(num_tasks)",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "sequencing_to_minimize_weighted_tardiness",
        instance: Box::new(SequencingToMinimizeWeightedTardiness::new(
            vec![3, 4, 2, 5, 3],
            vec![2, 3, 1, 4, 2],
            vec![5, 8, 4, 15, 10],
            13,
        )),
        optimal_config: vec![0, 0, 2, 1, 0],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/sequencing_to_minimize_weighted_tardiness.rs"]
mod tests;
