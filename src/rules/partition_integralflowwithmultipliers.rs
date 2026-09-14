//! Reduction from Partition to IntegralFlowWithMultipliers.
//!
//! For an even total sum `S`, this is Sahni's multiplier-flow gadget:
//! items are binary source choices amplified by vertex multipliers and merged
//! through a single bottleneck arc of capacity `S / 2`. For an odd total sum,
//! the reduction returns a fixed infeasible target instance.

use crate::models::graph::IntegralFlowWithMultipliers;
use crate::models::misc::Partition;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::solvers::ProblemOutcome;
use crate::solvers::SolveOutcome;
use crate::topology::DirectedGraph;

/// Result of reducing Partition to IntegralFlowWithMultipliers.
#[derive(Debug, Clone)]
pub struct ReductionPartitionToIntegralFlowWithMultipliers {
    target: IntegralFlowWithMultipliers,
    item_arc_count: usize,
}

impl ReductionResult for ReductionPartitionToIntegralFlowWithMultipliers {
    type Source = Partition;
    type Target = IntegralFlowWithMultipliers;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn recover_result(
        &self,
        source: &Self::Source,
        target: ProblemOutcome<Self::Target>,
    ) -> crate::rules::ExtractionResult<ProblemOutcome<Self::Source>> {
        match target {
            SolveOutcome::Infeasible => Ok(SolveOutcome::Infeasible),
            SolveOutcome::Optimal { solution, .. } => {
                let solution = self.map_solution(&solution)?;
                Ok(SolveOutcome::optimal(source, solution)?)
            }
            SolveOutcome::Feasible { solution, .. } => {
                let solution = self.map_solution(&solution)?;
                Ok(SolveOutcome::feasible(source, solution)?)
            }
        }
    }
}

impl ReductionPartitionToIntegralFlowWithMultipliers {
    fn map_solution(
        &self,
        target_solution: &<<Self as ReductionResult>::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<
        <<Self as ReductionResult>::Source as crate::traits::Problem>::Solution,
    > {
        Ok({
            target_solution[..self.item_arc_count]
                .iter()
                .map(|&flow| flow > 0)
                .collect()
        })
    }
}

#[reduction(
    transform = exact {
        num_vertices = "num_elements + 3",
        num_arcs = "2 * num_elements + 1",
    },
    unavailable = {
        max_capacity = "the target capacity depends on source numeric values not represented by Partition parameters",
        requirement = "the target requirement depends on source numeric values not represented by Partition parameters",
    }
)]
impl ReduceTo<IntegralFlowWithMultipliers> for Partition {
    type Result = ReductionPartitionToIntegralFlowWithMultipliers;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let total_sum = self.total_sum();
        let source_n = self.num_elements();

        if total_sum % 2 != 0 {
            let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2)]);
            return Ok(ReductionPartitionToIntegralFlowWithMultipliers {
                target: IntegralFlowWithMultipliers::new(graph, 0, 2, vec![1, 2, 1], vec![1, 1], 1),
                item_arc_count: source_n,
            });
        }

        let half_sum = total_sum / 2;
        let relay = source_n + 1;
        let sink = source_n + 2;

        let mut arcs = Vec::with_capacity(2 * source_n + 1);
        let mut capacities = Vec::with_capacity(2 * source_n + 1);
        let mut multipliers = vec![1; source_n + 3];

        for (index, &size) in self.sizes().iter().enumerate() {
            let item_vertex = index + 1;
            arcs.push((0, item_vertex));
            capacities.push(1);
            multipliers[item_vertex] = size;
        }

        for (index, &size) in self.sizes().iter().enumerate() {
            let item_vertex = index + 1;
            arcs.push((item_vertex, relay));
            capacities.push(size);
        }

        arcs.push((relay, sink));
        capacities.push(half_sum);
        multipliers[relay] = 1;

        let graph = DirectedGraph::new(source_n + 3, arcs);
        Ok(ReductionPartitionToIntegralFlowWithMultipliers {
            target: IntegralFlowWithMultipliers::new(
                graph,
                0,
                sink,
                multipliers,
                capacities,
                half_sum,
            ),
            item_arc_count: source_n,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "partition_to_integralflowwithmultipliers",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, IntegralFlowWithMultipliers>(
                Partition::new(vec![2, 3, 4, 5, 6, 4]).unwrap(),
                SolutionPair {
                    source_config: serde_json::json!(vec![true, false, true, false, true, false]),
                    target_config: serde_json::json!(vec![1, 0, 1, 0, 1, 0, 2, 0, 4, 0, 6, 0, 12]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/partition_integralflowwithmultipliers.rs"]
mod tests;
