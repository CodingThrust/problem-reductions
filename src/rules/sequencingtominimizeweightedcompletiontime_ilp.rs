//! Reduction from SequencingToMinimizeWeightedCompletionTime to ILP.
//!
//! The reduction uses integer completion-time variables `C_j` and integer
//! order variables `y_{i,j}` constrained to `{0, 1}` within `ILP<i32>`.
//! For each unordered pair `{i, j}`, a pair of big-M constraints forces one
//! task to finish before the other starts.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::SequencingToMinimizeWeightedCompletionTime;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionSTMWCTToILP {
    target: ILP<i32>,
    num_tasks: usize,
}

impl ReductionSTMWCTToILP {
    #[cfg(test)]
    pub(crate) fn completion_var(&self, task: usize) -> usize {
        task
    }

    #[cfg(test)]
    pub(crate) fn order_var(&self, i: usize, j: usize) -> usize {
        assert!(i < j, "order_var expects i < j");
        self.num_tasks + i * (2 * self.num_tasks - i - 1) / 2 + (j - i - 1)
    }

    fn encode_schedule_as_lehmer(schedule: &[usize]) -> Vec<usize> {
        let mut available: Vec<usize> = (0..schedule.len()).collect();
        let mut config = Vec::with_capacity(schedule.len());
        for &task in schedule {
            let digit = available
                .iter()
                .position(|&candidate| candidate == task)
                .expect("schedule must be a permutation");
            config.push(digit);
            available.remove(digit);
        }
        config
    }
}

impl ReductionResult for ReductionSTMWCTToILP {
    type Source = SequencingToMinimizeWeightedCompletionTime;
    type Target = ILP<i32>;

    fn target_problem(&self) -> &ILP<i32> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let mut schedule: Vec<usize> = (0..self.num_tasks).collect();
        schedule.sort_by_key(|&task| (target_solution.get(task).copied().unwrap_or(0), task));
        Self::encode_schedule_as_lehmer(&schedule)
    }
}

#[reduction(overhead = {
    num_vars = "num_tasks + num_tasks * (num_tasks - 1) / 2",
    num_constraints = "2 * num_tasks + 3 * num_tasks * (num_tasks - 1) / 2 + num_precedences",
})]
impl ReduceTo<ILP<i32>> for SequencingToMinimizeWeightedCompletionTime {
    type Result = ReductionSTMWCTToILP;

    fn reduce_to(&self) -> Self::Result {
        let num_tasks = self.num_tasks();
        let max_ilp_value = i32::MAX as u64;
        let max_exact_f64_integer = 1u64 << 53;
        assert!(
            self.lengths().iter().all(|&length| length <= max_ilp_value),
            "task lengths must fit in ILP<i32> variable bounds"
        );

        let total_processing_time_u64 = self.total_processing_time();
        assert!(
            total_processing_time_u64 <= max_ilp_value,
            "total processing time must fit in ILP<i32> variable bounds"
        );

        let total_weight = self
            .weights()
            .iter()
            .try_fold(0u64, |acc, &weight| acc.checked_add(weight))
            .expect("weighted completion objective must fit exactly in f64");
        assert!(
            total_processing_time_u64 == 0
                || total_weight <= max_exact_f64_integer / total_processing_time_u64,
            "weighted completion objective must fit exactly in f64"
        );

        let total_processing_time = total_processing_time_u64 as f64;
        let num_order_vars = num_tasks * (num_tasks.saturating_sub(1)) / 2;
        let num_vars = num_tasks + num_order_vars;

        let order_var = |i: usize, j: usize| -> usize {
            debug_assert!(i < j);
            num_tasks + i * (2 * num_tasks - i - 1) / 2 + (j - i - 1)
        };

        let mut constraints = Vec::new();

        for (task, &length) in self.lengths().iter().enumerate() {
            constraints.push(LinearConstraint::ge(vec![(task, 1.0)], length as f64));
            constraints.push(LinearConstraint::le(
                vec![(task, 1.0)],
                total_processing_time,
            ));
        }

        for i in 0..num_tasks {
            for j in (i + 1)..num_tasks {
                let order = order_var(i, j);
                let completion_i = i;
                let completion_j = j;
                let length_i = self.lengths()[i] as f64;
                let length_j = self.lengths()[j] as f64;

                constraints.push(LinearConstraint::le(vec![(order, 1.0)], 1.0));

                // If y_{i,j} = 1, then task i is before task j: C_j - C_i >= l_j.
                constraints.push(LinearConstraint::ge(
                    vec![
                        (completion_j, 1.0),
                        (completion_i, -1.0),
                        (order, -total_processing_time),
                    ],
                    length_j - total_processing_time,
                ));

                // If y_{i,j} = 0, then task j is before task i: C_i - C_j >= l_i.
                constraints.push(LinearConstraint::ge(
                    vec![
                        (completion_i, 1.0),
                        (completion_j, -1.0),
                        (order, total_processing_time),
                    ],
                    length_i,
                ));
            }
        }

        for &(pred, succ) in self.precedences() {
            constraints.push(LinearConstraint::ge(
                vec![(succ, 1.0), (pred, -1.0)],
                self.lengths()[succ] as f64,
            ));
        }

        let objective = self
            .weights()
            .iter()
            .enumerate()
            .map(|(task, &weight)| (task, weight as f64))
            .collect();

        Self::Result {
            target: ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize),
            num_tasks,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
        vec![crate::example_db::specs::RuleExampleSpec {
        id: "sequencingtominimizeweightedcompletiontime_to_ilp",
        build: || {
            let source = SequencingToMinimizeWeightedCompletionTime::new(vec![2, 1], vec![3, 5], vec![]);
            crate::example_db::specs::rule_example_via_ilp::<_, i32>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/sequencingtominimizeweightedcompletiontime_ilp.rs"]
mod tests;
