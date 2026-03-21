//! Reduction from Subset Sum to Closest Vector Problem.

use crate::models::algebraic::{ClosestVectorProblem, VarBounds};
use crate::models::misc::SubsetSum;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use num_bigint::BigUint;
use num_traits::ToPrimitive;

/// Result of reducing SubsetSum to ClosestVectorProblem.
#[derive(Debug, Clone)]
pub struct ReductionSubsetSumToClosestVectorProblem {
    target: ClosestVectorProblem<i32>,
}

impl ReductionResult for ReductionSubsetSumToClosestVectorProblem {
    type Source = SubsetSum;
    type Target = ClosestVectorProblem<i32>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

fn biguint_to_i32(value: &BigUint) -> i32 {
    value
        .to_i32()
        .expect("SubsetSum -> ClosestVectorProblem requires all sizes and target to fit in i32")
}

#[reduction(
    overhead = {
        ambient_dimension = "num_elements + 1",
        num_basis_vectors = "num_elements",
    }
)]
impl ReduceTo<ClosestVectorProblem<i32>> for SubsetSum {
    type Result = ReductionSubsetSumToClosestVectorProblem;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_elements();
        let mut basis = Vec::with_capacity(n);
        for (i, size) in self.sizes().iter().enumerate() {
            let mut column = vec![0i32; n + 1];
            column[i] = 1;
            column[n] = biguint_to_i32(size);
            basis.push(column);
        }

        let mut target = vec![0.5; n];
        target.push(biguint_to_i32(self.target()) as f64);

        ReductionSubsetSumToClosestVectorProblem {
            target: ClosestVectorProblem::new(basis, target, vec![VarBounds::binary(); n]),
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "subsetsum_to_closestvectorproblem",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, ClosestVectorProblem<i32>>(
                SubsetSum::new(vec![3u32, 7, 1, 8], 11u32),
                SolutionPair {
                    source_config: vec![1, 0, 0, 1],
                    target_config: vec![1, 0, 0, 1],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/subsetsum_closestvectorproblem.rs"]
mod tests;
