//! Integral Flow with Homologous Arcs problem implementation.
//!
//! Given a directed capacitated network with a source, sink, and pairs of arcs
//! that must carry equal flow, determine whether an integral flow meeting the
//! required sink inflow exists.

use crate::registry::{FieldInfo, ProblemSchemaEntry, ProblemSizeFieldEntry};
use crate::topology::DirectedGraph;
use crate::traits::{Problem, SatisfactionProblem};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "IntegralFlowHomologousArcs",
        display_name: "Integral Flow with Homologous Arcs",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Integral flow feasibility with arc-pair equality constraints",
        fields: &[
            FieldInfo { name: "graph", type_name: "DirectedGraph", description: "Directed graph G = (V, A)" },
            FieldInfo { name: "capacities", type_name: "Vec<u64>", description: "Capacity c(a) for each arc" },
            FieldInfo { name: "source", type_name: "usize", description: "Source vertex s" },
            FieldInfo { name: "sink", type_name: "usize", description: "Sink vertex t" },
            FieldInfo { name: "requirement", type_name: "u64", description: "Required net inflow R at the sink" },
            FieldInfo { name: "homologous_pairs", type_name: "Vec<(usize, usize)>", description: "Arc-index pairs (a, a') with f(a) = f(a')" },
        ],
    }
}

inventory::submit! {
    ProblemSizeFieldEntry {
        name: "IntegralFlowHomologousArcs",
        fields: &["num_vertices", "num_arcs"],
    }
}

/// Integral flow with homologous arcs.
///
/// A configuration stores one non-negative integer flow value for each arc in
/// the graph's arc order. The assignment is feasible when it respects arc
/// capacities, flow conservation at non-terminal vertices, every homologous-pair
/// equality constraint, and the required net inflow at the sink.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegralFlowHomologousArcs {
    graph: DirectedGraph,
    capacities: Vec<u64>,
    source: usize,
    sink: usize,
    requirement: u64,
    homologous_pairs: Vec<(usize, usize)>,
}

impl IntegralFlowHomologousArcs {
    pub fn new(
        graph: DirectedGraph,
        capacities: Vec<u64>,
        source: usize,
        sink: usize,
        requirement: u64,
        homologous_pairs: Vec<(usize, usize)>,
    ) -> Self {
        let num_vertices = graph.num_vertices();
        let num_arcs = graph.num_arcs();

        assert_eq!(
            capacities.len(),
            num_arcs,
            "capacities length must match graph.num_arcs()"
        );
        assert!(
            source < num_vertices,
            "source ({source}) must be less than num_vertices ({num_vertices})"
        );
        assert!(
            sink < num_vertices,
            "sink ({sink}) must be less than num_vertices ({num_vertices})"
        );

        for &(a, b) in &homologous_pairs {
            assert!(a < num_arcs, "homologous arc index {a} out of range");
            assert!(b < num_arcs, "homologous arc index {b} out of range");
        }

        for &capacity in &capacities {
            assert!(
                usize::try_from(capacity)
                    .ok()
                    .and_then(|value| value.checked_add(1))
                    .is_some(),
                "capacities must fit into usize for dims()"
            );
        }

        Self {
            graph,
            capacities,
            source,
            sink,
            requirement,
            homologous_pairs,
        }
    }

    pub fn graph(&self) -> &DirectedGraph {
        &self.graph
    }

    pub fn capacities(&self) -> &[u64] {
        &self.capacities
    }

    pub fn source(&self) -> usize {
        self.source
    }

    pub fn sink(&self) -> usize {
        self.sink
    }

    pub fn requirement(&self) -> u64 {
        self.requirement
    }

    pub fn homologous_pairs(&self) -> &[(usize, usize)] {
        &self.homologous_pairs
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

    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        self.evaluate(config)
    }

    fn domain_size(capacity: u64) -> usize {
        usize::try_from(capacity)
            .ok()
            .and_then(|value| value.checked_add(1))
            .expect("capacity already validated to fit into usize")
    }
}

impl Problem for IntegralFlowHomologousArcs {
    const NAME: &'static str = "IntegralFlowHomologousArcs";
    type Metric = bool;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        self.capacities
            .iter()
            .map(|&capacity| Self::domain_size(capacity))
            .collect()
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        if config.len() != self.num_arcs() {
            return false;
        }

        for &(a, b) in &self.homologous_pairs {
            if config[a] != config[b] {
                return false;
            }
        }

        let mut balances = vec![0_i128; self.num_vertices()];
        for (arc_index, ((u, v), &capacity)) in self
            .graph
            .arcs()
            .into_iter()
            .zip(self.capacities.iter())
            .enumerate()
        {
            let Ok(flow) = u64::try_from(config[arc_index]) else {
                return false;
            };
            if flow > capacity {
                return false;
            }
            let flow = i128::from(flow);
            balances[u] -= flow;
            balances[v] += flow;
        }

        for (vertex, &balance) in balances.iter().enumerate() {
            if vertex != self.source && vertex != self.sink && balance != 0 {
                return false;
            }
        }

        balances[self.sink] >= i128::from(self.requirement)
    }
}

impl SatisfactionProblem for IntegralFlowHomologousArcs {}

crate::declare_variants! {
    default sat IntegralFlowHomologousArcs => "(max_capacity + 1)^num_arcs",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "integral_flow_homologous_arcs",
        instance: Box::new(IntegralFlowHomologousArcs::new(
            DirectedGraph::new(
                6,
                vec![
                    (0, 1),
                    (0, 2),
                    (1, 3),
                    (2, 3),
                    (1, 4),
                    (2, 4),
                    (3, 5),
                    (4, 5),
                ],
            ),
            vec![1; 8],
            0,
            5,
            2,
            vec![(2, 5), (4, 3)],
        )),
        optimal_config: vec![1, 1, 1, 0, 0, 1, 1, 1],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/integral_flow_homologous_arcs.rs"]
mod tests;
