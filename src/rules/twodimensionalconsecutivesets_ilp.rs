//! Distinct labels within a subset occupy an interval of that subset's size.

use crate::models::algebraic::{Bounded, IntegerVariable, LinearConstraint, ObjectiveSense, ILP};
use crate::models::set::TwoDimensionalConsecutiveSets;
use crate::rules::ilp_helpers::{bounded_order_comparison, decode_usize_values};
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionTwoDimensionalConsecutiveSetsToILP {
    target: ILP<i64, i64, Bounded>,
    alphabet_size: usize,
}
impl ReductionResult for ReductionTwoDimensionalConsecutiveSetsToILP {
    type Source = TwoDimensionalConsecutiveSets;
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
        decode_usize_values(&solution[..self.alphabet_size])
    }
}
#[crate::aggregate_reduction(ilp_feasibility)]
impl crate::rules::AggregateReductionResult for ReductionTwoDimensionalConsecutiveSetsToILP {}

// P=sum_S choose(|S|,2) <= num_subsets * choose(alphabet_size,2).
#[crate::reduction(transform = upper_bound {
    num_vars = "alphabet_size + num_subsets * alphabet_size * (alphabet_size - 1) / 2",
    num_constraints = "2 * num_subsets * alphabet_size * (alphabet_size - 1)",
    num_nonzeros = "5 * num_subsets * alphabet_size * (alphabet_size - 1)",
    max_constraint_magnitude_bits = "alphabet_size",
})]
impl ReduceTo<ILP<i64, i64, Bounded>> for TwoDimensionalConsecutiveSets {
    type Result = ReductionTwoDimensionalConsecutiveSetsToILP;
    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.alphabet_size();
        let bound = Self::exact_i64(n, "bounding group labels")?;
        let mut variables = vec![
            IntegerVariable::new(Some(0), Some(bound - 1))
                .map_err(Self::target_construction)?;
            n
        ];
        let mut constraints = Vec::new();
        for subset in self.subsets() {
            let width = Self::exact_i64(subset.len().saturating_sub(1), "bounding subset span")?;
            for (i, &u) in subset.iter().enumerate() {
                for &v in &subset[i + 1..] {
                    let selector = variables.len();
                    variables.push(IntegerVariable::binary());
                    constraints.extend(bounded_order_comparison(u, v, selector, bound));
                    constraints.push(LinearConstraint::le(vec![(v, 1), (u, -1)], width));
                    constraints.push(LinearConstraint::le(vec![(u, 1), (v, -1)], width));
                }
            }
        }
        let target = ILP::with_variables(variables, constraints, vec![], ObjectiveSense::Minimize)
            .map_err(Self::target_construction)?;
        crate::rules::ilp_helpers::validate_bounded_constraint_arithmetic::<Self>(&target)?;
        Ok(Self::Result {
            target,
            alphabet_size: n,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "twodimensionalconsecutivesets_to_ilp",
        build: || {
            crate::example_db::specs::rule_example_via_bounded_ilp(
                TwoDimensionalConsecutiveSets::new(3, vec![vec![0, 1], vec![1, 2]]),
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/twodimensionalconsecutivesets_ilp.rs"]
mod tests;
