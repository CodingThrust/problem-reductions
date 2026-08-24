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
/// The configuration is a flat array of `n * m` values.
/// `config[i * n .. (i + 1) * n]` gives the permutation of jobs on machine `i`
/// (direct job indices, not Lehmer code). A segment is valid iff it is a
/// permutation of `0..n`. Invalid configs return `Min(None)`.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::OpenShopScheduling;
/// use problemreductions::{Problem, Solver, BruteForce};
/// use problemreductions::types::Min;
///
/// // 2 machines, 2 jobs
/// let p = vec![vec![1, 2], vec![2, 1]];
/// let problem = OpenShopScheduling::new(2, p);
/// let solver = BruteForce::new();
/// let value = Solver::solve(&solver, &problem).unwrap();
/// assert_eq!(value, Min(Some(3)));
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
        for (job, times) in value.processing_times.iter().enumerate() {
            if times.len() != value.num_machines {
                return Err(format!(
                    "processing_times[{job}] has {} entries, expected {}",
                    times.len(),
                    value.num_machines
                )
                .into());
            }
            if times.iter().any(|&time| time < 0) {
                return Err(format!("processing_times[{job}] contains a negative duration").into());
            }
        }
        Ok(Self {
            num_machines: value.num_machines,
            processing_times: value.processing_times,
        })
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
        for (job, times) in spec.processing_times.iter().enumerate() {
            if times.len() != spec.num_processors {
                return Err(format!(
                    "processing_times[{job}] has {} entries, expected {}",
                    times.len(),
                    spec.num_processors
                )
                .into());
            }
            if times.iter().any(|&time| time < 0) {
                return Err(format!("processing_times[{job}] contains a negative duration").into());
            }
        }
        Ok(Self::new(spec.num_processors, spec.processing_times))
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
    /// Panics if any job does not have exactly `num_machines` processing times.
    pub fn new(num_machines: usize, processing_times: Vec<Vec<i64>>) -> Self {
        for (j, times) in processing_times.iter().enumerate() {
            assert_eq!(
                times.len(),
                num_machines,
                "Job {} has {} processing times, expected {}",
                j,
                times.len(),
                num_machines
            );
            assert!(
                times.iter().all(|&time| time >= 0),
                "Job {j} has a negative processing time"
            );
        }
        Self {
            num_machines,
            processing_times,
        }
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

    /// Decode the per-machine job orderings from a config.
    ///
    /// Returns `None` if the config length is wrong or any segment is not a
    /// valid permutation of `0..n`.
    pub fn decode_orders(&self, config: &[usize]) -> Option<Vec<Vec<usize>>> {
        let n = self.num_jobs();
        let m = self.num_machines;
        if config.len() != n * m {
            return None;
        }
        let mut orders = Vec::with_capacity(m);
        for i in 0..m {
            let seg = &config[i * n..(i + 1) * n];
            // Validate that seg is a permutation of 0..n
            let mut seen = vec![false; n];
            for &job in seg {
                if job >= n || seen[job] {
                    return None;
                }
                seen[job] = true;
            }
            orders.push(seg.to_vec());
        }
        Some(orders)
    }

    /// Compute the makespan from a set of per-machine job orderings.
    ///
    /// Uses a greedy simulation: at each step, among all machines whose next
    /// scheduled job can start (both machine and job are free), schedule the
    /// one with the earliest available start time.
    pub fn compute_makespan(
        &self,
        orders: &[Vec<usize>],
    ) -> Result<i64, crate::traits::EvaluationError> {
        let n = self.num_jobs();
        let m = self.num_machines;

        if n == 0 || m == 0 {
            return Ok(0);
        }

        // `machine_avail[i]` = next time machine i is free.
        let mut machine_avail = vec![0_i64; m];
        // `job_avail[j]` = next time job j is free (all its currently scheduled
        // tasks have finished).
        let mut job_avail = vec![0_i64; n];
        // Pointer to next unscheduled position in each machine's ordering.
        let mut next_on_machine = vec![0usize; m];

        let total_tasks = n * m;
        let mut scheduled = 0;

        while scheduled < total_tasks {
            // Find the (machine, earliest start time) among all machines that
            // still have unscheduled tasks.
            let mut best_start = i64::MAX;
            let mut best_machine = usize::MAX;

            for i in 0..m {
                if next_on_machine[i] < n {
                    let j = orders[i][next_on_machine[i]];
                    let start = machine_avail[i].max(job_avail[j]);
                    // Tie-break by machine index to make the result deterministic.
                    if start < best_start || (start == best_start && i < best_machine) {
                        best_start = start;
                        best_machine = i;
                    }
                }
            }

            // Schedule the chosen task.
            let i = best_machine;
            let j = orders[i][next_on_machine[i]];
            let start = machine_avail[i].max(job_avail[j]);
            let finish = start
                .checked_add(self.processing_times[j][i])
                .ok_or_else(|| {
                    crate::traits::EvaluationError::IntegerOverflow(
                        "computing an open-shop task completion time".to_string(),
                    )
                })?;
            machine_avail[i] = finish;
            job_avail[j] = finish;
            next_on_machine[i] += 1;
            scheduled += 1;
        }

        Ok(machine_avail
            .iter()
            .copied()
            .max()
            .unwrap_or(0)
            .max(job_avail.iter().copied().max().unwrap_or(0)))
    }
}

impl Problem for OpenShopScheduling {
    const NAME: &'static str = "OpenShopScheduling";
    type Value = Min<i64>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        let n = self.num_jobs();
        let m = self.num_machines;
        vec![n; n * m]
    }

    fn evaluate(&self, config: &[usize]) -> Result<Min<i64>, crate::traits::EvaluationError> {
        match self.decode_orders(config) {
            Some(orders) => Ok(Min(Some(self.compute_makespan(&orders)?))),
            None => Ok(Min(None)),
        }
    }
}

crate::declare_variants! {
    default OpenShopScheduling => "factorial(num_jobs)^num_machines" create OpenShopSchedulingCreateSpec,
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
    // Optimal machine orderings (0-indexed jobs):
    //   M1: [J1, J2, J3, J4] = [0, 1, 2, 3]
    //   M2: [J2, J1, J4, J3] = [1, 0, 3, 2]
    //   M3: [J3, J4, J1, J2] = [2, 3, 0, 1]
    //
    // config = [M1 order | M2 order | M3 order]
    //        = [0, 1, 2, 3, 1, 0, 3, 2, 2, 3, 0, 1]
    //
    // Resulting schedule:
    //   J1: M1=[0,3), M2=[7,8), M3=[1,3)  — job non-overlap: [0,3),[1,3) overlap!
    //   Actually use simulation to verify:
    //   Step 1: best start = M1(J1:0), M2(J2:0), M3(J3:0) → M1 ties with M2,M3; pick M1
    //           J1 on M1: [0,3)
    //   ... (simulation produces makespan=8)
    //
    // 224 out of 13824 orderings achieve the optimal makespan of 8.
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "open_shop_scheduling",
        instance: Box::new(OpenShopScheduling::new(
            3,
            vec![vec![3, 1, 2], vec![2, 3, 1], vec![1, 2, 3], vec![2, 2, 1]],
        )),
        optimal_config: vec![0, 1, 2, 3, 1, 0, 3, 2, 2, 3, 0, 1],
        optimal_value: serde_json::json!(8),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/open_shop_scheduling.rs"]
mod tests;
