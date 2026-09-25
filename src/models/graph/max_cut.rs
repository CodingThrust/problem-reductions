//! MaxCut problem implementation.
//!
//! The Maximum Cut problem asks for a partition of vertices into two sets
//! that maximizes the total weight of edges crossing the partition.

use crate::registry::{CreateSpec, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::{Max, One, WeightElement};
use num_traits::Zero;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MaxCut",
        display_name: "Max Cut",
        aliases: &["MaximumBipartiteSubgraph"],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
            VariantDimension::new("weight", "i64", &["i64", "One"]),
        ],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Find maximum weight cut in a graph",
        fields: MaxCutI64CreateSpec::FIELDS,
    }
}

/// The Maximum Cut problem.
///
/// Given a weighted graph G = (V, E) with edge weights w_e,
/// find a partition of V into sets S and V\S such that
/// the total weight of edges crossing the cut is maximized.
///
/// # Representation
///
/// Each vertex is assigned a binary value:
/// - 0: vertex is in set S
/// - 1: vertex is in set V\S
///
/// An edge contributes to the cut if its endpoints are in different sets.
///
/// # Type Parameters
///
/// * `G` - The graph type (e.g., `SimpleGraph`, `KingsSubgraph`, `UnitDiskGraph`)
/// * `W` - The weight type for edges (e.g., `i64`, `f64`)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::MaxCut;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::types::Max;
/// use problemreductions::{Problem, BruteForce};
///
/// // Create a triangle with unit weights
/// let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
/// let problem = MaxCut::new(graph, vec![1, 1, 1]);
///
/// // Solve with brute force
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_witnesses(&problem).unwrap();
///
/// // Maximum cut in triangle is 2 (any partition cuts 2 edges)
/// for sol in solutions {
///     let size = problem.evaluate(&sol).unwrap();
///     assert_eq!(size, Max(Some(2)));
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "MaxCutData<G, W>")]
#[serde(bound(deserialize = "G: Graph + Deserialize<'de>, W: Clone + Default + Deserialize<'de>"))]
pub struct MaxCut<G, W> {
    /// The underlying graph structure.
    graph: G,
    /// Weights for each edge (in the same order as graph.edges()).
    edge_weights: Vec<W>,
}

#[derive(Deserialize)]
struct MaxCutData<G, W> {
    graph: G,
    edge_weights: Vec<W>,
}

impl<G, W> TryFrom<MaxCutData<G, W>> for MaxCut<G, W>
where
    G: Graph,
    W: Clone + Default,
{
    type Error = crate::registry::ConstructionError;

    fn try_from(data: MaxCutData<G, W>) -> Result<Self, Self::Error> {
        Self::try_new(data.graph, data.edge_weights)
    }
}

macro_rules! max_cut_create_spec {
    ($name:ident, $weight:ty, $one:expr $(, $edge_weights:ident)?) => {
        #[derive(Debug, Deserialize, crate::CreateSpec)]
        struct $name {
            #[create(codec = "edge-list")]
            graph: Vec<(usize, usize)>,
            num_vertices: Option<usize>,
            $(
            #[create(codec = "comma-separated")]
            $edge_weights: Option<Vec<$weight>>,
            )?
        }

        impl TryFrom<$name> for MaxCut<SimpleGraph, $weight> {
            type Error = crate::registry::ConstructionError;

            fn try_from(spec: $name) -> Result<Self, Self::Error> {
                let graph = simple_graph_from_create(spec.graph, spec.num_vertices)?;
                let edge_weights = { $(if let Some(value) = spec.$edge_weights { value } else)? { vec![$one; graph.num_edges()] } };
                Self::try_new(graph, edge_weights)
            }
        }
    };
}

max_cut_create_spec!(MaxCutI64CreateSpec, i64, 1, edge_weights);
max_cut_create_spec!(MaxCutOneCreateSpec, One, One);

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

impl<G: Graph, W: Clone + Default> MaxCut<G, W> {
    /// Create a MaxCut problem from a graph with specified edge weights.
    ///
    /// # Arguments
    /// * `graph` - The underlying graph
    /// * `edge_weights` - Weights for each edge (must match graph.num_edges())
    pub fn new(graph: G, edge_weights: Vec<W>) -> Self {
        Self::try_new(graph, edge_weights).unwrap_or_else(|error| panic!("{error}"))
    }

    fn try_new(graph: G, edge_weights: Vec<W>) -> Result<Self, crate::registry::ConstructionError> {
        if edge_weights.len() != graph.num_edges() {
            return Err(crate::registry::ConstructionError::length_mismatch(
                "edge_weights",
                edge_weights.len(),
                graph.num_edges(),
            ));
        }
        Ok(Self {
            graph,
            edge_weights,
        })
    }

    /// Create a MaxCut problem with unit weights.
    pub fn unweighted(graph: G) -> Self
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

    /// Get the edges with weights.
    pub fn edges(&self) -> Vec<(usize, usize, W)> {
        self.graph
            .edges()
            .into_iter()
            .zip(self.edge_weights.iter())
            .map(|((u, v), w)| (u, v, w.clone()))
            .collect()
    }

    /// Get the weight of an edge by its index.
    pub fn edge_weight_by_index(&self, idx: usize) -> Option<&W> {
        self.edge_weights.get(idx)
    }

    /// Get the weight of an edge between vertices u and v.
    pub fn edge_weight(&self, u: usize, v: usize) -> Option<&W> {
        // Find the edge index
        for (idx, (eu, ev)) in self.graph.edges().iter().enumerate() {
            if (*eu == u && *ev == v) || (*eu == v && *ev == u) {
                return self.edge_weights.get(idx);
            }
        }
        None
    }

    /// Get edge weights only.
    pub fn edge_weights(&self) -> Vec<W> {
        self.edge_weights.clone()
    }

    /// Compute the cut size for a given partition configuration.
    pub fn cut_size(&self, config: &[bool]) -> Result<W::Sum, crate::traits::EvaluationError>
    where
        W: WeightElement,
    {
        cut_size(&self.graph, &self.edge_weights, config)
    }
}

impl<G: Graph, W: WeightElement> MaxCut<G, W> {
    /// Get the number of vertices in the underlying graph.
    pub fn num_vertices(&self) -> usize {
        self.graph().num_vertices()
    }

    /// Get the number of edges in the underlying graph.
    pub fn num_edges(&self) -> usize {
        self.graph().num_edges()
    }
}

impl<G, W> Problem for MaxCut<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "MaxCut";
    type Solution = Vec<bool>;
    type Value = Max<W::Sum>;

    crate::problem_parameters![("num_edges", num_edges), ("num_vertices", num_vertices),];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G, W]
    }

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Max<W::Sum>, crate::traits::EvaluationError> {
        if config.len() != self.graph.num_vertices() {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "cut assignment length does not match the graph vertices".into(),
            ));
        }
        Ok({
            // All cuts are valid, so always return Valid
            Max(Some(cut_size(&self.graph, &self.edge_weights, config)?))
        })
    }
}

impl<G, W> crate::solvers::BruteForceProblem for MaxCut<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.graph.num_vertices()]
    }
}

/// Compute the total weight of edges crossing the cut.
///
/// # Arguments
/// * `graph` - The graph structure
/// * `edge_weights` - Weights for each edge (same order as `graph.edges()`)
/// * `partition` - Boolean slice indicating which set each vertex belongs to
pub(crate) fn cut_size<G, W>(
    graph: &G,
    edge_weights: &[W],
    partition: &[bool],
) -> Result<W::Sum, crate::traits::EvaluationError>
where
    G: Graph,
    W: WeightElement,
{
    let mut total = W::Sum::zero();
    for ((u, v), weight) in graph.edges().iter().zip(edge_weights.iter()) {
        if *u < partition.len() && *v < partition.len() && partition[*u] != partition[*v] {
            total = W::checked_add_to_sum(total, weight.to_sum(), "summing cut-edge weights")?;
        }
    }
    Ok(total)
}

crate::impl_random_generate!(MaxCut<SimpleGraph, i64>, crate::random::SimpleGraphRandomSpec, |spec| {
    let graph = spec.graph()?;
    let weights = vec![1; graph.num_edges()];
    Ok(MaxCut::new(graph, weights))
});

crate::declare_variants! {
    default MaxCut<SimpleGraph, i64> => "2^(2.372 * num_vertices / 3)" create MaxCutI64CreateSpec random,
    MaxCut<SimpleGraph, One> => "2^(0.7907 * num_vertices)" create MaxCutOneCreateSpec,
}

crate::register_brute_force! {
    MaxCut<SimpleGraph, i64> decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
    MaxCut<SimpleGraph, One> decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![
        crate::example_db::specs::ModelExampleSpec {
            id: "max_cut_simplegraph",
            instance: Box::new(MaxCut::<_, i64>::unweighted(SimpleGraph::new(
                5,
                vec![(0, 1), (0, 2), (1, 3), (2, 3), (2, 4), (3, 4)],
            ))),
            optimal_config: serde_json::json!(vec![true, false, false, true, false]),
            optimal_value: serde_json::json!(5),
        },
        crate::example_db::specs::ModelExampleSpec {
            id: "max_cut_seven_edge_graph",
            instance: Box::new(MaxCut::new(
                SimpleGraph::new(
                    5,
                    vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 4), (2, 3), (3, 4)],
                ),
                vec![One; 7],
            )),
            optimal_config: serde_json::json!(vec![false, true, false, true, false]),
            optimal_value: serde_json::json!(6),
        },
    ]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/max_cut.rs"]
mod tests;

crate::decision_problem_meta!(MaxCut<SimpleGraph, i64>, "DecisionMaxCut");
crate::register_decision_variant!(
    MaxCut<SimpleGraph, i64>, "DecisionMaxCut", "2^(2.372 * num_vertices / 3)", &[],
    "Does a feasible solution have objective value >= the bound?",
    category: crate::registry::ProblemCategory::Graph,
    dims: [
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
            VariantDimension::new("weight", "i64", &["i64"]),
        ],
    fields: [
        crate::registry::FieldInfo { name: "graph", type_name: "Vec<(usize,usize)>", description: "Graph edges as comma-separated vertex pairs." },
        crate::registry::FieldInfo { name: "num_vertices", type_name: "usize", description: "Number of vertices, including isolated vertices." },
        crate::registry::FieldInfo { name: "edge_weights", type_name: "Vec<i64>", description: "Weights for each edge in graph order." },
        crate::registry::FieldInfo { name: "bound", type_name: "i64", description: "Accept objective values >= this bound" },
    ],
    decode: |_, indices: Vec<usize>| crate::config::config_to_bits(&indices)
);

#[cfg(feature = "example-db")]
pub(crate) fn decision_canonical_rule_example_specs(
) -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "decision_max_cut_to_max_cut",
        build: || {
            let source = crate::models::decision::Decision::new(
                MaxCut::<_, i64>::unweighted(SimpleGraph::new(
                    5,
                    vec![(0, 1), (0, 2), (1, 3), (2, 3), (2, 4), (3, 4)],
                )),
                5,
            );
            let witness = serde_json::json!(vec![true, false, false, true, false]);
            crate::example_db::specs::rule_example_with_witness::<_, MaxCut<SimpleGraph, i64>>(
                source,
                crate::export::SolutionPair {
                    source_config: witness.clone(),
                    target_config: witness,
                },
            )
        },
    }]
}
