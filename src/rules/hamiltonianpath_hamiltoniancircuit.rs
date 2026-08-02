//! Reduction from HamiltonianPath to HamiltonianCircuit.
//!
//! For a graph with at least two vertices, the construction copies the source
//! graph and adds one universal vertex. Deleting that vertex from any target
//! Hamiltonian circuit leaves a Hamiltonian path in the source graph. Empty and
//! singleton sources reduce to a fixed triangle because both are feasible in
//! the HamiltonianPath model.

use crate::models::graph::{HamiltonianCircuit, HamiltonianPath};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing HamiltonianPath to HamiltonianCircuit.
#[derive(Debug, Clone)]
pub struct ReductionHamiltonianPathToHamiltonianCircuit {
    target: HamiltonianCircuit<SimpleGraph>,
    num_original_vertices: usize,
}

impl ReductionResult for ReductionHamiltonianPathToHamiltonianCircuit {
    type Source = HamiltonianPath<SimpleGraph>;
    type Target = HamiltonianCircuit<SimpleGraph>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let n = self.num_original_vertices;
        if n < 2 {
            return (0..n).collect();
        }

        let universal = n;
        let universal_position = target_solution
            .iter()
            .position(|&vertex| vertex == universal)
            .expect("target Hamiltonian circuit must contain the universal vertex");

        target_solution[universal_position + 1..]
            .iter()
            .chain(&target_solution[..universal_position])
            .copied()
            .collect()
    }
}

#[reduction(
    overhead = {
        num_vertices = "num_vertices + 3",
        num_edges = "num_edges + num_vertices + 3",
    }
)]
impl ReduceTo<HamiltonianCircuit<SimpleGraph>> for HamiltonianPath<SimpleGraph> {
    type Result = ReductionHamiltonianPathToHamiltonianCircuit;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let target_graph = if n < 2 {
            SimpleGraph::cycle(3)
        } else {
            let universal = n;
            let mut edges = self.graph().edges();
            edges.extend((0..n).map(|vertex| (universal, vertex)));
            SimpleGraph::new(n + 1, edges)
        };

        ReductionHamiltonianPathToHamiltonianCircuit {
            target: HamiltonianCircuit::new(target_graph),
            num_original_vertices: n,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "hamiltonianpath_to_hamiltoniancircuit",
        build: || {
            let source = HamiltonianPath::new(SimpleGraph::new(
                5,
                vec![(0, 1), (1, 2), (2, 3), (3, 4), (0, 2), (1, 3)],
            ));
            crate::example_db::specs::rule_example_with_witness::<_, HamiltonianCircuit<SimpleGraph>>(
                source,
                SolutionPair {
                    source_config: vec![0, 1, 2, 3, 4],
                    target_config: vec![5, 0, 1, 2, 3, 4],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/hamiltonianpath_hamiltoniancircuit.rs"]
mod tests;
