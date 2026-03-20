//! Independent Set problem implementation.
//!
//! The Independent Set problem asks for a maximum weight subset of vertices
//! such that no two vertices in the subset are adjacent.

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, KingsSubgraph, SimpleGraph, TriangularSubgraph, UnitDiskGraph};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, One, SolutionSize, WeightElement};
use num_traits::Zero;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MaximumIndependentSet",
        display_name: "Maximum Independent Set",
        aliases: &["MIS"],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph", "KingsSubgraph", "TriangularSubgraph", "UnitDiskGraph"]),
            VariantDimension::new("weight", "One", &["One", "i32"]),
        ],
        module_path: module_path!(),
        description: "Find maximum weight independent set in a graph",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
            FieldInfo { name: "weights", type_name: "Vec<W>", description: "Vertex weights w: V -> R" },
        ],
    }
}

/// The Independent Set problem.
///
/// Given a graph G = (V, E) and weights w_v for each vertex,
/// find a subset S ⊆ V such that:
/// - No two vertices in S are adjacent (independent set constraint)
/// - The total weight Σ_{v ∈ S} w_v is maximized
///
/// # Type Parameters
///
/// * `G` - The graph type (e.g., `SimpleGraph`, `KingsSubgraph`, `UnitDiskGraph`)
/// * `W` - The weight type (e.g., `i32`, `f64`, `One`)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::MaximumIndependentSet;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Create a triangle graph (3 vertices, 3 edges)
/// let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
/// let problem = MaximumIndependentSet::new(graph, vec![1; 3]);
///
/// // Solve with brute force
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_best(&problem);
///
/// // Maximum independent set in a triangle has size 1
/// assert!(solutions.iter().all(|s| s.iter().sum::<usize>() == 1));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaximumIndependentSet<G, W> {
    /// The underlying graph.
    graph: G,
    /// Weights for each vertex.
    weights: Vec<W>,
}

impl<G: Graph, W: Clone + Default> MaximumIndependentSet<G, W> {
    /// Create an Independent Set problem from a graph with given weights.
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

    /// Check if a configuration is a valid independent set.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        is_independent_set_config(&self.graph, config)
    }
}

impl<G: Graph, W: WeightElement> MaximumIndependentSet<G, W> {
    /// Get the number of vertices in the underlying graph.
    pub fn num_vertices(&self) -> usize {
        self.graph().num_vertices()
    }

    /// Get the number of edges in the underlying graph.
    pub fn num_edges(&self) -> usize {
        self.graph().num_edges()
    }
}

impl<G, W> Problem for MaximumIndependentSet<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "MaximumIndependentSet";
    type Metric = SolutionSize<W::Sum>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G, W]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_vertices()]
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<W::Sum> {
        if !is_independent_set_config(&self.graph, config) {
            return SolutionSize::Invalid;
        }
        let mut total = W::Sum::zero();
        for (i, &selected) in config.iter().enumerate() {
            if selected == 1 {
                total += self.weights[i].to_sum();
            }
        }
        SolutionSize::Valid(total)
    }
}

impl<G, W> OptimizationProblem for MaximumIndependentSet<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    type Value = W::Sum;

    fn direction(&self) -> Direction {
        Direction::Maximize
    }
}

/// Check if a configuration forms a valid independent set.
fn is_independent_set_config<G: Graph>(graph: &G, config: &[usize]) -> bool {
    for (u, v) in graph.edges() {
        if config.get(u).copied().unwrap_or(0) == 1 && config.get(v).copied().unwrap_or(0) == 1 {
            return false;
        }
    }
    true
}

crate::declare_variants! {
    opt MaximumIndependentSet<SimpleGraph, i32>        => "1.1996^num_vertices",
    default opt MaximumIndependentSet<SimpleGraph, One>         => "1.1996^num_vertices",
    opt MaximumIndependentSet<KingsSubgraph, i32>      => "2^sqrt(num_vertices)",
    opt MaximumIndependentSet<KingsSubgraph, One>       => "2^sqrt(num_vertices)",
    opt MaximumIndependentSet<TriangularSubgraph, i32> => "2^sqrt(num_vertices)",
    opt MaximumIndependentSet<UnitDiskGraph, i32>      => "2^sqrt(num_vertices)",
    opt MaximumIndependentSet<UnitDiskGraph, One>       => "2^sqrt(num_vertices)",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![
        crate::example_db::specs::ModelExampleSpec {
            id: "maximum_independent_set_simplegraph_one",
            instance: Box::new(MaximumIndependentSet::new(
                SimpleGraph::new(
                    10,
                    vec![
                        (0, 1),
                        (1, 2),
                        (2, 3),
                        (3, 4),
                        (4, 0),
                        (5, 7),
                        (7, 9),
                        (9, 6),
                        (6, 8),
                        (8, 5),
                        (0, 5),
                        (1, 6),
                        (2, 7),
                        (3, 8),
                        (4, 9),
                    ],
                ),
                vec![One; 10],
            )),
            optimal_config: vec![1, 0, 1, 0, 0, 0, 0, 0, 1, 1],
            optimal_value: serde_json::json!({"Valid": 4}),
        },
        crate::example_db::specs::ModelExampleSpec {
            id: "maximum_independent_set_simplegraph_i32",
            instance: Box::new(MaximumIndependentSet::new(
                SimpleGraph::new(
                    10,
                    vec![
                        (0, 1),
                        (1, 2),
                        (2, 3),
                        (3, 4),
                        (4, 0),
                        (5, 7),
                        (7, 9),
                        (9, 6),
                        (6, 8),
                        (8, 5),
                        (0, 5),
                        (1, 6),
                        (2, 7),
                        (3, 8),
                        (4, 9),
                    ],
                ),
                vec![5, 1, 1, 1, 1, 3, 1, 1, 1, 3],
            )),
            optimal_config: vec![1, 0, 1, 0, 0, 0, 0, 0, 1, 1],
            optimal_value: serde_json::json!({"Valid": 10}),
        },
    ]
}

/// Check if a set of vertices forms an independent set.
///
/// # Arguments
/// * `graph` - The graph
/// * `selected` - Boolean slice indicating which vertices are selected
///
/// # Panics
/// Panics if `selected.len() != graph.num_vertices()`.
#[cfg(test)]
pub(crate) fn is_independent_set<G: Graph>(graph: &G, selected: &[bool]) -> bool {
    assert_eq!(
        selected.len(),
        graph.num_vertices(),
        "selected length must match num_vertices"
    );
    for (u, v) in graph.edges() {
        if selected[u] && selected[v] {
            return false;
        }
    }
    true
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/maximum_independent_set.rs"]
mod tests;
