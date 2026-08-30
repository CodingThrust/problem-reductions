//! Directed Hamiltonian Path problem implementation.
//!
//! The Directed Hamiltonian Path problem asks whether a directed graph contains
//! a simple directed path that visits every vertex exactly once.

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::DirectedGraph;
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "DirectedHamiltonianPath",
        display_name: "Directed Hamiltonian Path",
        aliases: &["DHP"],
        dimensions: &[
            VariantDimension::new("graph", "DirectedGraph", &["DirectedGraph"]),
        ],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Does the directed graph contain a Hamiltonian path?",
        fields: &[
            FieldInfo { name: "graph", type_name: "DirectedGraph", description: "The directed graph G=(V,A)" },
        ],
    }
}

/// The Directed Hamiltonian Path problem.
///
/// Given a directed graph G = (V, A), determine whether G contains a Hamiltonian path,
/// i.e., a simple directed path that visits every vertex exactly once following arc
/// directions.
///
/// # Representation
///
/// A configuration encodes a permutation using the Lehmer code:
/// `dims() = [n, n-1, ..., 2, 1]`, yielding `n!` reachable configurations.
/// Each configuration is decoded to a permutation of `0..n`, and a solution is
/// valid when every consecutive pair `(path[i], path[i+1])` is an arc in the
/// directed graph.
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::DirectedHamiltonianPath;
/// use problemreductions::topology::DirectedGraph;
/// use problemreductions::{Problem, BruteForce};
///
/// // Simple directed path: 0->1->2->3
/// let graph = DirectedGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
/// let problem = DirectedHamiltonianPath::new(graph);
///
/// let solver = BruteForce::new();
/// let solution = solver.solve(&problem).unwrap();
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectedHamiltonianPath {
    graph: DirectedGraph,
}

impl DirectedHamiltonianPath {
    /// Create a new Directed Hamiltonian Path problem from a directed graph.
    pub fn new(graph: DirectedGraph) -> Self {
        Self { graph }
    }

    /// Get a reference to the underlying directed graph.
    pub fn graph(&self) -> &DirectedGraph {
        &self.graph
    }

    /// Get the number of vertices in the directed graph.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of arcs in the directed graph.
    pub fn num_arcs(&self) -> usize {
        self.graph.num_arcs()
    }

    /// Check if a permutation is a valid directed Hamiltonian path.
    pub fn is_valid_solution(&self, solution: &[usize]) -> bool {
        is_valid_directed_hamiltonian_path(&self.graph, solution)
    }
}

impl Problem for DirectedHamiltonianPath {
    const NAME: &'static str = "DirectedHamiltonianPath";
    type Solution = Vec<usize>;
    type Value = crate::types::Or;

    crate::problem_parameters![("num_arcs", num_arcs), ("num_vertices", num_vertices),];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn evaluate(
        &self,
        solution: &Self::Solution,
    ) -> Result<crate::types::Or, crate::traits::EvaluationError> {
        let n = self.graph.num_vertices();
        if solution.len() != n {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "path ordering length does not match the graph vertices".into(),
            ));
        }
        if solution.iter().any(|&vertex| vertex >= n) {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "path ordering contains an out-of-range vertex".into(),
            ));
        }
        Ok(crate::types::Or(is_valid_directed_hamiltonian_path(
            &self.graph,
            solution,
        )))
    }
}

impl crate::solvers::BruteForceProblem for DirectedHamiltonianPath {
    fn dimensions(&self) -> Vec<usize> {
        lehmer_dims(self.graph.num_vertices())
    }
}

/// Returns the Lehmer code dimension vector for `n` items: `[n, n-1, ..., 2, 1]`.
pub(crate) fn lehmer_dims(n: usize) -> Vec<usize> {
    (1..=n).rev().collect()
}

/// Decode a Lehmer code into a permutation.
///
/// Given a configuration `code` where `code[i] < n - i`, returns the
/// corresponding permutation of `0..n`.
pub(crate) fn decode_lehmer(code: &[usize]) -> Vec<usize> {
    let n = code.len();
    let mut available: Vec<usize> = (0..n).collect();
    let mut perm = Vec::with_capacity(n);
    for &idx in code {
        let idx = idx.min(available.len().saturating_sub(1));
        perm.push(available.remove(idx));
    }
    perm
}

/// Check if a permutation is a valid directed Hamiltonian path.
///
/// A valid directed Hamiltonian path visits every vertex exactly once and
/// every consecutive pair `(perm[i], perm[i+1])` must be an arc in the graph.
pub(crate) fn is_valid_directed_hamiltonian_path(graph: &DirectedGraph, perm: &[usize]) -> bool {
    let n = graph.num_vertices();
    if perm.len() != n {
        return false;
    }

    // Check that perm is a valid permutation of 0..n
    let mut seen = vec![false; n];
    for &v in perm {
        if v >= n || seen[v] {
            return false;
        }
        seen[v] = true;
    }

    // Check that consecutive pairs are directed arcs
    for i in 0..n.saturating_sub(1) {
        if !graph.has_arc(perm[i], perm[i + 1]) {
            return false;
        }
    }

    true
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // 6 vertices, arcs from issue #813
    // Hamiltonian path: [0, 1, 3, 2, 4, 5]
    let graph = DirectedGraph::new(
        6,
        vec![
            (0, 1),
            (0, 3),
            (1, 3),
            (1, 4),
            (2, 0),
            (2, 4),
            (3, 2),
            (3, 5),
            (4, 5),
            (5, 1),
        ],
    );
    let optimal_perm = vec![0usize, 1, 3, 2, 4, 5];
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "directed_hamiltonian_path",
        instance: Box::new(DirectedHamiltonianPath::new(graph)),
        optimal_config: serde_json::to_value(optimal_perm)
            .expect("solution serialization must succeed"),
        optimal_value: serde_json::json!(true),
    }]
}

crate::declare_variants! {
    default DirectedHamiltonianPath => "num_vertices^2 * 2^num_vertices",
}

crate::register_brute_force! {
    DirectedHamiltonianPath decode |_problem: &DirectedHamiltonianPath, indices: Vec<usize>| decode_lehmer(&indices),
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/directed_hamiltonian_path.rs"]
mod tests;
