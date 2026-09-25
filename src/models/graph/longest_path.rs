//! Longest Path problem implementation.
//!
//! The Longest Path problem asks for a simple path between two distinguished
//! vertices that maximizes the total edge length.

use crate::registry::{CreateSpec, ProblemSchemaEntry, VariantDimension};
use crate::topology::{is_simple_st_path, Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::{Max, One, WeightElement};
use num_traits::Zero;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "LongestPath",
        display_name: "Longest Path",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
            VariantDimension::new("weight", "i64", &["i64", "One"]),
        ],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Find a simple s-t path of maximum total edge length",
        fields: LongestPathI64CreateSpec::FIELDS,
    }
}

/// The Longest Path problem.
///
/// Given a graph `G = (V, E)` with positive edge lengths `l(e)` and
/// distinguished vertices `s` and `t`, find a simple path from `s` to `t`
/// maximizing the total length of its selected edges.
///
/// # Representation
///
/// Each edge is assigned a binary variable:
/// - `0`: the edge is not selected
/// - `1`: the edge is selected
///
/// A valid configuration must select exactly the edges of one simple
/// undirected path from `source_vertex` to `target_vertex`.
#[derive(Debug, Clone, Serialize)]
pub struct LongestPath<G, W: WeightElement> {
    graph: G,
    edge_lengths: Vec<W>,
    source_vertex: usize,
    target_vertex: usize,
}

#[derive(Deserialize)]
#[serde(bound(deserialize = "G: Graph + Deserialize<'de>, W: WeightElement + Deserialize<'de>"))]
struct LongestPathData<G, W: WeightElement> {
    graph: G,
    edge_lengths: Vec<W>,
    source_vertex: usize,
    target_vertex: usize,
}

impl<'de, G, W> Deserialize<'de> for LongestPath<G, W>
where
    G: Graph + Deserialize<'de>,
    W: WeightElement + Deserialize<'de>,
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let data = LongestPathData::<G, W>::deserialize(deserializer)?;
        Self::try_new(
            data.graph,
            data.edge_lengths,
            data.source_vertex,
            data.target_vertex,
        )
        .map_err(serde::de::Error::custom)
    }
}

macro_rules! longest_path_create_spec {
    (@lengths $spec:ident, $lengths:ident) => { $spec.$lengths };
    (@lengths $spec:ident) => { vec![One; $spec.graph.len()] };
    ($name:ident,$weight:ty $(, $lengths:ident)?) => {
        #[derive(Debug, Deserialize, crate::CreateSpec)]
        struct $name {
            #[create(codec = "edge-list")]
            graph: Vec<(usize, usize)>,
            num_vertices: Option<usize>,
            $(#[create(codec = "comma-separated")]
            $lengths: Vec<$weight>,)?
            source_vertex: usize,
            target_vertex: usize,
        }
        impl TryFrom<$name> for LongestPath<SimpleGraph, $weight> {
            type Error = crate::registry::ConstructionError;
            fn try_from(spec: $name) -> Result<Self, crate::registry::ConstructionError> {
                if spec.graph.is_empty() && spec.num_vertices.is_none() {
                    return Err("num_vertices is required for an empty graph".into());
                }
                for &(u, v) in &spec.graph {
                    if u == v {
                        return Err("self-loops are not allowed".into());
                    }
                }
                let inferred = spec
                    .graph
                    .iter()
                    .flat_map(|&(u, v)| [u, v])
                    .max()
                    .map(|v| v.checked_add(1).ok_or("vertex count overflows usize"))
                    .transpose()?
                    .unwrap_or(0);
                let count = spec.num_vertices.unwrap_or(inferred);
                if count < inferred {
                    return Err("num_vertices is too small".into());
                }
                let edge_lengths = longest_path_create_spec!(@lengths spec $(, $lengths)?);
                Self::try_new(
                    SimpleGraph::new(count, spec.graph),
                    edge_lengths,
                    spec.source_vertex,
                    spec.target_vertex,
                )
            }
        }
    };
}
longest_path_create_spec!(LongestPathI64CreateSpec, i64, edge_lengths);
longest_path_create_spec!(LongestPathOneCreateSpec, One);

impl<G: Graph, W: WeightElement> LongestPath<G, W> {
    /// Create a new LongestPath instance.
    pub fn new(graph: G, edge_lengths: Vec<W>, source_vertex: usize, target_vertex: usize) -> Self {
        Self::try_new(graph, edge_lengths, source_vertex, target_vertex)
            .unwrap_or_else(|error| panic!("{error}"))
    }

    fn try_new(
        graph: G,
        edge_lengths: Vec<W>,
        source_vertex: usize,
        target_vertex: usize,
    ) -> Result<Self, crate::registry::ConstructionError> {
        Self::check_weights(&graph, &edge_lengths)?;
        if source_vertex >= graph.num_vertices() {
            return Err(format!(
                "source_vertex {} out of bounds (graph has {} vertices)",
                source_vertex,
                graph.num_vertices()
            )
            .into());
        }
        if target_vertex >= graph.num_vertices() {
            return Err(format!(
                "target_vertex {} out of bounds (graph has {} vertices)",
                target_vertex,
                graph.num_vertices()
            )
            .into());
        }
        Ok(Self {
            graph,
            edge_lengths,
            source_vertex,
            target_vertex,
        })
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get the edge lengths.
    pub fn edge_lengths(&self) -> &[W] {
        &self.edge_lengths
    }

    /// Replace the edge lengths with a new vector.
    pub fn set_lengths(&mut self, edge_lengths: Vec<W>) {
        Self::check_weights(&self.graph, &edge_lengths).unwrap_or_else(|error| panic!("{error}"));
        self.edge_lengths = edge_lengths;
    }

    fn check_weights(graph: &G, weights: &[W]) -> Result<(), crate::registry::ConstructionError> {
        if weights.len() != graph.num_edges() {
            return Err("edge_lengths length must match num_edges".into());
        }
        if !weights
            .iter()
            .all(|weight| weight.to_sum() > W::Sum::zero())
        {
            return Err("All edge lengths must be positive (> 0)".into());
        }
        Ok(())
    }

    /// Get the source vertex.
    pub fn source_vertex(&self) -> usize {
        self.source_vertex
    }

    /// Get the target vertex.
    pub fn target_vertex(&self) -> usize {
        self.target_vertex
    }

    /// Check whether this problem uses non-unit edge lengths.
    pub fn is_weighted(&self) -> bool {
        !W::IS_UNIT
    }

    /// Get the number of vertices in the graph.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of edges in the graph.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    /// Check if a configuration encodes a valid simple source-target path.
    pub fn is_valid_solution(&self, config: &[bool]) -> bool {
        is_simple_st_path(
            self.graph.num_vertices(),
            &self.graph.edges(),
            self.source_vertex,
            self.target_vertex,
            config,
        )
    }
}

impl<G, W> Problem for LongestPath<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "LongestPath";
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
        if config.len() != self.graph.num_edges() {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "edge-selection length does not match the graph".into(),
            ));
        }
        Ok({
            if !self.is_valid_solution(config) {
                return Ok(Max(None));
            }

            let mut total = W::Sum::zero();
            for (idx, &selected) in config.iter().enumerate() {
                if selected {
                    total = W::checked_add_to_sum(
                        total,
                        self.edge_lengths[idx].to_sum(),
                        "summing path edge lengths",
                    )?;
                }
            }
            Max(Some(total))
        })
    }
}

impl<G, W> crate::solvers::BruteForceProblem for LongestPath<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.graph.num_edges()]
    }
}

crate::declare_variants! {
    default LongestPath<SimpleGraph, i64> => "num_vertices * 2^num_vertices" create LongestPathI64CreateSpec,
    LongestPath<SimpleGraph, One> => "num_vertices * 2^num_vertices" create LongestPathOneCreateSpec,
}

crate::register_brute_force! {
    LongestPath<SimpleGraph, i64> decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
    LongestPath<SimpleGraph, One> decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "longest_path_simplegraph",
        instance: Box::new(LongestPath::new(
            SimpleGraph::new(
                7,
                vec![
                    (0, 1),
                    (0, 2),
                    (1, 3),
                    (2, 3),
                    (2, 4),
                    (3, 5),
                    (4, 5),
                    (4, 6),
                    (5, 6),
                    (1, 6),
                ],
            ),
            vec![3, 2, 4, 1, 5, 2, 3, 2, 4, 1],
            0,
            6,
        )),
        optimal_config: serde_json::json!(vec![
            true, false, true, true, true, false, true, false, true, false
        ]),
        optimal_value: serde_json::json!(20),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/longest_path.rs"]
mod tests;

crate::decision_problem_meta!(LongestPath<SimpleGraph, One>, "DecisionLongestPath");
crate::register_decision_variant!(
    LongestPath<SimpleGraph, One>, "DecisionLongestPath", "num_vertices * 2^num_vertices", &[],
    "Does a feasible solution have objective value >= the bound?",
    category: crate::registry::ProblemCategory::Graph,
    dims: [
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
            VariantDimension::new("weight", "One", &["One"]),
        ],
    fields: [
        crate::registry::FieldInfo { name: "graph", type_name: "Vec<(usize,usize)>", description: "Graph edges as comma-separated vertex pairs." },
        crate::registry::FieldInfo { name: "num_vertices", type_name: "usize", description: "Number of vertices, including isolated vertices." },
        crate::registry::FieldInfo { name: "source_vertex", type_name: "usize", description: "Start vertex of the path." },
        crate::registry::FieldInfo { name: "target_vertex", type_name: "usize", description: "End vertex of the path." },
        crate::registry::FieldInfo { name: "bound", type_name: "i64", description: "Accept objective values >= this bound" },
    ],
    decode: |_, indices: Vec<usize>| crate::config::config_to_bits(&indices)
);

#[cfg(feature = "example-db")]
pub(crate) fn decision_canonical_rule_example_specs(
) -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "decision_longest_path_to_longest_path",
        build: || {
            let source = crate::models::decision::Decision::new(
                LongestPath::new(SimpleGraph::path(3), vec![crate::types::One; 2], 0, 2),
                2,
            );
            let witness = serde_json::json!(vec![true, true]);
            crate::example_db::specs::rule_example_with_witness::<_, LongestPath<SimpleGraph, One>>(
                source,
                crate::export::SolutionPair {
                    source_config: witness.clone(),
                    target_config: witness,
                },
            )
        },
    }]
}
