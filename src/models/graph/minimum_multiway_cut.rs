//! Minimum Multiway Cut problem implementation.
//!
//! The Minimum Multiway Cut problem asks for a minimum weight set of edges
//! whose removal disconnects all terminal pairs.

use crate::registry::{CreateSpec, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::{Min, WeightElement};
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumMultiwayCut",
        display_name: "Minimum Multiway Cut",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
            VariantDimension::new("weight", "i64", &["i64"]),
        ],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Find minimum weight set of edges whose removal disconnects all terminal pairs",
        fields: MinimumMultiwayCutCreateSpec::FIELDS,
    }
}

/// The Minimum Multiway Cut problem.
///
/// Given an undirected weighted graph G = (V, E, w) and a set of k terminal
/// vertices T = {t_1, ..., t_k}, find a minimum-weight set of edges C ⊆ E
/// such that no two terminals remain in the same connected component of
/// G' = (V, E \ C).
///
/// # Representation
///
/// Each edge is assigned a binary variable:
/// - 0: edge is kept
/// - 1: edge is removed (in the cut)
///
/// A configuration is feasible if removing the cut edges disconnects all
/// terminal pairs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumMultiwayCut<G, W> {
    graph: G,
    terminals: Vec<usize>,
    edge_weights: Vec<W>,
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct MinimumMultiwayCutCreateSpec {
    /// The undirected graph G=(V,E).
    graph: SimpleGraph,
    /// Terminal vertices that must be separated.
    terminals: Vec<usize>,
    /// Edge weights w: E -> R in graph edge order.
    edge_weights: Vec<i64>,
}

impl TryFrom<MinimumMultiwayCutCreateSpec> for MinimumMultiwayCut<SimpleGraph, i64> {
    type Error = crate::registry::ConstructionError;
    fn try_from(spec: MinimumMultiwayCutCreateSpec) -> Result<Self, Self::Error> {
        if spec.edge_weights.len() != spec.graph.num_edges() {
            return Err(format!(
                "edge_weights has {} entries, expected {}",
                spec.edge_weights.len(),
                spec.graph.num_edges()
            )
            .into());
        }
        if spec.terminals.len() < 2 {
            return Err("at least two terminals are required".to_string().into());
        }
        let mut distinct = spec.terminals.clone();
        distinct.sort_unstable();
        distinct.dedup();
        if distinct.len() != spec.terminals.len() {
            return Err("terminals must be distinct".to_string().into());
        }
        if let Some(&terminal) = spec
            .terminals
            .iter()
            .find(|&&t| t >= spec.graph.num_vertices())
        {
            return Err(format!(
                "terminal {terminal} is outside graph with {} vertices",
                spec.graph.num_vertices()
            )
            .into());
        }
        Ok(Self::new(spec.graph, spec.terminals, spec.edge_weights))
    }
}

impl<G: Graph, W: Clone + Default> MinimumMultiwayCut<G, W> {
    /// Create a MinimumMultiwayCut problem.
    ///
    /// `edge_weights` must have one entry per edge, in the same order as
    /// [`Graph::edges()`](crate::topology::Graph::edges). Each binary
    /// variable corresponds to an edge: 0 = keep, 1 = cut.
    ///
    /// # Panics
    /// - If `edge_weights.len() != graph.num_edges()`
    /// - If `terminals.len() < 2`
    /// - If any terminal index is out of bounds
    /// - If there are duplicate terminal indices
    pub fn new(graph: G, terminals: Vec<usize>, edge_weights: Vec<W>) -> Self {
        assert_eq!(
            edge_weights.len(),
            graph.num_edges(),
            "edge_weights length must match num_edges"
        );
        assert!(terminals.len() >= 2, "need at least 2 terminals");
        let mut sorted = terminals.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted.len(), terminals.len(), "duplicate terminal indices");
        for &t in &terminals {
            assert!(t < graph.num_vertices(), "terminal index out of bounds");
        }
        Self {
            graph,
            terminals,
            edge_weights,
        }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get the terminal vertices.
    pub fn terminals(&self) -> &[usize] {
        &self.terminals
    }

    /// Get the edge weights.
    pub fn edge_weights(&self) -> &[W] {
        &self.edge_weights
    }
}

impl<G: Graph, W: WeightElement> MinimumMultiwayCut<G, W> {
    /// Number of vertices in the graph.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Number of edges in the graph.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    /// Number of terminal vertices.
    pub fn num_terminals(&self) -> usize {
        self.terminals.len()
    }
}

/// Check if all terminals are in distinct connected components
/// when edges marked as cut (config[e]) are removed.
fn terminals_separated<G: Graph>(graph: &G, terminals: &[usize], config: &[bool]) -> bool {
    let n = graph.num_vertices();
    let edges = graph.edges();

    // Build adjacency list from non-cut edges
    let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
    for (idx, (u, v)) in edges.iter().enumerate() {
        if !config.get(idx).copied().unwrap_or(false) {
            adj[*u].push(*v);
            adj[*v].push(*u);
        }
    }

    // BFS from each terminal; if a terminal is already visited by a previous
    // terminal's BFS, they share a component => infeasible.
    let mut component = vec![usize::MAX; n];
    for (comp_id, &t) in terminals.iter().enumerate() {
        if component[t] != usize::MAX {
            return false;
        }
        let mut queue = VecDeque::new();
        queue.push_back(t);
        component[t] = comp_id;
        while let Some(u) = queue.pop_front() {
            for &v in &adj[u] {
                if component[v] == usize::MAX {
                    component[v] = comp_id;
                    queue.push_back(v);
                }
            }
        }
    }
    true
}

impl<G, W> Problem for MinimumMultiwayCut<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "MinimumMultiwayCut";
    type Solution = Vec<bool>;
    type Value = Min<W::Sum>;

    crate::problem_parameters![
        ("num_edges", num_edges),
        ("num_terminals", num_terminals),
        ("num_vertices", num_vertices),
    ];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G, W]
    }

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Min<W::Sum>, crate::traits::EvaluationError> {
        if config.len() != self.graph.num_edges() {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "edge-selection length does not match the graph".into(),
            ));
        }
        Ok({
            if !terminals_separated(&self.graph, &self.terminals, config) {
                return Ok(Min(None));
            }
            let mut total = W::Sum::zero();
            for (idx, &selected) in config.iter().enumerate() {
                if selected {
                    if let Some(w) = self.edge_weights.get(idx) {
                        total = W::checked_add_to_sum(
                            total,
                            w.to_sum(),
                            "summing multiway cut edge weights",
                        )?;
                    }
                }
            }
            Min(Some(total))
        })
    }
}

impl<G, W> crate::solvers::BruteForceProblem for MinimumMultiwayCut<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.graph.num_edges()]
    }
}

crate::declare_variants! {
    default MinimumMultiwayCut<SimpleGraph, i64> => "1.84^num_terminals * num_vertices^3" create MinimumMultiwayCutCreateSpec,
}

crate::register_brute_force! {
    MinimumMultiwayCut<SimpleGraph, i64> decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_multiway_cut_simplegraph",
        instance: Box::new(MinimumMultiwayCut::new(
            SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (0, 4), (1, 3)]),
            vec![0, 2, 4],
            vec![2, 3, 1, 2, 4, 5],
        )),
        optimal_config: serde_json::json!(vec![true, false, false, true, true, false]),
        optimal_value: serde_json::json!(8),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/minimum_multiway_cut.rs"]
mod tests;
