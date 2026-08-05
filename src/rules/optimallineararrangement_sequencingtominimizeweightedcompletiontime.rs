//! Reduction from OptimalLinearArrangement to SequencingToMinimizeWeightedCompletionTime.
//!
//! Lawler's construction uses one unit-length job per vertex and one
//! zero-length job per edge. Vertex job `v` gets weight `d_max - deg(v)`,
//! edge job `{u, v}` gets weight 2, and the edge job must follow both
//! endpoint jobs.
//!
//! The source OLA model uses 0-indexed positions, while completion times
//! in the scheduling model are 1-indexed because each vertex job has unit
//! length. The resulting additive shift is still
//! `d_max * n * (n + 1) / 2`: the `+1` offset is already accounted for by
//! completion times, so no extra correction term is needed.

use crate::models::graph::OptimalLinearArrangement;
use crate::models::misc::SequencingToMinimizeWeightedCompletionTime;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing OptimalLinearArrangement to SequencingToMinimizeWeightedCompletionTime.
#[derive(Debug, Clone)]
pub struct ReductionOLAToSequencingToMinimizeWeightedCompletionTime {
    target: SequencingToMinimizeWeightedCompletionTime,
    num_vertices: usize,
}

impl ReductionResult for ReductionOLAToSequencingToMinimizeWeightedCompletionTime {
    type Source = OptimalLinearArrangement<SimpleGraph>;
    type Target = SequencingToMinimizeWeightedCompletionTime;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &[usize],
    ) -> crate::rules::ExtractionResult<Vec<usize>> {
        Ok({
            let schedule =
                crate::models::misc::decode_lehmer(target_solution, self.target.num_tasks())
                    .expect("target solution must be a valid Lehmer code");
            let mut arrangement = vec![0usize; self.num_vertices];
            let mut next_position = 0usize;

            for task in schedule {
                if task < self.num_vertices {
                    arrangement[task] = next_position;
                    next_position += 1;
                }
            }

            arrangement
        })
    }
}

#[reduction(overhead = {
    num_tasks = "num_vertices + num_edges",
})]
impl ReduceTo<SequencingToMinimizeWeightedCompletionTime>
    for OptimalLinearArrangement<SimpleGraph>
{
    type Result = ReductionOLAToSequencingToMinimizeWeightedCompletionTime;

    fn reduce_to(&self) -> Self::Result {
        let graph = self.graph();
        let num_vertices = graph.num_vertices();
        let edges = graph.edges();
        let max_degree = (0..num_vertices)
            .map(|v| graph.degree(v))
            .max()
            .unwrap_or(0) as u64;

        let mut lengths = Vec::with_capacity(num_vertices + edges.len());
        let mut weights = Vec::with_capacity(num_vertices + edges.len());
        let mut precedences = Vec::with_capacity(2 * edges.len());

        for vertex in 0..num_vertices {
            lengths.push(1);
            weights.push(max_degree - graph.degree(vertex) as u64);
        }

        for (edge_index, &(u, v)) in edges.iter().enumerate() {
            let edge_task = num_vertices + edge_index;
            lengths.push(0);
            weights.push(2);
            precedences.push((u, edge_task));
            precedences.push((v, edge_task));
        }

        ReductionOLAToSequencingToMinimizeWeightedCompletionTime {
            target: SequencingToMinimizeWeightedCompletionTime::new(lengths, weights, precedences),
            num_vertices,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::example_db::specs::assemble_rule_example;
    use crate::export::SolutionPair;
    use crate::solvers::BruteForce;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "optimallineararrangement_to_sequencingtominimizeweightedcompletiontime",
        build: || {
            let source =
                OptimalLinearArrangement::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]));
            let reduction =
                ReduceTo::<SequencingToMinimizeWeightedCompletionTime>::reduce_to(&source);
            let target_config = BruteForce::new()
                .find_witness(reduction.target_problem())
                .expect("canonical example must be solvable");
            let source_config = reduction.extract_solution(&target_config).unwrap();
            assemble_rule_example(
                &source,
                reduction.target_problem(),
                vec![SolutionPair {
                    source_config,
                    target_config,
                }],
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/optimallineararrangement_sequencingtominimizeweightedcompletiontime.rs"]
mod tests;
