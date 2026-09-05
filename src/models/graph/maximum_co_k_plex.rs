//! Maximum Co-k-Plex problem implementation.
//!
//! Given an undirected graph G = (V, E), vertex weights w: V -> R, and an
//! integer k >= 1, find a subset S ⊆ V maximizing Σ_{v ∈ S} w(v) such that
//! the induced subgraph G[S] has maximum degree at most k - 1. Equivalently,
//! every selected vertex has at most k - 1 selected neighbours.
//!
//! For k = 1 the problem degenerates to [`MaximumIndependentSet`]; for larger
//! k it is the maximum (k-1)-dependent set / co-k-plex.

use crate::registry::{CreateSpec, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::{Max, One, WeightElement};
use crate::variant::{KValue, VariantParam, KN};
use num_traits::Zero;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MaximumCoKPlex",
        display_name: "Maximum Co-k-Plex",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
            VariantDimension::new("weight", "One", &["One", "i64"]),
            VariantDimension::new("k", "KN", &["KN"]),
        ],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Find maximum-weight vertex subset whose induced subgraph has maximum degree at most k-1",
        fields: MaximumCoKPlexCreateSpec::<One>::FIELDS,
    }
}

/// The Maximum Co-k-Plex problem.
///
/// Given a graph `G = (V, E)`, vertex weights `w_v`, and an integer
/// `k >= 1`, find `S ⊆ V` maximizing `Σ_{v ∈ S} w_v` subject to
/// `deg_{G[S]}(v) <= k - 1` for every `v ∈ S` (equivalently, the induced
/// subgraph has maximum degree at most `k - 1`).
///
/// # Type Parameters
///
/// * `G` - Graph type (e.g., [`SimpleGraph`]).
/// * `W` - Weight type (e.g., [`One`], `i64`).
/// * `K` - Compile-time [`KValue`] tag. [`KN`] stores `k` at runtime; fixed
///   variants (`K1`, `K2`, ...) can be added later by registering more
///   `declare_variants!` entries.
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::MaximumCoKPlex;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::types::One;
/// use problemreductions::variant::KN;
/// use problemreductions::{BruteForce, Problem};
///
/// // 5-cycle C_5 with k = 2 (induced degree <= 1).
/// let graph = SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)]);
/// let problem =
///     MaximumCoKPlex::<_, One, KN>::with_k(graph, vec![One; 5], 2);
/// assert_eq!(problem.bound_k(), 2);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(deserialize = "G: serde::Deserialize<'de>, W: serde::Deserialize<'de>"))]
pub struct MaximumCoKPlex<G, W, K: KValue> {
    /// The underlying graph.
    graph: G,
    /// Per-vertex weights `w_v`.
    weights: Vec<W>,
    /// Runtime co-k-plex parameter `k`. For compile-time `K` it equals `K::K`.
    ///
    /// Intentionally has no serde default: a malformed JSON missing
    /// `bound_k` (e.g. for the `KN` variant) must fail loudly at load time
    /// rather than silently fall back to `0`, which would make every
    /// `evaluate()` infeasible.
    bound_k: usize,
    #[serde(skip)]
    _phantom: std::marker::PhantomData<K>,
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct MaximumCoKPlexCreateSpec<W> {
    /// The underlying graph G=(V,E).
    graph: SimpleGraph,
    /// Vertex weights w: V -> R.
    weights: Vec<W>,
    /// Co-k-plex parameter k >= 1.
    k: usize,
}

impl<W: Clone + Default> TryFrom<MaximumCoKPlexCreateSpec<W>>
    for MaximumCoKPlex<SimpleGraph, W, KN>
{
    type Error = crate::registry::ConstructionError;

    fn try_from(spec: MaximumCoKPlexCreateSpec<W>) -> Result<Self, Self::Error> {
        if spec.weights.len() != spec.graph.num_vertices() {
            return Err(format!(
                "weights has {} entries, expected {}",
                spec.weights.len(),
                spec.graph.num_vertices()
            )
            .into());
        }
        if spec.k == 0 {
            return Err("k must be at least 1".to_string().into());
        }
        Ok(Self::with_k(spec.graph, spec.weights, spec.k))
    }
}

impl<G: Graph, W: Clone + Default, K: KValue> MaximumCoKPlex<G, W, K> {
    /// Create an instance with an explicit runtime `k`.
    ///
    /// # Panics
    /// Panics if `weights.len()` does not match `graph.num_vertices()`, if
    /// `bound_k == 0`, or if `K` declares a fixed value that disagrees with
    /// `bound_k`.
    pub fn with_k(graph: G, weights: Vec<W>, bound_k: usize) -> Self {
        assert_eq!(
            weights.len(),
            graph.num_vertices(),
            "weights length must match graph num_vertices"
        );
        assert!(bound_k >= 1, "co-k-plex parameter k must be at least 1");
        if let Some(fixed) = K::K {
            assert_eq!(
                fixed, bound_k,
                "fixed K type disagrees with runtime bound_k"
            );
        }
        Self {
            graph,
            weights,
            bound_k,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Create a new instance using the compile-time `K`.
    ///
    /// # Panics
    /// Panics if `K` is [`KN`] (use [`MaximumCoKPlex::with_k`] instead) or if
    /// `weights.len()` does not match `graph.num_vertices()`.
    pub fn new(graph: G, weights: Vec<W>) -> Self {
        let k = K::K.expect("KN requires with_k");
        Self::with_k(graph, weights, k)
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get a reference to the vertex weights.
    pub fn weights(&self) -> &[W] {
        &self.weights
    }

    /// Co-k-plex parameter `k`.
    pub fn bound_k(&self) -> usize {
        self.bound_k
    }

    /// Check if the problem uses a non-unit weight type.
    pub fn is_weighted(&self) -> bool
    where
        W: WeightElement,
    {
        !W::IS_UNIT
    }

    /// Check if a configuration satisfies the co-k-plex constraint.
    pub fn is_valid_solution(&self, config: &[bool]) -> bool {
        is_co_k_plex_config(&self.graph, config, self.bound_k)
    }
}

impl<G: Graph, W: WeightElement, K: KValue> MaximumCoKPlex<G, W, K> {
    /// Number of vertices in the underlying graph.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Number of edges in the underlying graph.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }
}

impl<G, W, K> Problem for MaximumCoKPlex<G, W, K>
where
    G: Graph + VariantParam,
    W: WeightElement + VariantParam,
    K: KValue,
{
    const NAME: &'static str = "MaximumCoKPlex";
    type Solution = Vec<bool>;
    type Value = Max<W::Sum>;

    crate::problem_parameters![("num_edges", num_edges), ("num_vertices", num_vertices),];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G, W, K]
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
            if !is_co_k_plex_config(&self.graph, config, self.bound_k) {
                return Ok(Max(None));
            }
            let mut total = W::Sum::zero();
            for (i, &selected) in config.iter().enumerate() {
                if selected {
                    total = W::checked_add_to_sum(
                        total,
                        self.weights[i].to_sum(),
                        "summing selected co-k-plex weights",
                    )?;
                }
            }
            Max(Some(total))
        })
    }
}

impl<G, W, K> crate::solvers::BruteForceProblem for MaximumCoKPlex<G, W, K>
where
    G: Graph + VariantParam,
    W: WeightElement + VariantParam,
    K: KValue,
{
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.graph.num_vertices()]
    }
}

/// Return true iff every selected vertex has at most `k - 1` selected
/// neighbours in the induced subgraph.
fn is_co_k_plex_config<G: Graph>(graph: &G, config: &[bool], bound_k: usize) -> bool {
    if bound_k == 0 {
        return false;
    }
    let n = graph.num_vertices();
    let mut induced_degree = vec![0usize; n];
    for (u, v) in graph.edges() {
        let u_selected = config.get(u).copied().unwrap_or(false);
        let v_selected = config.get(v).copied().unwrap_or(false);
        if u_selected && v_selected {
            induced_degree[u] += 1;
            induced_degree[v] += 1;
            if induced_degree[u] > bound_k - 1 || induced_degree[v] > bound_k - 1 {
                return false;
            }
        }
    }
    true
}

crate::declare_variants! {
    default MaximumCoKPlex<SimpleGraph, One, KN> => "2^num_vertices" create MaximumCoKPlexCreateSpec<One>,
    MaximumCoKPlex<SimpleGraph, i64, KN>          => "2^num_vertices" create MaximumCoKPlexCreateSpec<i64>,
}

crate::register_brute_force! {
    MaximumCoKPlex<SimpleGraph, One, KN> decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
    MaximumCoKPlex<SimpleGraph, i64, KN> decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "maximum_co_k_plex_simplegraph",
        instance: Box::new(MaximumCoKPlex::<_, i64, KN>::with_k(
            SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)]),
            vec![5, 1, 4, 1, 3],
            2,
        )),
        optimal_config: serde_json::json!([true, false, true, false, true]),
        optimal_value: serde_json::json!(12),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/maximum_co_k_plex.rs"]
mod tests;
