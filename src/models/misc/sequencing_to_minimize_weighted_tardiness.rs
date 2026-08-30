//! Sequencing to Minimize Weighted Tardiness problem implementation.
//!
//! A classical NP-complete single-machine scheduling problem (SS5 from
//! Garey & Johnson, 1979) asking whether there exists a job order whose
//! total weighted tardiness is at most a given bound.
//! Corresponds to scheduling notation `1 || sum w_j T_j`.

use crate::registry::{CreateSpec, ProblemSchemaEntry};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "SequencingToMinimizeWeightedTardiness",
        display_name: "Sequencing to Minimize Weighted Tardiness",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Misc,
        module_path: module_path!(),
        description: "Schedule jobs on one machine so total weighted tardiness is at most K",
        fields: SequencingToMinimizeWeightedTardinessCreateSpec::FIELDS,
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
/// use problemreductions::{BruteForce, Problem};
///
/// let problem = SequencingToMinimizeWeightedTardiness::new(
///     vec![3, 4, 2, 5, 3],
///     vec![2, 3, 1, 4, 2],
///     vec![5, 8, 4, 15, 10],
///     13,
/// );
///
/// let solver = BruteForce::new();
/// assert!(solver.solve(&problem).unwrap().is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequencingToMinimizeWeightedTardiness {
    lengths: Vec<i64>,
    weights: Vec<i64>,
    deadlines: Vec<i64>,
    bound: i64,
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct SequencingToMinimizeWeightedTardinessCreateSpec {
    /// Processing times for each job.
    lengths: Vec<i64>,
    /// Tardiness weights for each job.
    weights: Vec<i64>,
    /// Deadlines for each job.
    deadlines: Vec<i64>,
    /// Upper bound on total weighted tardiness.
    bound: i64,
}
impl TryFrom<SequencingToMinimizeWeightedTardinessCreateSpec>
    for SequencingToMinimizeWeightedTardiness
{
    type Error = crate::registry::ConstructionError;
    fn try_from(
        spec: SequencingToMinimizeWeightedTardinessCreateSpec,
    ) -> Result<Self, Self::Error> {
        if spec.lengths.len() != spec.weights.len() {
            return Err("weights length must equal lengths length"
                .to_string()
                .into());
        }
        if spec.lengths.len() != spec.deadlines.len() {
            return Err("deadlines length must equal lengths length"
                .to_string()
                .into());
        }
        Ok(Self::new(
            spec.lengths,
            spec.weights,
            spec.deadlines,
            spec.bound,
        ))
    }
}

impl SequencingToMinimizeWeightedTardiness {
    /// Create a new weighted tardiness scheduling instance.
    ///
    /// # Panics
    ///
    /// Panics if the input vectors do not have the same length.
    pub fn new(lengths: Vec<i64>, weights: Vec<i64>, deadlines: Vec<i64>, bound: i64) -> Self {
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
        assert!(
            lengths.iter().all(|&length| length >= 0),
            "task lengths must be nonnegative"
        );
        assert!(
            weights.iter().all(|&weight| weight >= 0),
            "task weights must be nonnegative"
        );
        assert!(
            deadlines.iter().all(|&deadline| deadline >= 0),
            "deadlines must be nonnegative"
        );
        assert!(bound >= 0, "bound must be nonnegative");
        Self {
            lengths,
            weights,
            deadlines,
            bound,
        }
    }

    /// Returns the job lengths.
    pub fn lengths(&self) -> &[i64] {
        &self.lengths
    }

    /// Returns the tardiness weights.
    pub fn weights(&self) -> &[i64] {
        &self.weights
    }

    /// Returns the deadlines.
    pub fn deadlines(&self) -> &[i64] {
        &self.deadlines
    }

    /// Returns the weighted tardiness bound.
    pub fn bound(&self) -> i64 {
        self.bound
    }

    /// Returns the number of jobs.
    pub fn num_tasks(&self) -> usize {
        self.lengths.len()
    }

    fn decode_schedule(&self, config: &[usize]) -> Option<Vec<usize>> {
        super::decode_permutation(config, self.num_tasks())
    }

    fn schedule_weighted_tardiness(
        &self,
        schedule: &[usize],
    ) -> Result<i64, crate::traits::EvaluationError> {
        let mut completion_time = 0i64;
        let mut total = 0i64;
        for &job in schedule {
            completion_time = completion_time
                .checked_add(self.lengths[job])
                .ok_or_else(|| {
                    crate::traits::EvaluationError::IntegerOverflow(
                        "summing weighted-tardiness completion times".to_string(),
                    )
                })?;
            let tardiness = completion_time
                .checked_sub(self.deadlines[job])
                .ok_or_else(|| {
                    crate::traits::EvaluationError::IntegerOverflow(
                        "computing job tardiness".to_string(),
                    )
                })?
                .max(0);
            let weighted_tardiness = tardiness.checked_mul(self.weights[job]).ok_or_else(|| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "multiplying tardiness by job weight".to_string(),
                )
            })?;
            total = total.checked_add(weighted_tardiness).ok_or_else(|| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "summing weighted job tardiness".to_string(),
                )
            })?;
        }
        Ok(total)
    }

    /// Compute the total weighted tardiness of a Lehmer-encoded schedule.
    ///
    /// Returns `Ok(None)` if the configuration is not a valid Lehmer code.
    pub fn total_weighted_tardiness(
        &self,
        config: &[usize],
    ) -> Result<Option<i64>, crate::traits::EvaluationError> {
        let Some(schedule) = self.decode_schedule(config) else {
            return Ok(None);
        };
        Ok(Some(self.schedule_weighted_tardiness(&schedule)?))
    }
}

impl Problem for SequencingToMinimizeWeightedTardiness {
    const NAME: &'static str = "SequencingToMinimizeWeightedTardiness";
    type Solution = Vec<usize>;
    type Value = crate::types::Or;

    crate::problem_parameters![("num_tasks", num_tasks),];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<crate::types::Or, crate::traits::EvaluationError> {
        let n = self.num_tasks();
        if config.len() != n {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "schedule length does not match the tasks".into(),
            ));
        }
        if config.iter().any(|&task| task >= n) {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "schedule contains an out-of-range task".into(),
            ));
        }
        Ok({
            crate::types::Or({
                self.total_weighted_tardiness(config)?
                    .is_some_and(|total| total <= self.bound)
            })
        })
    }
}

impl crate::solvers::BruteForceProblem for SequencingToMinimizeWeightedTardiness {
    fn dimensions(&self) -> Vec<usize> {
        super::lehmer_dims(self.num_tasks())
    }
}

crate::declare_variants! {
    default SequencingToMinimizeWeightedTardiness => "factorial(num_tasks)" create SequencingToMinimizeWeightedTardinessCreateSpec,
}

crate::register_brute_force! {
    SequencingToMinimizeWeightedTardiness decode |problem: &SequencingToMinimizeWeightedTardiness, indices: Vec<usize>| super::decode_lehmer(&indices, problem.num_tasks()).expect("enumerated Lehmer digits are valid"),
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
        optimal_config: serde_json::json!(vec![0, 1, 4, 3, 2]),
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/sequencing_to_minimize_weighted_tardiness.rs"]
mod tests;
