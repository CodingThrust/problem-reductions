//! Reduction from ExactCoverBy3Sets to StaffScheduling.
//!
//! Given an X3C instance with universe X (|X| = 3q) and collection C of
//! 3-element subsets, construct a StaffScheduling instance where:
//! - Each universe element becomes a period (m = 3q periods)
//! - Each subset S_j = {a, b, c} becomes a schedule with shifts at positions a, b, c
//! - All requirements are 1 (each period needs exactly 1 worker)
//! - The worker budget is q (an exact cover uses exactly q subsets)
//! - shifts_per_schedule = 3 (each schedule has exactly 3 active periods)
//!
//! An exact cover in X3C corresponds to a feasible staff assignment.

use crate::models::misc::StaffScheduling;
use crate::models::set::ExactCoverBy3Sets;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing ExactCoverBy3Sets to StaffScheduling.
#[derive(Debug, Clone)]
pub struct ReductionXC3SToStaffScheduling {
    target: StaffScheduling,
}

impl ReductionResult for ReductionXC3SToStaffScheduling {
    type Source = ExactCoverBy3Sets;
    type Target = StaffScheduling;

    fn target_problem(&self) -> &StaffScheduling {
        &self.target
    }

    /// Extract XC3S solution from StaffScheduling solution.
    ///
    /// StaffScheduling config[j] = number of workers assigned to schedule j.
    /// XC3S config[j] = 1 if subset j is selected, 0 otherwise.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution
            .iter()
            .map(|&count| if count > 0 { 1 } else { 0 })
            .collect()
    }
}

#[reduction(
    overhead = {
        num_periods = "universe_size",
        num_schedules = "num_subsets",
        num_workers = "universe_size / 3",
    }
)]
impl ReduceTo<StaffScheduling> for ExactCoverBy3Sets {
    type Result = ReductionXC3SToStaffScheduling;

    fn reduce_to(&self) -> Self::Result {
        let universe_size = self.universe_size();
        let q = universe_size / 3;

        // Build schedule patterns: one per subset
        let schedules: Vec<Vec<bool>> = self
            .subsets()
            .iter()
            .map(|subset| {
                let mut schedule = vec![false; universe_size];
                for &elem in subset {
                    schedule[elem] = true;
                }
                schedule
            })
            .collect();

        // Each period requires exactly 1 worker
        let requirements = vec![1u64; universe_size];

        let target = StaffScheduling::new(
            3, // shifts_per_schedule
            schedules,
            requirements,
            q as u64, // num_workers = q
        );

        ReductionXC3SToStaffScheduling { target }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "exactcoverby3sets_to_staffscheduling",
        build: || {
            // Universe {0,1,2,3,4,5}, subsets [{0,1,2}, {3,4,5}, {0,3,4}, {1,2,5}]
            // Exact cover: S0 + S1
            let source =
                ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [3, 4, 5], [0, 3, 4], [1, 2, 5]]);
            // In StaffScheduling, assigning 1 worker to schedule 0 and 1 worker to schedule 1
            crate::example_db::specs::rule_example_with_witness::<_, StaffScheduling>(
                source,
                SolutionPair {
                    source_config: vec![1, 1, 0, 0],
                    target_config: vec![1, 1, 0, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/exactcoverby3sets_staffscheduling.rs"]
mod tests;
