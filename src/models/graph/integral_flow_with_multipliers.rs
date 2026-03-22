//! Integral Flow With Multipliers problem implementation.
//!
//! Given a directed graph with arc capacities, vertex multipliers on
//! non-terminals, and a sink demand, determine whether there exists an
//! integral flow satisfying multiplier-scaled conservation.

use crate::registry::{FieldInfo, ProblemSchemaEntry, ProblemSizeFieldEntry};
use crate::topology::DirectedGraph;
use crate::traits::{Problem, SatisfactionProblem};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "IntegralFlowWithMultipliers",
        display_name: "Integral Flow With Multipliers",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Integral flow feasibility on a directed graph with multiplier-scaled conservation at non-terminal vertices",
        fields: &[
            FieldInfo { name: "graph", type_name: "DirectedGraph", description: "Directed graph G = (V, A)" },
            FieldInfo { name: "source", type_name: "usize", description: "Source vertex s" },
            FieldInfo { name: "sink", type_name: "usize", description: "Sink vertex t" },
            FieldInfo { name: "multipliers", type_name: "Vec<u64>", description: "Vertex multipliers h(v) in vertex order; source/sink entries are ignored" },
            FieldInfo { name: "capacities", type_name: "Vec<u64>", description: "Arc capacities c(a) in graph arc order" },
            FieldInfo { name: "requirement", type_name: "u64", description: "Required net inflow R at the sink" },
        ],
    }
}

inventory::submit! {
    ProblemSizeFieldEntry {
        name: "IntegralFlowWithMultipliers",
        fields: &["num_vertices", "num_arcs", "max_capacity", "requirement"],
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegralFlowWithMultipliers {
    graph: DirectedGraph,
    source: usize,
    sink: usize,
    multipliers: Vec<u64>,
    capacities: Vec<u64>,
    requirement: u64,
}

impl IntegralFlowWithMultipliers {
    pub fn new(
        graph: DirectedGraph,
        source: usize,
        sink: usize,
        multipliers: Vec<u64>,
        capacities: Vec<u64>,
        requirement: u64,
    ) -> Self {
        assert_eq!(
            capacities.len(),
            graph.num_arcs(),
            "capacities length must match graph num_arcs"
        );
        assert_eq!(
            multipliers.len(),
            graph.num_vertices(),
            "multipliers length must match graph num_vertices"
        );

        let num_vertices = graph.num_vertices();
        assert!(
            source < num_vertices,
            "source ({source}) must be less than num_vertices ({num_vertices})"
        );
        assert!(
            sink < num_vertices,
            "sink ({sink}) must be less than num_vertices ({num_vertices})"
        );
        assert_ne!(source, sink, "source and sink must be distinct");

        for (vertex, &multiplier) in multipliers.iter().enumerate() {
            if vertex != source && vertex != sink {
                assert!(multiplier > 0, "non-terminal multipliers must be positive");
            }
        }

        for &capacity in &capacities {
            let domain = usize::try_from(capacity)
                .ok()
                .and_then(|value| value.checked_add(1));
            assert!(
                domain.is_some(),
                "arc capacities must fit into usize for dims()"
            );
        }

        Self {
            graph,
            source,
            sink,
            multipliers,
            capacities,
            requirement,
        }
    }

    pub fn graph(&self) -> &DirectedGraph {
        &self.graph
    }

    pub fn source(&self) -> usize {
        self.source
    }

    pub fn sink(&self) -> usize {
        self.sink
    }

    pub fn multipliers(&self) -> &[u64] {
        &self.multipliers
    }

    pub fn capacities(&self) -> &[u64] {
        &self.capacities
    }

    pub fn requirement(&self) -> u64 {
        self.requirement
    }

    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    pub fn num_arcs(&self) -> usize {
        self.graph.num_arcs()
    }

    pub fn max_capacity(&self) -> u64 {
        self.capacities.iter().copied().max().unwrap_or(0)
    }

    fn domain_size(capacity: u64) -> usize {
        usize::try_from(capacity)
            .ok()
            .and_then(|value| value.checked_add(1))
            .expect("capacity already validated to fit into usize")
    }

    pub fn is_feasible(&self, config: &[usize]) -> bool {
        if config.len() != self.num_arcs() {
            return false;
        }

        let num_vertices = self.num_vertices();
        let mut inflow = vec![0_i128; num_vertices];
        let mut outflow = vec![0_i128; num_vertices];

        for (arc_index, ((u, v), &capacity)) in self
            .graph
            .arcs()
            .into_iter()
            .zip(self.capacities.iter())
            .enumerate()
        {
            let Some(flow_usize) = config.get(arc_index).copied() else {
                return false;
            };
            let Ok(flow_u64) = u64::try_from(flow_usize) else {
                return false;
            };
            if flow_u64 > capacity {
                return false;
            }
            let flow = i128::from(flow_u64);
            outflow[u] += flow;
            inflow[v] += flow;
        }

        for vertex in 0..num_vertices {
            if vertex == self.source || vertex == self.sink {
                continue;
            }
            let multiplier = i128::from(self.multipliers[vertex]);
            let Some(expected_outflow) = inflow[vertex].checked_mul(multiplier) else {
                return false;
            };
            if expected_outflow != outflow[vertex] {
                return false;
            }
        }

        let sink_net_flow = inflow[self.sink] - outflow[self.sink];
        sink_net_flow >= i128::from(self.requirement)
    }
}

impl Problem for IntegralFlowWithMultipliers {
    const NAME: &'static str = "IntegralFlowWithMultipliers";
    type Metric = bool;

    fn dims(&self) -> Vec<usize> {
        self.capacities
            .iter()
            .map(|&capacity| Self::domain_size(capacity))
            .collect()
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        self.is_feasible(config)
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

impl SatisfactionProblem for IntegralFlowWithMultipliers {}

crate::declare_variants! {
    default sat IntegralFlowWithMultipliers => "(max_capacity + 1)^num_arcs",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "integral_flow_with_multipliers",
        instance: Box::new(IntegralFlowWithMultipliers::new(
            DirectedGraph::new(
                8,
                vec![
                    (0, 1),
                    (0, 2),
                    (0, 3),
                    (0, 4),
                    (0, 5),
                    (0, 6),
                    (1, 7),
                    (2, 7),
                    (3, 7),
                    (4, 7),
                    (5, 7),
                    (6, 7),
                ],
            ),
            0,
            7,
            vec![1, 2, 3, 4, 5, 6, 4, 1],
            vec![1, 1, 1, 1, 1, 1, 2, 3, 4, 5, 6, 4],
            12,
        )),
        optimal_config: vec![1, 0, 1, 0, 1, 0, 2, 0, 4, 0, 6, 0],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/integral_flow_with_multipliers.rs"]
mod tests;
