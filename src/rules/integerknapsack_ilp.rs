//! Reduction from IntegerKnapsack to `ILP<i64, i64, Bounded>`.
//!
//! Each item multiplicity becomes a non-negative integer ILP variable. The
//! capacity inequality omits oversized items (whose multiplicities are zero), and explicit upper bounds
//! `c_i <= floor(B / s_i)` preserve the exact witness domain of the source.

use crate::models::algebraic::{Bounded, IntegerVariable, LinearConstraint, ObjectiveSense, ILP};
use crate::models::set::IntegerKnapsack;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionIntegerKnapsackToILP {
    target: ILP<i64, i64, Bounded>,
}

impl ReductionResult for ReductionIntegerKnapsackToILP {
    type Source = IntegerKnapsack;
    type Target = ILP<i64, i64, Bounded>;

    fn target_problem(&self) -> &ILP<i64, i64, Bounded> {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        crate::rules::ilp_helpers::decode_usize_values(target_solution)
    }
}

#[reduction(transform = {
    exact {
        num_vars = "num_items",
        num_constraints = "num_items + 1",
    },
    upper_bound {
        max_constraint_magnitude_bits = "capacity + 1",
        num_nonzeros = "2 * num_items",
    },
})]
impl ReduceTo<ILP<i64, i64, Bounded>> for IntegerKnapsack {
    type Result = ReductionIntegerKnapsackToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let num_vars = self.num_items();
        let mut constraints = Vec::with_capacity(num_vars + 1);
        let sizes = self.sizes();
        let values = self.values();

        constraints.push(LinearConstraint::le(
            sizes
                .iter()
                .enumerate()
                // Oversized items already have multiplicity fixed to zero by their bounds.
                .filter(|&(_, &size)| size <= self.capacity())
                .map(|(item, &size)| (item, size))
                .collect(),
            self.capacity(),
        ));

        for (i, &size) in self.sizes().iter().enumerate() {
            let upper_bound = self.capacity() / size;
            constraints.push(LinearConstraint::le(vec![(i, 1)], upper_bound));
        }

        let objective = values.iter().copied().enumerate().collect();

        let variables = self
            .sizes()
            .iter()
            .map(|&size| IntegerVariable::new(Some(0), Some(self.capacity() / size)))
            .collect::<Result<Vec<_>, _>>()
            .map_err(Self::target_construction)?;

        Ok(ReductionIntegerKnapsackToILP {
            target: ILP::with_variables(
                variables,
                constraints,
                objective,
                ObjectiveSense::Maximize,
            )
            .map_err(Self::target_construction)?,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "integerknapsack_to_ilp",
        build: || {
            let source = IntegerKnapsack::new(vec![3, 4, 5], vec![4, 5, 7], 10).unwrap();
            crate::example_db::specs::rule_example_via_bounded_ilp::<_>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/integerknapsack_ilp.rs"]
mod tests;
