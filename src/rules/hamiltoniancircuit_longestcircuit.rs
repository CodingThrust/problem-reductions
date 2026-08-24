//! Reduction from HamiltonianCircuit to LongestCircuit.
//!
//! Given an HC instance G = (V, E), construct an LC instance on the same graph
//! with unit edge weights. A Hamiltonian circuit exists iff the optimal circuit
//! length equals |V|.

use crate::models::graph::{HamiltonianCircuit, LongestCircuit};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing HamiltonianCircuit to LongestCircuit.
#[derive(Debug, Clone)]
pub struct ReductionHamiltonianCircuitToLongestCircuit {
    target: LongestCircuit<SimpleGraph, i64>,
}

impl ReductionResult for ReductionHamiltonianCircuitToLongestCircuit {
    type Source = HamiltonianCircuit<SimpleGraph>;
    type Target = LongestCircuit<SimpleGraph, i64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &[usize],
    ) -> crate::rules::ExtractionResult<Vec<usize>> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        crate::rules::graph_helpers::edges_to_cycle_order(self.target.graph(), target_solution)
    }
}

#[reduction(
    size = exact {
        num_vertices = "num_vertices",
        num_edges = "num_edges",
    }
)]
impl ReduceTo<LongestCircuit<SimpleGraph, i64>> for HamiltonianCircuit<SimpleGraph> {
    type Result = ReductionHamiltonianCircuitToLongestCircuit;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_vertices();
        let edges = self.graph().edges();
        let target = LongestCircuit::new(SimpleGraph::new(n, edges), vec![1i64; self.num_edges()]);
        Ok(ReductionHamiltonianCircuitToLongestCircuit { target })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "hamiltoniancircuit_to_longestcircuit",
        build: || {
            let source = HamiltonianCircuit::new(SimpleGraph::cycle(4));
            crate::example_db::specs::rule_example_with_witness::<_, LongestCircuit<SimpleGraph, i64>>(
                source,
                SolutionPair {
                    source_config: vec![0, 1, 2, 3],
                    target_config: vec![1, 1, 1, 1],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/hamiltoniancircuit_longestcircuit.rs"]
mod tests;
