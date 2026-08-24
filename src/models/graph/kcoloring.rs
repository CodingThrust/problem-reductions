//! Graph K-Coloring problem implementation.
//!
//! The K-Coloring problem asks whether a graph can be colored with K colors
//! such that no two adjacent vertices have the same color.

use crate::registry::{CreateSpec, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::variant::{KValue, VariantParam, K2, K3, K4, K5, KN};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "KColoring",
        display_name: "K-Coloring",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
            VariantDimension::new("k", "KN", &["KN", "K2", "K3", "K4", "K5"]),
        ],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Find valid k-coloring of a graph",
        fields: RuntimeKColoringCreateSpec::FIELDS,
    }
}

/// The Graph K-Coloring problem.
///
/// Given a graph G = (V, E) and K colors, find an assignment of colors
/// to vertices such that no two adjacent vertices have the same color.
///
/// # Type Parameters
///
/// * `K` - KValue type representing the number of colors (e.g., K3 for 3-coloring)
/// * `G` - Graph type (e.g., SimpleGraph, KingsSubgraph)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::KColoring;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::variant::K3;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Triangle graph needs at least 3 colors
/// let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
/// let problem = KColoring::<K3, _>::new(graph);
///
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_witnesses(&problem).unwrap();
///
/// // Verify all solutions are valid colorings
/// for sol in &solutions {
///     assert!(problem.evaluate(sol).unwrap());
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(deserialize = "G: serde::Deserialize<'de>"))]
pub struct KColoring<K: KValue, G> {
    /// The underlying graph.
    graph: G,
    /// Runtime number of colors. Always set; for compile-time K types it equals K::K.
    #[serde(default = "default_num_colors::<K>")]
    num_colors: usize,
    #[serde(skip)]
    _phantom: std::marker::PhantomData<K>,
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct FixedKColoringCreateSpec {
    /// Undirected graph edges.
    #[create(codec = "edge-list")]
    graph: Vec<(usize, usize)>,
    /// Vertex count, needed to preserve isolated vertices.
    num_vertices: Option<usize>,
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct RuntimeKColoringCreateSpec {
    /// Undirected graph edges.
    #[create(codec = "edge-list")]
    graph: Vec<(usize, usize)>,
    /// Vertex count, needed to preserve isolated vertices.
    num_vertices: Option<usize>,
    /// Runtime color count.
    k: usize,
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
    let count = num_vertices.unwrap_or(inferred);
    if count < inferred {
        return Err(format!(
            "num_vertices {count} is too small for graph endpoints; need at least {inferred}"
        )
        .into());
    }
    Ok(SimpleGraph::new(count, edges))
}

impl<K: KValue> TryFrom<FixedKColoringCreateSpec> for KColoring<K, SimpleGraph> {
    type Error = crate::registry::ConstructionError;

    fn try_from(spec: FixedKColoringCreateSpec) -> Result<Self, Self::Error> {
        let num_colors = K::K.ok_or("runtime KColoring requires k")?;
        Ok(Self {
            graph: simple_graph_from_create(spec.graph, spec.num_vertices)?,
            num_colors,
            _phantom: std::marker::PhantomData,
        })
    }
}

impl TryFrom<RuntimeKColoringCreateSpec> for KColoring<KN, SimpleGraph> {
    type Error = crate::registry::ConstructionError;

    fn try_from(spec: RuntimeKColoringCreateSpec) -> Result<Self, Self::Error> {
        if spec.k == 0 {
            return Err("k must be positive".to_string().into());
        }
        Ok(Self::with_k(
            simple_graph_from_create(spec.graph, spec.num_vertices)?,
            spec.k,
        ))
    }
}

fn default_num_colors<K: KValue>() -> usize {
    K::K.unwrap_or(0)
}

impl<K: KValue, G: Graph> KColoring<K, G> {
    /// Create a new K-Coloring problem from a graph.
    ///
    /// # Panics
    /// Panics if `K` is `KN` (use [`KColoring::<KN, G>::with_k`] instead).
    pub fn new(graph: G) -> Self {
        Self {
            graph,
            num_colors: K::K.expect("KN requires with_k"),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get the number of colors.
    pub fn num_colors(&self) -> usize {
        self.num_colors
    }

    /// Check if a configuration is a valid coloring.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        is_valid_coloring(&self.graph, config, self.num_colors)
    }

    /// Check if a coloring is valid.
    fn is_valid_coloring(&self, config: &[usize]) -> bool {
        for (u, v) in self.graph.edges() {
            let color_u = config.get(u).copied().unwrap_or(0);
            let color_v = config.get(v).copied().unwrap_or(0);
            if color_u == color_v {
                return false;
            }
        }
        true
    }
}

impl<G: Graph> KColoring<KN, G> {
    /// Create a K-Coloring problem with an explicit number of colors.
    ///
    /// Only available for `KN` (runtime K). For compile-time K types like
    /// `K3`, use [`new`](KColoring::new) which derives K from the type
    /// parameter.
    pub fn with_k(graph: G, num_colors: usize) -> Self {
        Self {
            graph,
            num_colors,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<K: KValue, G: Graph> KColoring<K, G> {
    /// Get the number of vertices in the underlying graph.
    pub fn num_vertices(&self) -> usize {
        self.graph().num_vertices()
    }

    /// Get the number of edges in the underlying graph.
    pub fn num_edges(&self) -> usize {
        self.graph().num_edges()
    }
}

impl<K: KValue, G> Problem for KColoring<K, G>
where
    G: Graph + VariantParam,
{
    const NAME: &'static str = "KColoring";
    type Value = crate::types::Or;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![K, G]
    }

    fn dims(&self) -> Vec<usize> {
        vec![self.num_colors; self.graph.num_vertices()]
    }

    fn evaluate(
        &self,
        config: &[usize],
    ) -> Result<crate::types::Or, crate::traits::EvaluationError> {
        Ok(crate::types::Or(self.is_valid_coloring(config)))
    }
}

/// Check if a coloring is valid for a graph.
///
/// # Panics
/// Panics if `coloring.len() != graph.num_vertices()`.
pub(crate) fn is_valid_coloring<G: Graph>(
    graph: &G,
    coloring: &[usize],
    num_colors: usize,
) -> bool {
    assert_eq!(
        coloring.len(),
        graph.num_vertices(),
        "coloring length must match num_vertices"
    );
    // Check all colors are valid
    if coloring.iter().any(|&c| c >= num_colors) {
        return false;
    }
    // Check no adjacent vertices have same color
    for (u, v) in graph.edges() {
        if coloring[u] == coloring[v] {
            return false;
        }
    }
    true
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "kcoloring_k3_simplegraph",
        instance: Box::new(KColoring::<K3, _>::new(SimpleGraph::new(
            5,
            vec![(0, 1), (0, 2), (1, 3), (2, 3), (2, 4), (3, 4)],
        ))),
        optimal_config: vec![0, 1, 1, 0, 2],
        optimal_value: serde_json::json!(true),
    }]
}

crate::impl_random_generate!(KColoring<KN, SimpleGraph>, crate::random::ColoringRandomSpec, |spec| {
    let k = spec.k.unwrap_or(3);
    if k == 0 {
        return Err("k must be positive".to_string().into());
    }
    Ok(KColoring::with_k(spec.graph()?, k))
});
crate::impl_random_generate!(KColoring<K2, SimpleGraph>, crate::random::ColoringRandomSpec, |spec| {
    if spec.k.is_some_and(|k| k != 2) { return Err("k must match the selected K2 variant".to_string().into()); }
    Ok(KColoring::new(spec.graph()?))
});
crate::impl_random_generate!(KColoring<K3, SimpleGraph>, crate::random::ColoringRandomSpec, |spec| {
    if spec.k.is_some_and(|k| k != 3) { return Err("k must match the selected K3 variant".to_string().into()); }
    Ok(KColoring::new(spec.graph()?))
});
crate::impl_random_generate!(KColoring<K4, SimpleGraph>, crate::random::ColoringRandomSpec, |spec| {
    if spec.k.is_some_and(|k| k != 4) { return Err("k must match the selected K4 variant".to_string().into()); }
    Ok(KColoring::new(spec.graph()?))
});
crate::impl_random_generate!(KColoring<K5, SimpleGraph>, crate::random::ColoringRandomSpec, |spec| {
    if spec.k.is_some_and(|k| k != 5) { return Err("k must match the selected K5 variant".to_string().into()); }
    Ok(KColoring::new(spec.graph()?))
});

crate::declare_variants! {
    default KColoring<KN, SimpleGraph> => "2^num_vertices" create RuntimeKColoringCreateSpec random,
    KColoring<K2, SimpleGraph> => "num_vertices + num_edges" create FixedKColoringCreateSpec random,
    KColoring<K3, SimpleGraph> => "1.3289^num_vertices" create FixedKColoringCreateSpec random,
    KColoring<K4, SimpleGraph> => "1.7159^num_vertices" create FixedKColoringCreateSpec random,
    // Best known: O*((2-ε)^n) for some ε > 0 (Zamir 2021), concrete ε unknown
    KColoring<K5, SimpleGraph> => "2^num_vertices" create FixedKColoringCreateSpec random,
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/kcoloring.rs"]
mod tests;
