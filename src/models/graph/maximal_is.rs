//! Maximal Independent Set problem implementation.
//!
//! The Maximal Independent Set problem asks for an independent set that
//! cannot be extended by adding any other vertex.

use crate::registry::{CreateSpec, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::{Max, WeightElement};
use num_traits::Zero;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MaximalIS",
        display_name: "Maximal IS",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
            VariantDimension::new("weight", "i64", &["i64"]),
        ],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Find maximum weight maximal independent set",
        fields: MaximalISCreateSpec::FIELDS,
    }
}

/// The Maximal Independent Set problem.
///
/// Given a graph G = (V, E), find an independent set S that is maximal,
/// meaning no vertex can be added to S while keeping it independent.
///
/// This is different from Maximum Independent Set - maximal means locally
/// optimal (cannot extend), while maximum means globally optimal (largest).
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::MaximalIS;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, BruteForce};
///
/// // Path graph 0-1-2
/// let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
/// let problem = MaximalIS::new(graph, vec![1; 3]);
///
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_witnesses(&problem).unwrap();
///
/// // Maximal independent sets: {0, 2} or {1}
/// for sol in &solutions {
///     assert!(problem.evaluate(sol).unwrap().is_valid());
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaximalIS<G, W> {
    /// The underlying graph.
    graph: G,
    /// Weights for each vertex.
    weights: Vec<W>,
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct MaximalISCreateSpec {
    /// The underlying graph G=(V,E).
    graph: SimpleGraph,
    /// Vertex weights w: V -> R.
    weights: Vec<i64>,
}

impl TryFrom<MaximalISCreateSpec> for MaximalIS<SimpleGraph, i64> {
    type Error = crate::registry::ConstructionError;
    fn try_from(spec: MaximalISCreateSpec) -> Result<Self, Self::Error> {
        if spec.weights.len() != spec.graph.num_vertices() {
            return Err(format!(
                "weights has {} entries, expected {}",
                spec.weights.len(),
                spec.graph.num_vertices()
            )
            .into());
        }
        Ok(Self::new(spec.graph, spec.weights))
    }
}

impl<G: Graph, W: Clone + Default> MaximalIS<G, W> {
    /// Create a Maximal Independent Set problem from a graph with given weights.
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

    /// Check if a configuration is a valid maximal independent set.
    pub fn is_valid_solution(&self, config: &[bool]) -> bool {
        self.is_maximal(config)
    }

    /// Check if a configuration is an independent set.
    fn is_independent(&self, config: &[bool]) -> bool {
        for (u, v) in self.graph.edges() {
            if config.get(u).copied().unwrap_or(false) && config.get(v).copied().unwrap_or(false) {
                return false;
            }
        }
        true
    }

    /// Check if an independent set is maximal (cannot be extended).
    fn is_maximal(&self, config: &[bool]) -> bool {
        if !self.is_independent(config) {
            return false;
        }

        let n = self.graph.num_vertices();
        for v in 0..n {
            if config.get(v).copied().unwrap_or(false) {
                continue; // Already in set
            }

            // Check if v can be added
            let neighbors = self.graph.neighbors(v);
            let can_add = neighbors
                .iter()
                .all(|&u| !config.get(u).copied().unwrap_or(false));

            if can_add {
                return false; // Set is not maximal
            }
        }

        true
    }
}

impl<G: Graph, W: WeightElement> MaximalIS<G, W> {
    /// Get the number of vertices in the underlying graph.
    pub fn num_vertices(&self) -> usize {
        self.graph().num_vertices()
    }

    /// Get the number of edges in the underlying graph.
    pub fn num_edges(&self) -> usize {
        self.graph().num_edges()
    }
}

impl<G, W> Problem for MaximalIS<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "MaximalIS";
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
                "vertex-selection length does not match the graph".into(),
            ));
        }
        Ok({
            if !self.is_maximal(config) {
                return Ok(Max(None));
            }
            let mut total = W::Sum::zero();
            for (i, &selected) in config.iter().enumerate() {
                if selected {
                    total = W::checked_add_to_sum(
                        total,
                        self.weights[i].to_sum(),
                        "summing selected maximal-independent-set weights",
                    )?;
                }
            }
            Max(Some(total))
        })
    }
}

impl<G, W> crate::solvers::BruteForceProblem for MaximalIS<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.graph.num_vertices()]
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "maximal_is_simplegraph",
        instance: Box::new(MaximalIS::new(
            SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]),
            vec![1i64; 5],
        )),
        optimal_config: serde_json::json!(vec![true, false, true, false, true]),
        optimal_value: serde_json::json!(3),
    }]
}

/// Check if a set is a maximal independent set.
///
/// # Panics
/// Panics if `selected.len() != graph.num_vertices()`.
#[cfg(test)]
pub(crate) fn is_maximal_independent_set<G: Graph>(graph: &G, selected: &[bool]) -> bool {
    assert_eq!(
        selected.len(),
        graph.num_vertices(),
        "selected length must match num_vertices"
    );

    // Check independence
    for (u, v) in graph.edges() {
        if selected[u] && selected[v] {
            return false;
        }
    }

    // Check maximality: no unselected vertex can be added
    for v in 0..graph.num_vertices() {
        if selected[v] {
            continue;
        }
        if graph.neighbors(v).iter().all(|&u| !selected[u]) {
            return false;
        }
    }

    true
}

crate::impl_random_generate!(MaximalIS<SimpleGraph, i64>, crate::random::SimpleGraphRandomSpec, |spec| {
    Ok(MaximalIS::new(spec.graph()?, vec![1; spec.num_vertices]))
});

crate::declare_variants! {
    default MaximalIS<SimpleGraph, i64> => "3^(num_vertices / 3)" create MaximalISCreateSpec random,
}

crate::register_brute_force! {
    MaximalIS<SimpleGraph, i64> decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/maximal_is.rs"]
mod tests;
