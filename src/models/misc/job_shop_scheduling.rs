//! Job Shop Scheduling problem implementation.
//!
//! Given `m` processors and a set of jobs, each job consisting of an ordered
//! sequence of processor-length tasks, find a schedule that minimizes the
//! makespan (completion time of the last task) while respecting both within-job
//! precedence and single-processor capacity constraints.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

inventory::submit! {
    ProblemSchemaEntry {
        name: "JobShopScheduling",
        display_name: "Job-Shop Scheduling",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Minimize the makespan of a job-shop schedule",
        fields: &[
            FieldInfo { name: "num_processors", type_name: "usize", description: "Number of processors m" },
            FieldInfo { name: "jobs", type_name: "Vec<Vec<(usize, u64)>>", description: "jobs[j][k] = (processor, length) for the k-th task of job j" },
        ],
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobShopScheduling {
    num_processors: usize,
    jobs: Vec<Vec<(usize, u64)>>,
}

struct FlattenedTasks {
    job_task_ids: Vec<Vec<usize>>,
    machine_task_ids: Vec<Vec<usize>>,
    lengths: Vec<u64>,
}

impl JobShopScheduling {
    pub fn new(num_processors: usize, jobs: Vec<Vec<(usize, u64)>>) -> Self {
        let num_tasks: usize = jobs.iter().map(Vec::len).sum();
        if num_tasks > 0 {
            assert!(
                num_processors > 0,
                "num_processors must be positive when tasks are present"
            );
        }

        for (job_index, job) in jobs.iter().enumerate() {
            for (task_index, &(processor, _length)) in job.iter().enumerate() {
                assert!(
                    processor < num_processors,
                    "job {job_index} task {task_index} uses processor {processor}, but num_processors = {num_processors}"
                );
            }

            for (task_index, pair) in job.windows(2).enumerate() {
                assert_ne!(
                    pair[0].0,
                    pair[1].0,
                    "job {job_index} tasks {task_index} and {} must use different processors",
                    task_index + 1
                );
            }
        }

        Self {
            num_processors,
            jobs,
        }
    }

    pub fn num_processors(&self) -> usize {
        self.num_processors
    }

    pub fn jobs(&self) -> &[Vec<(usize, u64)>] {
        &self.jobs
    }

    pub fn num_jobs(&self) -> usize {
        self.jobs.len()
    }

    pub fn num_tasks(&self) -> usize {
        self.jobs.iter().map(Vec::len).sum()
    }

    fn flatten_tasks(&self) -> FlattenedTasks {
        let mut job_task_ids = Vec::with_capacity(self.jobs.len());
        let mut machine_task_ids = vec![Vec::new(); self.num_processors];
        let mut lengths = Vec::with_capacity(self.num_tasks());
        let mut task_id = 0usize;

        for job in &self.jobs {
            let mut ids = Vec::with_capacity(job.len());
            for &(processor, length) in job {
                ids.push(task_id);
                machine_task_ids[processor].push(task_id);
                lengths.push(length);
                task_id += 1;
            }
            job_task_ids.push(ids);
        }

        FlattenedTasks {
            job_task_ids,
            machine_task_ids,
            lengths,
        }
    }

    fn decode_machine_orders(
        &self,
        config: &[usize],
        flattened: &FlattenedTasks,
    ) -> Option<Vec<Vec<usize>>> {
        if config.len() != flattened.lengths.len() {
            return None;
        }

        let mut offset = 0usize;
        let mut orders = Vec::with_capacity(flattened.machine_task_ids.len());

        for machine_tasks in &flattened.machine_task_ids {
            let next_offset = offset + machine_tasks.len();
            let segment = &config[offset..next_offset];
            offset = next_offset;

            let mut available = machine_tasks.clone();
            let mut order = Vec::with_capacity(machine_tasks.len());
            for &digit in segment {
                if digit >= available.len() {
                    return None;
                }
                order.push(available.remove(digit));
            }
            orders.push(order);
        }

        Some(orders)
    }

    /// Compute start times from a Lehmer-code config. Returns `None` if the
    /// config is invalid or induces a cycle in the precedence DAG.
    pub fn schedule_from_config(&self, config: &[usize]) -> Option<Vec<u64>> {
        self.schedule_from_config_inner(config, &self.flatten_tasks())
    }

    fn schedule_from_config_inner(
        &self,
        config: &[usize],
        flattened: &FlattenedTasks,
    ) -> Option<Vec<u64>> {
        let machine_orders = self.decode_machine_orders(config, flattened)?;
        let num_tasks = flattened.lengths.len();

        if num_tasks == 0 {
            return Some(Vec::new());
        }

        let mut adjacency = vec![Vec::<usize>::new(); num_tasks];
        let mut indegree = vec![0usize; num_tasks];

        for job_ids in &flattened.job_task_ids {
            for pair in job_ids.windows(2) {
                adjacency[pair[0]].push(pair[1]);
                indegree[pair[1]] += 1;
            }
        }

        for machine_order in &machine_orders {
            for pair in machine_order.windows(2) {
                adjacency[pair[0]].push(pair[1]);
                indegree[pair[1]] += 1;
            }
        }

        let mut queue = VecDeque::new();
        for (task_id, &degree) in indegree.iter().enumerate() {
            if degree == 0 {
                queue.push_back(task_id);
            }
        }

        let mut start_times = vec![0u64; num_tasks];
        let mut processed = 0usize;

        while let Some(task_id) = queue.pop_front() {
            processed += 1;
            let finish = start_times[task_id].checked_add(flattened.lengths[task_id])?;

            for &next_task in &adjacency[task_id] {
                start_times[next_task] = start_times[next_task].max(finish);
                indegree[next_task] -= 1;
                if indegree[next_task] == 0 {
                    queue.push_back(next_task);
                }
            }
        }

        if processed != num_tasks {
            return None;
        }

        Some(start_times)
    }
}

impl Problem for JobShopScheduling {
    const NAME: &'static str = "JobShopScheduling";
    type Value = Min<u64>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        self.flatten_tasks()
            .machine_task_ids
            .into_iter()
            .flat_map(|machine_tasks| (0..machine_tasks.len()).rev().map(|i| i + 1))
            .collect()
    }

    fn evaluate(&self, config: &[usize]) -> Min<u64> {
        let flattened = self.flatten_tasks();
        match self.schedule_from_config_inner(config, &flattened) {
            Some(start_times) => {
                let makespan = start_times
                    .iter()
                    .enumerate()
                    .map(|(i, &s)| s + flattened.lengths[i])
                    .max()
                    .unwrap_or(0);
                Min(Some(makespan))
            }
            None => Min(None),
        }
    }
}

crate::declare_variants! {
    default JobShopScheduling => "factorial(num_tasks)",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "job_shop_scheduling",
        instance: Box::new(JobShopScheduling::new(
            2,
            vec![
                vec![(0, 3), (1, 4)],
                vec![(1, 2), (0, 3), (1, 2)],
                vec![(0, 4), (1, 3)],
                vec![(1, 5), (0, 2)],
                vec![(0, 2), (1, 3), (0, 1)],
            ],
        )),
        // Machine 0 order [0,3,5,8,9,11] => [0,0,0,0,0,0]
        // Machine 1 order [2,7,1,6,10,4] => [1,3,0,1,1,0]
        optimal_config: vec![0, 0, 0, 0, 0, 0, 1, 3, 0, 1, 1, 0],
        optimal_value: serde_json::json!(19),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/job_shop_scheduling.rs"]
mod tests;
