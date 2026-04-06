//! Dominating Set problem implementation.
//!
//! The Dominating Set problem asks for a minimum weight subset of vertices
//! such that every vertex is either in the set or adjacent to a vertex in the set.

use crate::models::decision::Decision;
use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::rules::{EdgeCapabilities, ReduceToAggregate, ReductionEntry, ReductionOverhead};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::{Min, ProblemSize, WeightElement};
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumDominatingSet",
        display_name: "Minimum Dominating Set",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
            VariantDimension::new("weight", "i32", &["i32"]),
        ],
        module_path: module_path!(),
        description: "Find minimum weight dominating set in a graph",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
            FieldInfo { name: "weights", type_name: "Vec<W>", description: "Vertex weights w: V -> R" },
        ],
    }
}

/// The Dominating Set problem.
///
/// Given a graph G = (V, E) and weights w_v for each vertex,
/// find a subset D ⊆ V such that:
/// - Every vertex is either in D or adjacent to a vertex in D (domination)
/// - The total weight Σ_{v ∈ D} w_v is minimized
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::MinimumDominatingSet;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Star graph: center dominates all
/// let graph = SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3)]);
/// let problem = MinimumDominatingSet::new(graph, vec![1; 4]);
///
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_witnesses(&problem);
///
/// // Minimum dominating set is just the center vertex
/// assert!(solutions.contains(&vec![1, 0, 0, 0]));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumDominatingSet<G, W> {
    /// The underlying graph.
    graph: G,
    /// Weights for each vertex.
    weights: Vec<W>,
}

impl<G: Graph, W: Clone + Default> MinimumDominatingSet<G, W> {
    /// Create a Dominating Set problem from a graph with given weights.
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

    /// Get neighbors of a vertex.
    pub fn neighbors(&self, v: usize) -> Vec<usize> {
        self.graph.neighbors(v)
    }

    /// Get the closed neighborhood `N[v] = {v} ∪ N(v)`.
    pub fn closed_neighborhood(&self, v: usize) -> HashSet<usize> {
        let mut neighborhood: HashSet<usize> = self.neighbors(v).into_iter().collect();
        neighborhood.insert(v);
        neighborhood
    }

    /// Get a reference to the weights slice.
    pub fn weights(&self) -> &[W] {
        &self.weights
    }

    /// Check if a configuration is a valid dominating set.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        self.is_dominating(config)
    }

    /// Check if a set of vertices is a dominating set.
    fn is_dominating(&self, config: &[usize]) -> bool {
        let n = self.graph.num_vertices();
        let mut dominated = vec![false; n];

        for (v, &selected) in config.iter().enumerate() {
            if selected == 1 {
                // v dominates itself
                dominated[v] = true;
                // v dominates all its neighbors
                for neighbor in self.neighbors(v) {
                    if neighbor < n {
                        dominated[neighbor] = true;
                    }
                }
            }
        }

        dominated.iter().all(|&d| d)
    }
}

impl<G: Graph, W: WeightElement> MinimumDominatingSet<G, W> {
    /// Get the number of vertices in the underlying graph.
    pub fn num_vertices(&self) -> usize {
        self.graph().num_vertices()
    }

    /// Get the number of edges in the underlying graph.
    pub fn num_edges(&self) -> usize {
        self.graph().num_edges()
    }
}

impl<G, W> Problem for MinimumDominatingSet<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "MinimumDominatingSet";
    type Value = Min<W::Sum>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G, W]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_vertices()]
    }

    fn evaluate(&self, config: &[usize]) -> Min<W::Sum> {
        if !self.is_dominating(config) {
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

crate::declare_variants! {
    default MinimumDominatingSet<SimpleGraph, i32> => "1.4969^num_vertices",
}

crate::decision_problem_meta!(
    MinimumDominatingSet<SimpleGraph, i32>,
    "DecisionMinimumDominatingSet"
);

impl Decision<MinimumDominatingSet<SimpleGraph, i32>> {
    /// Number of vertices in the underlying graph.
    pub fn num_vertices(&self) -> usize {
        self.inner().num_vertices()
    }

    /// Number of edges in the underlying graph.
    pub fn num_edges(&self) -> usize {
        self.inner().num_edges()
    }

    /// Decision bound as a nonnegative integer.
    pub fn k(&self) -> usize {
        (*self.bound()).try_into().unwrap_or(0)
    }
}

crate::declare_variants! {
    default Decision<MinimumDominatingSet<SimpleGraph, i32>> => "1.4969^num_vertices",
}

inventory::submit! {
    ProblemSchemaEntry {
        name: "DecisionMinimumDominatingSet",
        display_name: "Decision Minimum Dominating Set",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
            VariantDimension::new("weight", "i32", &["i32"]),
        ],
        module_path: module_path!(),
        description: "Decision version: does a dominating set of cost <= bound exist?",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
            FieldInfo { name: "weights", type_name: "Vec<W>", description: "Vertex weights w: V -> R" },
            FieldInfo { name: "bound", type_name: "i32", description: "Decision bound (maximum allowed dominating-set cost)" },
        ],
    }
}

inventory::submit! {
    ReductionEntry {
        source_name: "DecisionMinimumDominatingSet",
        target_name: "MinimumDominatingSet",
        source_variant_fn: <Decision<MinimumDominatingSet<SimpleGraph, i32>> as Problem>::variant,
        target_variant_fn: <MinimumDominatingSet<SimpleGraph, i32> as Problem>::variant,
        overhead_fn: || ReductionOverhead::identity(&["num_vertices", "num_edges"]),
        module_path: module_path!(),
        reduce_fn: None,
        reduce_aggregate_fn: Some(|any| {
            let source = any
                .downcast_ref::<Decision<MinimumDominatingSet<SimpleGraph, i32>>>()
                .expect("DecisionMinimumDominatingSet aggregate reduction source type mismatch");
            Box::new(source.reduce_to_aggregate())
        }),
        capabilities: EdgeCapabilities::aggregate_only(),
        overhead_eval_fn: |any| {
            let source = any
                .downcast_ref::<Decision<MinimumDominatingSet<SimpleGraph, i32>>>()
                .expect("DecisionMinimumDominatingSet overhead source type mismatch");
            ProblemSize::new(vec![
                ("num_vertices", source.num_vertices()),
                ("num_edges", source.num_edges()),
            ])
        },
        source_size_fn: |any| {
            let source = any
                .downcast_ref::<Decision<MinimumDominatingSet<SimpleGraph, i32>>>()
                .expect("DecisionMinimumDominatingSet size source type mismatch");
            ProblemSize::new(vec![
                ("num_vertices", source.num_vertices()),
                ("num_edges", source.num_edges()),
                ("k", source.k()),
            ])
        },
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_dominating_set_simplegraph_i32",
        instance: Box::new(MinimumDominatingSet::new(
            SimpleGraph::new(5, vec![(0, 1), (0, 2), (1, 3), (2, 3), (2, 4), (3, 4)]),
            vec![1i32; 5],
        )),
        optimal_config: vec![0, 0, 1, 1, 0],
        optimal_value: serde_json::json!(2),
    }]
}

/// Check if a set of vertices is a dominating set.
///
/// # Panics
/// Panics if `selected.len() != graph.num_vertices()`.
#[cfg(test)]
pub(crate) fn is_dominating_set<G: Graph>(graph: &G, selected: &[bool]) -> bool {
    assert_eq!(
        selected.len(),
        graph.num_vertices(),
        "selected length must match num_vertices"
    );

    // Check each vertex is dominated
    for v in 0..graph.num_vertices() {
        if selected[v] {
            continue; // v dominates itself
        }
        // Check if any neighbor of v is selected
        if !graph.neighbors(v).iter().any(|&u| selected[u]) {
            return false;
        }
    }

    true
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/minimum_dominating_set.rs"]
mod tests;
