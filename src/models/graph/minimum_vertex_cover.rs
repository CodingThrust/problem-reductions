//! Vertex Covering problem implementation.
//!
//! The Vertex Cover problem asks for a minimum weight subset of vertices
//! such that every edge has at least one endpoint in the subset.

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::{Min, One, WeightElement};
use num_traits::Zero;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumVertexCover",
        display_name: "Minimum Vertex Cover",
        aliases: &["MVC"],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
            VariantDimension::new("weight", "i32", &["i32", "One"]),
        ],
        module_path: module_path!(),
        description: "Find minimum weight vertex cover in a graph",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
            FieldInfo { name: "weights", type_name: "Vec<W>", description: "Vertex weights w: V -> R" },
        ],
    }
}

/// The Vertex Covering problem.
///
/// Given a graph G = (V, E) and weights w_v for each vertex,
/// find a subset S ⊆ V such that:
/// - Every edge has at least one endpoint in S (covering constraint)
/// - The total weight Σ_{v ∈ S} w_v is minimized
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::MinimumVertexCover;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Create a path graph 0-1-2
/// let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
/// let problem = MinimumVertexCover::new(graph, vec![1; 3]);
///
/// // Solve with brute force
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_witnesses(&problem);
///
/// // Minimum vertex cover is just vertex 1
/// assert!(solutions.contains(&vec![0, 1, 0]));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumVertexCover<G, W> {
    /// The underlying graph.
    graph: G,
    /// Weights for each vertex.
    weights: Vec<W>,
}

impl<G: Graph, W: Clone + Default> MinimumVertexCover<G, W> {
    /// Create a Vertex Covering problem from a graph with given weights.
    pub fn new(graph: G, weights: Vec<W>) -> Self {
        assert_eq!(
            weights.len(),
            graph.num_vertices(),
            "weights length must match graph num_vertices"
        );
        Self { graph, weights }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get a reference to the weights.
    pub fn weights(&self) -> &[W] {
        &self.weights
    }

    /// Check if the problem uses a non-unit weight type.
    pub fn is_weighted(&self) -> bool
    where
        W: WeightElement,
    {
        !W::IS_UNIT
    }

    /// Check if a configuration is a valid vertex cover.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        is_vertex_cover_config(&self.graph, config)
    }
}

impl<G: Graph, W: WeightElement> MinimumVertexCover<G, W> {
    /// Get the number of vertices in the underlying graph.
    pub fn num_vertices(&self) -> usize {
        self.graph().num_vertices()
    }

    /// Get the number of edges in the underlying graph.
    pub fn num_edges(&self) -> usize {
        self.graph().num_edges()
    }
}

impl<G, W> Problem for MinimumVertexCover<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "MinimumVertexCover";
    type Value = Min<W::Sum>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G, W]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_vertices()]
    }

    fn evaluate(&self, config: &[usize]) -> Min<W::Sum> {
        if !is_vertex_cover_config(&self.graph, config) {
            return Min(None);
        }
        let mut total = W::Sum::zero();
        for (i, &selected) in config.iter().enumerate() {
            if selected == 1 {
                total += self.weights[i].to_sum();
            }
        }
        Min(Some(total))
    }
}

/// Check if a configuration forms a valid vertex cover.
pub(crate) fn is_vertex_cover_config<G: Graph>(graph: &G, config: &[usize]) -> bool {
    for (u, v) in graph.edges() {
        let u_covered = config.get(u).copied().unwrap_or(0) == 1;
        let v_covered = config.get(v).copied().unwrap_or(0) == 1;
        if !u_covered && !v_covered {
            return false;
        }
    }
    true
}

crate::declare_variants! {
    default MinimumVertexCover<SimpleGraph, i32> => "1.1996^num_vertices",
    MinimumVertexCover<SimpleGraph, One> => "1.1996^num_vertices",
}

impl<G, W> crate::models::decision::DecisionProblemMeta for MinimumVertexCover<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
    W::Sum: std::fmt::Debug + serde::Serialize + serde::de::DeserializeOwned,
{
    const DECISION_NAME: &'static str = "DecisionMinimumVertexCover";
}

crate::register_decision_variant!(
    MinimumVertexCover<SimpleGraph, i32>,
    "DecisionMinimumVertexCover",
    "1.1996^num_vertices",
    &["VertexCover", "VC"],
    "Decision version: does a vertex cover of cost <= bound exist?",
    [
        FieldInfo {
            name: "graph",
            type_name: "G",
            description: "The underlying graph G=(V,E)",
        },
        FieldInfo {
            name: "weights",
            type_name: "Vec<W>",
            description: "Vertex weights w: V -> R",
        },
        FieldInfo {
            name: "bound",
            type_name: "i32",
            description: "Decision bound (maximum allowed cover cost)",
        },
    ]
);

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_vertex_cover_simplegraph_i32",
        instance: Box::new(MinimumVertexCover::new(
            SimpleGraph::new(5, vec![(0, 1), (0, 2), (1, 3), (2, 3), (2, 4), (3, 4)]),
            vec![1i32; 5],
        )),
        optimal_config: vec![1, 0, 0, 1, 1],
        optimal_value: serde_json::json!(3),
    }]
}

#[cfg(feature = "example-db")]
pub(crate) fn decision_canonical_model_example_specs(
) -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "decision_minimum_vertex_cover_simplegraph_i32",
        instance: Box::new(crate::models::decision::Decision::new(
            MinimumVertexCover::new(
                SimpleGraph::new(4, vec![(0, 1), (1, 2), (0, 2), (2, 3)]),
                vec![1i32; 4],
            ),
            2,
        )),
        optimal_config: vec![1, 0, 1, 0],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(feature = "example-db")]
pub(crate) fn decision_canonical_rule_example_specs(
) -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "decision_minimum_vertex_cover_to_minimum_vertex_cover",
        build: || {
            use crate::example_db::specs::assemble_rule_example;
            use crate::export::SolutionPair;
            use crate::rules::{AggregateReductionResult, ReduceToAggregate};

            let source = crate::models::decision::Decision::new(
                MinimumVertexCover::new(
                    SimpleGraph::new(4, vec![(0, 1), (1, 2), (0, 2), (2, 3)]),
                    vec![1i32; 4],
                ),
                2,
            );
            let result = source.reduce_to_aggregate();
            let target = result.target_problem();
            let config = vec![1, 0, 1, 0];
            assemble_rule_example(
                &source,
                target,
                vec![SolutionPair {
                    source_config: config.clone(),
                    target_config: config,
                }],
            )
        },
    }]
}

/// Check if a set of vertices forms a vertex cover.
///
/// # Arguments
/// * `graph` - The graph
/// * `selected` - Boolean slice indicating which vertices are selected
///
/// # Panics
/// Panics if `selected.len() != graph.num_vertices()`.
#[cfg(test)]
pub(crate) fn is_vertex_cover<G: Graph>(graph: &G, selected: &[bool]) -> bool {
    assert_eq!(
        selected.len(),
        graph.num_vertices(),
        "selected length must match num_vertices"
    );
    for (u, v) in graph.edges() {
        if !selected[u] && !selected[v] {
            return false;
        }
    }
    true
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/minimum_vertex_cover.rs"]
mod tests;
