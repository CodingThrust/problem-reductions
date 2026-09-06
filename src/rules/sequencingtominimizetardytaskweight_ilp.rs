//! Reduction from SequencingToMinimizeTardyTaskWeight to `ILP<bool>`.
//!
//! Position-assignment ILP: binary x_{j,p} placing task j in position p,
//! with exact binary tardy indicators. Position-specific prefix bounds give
//! both implications of the deadline comparison, including signed model inputs.

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
        let value =
            crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;
        if !value.is_valid() {
            return Err(crate::rules::ExtractionError::invalid(
                "target ILP assignment is infeasible",
            ));
        }

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
        num_constraints = "2 * num_tasks + 2 * num_tasks * num_tasks",
    },
    unavailable = {
        num_nonzeros = "the exact target parameter is not represented by this reduction's symbolic transform",
    }
)]
impl ReduceTo<ILP<bool>> for SequencingToMinimizeTardyTaskWeight {
    type Result = ReductionSTMTTWToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let overflow = |operation: &str| {
            crate::rules::ReductionError::integer_overflow::<Self, ILP<bool>>(operation)
        };
        let integer = |value: i128| {
            i64::try_from(value).map_err(|_| overflow("representing exact tardiness constraints"))
        };
        let n = self.num_tasks();
        let num_x_vars = n
            .checked_mul(n)
            .ok_or_else(|| overflow("assignment count"))?;
        let num_vars = num_x_vars
            .checked_add(n)
            .ok_or_else(|| overflow("variable count"))?;
        let num_constraints = num_vars
            .checked_mul(2)
            .ok_or_else(|| overflow("constraint count"))?;
        // Upper bound: 2*n^2 assignment entries and two (n*p + 2)-term
        // inequalities for each task and position. The preceding count bound
        // also makes n^2 - n + 6 representable.
        num_x_vars
            .checked_mul(num_x_vars - n + 6)
            .ok_or_else(|| overflow("constraint nonzero count"))?;

        let lengths = self.lengths();
        let deadlines = self.deadlines();
        let weights = self.weights();
        let mut sorted = lengths.to_vec();
        sorted.sort_unstable();
        let mut lower = vec![0i128; n + 1];
        let mut upper = vec![0i128; n + 1];
        let mut partial_lower = vec![0i128; n + 1];
        let mut partial_upper = vec![0i128; n + 1];
        // Checked polynomial dimensions bound n, so summing n i64 values and
        // the subsequent constant arithmetic fit i128 before checked narrowing.
        for p in 0..n {
            lower[p + 1] = lower[p] + i128::from(sorted[p]);
            upper[p + 1] = upper[p] + i128::from(sorted[n - 1 - p]);
            partial_lower[p + 1] = partial_lower[p].min(lower[p + 1]);
            partial_upper[p + 1] = partial_upper[p].max(upper[p + 1]);
        }
        // All source completion times and objective accumulation partial sums
        // must be representable, including when lengths or weights are signed.
        integer(partial_lower[n])?;
        integer(partial_upper[n])?;
        integer(weights.iter().map(|&w| i128::from(w.min(0))).sum())?;
        integer(weights.iter().map(|&w| i128::from(w.max(0))).sum())?;

        let x_var = |j: usize, p: usize| j * n + p;
        let u_var = |j: usize| num_x_vars + j;
        let mut constraints = Vec::with_capacity(num_constraints);
        // Keep assignment constraints first: only permutation matrices reach
        // the prefix inequalities in the target's formal sequential evaluator.
        for j in 0..n {
            constraints.push(LinearConstraint::eq(
                (0..n).map(|p| (x_var(j, p), 1)).collect(),
                1,
            ));
        }
        for p in 0..n {
            constraints.push(LinearConstraint::eq(
                (0..n).map(|j| (x_var(j, p), 1)).collect(),
                1,
            ));
        }

        for j in 0..n {
            for p in 0..n {
                // P_p lies in [lower[p], upper[p]]. Replacing the integer
                // comparison cutoff by its projection onto [lower[p]-1,
                // upper[p]] preserves P_p > deadline[j]-length[j] exactly.
                let cutoff = (i128::from(deadlines[j]) - i128::from(lengths[j]))
                    .clamp(lower[p] - 1, upper[p]);
                let on_time_m = upper[p] - cutoff;
                let tardy_m = cutoff + 1 - lower[p];
                // Bounds cover every sparse dot-product partial sum for a
                // permutation matrix and every indicator vector, not just the
                // final sum of feasible target witnesses.
                integer(partial_lower[p] - on_time_m)?;
                integer(partial_upper[p] + on_time_m)?;
                integer(partial_lower[p] - 2 * tardy_m)?;
                let on_time_m = integer(on_time_m)?;
                let tardy_m = integer(tardy_m)?;
                let mut on_time_terms = Vec::with_capacity(n * p + 2);
                let mut tardy_terms = Vec::with_capacity(n * p + 2);
                for pp in 0..p {
                    for (task, &length) in lengths.iter().enumerate() {
                        on_time_terms.push((x_var(task, pp), length));
                        tardy_terms.push((x_var(task, pp), length));
                    }
                }
                on_time_terms.push((x_var(j, p), on_time_m));
                on_time_terms.push((u_var(j), -on_time_m));
                tardy_terms.push((x_var(j, p), -tardy_m));
                tardy_terms.push((u_var(j), -tardy_m));
                // x=1,u=0 forces P_p <= cutoff; x=1,u=1 forces
                // P_p >= cutoff+1. With x=0 both rows are redundant.
                constraints.push(LinearConstraint::le(on_time_terms, integer(upper[p])?));
                constraints.push(LinearConstraint::ge(
                    tardy_terms,
                    integer(lower[p] - i128::from(tardy_m))?,
                ));
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
