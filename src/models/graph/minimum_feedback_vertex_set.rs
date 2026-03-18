//! Feedback Vertex Set problem implementation.
//!
//! The Feedback Vertex Set problem asks for a minimum weight subset of vertices
//! whose removal makes the directed graph acyclic (a DAG).

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::DirectedGraph;
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize, WeightElement};
use num_traits::Zero;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumFeedbackVertexSet",
        display_name: "Minimum Feedback Vertex Set",
        aliases: &["FVS"],
        dimensions: &[
            VariantDimension::new("weight", "i32", &["i32"]),
        ],
        module_path: module_path!(),
        description: "Find minimum weight feedback vertex set in a directed graph",
        fields: &[
            FieldInfo { name: "graph", type_name: "DirectedGraph", description: "The directed graph G=(V,A)" },
            FieldInfo { name: "weights", type_name: "Vec<W>", description: "Vertex weights w: V -> R" },
        ],
    }
}

/// The Minimum Feedback Vertex Set problem.
///
/// Given a directed graph G = (V, A) and weights w_v for each vertex,
/// find a subset F ⊆ V such that:
/// - Removing F from G yields a directed acyclic graph (DAG)
/// - The total weight Σ_{v ∈ F} w_v is minimized
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::MinimumFeedbackVertexSet;
/// use problemreductions::topology::DirectedGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Simple 3-cycle: 0 → 1 → 2 → 0
/// let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]);
/// let problem = MinimumFeedbackVertexSet::new(graph, vec![1; 3]);
///
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_best(&problem);
///
/// // Any single vertex breaks the cycle
/// assert_eq!(solutions.len(), 3);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumFeedbackVertexSet<W> {
    /// The underlying directed graph.
    graph: DirectedGraph,
    /// Weights for each vertex.
    weights: Vec<W>,
}

impl<W: Clone + Default> MinimumFeedbackVertexSet<W> {
    /// Create a Feedback Vertex Set problem from a directed graph with given weights.
    pub fn new(graph: DirectedGraph, weights: Vec<W>) -> Self {
        assert_eq!(
            weights.len(),
            graph.num_vertices(),
            "weights length must match graph num_vertices"
        );
        Self { graph, weights }
    }

    /// Get a reference to the underlying directed graph.
    pub fn graph(&self) -> &DirectedGraph {
        &self.graph
    }

    /// Get a reference to the weights slice.
    pub fn weights(&self) -> &[W] {
        &self.weights
    }

    /// Set vertex weights.
    pub fn set_weights(&mut self, weights: Vec<W>) {
        assert_eq!(
            weights.len(),
            self.graph.num_vertices(),
            "weights length must match graph num_vertices"
        );
        self.weights = weights;
    }

    /// Check if a configuration is a valid feedback vertex set.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        if config.len() != self.graph.num_vertices() {
            return false;
        }
        let keep: Vec<bool> = config.iter().map(|&c| c == 0).collect();
        self.graph.induced_subgraph(&keep).is_dag()
    }
}

impl<W: WeightElement> MinimumFeedbackVertexSet<W> {
    /// Check if the problem has non-unit weights.
    pub fn is_weighted(&self) -> bool {
        !W::IS_UNIT
    }

    /// Get the number of vertices in the underlying directed graph.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of arcs in the underlying directed graph.
    pub fn num_arcs(&self) -> usize {
        self.graph.num_arcs()
    }
}

impl<W> Problem for MinimumFeedbackVertexSet<W>
where
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "MinimumFeedbackVertexSet";
    type Metric = SolutionSize<W::Sum>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![W]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_vertices()]
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<W::Sum> {
        if config.len() != self.graph.num_vertices() {
            return SolutionSize::Invalid;
        }
        // keep[v] = true if vertex v is NOT selected for removal
        let keep: Vec<bool> = config.iter().map(|&c| c == 0).collect();
        let subgraph = self.graph.induced_subgraph(&keep);
        if !subgraph.is_dag() {
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

impl<W> OptimizationProblem for MinimumFeedbackVertexSet<W>
where
    W: WeightElement + crate::variant::VariantParam,
{
    type Value = W::Sum;

    fn direction(&self) -> Direction {
        Direction::Minimize
    }
}

crate::declare_variants! {
    default opt MinimumFeedbackVertexSet<i32> => "1.9977^num_vertices",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    use crate::topology::DirectedGraph;
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_feedback_vertex_set_i32",
        instance: Box::new(MinimumFeedbackVertexSet::new(
            DirectedGraph::new(
                5,
                vec![(0, 1), (1, 2), (2, 0), (0, 3), (3, 4), (4, 1), (4, 2)],
            ),
            vec![1i32; 5],
        )),
        optimal_config: vec![1, 0, 0, 0, 0],
        optimal_value: serde_json::json!({"Valid": 1}),
    }]
}

/// Check if a set of vertices is a feedback vertex set (removing them makes the graph a DAG).
///
/// # Panics
/// Panics if `selected.len() != graph.num_vertices()`.
#[cfg(test)]
pub(crate) fn is_feedback_vertex_set(graph: &DirectedGraph, selected: &[bool]) -> bool {
    assert_eq!(
        selected.len(),
        graph.num_vertices(),
        "selected length must match num_vertices"
    );
    // keep = NOT selected
    let keep: Vec<bool> = selected.iter().map(|&s| !s).collect();
    graph.induced_subgraph(&keep).is_dag()
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/minimum_feedback_vertex_set.rs"]
mod tests;
