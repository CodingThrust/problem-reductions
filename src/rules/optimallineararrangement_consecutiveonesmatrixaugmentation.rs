//! Incidence-matrix reduction from decision linear arrangement to consecutive
//! ones augmentation (Booth, 1975, Theorem 4.19, with the matrix transposed).
//! For every ordering, augmentation cost equals total edge length minus the
//! number of non-loop edges. Loops contribute zero to both costs; parallel
//! edges contribute separately.

use crate::models::algebraic::ConsecutiveOnesMatrixAugmentation;
use crate::models::decision::Decision;
use crate::models::graph::OptimalLinearArrangement;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// The target incidence matrix, or a fixed infeasible matrix when the source
/// bound is below the universal lower bound on arrangement length.
#[derive(Debug, Clone)]
pub struct ReductionOptimalLinearArrangementToConsecutiveOnesMatrixAugmentation {
    target: ConsecutiveOnesMatrixAugmentation,
}

impl ReductionResult for ReductionOptimalLinearArrangementToConsecutiveOnesMatrixAugmentation {
    type Source = Decision<OptimalLinearArrangement<SimpleGraph>>;
    type Target = ConsecutiveOnesMatrixAugmentation;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        let value =
            crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;
        if !value.0 {
            return Err(crate::rules::ExtractionError::invalid(
                "target column order is not a satisfying augmentation certificate",
            ));
        }
        // Validation establishes a permutation within the augmentation budget.
        // The NO sentinel has no such certificate; all remaining columns are
        // source vertices, including the empty permutation for an empty graph.
        let mut arrangement = vec![0; target_solution.len()];
        for (position, &vertex) in target_solution.iter().enumerate() {
            arrangement[vertex] = position;
        }
        Ok(arrangement)
    }
}

#[reduction(
    transform = upper_bound {
        num_rows = "num_edges + 3",
        num_cols = "num_vertices + 3",
    }
)]
impl ReduceTo<ConsecutiveOnesMatrixAugmentation>
    for Decision<OptimalLinearArrangement<SimpleGraph>>
{
    type Result = ReductionOptimalLinearArrangementToConsecutiveOnesMatrixAugmentation;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_vertices();
        let edges = self.inner().graph().edges();
        let non_loops = <Self as ReduceTo<ConsecutiveOnesMatrixAugmentation>>::exact_i64(
            edges.iter().filter(|(u, v)| u != v).count(),
            "converting the number of non-loop edges to i64",
        )?;
        let k = *self.bound();
        let (matrix, bound) = if k < non_loops {
            // Every non-loop edge has length at least one. The three pairs of
            // columns cannot all be adjacent, so this matrix is genuinely NO.
            (
                vec![
                    vec![true, true, false],
                    vec![false, true, true],
                    vec![true, false, true],
                ],
                0,
            )
        } else {
            // A zero row preserves the vertex columns when there are no edges.
            // An empty graph is represented by one empty row, with no columns.
            let mut matrix = vec![vec![false; n]; edges.len().max(1)];
            for (row, (u, v)) in edges.into_iter().enumerate() {
                matrix[row][u] = true;
                matrix[row][v] = true;
            }
            // 0 <= non_loops <= k <= i64::MAX, so subtraction is exact.
            (matrix, k - non_loops)
        };
        Ok(
            ReductionOptimalLinearArrangementToConsecutiveOnesMatrixAugmentation {
                target: ConsecutiveOnesMatrixAugmentation::try_new(matrix, bound).map_err(
                    <Self as ReduceTo<ConsecutiveOnesMatrixAugmentation>>::target_construction,
                )?,
            },
        )
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "optimallineararrangement_to_consecutiveonesmatrixaugmentation",
        build: || {
            use crate::example_db::specs::assemble_rule_example;

            // 6 vertices, 7 edges (path + two chords). Optimal arrangement
            // [0,1,2,3,4,5] has total edge length 11; with k = 11 the target
            // bound is 11 - 7 = 4 and the identity column permutation works.
            let source = Decision::new(
                OptimalLinearArrangement::new(SimpleGraph::new(
                    6,
                    vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (0, 3), (2, 5)],
                )),
                11,
            );
            let reduction = ReduceTo::<ConsecutiveOnesMatrixAugmentation>::reduce_to(&source)
                .expect("reduction should succeed");
            // Source arrangement f(v) = v <=> target column permutation = identity.
            let source_config = vec![0, 1, 2, 3, 4, 5];
            let target_config = vec![0, 1, 2, 3, 4, 5];
            assemble_rule_example(
                &source,
                reduction.target_problem(),
                vec![SolutionPair {
                    source_config: serde_json::to_value(source_config)
                        .expect("solution serialization must succeed"),
                    target_config: serde_json::to_value(target_config)
                        .expect("solution serialization must succeed"),
                }],
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/optimallineararrangement_consecutiveonesmatrixaugmentation.rs"]
mod tests;
