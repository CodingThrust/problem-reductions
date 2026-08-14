//! Prize-Collecting Steiner Forest problem implementation.
//!
//! Given an undirected network `G = (V, E)` with nonnegative vertex prizes
//! `p: V -> R_{>=0}`, nonnegative edge costs `c: E -> R_{>=0}`, and
//! nonnegative tradeoff parameters `beta` and `omega`, find a forest
//! `F = (V_F, E_F)` -- a subgraph that is a disjoint union of trees,
//! including singleton-vertex trees -- minimizing
//!
//! ```text
//! beta * sum_{v in V \ V_F} p(v) + sum_{e in E_F} c(e) + omega * kappa(F),
//! ```
//!
//! where `kappa(F)` is the number of (tree) components of `F`. Singleton
//! selected vertices are allowed and count as one-vertex tree components;
//! unselected vertices are not part of any component.
//!
//! Reference:
//! - Nurcan Tuncbag, Alfredo Braunstein, Andrea Pagnani, Shao-Shan Carol
//!   Huang, Jennifer Chayes, Christian Borgs, Riccardo Zecchina, and Ernest
//!   Fraenkel. "Simultaneous Reconstruction of Multiple Signaling Pathways
//!   via the Prize-Collecting Steiner Forest Problem." Journal of
//!   Computational Biology 20(2):124--136, 2013.
//!   <https://doi.org/10.1089/cmb.2012.0092>
//! - Earlier conference version, RECOMB 2012, LNBI 7262, pp. 287--301.
//!   <https://doi.org/10.1007/978-3-642-29627-7_31>

use crate::registry::{CreateSpec, ProblemSchemaEntry, ProblemSizeFieldEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::{Min, WeightElement};
use crate::variant::VariantParam;
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

inventory::submit! {
    ProblemSchemaEntry {
        name: "PrizeCollectingSteinerForest",
        display_name: "Prize-Collecting Steiner Forest",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
            VariantDimension::new("weight", "i32", &["i32", "f64"]),
        ],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Find a forest minimizing omitted-prize plus edge-cost plus omega times the number of tree components",
        fields: PrizeCollectingSteinerForestI32CreateSpec::FIELDS,
    }
}

inventory::submit! {
    ProblemSizeFieldEntry {
        name: "PrizeCollectingSteinerForest",
        fields: &["num_vertices", "num_edges", "num_vertices_with_prize"],
    }
}

/// The Prize-Collecting Steiner Forest problem (biology-paper variant).
///
/// Configuration layout (length `num_vertices + num_edges`):
/// - the first `num_vertices` bits are vertex selectors `x_v` (1 iff
///   `v in V_F`),
/// - the next `num_edges` bits are edge selectors `y_e` (1 iff `e in E_F`),
///   in `graph.edges()` order.
///
/// A configuration is feasible iff every selected edge has both endpoints
/// selected and the resulting subgraph is acyclic. Singleton selected
/// vertices are allowed.
///
/// # Type Parameters
///
/// * `G` - Graph type (currently `SimpleGraph`).
/// * `W` - Weight / cost type (e.g., `i32`, `f64`).
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::PrizeCollectingSteinerForest;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::types::Min;
/// use problemreductions::{BruteForce, Problem, Solver};
///
/// // Path 0 - 1 - 2 with edge costs c(0,1)=1, c(1,2)=6 and vertex prizes
/// // p = (5, 2, 5), beta = 1, omega = 2.
/// let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
/// let problem =
///     PrizeCollectingSteinerForest::<_, i32>::new(graph, vec![5, 2, 5], vec![1, 6], 1, 2);
/// // V_F = {0,1,2}, E_F = {(0,1)} gives two components {0,1} and {2}:
/// // objective = 0 + 1 + 2*2 = 5.
/// assert_eq!(BruteForce::new().solve(&problem), Min(Some(5)));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(deserialize = "G: serde::Deserialize<'de>, W: serde::Deserialize<'de>"))]
pub struct PrizeCollectingSteinerForest<G, W> {
    /// The underlying network.
    graph: G,
    /// Vertex prizes `p: V -> R_{>=0}` (in vertex-index order).
    vertex_prizes: Vec<W>,
    /// Edge costs `c: E -> R_{>=0}` (in `graph.edges()` order).
    edge_costs: Vec<W>,
    /// Tradeoff coefficient on the omitted-prize term.
    beta: W,
    /// Per-component penalty.
    omega: W,
}

macro_rules! prize_collecting_steiner_forest_create_spec {
    ($name:ident, $weight:ty, $one:expr) => {
        #[derive(Debug, Deserialize, crate::CreateSpec)]
        struct $name {
            #[create(codec = "edge-list")]
            graph: Vec<(usize, usize)>,
            num_vertices: Option<usize>,
            #[create(codec = "comma-separated")]
            vertex_prizes: Option<Vec<$weight>>,
            #[create(codec = "comma-separated")]
            edge_costs: Option<Vec<$weight>>,
            beta: $weight,
            omega: $weight,
        }

        impl TryFrom<$name> for PrizeCollectingSteinerForest<SimpleGraph, $weight> {
            type Error = String;

            fn try_from(spec: $name) -> Result<Self, Self::Error> {
                let graph = simple_graph_from_create(spec.graph, spec.num_vertices)?;
                let vertex_prizes = spec
                    .vertex_prizes
                    .unwrap_or_else(|| vec![$one; graph.num_vertices()]);
                if vertex_prizes.len() != graph.num_vertices() {
                    return Err(format!(
                        "vertex_prizes has length {}, expected {}",
                        vertex_prizes.len(),
                        graph.num_vertices()
                    ));
                }
                let edge_costs = spec
                    .edge_costs
                    .unwrap_or_else(|| vec![$one; graph.num_edges()]);
                if edge_costs.len() != graph.num_edges() {
                    return Err(format!(
                        "edge_costs has length {}, expected {}",
                        edge_costs.len(),
                        graph.num_edges()
                    ));
                }
                Ok(Self::new(
                    graph,
                    vertex_prizes,
                    edge_costs,
                    spec.beta,
                    spec.omega,
                ))
            }
        }
    };
}

prize_collecting_steiner_forest_create_spec!(PrizeCollectingSteinerForestI32CreateSpec, i32, 1);
prize_collecting_steiner_forest_create_spec!(PrizeCollectingSteinerForestF64CreateSpec, f64, 1.0);

fn simple_graph_from_create(
    edges: Vec<(usize, usize)>,
    num_vertices: Option<usize>,
) -> Result<SimpleGraph, String> {
    if edges.is_empty() && num_vertices.is_none() {
        return Err("num_vertices is required for an empty graph".to_string());
    }
    for (index, &(u, v)) in edges.iter().enumerate() {
        if u == v {
            return Err(format!("graph edge {index} is a self-loop at vertex {u}"));
        }
    }
    let inferred = edges
        .iter()
        .flat_map(|&(u, v)| [u, v])
        .max()
        .map(|vertex| vertex.checked_add(1).ok_or("vertex count overflows usize"))
        .transpose()?
        .unwrap_or(0);
    let num_vertices = num_vertices.unwrap_or(inferred);
    if num_vertices < inferred {
        return Err(format!(
            "num_vertices {num_vertices} is too small for graph endpoints; need at least {inferred}"
        ));
    }
    Ok(SimpleGraph::new(num_vertices, edges))
}

impl<G: Graph, W: Clone + Default> PrizeCollectingSteinerForest<G, W> {
    /// Create a new Prize-Collecting Steiner Forest instance.
    ///
    /// # Panics
    /// Panics if `vertex_prizes.len() != graph.num_vertices()` or
    /// `edge_costs.len() != graph.num_edges()`.
    pub fn new(graph: G, vertex_prizes: Vec<W>, edge_costs: Vec<W>, beta: W, omega: W) -> Self {
        assert_eq!(
            vertex_prizes.len(),
            graph.num_vertices(),
            "vertex_prizes length must match graph num_vertices"
        );
        assert_eq!(
            edge_costs.len(),
            graph.num_edges(),
            "edge_costs length must match graph num_edges"
        );
        Self {
            graph,
            vertex_prizes,
            edge_costs,
            beta,
            omega,
        }
    }

    /// Reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Vertex prizes in vertex-index order.
    pub fn vertex_prizes(&self) -> &[W] {
        &self.vertex_prizes
    }

    /// Edge costs in `graph.edges()` order.
    pub fn edge_costs(&self) -> &[W] {
        &self.edge_costs
    }

    /// Tradeoff coefficient on the omitted-prize term.
    pub fn beta(&self) -> &W {
        &self.beta
    }

    /// Per-component penalty.
    pub fn omega(&self) -> &W {
        &self.omega
    }
}

impl<G: Graph, W: WeightElement> PrizeCollectingSteinerForest<G, W> {
    /// Number of vertices in the underlying graph.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Number of edges in the underlying graph.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    /// Number of vertices with a strictly positive prize, i.e.
    /// `|{ v in V : p(v) > 0 }|`.
    pub fn num_vertices_with_prize(&self) -> usize {
        let zero = <W::Sum as Zero>::zero();
        self.vertex_prizes
            .iter()
            .filter(|prize| prize.to_sum() > zero)
            .count()
    }

    /// Whether this configuration is a feasible forest (selected edges only
    /// touch selected vertices and induce an acyclic subgraph).
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        forest_components(&self.graph, config).is_some()
    }
}

impl<G, W> Problem for PrizeCollectingSteinerForest<G, W>
where
    G: Graph + VariantParam,
    W: WeightElement + VariantParam,
{
    const NAME: &'static str = "PrizeCollectingSteinerForest";
    type Value = Min<W::Sum>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G, W]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_vertices() + self.graph.num_edges()]
    }

    fn evaluate(&self, config: &[usize]) -> Min<W::Sum> {
        let n = self.graph.num_vertices();
        let kappa = match forest_components(&self.graph, config) {
            Some(kappa) => kappa,
            None => return Min(None),
        };

        // Objective: beta * sum_{v notin V_F} p(v)
        //          + sum_{e in E_F} c(e)
        //          + omega * kappa(F).
        //
        // `W::Sum: Num` (via `NumericSize`) gives us `Mul`, so we form the
        // products `beta * (omitted prize sum)` and `omega * kappa` directly.
        let mut omitted_prizes = W::Sum::zero();
        for (v, prize) in self.vertex_prizes.iter().enumerate() {
            if config[v] == 0 {
                omitted_prizes += prize.to_sum();
            }
        }
        let omitted_term = self.beta.to_sum() * omitted_prizes;

        let mut edge_term = W::Sum::zero();
        for (i, cost) in self.edge_costs.iter().enumerate() {
            if config[n + i] == 1 {
                edge_term += cost.to_sum();
            }
        }

        // Represent `kappa` in `W::Sum` by summing `omega` `kappa` times.
        // `NumericSize` does not require a `From<usize>` conversion, so we
        // accumulate additively rather than casting.
        let omega_sum = self.omega.to_sum();
        let mut kappa_sum = W::Sum::zero();
        for _ in 0..kappa {
            kappa_sum += omega_sum.clone();
        }

        let mut total = W::Sum::zero();
        total += omitted_term;
        total += edge_term;
        total += kappa_sum;
        Min(Some(total))
    }
}

/// Validate a `(V_F, E_F)` configuration and, if feasible, return the number of
/// tree components `kappa(F)` among the selected vertices. Feasible means every
/// selected edge is incident only to selected vertices and the selected
/// subgraph is acyclic. Returns `None` for any infeasible configuration.
fn forest_components<G: Graph>(graph: &G, config: &[usize]) -> Option<usize> {
    let n = graph.num_vertices();
    let m = graph.num_edges();
    if config.len() != n + m {
        return None;
    }
    let edges = graph.edges();
    let mut adj: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];
    for (i, &(u, v)) in edges.iter().enumerate() {
        let y_e = config[n + i];
        if y_e == 0 {
            continue;
        }
        if y_e != 1 {
            return None;
        }
        if config[u] != 1 || config[v] != 1 {
            return None;
        }
        adj[u].push((v, i));
        adj[v].push((u, i));
    }
    let mut visited = vec![false; n];
    let mut kappa: usize = 0;
    for start in 0..n {
        if config[start] != 1 || visited[start] {
            continue;
        }
        kappa += 1;
        visited[start] = true;
        let mut parent_edge: Vec<Option<usize>> = vec![None; n];
        let mut queue: VecDeque<usize> = VecDeque::new();
        queue.push_back(start);
        while let Some(u) = queue.pop_front() {
            for &(w, edge_idx) in &adj[u] {
                if parent_edge[u] == Some(edge_idx) {
                    continue;
                }
                if visited[w] {
                    return None; // back-edge inside the component => cycle
                }
                visited[w] = true;
                parent_edge[w] = Some(edge_idx);
                queue.push_back(w);
            }
        }
    }
    Some(kappa)
}

crate::declare_variants! {
    default PrizeCollectingSteinerForest<SimpleGraph, i32> => "2^(num_vertices + num_edges)" create PrizeCollectingSteinerForestI32CreateSpec,
    PrizeCollectingSteinerForest<SimpleGraph, f64> => "2^(num_vertices + num_edges)" create PrizeCollectingSteinerForestF64CreateSpec,
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // Issue #1026 canonical instance: path 0 - 1 - 2 with edge costs
    // c(0,1)=1, c(1,2)=6, vertex prizes p = (5, 2, 5), beta = 1, omega = 2.
    // Optimum: V_F = {0,1,2}, E_F = {(0,1)} (two components {0,1} and {2}),
    // objective = 0 + 1 + 2*2 = 5.
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "prize_collecting_steiner_forest_simplegraph_i32",
        instance: Box::new(PrizeCollectingSteinerForest::<SimpleGraph, i32>::new(
            SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
            vec![5, 2, 5],
            vec![1, 6],
            1,
            2,
        )),
        // 3 vertex bits + 2 edge bits = 5-bit configuration.
        optimal_config: vec![1, 1, 1, 1, 0],
        optimal_value: serde_json::json!(5),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/prize_collecting_steiner_forest.rs"]
mod tests;
