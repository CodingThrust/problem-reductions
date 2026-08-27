//! MaximumMatching problem implementation.
//!
//! The Maximum Matching problem asks for a maximum weight set of edges
//! such that no two edges share a vertex.

use crate::registry::{CreateSpec, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::{Max, WeightElement};
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

inventory::submit! {
    ProblemSchemaEntry {
        name: "MaximumMatching",
        display_name: "Maximum Matching",
        aliases: &["MaxMatching"],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
            VariantDimension::new("weight", "i64", &["i64"]),
        ],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Find maximum weight matching in a graph",
        fields: MaximumMatchingCreateSpec::FIELDS,
    }
}

/// The Maximum Matching problem.
///
/// Given a graph G = (V, E) with edge weights, find a maximum weight
/// subset M ⊆ E such that no two edges in M share a vertex.
///
/// # Type Parameters
///
/// * `G` - The graph type (e.g., `SimpleGraph`, `KingsSubgraph`, `UnitDiskGraph`)
/// * `W` - The weight type (e.g., `i64`, `f64`, `One`)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::MaximumMatching;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, BruteForce};
///
/// // Path graph 0-1-2
/// let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
/// let problem = MaximumMatching::<_, i64>::unit_weights(graph);
///
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_witnesses(&problem).unwrap();
///
/// // Maximum matching has 1 edge
/// for sol in &solutions {
///     assert_eq!(sol.iter().filter(|&&selected| selected).count(), 1);
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaximumMatching<G, W> {
    /// The underlying graph.
    graph: G,
    /// Weights for each edge (in edge index order).
    edge_weights: Vec<W>,
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct MaximumMatchingCreateSpec {
    #[create(codec = "edge-list")]
    graph: Vec<(usize, usize)>,
    num_vertices: Option<usize>,
    #[create(codec = "comma-separated")]
    edge_weights: Option<Vec<i64>>,
}

impl TryFrom<MaximumMatchingCreateSpec> for MaximumMatching<SimpleGraph, i64> {
    type Error = crate::registry::ConstructionError;

    fn try_from(spec: MaximumMatchingCreateSpec) -> Result<Self, Self::Error> {
        let graph = simple_graph_from_create(spec.graph, spec.num_vertices)?;
        let edge_weights = spec
            .edge_weights
            .unwrap_or_else(|| vec![1; graph.num_edges()]);
        if edge_weights.len() != graph.num_edges() {
            return Err(format!(
                "edge_weights has length {}, expected {}",
                edge_weights.len(),
                graph.num_edges()
            )
            .into());
        }
        Ok(Self::new(graph, edge_weights))
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
        return Err(format!(
            "num_vertices {num_vertices} is too small for graph endpoints; need at least {inferred}"
        )
        .into());
    }
    Ok(SimpleGraph::new(num_vertices, edges))
}

impl<G: Graph, W: Clone + Default> MaximumMatching<G, W> {
    /// Create a MaximumMatching problem from a graph with given edge weights.
    ///
    /// # Arguments
    /// * `graph` - The graph
    /// * `edge_weights` - Weight for each edge (in graph.edges() order)
    pub fn new(graph: G, edge_weights: Vec<W>) -> Self {
        assert_eq!(
            edge_weights.len(),
            graph.num_edges(),
            "edge_weights length must match num_edges"
        );
        Self {
            graph,
            edge_weights,
        }
    }

    /// Create a MaximumMatching problem with unit weights.
    pub fn unit_weights(graph: G) -> Self
    where
        W: WeightElement,
    {
        let edge_weights = vec![W::unit(); graph.num_edges()];
        Self {
            graph,
            edge_weights,
        }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get edge endpoints.
    pub fn edge_endpoints(&self, edge_idx: usize) -> Option<(usize, usize)> {
        self.graph.edges().get(edge_idx).copied()
    }

    /// Get all edges with their endpoints and weights.
    pub fn edges(&self) -> Vec<(usize, usize, W)> {
        self.graph
            .edges()
            .into_iter()
            .zip(self.edge_weights.iter().cloned())
            .map(|((u, v), w)| (u, v, w))
            .collect()
    }

    /// Build a map from vertices to incident edges.
    pub fn vertex_to_edges(&self) -> HashMap<usize, Vec<usize>> {
        let mut v2e: HashMap<usize, Vec<usize>> = HashMap::new();
        for (idx, (u, v)) in self.graph.edges().iter().enumerate() {
            v2e.entry(*u).or_default().push(idx);
            v2e.entry(*v).or_default().push(idx);
        }
        v2e
    }

    /// Check if a configuration is a valid matching.
    pub fn is_valid_solution(&self, config: &[bool]) -> bool {
        self.is_valid_matching(config)
    }

    /// Check if a configuration is a valid matching (internal).
    fn is_valid_matching(&self, config: &[bool]) -> bool {
        let mut vertex_used = vec![false; self.graph.num_vertices()];

        for (idx, &selected) in config.iter().enumerate() {
            if selected {
                if let Some((u, v)) = self.edge_endpoints(idx) {
                    if vertex_used[u] || vertex_used[v] {
                        return false;
                    }
                    vertex_used[u] = true;
                    vertex_used[v] = true;
                }
            }
        }
        true
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
}

impl<G: Graph, W: WeightElement> MaximumMatching<G, W> {
    /// Get the number of vertices in the underlying graph.
    pub fn num_vertices(&self) -> usize {
        self.graph().num_vertices()
    }

    /// Get the number of edges in the underlying graph.
    pub fn num_edges(&self) -> usize {
        self.graph().num_edges()
    }
}

impl<G, W> Problem for MaximumMatching<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "MaximumMatching";
    type Solution = Vec<bool>;
    type Value = Max<W::Sum>;

    crate::problem_size![("num_edges", num_edges), ("num_vertices", num_vertices),];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G, W]
    }

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Max<W::Sum>, crate::traits::EvaluationError> {
        if config.len() != self.graph.num_edges() {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "edge-selection length does not match the graph".into(),
            ));
        }
        Ok({
            if !self.is_valid_matching(config) {
                return Ok(Max(None));
            }
            let mut total = W::Sum::zero();
            for (idx, &selected) in config.iter().enumerate() {
                if selected {
                    if let Some(w) = self.edge_weights.get(idx) {
                        total = W::checked_add_to_sum(
                            total,
                            w.to_sum(),
                            "summing selected matching-edge weights",
                        )?;
                    }
                }
            }
            Max(Some(total))
        })
    }
}

impl<G, W> crate::solvers::BruteForceProblem for MaximumMatching<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.graph.num_edges()]
    }
}

crate::impl_random_generate!(MaximumMatching<SimpleGraph, i64>, crate::random::SimpleGraphRandomSpec, |spec| {
    let graph = spec.graph()?;
    let weights = vec![1; graph.num_edges()];
    Ok(MaximumMatching::new(graph, weights))
});

crate::declare_variants! {
    default MaximumMatching<SimpleGraph, i64> => "num_vertices^3" create MaximumMatchingCreateSpec random,
}

crate::register_brute_force! {
    MaximumMatching<SimpleGraph, i64> decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "maximum_matching_simplegraph",
        instance: Box::new(MaximumMatching::<_, i64>::unit_weights(SimpleGraph::new(
            5,
            vec![(0, 1), (0, 2), (1, 3), (2, 3), (2, 4), (3, 4)],
        ))),
        optimal_config: serde_json::json!(vec![true, false, false, false, true, false]),
        optimal_value: serde_json::json!(2),
    }]
}

/// Check if a selection of edges forms a valid matching.
///
/// # Panics
/// Panics if `selected.len() != graph.num_edges()`.
#[cfg(test)]
pub(crate) fn is_matching<G: Graph>(graph: &G, selected: &[bool]) -> bool {
    assert_eq!(
        selected.len(),
        graph.num_edges(),
        "selected length must match num_edges"
    );

    let edges = graph.edges();
    let mut vertex_used = vec![false; graph.num_vertices()];
    for (idx, &sel) in selected.iter().enumerate() {
        if sel {
            let (u, v) = edges[idx];
            if vertex_used[u] || vertex_used[v] {
                return false;
            }
            vertex_used[u] = true;
            vertex_used[v] = true;
        }
    }
    true
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/maximum_matching.rs"]
mod tests;
