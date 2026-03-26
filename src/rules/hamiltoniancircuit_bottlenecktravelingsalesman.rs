//! Reduction from HamiltonianCircuit to BottleneckTravelingSalesman.
//!
//! The standard construction embeds the source graph into the complete graph on the
//! same vertex set, assigning weight 1 to source edges and weight 2 to non-edges.
//! The optimal bottleneck tour equals 1 iff the source graph contains a Hamiltonian circuit.

use crate::models::graph::{BottleneckTravelingSalesman, HamiltonianCircuit};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing HamiltonianCircuit to BottleneckTravelingSalesman.
#[derive(Debug, Clone)]
pub struct ReductionHamiltonianCircuitToBottleneckTravelingSalesman {
    target: BottleneckTravelingSalesman,
}

impl ReductionResult for ReductionHamiltonianCircuitToBottleneckTravelingSalesman {
    type Source = HamiltonianCircuit<SimpleGraph>;
    type Target = BottleneckTravelingSalesman;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        crate::rules::graph_helpers::edges_to_cycle_order(self.target.graph(), target_solution)
    }
}

#[reduction(
    overhead = {
        num_vertices = "num_vertices",
        num_edges = "num_vertices * (num_vertices - 1) / 2",
    }
)]
impl ReduceTo<BottleneckTravelingSalesman> for HamiltonianCircuit<SimpleGraph> {
    type Result = ReductionHamiltonianCircuitToBottleneckTravelingSalesman;

    fn reduce_to(&self) -> Self::Result {
        let num_vertices = self.num_vertices();
        let target_graph = SimpleGraph::complete(num_vertices);
        let weights = target_graph
            .edges()
            .into_iter()
            .map(|(u, v)| if self.graph().has_edge(u, v) { 1 } else { 2 })
            .collect();
        let target = BottleneckTravelingSalesman::new(target_graph, weights);

        ReductionHamiltonianCircuitToBottleneckTravelingSalesman { target }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "hamiltoniancircuit_to_bottlenecktravelingsalesman",
        build: || {
            let source = HamiltonianCircuit::new(SimpleGraph::cycle(4));
            crate::example_db::specs::rule_example_with_witness::<_, BottleneckTravelingSalesman>(
                source,
                SolutionPair {
                    source_config: vec![0, 1, 2, 3],
                    target_config: vec![1, 0, 1, 1, 0, 1],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/hamiltoniancircuit_bottlenecktravelingsalesman.rs"]
mod tests;
