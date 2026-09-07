//! Integral Flow with Homologous Arcs problem implementation.
//!
//! Given a directed capacitated network with a source, sink, and pairs of arcs
//! that must carry equal flow, determine whether an integral flow meeting the
//! required sink inflow exists.

use crate::registry::{CreateSpec, ProblemSchemaEntry};
use crate::topology::DirectedGraph;
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "IntegralFlowHomologousArcs",
        display_name: "Integral Flow with Homologous Arcs",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Integral flow feasibility with arc-pair equality constraints",
        fields: IntegralFlowHomologousArcsCreateSpec::FIELDS,
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
    capacities: Vec<i64>,
    source: usize,
    sink: usize,
    requirement: i64,
    homologous_pairs: Vec<(usize, usize)>,
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct IntegralFlowHomologousArcsCreateSpec {
    #[create(codec = "arc-list")]
    arcs: Vec<(usize, usize)>,
    num_vertices: Option<usize>,
    #[create(codec = "comma-separated")]
    capacities: Option<Vec<i64>>,
    source: usize,
    sink: usize,
    requirement: i64,
    #[create(codec = "equality-pair-list")]
    homologous_pairs: Vec<(usize, usize)>,
}

impl TryFrom<IntegralFlowHomologousArcsCreateSpec> for IntegralFlowHomologousArcs {
    type Error = crate::registry::ConstructionError;
    fn try_from(
        spec: IntegralFlowHomologousArcsCreateSpec,
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
        let capacities = spec.capacities.unwrap_or_else(|| vec![1; spec.arcs.len()]);
        if capacities.len() != spec.arcs.len() {
            return Err("capacities length must match arcs length".into());
        }
        if spec.source >= count || spec.sink >= count {
            return Err("source and sink must be valid vertices".into());
        }
        for &(a, b) in &spec.homologous_pairs {
            if a >= spec.arcs.len() || b >= spec.arcs.len() {
                return Err("homologous pair arc index is out of range".into());
            }
        }
        for &c in &capacities {
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
            capacities,
            source: spec.source,
            sink: spec.sink,
            requirement: spec.requirement,
            homologous_pairs: spec.homologous_pairs,
        })
    }
}

impl IntegralFlowHomologousArcs {
    pub fn new(
        graph: DirectedGraph,
        capacities: Vec<i64>,
        source: usize,
        sink: usize,
        requirement: i64,
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

    pub fn capacities(&self) -> &[i64] {
        &self.capacities
    }

    pub fn source(&self) -> usize {
        self.source
    }

    pub fn sink(&self) -> usize {
        self.sink
    }

    pub fn requirement(&self) -> i64 {
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

    pub fn max_capacity(&self) -> i64 {
        self.capacities.iter().copied().max().unwrap_or(0)
    }

    pub fn is_valid_solution(
        &self,
        config: &[usize],
    ) -> Result<bool, crate::traits::EvaluationError> {
        Ok(self.evaluate_solution(config)?.0)
    }

    fn evaluate_solution(
        &self,
        config: &[usize],
    ) -> Result<crate::types::Or, crate::traits::EvaluationError> {
        if config.len() != self.num_arcs() {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "flow vector length does not match the graph arcs".into(),
            ));
        }

        for &(a, b) in &self.homologous_pairs {
            if config[a] != config[b] {
                return Ok(crate::types::Or(false));
            }
        }

        let mut balances = vec![0_i64; self.num_vertices()];
        for (arc_index, ((u, v), &capacity)) in self
            .graph
            .arcs()
            .into_iter()
            .zip(self.capacities.iter())
            .enumerate()
        {
            let Ok(flow) = i64::try_from(config[arc_index]) else {
                return Ok(crate::types::Or(false));
            };
            if flow > capacity {
                return Ok(crate::types::Or(false));
            }
            balances[u] = balances[u].checked_sub(flow).ok_or_else(|| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "subtracting outgoing homologous-arc flow".into(),
                )
            })?;
            balances[v] = balances[v].checked_add(flow).ok_or_else(|| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "adding incoming homologous-arc flow".into(),
                )
            })?;
        }

        for (vertex, &balance) in balances.iter().enumerate() {
            if vertex != self.source && vertex != self.sink && balance != 0 {
                return Ok(crate::types::Or(false));
            }
        }

        Ok(crate::types::Or(balances[self.sink] >= self.requirement))
    }

    fn domain_size(capacity: i64) -> usize {
        usize::try_from(capacity)
            .ok()
            .and_then(|value| value.checked_add(1))
            .expect("capacity already validated to fit into usize")
    }
}

impl Problem for IntegralFlowHomologousArcs {
    const NAME: &'static str = "IntegralFlowHomologousArcs";
    type Solution = Vec<usize>;
    type Value = crate::types::Or;

    crate::problem_parameters![
        ("max_capacity", max_capacity),
        ("num_arcs", num_arcs),
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

impl crate::solvers::BruteForceProblem for IntegralFlowHomologousArcs {
    fn dimensions(&self) -> Vec<usize> {
        self.capacities
            .iter()
            .map(|&capacity| Self::domain_size(capacity))
            .collect()
    }
}

crate::declare_variants! {
    default IntegralFlowHomologousArcs => "(max_capacity + 1)^num_arcs" create IntegralFlowHomologousArcsCreateSpec,
}

crate::register_brute_force! {
    IntegralFlowHomologousArcs,
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
        optimal_config: serde_json::json!(vec![1, 1, 1, 0, 0, 1, 1, 1]),
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/integral_flow_homologous_arcs.rs"]
mod tests;
