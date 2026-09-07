//! Open Shop Scheduling problem implementation.
//!
//! Given `m` machines and a set of `n` jobs, each job consisting of one task
//! per machine (the task order for each job is free), find a schedule that
//! minimizes the makespan (completion time of the last task) while respecting
//! both machine capacity (one job at a time per machine) and job capacity
//! (each job uses at most one machine at a time) constraints.

use crate::registry::{CreateSpec, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "OpenShopScheduling",
        display_name: "Open Shop Scheduling",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Misc,
        module_path: module_path!(),
        description: "Minimize the makespan of an open-shop schedule",
        fields: OpenShopSchedulingCreateSpec::FIELDS,
    }
}

/// The Open Shop Scheduling problem.
///
/// Given `m` machines and `n` jobs, where job `j` has one task on each machine
/// `i` with processing time `p[j][i]`, find a non-preemptive schedule that
/// minimizes the makespan. Unlike flow-shop or job-shop scheduling, there is no
/// prescribed order for the tasks of a given job — each job's tasks may be
/// processed on the machines in any order.
///
/// # Constraints
///
/// 1. **Machine constraint:** Each machine processes at most one job at a time.
/// 2. **Job constraint:** Each job occupies at most one machine at a time.
///
/// # Configuration Encoding
///
/// The configuration is a flat array of `n * m` non-negative start times in
/// job-major order: `config[j * m + i]` is the start time of job `j` on
/// machine `i`. A configuration is valid exactly when operations of the same
/// job and operations on the same machine do not overlap.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::OpenShopScheduling;
/// use problemreductions::{Problem, BruteForce};
/// use problemreductions::types::Min;
///
/// // 2 machines, 2 jobs
/// let p = vec![vec![1, 2], vec![2, 1]];
/// let problem = OpenShopScheduling::new(2, p);
/// let solver = BruteForce::new();
/// let solution = solver.solve(&problem).unwrap().unwrap();
/// assert_eq!(problem.evaluate(&solution).unwrap(), Min(Some(3)));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "OpenShopSchedulingSerde")]
pub struct OpenShopScheduling {
    /// Number of machines m.
    num_machines: usize,
    /// Processing time matrix: `processing_times[j][i]` is the time to process
    /// job `j` on machine `i`. Dimensions: n jobs × m machines.
    processing_times: Vec<Vec<i64>>,
}

#[derive(Deserialize)]
struct OpenShopSchedulingSerde {
    num_machines: usize,
    processing_times: Vec<Vec<i64>>,
}

impl TryFrom<OpenShopSchedulingSerde> for OpenShopScheduling {
    type Error = crate::registry::ConstructionError;

    fn try_from(value: OpenShopSchedulingSerde) -> Result<Self, Self::Error> {
        Self::try_new(value.num_machines, value.processing_times)
    }
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct OpenShopSchedulingCreateSpec {
    /// Number of machines m.
    num_processors: usize,
    /// Processing time of each job on each machine (n x m).
    processing_times: Vec<Vec<i64>>,
}

impl TryFrom<OpenShopSchedulingCreateSpec> for OpenShopScheduling {
    type Error = crate::registry::ConstructionError;

    fn try_from(spec: OpenShopSchedulingCreateSpec) -> Result<Self, Self::Error> {
        Self::try_new(spec.num_processors, spec.processing_times)
    }
}

impl OpenShopScheduling {
    /// Create a new Open Shop Scheduling instance.
    ///
    /// # Arguments
    /// * `num_machines` - Number of machines m
    /// * `processing_times` - `processing_times[j][i]` = processing time of job j on machine i.
    ///   Each inner Vec must have length `num_machines`.
    ///
    /// # Panics
    /// Panics if the processing matrix or its schedule horizon is invalid.
    pub fn new(num_machines: usize, processing_times: Vec<Vec<i64>>) -> Self {
        Self::try_new(num_machines, processing_times)
            .expect("invalid open-shop scheduling instance")
    }

    /// Construct an instance, validating dimensions, durations, and the horizon.
    pub fn try_new(
        num_machines: usize,
        processing_times: Vec<Vec<i64>>,
    ) -> Result<Self, crate::registry::ConstructionError> {
        for (job, times) in processing_times.iter().enumerate() {
            if times.len() != num_machines {
                return Err(format!(
                    "processing_times[{job}] has {} entries, expected {num_machines}",
                    times.len(),
                )
                .into());
            }
            if times.iter().any(|&time| time < 0) {
                return Err(format!("processing_times[{job}] contains a negative duration").into());
            }
        }
        processing_times
            .len()
            .checked_mul(num_machines)
            .ok_or_else(|| {
                crate::registry::ConstructionError::IntegerOverflow(
                    "operation count overflows usize".into(),
                )
            })?;
        let horizon = processing_times
            .iter()
            .flatten()
            .try_fold(0i64, |total, &time| total.checked_add(time))
            .ok_or_else(|| {
                crate::registry::ConstructionError::IntegerOverflow(
                    "schedule horizon overflows i64".into(),
                )
            })?;
        usize::try_from(horizon)
            .ok()
            .and_then(|value| value.checked_add(1))
            .ok_or_else(|| {
                crate::registry::ConstructionError::IntegerOverflow(
                    "schedule horizon domain overflows usize".into(),
                )
            })?;
        Ok(Self {
            num_machines,
            processing_times,
        })
    }

    /// Get the number of machines.
    pub fn num_machines(&self) -> usize {
        self.num_machines
    }

    /// Get the number of jobs.
    pub fn num_jobs(&self) -> usize {
        self.processing_times.len()
    }

    /// Get the processing time matrix.
    pub fn processing_times(&self) -> &[Vec<i64>] {
        &self.processing_times
    }

    /// Return the sum of all processing times, a valid serial-schedule horizon.
    pub fn schedule_horizon(&self) -> usize {
        self.processing_times
            .iter()
            .flatten()
            .try_fold(0usize, |total, &time| {
                usize::try_from(time)
                    .ok()
                    .and_then(|time| total.checked_add(time))
            })
            .expect("processing times must fit the brute-force schedule horizon")
    }

    fn finish_time(
        &self,
        config: &[usize],
        job: usize,
        machine: usize,
    ) -> Result<i64, crate::traits::EvaluationError> {
        let start = i64::try_from(config[job * self.num_machines + machine]).map_err(|_| {
            crate::traits::EvaluationError::IntegerOverflow(
                "converting an open-shop start time to i64".into(),
            )
        })?;
        start
            .checked_add(self.processing_times[job][machine])
            .ok_or_else(|| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "computing an open-shop completion time".into(),
                )
            })
    }

    fn operations_overlap(
        &self,
        config: &[usize],
        first: (usize, usize),
        second: (usize, usize),
    ) -> Result<bool, crate::traits::EvaluationError> {
        let (j1, i1) = first;
        let (j2, i2) = second;
        let s1 = i64::try_from(config[j1 * self.num_machines + i1]).map_err(|_| {
            crate::traits::EvaluationError::IntegerOverflow("converting start time to i64".into())
        })?;
        let s2 = i64::try_from(config[j2 * self.num_machines + i2]).map_err(|_| {
            crate::traits::EvaluationError::IntegerOverflow("converting start time to i64".into())
        })?;
        let f1 = self.finish_time(config, j1, i1)?;
        let f2 = self.finish_time(config, j2, i2)?;
        Ok(s1 < f2 && s2 < f1)
    }
}

impl Problem for OpenShopScheduling {
    const NAME: &'static str = "OpenShopScheduling";
    type Solution = Vec<usize>;
    type Value = Min<i64>;

    crate::problem_parameters![
        ("num_jobs", num_jobs),
        ("num_machines", num_machines),
        ("schedule_horizon", schedule_horizon),
    ];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Min<i64>, crate::traits::EvaluationError> {
        let n = self.num_jobs();
        let m = self.num_machines;
        if config.len() != n * m {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "start-time representation length does not match the instance".into(),
            ));
        }
        for machine in 0..m {
            for first in 0..n {
                for second in (first + 1)..n {
                    if self.operations_overlap(config, (first, machine), (second, machine))? {
                        return Ok(Min(None));
                    }
                }
            }
        }
        for job in 0..n {
            for first in 0..m {
                for second in (first + 1)..m {
                    if self.operations_overlap(config, (job, first), (job, second))? {
                        return Ok(Min(None));
                    }
                }
            }
        }
        let mut makespan = 0;
        for job in 0..n {
            for machine in 0..m {
                makespan = makespan.max(self.finish_time(config, job, machine)?);
            }
        }
        Ok(Min(Some(makespan)))
    }
}

impl crate::solvers::BruteForceProblem for OpenShopScheduling {
    fn dimensions(&self) -> Vec<usize> {
        let domain = self
            .schedule_horizon()
            .checked_add(1)
            .expect("schedule horizon overflow");
        vec![domain; self.num_jobs() * self.num_machines]
    }
}

crate::declare_variants! {
    default OpenShopScheduling => "(schedule_horizon + 1)^(num_jobs * num_machines)" create OpenShopSchedulingCreateSpec,
}

crate::register_brute_force! {
    OpenShopScheduling,
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // 4 jobs × 3 machines example from issue #506.
    // processing_times[j][i]:
    //   J1: p[0] = [3, 1, 2]
    //   J2: p[1] = [2, 3, 1]
    //   J3: p[2] = [1, 2, 3]
    //   J4: p[3] = [2, 2, 1]
    //
    // Per-machine totals: M1=8, M2=8, M3=7.  Per-job totals: J1=6, J2=6, J3=6, J4=5.
    // Lower bound: max(8, 6) = 8. True optimal makespan = 8.
    //
    // Job-major start times: J1=[0,3,4], J2=[3,0,6], J3=[5,6,0], J4=[6,4,3].
    // Each job and machine has non-overlapping operations; the last finish is 8.
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "open_shop_scheduling",
        instance: Box::new(OpenShopScheduling::new(
            3,
            vec![vec![3, 1, 2], vec![2, 3, 1], vec![1, 2, 3], vec![2, 2, 1]],
        )),
        optimal_config: serde_json::json!(vec![0, 3, 4, 3, 0, 6, 5, 6, 0, 6, 4, 3]),
        optimal_value: serde_json::json!(8),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/open_shop_scheduling.rs"]
mod tests;
