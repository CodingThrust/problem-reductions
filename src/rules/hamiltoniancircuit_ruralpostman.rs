//! Reduction from HamiltonianCircuit to RuralPostman.
//!
//! Vertex-splitting construction inspired by Lenstra & Rinnooy Kan (1976).
//!
//! # Construction
//!
//! Given a graph G = (V, E) with n vertices and m edges:
//! - Split each vertex v_i into v_i^a (vertex 2i) and v_i^b (vertex 2i+1).
//! - Add a required edge {v_i^a, v_i^b} with weight 1 for each vertex (n required edges).
//! - For each edge {v_i, v_j} in E, add two non-required connectivity edges:
//!   {v_i^b, v_j^a} and {v_j^b, v_i^a}, each with weight 1.
//!
//! The target graph has 2n vertices, n + 2m edges, and n required edges.
//!
//! # Correctness
//!
//! G has a Hamiltonian circuit iff the optimal RPP cost equals 2n:
//! - If G has HC (v_{p_0}, ..., v_{p_{n-1}}): the RPP tour traverses
//!   v_{p_0}^a -> v_{p_0}^b -> v_{p_1}^a -> v_{p_1}^b -> ... -> v_{p_0}^a,
//!   using n required edges (cost n) and n connectivity edges (cost n), total 2n.
//! - If G has no HC: every valid RPP tour covering all required edges needs
//!   strictly more than n connectivity edges (the bipartite graph between
//!   b-vertices and a-vertices does not admit a perfect matching corresponding
//!   to a Hamiltonian circuit), so cost > 2n.

use crate::models::graph::{HamiltonianCircuit, RuralPostman};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing HamiltonianCircuit to RuralPostman.
#[derive(Debug, Clone)]
pub struct ReductionHamiltonianCircuitToRuralPostman {
    target: RuralPostman<SimpleGraph, i32>,
    /// Number of vertices in the original graph.
    n: usize,
    /// Edges of the original graph (for solution extraction).
    source_edges: Vec<(usize, usize)>,
}

impl ReductionResult for ReductionHamiltonianCircuitToRuralPostman {
    type Source = HamiltonianCircuit<SimpleGraph>;
    type Target = RuralPostman<SimpleGraph, i32>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        // The target solution is edge multiplicities.
        // Required edges are indices 0..n (the {v_i^a, v_i^b} edges).
        // Connectivity edges start at index n.
        // For each source edge (v_i, v_j) at source index k:
        //   target edge n + 2*k is {v_i^b, v_j^a}
        //   target edge n + 2*k + 1 is {v_j^b, v_i^a}
        //
        // A connectivity edge {v_i^b, v_j^a} used with multiplicity 1 means
        // the tour goes from vertex i to vertex j (j follows i in the HC).

        let n = self.n;

        // Build successor map from connectivity edges used exactly once
        let mut successor = vec![usize::MAX; n];
        for (k, &(vi, vj)) in self.source_edges.iter().enumerate() {
            let fwd_idx = n + 2 * k; // {v_i^b, v_j^a}
            let bwd_idx = n + 2 * k + 1; // {v_j^b, v_i^a}

            let fwd_mult = target_solution.get(fwd_idx).copied().unwrap_or(0);
            let bwd_mult = target_solution.get(bwd_idx).copied().unwrap_or(0);

            // In an optimal HC solution, each connectivity edge is used 0 or 1 times.
            // Each vertex should have exactly one outgoing connectivity edge.
            if fwd_mult > 0 && successor[vi] == usize::MAX {
                successor[vi] = vj;
            }
            if bwd_mult > 0 && successor[vj] == usize::MAX {
                successor[vj] = vi;
            }
        }

        // Walk the successor chain starting from vertex 0
        let mut cycle = Vec::with_capacity(n);
        let mut current = 0;
        for _ in 0..n {
            cycle.push(current);
            let next = successor[current];
            if next == usize::MAX {
                // No valid successor found; return fallback
                return vec![0; n];
            }
            current = next;
        }

        cycle
    }
}

#[reduction(
    overhead = {
        num_vertices = "2 * num_vertices",
        num_edges = "num_vertices + 2 * num_edges",
        num_required_edges = "num_vertices",
    }
)]
impl ReduceTo<RuralPostman<SimpleGraph, i32>> for HamiltonianCircuit<SimpleGraph> {
    type Result = ReductionHamiltonianCircuitToRuralPostman;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let source_edges: Vec<(usize, usize)> = self.graph().edges();
        let m = source_edges.len();

        // Build target graph with 2n vertices
        let num_target_edges = n + 2 * m;
        let mut target_edges = Vec::with_capacity(num_target_edges);
        let mut edge_weights = Vec::with_capacity(num_target_edges);
        let mut required_edges = Vec::with_capacity(n);

        // Required edges: {v_i^a, v_i^b} = {2i, 2i+1} with weight 1
        for i in 0..n {
            target_edges.push((2 * i, 2 * i + 1));
            edge_weights.push(1);
            required_edges.push(i); // edge index i is required
        }

        // Connectivity edges for each source edge {v_i, v_j}:
        //   {v_i^b, v_j^a} = {2i+1, 2j} with weight 1
        //   {v_j^b, v_i^a} = {2j+1, 2i} with weight 1
        for &(vi, vj) in &source_edges {
            target_edges.push((2 * vi + 1, 2 * vj));
            edge_weights.push(1);
            target_edges.push((2 * vj + 1, 2 * vi));
            edge_weights.push(1);
        }

        let target_graph = SimpleGraph::new(2 * n, target_edges);
        let target = RuralPostman::new(target_graph, edge_weights, required_edges);

        ReductionHamiltonianCircuitToRuralPostman {
            target,
            n,
            source_edges,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "hamiltoniancircuit_to_ruralpostman",
        build: || {
            // Triangle graph: 3 vertices, 3 edges, HC = [0, 1, 2]
            let source = HamiltonianCircuit::new(SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]));

            // Target graph has 6 vertices, 3 + 6 = 9 edges, 3 required edges.
            // HC [0, 1, 2] uses connectivity edges:
            //   0->1: fwd edge of source edge 0=(0,1), idx=3
            //   1->2: fwd edge of source edge 1=(1,2), idx=5
            //   2->0: bwd edge of source edge 2=(0,2), idx=8
            // Required edges all have multiplicity 1.
            // target_config = [1, 1, 1, 1, 0, 1, 0, 0, 1]
            crate::example_db::specs::rule_example_with_witness::<_, RuralPostman<SimpleGraph, i32>>(
                source,
                SolutionPair {
                    source_config: vec![0, 1, 2],
                    target_config: vec![1, 1, 1, 1, 0, 1, 0, 0, 1],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/hamiltoniancircuit_ruralpostman.rs"]
mod tests;
