//! Reduction from MultiprocessorScheduling to ILP (Integer Linear Programming).
//!
//! The Multiprocessor Scheduling feasibility problem can be formulated as a binary ILP:
//! - Variables: Binary x_{j,p} (task j assigned to processor p), one-hot per task
//! - Constraints: Σ_p x_{j,p} = 1 for each task j (assignment); Σ_j len_j·x_{j,p} ≤ deadline for each p (load)
//! - Objective: Minimize 0 (feasibility)
//! - Extraction: argmax_p x_{j,p} for each task j

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::MultiprocessorScheduling;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::types::i64_to_exact_f64;

/// Result of reducing MultiprocessorScheduling to ILP.
///
/// Variable layout: x_{j,p} at index j * num_processors + p.
/// - j ∈ 0..num_tasks, p ∈ 0..num_processors
///
/// Total: num_tasks * num_processors variables.
#[derive(Debug, Clone)]
pub struct ReductionMSToILP {
    target: ILP<bool>,
    num_tasks: usize,
    num_processors: usize,
}

impl ReductionResult for ReductionMSToILP {
    type Source = MultiprocessorScheduling;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    /// Extract solution: for each task j, find the unique processor p where x_{j,p} = 1.
    fn extract_solution(
        &self,
        target_solution: &[usize],
    ) -> crate::rules::ExtractionResult<Vec<usize>> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        crate::rules::ilp_helpers::one_hot_decode_rows(
            target_solution,
            self.num_tasks,
            self.num_processors,
            0,
        )
    }
}

#[reduction(
    size = exact {
        num_vars = "num_tasks * num_processors",
        num_constraints = "num_tasks + num_processors",
    },
)]
impl ReduceTo<ILP<bool>> for MultiprocessorScheduling {
    type Result = ReductionMSToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let num_tasks = self.num_tasks();
        let num_processors = self.num_processors();
        let num_vars = num_tasks * num_processors;
        let lengths = self
            .lengths()
            .iter()
            .copied()
            .map(i64_to_exact_f64)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| {
                crate::rules::ReductionError::inexact_float_conversion::<
                    MultiprocessorScheduling,
                    ILP<bool>,
                >(error)
            })?;
        let deadline = i64_to_exact_f64(self.deadline()).map_err(|error| {
            crate::rules::ReductionError::inexact_float_conversion::<
                MultiprocessorScheduling,
                ILP<bool>,
            >(error)
        })?;

        let mut constraints = Vec::with_capacity(num_tasks + num_processors);

        // Assignment constraints: for each task j, Σ_p x_{j,p} = 1
        for j in 0..num_tasks {
            let terms: Vec<(usize, f64)> = (0..num_processors)
                .map(|p| (j * num_processors + p, 1.0))
                .collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // Load constraints: for each processor p, Σ_j len_j * x_{j,p} ≤ deadline
        for p in 0..num_processors {
            let terms: Vec<(usize, f64)> = (0..num_tasks)
                .map(|j| (j * num_processors + p, lengths[j]))
                .collect();
            constraints.push(LinearConstraint::le(terms, deadline));
        }

        let target = ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize);

        Ok(ReductionMSToILP {
            target,
            num_tasks,
            num_processors,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "multiprocessorscheduling_to_ilp",
        build: || {
            // 3 tasks with lengths [4, 5, 3], 2 processors, deadline 7
            // Assignment: task 0 → processor 0, task 1 → processor 1, task 2 → processor 1
            // Loads: processor 0 = 4, processor 1 = 5+3 = 8 (wait, try different)
            // Assignment: task 0 → processor 0, task 1 → processor 1, task 2 → processor 0
            // Loads: processor 0 = 4+3=7, processor 1 = 5 ≤ 7. Feasible!
            let source = MultiprocessorScheduling::new(vec![4, 5, 3], 2, 7);
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/multiprocessorscheduling_ilp.rs"]
mod tests;
