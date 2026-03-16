//! Staff Scheduling problem implementation.
//!
//! Given a collection of schedule patterns, period staffing requirements, and a
//! worker budget, determine whether workers can be assigned to schedules so that
//! all requirements are met without exceeding the budget.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{Problem, SatisfactionProblem};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "StaffScheduling",
        display_name: "Staff Scheduling",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Assign workers to schedule patterns to satisfy per-period staffing requirements within a worker budget",
        fields: &[
            FieldInfo { name: "shifts_per_schedule", type_name: "usize", description: "Required number of active periods in each schedule pattern" },
            FieldInfo { name: "schedules", type_name: "Vec<Vec<bool>>", description: "Binary schedule patterns available to workers" },
            FieldInfo { name: "requirements", type_name: "Vec<u64>", description: "Minimum staffing requirement for each period" },
            FieldInfo { name: "num_workers", type_name: "u64", description: "Maximum number of workers available" },
        ],
    }
}

/// The Staff Scheduling problem.
///
/// Each variable represents how many workers adopt a particular schedule
/// pattern. A configuration is satisfying iff the total assigned workers does
/// not exceed `num_workers` and every period's staffing requirement is met.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffScheduling {
    shifts_per_schedule: usize,
    schedules: Vec<Vec<bool>>,
    requirements: Vec<u64>,
    num_workers: u64,
}

impl StaffScheduling {
    /// Create a new Staff Scheduling instance.
    pub fn new(
        shifts_per_schedule: usize,
        schedules: Vec<Vec<bool>>,
        requirements: Vec<u64>,
        num_workers: u64,
    ) -> Self {
        assert!(
            num_workers < usize::MAX as u64,
            "num_workers must fit in usize so dims() can encode 0..=num_workers"
        );

        let num_periods = requirements.len();
        for (index, schedule) in schedules.iter().enumerate() {
            assert_eq!(
                schedule.len(),
                num_periods,
                "schedule {} has {} periods, expected {}",
                index,
                schedule.len(),
                num_periods
            );
            let ones = schedule.iter().filter(|&&active| active).count();
            assert_eq!(
                ones, shifts_per_schedule,
                "schedule {} has {} active periods, expected {}",
                index, ones, shifts_per_schedule
            );
        }

        Self {
            shifts_per_schedule,
            schedules,
            requirements,
            num_workers,
        }
    }

    /// Get the number of periods.
    pub fn num_periods(&self) -> usize {
        self.requirements.len()
    }

    /// Get the required number of active periods per schedule.
    pub fn shifts_per_schedule(&self) -> usize {
        self.shifts_per_schedule
    }

    /// Get the schedule patterns.
    pub fn schedules(&self) -> &[Vec<bool>] {
        &self.schedules
    }

    /// Get the staffing requirements.
    pub fn requirements(&self) -> &[u64] {
        &self.requirements
    }

    /// Get the worker budget.
    pub fn num_workers(&self) -> u64 {
        self.num_workers
    }

    /// Get the number of schedule patterns.
    pub fn num_schedules(&self) -> usize {
        self.schedules.len()
    }

    fn worker_limit(&self) -> usize {
        self.num_workers as usize
    }

    fn worker_counts_valid(&self, config: &[usize]) -> bool {
        config.iter().all(|&count| count <= self.worker_limit())
    }

    fn within_budget(&self, config: &[usize]) -> bool {
        config.iter().map(|&count| count as u128).sum::<u128>() <= self.num_workers as u128
    }

    fn meets_requirements(&self, config: &[usize]) -> bool {
        let mut coverage = vec![0u128; self.num_periods()];

        for (count, schedule) in config.iter().zip(&self.schedules) {
            if *count == 0 {
                continue;
            }
            let count = *count as u128;
            for (period, active) in schedule.iter().enumerate() {
                if *active {
                    coverage[period] += count;
                }
            }
        }

        coverage
            .iter()
            .zip(&self.requirements)
            .all(|(covered, required)| *covered >= *required as u128)
    }
}

impl Problem for StaffScheduling {
    const NAME: &'static str = "StaffScheduling";
    type Metric = bool;

    fn dims(&self) -> Vec<usize> {
        vec![self.worker_limit() + 1; self.num_schedules()]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        if config.len() != self.num_schedules() {
            return false;
        }
        self.worker_counts_valid(config)
            && self.within_budget(config)
            && self.meets_requirements(config)
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

impl SatisfactionProblem for StaffScheduling {}

crate::declare_variants! {
    default sat StaffScheduling => "(num_workers + 1)^num_schedules",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "staff_scheduling",
        build: || {
            let problem = StaffScheduling::new(
                5,
                vec![
                    vec![true, true, true, true, true, false, false],
                    vec![false, true, true, true, true, true, false],
                    vec![false, false, true, true, true, true, true],
                    vec![true, false, false, true, true, true, true],
                    vec![true, true, false, false, true, true, true],
                ],
                vec![2, 2, 2, 3, 3, 2, 1],
                4,
            );
            crate::example_db::specs::satisfaction_example(problem, vec![vec![1, 1, 1, 1, 0]])
        },
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/staff_scheduling.rs"]
mod tests;
