//! Reduction from HamiltonianCircuit to TravelingSalesman.
//!
//! The standard construction embeds the source graph into the complete graph on the
//! same vertex set, assigning weight 1 to source edges and weight 2 to non-edges.
//! The target optimum is exactly n iff the source graph contains a Hamiltonian circuit.

use crate::models::graph::{HamiltonianCircuit, TravelingSalesman};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing HamiltonianCircuit to TravelingSalesman.
#[derive(Debug, Clone)]
pub struct ReductionHamiltonianCircuitToTravelingSalesman {
    target: TravelingSalesman<SimpleGraph, i64>,
}

impl ReductionResult for ReductionHamiltonianCircuitToTravelingSalesman {
    type Source = HamiltonianCircuit<SimpleGraph>;
    type Target = TravelingSalesman<SimpleGraph, i64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_witness(
            self.target_problem(),
            target_solution,
            |value| crate::rules::AggregateReductionResult::extract_value(self, value).0,
            "target witness does not certify a YES answer for the source",
        )?;

        crate::rules::graph_helpers::edges_to_cycle_order(self.target.graph(), target_solution)
    }
}

#[crate::aggregate_reduction]
impl crate::rules::AggregateReductionResult for ReductionHamiltonianCircuitToTravelingSalesman {
    type Source = HamiltonianCircuit<SimpleGraph>;
    type Target = TravelingSalesman<SimpleGraph, i64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_value(&self, value: crate::types::Min<i64>) -> crate::types::Or {
        crate::types::Or(
            value
                .0
                .is_some_and(|cost| i128::from(cost) == self.target.num_vertices() as i128),
        )
    }
}

#[reduction(
    transform = exact {
        num_vertices = "num_vertices",
        num_edges = "num_vertices * (num_vertices - 1) / 2",
    }
)]
impl ReduceTo<TravelingSalesman<SimpleGraph, i64>> for HamiltonianCircuit<SimpleGraph> {
    type Result = ReductionHamiltonianCircuitToTravelingSalesman;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let num_vertices = self.num_vertices();
        let target_graph = SimpleGraph::complete(num_vertices);
        let weights = target_graph
            .edges()
            .into_iter()
            .map(|(u, v)| if self.graph().has_edge(u, v) { 1 } else { 2 })
            .collect();
        let target = TravelingSalesman::new(target_graph, weights);

        Ok(ReductionHamiltonianCircuitToTravelingSalesman { target })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "hamiltoniancircuit_to_travelingsalesman",
        build: || {
            let source = HamiltonianCircuit::new(SimpleGraph::cycle(4));
            crate::example_db::specs::rule_example_with_witness::<
                _,
                TravelingSalesman<SimpleGraph, i64>,
            >(
                source,
                SolutionPair {
                    source_config: serde_json::json!(vec![0, 1, 2, 3]),
                    target_config: serde_json::json!(vec![true, false, true, true, false, true]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/hamiltoniancircuit_travelingsalesman.rs"]
mod tests;
