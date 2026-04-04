//! Open Shop Scheduling problem implementation.
//!
//! Given `m` machines and a set of jobs, each consisting of one task per
//! machine, find a schedule minimizing the makespan.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "OpenShopScheduling",
        display_name: "Open Shop Scheduling",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Schedule open-shop jobs on m machines to minimize makespan",
        fields: &[
            FieldInfo { name: "num_machines", type_name: "usize", description: "Number of machines m" },
            FieldInfo { name: "processing_times", type_name: "Vec<Vec<u64>>", description: "processing_times[j][i] = processing time of job j on machine i" },
        ],
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenShopScheduling {
    num_machines: usize,
    processing_times: Vec<Vec<u64>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DecodedOpenShopSchedule {
    pub machine_orders: Vec<Vec<usize>>,
    pub start_times: Vec<Vec<u64>>,
    pub makespan: u64,
}

impl OpenShopScheduling {
    pub fn new(num_machines: usize, processing_times: Vec<Vec<u64>>) -> Self {
        assert!(num_machines > 0, "num_machines must be positive");
        for (job, row) in processing_times.iter().enumerate() {
            assert_eq!(
                row.len(),
                num_machines,
                "job {} has {} tasks, expected {}",
                job,
                row.len(),
                num_machines
            );
        }
        Self {
            num_machines,
            processing_times,
        }
    }

    pub fn num_jobs(&self) -> usize {
        self.processing_times.len()
    }

    pub fn num_machines(&self) -> usize {
        self.num_machines
    }

    pub fn processing_times(&self) -> &[Vec<u64>] {
        &self.processing_times
    }

    fn decode_permutation(&self, digits: &[usize]) -> Option<Vec<usize>> {
        let n = self.num_jobs();
        if digits.len() != n {
            return None;
        }

        let mut available: Vec<usize> = (0..n).collect();
        let mut permutation = Vec::with_capacity(n);
        for &digit in digits {
            if digit >= available.len() {
                return None;
            }
            permutation.push(available.remove(digit));
        }
        Some(permutation)
    }

    pub(crate) fn decode_machine_orders(&self, config: &[usize]) -> Option<Vec<Vec<usize>>> {
        let n = self.num_jobs();
        let expected_len = n.checked_mul(self.num_machines)?;
        if config.len() != expected_len {
            return None;
        }
        if n == 0 {
            return Some(vec![Vec::new(); self.num_machines]);
        }

        config
            .chunks_exact(n)
            .map(|chunk| self.decode_permutation(chunk))
            .collect()
    }

    pub(crate) fn schedule_from_machine_orders(
        &self,
        machine_orders: &[Vec<usize>],
    ) -> Option<DecodedOpenShopSchedule> {
        if machine_orders.len() != self.num_machines {
            return None;
        }

        let n = self.num_jobs();
        if n == 0 {
            return Some(DecodedOpenShopSchedule {
                machine_orders: machine_orders.to_vec(),
                start_times: Vec::new(),
                makespan: 0,
            });
        }

        for order in machine_orders {
            if order.len() != n {
                return None;
            }
            let mut seen = vec![false; n];
            for &job in order {
                if job >= n || seen[job] {
                    return None;
                }
                seen[job] = true;
            }
        }

        let mut next_position = vec![0usize; self.num_machines];
        let mut machine_available = vec![0u64; self.num_machines];
        let mut job_available = vec![0u64; n];
        let mut start_times = vec![vec![0u64; self.num_machines]; n];

        for _ in 0..(n * self.num_machines) {
            let mut best_candidate: Option<(u64, u64, usize, usize)> = None;
            for machine in 0..self.num_machines {
                let position = next_position[machine];
                if position >= n {
                    continue;
                }

                let job = machine_orders[machine][position];
                let start = machine_available[machine].max(job_available[job]);
                let completion = start
                    .checked_add(self.processing_times[job][machine])
                    .expect("makespan overflowed u64");
                let candidate = (completion, start, machine, job);
                if best_candidate.is_none_or(|current| candidate < current) {
                    best_candidate = Some(candidate);
                }
            }

            let (completion, start, machine, job) =
                best_candidate.expect("there must be a schedulable operation");
            start_times[job][machine] = start;
            machine_available[machine] = completion;
            job_available[job] = completion;
            next_position[machine] += 1;
        }

        Some(DecodedOpenShopSchedule {
            machine_orders: machine_orders.to_vec(),
            start_times,
            makespan: job_available.into_iter().max().unwrap_or(0),
        })
    }

    pub(crate) fn schedule_from_config(&self, config: &[usize]) -> Option<DecodedOpenShopSchedule> {
        let machine_orders = self.decode_machine_orders(config)?;
        self.schedule_from_machine_orders(&machine_orders)
    }
}

impl Problem for OpenShopScheduling {
    const NAME: &'static str = "OpenShopScheduling";
    type Value = Min<u64>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        let n = self.num_jobs();
        let lehmer_dims: Vec<usize> = (0..n).rev().map(|i| i + 1).collect();
        (0..self.num_machines)
            .flat_map(|_| lehmer_dims.iter().copied())
            .collect()
    }

    fn evaluate(&self, config: &[usize]) -> Min<u64> {
        self.schedule_from_config(config)
            .map(|schedule| Min(Some(schedule.makespan)))
            .unwrap_or(Min(None))
    }
}

crate::declare_variants! {
    default OpenShopScheduling => "factorial(num_jobs)^num_machines",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "open_shop_scheduling",
        instance: Box::new(OpenShopScheduling::new(
            3,
            vec![vec![3, 1, 2], vec![2, 3, 1], vec![1, 2, 3], vec![2, 2, 1]],
        )),
        optimal_config: vec![0, 0, 0, 0, 1, 0, 1, 0, 2, 2, 0, 0],
        optimal_value: serde_json::json!(8),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/open_shop_scheduling.rs"]
mod tests;
