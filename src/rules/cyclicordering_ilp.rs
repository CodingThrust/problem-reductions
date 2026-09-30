//! Sparse rank comparisons for cyclic ordering triples.

use crate::models::algebraic::{Bounded, IntegerVariable, LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::CyclicOrdering;
use crate::rules::ilp_helpers::{bounded_order_comparison, ranks_to_positions};
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionCyclicOrderingToILP {
    target: ILP<i64, i64, Bounded>,
    num_elements: usize,
}

impl ReductionResult for ReductionCyclicOrderingToILP {
    type Source = CyclicOrdering;
    type Target = ILP<i64, i64, Bounded>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, solution: &Vec<i64>) -> crate::rules::ExtractionResult<Vec<usize>> {
        crate::rules::traits::validate_target_witness(
            &self.target,
            solution,
            |value| value.value.is_some(),
            "target ILP assignment is infeasible",
        )?;
        Ok(ranks_to_positions(&solution[..self.num_elements]))
    }
}

#[crate::aggregate_reduction(ilp_feasibility)]
impl crate::rules::AggregateReductionResult for ReductionCyclicOrderingToILP {}

#[crate::reduction(transform = {
    exact {
        num_vars = "num_elements + 3 * num_triples",
        num_constraints = "7 * num_triples",
        num_nonzeros = "21 * num_triples",
    },
    upper_bound { max_constraint_magnitude_bits = "num_elements", },
})]
impl ReduceTo<ILP<i64, i64, Bounded>> for CyclicOrdering {
    type Result = ReductionCyclicOrderingToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_elements();
        let count = self
            .num_triples()
            .checked_mul(3)
            .and_then(|v| n.checked_add(v))
            .ok_or_else(|| {
                crate::rules::ReductionError::integer_overflow::<Self, ILP<i64, i64, Bounded>>(
                    "counting cyclic-ordering variables",
                )
            })?;
        let bound = Self::exact_i64(n, "bounding ordering ranks")?;
        let mut variables = vec![
            IntegerVariable::new(Some(0), Some(bound - 1))
                .map_err(Self::target_construction)?;
            n
        ];
        variables.resize(count, IntegerVariable::binary());
        let mut constraints = Vec::new();
        for (index, &(a, b, c)) in self.triples().iter().enumerate() {
            let base = n + 3 * index;
            for (offset, (u, v)) in [(a, b), (b, c), (c, a)].into_iter().enumerate() {
                constraints.extend(bounded_order_comparison(u, v, base + offset, bound));
            }
            constraints.push(LinearConstraint::eq(
                (base..base + 3).map(|i| (i, 1)).collect(),
                2,
            ));
        }
        let target = ILP::with_variables(variables, constraints, vec![], ObjectiveSense::Minimize)
            .map_err(Self::target_construction)?;
        crate::rules::ilp_helpers::validate_bounded_constraint_arithmetic::<Self>(&target)?;
        Ok(Self::Result {
            target,
            num_elements: n,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "cyclicordering_to_ilp",
        build: || {
            crate::example_db::specs::rule_example_via_bounded_ilp(CyclicOrdering::new(
                3,
                vec![(0, 1, 2)],
            ))
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/cyclicordering_ilp.rs"]
mod tests;
