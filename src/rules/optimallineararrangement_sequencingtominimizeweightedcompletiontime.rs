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
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok({
            let mut arrangement = vec![0usize; self.num_vertices];
            let mut next_position = 0usize;

            for &task in target_solution {
                if task < self.num_vertices {
                    arrangement[task] = next_position;
                    next_position += 1;
                }
            }

            arrangement
        })
    }
}

#[reduction(
    size = exact {
        num_tasks = "num_vertices + num_edges",
    })]
impl ReduceTo<SequencingToMinimizeWeightedCompletionTime>
    for OptimalLinearArrangement<SimpleGraph>
{
    type Result = ReductionOLAToSequencingToMinimizeWeightedCompletionTime;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let graph = self.graph();
        let num_vertices = graph.num_vertices();
        let edges = graph.edges();
        let max_degree = (0..num_vertices)
            .map(|v| graph.degree(v))
            .max()
            .unwrap_or(0);
        let max_degree = i64::try_from(max_degree).map_err(|_| {
            crate::rules::ReductionError::integer_overflow::<
                OptimalLinearArrangement<SimpleGraph>,
                SequencingToMinimizeWeightedCompletionTime,
            >("converting the maximum degree to i64")
        })?;

        let mut lengths = Vec::with_capacity(num_vertices + edges.len());
        let mut weights = Vec::with_capacity(num_vertices + edges.len());
        let mut precedences = Vec::with_capacity(2 * edges.len());

        for vertex in 0..num_vertices {
            let degree = i64::try_from(graph.degree(vertex)).map_err(|_| {
                crate::rules::ReductionError::integer_overflow::<
                    OptimalLinearArrangement<SimpleGraph>,
                    SequencingToMinimizeWeightedCompletionTime,
                >("converting a vertex degree to i64")
            })?;
            lengths.push(1);
            weights.push(max_degree - degree);
        }

        for (edge_index, &(u, v)) in edges.iter().enumerate() {
            let edge_task = num_vertices + edge_index;
            lengths.push(0);
            weights.push(2);
            precedences.push((u, edge_task));
            precedences.push((v, edge_task));
        }

        Ok(ReductionOLAToSequencingToMinimizeWeightedCompletionTime {
            target: SequencingToMinimizeWeightedCompletionTime::new(lengths, weights, precedences),
            num_vertices,
        })
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
                ReduceTo::<SequencingToMinimizeWeightedCompletionTime>::reduce_to(&source)
                    .expect("reduction should succeed");
            let target_config = BruteForce::new()
                .solve(reduction.target_problem())
                .expect("canonical target evaluation must succeed")
                .expect("canonical example must be solvable");
            let source_config = reduction.extract_solution(&target_config).unwrap();
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
#[path = "../unit_tests/rules/optimallineararrangement_sequencingtominimizeweightedcompletiontime.rs"]
mod tests;
