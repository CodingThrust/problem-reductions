//! Steiner Tree in Graphs problem implementation.
//!
//! The Steiner Tree problem asks for a minimum-weight subtree of a graph
//! that connects all terminal vertices.

use crate::registry::{CreateSpec, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::{Min, One, WeightElement};
use num_traits::Zero;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "SteinerTreeInGraphs",
        display_name: "Steiner Tree in Graphs",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
            VariantDimension::new("weight", "i64", &["One", "i64"]),
        ],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Find minimum weight subtree connecting all terminal vertices",
        fields: SteinerTreeInGraphsCreateSpec::<i64>::FIELDS,
    }
}

/// The Steiner Tree in Graphs problem.
///
/// Given a weighted graph G = (V, E) with edge weights w_e and a
/// subset R ⊆ V of required terminal vertices, find a subtree T of G
/// that includes all vertices of R and minimizes the total edge weight
/// Σ_{e ∈ T} w(e).
///
/// # Representation
///
/// Each edge is assigned a binary variable:
/// - 0: edge is not in the tree
/// - 1: edge is in the tree
///
/// A valid Steiner tree requires:
/// - All terminal vertices are connected through selected edges
/// - Selected edges form a connected subgraph (optimally a tree)
///
/// # Type Parameters
///
/// * `G` - The graph type (e.g., `SimpleGraph`)
/// * `W` - The weight type for edges (e.g., `i64`, `f64`)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::SteinerTreeInGraphs;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, BruteForce};
///
/// // Path graph 0-1-2-3, terminals {0, 3}
/// let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
/// let problem = SteinerTreeInGraphs::new(graph, vec![0, 3], vec![1, 1, 1]);
///
/// let solver = BruteForce::new();
/// let solution = solver.solve(&problem).unwrap().unwrap();
/// // Optimal: select all 3 edges (the only path from 0 to 3)
/// assert_eq!(solution, vec![true, true, true]);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteinerTreeInGraphs<G, W> {
    /// The underlying graph.
    graph: G,
    /// Required terminal vertices.
    terminals: Vec<usize>,
    /// Weights for each edge (in edge index order).
    edge_weights: Vec<W>,
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct SteinerTreeInGraphsCreateSpec<W> {
    /// The underlying graph.
    graph: SimpleGraph,
    /// Required terminal vertices.
    terminals: Vec<usize>,
    /// Edge weights; defaults to one per edge.
    edge_weights: Option<Vec<W>>,
}
impl<W> TryFrom<SteinerTreeInGraphsCreateSpec<W>> for SteinerTreeInGraphs<SimpleGraph, W>
where
    W: WeightElement,
{
    type Error = crate::registry::ConstructionError;
    fn try_from(spec: SteinerTreeInGraphsCreateSpec<W>) -> Result<Self, Self::Error> {
        let count = spec.graph.num_edges();
        let edge_weights = spec
            .edge_weights
            .unwrap_or_else(|| (0..count).map(|_| W::unit()).collect());
        if edge_weights.len() != count {
            return Err(format!(
                "edge_weights has {} entries, expected {count}",
                edge_weights.len()
            )
            .into());
        }
        if let Some(&terminal) = spec
            .terminals
            .iter()
            .find(|&&t| t >= spec.graph.num_vertices())
        {
            return Err(format!("terminal {terminal} is outside the graph").into());
        }
        Ok(Self::new(spec.graph, spec.terminals, edge_weights))
    }
}

impl<G: Graph, W: Clone + Default> SteinerTreeInGraphs<G, W> {
    /// Create a SteinerTreeInGraphs problem from a graph, terminals, and edge weights.
    ///
    /// # Panics
    /// Panics if `edge_weights.len() != graph.num_edges()` or any terminal index is out of bounds.
    pub fn new(graph: G, terminals: Vec<usize>, edge_weights: Vec<W>) -> Self {
        assert_eq!(
            edge_weights.len(),
            graph.num_edges(),
            "edge_weights length must match num_edges"
        );
        for &t in &terminals {
            assert!(
                t < graph.num_vertices(),
                "terminal vertex {} out of bounds (num_vertices = {})",
                t,
                graph.num_vertices()
            );
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

    /// Get all edges with their weights.
    pub fn edges(&self) -> Vec<(usize, usize, W)> {
        self.graph
            .edges()
            .into_iter()
            .zip(self.edge_weights.iter().cloned())
            .map(|((u, v), w)| (u, v, w))
            .collect()
    }

    /// Set new weights for the problem.
    pub fn set_weights(&mut self, weights: Vec<W>) {
        assert_eq!(weights.len(), self.graph.num_edges());
        self.edge_weights = weights;
    }

    /// Get the weights for the problem.
    pub fn weights(&self) -> Vec<W> {
        self.edge_weights.clone()
    }

    /// Check if the problem uses a non-unit weight type.
    pub fn is_weighted(&self) -> bool
    where
        W: WeightElement,
    {
        !W::IS_UNIT
    }

    /// Check if a configuration is a valid Steiner tree.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        if config.len() != self.graph.num_edges() {
            return false;
        }
        let selected: Vec<bool> = config.iter().map(|&s| s == 1).collect();
        is_steiner_tree(&self.graph, &self.terminals, &selected)
    }
}

impl<G: Graph, W: WeightElement> SteinerTreeInGraphs<G, W> {
    /// Get the number of vertices in the underlying graph.
    pub fn num_vertices(&self) -> usize {
        self.graph().num_vertices()
    }

    /// Get the number of edges in the underlying graph.
    pub fn num_edges(&self) -> usize {
        self.graph().num_edges()
    }

    /// Get the number of terminal vertices.
    pub fn num_terminals(&self) -> usize {
        self.terminals.len()
    }
}

impl<G, W> Problem for SteinerTreeInGraphs<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "SteinerTreeInGraphs";
    type Solution = Vec<bool>;
    type Value = Min<W::Sum>;

    crate::problem_size![
        ("num_edges", num_edges),
        ("num_terminals", num_terminals),
        ("num_vertices", num_vertices),
    ];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G, W]
    }

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Min<W::Sum>, crate::traits::EvaluationError> {
        Ok({
            if config.len() != self.graph.num_edges() {
                return Err(crate::traits::EvaluationError::InvalidConfiguration(
                    "edge-selection length does not match the graph".into(),
                ));
            }
            let selected = config;
            if !is_steiner_tree(&self.graph, &self.terminals, selected) {
                return Ok(Min(None));
            }
            let mut total = W::Sum::zero();
            for (idx, &sel) in config.iter().enumerate() {
                if sel {
                    if let Some(w) = self.edge_weights.get(idx) {
                        total = W::checked_add_to_sum(
                            total,
                            w.to_sum(),
                            "summing Steiner tree edge weights",
                        )?;
                    }
                }
            }
            Min(Some(total))
        })
    }
}

impl<G, W> crate::solvers::BruteForceProblem for SteinerTreeInGraphs<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.graph.num_edges()]
    }
}

/// Check if a selection of edges forms a valid Steiner tree (connected subgraph spanning all terminals).
///
/// A valid Steiner tree requires:
/// 1. All terminal vertices are reachable from each other through selected edges.
/// 2. The selected edges form a connected subgraph that includes all terminals.
///
/// Note: The optimal solution is always a tree, but we accept any connected subgraph
/// spanning all terminals (the brute-force solver will find the minimum-weight one).
///
/// # Panics
/// Panics if `selected.len() != graph.num_edges()`.
pub(crate) fn is_steiner_tree<G: Graph>(graph: &G, terminals: &[usize], selected: &[bool]) -> bool {
    assert_eq!(
        selected.len(),
        graph.num_edges(),
        "selected length must match num_edges"
    );

    // If no terminals, any selection is trivially valid (including empty)
    if terminals.is_empty() {
        return true;
    }

    // If only one terminal, it's valid as long as that terminal exists
    // (no edges needed to connect a single vertex)
    if terminals.len() == 1 {
        return true;
    }

    // Build adjacency list from selected edges
    let n = graph.num_vertices();
    let edges = graph.edges();
    let mut adj: Vec<Vec<usize>> = vec![vec![]; n];

    let mut has_any_edge = false;
    for (idx, &sel) in selected.iter().enumerate() {
        if sel {
            let (u, v) = edges[idx];
            adj[u].push(v);
            adj[v].push(u);
            has_any_edge = true;
        }
    }

    if !has_any_edge {
        return false;
    }

    // BFS from the first terminal to check connectivity of all terminals
    let start = terminals[0];
    let mut visited = vec![false; n];
    let mut queue = std::collections::VecDeque::new();
    visited[start] = true;
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        for &neighbor in &adj[node] {
            if !visited[neighbor] {
                visited[neighbor] = true;
                queue.push_back(neighbor);
            }
        }
    }

    // All terminals must be reachable
    terminals.iter().all(|&t| visited[t])
}

crate::impl_random_generate!(SteinerTreeInGraphs<SimpleGraph, i64>, crate::random::SimpleGraphRandomSpec, |spec| {
    if spec.num_vertices < 2 {
        return Err("num_vertices must be at least 2".to_string().into());
    }
    let graph = spec.graph()?;
    let terminals = (0..std::cmp::max(2, spec.num_vertices / 2)).collect();
    let weights = vec![1; graph.num_edges()];
    Ok(SteinerTreeInGraphs::new(graph, terminals, weights))
});

crate::declare_variants! {
    default SteinerTreeInGraphs<SimpleGraph, i64> => "2^num_terminals * num_vertices^3" create SteinerTreeInGraphsCreateSpec<i64> random,
    SteinerTreeInGraphs<SimpleGraph, One> => "2^num_terminals * num_vertices^3" create SteinerTreeInGraphsCreateSpec<One>,
}

crate::register_brute_force! {
    SteinerTreeInGraphs<SimpleGraph, i64> decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
    SteinerTreeInGraphs<SimpleGraph, One> decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "steiner_tree_in_graphs_simplegraph",
        instance: Box::new(SteinerTreeInGraphs::new(
            SimpleGraph::new(
                6,
                vec![(0, 1), (0, 2), (1, 3), (2, 3), (2, 5), (3, 4), (4, 5)],
            ),
            vec![0, 3, 5],
            vec![3, 2, 4, 1, 2, 3, 1],
        )),
        // Optimal: edges {0,2}(w=2), {2,3}(w=1), {2,5}(w=2) = weight 5
        optimal_config: serde_json::json!(vec![false, true, false, true, true, false, false]),
        optimal_value: serde_json::json!(5),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/steiner_tree_in_graphs.rs"]
mod tests;
