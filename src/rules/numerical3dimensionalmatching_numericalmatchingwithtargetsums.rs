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
    source_sizes_w: Vec<i64>,
    source_bound: i64,
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
                let target_sum = checked_target_sum(self.source_bound, w_size)
                    .map_err(crate::rules::ExtractionError::invalid)?;
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

fn checked_target_sum(bound: i64, w_size: i64) -> Result<i64, &'static str> {
    bound
        .checked_sub(w_size)
        .ok_or("computing a derived target sum overflowed")
}

#[reduction(
    size = exact {
        num_pairs = "num_groups",
    })]
impl ReduceTo<NumericalMatchingWithTargetSums> for Numerical3DimensionalMatching {
    type Result = ReductionN3DMToNMTS;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let map_error = |message| {
            crate::rules::ReductionError::invalid_target::<
                Numerical3DimensionalMatching,
                NumericalMatchingWithTargetSums,
            >(message)
        };
        let target = NumericalMatchingWithTargetSums::new(
            self.sizes_x().to_vec(),
            self.sizes_y().to_vec(),
            self.sizes_w()
                .iter()
                .copied()
                .map(|w_size| checked_target_sum(self.bound(), w_size))
                .collect::<Result<_, _>>()
                .map_err(map_error)?,
        );

        Ok(ReductionN3DMToNMTS {
            target,
            source_sizes_w: self.sizes_w().to_vec(),
            source_bound: self.bound(),
        })
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
