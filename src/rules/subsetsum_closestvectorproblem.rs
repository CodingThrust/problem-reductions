//! Reduction from Subset Sum to Closest Vector Problem.

use crate::models::algebraic::ClosestVectorProblem;
use crate::models::misc::SubsetSum;
use crate::reduction;
use crate::registry::ConstructionError;
use crate::rules::traits::{ReduceTo, ReductionResult};
use num_traits::ToPrimitive;

/// Result of reducing SubsetSum to ClosestVectorProblem.
#[derive(Debug, Clone)]
pub struct ReductionSubsetSumToClosestVectorProblem {
    target: ClosestVectorProblem<i64>,
}

impl ReductionResult for ReductionSubsetSumToClosestVectorProblem {
    type Source = SubsetSum;
    type Target = ClosestVectorProblem<i64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok(target_solution.iter().map(|&value| value == 1).collect())
    }
}

#[reduction(
    transform = exact {
        ambient_dimension = "num_elements + 1",
        num_basis_vectors = "num_elements",
    },
)]
impl ReduceTo<ClosestVectorProblem<i64>> for SubsetSum {
    type Result = ReductionSubsetSumToClosestVectorProblem;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_elements();
        let mut basis = Vec::with_capacity(n);
        for (i, size) in self.sizes().iter().enumerate() {
            let mut column = vec![0i64; n + 1];
            column[i] = 2;
            let size = size.to_i64().ok_or_else(|| {
                crate::rules::ReductionError::construction::<SubsetSum, ClosestVectorProblem<i64>>(
                    ConstructionError::IntegerOverflow(
                        "an item size does not fit the ClosestVectorProblem i64 domain".into(),
                    ),
                )
            })?;
            column[n] =
                size.checked_mul(2).ok_or_else(|| {
                    crate::rules::ReductionError::integer_overflow::<
                        SubsetSum,
                        ClosestVectorProblem<i64>,
                    >("scaling a Subset Sum item size")
                })?;
            basis.push(column);
        }

        let mut target = vec![1_i64; n];
        let target_sum = self.target().to_i64().ok_or_else(|| {
            crate::rules::ReductionError::construction::<SubsetSum, ClosestVectorProblem<i64>>(
                ConstructionError::IntegerOverflow(
                    "the target sum does not fit the ClosestVectorProblem i64 domain".into(),
                ),
            )
        })?;
        target.push(target_sum.checked_mul(2).ok_or_else(|| {
            crate::rules::ReductionError::integer_overflow::<SubsetSum, ClosestVectorProblem<i64>>(
                "scaling the Subset Sum target",
            )
        })?);

        Ok(ReductionSubsetSumToClosestVectorProblem {
            target: ClosestVectorProblem::new(basis, target).map_err(|error| {
                crate::rules::ReductionError::construction::<SubsetSum, ClosestVectorProblem<i64>>(
                    error,
                )
            })?,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "subsetsum_to_closestvectorproblem",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, ClosestVectorProblem<i64>>(
                SubsetSum::new(vec![3u32, 7, 1, 8], 11u32),
                SolutionPair {
                    source_config: serde_json::json!(vec![true, false, false, true]),
                    target_config: serde_json::json!(vec![1, 0, 0, 1]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/subsetsum_closestvectorproblem.rs"]
mod tests;
