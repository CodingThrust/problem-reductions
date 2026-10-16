//! Reduction from Partition to CosineProductIntegration.
//!
//! Given a Partition instance with sizes `[s_1, ..., s_n]`, construct a
//! CosineProductIntegration instance with coefficients `[s_1, ..., s_n]`
//! (cast from `u64` to `i64`).
//!
//! A balanced partition exists iff a balanced sign assignment exists:
//! subset A has sum = total/2 iff the sign vector `ε_i = +1` for `i ∈ A`,
//! `ε_i = -1` for `i ∉ A` satisfies `∑ ε_i s_i = 0`.
//!
//! Solution extraction is the identity mapping.

use crate::models::misc::{CosineProductIntegration, Partition};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing Partition to CosineProductIntegration.
#[derive(Debug, Clone)]
pub struct ReductionPartitionToCPI {
    target: CosineProductIntegration,
}

impl ReductionResult for ReductionPartitionToCPI {
    type Source = Partition;
    type Target = CosineProductIntegration;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_witness(
            self.target_problem(),
            target_solution,
            |value| value.0,
            "target witness does not satisfy the target problem",
        )?;

        Ok(target_solution.to_vec())
    }
}

#[crate::aggregate_reduction(identity)]
impl crate::rules::AggregateReductionResult for ReductionPartitionToCPI {}

#[reduction(
    transform = exact {
        num_coefficients = "num_elements",
    })]
impl ReduceTo<CosineProductIntegration> for Partition {
    type Result = ReductionPartitionToCPI;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let coefficients = self.sizes().to_vec();
        Ok(ReductionPartitionToCPI {
            target: CosineProductIntegration::new(coefficients),
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "partition_to_cosineproductintegration",
        build: || {
            // Selecting {3, 2} splits the total into 5 + 5; with true denoting
            // a negative sign, the target sum is -3+1+1-2+2+1 = 0.
            crate::example_db::specs::rule_example_with_witness::<_, CosineProductIntegration>(
                Partition::new(vec![3, 1, 1, 2, 2, 1]).unwrap(),
                SolutionPair {
                    source_config: serde_json::json!(vec![true, false, false, true, false, false]),
                    target_config: serde_json::json!(vec![true, false, false, true, false, false]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/partition_cosineproductintegration.rs"]
mod tests;
