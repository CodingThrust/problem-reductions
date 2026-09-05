//! Reduction from MinimumVertexCover (unit-weight) to LongestCommonSubsequence.

use crate::models::graph::MinimumVertexCover;
use crate::models::misc::LongestCommonSubsequence;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::types::One;

#[derive(Debug, Clone)]
pub struct ReductionVCToLCS {
    target: LongestCommonSubsequence,
    num_vertices: usize,
}

impl ReductionResult for ReductionVCToLCS {
    type Source = MinimumVertexCover<SimpleGraph, One>;
    type Target = LongestCommonSubsequence;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok({
            let mut cover = vec![true; self.num_vertices];
            for &symbol in target_solution {
                let Some(symbol) = symbol else { break };
                cover[symbol] = false;
            }
            cover
        })
    }
}

#[reduction(
    transform = exact {
        alphabet_size = "num_vertices",
        num_strings = "num_edges + 1",
        max_length = "num_vertices",
        total_length = "num_vertices + 2 * num_edges * num_vertices - 2 * num_edges",
    },
    unavailable = {
        cross_frequency_product = "the exact target parameter is not represented by this reduction's symbolic transform",
        num_transitions = "the exact target parameter is not represented by this reduction's symbolic transform",
        sum_triangular_lengths = "the exact target parameter is not represented by this reduction's symbolic transform",
    }
)]
impl ReduceTo<LongestCommonSubsequence> for MinimumVertexCover<SimpleGraph, One> {
    type Result = ReductionVCToLCS;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let num_vertices = self.graph().num_vertices();
        let mut strings = Vec::with_capacity(self.graph().num_edges() + 1);
        strings.push((0..num_vertices).collect());

        for (left, right) in self.graph().edges() {
            // The backward direction relies on each edge string forcing the
            // larger endpoint to appear before the smaller one.
            let (u, v) = if left < right {
                (left, right)
            } else {
                (right, left)
            };
            let mut edge_string = string_without_vertex(num_vertices, u);
            edge_string.extend(string_without_vertex(num_vertices, v));
            strings.push(edge_string);
        }

        let target = LongestCommonSubsequence::new(num_vertices, strings);
        Ok(ReductionVCToLCS {
            target,
            num_vertices,
        })
    }
}

fn string_without_vertex(num_vertices: usize, omitted: usize) -> Vec<usize> {
    (0..num_vertices)
        .filter(|&vertex| vertex != omitted)
        .collect()
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimumvertexcover_to_longestcommonsubsequence",
        build: || {
            let source = MinimumVertexCover::new(
                SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
                vec![One; 4],
            );
            crate::example_db::specs::rule_example_with_witness::<_, LongestCommonSubsequence>(
                source,
                SolutionPair {
                    source_config: serde_json::json!(vec![false, true, true, false]),
                    target_config: serde_json::json!(vec![Some(0), Some(3), None, None]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumvertexcover_longestcommonsubsequence.rs"]
mod tests;
