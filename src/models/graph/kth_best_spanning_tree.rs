//! Kth Best Spanning Tree problem implementation.
//!
//! Given a weighted graph, determine whether it contains `k` distinct spanning
//! trees whose total weights are all at most a prescribed bound.

use crate::registry::{CreateSpec, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::WeightElement;
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

inventory::submit! {
    ProblemSchemaEntry {
        name: "KthBestSpanningTree",
        display_name: "Kth Best Spanning Tree",
        aliases: &[],
        dimensions: &[VariantDimension::new("weight", "i64", &["i64"])],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Do there exist k distinct spanning trees with total weight at most B?",
        fields: KthBestSpanningTreeCreateSpec::FIELDS,
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

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct KthBestSpanningTreeCreateSpec {
    #[create(codec = "edge-list")]
    graph: Vec<(usize, usize)>,
    num_vertices: Option<usize>,
    #[create(codec = "comma-separated")]
    edge_weights: Option<Vec<i64>>,
    k: usize,
    bound: i64,
}

impl TryFrom<KthBestSpanningTreeCreateSpec> for KthBestSpanningTree<i64> {
    type Error = crate::registry::ConstructionError;

    fn try_from(spec: KthBestSpanningTreeCreateSpec) -> Result<Self, Self::Error> {
        let graph = simple_graph_from_create(spec.graph, spec.num_vertices)?;
        let weights = spec
            .edge_weights
            .unwrap_or_else(|| vec![1; graph.num_edges()]);
        if weights.len() != graph.num_edges() {
            return Err(format!(
                "edge_weights has length {}, expected {}",
                weights.len(),
                graph.num_edges()
            )
            .into());
        }
        if spec.k == 0 {
            return Err("k must be positive".to_string().into());
        }
        Ok(Self::new(graph, weights, spec.k, spec.bound))
    }
}

fn simple_graph_from_create(
    edges: Vec<(usize, usize)>,
    num_vertices: Option<usize>,
) -> Result<SimpleGraph, crate::registry::ConstructionError> {
    if edges.is_empty() && num_vertices.is_none() {
        return Err("num_vertices is required for an empty graph"
            .to_string()
            .into());
    }
    for (index, &(u, v)) in edges.iter().enumerate() {
        if u == v {
            return Err(format!("graph edge {index} is a self-loop at vertex {u}").into());
        }
    }
    let inferred = edges
        .iter()
        .flat_map(|&(u, v)| [u, v])
        .max()
        .map(|vertex| vertex.checked_add(1).ok_or("vertex count overflows usize"))
        .transpose()?
        .unwrap_or(0);
    let num_vertices = num_vertices.unwrap_or(inferred);
    if num_vertices < inferred {
        return Err(format!("num_vertices {num_vertices} is too small for graph endpoints; need at least {inferred}").into());
    }
    Ok(SimpleGraph::new(num_vertices, edges))
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
    pub fn is_valid_solution(
        &self,
        config: &[Vec<bool>],
    ) -> Result<bool, crate::traits::EvaluationError> {
        if config.len() != self.k
            || config
                .iter()
                .any(|tree| tree.len() != self.graph.num_edges())
        {
            return Ok(false);
        }

        let edges = self.graph.edges();
        if !self.blocks_are_pairwise_distinct(config) {
            return Ok(false);
        }
        for tree in config {
            if !self.block_is_valid_tree(tree, &edges)? {
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn block_is_valid_tree(
        &self,
        block: &[bool],
        edges: &[(usize, usize)],
    ) -> Result<bool, crate::traits::EvaluationError> {
        if block.len() != edges.len() {
            return Ok(false);
        }

        let num_vertices = self.graph.num_vertices();
        let selected_count = block.iter().filter(|&&selected| selected).count();
        if selected_count != num_vertices.saturating_sub(1) {
            return Ok(false);
        }

        let mut total_weight = W::Sum::zero();
        let mut adjacency = vec![Vec::new(); num_vertices];
        let mut start = None;

        for (idx, &selected) in block.iter().enumerate() {
            if !selected {
                continue;
            }
            total_weight = W::checked_add_to_sum(
                total_weight,
                self.weights[idx].to_sum(),
                "summing spanning tree edge weights",
            )?;
            let (u, v) = edges[idx];
            adjacency[u].push(v);
            adjacency[v].push(u);
            if start.is_none() {
                start = Some(u);
            }
        }

        if total_weight > self.bound {
            return Ok(false);
        }

        if num_vertices <= 1 {
            return Ok(true);
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

        Ok(visited.into_iter().all(|seen| seen))
    }

    fn blocks_are_pairwise_distinct(&self, config: &[Vec<bool>]) -> bool {
        for left in 0..config.len() {
            for right in (left + 1)..config.len() {
                if config[left] == config[right] {
                    return false;
                }
            }
        }
        true
    }
}

impl<W> Problem for KthBestSpanningTree<W>
where
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "KthBestSpanningTree";
    type Solution = Vec<Vec<bool>>;
    type Value = crate::types::Or;

    crate::problem_parameters![
        ("num_vertices", num_vertices),
        ("num_edges", num_edges),
        ("k", k),
    ];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![W]
    }

    fn evaluate(
        &self,
        solution: &Self::Solution,
    ) -> Result<crate::types::Or, crate::traits::EvaluationError> {
        if solution.len() != self.k
            || solution
                .iter()
                .any(|tree| tree.len() != self.graph.num_edges())
        {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "spanning-tree collection dimensions do not match the instance".into(),
            ));
        }
        Ok(crate::types::Or(self.is_valid_solution(solution)?))
    }
}

impl<W> crate::solvers::BruteForceProblem for KthBestSpanningTree<W>
where
    W: WeightElement + crate::variant::VariantParam,
{
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.k * self.graph.num_edges()]
    }
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
        id: "kth_best_spanning_tree",
        instance: Box::new(problem),
        optimal_config: serde_json::json!([
            [true, true, true, false, false, false],
            [true, true, false, false, true, false]
        ]),
        optimal_value: serde_json::json!(true),
    }]
}

crate::declare_variants! {
    default KthBestSpanningTree<i64> => "2^(num_edges * k)" create KthBestSpanningTreeCreateSpec,
}

crate::register_brute_force! {
    KthBestSpanningTree<i64> decode |problem: &KthBestSpanningTree<i64>, indices: Vec<usize>| indices.chunks(problem.num_edges()).map(crate::config::config_to_bits).collect(),
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/kth_best_spanning_tree.rs"]
mod tests;
