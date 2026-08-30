//! Minimum Capacitated Spanning Tree problem implementation.
//!
//! Given a weighted graph with a designated root vertex, vertex requirements,
//! and a capacity bound, find a minimum-weight spanning tree rooted at the root
//! such that for each edge, the sum of requirements in its subtree (on the
//! non-root side) does not exceed the capacity.

use num_traits::Zero;
use serde::{Deserialize, Serialize};

use crate::registry::{CreateSpec, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::{Min, WeightElement};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumCapacitatedSpanningTree",
        display_name: "Minimum Capacitated Spanning Tree",
        aliases: &["MCST"],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
            VariantDimension::new("weight", "i64", &["i64"]),
        ],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Find minimum weight spanning tree with subtree capacity constraints",
        fields: MinimumCapacitatedSpanningTreeCreateSpec::FIELDS,
    }
}

/// The Minimum Capacitated Spanning Tree problem.
///
/// Given a weighted graph G = (V, E), edge weights w_e, a root vertex v0,
/// vertex requirements r_v (with r_{v0} = 0), and a capacity C, find a
/// spanning tree T rooted at v0 such that:
/// - For each edge e in T, the sum of requirements of all vertices in the
///   subtree on the non-root side of e is at most C.
/// - The total weight of T is minimized.
///
/// # Representation
///
/// Each edge is assigned a binary variable:
/// - 0: edge is not in the spanning tree
/// - 1: edge is in the spanning tree
///
/// # Type Parameters
///
/// * `G` - The graph type (e.g., `SimpleGraph`)
/// * `W` - The weight type for edges and requirements (e.g., `i64`)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumCapacitatedSpanningTree<G, W: WeightElement> {
    /// The underlying graph.
    graph: G,
    /// Weights for each edge (in edge index order).
    weights: Vec<W>,
    /// Root vertex index.
    root: usize,
    /// Vertex requirements (root has requirement 0).
    requirements: Vec<W>,
    /// Subtree capacity bound.
    capacity: W::Sum,
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct MinimumCapacitatedSpanningTreeCreateSpec {
    /// The underlying graph.
    graph: SimpleGraph,
    /// Edge weights; defaults to one per edge.
    weights: Option<Vec<i64>>,
    /// Root vertex.
    root: usize,
    /// Vertex requirements.
    requirements: Vec<i64>,
    /// Subtree capacity bound.
    capacity: i64,
}
impl TryFrom<MinimumCapacitatedSpanningTreeCreateSpec>
    for MinimumCapacitatedSpanningTree<SimpleGraph, i64>
{
    type Error = crate::registry::ConstructionError;
    fn try_from(spec: MinimumCapacitatedSpanningTreeCreateSpec) -> Result<Self, Self::Error> {
        let edges = spec.graph.num_edges();
        let weights = spec.weights.unwrap_or_else(|| vec![1; edges]);
        if weights.len() != edges {
            return Err(format!("weights has {} entries, expected {edges}", weights.len()).into());
        }
        let vertices = spec.graph.num_vertices();
        if vertices < 2 {
            return Err("graph must have at least two vertices".to_string().into());
        }
        if spec.requirements.len() != vertices {
            return Err(format!(
                "requirements has {} entries, expected {vertices}",
                spec.requirements.len()
            )
            .into());
        }
        if spec.root >= vertices {
            return Err("root is outside the graph".to_string().into());
        }
        Ok(Self::new(
            spec.graph,
            weights,
            spec.root,
            spec.requirements,
            spec.capacity,
        ))
    }
}

impl<G: Graph, W: WeightElement> MinimumCapacitatedSpanningTree<G, W> {
    /// Create a MinimumCapacitatedSpanningTree problem.
    ///
    /// # Panics
    /// - If `weights.len() != graph.num_edges()`
    /// - If `requirements.len() != graph.num_vertices()`
    /// - If `root >= graph.num_vertices()`
    /// - If `graph.num_vertices() < 2`
    pub fn new(
        graph: G,
        weights: Vec<W>,
        root: usize,
        requirements: Vec<W>,
        capacity: W::Sum,
    ) -> Self {
        assert_eq!(
            weights.len(),
            graph.num_edges(),
            "weights length must match num_edges"
        );
        assert_eq!(
            requirements.len(),
            graph.num_vertices(),
            "requirements length must match num_vertices"
        );
        assert!(
            root < graph.num_vertices(),
            "root {root} out of range (num_vertices = {})",
            graph.num_vertices()
        );
        assert!(
            graph.num_vertices() >= 2,
            "graph must have at least 2 vertices"
        );
        Self {
            graph,
            weights,
            root,
            requirements,
            capacity,
        }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get the edge weights.
    pub fn weights(&self) -> &[W] {
        &self.weights
    }

    /// Set new edge weights.
    pub fn set_weights(&mut self, weights: Vec<W>) {
        assert_eq!(weights.len(), self.graph.num_edges());
        self.weights = weights;
    }

    /// Check if the problem uses a non-unit weight type.
    pub fn is_weighted(&self) -> bool {
        !W::IS_UNIT
    }

    /// Get the root vertex.
    pub fn root(&self) -> usize {
        self.root
    }

    /// Get the vertex requirements.
    pub fn requirements(&self) -> &[W] {
        &self.requirements
    }

    /// Get the capacity bound.
    pub fn capacity(&self) -> &W::Sum {
        &self.capacity
    }

    /// Get the number of vertices in the underlying graph.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of edges in the underlying graph.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    /// Check if a configuration is a valid capacitated spanning tree.
    pub fn is_valid_solution(
        &self,
        config: &[bool],
    ) -> Result<bool, crate::traits::EvaluationError> {
        is_valid_capacitated_spanning_tree(
            &self.graph,
            &self.requirements,
            self.root,
            &self.capacity,
            config,
        )
    }
}

/// Check if a configuration forms a valid spanning tree:
/// 1. Exactly n-1 edges selected
/// 2. Selected edges form a connected subgraph
fn is_spanning_tree<G: Graph>(graph: &G, config: &[bool]) -> bool {
    let n = graph.num_vertices();
    let edges = graph.edges();
    if config.len() != edges.len() {
        return false;
    }

    let selected_count = config.iter().filter(|&&selected| selected).count();
    if selected_count != n - 1 {
        return false;
    }

    // Build adjacency and BFS from vertex 0
    let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
    for (idx, &sel) in config.iter().enumerate() {
        if sel {
            let (u, v) = edges[idx];
            adj[u].push(v);
            adj[v].push(u);
        }
    }

    let mut visited = vec![false; n];
    let mut queue = std::collections::VecDeque::new();
    visited[0] = true;
    queue.push_back(0);
    while let Some(v) = queue.pop_front() {
        for &u in &adj[v] {
            if !visited[u] {
                visited[u] = true;
                queue.push_back(u);
            }
        }
    }

    visited.iter().all(|&v| v)
}

/// Compute the subtree requirement sum for each edge in the tree rooted at `root`.
/// Returns None if the tree is invalid, otherwise returns the max subtree sum.
fn check_capacity<G: Graph, W: WeightElement>(
    graph: &G,
    requirements: &[W],
    root: usize,
    capacity: &W::Sum,
    config: &[bool],
) -> Result<bool, crate::traits::EvaluationError> {
    let n = graph.num_vertices();
    let edges = graph.edges();

    // Build adjacency list with edge indices
    let mut adj: Vec<Vec<(usize, usize)>> = vec![vec![]; n]; // (neighbor, edge_idx)
    for (idx, &sel) in config.iter().enumerate() {
        if sel {
            let (u, v) = edges[idx];
            adj[u].push((v, idx));
            adj[v].push((u, idx));
        }
    }

    // Root the tree using BFS from root
    let mut parent = vec![usize::MAX; n];
    let mut order = Vec::with_capacity(n);
    let mut visited = vec![false; n];
    let mut queue = std::collections::VecDeque::new();
    visited[root] = true;
    parent[root] = root;
    queue.push_back(root);
    while let Some(v) = queue.pop_front() {
        order.push(v);
        for &(u, _) in &adj[v] {
            if !visited[u] {
                visited[u] = true;
                parent[u] = v;
                queue.push_back(u);
            }
        }
    }

    // Compute subtree sums bottom-up
    let mut subtree_sum: Vec<W::Sum> = requirements.iter().map(|r| r.to_sum()).collect();
    for &v in order.iter().rev() {
        if v != root {
            let p = parent[v];
            let sv = subtree_sum[v].clone();
            subtree_sum[p] = W::checked_add_to_sum(
                subtree_sum[p].clone(),
                sv,
                "summing capacitated spanning tree requirements",
            )?;
        }
    }

    // Check capacity for each non-root vertex (its subtree sum is the flow on its parent edge)
    for (v, sum) in subtree_sum.iter().enumerate() {
        if v != root && *sum > *capacity {
            return Ok(false);
        }
    }

    Ok(true)
}

/// Check if a configuration forms a valid capacitated spanning tree.
fn is_valid_capacitated_spanning_tree<G: Graph, W: WeightElement>(
    graph: &G,
    requirements: &[W],
    root: usize,
    capacity: &W::Sum,
    config: &[bool],
) -> Result<bool, crate::traits::EvaluationError> {
    if !is_spanning_tree(graph, config) {
        return Ok(false);
    }
    check_capacity(graph, requirements, root, capacity, config)
}

impl<G, W> Problem for MinimumCapacitatedSpanningTree<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "MinimumCapacitatedSpanningTree";
    type Solution = Vec<bool>;
    type Value = Min<W::Sum>;

    crate::problem_parameters![("num_edges", num_edges), ("num_vertices", num_vertices),];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G, W]
    }

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Min<W::Sum>, crate::traits::EvaluationError> {
        if config.len() != self.graph.num_edges() {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "edge-selection length does not match the graph".into(),
            ));
        }
        Ok({
            if !is_valid_capacitated_spanning_tree(
                &self.graph,
                &self.requirements,
                self.root,
                &self.capacity,
                config,
            )? {
                return Ok(Min(None));
            }
            let mut total = W::Sum::zero();
            for (idx, &selected) in config.iter().enumerate() {
                if selected {
                    if let Some(w) = self.weights.get(idx) {
                        total = W::checked_add_to_sum(
                            total,
                            w.to_sum(),
                            "summing capacitated spanning tree edge weights",
                        )?;
                    }
                }
            }
            Min(Some(total))
        })
    }
}

impl<G, W> crate::solvers::BruteForceProblem for MinimumCapacitatedSpanningTree<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.graph.num_edges()]
    }
}

crate::declare_variants! {
    default MinimumCapacitatedSpanningTree<SimpleGraph, i64> => "2^num_edges" create MinimumCapacitatedSpanningTreeCreateSpec,
}

crate::register_brute_force! {
    MinimumCapacitatedSpanningTree<SimpleGraph, i64> decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_capacitated_spanning_tree_simplegraph",
        instance: Box::new(MinimumCapacitatedSpanningTree::new(
            SimpleGraph::new(
                5,
                vec![
                    (0, 1),
                    (0, 2),
                    (0, 3),
                    (1, 2),
                    (1, 4),
                    (2, 3),
                    (2, 4),
                    (3, 4),
                ],
            ),
            vec![2, 1, 4, 3, 1, 2, 3, 1], // edge weights
            0,                            // root
            vec![0, 1, 1, 1, 1],          // requirements (root=0)
            3,                            // capacity
        )),
        // Optimal: edges {(0,1),(0,2),(1,4),(3,4)} = indices {0,1,4,7}
        // Weight = 2+1+1+1 = 5
        // Subtree sums: subtree(1)={1,4}->req=2<=3, subtree(2)={2}->req=1<=3,
        //   subtree(4)={4}->req=1<=3, subtree(3)={3}->req=1<=3
        optimal_config: serde_json::json!(vec![true, true, false, false, true, false, false, true]),
        optimal_value: serde_json::json!(5),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/minimum_capacitated_spanning_tree.rs"]
mod tests;
