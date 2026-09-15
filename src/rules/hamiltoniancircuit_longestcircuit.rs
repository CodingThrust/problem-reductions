//! Reduction from HamiltonianCircuit to LongestCircuit.
//!
//! Given an HC instance G = (V, E), construct an LC instance on the same graph
//! with unit edge weights. A Hamiltonian circuit exists iff the optimal circuit
//! length equals |V|.

use crate::models::decision::Decision;
use crate::models::graph::{HamiltonianCircuit, LongestCircuit};
use crate::reduction;
use crate::rules::traits::{recover_preserving_status, ReduceTo, ReductionResult};
use crate::solvers::ProblemOutcome;
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing HamiltonianCircuit to LongestCircuit.
#[derive(Debug, Clone)]
pub struct ReductionHamiltonianCircuitToLongestCircuit {
    target: Decision<LongestCircuit<SimpleGraph, i64>>,
}

impl ReductionResult for ReductionHamiltonianCircuitToLongestCircuit {
    type Source = HamiltonianCircuit<SimpleGraph>;
    type Target = Decision<LongestCircuit<SimpleGraph, i64>>;

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

impl ReductionHamiltonianCircuitToLongestCircuit {
    fn map_solution(
        &self,
        target_solution: &<<Self as ReductionResult>::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<
        <<Self as ReductionResult>::Source as crate::traits::Problem>::Solution,
    > {
        Ok(crate::rules::graph_helpers::edges_to_cycle_order(
            self.target.inner().graph(),
            target_solution,
        ))
    }
}

#[reduction(
    transform = exact {
        num_vertices = "num_vertices",
        num_edges = "num_edges",
    }
)]
impl ReduceTo<Decision<LongestCircuit<SimpleGraph, i64>>> for HamiltonianCircuit<SimpleGraph> {
    type Result = ReductionHamiltonianCircuitToLongestCircuit;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_vertices();
        let edges = self.graph().edges();
        let target = LongestCircuit::new(SimpleGraph::new(n, edges), vec![1i64; self.num_edges()]);
        Ok(ReductionHamiltonianCircuitToLongestCircuit {
            target: Decision::new(
                target,
                <Self as ReduceTo<Decision<LongestCircuit<SimpleGraph, i64>>>>::exact_i64(
                    n,
                    "encoding the circuit bound",
                )?,
            ),
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "hamiltoniancircuit_to_longestcircuit",
        build: || {
            let source = HamiltonianCircuit::new(SimpleGraph::cycle(4));
            crate::example_db::specs::rule_example_with_witness::<
                _,
                Decision<LongestCircuit<SimpleGraph, i64>>,
            >(
                source,
                SolutionPair {
                    source_config: serde_json::json!(vec![0, 1, 2, 3]),
                    target_config: serde_json::json!(vec![true, true, true, true]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/hamiltoniancircuit_longestcircuit.rs"]
mod tests;
