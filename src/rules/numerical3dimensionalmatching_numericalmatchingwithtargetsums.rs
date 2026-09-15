//! Reduction from Numerical 3-Dimensional Matching to Numerical Matching with Target Sums.
//!
//! Given N3DM sets W, X, Y with bound B, keep X and Y unchanged and absorb each
//! w_i into a target sum B - s(w_i). The only implementation-specific caveat is
//! that NMTS stores signed `i64` sizes, so copied X/Y sizes and complements must
//! fit in `i64`.

use crate::models::misc::{Numerical3DimensionalMatching, NumericalMatchingWithTargetSums};
use crate::reduction;
use crate::rules::traits::{recover_preserving_status, ReduceTo, ReductionResult};
use crate::solvers::ProblemOutcome;

/// Result of reducing Numerical3DimensionalMatching to NumericalMatchingWithTargetSums.
#[derive(Debug, Clone)]
pub struct ReductionN3DMToNMTS {
    target: NumericalMatchingWithTargetSums,
}

impl ReductionResult for ReductionN3DMToNMTS {
    type Source = Numerical3DimensionalMatching;
    type Target = NumericalMatchingWithTargetSums;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn recover_result(
        &self,
        source: &Self::Source,
        target: ProblemOutcome<Self::Target>,
    ) -> crate::rules::ExtractionResult<ProblemOutcome<Self::Source>> {
        recover_preserving_status(source, target, |solution| self.map_solution(solution))
    }
}

impl ReductionN3DMToNMTS {
    fn map_solution(
        &self,
        target_solution: &<<Self as ReductionResult>::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<
        <<Self as ReductionResult>::Source as crate::traits::Problem>::Solution,
    > {
        Ok({
            let mut pairs: Vec<_> = target_solution
                .iter()
                .enumerate()
                .map(|(x, &y)| (self.target.sizes_x()[x] + self.target.sizes_y()[y], x, y))
                .collect();
            let mut targets: Vec<_> = self.target.targets().iter().copied().enumerate().collect();
            pairs.sort_unstable();
            targets.sort_unstable_by_key(|&(w, sum)| (sum, w));

            // Feasibility equates the pair-sum and target multisets. Target
            // index w retains the source W order from construction.
            let mut x_perm = vec![0; targets.len()];
            let mut y_perm = vec![0; targets.len()];
            for ((w, _), (_, x, y)) in targets.into_iter().zip(pairs) {
                x_perm[w] = x;
                y_perm[w] = y;
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
    transform = exact {
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

        Ok(ReductionN3DMToNMTS { target })
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
                    source_config: serde_json::json!(vec![0, 1, 1, 0]),
                    target_config: serde_json::json!(vec![1, 0]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/numerical3dimensionalmatching_numericalmatchingwithtargetsums.rs"]
mod tests;
