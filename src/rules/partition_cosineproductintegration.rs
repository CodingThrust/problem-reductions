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

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(overhead = {
    num_coefficients = "num_elements",
})]
impl ReduceTo<CosineProductIntegration> for Partition {
    type Result = ReductionPartitionToCPI;

    fn reduce_to(&self) -> Self::Result {
        let coefficients: Vec<i64> = self.sizes().iter().map(|&s| s as i64).collect();
        ReductionPartitionToCPI {
            target: CosineProductIntegration::new(coefficients),
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "partition_to_cosineproductintegration",
        build: || {
            // sizes [3, 1, 1, 2, 2, 1]: partition {3,2,1}={6} and {1,2,1}={4}? No...
            // Actually [3,1,1,2,2,1] sum=10, need sum=5 each.
            // config [1,0,0,1,0,0] → selected={3,2}=5, rest={1,1,2,1}=5 ✓
            // sign assignment: bit=1→−, bit=0→+ : (+3,−1,−1,+2,−2,−1) = 3-1-1+2-2-1=0? No, 3-1-1+2-2-1=0. Yes!
            // Wait: config [1,0,0,1,0,0] means elements 0,3 in subset 1.
            // For CPI: bit 1 means −a_i. So −3+1+1−2+2+1 = 0. Yes!
            crate::example_db::specs::rule_example_with_witness::<_, CosineProductIntegration>(
                Partition::new(vec![3, 1, 1, 2, 2, 1]),
                SolutionPair {
                    source_config: vec![1, 0, 0, 1, 0, 0],
                    target_config: vec![1, 0, 0, 1, 0, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/partition_cosineproductintegration.rs"]
mod tests;
