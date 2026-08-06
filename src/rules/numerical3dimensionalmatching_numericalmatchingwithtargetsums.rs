//! Reduction from Numerical 3-Dimensional Matching to Numerical Matching with Target Sums.
//!
//! Given N3DM sets W, X, Y with bound B, keep X and Y unchanged and absorb each
//! w_i into a target sum B - s(w_i). The only implementation-specific caveat is
//! that NMTS stores signed `i64` sizes, so copied X/Y sizes and complements must
//! fit in `i64`.

use crate::models::misc::{Numerical3DimensionalMatching, NumericalMatchingWithTargetSums};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use std::collections::BTreeMap;

/// Result of reducing Numerical3DimensionalMatching to NumericalMatchingWithTargetSums.
#[derive(Debug, Clone)]
pub struct ReductionN3DMToNMTS {
    target: NumericalMatchingWithTargetSums,
    source_sizes_w: Vec<u64>,
    source_bound: u64,
}

impl ReductionResult for ReductionN3DMToNMTS {
    type Source = Numerical3DimensionalMatching;
    type Target = NumericalMatchingWithTargetSums;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &[usize],
    ) -> crate::rules::ExtractionResult<Vec<usize>> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok({
            let mut x_indices_by_pair_sum: BTreeMap<i64, Vec<usize>> = BTreeMap::new();
            for (x_index, &y_index) in target_solution.iter().enumerate() {
                let pair_sum = self.target.sizes_x()[x_index]
                    .checked_add(self.target.sizes_y()[y_index])
                    .ok_or_else(|| {
                        crate::rules::ExtractionError::invalid(
                            "target pair sum overflows the target numeric domain",
                        )
                    })?;
                x_indices_by_pair_sum
                    .entry(pair_sum)
                    .or_default()
                    .push(x_index);
            }

            let mut x_perm = Vec::with_capacity(self.source_sizes_w.len());
            let mut y_perm = Vec::with_capacity(self.source_sizes_w.len());
            for &w_size in &self.source_sizes_w {
                let target_sum = checked_target_sum_to_i64(self.source_bound, w_size);
                let x_index = x_indices_by_pair_sum
                    .get_mut(&target_sum)
                    .and_then(Vec::pop)
                    .ok_or_else(|| {
                        crate::rules::ExtractionError::invalid(format!(
                            "target matching does not realize required pair sum {target_sum}"
                        ))
                    })?;
                x_perm.push(x_index);
                y_perm.push(target_solution[x_index]);
            }

            x_perm.extend(y_perm);
            x_perm
        })
    }
}

fn checked_size_to_i64(size: u64) -> i64 {
    i64::try_from(size).expect(
        "Numerical3DimensionalMatching -> NumericalMatchingWithTargetSums requires X/Y sizes to fit in i64",
    )
}

fn checked_target_sum_to_i64(bound: u64, w_size: u64) -> i64 {
    let target_sum = bound
        .checked_sub(w_size)
        .expect("N3DM invariants require each w_i to be strictly smaller than B");
    i64::try_from(target_sum).expect(
        "Numerical3DimensionalMatching -> NumericalMatchingWithTargetSums requires each complement B - s(w_i) to fit in i64",
    )
}

#[reduction(overhead = {
    num_pairs = "num_groups",
})]
impl ReduceTo<NumericalMatchingWithTargetSums> for Numerical3DimensionalMatching {
    type Result = ReductionN3DMToNMTS;

    fn reduce_to(&self) -> Self::Result {
        let target = NumericalMatchingWithTargetSums::new(
            self.sizes_x()
                .iter()
                .copied()
                .map(checked_size_to_i64)
                .collect(),
            self.sizes_y()
                .iter()
                .copied()
                .map(checked_size_to_i64)
                .collect(),
            self.sizes_w()
                .iter()
                .copied()
                .map(|w_size| checked_target_sum_to_i64(self.bound(), w_size))
                .collect(),
        );

        ReductionN3DMToNMTS {
            target,
            source_sizes_w: self.sizes_w().to_vec(),
            source_bound: self.bound(),
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "numerical3dimensionalmatching_to_numericalmatchingwithtargetsums",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, NumericalMatchingWithTargetSums>(
                Numerical3DimensionalMatching::new(vec![4, 5], vec![4, 5], vec![5, 7], 15),
                SolutionPair {
                    source_config: vec![0, 1, 1, 0],
                    target_config: vec![1, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/numerical3dimensionalmatching_numericalmatchingwithtargetsums.rs"]
mod tests;
