//! Reduction from HamiltonianPath to ConsecutiveOnesSubmatrix.
//!
//! Given a Hamiltonian Path instance G = (V, E) with n vertices and m edges,
//! we construct a ConsecutiveOnesSubmatrix instance as follows (Booth 1975,
//! Garey & Johnson SR14):
//!
//! 1. Build the vertex-edge incidence matrix A of size n × m:
//!    a_{i,j} = 1 iff vertex i is an endpoint of edge j.
//! 2. Set bound K = n − 1 (number of edges in a Hamiltonian path).
//!
//! G has a Hamiltonian path iff K columns of A can be permuted so that each
//! row has all its 1's consecutive (the consecutive ones property).
//!
//! Overhead: num_rows = num_vertices, num_cols = num_edges, bound = num_vertices − 1.

use crate::models::algebraic::ConsecutiveOnesSubmatrix;
use crate::models::graph::HamiltonianPath;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing HamiltonianPath to ConsecutiveOnesSubmatrix.
///
/// Stores the target problem, the original graph edge list (for solution
/// extraction), and the number of original vertices.
#[derive(Debug, Clone)]
pub struct ReductionHamiltonianPathToConsecutiveOnesSubmatrix {
    target: ConsecutiveOnesSubmatrix,
    /// Edges of the original graph, indexed the same as columns in the matrix.
    edges: Vec<(usize, usize)>,
    /// Number of vertices in the original graph.
    num_vertices: usize,
}

impl ReductionResult for ReductionHamiltonianPathToConsecutiveOnesSubmatrix {
    type Source = HamiltonianPath<SimpleGraph>;
    type Target = ConsecutiveOnesSubmatrix;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let n = self.num_vertices;
        if n == 0 {
            return vec![];
        }
        if n == 1 {
            return vec![0];
        }

        // target_solution is a binary vector over columns (edges).
        // Selected columns correspond to edges forming the Hamiltonian path.
        let selected_edges: Vec<(usize, usize)> = target_solution
            .iter()
            .enumerate()
            .filter(|(_, &v)| v == 1)
            .map(|(j, _)| self.edges[j])
            .collect();

        if selected_edges.len() != n - 1 {
            return vec![0; n];
        }

        // Build adjacency list from selected edges.
        let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
        for &(u, v) in &selected_edges {
            adj[u].push(v);
            adj[v].push(u);
        }

        // Find the path endpoints (degree-1 vertices in the selected subgraph).
        let endpoints: Vec<usize> = (0..n).filter(|&v| adj[v].len() == 1).collect();
        if endpoints.len() != 2 {
            // Not a valid path — fallback.
            return vec![0; n];
        }

        // Walk the path from one endpoint.
        let mut path = Vec::with_capacity(n);
        let mut current = endpoints[0];
        let mut prev = usize::MAX;
        for _ in 0..n {
            path.push(current);
            let next = adj[current].iter().copied().find(|&nb| nb != prev);
            prev = current;
            match next {
                Some(nx) => current = nx,
                None => break,
            }
        }

        if path.len() != n {
            return vec![0; n];
        }

        path
    }
}

#[reduction(
    overhead = {
        num_rows = "num_vertices",
        num_cols = "num_edges",
    }
)]
impl ReduceTo<ConsecutiveOnesSubmatrix> for HamiltonianPath<SimpleGraph> {
    type Result = ReductionHamiltonianPathToConsecutiveOnesSubmatrix;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let edges = self.graph().edges();
        let m = edges.len();

        // K = n - 1 (but at least 0 for degenerate cases).
        let bound = if n > 0 { (n - 1) as i64 } else { 0 };

        // If there are fewer edges than required (m < n-1), a Hamiltonian path
        // is impossible. Construct a trivially unsatisfiable C1P instance:
        // a 3×3 Tucker-style matrix with bound = 3 that has no valid column
        // permutation satisfying C1P.
        if n > 1 && m < n - 1 {
            let tucker = vec![
                vec![true, true, false],
                vec![true, false, true],
                vec![false, true, true],
            ];
            let target = ConsecutiveOnesSubmatrix::new(tucker, 3);
            return ReductionHamiltonianPathToConsecutiveOnesSubmatrix {
                target,
                edges,
                num_vertices: n,
            };
        }

        // Build n × m vertex-edge incidence matrix.
        let mut matrix = vec![vec![false; m]; n];
        for (j, &(u, v)) in edges.iter().enumerate() {
            matrix[u][j] = true;
            matrix[v][j] = true;
        }

        let target = ConsecutiveOnesSubmatrix::new(matrix, bound);

        ReductionHamiltonianPathToConsecutiveOnesSubmatrix {
            target,
            edges,
            num_vertices: n,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "hamiltonianpath_to_consecutiveonessubmatrix",
        build: || {
            // Path graph: 0-1-2-3 (has a Hamiltonian path: 0,1,2,3)
            let source = HamiltonianPath::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]));

            // Edges: [(0,1), (1,2), (2,3)] — all 3 edges are in the path.
            // K = 3 = n-1, so target_config selects all columns.
            let target_config = vec![1, 1, 1];

            crate::example_db::specs::rule_example_with_witness::<_, ConsecutiveOnesSubmatrix>(
                source,
                SolutionPair {
                    source_config: vec![0, 1, 2, 3],
                    target_config,
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/hamiltonianpath_consecutiveonessubmatrix.rs"]
mod tests;
