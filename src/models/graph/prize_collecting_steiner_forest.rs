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

use crate::registry::{ConstructionError, CreateSpec, ProblemSchemaEntry, VariantDimension};
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
            VariantDimension::new("weight", "i64", &["i64", "f64"]),
        ],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Find a forest minimizing omitted-prize plus edge-cost plus omega times the number of tree components",
        fields: PrizeCollectingSteinerForestI64CreateSpec::FIELDS,
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
/// * `W` - Weight / cost type (e.g., `i64`, `f64`).
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::PrizeCollectingSteinerForest;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::types::Min;
/// use problemreductions::{BruteForce, Problem};
///
/// // Path 0 - 1 - 2 with edge costs c(0,1)=1, c(1,2)=6 and vertex prizes
/// // p = (5, 2, 5), beta = 1, omega = 2.
/// let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
/// let problem =
///     PrizeCollectingSteinerForest::<_, i64>::new(graph, vec![5, 2, 5], vec![1, 6], 1, 2).unwrap();
/// // V_F = {0,1,2}, E_F = {(0,1)} gives two components {0,1} and {2}:
/// // objective = 0 + 1 + 2*2 = 5.
/// let solution = BruteForce::new().solve(&problem).unwrap().unwrap();
/// assert_eq!(problem.evaluate(&solution).unwrap(), Min(Some(5)));
/// ```
#[derive(Debug, Clone, Serialize)]
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

#[derive(Deserialize)]
struct PrizeCollectingSteinerForestData<G, W> {
    graph: G,
    vertex_prizes: Vec<W>,
    edge_costs: Vec<W>,
    beta: W,
    omega: W,
}

impl<'de, G, W> Deserialize<'de> for PrizeCollectingSteinerForest<G, W>
where
    G: Graph + Deserialize<'de>,
    W: WeightElement + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let data = PrizeCollectingSteinerForestData::deserialize(deserializer)?;
        Self::new(
            data.graph,
            data.vertex_prizes,
            data.edge_costs,
            data.beta,
            data.omega,
        )
        .map_err(serde::de::Error::custom)
    }
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
            type Error = ConstructionError;

            fn try_from(spec: $name) -> Result<Self, Self::Error> {
                let graph = simple_graph_from_create(spec.graph, spec.num_vertices)?;
                let vertex_prizes = spec
                    .vertex_prizes
                    .unwrap_or_else(|| vec![$one; graph.num_vertices()]);
                let edge_costs = spec
                    .edge_costs
                    .unwrap_or_else(|| vec![$one; graph.num_edges()]);
                Self::new(graph, vertex_prizes, edge_costs, spec.beta, spec.omega)
            }
        }
    };
}

prize_collecting_steiner_forest_create_spec!(PrizeCollectingSteinerForestI64CreateSpec, i64, 1);
prize_collecting_steiner_forest_create_spec!(PrizeCollectingSteinerForestF64CreateSpec, f64, 1.0);

fn simple_graph_from_create(
    edges: Vec<(usize, usize)>,
    num_vertices: Option<usize>,
) -> Result<SimpleGraph, ConstructionError> {
    if edges.is_empty() && num_vertices.is_none() {
        return Err(ConstructionError::Conversion(
            "num_vertices is required for an empty graph".into(),
        ));
    }
    for (index, &(u, v)) in edges.iter().enumerate() {
        if u == v {
            return Err(ConstructionError::Conversion(format!(
                "graph edge {index} is a self-loop at vertex {u}"
            )));
        }
    }
    let inferred = edges
        .iter()
        .flat_map(|&(u, v)| [u, v])
        .max()
        .map(|vertex| {
            vertex.checked_add(1).ok_or_else(|| {
                ConstructionError::IntegerOverflow(
                    "inferring the PrizeCollectingSteinerForest vertex count".into(),
                )
            })
        })
        .transpose()?
        .unwrap_or(0);
    let num_vertices = num_vertices.unwrap_or(inferred);
    if num_vertices < inferred {
        return Err(ConstructionError::Conversion(format!(
            "num_vertices {num_vertices} is too small for graph endpoints; need at least {inferred}"
        )));
    }
    Ok(SimpleGraph::new(num_vertices, edges))
}

impl<G: Graph, W: WeightElement> PrizeCollectingSteinerForest<G, W> {
    /// Create a new Prize-Collecting Steiner Forest instance.
    ///
    pub fn new(
        graph: G,
        vertex_prizes: Vec<W>,
        edge_costs: Vec<W>,
        beta: W,
        omega: W,
    ) -> Result<Self, ConstructionError> {
        if vertex_prizes.len() != graph.num_vertices() {
            return Err(ConstructionError::Conversion(
                "vertex_prizes length must match graph num_vertices".into(),
            ));
        }
        if edge_costs.len() != graph.num_edges() {
            return Err(ConstructionError::Conversion(
                "edge_costs length must match graph num_edges".into(),
            ));
        }
        for (index, prize) in vertex_prizes.iter().enumerate() {
            prize.validate_element(&format!("vertex prize at index {index}"))?;
        }
        for (index, cost) in edge_costs.iter().enumerate() {
            cost.validate_element(&format!("edge cost at index {index}"))?;
        }
        beta.validate_element("beta")?;
        omega.validate_element("omega")?;
        Ok(Self {
            graph,
            vertex_prizes,
            edge_costs,
            beta,
            omega,
        })
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
    pub fn is_valid_solution(&self, solution: &(Vec<bool>, Vec<bool>)) -> bool {
        forest_components(&self.graph, &solution.0, &solution.1).is_some()
    }
}

impl<G, W> Problem for PrizeCollectingSteinerForest<G, W>
where
    G: Graph + VariantParam,
    W: WeightElement + VariantParam,
{
    const NAME: &'static str = "PrizeCollectingSteinerForest";
    type Solution = (Vec<bool>, Vec<bool>);
    type Value = Min<W::Sum>;

    crate::problem_parameters![
        ("num_edges", num_edges),
        ("num_vertices", num_vertices),
        ("num_vertices_with_prize", num_vertices_with_prize),
    ];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G, W]
    }

    fn evaluate(
        &self,
        solution: &Self::Solution,
    ) -> Result<Min<W::Sum>, crate::traits::EvaluationError> {
        let (vertices, edges) = solution;
        if vertices.len() != self.graph.num_vertices() || edges.len() != self.graph.num_edges() {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "Steiner forest selection dimensions do not match the graph".into(),
            ));
        }
        Ok({
            let kappa = match forest_components(&self.graph, vertices, edges) {
                Some(kappa) => kappa,
                None => return Ok(Min(None)),
            };

            // Objective: beta * sum_{v notin V_F} p(v)
            //          + sum_{e in E_F} c(e)
            //          + omega * kappa(F).
            //
            let mut omitted_prizes = W::Sum::zero();
            for (v, prize) in self.vertex_prizes.iter().enumerate() {
                if !vertices[v] {
                    omitted_prizes = W::checked_add_to_sum(
                        omitted_prizes,
                        prize.to_sum(),
                        "summing omitted Steiner forest prizes",
                    )?;
                }
            }
            let omitted_term = W::checked_mul_sum(
                self.beta.to_sum(),
                omitted_prizes,
                "multiplying omitted prizes by beta",
            )?;

            let mut edge_term = W::Sum::zero();
            for (i, cost) in self.edge_costs.iter().enumerate() {
                if edges[i] {
                    edge_term = W::checked_add_to_sum(
                        edge_term,
                        cost.to_sum(),
                        "summing Steiner forest edge costs",
                    )?;
                }
            }

            // Represent `kappa` in `W::Sum` by summing `omega` `kappa` times.
            let omega_sum = self.omega.to_sum();
            let mut kappa_sum = W::Sum::zero();
            for _ in 0..kappa {
                kappa_sum = W::checked_add_to_sum(
                    kappa_sum,
                    omega_sum.clone(),
                    "multiplying Steiner forest component penalty",
                )?;
            }

            let mut total = W::Sum::zero();
            total = W::checked_add_to_sum(
                total,
                omitted_term,
                "summing Steiner forest objective terms",
            )?;
            total =
                W::checked_add_to_sum(total, edge_term, "summing Steiner forest objective terms")?;
            total =
                W::checked_add_to_sum(total, kappa_sum, "summing Steiner forest objective terms")?;
            Min(Some(total))
        })
    }
}

impl<G, W> crate::solvers::BruteForceProblem for PrizeCollectingSteinerForest<G, W>
where
    G: Graph + VariantParam,
    W: WeightElement + VariantParam,
{
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.graph.num_vertices() + self.graph.num_edges()]
    }
}

/// Validate a `(V_F, E_F)` configuration and, if feasible, return the number of
/// tree components `kappa(F)` among the selected vertices. Feasible means every
/// selected edge is incident only to selected vertices and the selected
/// subgraph is acyclic. Returns `None` for any infeasible configuration.
fn forest_components<G: Graph>(
    graph: &G,
    selected_vertices: &[bool],
    selected_edges: &[bool],
) -> Option<usize> {
    let n = graph.num_vertices();
    let m = graph.num_edges();
    if selected_vertices.len() != n || selected_edges.len() != m {
        return None;
    }
    let edges = graph.edges();
    let mut adj: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];
    for (i, &(u, v)) in edges.iter().enumerate() {
        if !selected_edges[i] {
            continue;
        }
        if !selected_vertices[u] || !selected_vertices[v] {
            return None;
        }
        adj[u].push((v, i));
        adj[v].push((u, i));
    }
    let mut visited = vec![false; n];
    let mut kappa: usize = 0;
    for start in 0..n {
        if !selected_vertices[start] || visited[start] {
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
    default PrizeCollectingSteinerForest<SimpleGraph, i64> => "2^(num_vertices + num_edges)" create PrizeCollectingSteinerForestI64CreateSpec,
    PrizeCollectingSteinerForest<SimpleGraph, f64> => "2^(num_vertices + num_edges)" create PrizeCollectingSteinerForestF64CreateSpec,
}

crate::register_brute_force! {
    PrizeCollectingSteinerForest<SimpleGraph, i64> decode |problem: &PrizeCollectingSteinerForest<SimpleGraph, i64>, indices: Vec<usize>| { let split = problem.num_vertices(); (crate::config::config_to_bits(&indices[..split]), crate::config::config_to_bits(&indices[split..])) },
    PrizeCollectingSteinerForest<SimpleGraph, f64> decode |problem: &PrizeCollectingSteinerForest<SimpleGraph, f64>, indices: Vec<usize>| { let split = problem.num_vertices(); (crate::config::config_to_bits(&indices[..split]), crate::config::config_to_bits(&indices[split..])) },
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // Issue #1026 canonical instance: path 0 - 1 - 2 with edge costs
    // c(0,1)=1, c(1,2)=6, vertex prizes p = (5, 2, 5), beta = 1, omega = 2.
    // Optimum: V_F = {0,1,2}, E_F = {(0,1)} (two components {0,1} and {2}),
    // objective = 0 + 1 + 2*2 = 5.
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "prize_collecting_steiner_forest_simplegraph",
        instance: Box::new(
            PrizeCollectingSteinerForest::<SimpleGraph, i64>::new(
                SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
                vec![5, 2, 5],
                vec![1, 6],
                1,
                2,
            )
            .unwrap(),
        ),
        optimal_config: serde_json::json!((vec![true, true, true], vec![true, false])),
        optimal_value: serde_json::json!(5),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/prize_collecting_steiner_forest.rs"]
mod tests;
