//! Minimum Multiway Cut problem implementation.
//!
//! The Minimum Multiway Cut problem asks for a minimum weight set of edges
//! whose removal disconnects all terminal pairs.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize, WeightElement};
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumMultiwayCut",
        module_path: module_path!(),
        description: "Find minimum weight set of edges whose removal disconnects all terminal pairs",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The undirected graph G=(V,E)" },
            FieldInfo { name: "terminals", type_name: "Vec<usize>", description: "Terminal vertices that must be separated" },
            FieldInfo { name: "edge_weights", type_name: "Vec<W>", description: "Edge weights w: E -> R (same order as graph.edges())" },
        ],
    }
}

/// The Minimum Multiway Cut problem.
///
/// Given an undirected weighted graph G = (V, E, w) and a set of k terminal
/// vertices T = {t_1, ..., t_k}, find a minimum-weight set of edges C ⊆ E
/// such that no two terminals remain in the same connected component of
/// G' = (V, E \ C).
///
/// # Representation
///
/// Each edge is assigned a binary variable:
/// - 0: edge is kept
/// - 1: edge is removed (in the cut)
///
/// A configuration is feasible if removing the cut edges disconnects all
/// terminal pairs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumMultiwayCut<G, W> {
    graph: G,
    terminals: Vec<usize>,
    edge_weights: Vec<W>,
}

impl<G: Graph, W: Clone + Default> MinimumMultiwayCut<G, W> {
    /// Create a MinimumMultiwayCut problem.
    ///
    /// `edge_weights` must have one entry per edge, in the same order as
    /// [`Graph::edges()`](crate::topology::Graph::edges). Each binary
    /// variable corresponds to an edge: 0 = keep, 1 = cut.
    ///
    /// # Panics
    /// - If `edge_weights.len() != graph.num_edges()`
    /// - If `terminals.len() < 2`
    /// - If any terminal index is out of bounds
    /// - If there are duplicate terminal indices
    pub fn new(graph: G, terminals: Vec<usize>, edge_weights: Vec<W>) -> Self {
        assert_eq!(
            edge_weights.len(),
            graph.num_edges(),
            "edge_weights length must match num_edges"
        );
        assert!(terminals.len() >= 2, "need at least 2 terminals");
        let mut sorted = terminals.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted.len(), terminals.len(), "duplicate terminal indices");
        for &t in &terminals {
            assert!(t < graph.num_vertices(), "terminal index out of bounds");
        }
        Self {
            graph,
            terminals,
            edge_weights,
        }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get the terminal vertices.
    pub fn terminals(&self) -> &[usize] {
        &self.terminals
    }

    /// Get the edge weights.
    pub fn edge_weights(&self) -> &[W] {
        &self.edge_weights
    }
}

impl<G: Graph, W: WeightElement> MinimumMultiwayCut<G, W> {
    /// Number of vertices in the graph.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Number of edges in the graph.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    /// Number of terminal vertices.
    pub fn num_terminals(&self) -> usize {
        self.terminals.len()
    }
}

/// Check if all terminals are in distinct connected components
/// when edges marked as cut (config[e] == 1) are removed.
fn terminals_separated<G: Graph>(graph: &G, terminals: &[usize], config: &[usize]) -> bool {
    let n = graph.num_vertices();
    let edges = graph.edges();

    // Build adjacency list from non-cut edges
    let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
    for (idx, (u, v)) in edges.iter().enumerate() {
        if config[idx] == 0 {
            adj[*u].push(*v);
            adj[*v].push(*u);
        }
    }

    // BFS from each terminal; if a terminal is already visited by a previous
    // terminal's BFS, they share a component => infeasible.
    let mut component = vec![usize::MAX; n];
    for (comp_id, &t) in terminals.iter().enumerate() {
        if component[t] != usize::MAX {
            return false;
        }
        let mut queue = VecDeque::new();
        queue.push_back(t);
        component[t] = comp_id;
        while let Some(u) = queue.pop_front() {
            for &v in &adj[u] {
                if component[v] == usize::MAX {
                    component[v] = comp_id;
                    queue.push_back(v);
                }
            }
        }
    }
    true
}

impl<G, W> Problem for MinimumMultiwayCut<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "MinimumMultiwayCut";
    type Metric = SolutionSize<W::Sum>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G, W]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_edges()]
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<W::Sum> {
        if !terminals_separated(&self.graph, &self.terminals, config) {
            return SolutionSize::Invalid;
        }
        let mut total = W::Sum::zero();
        for (idx, &selected) in config.iter().enumerate() {
            if selected == 1 {
                total += self.edge_weights[idx].to_sum();
            }
        }
        SolutionSize::Valid(total)
    }
}

impl<G, W> OptimizationProblem for MinimumMultiwayCut<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    type Value = W::Sum;

    fn direction(&self) -> Direction {
        Direction::Minimize
    }
}

crate::declare_variants! {
    MinimumMultiwayCut<SimpleGraph, i32> => "1.84^num_terminals * num_vertices^3",
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/minimum_multiway_cut.rs"]
mod tests;
