//! Reduction from HamiltonianCircuit to HamiltonianPath.
//!
//! Given a Hamiltonian Circuit instance G = (V, E) with n vertices, we construct
//! a Hamiltonian Path instance G' with n + 3 vertices as follows:
//!
//! 1. Pick an arbitrary vertex v = 0.
//! 2. Create a duplicate vertex v' (index n) connected to all neighbors of v.
//! 3. Add a pendant vertex s (index n+1) with the single edge {s, v}.
//! 4. Add a pendant vertex t (index n+2) with the single edge {t, v'}.
//!
//! G has a Hamiltonian circuit iff G' has a Hamiltonian path (from s to t).
//!
//! The target graph G' has n + 3 vertices and m + deg(v) + 2 edges.

use crate::models::graph::{HamiltonianCircuit, HamiltonianPath};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing HamiltonianCircuit to HamiltonianPath.
///
/// Stores the target HamiltonianPath instance and the number of original vertices
/// to enable solution extraction.
#[derive(Debug, Clone)]
pub struct ReductionHamiltonianCircuitToHamiltonianPath {
    target: HamiltonianPath<SimpleGraph>,
    /// Number of vertices in the original graph.
    num_original_vertices: usize,
}

impl ReductionResult for ReductionHamiltonianCircuitToHamiltonianPath {
    type Source = HamiltonianCircuit<SimpleGraph>;
    type Target = HamiltonianPath<SimpleGraph>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let n = self.num_original_vertices;
        if n == 0 {
            return vec![];
        }

        if target_solution.len() != n + 3 {
            return vec![0; n];
        }

        let v_prime = n; // index of duplicated vertex v'
        let s = n + 1; // pendant attached to v=0
        let t = n + 2; // pendant attached to v'

        // The two pendants force any valid witness to have endpoints s and t.
        let reversed;
        let oriented = match (target_solution.first(), target_solution.last()) {
            (Some(&start), Some(&end)) if start == s && end == t => target_solution,
            (Some(&start), Some(&end)) if start == t && end == s => {
                reversed = target_solution.iter().copied().rev().collect::<Vec<_>>();
                reversed.as_slice()
            }
            _ => return vec![0; n],
        };

        if oriented.get(1) != Some(&0) || oriented.get(n + 1) != Some(&v_prime) {
            return vec![0; n];
        }

        oriented[1..=n].to_vec()
    }
}

#[reduction(
    overhead = {
        num_vertices = "num_vertices + 3",
        num_edges = "num_edges + num_vertices + 1",
    }
)]
impl ReduceTo<HamiltonianPath<SimpleGraph>> for HamiltonianCircuit<SimpleGraph> {
    type Result = ReductionHamiltonianCircuitToHamiltonianPath;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();

        // HC is unsatisfiable for n < 3; return a trivially unsatisfiable HP instance.
        if n < 3 {
            let target_graph = SimpleGraph::empty(n + 3);
            let target = HamiltonianPath::new(target_graph);
            return ReductionHamiltonianCircuitToHamiltonianPath {
                target,
                num_original_vertices: n,
            };
        }

        let source_graph = self.graph();

        // New vertex indices:
        // 0..n-1: original vertices
        // n: v' (duplicate of vertex 0)
        // n+1: s (pendant of vertex 0)
        // n+2: t (pendant of v')
        let v_prime = n;
        let s = n + 1;
        let t = n + 2;

        let mut edges: Vec<(usize, usize)> = Vec::new();

        // 1. Copy all original edges
        for (u, v) in source_graph.edges() {
            edges.push((u, v));
        }

        // 2. Connect v' to all neighbors of vertex 0
        for neighbor in source_graph.neighbors(0) {
            edges.push((v_prime, neighbor));
        }

        // 3. Add pendant edge {s, 0}
        edges.push((s, 0));

        // 4. Add pendant edge {t, v'}
        edges.push((t, v_prime));

        let target_graph = SimpleGraph::new(n + 3, edges);
        let target = HamiltonianPath::new(target_graph);

        ReductionHamiltonianCircuitToHamiltonianPath {
            target,
            num_original_vertices: n,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "hamiltoniancircuit_to_hamiltonianpath",
        build: || {
            // Square graph (4-cycle): 0-1-2-3-0
            let source = HamiltonianCircuit::new(SimpleGraph::cycle(4));
            crate::example_db::specs::rule_example_with_witness::<_, HamiltonianPath<SimpleGraph>>(
                source,
                SolutionPair {
                    // HC solution: visit vertices in order 0, 1, 2, 3
                    source_config: vec![0, 1, 2, 3],
                    // HP solution on G' (7 vertices): s=5, 0, 1, 2, 3, v'=4, t=6
                    target_config: vec![5, 0, 1, 2, 3, 4, 6],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/hamiltoniancircuit_hamiltonianpath.rs"]
mod tests;
