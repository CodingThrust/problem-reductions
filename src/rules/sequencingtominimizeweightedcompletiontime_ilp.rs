//! Strict linear ordering with exact signed completion-time equations.
//! Triangle inequalities rule out cyclic orders even for zero-duration jobs.

use crate::models::algebraic::{Bounded, IntegerVariable, LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::SequencingToMinimizeWeightedCompletionTime;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionSTMWCTToILP {
    target: ILP<i64, i64, Bounded>,
    num_tasks: usize,
}

impl ReductionResult for ReductionSTMWCTToILP {
    type Source = SequencingToMinimizeWeightedCompletionTime;
    type Target = ILP<i64, i64, Bounded>;

    fn target_problem(&self) -> &ILP<i64, i64, Bounded> {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_witness(
            self.target_problem(),
            target_solution,
            |value| value.value.is_some(),
            "target ILP assignment is infeasible",
        )?;

        let mut ranks = vec![0; self.num_tasks];
        for i in 0..self.num_tasks {
            for j in i + 1..self.num_tasks {
                let pair = self.num_tasks + i * (2 * self.num_tasks - i - 1) / 2 + j - i - 1;
                ranks[if target_solution[pair] == 1 { j } else { i }] += 1;
            }
        }
        let mut order: Vec<_> = (0..self.num_tasks).collect();
        order.sort_by_key(|&j| ranks[j]);
        Ok(order)
    }
}

#[reduction(transform = {
    exact {
        num_vars = "num_tasks + num_tasks * (num_tasks - 1) / 2",
        num_constraints = "num_tasks * (num_tasks - 1) * (num_tasks - 2) / 3 + num_tasks + num_precedences",
    },
    upper_bound {
        max_constraint_magnitude_bits = "max_processing_time_bits + num_tasks",
        num_nonzeros = "num_tasks * (num_tasks - 1) * (num_tasks - 2) + num_tasks^2 + num_precedences",
    },
})]
impl ReduceTo<ILP<i64, i64, Bounded>> for SequencingToMinimizeWeightedCompletionTime {
    type Result = ReductionSTMWCTToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let overflow = |operation: &str| {
            crate::rules::ReductionError::integer_overflow::<Self, ILP<i64, i64, Bounded>>(
                operation,
            )
        };
        let integer = |value: i128| {
            i64::try_from(value).map_err(|_| overflow("representing an exact completion equation"))
        };
        let n = self.num_tasks();
        let pairs = n
            .checked_mul(n.saturating_sub(1))
            .ok_or_else(|| overflow("counting ordering variables"))?
            / 2;
        let num_vars = n
            .checked_add(pairs)
            .ok_or_else(|| overflow("counting sequencing variables"))?;
        let rows = pairs
            .checked_mul(n.saturating_sub(2))
            .and_then(|x| (x / 3).checked_mul(2))
            .and_then(|x| x.checked_add(n))
            .and_then(|x| x.checked_add(self.num_precedences()))
            .ok_or_else(|| overflow("counting sequencing constraints"))?;
        let lengths = self.lengths();
        // Any permutation prefix lies between the sums of negative and positive lengths.
        let lower: i128 = lengths.iter().map(|&p| i128::from(p.min(0))).sum();
        let upper: i128 = lengths.iter().map(|&p| i128::from(p.max(0))).sum();
        integer(lower)?;
        integer(upper)?;
        let mut variables = Vec::with_capacity(num_vars);
        for &p in lengths {
            let low = integer(i128::from(p) + lower - i128::from(p.min(0)))?;
            let high = integer(i128::from(p) + upper - i128::from(p.max(0)))?;
            variables.push(
                IntegerVariable::new(Some(low), Some(high)).map_err(Self::target_construction)?,
            );
        }
        variables.resize(num_vars, IntegerVariable::binary());
        let pair = |i: usize, j: usize| n + i * (2 * n - i - 1) / 2 + j - i - 1;
        let mut constraints = Vec::with_capacity(rows);
        for i in 0..n {
            for j in i + 1..n {
                for k in j + 1..n {
                    let terms = vec![(pair(i, j), 1), (pair(j, k), 1), (pair(i, k), -1)];
                    constraints.push(LinearConstraint::ge(terms.clone(), 0));
                    constraints.push(LinearConstraint::le(terms, 1));
                }
            }
        }
        for &(a, b) in self.precedences() {
            let terms = if a == b {
                vec![]
            } else {
                vec![(pair(a.min(b), a.max(b)), 1)]
            };
            constraints.push(LinearConstraint::eq(terms, i64::from(a <= b)));
        }
        for j in 0..n {
            let mut terms = vec![(j, 1)];
            for (i, &p) in lengths.iter().enumerate().take(j) {
                terms.push((pair(i, j), integer(-i128::from(p))?));
            }
            for (i, &p) in lengths.iter().enumerate().skip(j + 1) {
                terms.push((pair(j, i), p));
            }
            let rhs = integer(lengths[j..].iter().map(|&p| i128::from(p)).sum())?;
            constraints.push(LinearConstraint::eq(terms, rhs));
        }
        // Keep the source's coefficients and task-index accumulation order exactly.
        let objective = self.weights().iter().copied().enumerate().collect();
        let target =
            ILP::with_variables(variables, constraints, objective, ObjectiveSense::Minimize)
                .map_err(Self::target_construction)?;
        crate::rules::ilp_helpers::validate_bounded_constraint_arithmetic::<Self>(&target)?;
        Ok(Self::Result {
            target,
            num_tasks: n,
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
            crate::example_db::specs::rule_example_via_bounded_ilp::<_>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/sequencingtominimizeweightedcompletiontime_ilp.rs"]
mod tests;
