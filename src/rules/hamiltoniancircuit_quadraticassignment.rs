//! Reduction from HamiltonianCircuit to QuadraticAssignment.
//!
//! Uses an exact-decision normalization of the Sahni & Gonzalez (1976)
//! cycle-position construction. The QAP cost counts missing edges in a cyclic
//! permutation, so zero cost certifies a Hamiltonian circuit. Sources with fewer
//! than three vertices map to a fixed positive-cost instance.

use crate::models::algebraic::QuadraticAssignment;
use crate::models::graph::HamiltonianCircuit;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing HamiltonianCircuit to QuadraticAssignment.
#[derive(Debug, Clone)]
pub struct ReductionHamiltonianCircuitToQuadraticAssignment {
    target: QuadraticAssignment,
}

impl ReductionResult for ReductionHamiltonianCircuitToQuadraticAssignment {
    type Source = HamiltonianCircuit<SimpleGraph>;
    type Target = QuadraticAssignment;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        let value =
            crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;
        if !crate::rules::AggregateReductionResult::extract_value(self, value).0 {
            return Err(crate::rules::ExtractionError::invalid(
                "target assignment does not certify a Hamiltonian circuit",
            ));
        }

        // Zero cost makes this permutation itself a Hamiltonian circuit.
        Ok(target_solution.to_vec())
    }
}

impl crate::rules::AggregateReductionResult for ReductionHamiltonianCircuitToQuadraticAssignment {
    type Source = HamiltonianCircuit<SimpleGraph>;
    type Target = QuadraticAssignment;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_value(&self, target_value: crate::types::Min<i64>) -> crate::types::Or {
        crate::types::Or(target_value == crate::types::Min(Some(0)))
    }
}

#[reduction(
    aggregate = custom,
    transform = upper_bound {
        num_facilities = "num_vertices + 3",
        num_locations = "num_vertices + 3",
    }
)]
impl ReduceTo<QuadraticAssignment> for HamiltonianCircuit<SimpleGraph> {
    type Result = ReductionHamiltonianCircuitToQuadraticAssignment;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let source_n = self.num_vertices();
        // The source requires at least three vertices. For smaller graphs use
        // three positions with every distinct pair charged, hence optimum 3.
        let n = source_n.max(3);
        let cost_matrix: Vec<Vec<i64>> = (0..n)
            .map(|i| (0..n).map(|j| i64::from(j == (i + 1) % n)).collect())
            .collect();
        let distance_matrix: Vec<Vec<i64>> = (0..n)
            .map(|k| {
                (0..n)
                    .map(|l| i64::from(k != l && (source_n < 3 || !self.graph().has_edge(k, l))))
                    .collect()
            })
            .collect();

        let target = QuadraticAssignment::new(cost_matrix, distance_matrix);
        Ok(ReductionHamiltonianCircuitToQuadraticAssignment { target })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "hamiltoniancircuit_to_quadraticassignment",
        build: || {
            let source = HamiltonianCircuit::new(SimpleGraph::cycle(4));
            crate::example_db::specs::rule_example_with_witness::<_, QuadraticAssignment>(
                source,
                SolutionPair {
                    source_config: serde_json::json!(vec![0, 1, 2, 3]),
                    target_config: serde_json::json!(vec![0, 1, 2, 3]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/hamiltoniancircuit_quadraticassignment.rs"]
mod tests;
