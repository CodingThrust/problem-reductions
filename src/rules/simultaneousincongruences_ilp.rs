//! Reduction from SimultaneousIncongruences to a bounded-integer feasibility ILP.
//!
//! For every pair `(a, b)` with `rho = a mod b` the target holds a quotient `q`, a
//! remainder `r` and a binary selector `z`. The division row `x = b q + r` makes `r`
//! the Euclidean remainder of `x`, and the two selector rows are jointly satisfiable
//! exactly when `r != rho`.

use crate::models::algebraic::{
    Bounded, ILP, IntegerVariable, LinearConstraint, ObjectiveSense, SimultaneousIncongruences,
};
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing SimultaneousIncongruences to ILP.
#[derive(Debug, Clone)]
pub struct ReductionSimultaneousIncongruencesToILP {
    target: ILP<i64, i64, Bounded>,
}

impl ReductionResult for ReductionSimultaneousIncongruencesToILP {
    type Source = SimultaneousIncongruences;
    type Target = ILP<i64, i64, Bounded>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, solution: &Vec<i64>) -> crate::rules::ExtractionResult<i64> {
        crate::rules::traits::validate_target_witness(
            &self.target,
            solution,
            |value| value.value.is_some(),
            "target ILP assignment is infeasible",
        )?;
        Ok(solution[0])
    }
}

#[crate::aggregate_reduction(ilp_feasibility)]
impl crate::rules::AggregateReductionResult for ReductionSimultaneousIncongruencesToILP {}

#[crate::reduction(transform = {
    exact {
        num_vars = "1 + 3 * num_pairs",
        num_constraints = "3 * num_pairs",
        num_nonzeros = "7 * num_pairs",
    },
    upper_bound { max_constraint_magnitude_bits = "lcm_bits", },
})]
impl ReduceTo<ILP<i64, i64, Bounded>> for SimultaneousIncongruences {
    type Result = ReductionSimultaneousIncongruencesToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        type Target = ILP<i64, i64, Bounded>;
        let overflow = |operation: &str| {
            crate::rules::ReductionError::integer_overflow::<Self, Target>(operation)
        };
        let pairs = self.pairs();
        let m = pairs.len();
        let count = m
            .checked_mul(3)
            .and_then(|n| n.checked_add(1))
            .ok_or_else(|| overflow("counting target variables"))?;
        let lcm = self.lcm_moduli();
        let mut variables = Vec::with_capacity(count);
        variables
            .push(IntegerVariable::new(Some(0), Some(lcm - 1)).map_err(Self::target_construction)?);
        for &(_, b) in pairs {
            variables.push(
                IntegerVariable::new(Some(0), Some(lcm / b - 1))
                    .map_err(Self::target_construction)?,
            );
        }
        for &(_, b) in pairs {
            variables.push(
                IntegerVariable::new(Some(0), Some(b - 1)).map_err(Self::target_construction)?,
            );
        }
        variables.resize(count, IntegerVariable::binary());

        let mut constraints = Vec::with_capacity(3 * m);
        for (i, &(a, b)) in pairs.iter().enumerate() {
            let rho = a % b;
            let (q, r, z) = (1 + i, 1 + m + i, 1 + 2 * m + i);
            let lower_rhs = (rho - b)
                .checked_add(1)
                .ok_or_else(|| overflow("computing a selector bound"))?;
            constraints.push(LinearConstraint::eq(vec![(0, 1), (q, -b), (r, -1)], 0));
            constraints.push(LinearConstraint::le(vec![(r, 1), (z, -b)], rho - 1));
            constraints.push(LinearConstraint::ge(vec![(r, 1), (z, -b)], lower_rhs));
        }
        let target = ILP::with_variables(variables, constraints, vec![], ObjectiveSense::Minimize)
            .map_err(Self::target_construction)?;
        crate::rules::ilp_helpers::validate_bounded_constraint_arithmetic::<Self>(&target)?;
        Ok(Self::Result { target })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "simultaneousincongruences_to_ilp",
        build: || {
            crate::example_db::specs::rule_example_via_bounded_ilp(
                SimultaneousIncongruences::new(vec![(2, 2), (1, 3), (2, 3)])
                    .expect("canonical pairs are valid"),
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/simultaneousincongruences_ilp.rs"]
mod tests;
