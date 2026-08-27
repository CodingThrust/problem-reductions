//! KClique problem implementation.
//!
//! KClique is the decision version of Clique: determine whether a graph
//! contains a clique of size at least `k`.

use crate::registry::{CreateSpec, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "KClique",
        display_name: "k-Clique",
        aliases: &["Clique"],
        dimensions: &[VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"])],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Determine whether a graph contains a clique of size at least k",
        fields: KCliqueCreateSpec::FIELDS,
    }
}

/// The k-Clique decision problem.
///
/// Given a graph `G = (V, E)` and a positive integer `k`, determine whether
/// there exists a subset `K ⊆ V` of size at least `k` such that every pair of
/// distinct vertices in `K` is adjacent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KClique<G> {
    graph: G,
    k: usize,
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct KCliqueCreateSpec {
    #[create(codec = "edge-list")]
    graph: Vec<(usize, usize)>,
    num_vertices: Option<usize>,
    k: usize,
}

impl TryFrom<KCliqueCreateSpec> for KClique<SimpleGraph> {
    type Error = crate::registry::ConstructionError;
    fn try_from(spec: KCliqueCreateSpec) -> Result<Self, Self::Error> {
        if spec.graph.is_empty() && spec.num_vertices.is_none() {
            return Err("num_vertices is required for an empty graph".into());
        }
        for &(u, v) in &spec.graph {
            if u == v {
                return Err(format!("self-loop {u}-{v} is not allowed").into());
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
            return Err("num_vertices is too small for graph endpoints".into());
        }
        if spec.k == 0 {
            return Err("k must be positive".into());
        }
        if spec.k > count {
            return Err("k must be <= graph num_vertices".into());
        }
        Ok(Self {
            graph: SimpleGraph::new(count, spec.graph),
            k: spec.k,
        })
    }
}

impl<G: Graph> KClique<G> {
    /// Create a new k-Clique problem instance.
    pub fn new(graph: G, k: usize) -> Self {
        assert!(k > 0, "k must be positive");
        assert!(k <= graph.num_vertices(), "k must be <= graph num_vertices");
        Self { graph, k }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get the clique-size threshold.
    pub fn k(&self) -> usize {
        self.k
    }

    /// Get the number of vertices in the underlying graph.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of edges in the underlying graph.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    /// Check whether a configuration is a valid witness.
    pub fn is_valid_solution(&self, config: &[bool]) -> bool {
        is_kclique_config(&self.graph, config, self.k)
    }

    /// Build a binary selection config from the listed vertices.
    pub fn config_from_vertices(num_vertices: usize, selected_vertices: &[usize]) -> Vec<bool> {
        let mut config = vec![false; num_vertices];
        for &vertex in selected_vertices {
            config[vertex] = true;
        }
        config
    }

    /// Convenience wrapper around [`Self::config_from_vertices`] using `self.num_vertices()`.
    pub fn config_from_selected_vertices(&self, selected_vertices: &[usize]) -> Vec<bool> {
        Self::config_from_vertices(self.num_vertices(), selected_vertices)
    }
}

impl<G> Problem for KClique<G>
where
    G: Graph + crate::variant::VariantParam,
{
    const NAME: &'static str = "KClique";
    type Solution = Vec<bool>;
    type Value = crate::types::Or;

    crate::problem_size![
        ("k", k),
        ("num_edges", num_edges),
        ("num_vertices", num_vertices),
    ];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G]
    }

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<crate::types::Or, crate::traits::EvaluationError> {
        if config.len() != self.graph.num_vertices() {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "vertex-selection length does not match the graph".into(),
            ));
        }
        Ok(crate::types::Or(is_kclique_config(
            &self.graph,
            config,
            self.k,
        )))
    }
}

impl<G> crate::solvers::BruteForceProblem for KClique<G>
where
    G: Graph + crate::variant::VariantParam,
{
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.graph.num_vertices()]
    }
}

fn is_kclique_config<G: Graph>(graph: &G, config: &[bool], k: usize) -> bool {
    if config.len() != graph.num_vertices() {
        return false;
    }

    let selected: Vec<usize> = config
        .iter()
        .enumerate()
        .filter_map(|(index, &selected)| selected.then_some(index))
        .collect();

    if selected.len() < k {
        return false;
    }

    for i in 0..selected.len() {
        for j in (i + 1)..selected.len() {
            if !graph.has_edge(selected[i], selected[j]) {
                return false;
            }
        }
    }
    true
}

crate::impl_random_generate!(
    KClique<SimpleGraph>,
    crate::random::CliqueRandomSpec,
    |spec| {
        if spec.k == 0 || spec.k > spec.num_vertices {
            return Err(format!(
                "k must be between 1 and num_vertices ({})",
                spec.num_vertices
            )
            .into());
        }
        Ok(KClique::new(spec.graph()?, spec.k))
    }
);

crate::declare_variants! {
    default KClique<SimpleGraph> => "1.1996^num_vertices" create KCliqueCreateSpec random,
}

crate::register_brute_force! {
    KClique<SimpleGraph> decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "kclique_simplegraph",
        instance: Box::new(KClique::new(
            SimpleGraph::new(5, vec![(0, 1), (0, 2), (1, 3), (2, 3), (2, 4), (3, 4)]),
            3,
        )),
        optimal_config: serde_json::json!(vec![false, false, true, true, true]),
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/kclique.rs"]
mod tests;
