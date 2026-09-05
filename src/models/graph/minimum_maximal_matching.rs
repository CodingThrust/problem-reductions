//! MinimumMaximalMatching problem implementation.
//!
//! The Minimum Maximal Matching problem asks for a matching of minimum size
//! that is maximal (cannot be extended by adding any edge).

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::{BipartiteGraph, Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumMaximalMatching",
        display_name: "Minimum Maximal Matching",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph", "BipartiteGraph"]),
        ],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Find a minimum-size matching that cannot be extended",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
        ],
    }
}

/// The Minimum Maximal Matching problem.
///
/// Given a graph G = (V, E), find a matching M ⊆ E of minimum cardinality
/// such that M is maximal: every edge not in M shares an endpoint with some
/// edge in M (i.e., M cannot be extended by adding any further edge).
///
/// # Type Parameters
///
/// * `G` - The graph type (e.g., `SimpleGraph`)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::MinimumMaximalMatching;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, BruteForce};
///
/// // Path graph P4: 0-1-2-3
/// let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
/// let problem = MinimumMaximalMatching::new(graph);
///
/// let solver = BruteForce::new();
/// let solution = solver.solve(&problem).unwrap().unwrap();
///
/// // Minimum maximal matching has 1 edge (e.g., edge (1,2))
/// let count = solution.iter().filter(|&&selected| selected).count();
/// assert_eq!(count, 1);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumMaximalMatching<G> {
    /// The underlying graph.
    graph: G,
}

impl<G: Graph> MinimumMaximalMatching<G> {
    /// Create a MinimumMaximalMatching problem from a graph.
    pub fn new(graph: G) -> Self {
        Self { graph }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get the number of vertices in the underlying graph.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of edges in the underlying graph.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    /// Check whether a configuration is a valid maximal matching.
    ///
    /// Returns `true` iff:
    /// 1. The selected edges form a matching (no two share an endpoint).
    /// 2. The matching is maximal (every non-selected edge shares an endpoint
    ///    with some selected edge).
    pub fn is_valid_maximal_matching(&self, config: &[bool]) -> bool {
        let edges = self.graph.edges();
        let n = self.graph.num_vertices();

        // Step 1: Check matching property.
        let mut vertex_used = vec![false; n];
        for (idx, &sel) in config.iter().enumerate() {
            if sel {
                let (u, v) = edges[idx];
                if vertex_used[u] || vertex_used[v] {
                    return false;
                }
                vertex_used[u] = true;
                vertex_used[v] = true;
            }
        }

        // Step 2: Check maximality — every unselected edge must be blocked.
        for (idx, &sel) in config.iter().enumerate() {
            if !sel {
                let (u, v) = edges[idx];
                // Edge (u,v) is blocked iff u or v is already matched.
                if !vertex_used[u] && !vertex_used[v] {
                    return false;
                }
            }
        }

        true
    }
}

impl<G> Problem for MinimumMaximalMatching<G>
where
    G: Graph + crate::variant::VariantParam,
{
    const NAME: &'static str = "MinimumMaximalMatching";
    type Solution = Vec<bool>;
    type Value = Min<i64>;

    crate::problem_parameters![("num_edges", num_edges), ("num_vertices", num_vertices),];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G]
    }

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Min<i64>, crate::traits::EvaluationError> {
        Ok({
            if config.len() != self.graph.num_edges() {
                return Err(crate::traits::EvaluationError::InvalidConfiguration(
                    "edge-selection length does not match the graph".into(),
                ));
            }
            if !self.is_valid_maximal_matching(config) {
                return Ok(Min(None));
            }
            let count = config.iter().filter(|&&selected| selected).count();
            Min(Some(i64::try_from(count).map_err(|_| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "converting matching cardinality to i64".into(),
                )
            })?))
        })
    }
}

impl<G> crate::solvers::BruteForceProblem for MinimumMaximalMatching<G>
where
    G: Graph + crate::variant::VariantParam,
{
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.graph.num_edges()]
    }
}

crate::impl_random_generate!(
    MinimumMaximalMatching<SimpleGraph>,
    crate::random::SimpleGraphRandomSpec,
    |spec| { Ok(MinimumMaximalMatching::new(spec.graph()?)) }
);

crate::declare_variants! {
    default MinimumMaximalMatching<SimpleGraph> => "1.3160^num_vertices" random,
    MinimumMaximalMatching<BipartiteGraph> => "1.3160^num_vertices",
}

crate::register_brute_force! {
    MinimumMaximalMatching<SimpleGraph> decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
    MinimumMaximalMatching<BipartiteGraph> decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // Path graph P6: 6 vertices, edges [(0,1),(1,2),(2,3),(3,4),(4,5)]
    // config [0,1,0,1,0] = edges {(1,2),(3,4)} — a maximal matching of size 2.
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_maximal_matching_simplegraph",
        instance: Box::new(MinimumMaximalMatching::new(SimpleGraph::new(
            6,
            vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5)],
        ))),
        optimal_config: serde_json::json!(vec![false, true, false, true, false]),
        optimal_value: serde_json::json!(2),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/minimum_maximal_matching.rs"]
mod tests;
