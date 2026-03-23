//! Kth Best Spanning Tree problem implementation.
//!
//! Given a weighted graph, determine whether it contains `k` distinct spanning
//! trees whose total weights are all at most a prescribed bound.

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::{Problem, SatisfactionProblem};
use crate::types::WeightElement;
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

inventory::submit! {
    ProblemSchemaEntry {
        name: "KthBestSpanningTree",
        display_name: "Kth Best Spanning Tree",
        aliases: &[],
        dimensions: &[VariantDimension::new("weight", "i32", &["i32"])],
        module_path: module_path!(),
        description: "Do there exist k distinct spanning trees with total weight at most B?",
        fields: &[
            FieldInfo { name: "graph", type_name: "SimpleGraph", description: "The underlying graph G=(V,E)" },
            FieldInfo { name: "weights", type_name: "Vec<W>", description: "Edge weights w(e) for each edge in E" },
            FieldInfo { name: "k", type_name: "usize", description: "Number of distinct spanning trees required" },
            FieldInfo { name: "bound", type_name: "W::Sum", description: "Upper bound B on each spanning tree weight" },
        ],
    }
}

/// Kth Best Spanning Tree.
///
/// Given an undirected graph `G = (V, E)`, non-negative edge weights `w(e)`,
/// a positive integer `k`, and a bound `B`, determine whether there are `k`
/// distinct spanning trees of `G` whose total weights are all at most `B`.
///
/// # Representation
///
/// A configuration is `k` consecutive binary blocks of length `|E|`.
/// Each block selects the edges of one candidate spanning tree.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KthBestSpanningTree<W: WeightElement> {
    graph: SimpleGraph,
    weights: Vec<W>,
    k: usize,
    bound: W::Sum,
}

impl<W: WeightElement> KthBestSpanningTree<W> {
    /// Create a new KthBestSpanningTree instance.
    ///
    /// # Panics
    ///
    /// Panics if the number of weights does not match the number of edges, or
    /// if `k` is zero.
    pub fn new(graph: SimpleGraph, weights: Vec<W>, k: usize, bound: W::Sum) -> Self {
        assert_eq!(
            weights.len(),
            graph.num_edges(),
            "weights length must match graph num_edges"
        );
        assert!(k > 0, "k must be positive");

        Self {
            graph,
            weights,
            k,
            bound,
        }
    }

    /// Get the underlying graph.
    pub fn graph(&self) -> &SimpleGraph {
        &self.graph
    }

    /// Get the edge weights.
    pub fn weights(&self) -> &[W] {
        &self.weights
    }

    /// Get the requested number of trees.
    pub fn k(&self) -> usize {
        self.k
    }

    /// Get the weight bound.
    pub fn bound(&self) -> &W::Sum {
        &self.bound
    }

    /// Get the number of vertices.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of edges.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    /// Check whether the problem uses a non-unit weight type.
    pub fn is_weighted(&self) -> bool {
        !W::IS_UNIT
    }

    /// Check whether a configuration satisfies the problem.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        self.evaluate_config(config)
    }

    fn block_is_valid_tree(&self, block: &[usize], edges: &[(usize, usize)]) -> bool {
        if block.len() != edges.len() || block.iter().any(|&value| value > 1) {
            return false;
        }

        let num_vertices = self.graph.num_vertices();
        let selected_count = block.iter().filter(|&&value| value == 1).count();
        if selected_count != num_vertices.saturating_sub(1) {
            return false;
        }

        let mut total_weight = W::Sum::zero();
        let mut adjacency = vec![Vec::new(); num_vertices];
        let mut start = None;

        for (idx, &selected) in block.iter().enumerate() {
            if selected == 0 {
                continue;
            }
            total_weight += self.weights[idx].to_sum();
            let (u, v) = edges[idx];
            adjacency[u].push(v);
            adjacency[v].push(u);
            if start.is_none() {
                start = Some(u);
            }
        }

        if total_weight > self.bound {
            return false;
        }

        if num_vertices <= 1 {
            return true;
        }

        // SAFETY: num_vertices > 1 and selected_count == num_vertices - 1 > 0,
        // so at least one edge was selected and `start` is Some.
        let start = start.expect("at least one selected edge");

        let mut visited = vec![false; num_vertices];
        let mut queue = VecDeque::new();
        visited[start] = true;
        queue.push_back(start);

        while let Some(vertex) = queue.pop_front() {
            for &neighbor in &adjacency[vertex] {
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    queue.push_back(neighbor);
                }
            }
        }

        visited.into_iter().all(|seen| seen)
    }

    fn blocks_are_pairwise_distinct(&self, config: &[usize], block_size: usize) -> bool {
        debug_assert!(block_size > 0, "block_size must be positive");
        let blocks: Vec<&[usize]> = config.chunks_exact(block_size).collect();
        for left in 0..blocks.len() {
            for right in (left + 1)..blocks.len() {
                if blocks[left] == blocks[right] {
                    return false;
                }
            }
        }
        true
    }

    fn evaluate_config(&self, config: &[usize]) -> bool {
        let block_size = self.graph.num_edges();
        let expected_len = self.k * block_size;
        if config.len() != expected_len {
            return false;
        }

        if block_size == 0 {
            return self.k == 1 && self.block_is_valid_tree(config, &[]);
        }

        let edges = self.graph.edges();

        if !self.blocks_are_pairwise_distinct(config, block_size) {
            return false;
        }

        config
            .chunks_exact(block_size)
            .all(|block| self.block_is_valid_tree(block, &edges))
    }
}

impl<W> Problem for KthBestSpanningTree<W>
where
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "KthBestSpanningTree";
    type Value = bool;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![W]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.k * self.graph.num_edges()]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        self.evaluate_config(config)
    }
}

impl<W> SatisfactionProblem for KthBestSpanningTree<W> where
    W: WeightElement + crate::variant::VariantParam
{
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // K4 with weights [1,1,2,2,2,3], k=2, B=4.
    // 16 spanning trees; exactly 2 have weight ≤ 4 (both weight 4):
    //   {01,02,03} (star at 0) and {01,02,13}.
    // Satisfying configs = 2 (the two orderings of this pair).
    // 12 variables → 2^12 = 4096 configs, fast to enumerate.
    let graph = SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
    let problem = KthBestSpanningTree::new(graph, vec![1, 1, 2, 2, 2, 3], 2, 4);
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "kth_best_spanning_tree_i32",
        instance: Box::new(problem),
        optimal_config: vec![1, 1, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0],
        optimal_value: serde_json::json!(true),
    }]
}

crate::declare_variants! {
    default sat KthBestSpanningTree<i32> => "2^(num_edges * k)",
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/kth_best_spanning_tree.rs"]
mod tests;
