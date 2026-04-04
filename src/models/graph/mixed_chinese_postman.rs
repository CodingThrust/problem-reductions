//! Mixed Chinese Postman problem implementation.
//!
//! Given a mixed graph with directed arcs and undirected edges, find a
//! minimum-cost closed walk that traverses every directed arc in its prescribed
//! direction and every undirected edge in at least one direction.

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
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
            VariantDimension::new("weight", "i32", &["i32", "One"]),
        ],
        module_path: module_path!(),
        description: "Find a minimum-cost closed walk covering all arcs and edges in a mixed graph",
        fields: &[
            FieldInfo { name: "graph", type_name: "MixedGraph", description: "The mixed graph G=(V,A,E)" },
            FieldInfo { name: "arc_weights", type_name: "Vec<W>", description: "Lengths for the directed arcs in A" },
            FieldInfo { name: "edge_weights", type_name: "Vec<W>", description: "Lengths for the undirected edges in E" },
        ],
    }
}

/// Mixed Chinese Postman.
///
/// Each configuration picks a required traversal direction for every undirected
/// edge. The minimum-cost closed walk is then computed via the directed Chinese
/// Postman subproblem, using all available arcs (including both directions of
/// every undirected edge) for degree-balancing detours.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MixedChinesePostman<W: WeightElement<Sum = i32>> {
    graph: MixedGraph,
    arc_weights: Vec<W>,
    edge_weights: Vec<W>,
}

impl<W: WeightElement<Sum = i32>> MixedChinesePostman<W> {
    /// Create a new mixed Chinese postman instance.
    ///
    /// # Panics
    ///
    /// Panics if the weight-vector lengths do not match the graph shape or if
    /// any weight is negative.
    pub fn new(graph: MixedGraph, arc_weights: Vec<W>, edge_weights: Vec<W>) -> Self {
        assert_eq!(
            arc_weights.len(),
            graph.num_arcs(),
            "arc_weights length must match num_arcs"
        );
        assert_eq!(
            edge_weights.len(),
            graph.num_edges(),
            "edge_weights length must match num_edges"
        );
        for (index, weight) in arc_weights.iter().enumerate() {
            assert!(
                matches!(
                    weight.to_sum().partial_cmp(&W::Sum::zero()),
                    Some(Ordering::Equal | Ordering::Greater)
                ),
                "arc weight at index {} must be nonnegative",
                index
            );
        }
        for (index, weight) in edge_weights.iter().enumerate() {
            assert!(
                matches!(
                    weight.to_sum().partial_cmp(&W::Sum::zero()),
                    Some(Ordering::Equal | Ordering::Greater)
                ),
                "edge weight at index {} must be nonnegative",
                index
            );
        }

        Self {
            graph,
            arc_weights,
            edge_weights,
        }
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

    fn oriented_arc_pairs(&self, config: &[usize]) -> Option<Vec<(usize, usize)>> {
        if config.len() != self.graph.num_edges() {
            return None;
        }

        let mut arcs = self.graph.arcs();
        for ((u, v), &direction) in self.graph.edges().iter().zip(config.iter()) {
            match direction {
                0 => arcs.push((*u, *v)),
                1 => arcs.push((*v, *u)),
                _ => return None,
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
            .map(|((u, v), weight)| (u, v, i64::from(weight.to_sum())))
            .collect();

        for ((u, v), weight) in self.graph.edges().iter().zip(self.edge_weights.iter()) {
            let cost = i64::from(weight.to_sum());
            arcs.push((*u, *v, cost));
            arcs.push((*v, *u, cost));
        }

        arcs
    }

    fn base_cost(&self) -> i64 {
        self.arc_weights
            .iter()
            .map(|weight| i64::from(weight.to_sum()))
            .sum::<i64>()
            + self
                .edge_weights
                .iter()
                .map(|weight| i64::from(weight.to_sum()))
                .sum::<i64>()
    }
}

impl<W> MixedChinesePostman<W>
where
    W: WeightElement<Sum = i32> + crate::variant::VariantParam,
{
    /// Check whether a configuration yields a valid orientation (strongly
    /// connected with proper coverage).
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        self.evaluate(config).0.is_some()
    }
}

impl<W> Problem for MixedChinesePostman<W>
where
    W: WeightElement<Sum = i32> + crate::variant::VariantParam,
{
    const NAME: &'static str = "MixedChinesePostman";
    type Value = Min<W::Sum>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![W]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_edges()]
    }

    fn evaluate(&self, config: &[usize]) -> Min<W::Sum> {
        let Some(oriented_pairs) = self.oriented_arc_pairs(config) else {
            return Min(None);
        };

        // Connectivity uses the full available graph: original arcs plus both
        // directions of every undirected edge.
        if !DirectedGraph::new(self.graph.num_vertices(), self.available_arc_pairs())
            .is_strongly_connected()
        {
            return Min(None);
        }

        // Shortest paths also use the full available graph so that balancing
        // can route through undirected edges in either direction.
        let distances =
            all_pairs_shortest_paths(self.graph.num_vertices(), &self.weighted_available_arcs());
        // Degree imbalance is computed from the required arcs only (original
        // arcs plus the chosen orientation of each undirected edge).
        let balance = degree_imbalances(self.graph.num_vertices(), &oriented_pairs);
        let Some(extra_cost) = minimum_balancing_cost(&balance, &distances) else {
            return Min(None);
        };

        let total = self.base_cost() + extra_cost;
        Min(Some(total as W::Sum))
    }
}

crate::declare_variants! {
    default MixedChinesePostman<i32> => "2^num_edges * num_vertices^3",
    MixedChinesePostman<One> => "2^num_edges * num_vertices^3",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "mixed_chinese_postman_i32",
        instance: Box::new(MixedChinesePostman::new(
            MixedGraph::new(
                5,
                vec![(0, 1), (1, 2), (2, 3), (3, 0)],
                vec![(0, 2), (1, 3), (0, 4), (4, 2)],
            ),
            vec![2, 3, 1, 4],
            vec![2, 3, 1, 2],
        )),
        optimal_config: vec![1, 1, 0, 0],
        optimal_value: serde_json::json!(21),
    }]
}

fn all_pairs_shortest_paths(num_vertices: usize, arcs: &[(usize, usize, i64)]) -> Vec<Vec<i64>> {
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
                let through = distances[src][via] + distances[via][dst];
                if through < distances[src][dst] {
                    distances[src][dst] = through;
                }
            }
        }
    }

    distances
}

fn degree_imbalances(num_vertices: usize, arcs: &[(usize, usize)]) -> Vec<i32> {
    let mut balance = vec![0_i32; num_vertices];
    for &(u, v) in arcs {
        balance[u] += 1;
        balance[v] -= 1;
    }
    balance
}

fn minimum_balancing_cost(balance: &[i32], distances: &[Vec<i64>]) -> Option<i64> {
    let mut deficits = Vec::new();
    let mut surpluses = Vec::new();

    for (vertex, &value) in balance.iter().enumerate() {
        if value < 0 {
            for _ in 0..usize::try_from(-value).ok()? {
                deficits.push(vertex);
            }
        } else if value > 0 {
            for _ in 0..usize::try_from(value).ok()? {
                surpluses.push(vertex);
            }
        }
    }

    if deficits.len() != surpluses.len() {
        return None;
    }
    if deficits.is_empty() {
        return Some(0);
    }

    let mut costs = vec![vec![INF_COST; surpluses.len()]; deficits.len()];
    for (row, &src) in deficits.iter().enumerate() {
        for (col, &dst) in surpluses.iter().enumerate() {
            costs[row][col] = distances[src][dst];
        }
    }

    hungarian_min_cost(&costs)
}

fn hungarian_min_cost(costs: &[Vec<i64>]) -> Option<i64> {
    let size = costs.len();
    if size == 0 {
        return Some(0);
    }
    if costs.iter().any(|row| row.len() != size) {
        return None;
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

                let current = costs[row0 - 1][column - 1] - u[row0] - v[column];
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
                return None;
            }

            for column in 0..=size {
                if used[column] {
                    u[p[column]] += delta;
                    v[column] -= delta;
                } else {
                    minv[column] -= delta;
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
            return None;
        }
        total += cost;
    }
    Some(total)
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/mixed_chinese_postman.rs"]
mod tests;
