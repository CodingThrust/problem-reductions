//! Integral Flow With Multipliers problem implementation.
//!
//! Given a directed graph with arc capacities, vertex multipliers on
//! non-terminals, and a sink demand, determine whether there exists an
//! integral flow satisfying multiplier-scaled conservation.

use crate::registry::{CreateSpec, ProblemSchemaEntry};
use crate::topology::DirectedGraph;
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "IntegralFlowWithMultipliers",
        display_name: "Integral Flow With Multipliers",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Integral flow feasibility on a directed graph with multiplier-scaled conservation at non-terminal vertices",
        fields: IntegralFlowWithMultipliersCreateSpec::FIELDS,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegralFlowWithMultipliers {
    graph: DirectedGraph,
    source: usize,
    sink: usize,
    multipliers: Vec<i64>,
    capacities: Vec<i64>,
    requirement: i64,
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct IntegralFlowWithMultipliersCreateSpec {
    #[create(codec = "arc-list")]
    arcs: Vec<(usize, usize)>,
    num_vertices: Option<usize>,
    #[create(codec = "comma-separated")]
    capacities: Vec<i64>,
    source: usize,
    sink: usize,
    #[create(codec = "comma-separated")]
    multipliers: Vec<i64>,
    requirement: i64,
}

impl TryFrom<IntegralFlowWithMultipliersCreateSpec> for IntegralFlowWithMultipliers {
    type Error = crate::registry::ConstructionError;
    fn try_from(
        spec: IntegralFlowWithMultipliersCreateSpec,
    ) -> Result<Self, crate::registry::ConstructionError> {
        if spec.arcs.is_empty() {
            return Err("arcs must be non-empty".into());
        }
        let inferred = spec
            .arcs
            .iter()
            .flat_map(|&(u, v)| [u, v])
            .max()
            .map(|v| v.checked_add(1).ok_or("vertex count overflows usize"))
            .transpose()?
            .unwrap_or(0);
        let count = spec.num_vertices.unwrap_or(inferred);
        if count < inferred {
            return Err("num_vertices is too small".into());
        }
        if spec.capacities.len() != spec.arcs.len() {
            return Err("capacities length must match arcs length".into());
        }
        if spec.multipliers.len() != count {
            return Err("multipliers length must match num_vertices".into());
        }
        if spec.source >= count || spec.sink >= count {
            return Err("source and sink must be valid vertices".into());
        }
        if spec.source == spec.sink {
            return Err("source and sink must be distinct".into());
        }
        for (v, &m) in spec.multipliers.iter().enumerate() {
            if v != spec.source && v != spec.sink && m == 0 {
                return Err("non-terminal multipliers must be positive".into());
            }
        }
        for &c in &spec.capacities {
            if usize::try_from(c)
                .ok()
                .and_then(|v| v.checked_add(1))
                .is_none()
            {
                return Err("capacity is too large".into());
            }
        }
        Ok(Self {
            graph: DirectedGraph::new(count, spec.arcs),
            source: spec.source,
            sink: spec.sink,
            multipliers: spec.multipliers,
            capacities: spec.capacities,
            requirement: spec.requirement,
        })
    }
}

impl IntegralFlowWithMultipliers {
    pub fn new(
        graph: DirectedGraph,
        source: usize,
        sink: usize,
        multipliers: Vec<i64>,
        capacities: Vec<i64>,
        requirement: i64,
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

    pub fn multipliers(&self) -> &[i64] {
        &self.multipliers
    }

    pub fn capacities(&self) -> &[i64] {
        &self.capacities
    }

    pub fn requirement(&self) -> i64 {
        self.requirement
    }

    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    pub fn num_arcs(&self) -> usize {
        self.graph.num_arcs()
    }

    pub fn max_capacity(&self) -> i64 {
        self.capacities.iter().copied().max().unwrap_or(0)
    }

    fn domain_size(capacity: i64) -> usize {
        usize::try_from(capacity)
            .ok()
            .and_then(|value| value.checked_add(1))
            .expect("capacity already validated to fit into usize")
    }

    pub fn is_feasible(&self, config: &[usize]) -> Result<bool, crate::traits::EvaluationError> {
        if config.len() != self.num_arcs() {
            return Ok(false);
        }

        let num_vertices = self.num_vertices();
        let mut inflow = vec![0_i64; num_vertices];
        let mut outflow = vec![0_i64; num_vertices];

        for (arc_index, ((u, v), &capacity)) in self
            .graph
            .arcs()
            .into_iter()
            .zip(self.capacities.iter())
            .enumerate()
        {
            let Some(flow_usize) = config.get(arc_index).copied() else {
                return Ok(false);
            };
            let Ok(flow_u64) = i64::try_from(flow_usize) else {
                return Ok(false);
            };
            if flow_u64 > capacity {
                return Ok(false);
            }
            outflow[u] = outflow[u].checked_add(flow_u64).ok_or_else(|| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "summing outgoing multiplied flow".into(),
                )
            })?;
            inflow[v] = inflow[v].checked_add(flow_u64).ok_or_else(|| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "summing incoming multiplied flow".into(),
                )
            })?;
        }

        for vertex in 0..num_vertices {
            if vertex == self.source || vertex == self.sink {
                continue;
            }
            let expected_outflow = inflow[vertex]
                .checked_mul(self.multipliers[vertex])
                .ok_or_else(|| {
                    crate::traits::EvaluationError::IntegerOverflow(
                        "multiplying incoming flow by vertex multiplier".into(),
                    )
                })?;
            if expected_outflow != outflow[vertex] {
                return Ok(false);
            }
        }

        let sink_net_flow = inflow[self.sink]
            .checked_sub(outflow[self.sink])
            .ok_or_else(|| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "computing net flow into sink".into(),
                )
            })?;
        Ok(sink_net_flow >= self.requirement)
    }
}

impl Problem for IntegralFlowWithMultipliers {
    const NAME: &'static str = "IntegralFlowWithMultipliers";
    type Solution = Vec<usize>;
    type Value = crate::types::Or;

    crate::problem_size![
        ("max_capacity", max_capacity),
        ("num_arcs", num_arcs),
        ("num_vertices", num_vertices),
        ("requirement", requirement),
    ];

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<crate::types::Or, crate::traits::EvaluationError> {
        if config.len() != self.num_arcs() {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "flow vector length does not match the graph arcs".into(),
            ));
        }
        Ok(crate::types::Or(self.is_feasible(config)?))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

impl crate::solvers::BruteForceProblem for IntegralFlowWithMultipliers {
    fn dimensions(&self) -> Vec<usize> {
        self.capacities
            .iter()
            .map(|&capacity| Self::domain_size(capacity))
            .collect()
    }
}

crate::declare_variants! {
    default IntegralFlowWithMultipliers => "(max_capacity + 1)^num_arcs" create IntegralFlowWithMultipliersCreateSpec,
}

crate::register_brute_force! {
    IntegralFlowWithMultipliers,
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
        optimal_config: serde_json::json!(vec![1, 0, 1, 0, 1, 0, 2, 0, 4, 0, 6, 0]),
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/integral_flow_with_multipliers.rs"]
mod tests;
