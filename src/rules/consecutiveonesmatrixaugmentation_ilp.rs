//! Bound each row’s span in a column permutation using two integer endpoints.

use crate::models::algebraic::{
    Bounded, ConsecutiveOnesMatrixAugmentation, IntegerVariable, LinearConstraint, ObjectiveSense,
    ILP,
};
use crate::reduction;
use crate::rules::ilp_helpers::{one_hot_assignment_constraints, one_hot_decode};
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionCOMAToILP {
    target: ILP<i64, i64, Bounded>,
    num_cols: usize,
}

impl ReductionResult for ReductionCOMAToILP {
    type Source = ConsecutiveOnesMatrixAugmentation;
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

        one_hot_decode(target_solution, self.num_cols, self.num_cols, 0)
    }
}

#[crate::aggregate_reduction(ilp_feasibility)]
impl crate::rules::AggregateReductionResult for ReductionCOMAToILP {}

#[reduction(transform = {
    exact { num_vars = "num_cols^2 + 2 * num_rows", },
    upper_bound {
        num_constraints = "2 * num_cols + 2 * num_rows * num_cols + 1",
        num_nonzeros = "2 * num_cols^2 + 2 * num_rows * num_cols^2 + 2 * num_rows",
        max_constraint_magnitude_bits = "2 * num_rows * num_cols + num_cols + 1",
    },
})]
impl ReduceTo<ILP<i64, i64, Bounded>> for ConsecutiveOnesMatrixAugmentation {
    type Result = ReductionCOMAToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let m = self.num_rows();
        let n = self.num_cols();

        let overflow = || {
            crate::rules::ReductionError::integer_overflow::<Self, ILP<i64, i64, Bounded>>(
                "counting matrix interval variables",
            )
        };
        let square = n.checked_mul(n).ok_or_else(overflow)?;
        let num_vars = m
            .checked_mul(2)
            .and_then(|count| square.checked_add(count))
            .ok_or_else(overflow)?;
        Self::exact_i64(square, "bounding permutation row arithmetic")?;
        let cells = Self::exact_i64(
            m.checked_mul(n).ok_or_else(overflow)?,
            "counting matrix cells",
        )?;
        let last_position = Self::exact_i64(n.saturating_sub(1), "bounding interval endpoints")?;
        let mut variables = vec![IntegerVariable::binary(); square];
        let mut constraints = one_hot_assignment_constraints(n, n, 0);
        let mut budget = Vec::new();
        let mut total_ones = 0_i64;
        let mut active_rows = 0_i64;
        for (r, row) in self.matrix().iter().enumerate() {
            let nonempty = row.iter().any(|&value| value);
            let endpoint =
                IntegerVariable::new(Some(0), Some(if nonempty { last_position } else { 0 }))
                    .map_err(Self::target_construction)?;
            variables.extend([endpoint, endpoint]);
            let left = square + 2 * r;
            let right = left + 1;
            for (c, &one) in row.iter().enumerate() {
                if !one {
                    continue;
                }
                total_ones += 1;
                let mut after_left = vec![(left, -1)];
                let mut before_right = vec![(right, 1)];
                for p in 1..n {
                    let coefficient = Self::exact_i64(p, "encoding a column position")?;
                    after_left.push((c * n + p, coefficient));
                    before_right.push((c * n + p, -coefficient));
                }
                constraints.push(LinearConstraint::ge(after_left, 0));
                constraints.push(LinearConstraint::ge(before_right, 0));
            }
            if nonempty {
                active_rows += 1;
                budget.extend([(left, -1), (right, 1)]);
            }
        }
        // No ordering can fill more than the matrix's total number of zeros.
        // Clipping before addition keeps even i64::MAX budgets representable.
        let rhs = self.bound().min(cells - total_ones) + total_ones - active_rows;
        constraints.push(LinearConstraint::le(budget, rhs));
        debug_assert_eq!(variables.len(), num_vars);
        let target = ILP::with_variables(variables, constraints, vec![], ObjectiveSense::Minimize)
            .map_err(Self::target_construction)?;
        Ok(ReductionCOMAToILP {
            target,
            num_cols: n,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "consecutiveonesmatrixaugmentation_to_ilp",
        build: || {
            crate::example_db::specs::rule_example_via_bounded_ilp(
                ConsecutiveOnesMatrixAugmentation::new(
                    vec![vec![true, false, true], vec![false, true, true]],
                    1,
                ),
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/consecutiveonesmatrixaugmentation_ilp.rs"]
mod tests;
