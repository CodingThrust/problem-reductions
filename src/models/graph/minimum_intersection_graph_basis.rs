//! Minimum Intersection Graph Basis problem implementation.
//!
//! Given a graph G = (V, E), find a universe U of minimum cardinality such that
//! each vertex v can be assigned a subset S[v] ⊆ U with the intersection graph
//! of {S[v]} equal to G.

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumIntersectionGraphBasis",
        display_name: "Minimum Intersection Graph Basis",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
        ],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Find minimum universe size for intersection graph representation",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
        ],
    }
}

/// The Minimum Intersection Graph Basis problem.
///
/// Given a graph G = (V, E), find a universe U of minimum cardinality and
/// an assignment of subsets `S[v]` ⊆ U for each vertex v ∈ V such that:
/// - For every edge (u, v) ∈ E: `S[u] ∩ S[v] ≠ ∅`
/// - For every non-edge pair (u, v) ∉ E: `S[u] ∩ S[v] = ∅`
/// - |U| is minimized
///
/// The minimum |U| is the *intersection number* of G.
///
/// Variables: n × |E| binary variables where n = |V| and |E| is the upper bound
/// on universe size. `config[v * |E| + s] = 1` means element s ∈ `S[v]`.
///
/// # Type Parameters
///
/// * `G` - The graph type (e.g., `SimpleGraph`)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::MinimumIntersectionGraphBasis;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, BruteForce};
///
/// // Path P3: 3 vertices, edges (0,1), (1,2)
/// let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
/// let problem = MinimumIntersectionGraphBasis::new(graph);
///
/// let solver = BruteForce::new();
/// let solution = solver.solve(&problem).unwrap().unwrap();
/// let value = problem.evaluate(&solution).unwrap();
/// assert_eq!(value, problemreductions::types::Min(Some(2)));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumIntersectionGraphBasis<G> {
    /// The underlying graph.
    graph: G,
}

impl<G: Graph> MinimumIntersectionGraphBasis<G> {
    /// Create a MinimumIntersectionGraphBasis problem from a graph.
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
}

impl<G> Problem for MinimumIntersectionGraphBasis<G>
where
    G: Graph + crate::variant::VariantParam,
{
    const NAME: &'static str = "MinimumIntersectionGraphBasis";
    type Solution = Vec<Vec<bool>>;
    type Value = Min<i64>;

    crate::problem_parameters![("num_edges", num_edges), ("num_vertices", num_vertices),];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G]
    }

    fn evaluate(
        &self,
        solution: &Self::Solution,
    ) -> Result<Min<i64>, crate::traits::EvaluationError> {
        let n = self.graph.num_vertices();
        let m = self.graph.num_edges();
        if solution.len() != n || solution.iter().any(|subset| subset.len() != m) {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "intersection-basis dimensions do not match the graph".into(),
            ));
        }
        Ok({
            if m == 0 {
                return Ok(Min(Some(0)));
            }

            // Parse subsets: S[v] contains element s when solution[v][s] is true.
            let subsets: Vec<HashSet<usize>> = solution
                .iter()
                .map(|row| {
                    row.iter()
                        .enumerate()
                        .filter_map(|(element, &selected)| selected.then_some(element))
                        .collect()
                })
                .collect();

            // Check edge constraints: for every edge (u, v), S[u] ∩ S[v] ≠ ∅
            let edges = self.graph.edges();
            for &(u, v) in &edges {
                if subsets[u].is_disjoint(&subsets[v]) {
                    return Ok(Min(None));
                }
            }

            // Check non-edge constraints: for every non-edge pair (u, v), S[u] ∩ S[v] = ∅
            for u in 0..n {
                for v in (u + 1)..n {
                    if !self.graph.has_edge(u, v) && !subsets[u].is_disjoint(&subsets[v]) {
                        return Ok(Min(None));
                    }
                }
            }

            // Count elements used (union of all subsets)
            let used: HashSet<usize> = subsets.iter().flat_map(|s| s.iter().copied()).collect();
            Min(Some(i64::try_from(used.len()).map_err(|_| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "converting intersection-basis size to i64".into(),
                )
            })?))
        })
    }
}

impl<G> crate::solvers::BruteForceProblem for MinimumIntersectionGraphBasis<G>
where
    G: Graph + crate::variant::VariantParam,
{
    fn dimensions(&self) -> Vec<usize> {
        let n = self.graph.num_vertices();
        let m = self.graph.num_edges();
        if m == 0 {
            // No edges: no variables needed; empty assignment is trivially valid.
            return vec![];
        }
        vec![2; n * m]
    }
}

crate::impl_random_generate!(
    MinimumIntersectionGraphBasis<SimpleGraph>,
    crate::random::SimpleGraphRandomSpec,
    |spec| { Ok(MinimumIntersectionGraphBasis::new(spec.graph()?)) }
);

crate::declare_variants! {
    default MinimumIntersectionGraphBasis<SimpleGraph> => "num_edges^num_edges" random,
}

crate::register_brute_force! {
    MinimumIntersectionGraphBasis<SimpleGraph> decode |problem: &MinimumIntersectionGraphBasis<SimpleGraph>, indices: Vec<usize>| if problem.num_edges() == 0 { vec![Vec::new(); problem.num_vertices()] } else { indices.chunks(problem.num_edges()).map(crate::config::config_to_bits).collect() },
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // P3: 3 vertices, edges (0,1), (1,2), num_edges=2
    // Intersection number = 2: S[0]={0}, S[1]={0,1}, S[2]={1}
    // Incidence rows: vertex 0: [true,false], vertex 1: [true,true],
    // vertex 2: [false,true].
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_intersection_graph_basis_simplegraph",
        instance: Box::new(MinimumIntersectionGraphBasis::new(SimpleGraph::new(
            3,
            vec![(0, 1), (1, 2)],
        ))),
        optimal_config: serde_json::json!(vec![
            vec![true, false],
            vec![true, true],
            vec![false, true]
        ]),
        optimal_value: serde_json::json!(2),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/minimum_intersection_graph_basis.rs"]
mod tests;
