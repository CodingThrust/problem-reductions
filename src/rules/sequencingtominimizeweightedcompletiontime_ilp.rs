//! Reduction from SequencingToMinimizeWeightedCompletionTime to ILP.
//!
//! The reduction uses integer completion-time variables `C_j` and integer
//! order variables `y_{i,j}` constrained to `{0, 1}` within `ILP<i64>`.
//! For each unordered pair `{i, j}`, a pair of big-M constraints forces one
//! task to finish before the other starts.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::SequencingToMinimizeWeightedCompletionTime;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::types::{i64_to_exact_f64, MAX_EXACT_F64_INTEGER};

#[derive(Debug, Clone)]
pub struct ReductionSTMWCTToILP {
    target: ILP<i64>,
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
}

impl ReductionResult for ReductionSTMWCTToILP {
    type Source = SequencingToMinimizeWeightedCompletionTime;
    type Target = ILP<i64>;

    fn target_problem(&self) -> &ILP<i64> {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok({
            let mut schedule: Vec<usize> = (0..self.num_tasks).collect();
            schedule.sort_by_key(|&task| (target_solution[task], task));
            schedule
        })
    }
}

#[reduction(
    transform = exact {
        num_vars = "num_tasks + num_tasks * (num_tasks - 1) / 2",
        num_constraints = "2 * num_tasks + 3 * num_tasks * (num_tasks - 1) / 2 + num_precedences",
    },
    unavailable = {
        num_nonzeros = "the exact target parameter is not represented by this reduction's symbolic transform",
    }
)]
impl ReduceTo<ILP<i64>> for SequencingToMinimizeWeightedCompletionTime {
    type Result = ReductionSTMWCTToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let num_tasks = self.num_tasks();

        let total_processing_time = self.lengths().iter().try_fold(0i64, |total, &length| {
            total.checked_add(length).ok_or_else(|| {
                crate::rules::ReductionError::integer_overflow::<
                    SequencingToMinimizeWeightedCompletionTime,
                    ILP<i64>,
                >("summing task processing times")
            })
        })?;
        let total_weight = self
            .weights()
            .iter()
            .try_fold(0i64, |acc, &weight| acc.checked_add(weight))
            .ok_or_else(|| {
                crate::rules::ReductionError::integer_overflow::<
                    SequencingToMinimizeWeightedCompletionTime,
                    ILP<i64>,
                >("summing task weights")
            })?;
        let maximum_objective =
            total_processing_time
                .checked_mul(total_weight)
                .ok_or_else(|| {
                    crate::rules::ReductionError::integer_overflow::<
                        SequencingToMinimizeWeightedCompletionTime,
                        ILP<i64>,
                    >("bounding the weighted completion objective")
                })?;
        if maximum_objective > MAX_EXACT_F64_INTEGER {
            return Err(crate::rules::ReductionError::invalid_target::<
                SequencingToMinimizeWeightedCompletionTime,
                ILP<i64>,
            >(
                "weighted completion objective is not exactly representable by the ILP backend",
            ));
        }

        let lengths = self.lengths();
        let weights = self
            .weights()
            .iter()
            .copied()
            .map(i64_to_exact_f64)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| {
                crate::rules::ReductionError::inexact_float_conversion::<
                    SequencingToMinimizeWeightedCompletionTime,
                    ILP<i64>,
                >(error)
            })?;
        let num_order_vars = num_tasks * (num_tasks.saturating_sub(1)) / 2;
        let num_vars = num_tasks + num_order_vars;

        let order_var = |i: usize, j: usize| -> usize {
            debug_assert!(i < j);
            num_tasks + i * (2 * num_tasks - i - 1) / 2 + (j - i - 1)
        };

        let mut constraints = Vec::new();

        for (task, &length) in lengths.iter().enumerate() {
            constraints.push(LinearConstraint::ge(vec![(task, 1)], length));
            constraints.push(LinearConstraint::le(vec![(task, 1)], total_processing_time));
        }

        for i in 0..num_tasks {
            for j in (i + 1)..num_tasks {
                let order = order_var(i, j);
                let completion_i = i;
                let completion_j = j;
                let length_i = lengths[i];
                let length_j = lengths[j];

                constraints.push(LinearConstraint::le(vec![(order, 1)], 1));

                // If y_{i,j} = 1, then task i is before task j: C_j - C_i >= l_j.
                constraints.push(LinearConstraint::ge(
                    vec![
                        (completion_j, 1),
                        (completion_i, -1),
                        (order, -total_processing_time),
                    ],
                    length_j - total_processing_time,
                ));

                // If y_{i,j} = 0, then task j is before task i: C_i - C_j >= l_i.
                constraints.push(LinearConstraint::ge(
                    vec![
                        (completion_i, 1),
                        (completion_j, -1),
                        (order, total_processing_time),
                    ],
                    length_i,
                ));
            }
        }

        for &(pred, succ) in self.precedences() {
            constraints.push(LinearConstraint::ge(
                vec![(succ, 1), (pred, -1)],
                lengths[succ],
            ));
        }

        let objective = weights.into_iter().enumerate().collect();

        Ok(Self::Result {
            target: ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize)
                .map_err(Self::target_construction)?,
            num_tasks,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "sequencingtominimizeweightedcompletiontime_to_ilp",
        build: || {
            let source =
                SequencingToMinimizeWeightedCompletionTime::new(vec![2, 1], vec![3, 5], vec![]);
            crate::example_db::specs::rule_example_via_ilp::<_, i64>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/sequencingtominimizeweightedcompletiontime_ilp.rs"]
mod tests;
