//! Reduction from ExactCoverBy3Sets to AlgebraicEquationsOverGF2.

use crate::models::algebraic::AlgebraicEquationsOverGF2;
use crate::models::set::ExactCoverBy3Sets;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionX3CToAlgebraicEquationsOverGF2 {
    target: AlgebraicEquationsOverGF2,
}

impl ReductionResult for ReductionX3CToAlgebraicEquationsOverGF2 {
    type Source = ExactCoverBy3Sets;
    type Target = AlgebraicEquationsOverGF2;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(overhead = {
    num_vars = "num_sets",
})]
impl ReduceTo<AlgebraicEquationsOverGF2> for ExactCoverBy3Sets {
    type Result = ReductionX3CToAlgebraicEquationsOverGF2;

    fn reduce_to(&self) -> Self::Result {
        let mut sets_per_element = vec![Vec::new(); self.universe_size()];
        for (set_index, set) in self.sets().iter().enumerate() {
            for &element in set {
                sets_per_element[element].push(set_index);
            }
        }

        let mut equations = Vec::new();
        for containing_sets in sets_per_element {
            let mut linear_equation = containing_sets
                .iter()
                .map(|&set_index| vec![set_index])
                .collect::<Vec<_>>();
            linear_equation.push(vec![]);
            equations.push(linear_equation);

            for left in 0..containing_sets.len() {
                for right in (left + 1)..containing_sets.len() {
                    equations.push(vec![vec![containing_sets[left], containing_sets[right]]]);
                }
            }
        }

        ReductionX3CToAlgebraicEquationsOverGF2 {
            target: AlgebraicEquationsOverGF2::new(self.num_sets(), equations)
                .expect("reduction produces valid equations"),
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "exactcoverby3sets_to_algebraicequationsovergf2",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, AlgebraicEquationsOverGF2>(
                ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [3, 4, 5], [0, 3, 4]]),
                SolutionPair {
                    source_config: vec![1, 1, 0],
                    target_config: vec![1, 1, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/exactcoverby3sets_algebraicequationsovergf2.rs"]
mod tests;
