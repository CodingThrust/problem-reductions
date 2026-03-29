//! Reduction from OptimalLinearArrangement to SequencingToMinimizeWeightedCompletionTime.
//!
//! Based on Lawler (1978) and Lawler-Queyranne-Schulz-Shmoys (LQSS), Lemma 4.14.
//! Vertex jobs have unit processing time and weight `d_max - deg(v)`, while
//! zero-processing-time edge jobs with weight 2 are constrained to follow both
//! endpoints. The total weighted completion time equals the OLA cost plus a
//! constant `d_max * n * (n + 1) / 2`.

use crate::models::graph::OptimalLinearArrangement;
use crate::models::misc::SequencingToMinimizeWeightedCompletionTime;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing OptimalLinearArrangement to SequencingToMinimizeWeightedCompletionTime.
#[derive(Debug, Clone)]
pub struct ReductionOLAToSTMWCT {
    target: SequencingToMinimizeWeightedCompletionTime,
    /// Number of vertices in the source graph (vertex tasks are indices 0..num_vertices).
    num_vertices: usize,
}

impl ReductionResult for ReductionOLAToSTMWCT {
    type Source = OptimalLinearArrangement<SimpleGraph>;
    type Target = SequencingToMinimizeWeightedCompletionTime;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Extract the OLA solution from the scheduling solution.
    ///
    /// The first `num_vertices` tasks in the schedule correspond to vertex tasks.
    /// Their ordering in the schedule gives the linear arrangement: if vertex task
    /// v is at schedule position p, then f(v) = p (among the vertex tasks only).
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        // Decode the Lehmer code to get the schedule (task execution order).
        let schedule = decode_lehmer(target_solution, target_solution.len())
            .expect("target solution must be a valid Lehmer code");

        // Collect vertex task positions in the schedule.
        // The OLA config maps vertex -> position in {0..n-1}.
        let n = self.num_vertices;
        let mut vertex_order: Vec<usize> = Vec::with_capacity(n);
        for &task in &schedule {
            if task < n {
                vertex_order.push(task);
            }
        }

        // vertex_order[i] = the vertex that appears at position i among vertex tasks.
        // We need config[vertex] = position, so invert this.
        let mut config = vec![0usize; n];
        for (position, &vertex) in vertex_order.iter().enumerate() {
            config[vertex] = position;
        }
        config
    }
}

/// Decode a Lehmer code into a permutation (mirrors the misc module helper).
fn decode_lehmer(config: &[usize], n: usize) -> Option<Vec<usize>> {
    if config.len() != n {
        return None;
    }
    let mut available: Vec<usize> = (0..n).collect();
    let mut schedule = Vec::with_capacity(n);
    for &digit in config {
        if digit >= available.len() {
            return None;
        }
        schedule.push(available.remove(digit));
    }
    Some(schedule)
}

/// Encode a schedule (permutation) as a Lehmer code.
fn encode_schedule_as_lehmer(schedule: &[usize]) -> Vec<usize> {
    let mut available: Vec<usize> = (0..schedule.len()).collect();
    let mut config = Vec::with_capacity(schedule.len());
    for &task in schedule {
        let digit = available
            .iter()
            .position(|&candidate| candidate == task)
            .expect("schedule must be a permutation");
        config.push(digit);
        available.remove(digit);
    }
    config
}

#[reduction(overhead = {
    num_tasks = "num_vertices + num_edges",
})]
impl ReduceTo<SequencingToMinimizeWeightedCompletionTime>
    for OptimalLinearArrangement<SimpleGraph>
{
    type Result = ReductionOLAToSTMWCT;

    fn reduce_to(&self) -> Self::Result {
        let n = self.graph().num_vertices();
        let edges: Vec<(usize, usize)> = self.graph().edges();
        let m = edges.len();

        // Compute vertex degrees.
        let mut degrees = vec![0u64; n];
        for &(u, v) in &edges {
            degrees[u] += 1;
            degrees[v] += 1;
        }
        let d_max = degrees.iter().copied().max().unwrap_or(0);

        // Build task arrays: n vertex tasks + m edge tasks.
        let total_tasks = n + m;
        let mut lengths = Vec::with_capacity(total_tasks);
        let mut weights = Vec::with_capacity(total_tasks);
        let mut precedences = Vec::new();

        // Vertex tasks: l = 1, w = d_max - deg(v).
        for v in 0..n {
            lengths.push(1u64);
            weights.push(d_max - degrees[v]);
        }

        // Edge tasks: l = 0, w = 2.
        // Precedence: both endpoint vertex tasks must precede the edge task.
        for (idx, &(u, v)) in edges.iter().enumerate() {
            let edge_task = n + idx;
            lengths.push(0u64);
            weights.push(2u64);
            precedences.push((u, edge_task));
            precedences.push((v, edge_task));
        }

        let target =
            SequencingToMinimizeWeightedCompletionTime::new(lengths, weights, precedences);

        ReductionOLAToSTMWCT {
            target,
            num_vertices: n,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "optimallineararrangement_to_sequencingtominimizeweightedcompletiontime",
        build: || {
            // Path graph P_4: 0-1-2-3
            let source = OptimalLinearArrangement::new(SimpleGraph::new(
                4,
                vec![(0, 1), (1, 2), (2, 3)],
            ));

            // Optimal arrangement: identity [0,1,2,3] with OLA cost 3.
            // d_max = 2, constant = 2 * 4 * 5 / 2 = 20, scheduling cost = 23.
            //
            // Schedule: vertex tasks in order [0,1,2,3], edge tasks immediately
            // after both endpoints.
            // Full schedule: [0, 1, t_01, 2, t_12, 3, t_23] = [0, 1, 4, 2, 5, 3, 6]
            let schedule = vec![0, 1, 4, 2, 5, 3, 6];
            let target_config = encode_schedule_as_lehmer(&schedule);

            // Source config: vertex -> position (0-indexed)
            let source_config = vec![0, 1, 2, 3];

            crate::example_db::specs::rule_example_with_witness::<
                _,
                SequencingToMinimizeWeightedCompletionTime,
            >(
                source,
                SolutionPair {
                    source_config,
                    target_config,
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/optimallineararrangement_sequencingtominimizeweightedcompletiontime.rs"]
mod tests;
