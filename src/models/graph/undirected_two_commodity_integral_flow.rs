//! Undirected two-commodity integral flow problem implementation.
//!
//! The problem asks whether two integral commodities can be routed through an
//! undirected capacitated graph while sharing edge capacities.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::{Problem, SatisfactionProblem};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "UndirectedTwoCommodityIntegralFlow",
        display_name: "Undirected Two-Commodity Integral Flow",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Determine whether two integral commodities can satisfy sink demands in an undirected capacitated graph",
        fields: &[
            FieldInfo { name: "graph", type_name: "SimpleGraph", description: "Undirected graph G=(V,E)" },
            FieldInfo { name: "capacities", type_name: "Vec<u64>", description: "Edge capacities c(e) in graph edge order" },
            FieldInfo { name: "source_1", type_name: "usize", description: "Source vertex s_1 for commodity 1" },
            FieldInfo { name: "sink_1", type_name: "usize", description: "Sink vertex t_1 for commodity 1" },
            FieldInfo { name: "source_2", type_name: "usize", description: "Source vertex s_2 for commodity 2" },
            FieldInfo { name: "sink_2", type_name: "usize", description: "Sink vertex t_2 for commodity 2" },
            FieldInfo { name: "requirement_1", type_name: "u64", description: "Required net inflow R_1 at sink t_1" },
            FieldInfo { name: "requirement_2", type_name: "u64", description: "Required net inflow R_2 at sink t_2" },
        ],
    }
}

/// Undirected two-commodity integral flow on a capacitated graph.
///
/// For each undirected edge `{u, v}`, a configuration stores four variables in
/// the graph's edge order:
/// - `f1(u, v)`
/// - `f1(v, u)`
/// - `f2(u, v)`
/// - `f2(v, u)`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UndirectedTwoCommodityIntegralFlow {
    graph: SimpleGraph,
    capacities: Vec<u64>,
    source_1: usize,
    sink_1: usize,
    source_2: usize,
    sink_2: usize,
    requirement_1: u64,
    requirement_2: u64,
}

impl UndirectedTwoCommodityIntegralFlow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        graph: SimpleGraph,
        capacities: Vec<u64>,
        source_1: usize,
        sink_1: usize,
        source_2: usize,
        sink_2: usize,
        requirement_1: u64,
        requirement_2: u64,
    ) -> Self {
        assert_eq!(
            capacities.len(),
            graph.num_edges(),
            "capacities length must match graph num_edges"
        );

        let num_vertices = graph.num_vertices();
        for (label, vertex) in [
            ("source_1", source_1),
            ("sink_1", sink_1),
            ("source_2", source_2),
            ("sink_2", sink_2),
        ] {
            assert!(
                vertex < num_vertices,
                "{label} must be less than num_vertices ({num_vertices})"
            );
        }

        for &capacity in &capacities {
            let domain = usize::try_from(capacity)
                .ok()
                .and_then(|value| value.checked_add(1));
            assert!(
                domain.is_some(),
                "edge capacities must fit into usize for dims()"
            );
        }

        Self {
            graph,
            capacities,
            source_1,
            sink_1,
            source_2,
            sink_2,
            requirement_1,
            requirement_2,
        }
    }

    pub fn graph(&self) -> &SimpleGraph {
        &self.graph
    }

    pub fn capacities(&self) -> &[u64] {
        &self.capacities
    }

    pub fn source_1(&self) -> usize {
        self.source_1
    }

    pub fn sink_1(&self) -> usize {
        self.sink_1
    }

    pub fn source_2(&self) -> usize {
        self.source_2
    }

    pub fn sink_2(&self) -> usize {
        self.sink_2
    }

    pub fn requirement_1(&self) -> u64 {
        self.requirement_1
    }

    pub fn requirement_2(&self) -> u64 {
        self.requirement_2
    }

    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        self.evaluate(config)
    }

    fn config_len(&self) -> usize {
        self.num_edges() * 4
    }

    fn domain_size(capacity: u64) -> usize {
        usize::try_from(capacity)
            .ok()
            .and_then(|value| value.checked_add(1))
            .expect("capacity already validated to fit into usize")
    }

    fn edge_flows(&self, config: &[usize], edge_index: usize) -> Option<[usize; 4]> {
        let start = edge_index.checked_mul(4)?;
        Some([
            *config.get(start)?,
            *config.get(start + 1)?,
            *config.get(start + 2)?,
            *config.get(start + 3)?,
        ])
    }

    fn is_terminal(&self, vertex: usize) -> bool {
        [self.source_1, self.sink_1, self.source_2, self.sink_2].contains(&vertex)
    }

    fn flow_pair_for_commodity(flows: [usize; 4], commodity: usize) -> (usize, usize) {
        match commodity {
            1 => (flows[0], flows[1]),
            2 => (flows[2], flows[3]),
            _ => unreachable!("commodity must be 1 or 2"),
        }
    }

    fn commodity_balance(&self, config: &[usize], commodity: usize, vertex: usize) -> Option<i64> {
        let mut balance = 0i64;
        for (edge_index, (u, v)) in self.graph.edges().into_iter().enumerate() {
            let flows = self.edge_flows(config, edge_index)?;
            let (uv, vu) = Self::flow_pair_for_commodity(flows, commodity);
            let uv = i64::try_from(uv).ok()?;
            let vu = i64::try_from(vu).ok()?;

            if vertex == u {
                balance -= uv;
                balance += vu;
            } else if vertex == v {
                balance += uv;
                balance -= vu;
            }
        }
        Some(balance)
    }

    fn net_flow_into_sink(&self, config: &[usize], commodity: usize) -> Option<u64> {
        let sink = match commodity {
            1 => self.sink_1,
            2 => self.sink_2,
            _ => unreachable!("commodity must be 1 or 2"),
        };
        let balance = self.commodity_balance(config, commodity, sink)?;
        u64::try_from(balance).ok()
    }
}

impl Problem for UndirectedTwoCommodityIntegralFlow {
    const NAME: &'static str = "UndirectedTwoCommodityIntegralFlow";
    type Metric = bool;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        self.capacities
            .iter()
            .flat_map(|&capacity| {
                let domain = Self::domain_size(capacity);
                std::iter::repeat_n(domain, 4)
            })
            .collect()
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        if config.len() != self.config_len() {
            return false;
        }

        for (edge_index, &capacity) in self.capacities.iter().enumerate() {
            let Some(flows) = self.edge_flows(config, edge_index) else {
                return false;
            };

            if flows
                .iter()
                .any(|&value| u64::try_from(value).map_or(true, |value| value > capacity))
            {
                return false;
            }

            if flows[0] > 0 && flows[1] > 0 {
                return false;
            }
            if flows[2] > 0 && flows[3] > 0 {
                return false;
            }

            let shared = std::cmp::max(flows[0], flows[1]) + std::cmp::max(flows[2], flows[3]);
            if u64::try_from(shared).map_or(true, |value| value > capacity) {
                return false;
            }
        }

        for vertex in 0..self.num_vertices() {
            if self.is_terminal(vertex) {
                continue;
            }

            if self.commodity_balance(config, 1, vertex) != Some(0)
                || self.commodity_balance(config, 2, vertex) != Some(0)
            {
                return false;
            }
        }

        self.net_flow_into_sink(config, 1)
            .is_some_and(|flow| flow >= self.requirement_1)
            && self
                .net_flow_into_sink(config, 2)
                .is_some_and(|flow| flow >= self.requirement_2)
    }
}

impl SatisfactionProblem for UndirectedTwoCommodityIntegralFlow {}

crate::declare_variants! {
    default sat UndirectedTwoCommodityIntegralFlow => "5^num_edges",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "undirected_two_commodity_integral_flow",
        build: || {
            let problem = UndirectedTwoCommodityIntegralFlow::new(
                SimpleGraph::new(4, vec![(0, 2), (1, 2), (2, 3)]),
                vec![1, 1, 2],
                0,
                3,
                1,
                3,
                1,
                1,
            );
            crate::example_db::specs::satisfaction_example(
                problem,
                vec![vec![1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0]],
            )
        },
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/undirected_two_commodity_integral_flow.rs"]
mod tests;
