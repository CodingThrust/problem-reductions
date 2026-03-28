//! Reduction from HamiltonianCircuit to QuadraticAssignment.
//!
//! Uses the Sahni & Gonzalez (1976) construction. The cost matrix encodes
//! cycle adjacency on positions: c[i][j] = 1 if j = (i+1) mod n, else 0.
//! The distance matrix encodes graph connectivity: d[k][l] = 1 if {k,l} ∈ E,
//! else ω = n+1 (penalty). A Hamiltonian circuit exists iff the QAP optimum
//! equals n.

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

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        // QAP config is a permutation γ mapping positions to vertices,
        // which is directly the Hamiltonian circuit visit order.
        target_solution.to_vec()
    }
}

#[reduction(
    overhead = {
        num_facilities = "num_vertices",
        num_locations = "num_vertices",
    }
)]
impl ReduceTo<QuadraticAssignment> for HamiltonianCircuit<SimpleGraph> {
    type Result = ReductionHamiltonianCircuitToQuadraticAssignment;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let omega = (n + 1) as i64;

        // Cost matrix C: cycle adjacency on positions.
        // c[i][j] = 1 if j == (i+1) mod n, else 0.
        let cost_matrix: Vec<Vec<i64>> = (0..n)
            .map(|i| {
                (0..n)
                    .map(|j| if j == (i + 1) % n { 1 } else { 0 })
                    .collect()
            })
            .collect();

        // Distance matrix D: graph connectivity.
        // d[k][l] = 1 if {k,l} ∈ E, else ω (penalty) for k ≠ l; d[k][k] = 0.
        let distance_matrix: Vec<Vec<i64>> = (0..n)
            .map(|k| {
                (0..n)
                    .map(|l| {
                        if k == l {
                            0
                        } else if self.graph().has_edge(k, l) {
                            1
                        } else {
                            omega
                        }
                    })
                    .collect()
            })
            .collect();

        let target = QuadraticAssignment::new(cost_matrix, distance_matrix);
        ReductionHamiltonianCircuitToQuadraticAssignment { target }
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
                    source_config: vec![0, 1, 2, 3],
                    target_config: vec![0, 1, 2, 3],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/hamiltoniancircuit_quadraticassignment.rs"]
mod tests;
