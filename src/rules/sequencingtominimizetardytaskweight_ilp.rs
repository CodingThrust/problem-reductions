//! Reduction from SequencingToMinimizeTardyTaskWeight to `ILP<bool>`.
//!
//! Position-assignment ILP: binary x_{j,p} placing task j in position p,
//! with binary tardy indicator u_j. A big-M constraint forces u_j = 1
//! whenever the completion time at position p exceeds the deadline d_j.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::SequencingToMinimizeTardyTaskWeight;
use crate::reduction;
use crate::rules::ilp_helpers::one_hot_decode;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing SequencingToMinimizeTardyTaskWeight to `ILP<bool>`.
#[derive(Debug, Clone)]
pub struct ReductionSTMTTWToILP {
    target: ILP<bool>,
    num_tasks: usize,
}

impl ReductionResult for ReductionSTMTTWToILP {
    type Source = SequencingToMinimizeTardyTaskWeight;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok({
            let n = self.num_tasks;
            // Decode the n*n block of x_{j,p} variables into a schedule permutation.
            // The source uses direct permutation encoding (config = schedule directly),
            // so return the schedule as-is (it is already a permutation of 0..n).
            one_hot_decode(target_solution, n, n, 0)?
        })
    }
}

#[reduction(
    transform = exact {
        num_vars = "num_tasks * num_tasks + num_tasks",
        num_constraints = "2 * num_tasks + num_tasks * num_tasks",
    },
    unavailable = {
        num_nonzeros = "the exact target parameter is not represented by this reduction's symbolic transform",
    }
)]
impl ReduceTo<ILP<bool>> for SequencingToMinimizeTardyTaskWeight {
    type Result = ReductionSTMTTWToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_tasks();
        let num_x_vars = n * n;
        let num_vars = num_x_vars + n;
        let total_length = self
            .lengths()
            .iter()
            .try_fold(0_i64, |total, &length| total.checked_add(length))
            .ok_or_else(|| {
                crate::rules::ReductionError::integer_overflow::<
                    SequencingToMinimizeTardyTaskWeight,
                    ILP<bool>,
                >("summing task processing times")
            })?;
        let big_m = total_length;
        let lengths = self.lengths();
        let deadlines = self.deadlines();
        let weights = self.weights();

        let x_var = |j: usize, p: usize| -> usize { j * n + p };
        let u_var = |j: usize| -> usize { num_x_vars + j };

        let mut constraints = Vec::new();

        // 1. Each task assigned to exactly one position
        for j in 0..n {
            let terms: Vec<(usize, i64)> = (0..n).map(|p| (x_var(j, p), 1)).collect();
            constraints.push(LinearConstraint::eq(terms, 1));
        }

        // 2. Each position has exactly one task
        for p in 0..n {
            let terms: Vec<(usize, i64)> = (0..n).map(|j| (x_var(j, p), 1)).collect();
            constraints.push(LinearConstraint::eq(terms, 1));
        }

        // 3. Tardy indicator: for each (j, p), if x_{j,p}=1 then
        //    completion_time_at_p >= l_j + sum_{p' < p} sum_{j'} l_{j'} * x_{j',p'}
        //    If completion > d_j then u_j must be 1.
        //    Linearized as: big_m * x_{j,p} + sum_{p'<p} sum_{j'} l_{j'} * x_{j',p'} - big_m * u_j <= d_j - l_j + big_m
        for j in 0..n {
            for p in 0..n {
                let mut terms: Vec<(usize, i64)> = Vec::new();
                terms.push((x_var(j, p), big_m));
                for pp in 0..p {
                    for (jj, &length) in lengths.iter().enumerate() {
                        terms.push((x_var(jj, pp), length));
                    }
                }
                terms.push((u_var(j), -big_m));
                let rhs = deadlines[j]
                    .checked_sub(lengths[j])
                    .and_then(|value| value.checked_add(big_m))
                    .ok_or_else(|| {
                        crate::rules::ReductionError::integer_overflow::<
                            SequencingToMinimizeTardyTaskWeight,
                            ILP<bool>,
                        >("computing a tardiness constraint bound")
                    })?;
                constraints.push(LinearConstraint::le(terms, rhs));
            }
        }

        // Objective: minimize sum w_j * u_j
        let objective: Vec<(usize, i64)> =
            (0..n).map(|task| (u_var(task), weights[task])).collect();

        Ok(ReductionSTMTTWToILP {
            target: ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize)
                .map_err(Self::target_construction)?,
            num_tasks: n,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "sequencingtominimizetardytaskweight_to_ilp",
        build: || {
            let source = SequencingToMinimizeTardyTaskWeight::new(
                vec![3, 2, 4, 1, 2],
                vec![5, 3, 7, 2, 4],
                vec![6, 4, 10, 2, 8],
            );
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/sequencingtominimizetardytaskweight_ilp.rs"]
mod tests;
