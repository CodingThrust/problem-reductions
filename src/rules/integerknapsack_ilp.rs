//! Reduction from IntegerKnapsack to ILP<i32>.
//!
//! Each item multiplicity becomes a non-negative integer ILP variable. The
//! capacity inequality is kept directly, and explicit upper bounds
//! `c_i <= floor(B / s_i)` preserve the exact witness domain of the source.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::set::IntegerKnapsack;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionIntegerKnapsackToILP {
    target: ILP<i32>,
}

impl ReductionResult for ReductionIntegerKnapsackToILP {
    type Source = IntegerKnapsack;
    type Target = ILP<i32>;

    fn target_problem(&self) -> &ILP<i32> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(
    overhead = {
        num_vars = "num_items",
        num_constraints = "num_items + 1",
    }
)]
impl ReduceTo<ILP<i32>> for IntegerKnapsack {
    type Result = ReductionIntegerKnapsackToILP;

    fn reduce_to(&self) -> Self::Result {
        let num_vars = self.num_items();
        let mut constraints = Vec::with_capacity(num_vars + 1);

        constraints.push(LinearConstraint::le(
            self.sizes()
                .iter()
                .enumerate()
                .map(|(i, &size)| (i, size as f64))
                .collect(),
            self.capacity() as f64,
        ));

        for (i, &size) in self.sizes().iter().enumerate() {
            let upper_bound = self.capacity() / size;
            assert!(
                upper_bound <= i32::MAX as i64,
                "IntegerKnapsack -> ILP requires multiplicity bounds to fit in ILP<i32> variable bounds"
            );
            constraints.push(LinearConstraint::le(vec![(i, 1.0)], upper_bound as f64));
        }

        let objective = self
            .values()
            .iter()
            .enumerate()
            .map(|(i, &value)| (i, value as f64))
            .collect();

        ReductionIntegerKnapsackToILP {
            target: ILP::new(num_vars, constraints, objective, ObjectiveSense::Maximize),
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "integerknapsack_to_ilp",
        build: || {
            let source = IntegerKnapsack::new(vec![3, 4, 5], vec![4, 5, 7], 10);
            crate::example_db::specs::rule_example_via_ilp::<_, i32>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/integerknapsack_ilp.rs"]
mod tests;
