use crate::models::algebraic::IntegerExpressionMembership;
use crate::models::misc::SubsetSum;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use num_traits::ToPrimitive;

#[derive(Debug, Clone)]
pub struct ReductionSubsetSumToIntegerExpressionMembership {
    target: IntegerExpressionMembership,
}

impl ReductionResult for ReductionSubsetSumToIntegerExpressionMembership {
    type Source = SubsetSum;
    type Target = IntegerExpressionMembership;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution
            .iter()
            .map(|&choice_index| usize::from(choice_index == 1))
            .collect()
    }
}

#[reduction(overhead = {
    num_positions = "num_elements",
})]
impl ReduceTo<IntegerExpressionMembership> for SubsetSum {
    type Result = ReductionSubsetSumToIntegerExpressionMembership;

    fn reduce_to(&self) -> Self::Result {
        let choices = self
            .sizes()
            .iter()
            .map(|size| {
                let size = size.to_u64().unwrap();
                vec![
                    1,
                    size.checked_add(1).expect(
                        "SubsetSum -> IntegerExpressionMembership requires shifted values to fit in u64",
                    ),
                ]
            })
            .collect();
        let shift = u64::try_from(self.num_elements())
            .expect("SubsetSum -> IntegerExpressionMembership requires num_elements to fit in u64");
        let target = self.target().to_u64().unwrap().checked_add(shift).expect(
            "SubsetSum -> IntegerExpressionMembership requires shifted target to fit in u64",
        );

        ReductionSubsetSumToIntegerExpressionMembership {
            target: IntegerExpressionMembership::new(choices, target),
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "subsetsum_to_integerexpressionmembership",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, IntegerExpressionMembership>(
                SubsetSum::new(vec![1u32, 5, 6, 8], 11u32),
                SolutionPair {
                    source_config: vec![0, 1, 1, 0],
                    target_config: vec![0, 1, 1, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/subsetsum_integerexpressionmembership.rs"]
mod tests;
