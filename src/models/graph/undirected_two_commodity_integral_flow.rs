//! Undirected two-commodity integral flow problem implementation.
//!
//! The problem asks whether two integral commodities can be routed through an
//! undirected capacitated graph while sharing edge capacities.

use crate::registry::{CreateSpec, ProblemSchemaEntry};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "UndirectedTwoCommodityIntegralFlow",
        display_name: "Undirected Two-Commodity Integral Flow",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Determine whether two integral commodities can satisfy sink demands in an undirected capacitated graph",
        fields: UndirectedTwoCommodityIntegralFlowCreateSpec::FIELDS,
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
    capacities: Vec<i64>,
    source_1: usize,
    sink_1: usize,
    source_2: usize,
    sink_2: usize,
    requirement_1: i64,
    requirement_2: i64,
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct UndirectedTwoCommodityIntegralFlowCreateSpec {
    /// Undirected graph edges.
    #[create(codec = "edge-list")]
    graph: Vec<(usize, usize)>,
    /// Vertex count, needed for isolated vertices.
    num_vertices: Option<usize>,
    /// Edge capacities.
    #[create(codec = "comma-separated")]
    capacities: Vec<i64>,
    source_1: usize,
    sink_1: usize,
    source_2: usize,
    sink_2: usize,
    requirement_1: i64,
    requirement_2: i64,
}

impl TryFrom<UndirectedTwoCommodityIntegralFlowCreateSpec> for UndirectedTwoCommodityIntegralFlow {
    type Error = crate::registry::ConstructionError;
    fn try_from(spec: UndirectedTwoCommodityIntegralFlowCreateSpec) -> Result<Self, Self::Error> {
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
        if spec.capacities.len() != spec.graph.len() {
            return Err("capacities length must match graph edge count".into());
        }
        for &capacity in &spec.capacities {
            if usize::try_from(capacity)
                .ok()
                .and_then(|v| v.checked_add(1))
                .is_none()
            {
                return Err("capacity is too large for this platform".into());
            }
        }
        for (label, vertex) in [
            ("source_1", spec.source_1),
            ("sink_1", spec.sink_1),
            ("source_2", spec.source_2),
            ("sink_2", spec.sink_2),
        ] {
            if vertex >= count {
                return Err(format!("{label} must be less than num_vertices").into());
            }
        }
        Ok(Self {
            graph: SimpleGraph::new(count, spec.graph),
            capacities: spec.capacities,
            source_1: spec.source_1,
            sink_1: spec.sink_1,
            source_2: spec.source_2,
            sink_2: spec.sink_2,
            requirement_1: spec.requirement_1,
            requirement_2: spec.requirement_2,
        })
    }
}

impl UndirectedTwoCommodityIntegralFlow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        graph: SimpleGraph,
        capacities: Vec<i64>,
        source_1: usize,
        sink_1: usize,
        source_2: usize,
        sink_2: usize,
        requirement_1: i64,
        requirement_2: i64,
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

    pub fn capacities(&self) -> &[i64] {
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

    pub fn requirement_1(&self) -> i64 {
        self.requirement_1
    }

    pub fn requirement_2(&self) -> i64 {
        self.requirement_2
    }

    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    pub fn num_conservation_constraints(&self) -> usize {
        [(self.source_1, self.sink_1), (self.source_2, self.sink_2)]
            .into_iter()
            .map(|(source, sink)| self.num_vertices() - if source == sink { 1 } else { 2 })
            .sum()
    }

    pub fn is_valid_solution(
        &self,
        config: &[usize],
    ) -> Result<bool, crate::traits::EvaluationError> {
        Ok(self.evaluate_solution(config)?.0)
    }

    fn config_len(&self) -> usize {
        self.num_edges() * 4
    }

    fn domain_size(capacity: i64) -> usize {
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

    fn flow_pair_for_commodity(flows: [usize; 4], commodity: usize) -> (usize, usize) {
        match commodity {
            1 => (flows[0], flows[1]),
            2 => (flows[2], flows[3]),
            _ => unreachable!("commodity must be 1 or 2"),
        }
    }

    fn commodity_balance(
        &self,
        config: &[usize],
        commodity: usize,
        vertex: usize,
    ) -> Result<Option<i64>, crate::traits::EvaluationError> {
        let mut balance = 0_i64;
        for (edge_index, (u, v)) in self.graph.edges().into_iter().enumerate() {
            let Some(flows) = self.edge_flows(config, edge_index) else {
                return Ok(None);
            };
            let (uv, vu) = Self::flow_pair_for_commodity(flows, commodity);
            let uv = i64::try_from(uv).map_err(|_| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "converting forward commodity flow to i64".into(),
                )
            })?;
            let vu = i64::try_from(vu).map_err(|_| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "converting reverse commodity flow to i64".into(),
                )
            })?;

            if vertex == u {
                balance = balance.checked_sub(uv).ok_or_else(|| {
                    crate::traits::EvaluationError::IntegerOverflow(
                        "subtracting forward commodity flow".into(),
                    )
                })?;
                balance = balance.checked_add(vu).ok_or_else(|| {
                    crate::traits::EvaluationError::IntegerOverflow(
                        "adding reverse commodity flow".into(),
                    )
                })?;
            } else if vertex == v {
                balance = balance.checked_add(uv).ok_or_else(|| {
                    crate::traits::EvaluationError::IntegerOverflow(
                        "adding forward commodity flow".into(),
                    )
                })?;
                balance = balance.checked_sub(vu).ok_or_else(|| {
                    crate::traits::EvaluationError::IntegerOverflow(
                        "subtracting reverse commodity flow".into(),
                    )
                })?;
            }
        }
        Ok(Some(balance))
    }

    fn net_flow_into_sink(
        &self,
        config: &[usize],
        commodity: usize,
    ) -> Result<Option<i64>, crate::traits::EvaluationError> {
        let sink = match commodity {
            1 => self.sink_1,
            2 => self.sink_2,
            _ => unreachable!("commodity must be 1 or 2"),
        };
        self.commodity_balance(config, commodity, sink)
    }

    fn evaluate_solution(
        &self,
        config: &[usize],
    ) -> Result<crate::types::Or, crate::traits::EvaluationError> {
        if config.len() != self.config_len() {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "flow representation length does not match the graph".into(),
            ));
        }

        for (edge_index, &capacity) in self.capacities.iter().enumerate() {
            let Some(flows) = self.edge_flows(config, edge_index) else {
                return Ok(crate::types::Or(false));
            };

            if flows
                .iter()
                .any(|&value| i64::try_from(value).map_or(true, |value| value > capacity))
            {
                return Ok(crate::types::Or(false));
            }
            if flows[0] > 0 && flows[1] > 0 || flows[2] > 0 && flows[3] > 0 {
                return Ok(crate::types::Or(false));
            }

            let commodity_1 = i64::try_from(std::cmp::max(flows[0], flows[1]))
                .expect("flow values already validated against i64 capacities");
            let commodity_2 = i64::try_from(std::cmp::max(flows[2], flows[3]))
                .expect("flow values already validated against i64 capacities");
            let shared = commodity_1.checked_add(commodity_2).ok_or_else(|| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "summing two commodities on an undirected edge".into(),
                )
            })?;
            if shared > capacity {
                return Ok(crate::types::Or(false));
            }
        }

        for (commodity, source, sink) in [
            (1, self.source_1, self.sink_1),
            (2, self.source_2, self.sink_2),
        ] {
            for vertex in 0..self.num_vertices() {
                if vertex != source
                    && vertex != sink
                    && self.commodity_balance(config, commodity, vertex)? != Some(0)
                {
                    return Ok(crate::types::Or(false));
                }
            }
        }

        Ok(crate::types::Or(
            self.net_flow_into_sink(config, 1)?
                .is_some_and(|flow| flow >= self.requirement_1)
                && self
                    .net_flow_into_sink(config, 2)?
                    .is_some_and(|flow| flow >= self.requirement_2),
        ))
    }
}

impl Problem for UndirectedTwoCommodityIntegralFlow {
    const NAME: &'static str = "UndirectedTwoCommodityIntegralFlow";
    type Solution = Vec<usize>;
    type Value = crate::types::Or;

    crate::problem_parameters![
        ("num_edges", num_edges),
        ("num_conservation_constraints", num_conservation_constraints),
        ("num_vertices", num_vertices),
    ];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<crate::types::Or, crate::traits::EvaluationError> {
        self.evaluate_solution(config)
    }
}

impl crate::solvers::BruteForceProblem for UndirectedTwoCommodityIntegralFlow {
    fn dimensions(&self) -> Vec<usize> {
        self.capacities
            .iter()
            .flat_map(|&capacity| {
                let domain = Self::domain_size(capacity);
                std::iter::repeat_n(domain, 4)
            })
            .collect()
    }
}

crate::declare_variants! {
    default UndirectedTwoCommodityIntegralFlow => "5^num_edges" create UndirectedTwoCommodityIntegralFlowCreateSpec,
}

crate::register_brute_force! {
    UndirectedTwoCommodityIntegralFlow,
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "undirected_two_commodity_integral_flow",
        instance: Box::new(UndirectedTwoCommodityIntegralFlow::new(
            SimpleGraph::new(4, vec![(0, 2), (1, 2), (2, 3)]),
            vec![1, 1, 2],
            0,
            3,
            1,
            3,
            1,
            1,
        )),
        optimal_config: serde_json::json!(vec![1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0]),
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/undirected_two_commodity_integral_flow.rs"]
mod tests;
