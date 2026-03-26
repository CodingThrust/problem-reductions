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

        let n_prime = n + 3; // total vertices in target graph
        if target_solution.len() != n_prime {
            return vec![0; n];
        }

        let v_prime = n; // index of duplicated vertex v'
        let s = n + 1; // pendant attached to v=0
        let t = n + 2; // pendant attached to v'

        // The HP path must start at s or t (pendants with degree 1).
        // Determine path orientation: we want s at the start.
        // target_solution[i] = vertex visited at position i.

        // Find position of s and t in the path
        let s_pos = target_solution.iter().position(|&v| v == s);
        let t_pos = target_solution.iter().position(|&v| v == t);

        let (s_pos, t_pos) = match (s_pos, t_pos) {
            (Some(sp), Some(tp)) => (sp, tp),
            _ => return vec![0; n], // invalid solution
        };

        // Build the path in order from s to t
        let path: Vec<usize> = if s_pos == 0 && t_pos == n_prime - 1 {
            // Already oriented s -> ... -> t
            target_solution.to_vec()
        } else if t_pos == 0 && s_pos == n_prime - 1 {
            // Reverse: t -> ... -> s, flip to s -> ... -> t
            target_solution.iter().copied().rev().collect()
        } else {
            // s or t not at endpoints -- invalid HP for our construction
            return vec![0; n];
        };

        // path = [s, v=0, ..., v'=n, t]
        // Strip s (first) and t (last) to get inner path of length n+1
        let inner = &path[1..n_prime - 1]; // length n+1

        // The inner path should start with v=0 and end with v'=n, or vice versa.
        // Orient so it starts with v=0.
        let oriented: Vec<usize> = if inner[0] == 0 && inner[n] == v_prime {
            inner.to_vec()
        } else if inner[0] == v_prime && inner[n] == 0 {
            inner.iter().copied().rev().collect()
        } else {
            return vec![0; n];
        };

        // oriented = [0, perm of 1..n-1, v'=n]
        // The HC config is the first n entries (dropping v' at the end).
        oriented[..n].to_vec()
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
