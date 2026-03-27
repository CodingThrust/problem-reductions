//! Reduction from HamiltonianCircuit to BiconnectivityAugmentation.
//!
//! Based on the Eswaran & Tarjan (1976) approach:
//!
//! Given a Hamiltonian Circuit instance G = (V, E) with n vertices, construct a
//! BiconnectivityAugmentation instance as follows:
//!
//! 1. Start with an edgeless graph on n vertices.
//! 2. For each pair (u, v) with u < v, create a potential edge with:
//!    - weight 1 if {u, v} is in E
//!    - weight 2 if {u, v} is not in E
//! 3. Set budget B = n.
//!
//! G has a Hamiltonian circuit iff there exists a biconnectivity augmentation of
//! cost exactly n using only weight-1 edges (i.e., original edges).
//!
//! The selected weight-1 edges form a Hamiltonian cycle in G, which is necessarily
//! biconnected. Any augmentation using a weight-2 edge would cost at least n+1,
//! exceeding the budget of n (since at least n edges are needed for biconnectivity).

use crate::models::graph::{BiconnectivityAugmentation, HamiltonianCircuit};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing HamiltonianCircuit to BiconnectivityAugmentation.
///
/// Stores the target problem and the mapping from potential edge indices to
/// vertex pairs for solution extraction.
#[derive(Debug, Clone)]
pub struct ReductionHamiltonianCircuitToBiconnectivityAugmentation {
    target: BiconnectivityAugmentation<SimpleGraph, i32>,
    /// Number of vertices in the original graph.
    num_vertices: usize,
    /// Potential edges as (u, v) pairs, in the same order as the target's potential_weights.
    potential_edges: Vec<(usize, usize)>,
}

impl ReductionResult for ReductionHamiltonianCircuitToBiconnectivityAugmentation {
    type Source = HamiltonianCircuit<SimpleGraph>;
    type Target = BiconnectivityAugmentation<SimpleGraph, i32>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let n = self.num_vertices;
        if n < 3 {
            return vec![0; n];
        }

        // Collect selected edges (those with config value 1)
        let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
        for (i, &(u, v)) in self.potential_edges.iter().enumerate() {
            if i < target_solution.len() && target_solution[i] == 1 {
                adj[u].push(v);
                adj[v].push(u);
            }
        }

        // Check that every vertex has exactly degree 2 (Hamiltonian cycle)
        if adj.iter().any(|neighbors| neighbors.len() != 2) {
            return vec![0; n];
        }

        // Walk the cycle starting from vertex 0
        let mut circuit = Vec::with_capacity(n);
        circuit.push(0);
        let mut prev = 0;
        let mut current = adj[0][0];
        while current != 0 {
            circuit.push(current);
            let next = if adj[current][0] == prev {
                adj[current][1]
            } else {
                adj[current][0]
            };
            prev = current;
            current = next;

            // Safety: if we've visited more than n vertices, something is wrong
            if circuit.len() > n {
                return vec![0; n];
            }
        }

        if circuit.len() == n {
            circuit
        } else {
            vec![0; n]
        }
    }
}

#[reduction(
    overhead = {
        num_vertices = "num_vertices",
        num_edges = "0",
        num_potential_edges = "num_vertices * (num_vertices - 1) / 2",
    }
)]
impl ReduceTo<BiconnectivityAugmentation<SimpleGraph, i32>> for HamiltonianCircuit<SimpleGraph> {
    type Result = ReductionHamiltonianCircuitToBiconnectivityAugmentation;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let graph = self.graph();

        // Edgeless initial graph
        let initial_graph = SimpleGraph::empty(n);

        // Create potential edges for all pairs (u, v) with u < v
        let mut potential_weights = Vec::new();
        let mut potential_edges = Vec::new();
        for u in 0..n {
            for v in (u + 1)..n {
                let weight = if graph.has_edge(u, v) { 1 } else { 2 };
                potential_weights.push((u, v, weight));
                potential_edges.push((u, v));
            }
        }

        // Budget = n (exactly enough for n weight-1 edges)
        let budget = n as i32;

        let target = BiconnectivityAugmentation::new(initial_graph, potential_weights, budget);

        ReductionHamiltonianCircuitToBiconnectivityAugmentation {
            target,
            num_vertices: n,
            potential_edges,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "hamiltoniancircuit_to_biconnectivityaugmentation",
        build: || {
            // Square graph (4-cycle): 0-1-2-3-0
            let source = HamiltonianCircuit::new(SimpleGraph::cycle(4));
            // Potential edges for 4 vertices (indices 0..5):
            // 0: (0,1) w=1, 1: (0,2) w=2, 2: (0,3) w=1,
            // 3: (1,2) w=1, 4: (1,3) w=2, 5: (2,3) w=1
            // HC 0-1-2-3-0 selects edges (0,1),(1,2),(2,3),(0,3) => indices 0,3,5,2
            // Config: [1, 0, 1, 1, 0, 1]
            crate::example_db::specs::rule_example_with_witness::<
                _,
                BiconnectivityAugmentation<SimpleGraph, i32>,
            >(
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
#[path = "../unit_tests/rules/hamiltoniancircuit_biconnectivityaugmentation.rs"]
mod tests;
