//! Mixed Chinese Postman problem implementation.
//!
//! Given a mixed graph with directed arcs and undirected edges, find a
//! minimum-cost closed walk that traverses every directed arc in its prescribed
//! direction and every undirected edge in at least one direction.

use crate::registry::{CreateSpec, ProblemSchemaEntry, VariantDimension};
use crate::topology::{DirectedGraph, MixedGraph};
use crate::traits::Problem;
use crate::types::{Min, One, WeightElement};
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

const INF_COST: i64 = i64::MAX / 4;

inventory::submit! {
    ProblemSchemaEntry {
        name: "MixedChinesePostman",
        display_name: "Mixed Chinese Postman",
        aliases: &["MCPP"],
        dimensions: &[
            VariantDimension::new("weight", "i64", &["i64", "One"]),
        ],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Find a minimum-cost closed walk covering all arcs and edges in a mixed graph",
        fields: MixedChinesePostmanI64CreateSpec::FIELDS,
    }
}

/// Mixed Chinese Postman.
///
/// Each configuration picks a required traversal direction for every undirected
/// edge. The minimum-cost closed walk is then computed via the directed Chinese
/// Postman subproblem, using all available arcs (including both directions of
/// every undirected edge) for degree-balancing detours.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MixedChinesePostman<W: WeightElement<Sum = i64>> {
    graph: MixedGraph,
    arc_weights: Vec<W>,
    edge_weights: Vec<W>,
}

macro_rules! mixed_chinese_postman_create_spec {
    ($name:ident, $weight:ty, $one:expr) => {
        #[derive(Debug, Deserialize, crate::CreateSpec)]
        struct $name {
            /// Undirected graph edges.
            #[create(codec = "edge-list")]
            graph: Vec<(usize, usize)>,
            /// Directed graph arcs.
            #[create(codec = "arc-list")]
            arcs: Vec<(usize, usize)>,
            /// Vertex count, needed to preserve isolated vertices.
            num_vertices: Option<usize>,
            /// Directed-arc lengths; defaults to one per arc.
            #[create(codec = "comma-separated")]
            arc_weights: Option<Vec<$weight>>,
            /// Undirected-edge lengths; defaults to one per edge.
            #[create(codec = "comma-separated")]
            edge_weights: Option<Vec<$weight>>,
        }

        impl TryFrom<$name> for MixedChinesePostman<$weight> {
            type Error = crate::registry::ConstructionError;

            fn try_from(spec: $name) -> Result<Self, Self::Error> {
                if spec.graph.is_empty() && spec.num_vertices.is_none() {
                    return Err("num_vertices is required for an empty graph".to_string().into());
                }
                if spec.arcs.is_empty() {
                    return Err("arcs must be non-empty".to_string().into());
                }
                for (index, &(u, v)) in spec.graph.iter().enumerate() {
                    if u == v {
                        return Err(format!("graph edge {index} is a self-loop at vertex {u}").into());
                    }
                }
                let inferred = spec
                    .graph
                    .iter()
                    .flat_map(|&(u, v)| [u, v])
                    .max()
                    .map(|vertex| vertex.checked_add(1).ok_or("vertex count overflows usize"))
                    .transpose()?
                    .unwrap_or(0);
                let num_vertices = spec.num_vertices.unwrap_or(inferred);
                if num_vertices < inferred {
                    return Err(format!(
                        "num_vertices {num_vertices} is too small for graph endpoints; need at least {inferred}"
                    ).into());
                }
                for (index, &(u, v)) in spec.arcs.iter().enumerate() {
                    if u >= num_vertices || v >= num_vertices {
                        return Err(format!(
                            "arc {index} endpoint is out of range for {num_vertices} vertices"
                        ).into());
                    }
                }
                let arc_weights = spec
                    .arc_weights
                    .unwrap_or_else(|| vec![$one; spec.arcs.len()]);
                let edge_weights = spec
                    .edge_weights
                    .unwrap_or_else(|| vec![$one; spec.graph.len()]);
                MixedChinesePostman::try_new(
                    MixedGraph::new(num_vertices, spec.arcs, spec.graph),
                    arc_weights,
                    edge_weights,
                )
            }
        }
    };
}

mixed_chinese_postman_create_spec!(MixedChinesePostmanI64CreateSpec, i64, 1_i64);
mixed_chinese_postman_create_spec!(MixedChinesePostmanOneCreateSpec, One, One);

impl<W: WeightElement<Sum = i64>> MixedChinesePostman<W> {
    /// Create a new mixed Chinese postman instance.
    ///
    /// # Panics
    ///
    /// Panics if the weight-vector lengths do not match the graph shape or if
    /// any weight is negative.
    pub fn new(graph: MixedGraph, arc_weights: Vec<W>, edge_weights: Vec<W>) -> Self {
        Self::try_new(graph, arc_weights, edge_weights)
            .unwrap_or_else(|message| panic!("{message}"))
    }

    /// Create an instance, returning validation errors instead of panicking.
    pub fn try_new(
        graph: MixedGraph,
        arc_weights: Vec<W>,
        edge_weights: Vec<W>,
    ) -> Result<Self, crate::registry::ConstructionError> {
        if arc_weights.len() != graph.num_arcs() {
            return Err("arc_weights length must match num_arcs".to_string().into());
        }
        if edge_weights.len() != graph.num_edges() {
            return Err("edge_weights length must match num_edges"
                .to_string()
                .into());
        }
        for (index, weight) in arc_weights.iter().enumerate() {
            if !matches!(
                weight.to_sum().partial_cmp(&W::Sum::zero()),
                Some(Ordering::Equal | Ordering::Greater)
            ) {
                return Err(format!("arc weight at index {index} must be nonnegative").into());
            }
        }
        for (index, weight) in edge_weights.iter().enumerate() {
            if !matches!(
                weight.to_sum().partial_cmp(&W::Sum::zero()),
                Some(Ordering::Equal | Ordering::Greater)
            ) {
                return Err(format!("edge weight at index {index} must be nonnegative").into());
            }
        }

        Ok(Self {
            graph,
            arc_weights,
            edge_weights,
        })
    }

    /// Return the mixed graph.
    pub fn graph(&self) -> &MixedGraph {
        &self.graph
    }

    /// Return the directed-arc lengths.
    pub fn arc_weights(&self) -> &[W] {
        &self.arc_weights
    }

    /// Return the undirected-edge lengths.
    pub fn edge_weights(&self) -> &[W] {
        &self.edge_weights
    }

    /// Return the number of vertices.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Return the number of directed arcs.
    pub fn num_arcs(&self) -> usize {
        self.graph.num_arcs()
    }

    /// Return the number of undirected edges.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    /// Return whether this instance uses non-unit lengths.
    pub fn is_weighted(&self) -> bool {
        !W::IS_UNIT
    }

    fn oriented_arc_pairs(&self, config: &[bool]) -> Option<Vec<(usize, usize)>> {
        if config.len() != self.graph.num_edges() {
            return None;
        }

        let mut arcs = self.graph.arcs();
        for ((u, v), &reverse) in self.graph.edges().iter().zip(config.iter()) {
            if reverse {
                arcs.push((*v, *u));
            } else {
                arcs.push((*u, *v));
            }
        }
        Some(arcs)
    }

    fn available_arc_pairs(&self) -> Vec<(usize, usize)> {
        let mut arcs = self.graph.arcs();
        for &(u, v) in self.graph.edges().iter() {
            arcs.push((u, v));
            arcs.push((v, u));
        }
        arcs
    }

    fn weighted_available_arcs(&self) -> Vec<(usize, usize, i64)> {
        let mut arcs: Vec<(usize, usize, i64)> = self
            .graph
            .arcs()
            .into_iter()
            .zip(self.arc_weights.iter())
            .map(|((u, v), weight)| (u, v, weight.to_sum()))
            .collect();

        for ((u, v), weight) in self.graph.edges().iter().zip(self.edge_weights.iter()) {
            let cost = weight.to_sum();
            arcs.push((*u, *v, cost));
            arcs.push((*v, *u, cost));
        }

        arcs
    }

    fn base_cost(&self) -> Result<i64, crate::traits::EvaluationError> {
        let mut total = 0_i64;
        for weight in self.arc_weights.iter().chain(self.edge_weights.iter()) {
            total = W::checked_add_to_sum(
                total,
                weight.to_sum(),
                "summing mixed Chinese postman base costs",
            )?;
        }
        Ok(total)
    }
}

impl<W> MixedChinesePostman<W>
where
    W: WeightElement<Sum = i64> + crate::variant::VariantParam,
{
    /// Check whether a configuration yields a valid orientation (strongly
    /// connected with proper coverage).
    pub fn is_valid_solution(
        &self,
        config: &[bool],
    ) -> Result<bool, crate::traits::EvaluationError> {
        Ok(self.evaluate_solution(config)?.0.is_some())
    }

    fn evaluate_solution(
        &self,
        config: &[bool],
    ) -> Result<Min<W::Sum>, crate::traits::EvaluationError> {
        if config.len() != self.graph.num_edges() {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "edge-orientation length does not match the undirected edges".into(),
            ));
        }
        let Some(oriented_pairs) = self.oriented_arc_pairs(config) else {
            return Ok(Min(None));
        };

        if !DirectedGraph::new(self.graph.num_vertices(), self.available_arc_pairs())
            .is_strongly_connected()
        {
            return Ok(Min(None));
        }

        let distances =
            all_pairs_shortest_paths(self.graph.num_vertices(), &self.weighted_available_arcs())?;
        let balance = degree_imbalances(self.graph.num_vertices(), &oriented_pairs)?;
        let Some(extra_cost) = minimum_balancing_cost(&balance, &distances)? else {
            return Ok(Min(None));
        };

        let total = self.base_cost()?.checked_add(extra_cost).ok_or_else(|| {
            crate::traits::EvaluationError::IntegerOverflow(
                "summing mixed Chinese postman objective".to_string(),
            )
        })?;
        Ok(Min(Some(total)))
    }
}

impl<W> Problem for MixedChinesePostman<W>
where
    W: WeightElement<Sum = i64> + crate::variant::VariantParam,
{
    const NAME: &'static str = "MixedChinesePostman";
    type Solution = Vec<bool>;
    type Value = Min<W::Sum>;

    crate::problem_size![
        ("num_arcs", num_arcs),
        ("num_edges", num_edges),
        ("num_vertices", num_vertices),
    ];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![W]
    }

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Min<W::Sum>, crate::traits::EvaluationError> {
        self.evaluate_solution(config)
    }
}

impl<W> crate::solvers::BruteForceProblem for MixedChinesePostman<W>
where
    W: WeightElement<Sum = i64> + crate::variant::VariantParam,
{
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.graph.num_edges()]
    }
}

crate::declare_variants! {
    default MixedChinesePostman<i64> => "2^num_edges * num_vertices^3" create MixedChinesePostmanI64CreateSpec,
    MixedChinesePostman<One> => "2^num_edges * num_vertices^3" create MixedChinesePostmanOneCreateSpec,
}

crate::register_brute_force! {
    MixedChinesePostman<i64> decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
    MixedChinesePostman<One> decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "mixed_chinese_postman",
        instance: Box::new(MixedChinesePostman::new(
            MixedGraph::new(
                5,
                vec![(0, 1), (1, 2), (2, 3), (3, 0)],
                vec![(0, 2), (1, 3), (0, 4), (4, 2)],
            ),
            vec![2, 3, 1, 4],
            vec![2, 3, 1, 2],
        )),
        optimal_config: serde_json::json!(vec![true, true, false, false]),
        optimal_value: serde_json::json!(21),
    }]
}

fn all_pairs_shortest_paths(
    num_vertices: usize,
    arcs: &[(usize, usize, i64)],
) -> Result<Vec<Vec<i64>>, crate::traits::EvaluationError> {
    let mut distances = vec![vec![INF_COST; num_vertices]; num_vertices];

    for (vertex, row) in distances.iter_mut().enumerate() {
        row[vertex] = 0;
    }

    for &(u, v, cost) in arcs {
        if cost < distances[u][v] {
            distances[u][v] = cost;
        }
    }

    for via in 0..num_vertices {
        for src in 0..num_vertices {
            if distances[src][via] == INF_COST {
                continue;
            }
            for dst in 0..num_vertices {
                if distances[via][dst] == INF_COST {
                    continue;
                }
                let through = distances[src][via]
                    .checked_add(distances[via][dst])
                    .ok_or_else(|| {
                        crate::traits::EvaluationError::IntegerOverflow(
                            "computing mixed Chinese postman shortest paths".to_string(),
                        )
                    })?;
                if through < distances[src][dst] {
                    distances[src][dst] = through;
                }
            }
        }
    }

    Ok(distances)
}

fn degree_imbalances(
    num_vertices: usize,
    arcs: &[(usize, usize)],
) -> Result<Vec<i64>, crate::traits::EvaluationError> {
    let mut balance = vec![0_i64; num_vertices];
    for &(u, v) in arcs {
        balance[u] = balance[u].checked_add(1).ok_or_else(|| {
            crate::traits::EvaluationError::IntegerOverflow(
                "computing mixed Chinese postman degree imbalance".to_string(),
            )
        })?;
        balance[v] = balance[v].checked_sub(1).ok_or_else(|| {
            crate::traits::EvaluationError::IntegerOverflow(
                "computing mixed Chinese postman degree imbalance".to_string(),
            )
        })?;
    }
    Ok(balance)
}

fn minimum_balancing_cost(
    balance: &[i64],
    distances: &[Vec<i64>],
) -> Result<Option<i64>, crate::traits::EvaluationError> {
    let mut deficits = Vec::new();
    let mut surpluses = Vec::new();

    for (vertex, &value) in balance.iter().enumerate() {
        if value < 0 {
            for _ in 0..value.unsigned_abs() {
                deficits.push(vertex);
            }
        } else if value > 0 {
            for _ in 0..value.unsigned_abs() {
                surpluses.push(vertex);
            }
        }
    }

    if deficits.len() != surpluses.len() {
        return Ok(None);
    }
    if deficits.is_empty() {
        return Ok(Some(0));
    }

    let mut costs = vec![vec![INF_COST; surpluses.len()]; deficits.len()];
    for (row, &src) in deficits.iter().enumerate() {
        for (col, &dst) in surpluses.iter().enumerate() {
            costs[row][col] = distances[src][dst];
        }
    }

    hungarian_min_cost(&costs)
}

fn hungarian_min_cost(costs: &[Vec<i64>]) -> Result<Option<i64>, crate::traits::EvaluationError> {
    let size = costs.len();
    if size == 0 {
        return Ok(Some(0));
    }
    if costs.iter().any(|row| row.len() != size) {
        return Ok(None);
    }

    let mut u = vec![0_i64; size + 1];
    let mut v = vec![0_i64; size + 1];
    let mut p = vec![0_usize; size + 1];
    let mut way = vec![0_usize; size + 1];

    for row in 1..=size {
        p[0] = row;
        let mut column0 = 0;
        let mut minv = vec![INF_COST; size + 1];
        let mut used = vec![false; size + 1];

        loop {
            used[column0] = true;
            let row0 = p[column0];
            let mut delta = INF_COST;
            let mut column1 = 0;

            for column in 1..=size {
                if used[column] {
                    continue;
                }

                let current = costs[row0 - 1][column - 1]
                    .checked_sub(u[row0])
                    .and_then(|value| value.checked_sub(v[column]))
                    .ok_or_else(|| {
                        crate::traits::EvaluationError::IntegerOverflow(
                            "computing mixed Chinese postman assignment costs".to_string(),
                        )
                    })?;
                if current < minv[column] {
                    minv[column] = current;
                    way[column] = column0;
                }
                if minv[column] < delta {
                    delta = minv[column];
                    column1 = column;
                }
            }

            if delta == INF_COST {
                return Ok(None);
            }

            for column in 0..=size {
                if used[column] {
                    u[p[column]] = u[p[column]].checked_add(delta).ok_or_else(|| {
                        crate::traits::EvaluationError::IntegerOverflow(
                            "updating mixed Chinese postman assignment potentials".to_string(),
                        )
                    })?;
                    v[column] = v[column].checked_sub(delta).ok_or_else(|| {
                        crate::traits::EvaluationError::IntegerOverflow(
                            "updating mixed Chinese postman assignment potentials".to_string(),
                        )
                    })?;
                } else {
                    minv[column] = minv[column].checked_sub(delta).ok_or_else(|| {
                        crate::traits::EvaluationError::IntegerOverflow(
                            "updating mixed Chinese postman reduced costs".to_string(),
                        )
                    })?;
                }
            }

            column0 = column1;
            if p[column0] == 0 {
                break;
            }
        }

        loop {
            let column1 = way[column0];
            p[column0] = p[column1];
            column0 = column1;
            if column0 == 0 {
                break;
            }
        }
    }

    let mut assignment = vec![0_usize; size + 1];
    for column in 1..=size {
        assignment[p[column]] = column;
    }

    let mut total = 0_i64;
    for row in 1..=size {
        let cost = costs[row - 1][assignment[row] - 1];
        if cost == INF_COST {
            return Ok(None);
        }
        total = total.checked_add(cost).ok_or_else(|| {
            crate::traits::EvaluationError::IntegerOverflow(
                "summing mixed Chinese postman assignment costs".to_string(),
            )
        })?;
    }
    Ok(Some(total))
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/mixed_chinese_postman.rs"]
mod tests;
