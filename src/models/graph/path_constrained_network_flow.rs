//! Path-Constrained Network Flow problem implementation.
//!
//! Given a directed graph with arc capacities, a designated source and sink,
//! and a prescribed collection of directed s-t paths, determine whether there
//! exists an integral amount of flow for each prescribed path such that arc
//! capacities are respected and the total delivered flow reaches the required
//! threshold.

use crate::registry::{CreateSpec, ProblemSchemaEntry};
use crate::topology::DirectedGraph;
use crate::traits::Problem;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

inventory::submit! {
    ProblemSchemaEntry {
        name: "PathConstrainedNetworkFlow",
        display_name: "Path-Constrained Network Flow",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Integral flow feasibility on a prescribed collection of directed s-t paths",
        fields: PathConstrainedNetworkFlowCreateSpec::FIELDS,
    }
}

/// Path-Constrained Network Flow.
///
/// A configuration contains one integer variable per prescribed path. If
/// `config[i] = x`, then `x` units of flow are routed along the i-th prescribed
/// path. A configuration is feasible when:
/// - each path variable stays within its bottleneck capacity
/// - the induced arc loads do not exceed the arc capacities
/// - the total delivered flow reaches the requirement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathConstrainedNetworkFlow {
    graph: DirectedGraph,
    capacities: Vec<i64>,
    source: usize,
    sink: usize,
    paths: Vec<Vec<usize>>,
    requirement: i64,
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct PathConstrainedNetworkFlowCreateSpec {
    /// Directed graph arcs.
    #[create(codec = "arc-list")]
    arcs: Vec<(usize, usize)>,
    /// Vertex count, needed to preserve isolated vertices.
    num_vertices: Option<usize>,
    /// Arc capacities; defaults to one per arc.
    #[create(codec = "comma-separated")]
    capacities: Option<Vec<i64>>,
    /// Source vertex.
    source: usize,
    /// Sink vertex.
    sink: usize,
    /// Prescribed paths as arc-index sequences.
    #[create(codec = "semicolon-separated")]
    paths: Vec<Vec<usize>>,
    /// Required total flow.
    requirement: i64,
}

impl TryFrom<PathConstrainedNetworkFlowCreateSpec> for PathConstrainedNetworkFlow {
    type Error = crate::registry::ConstructionError;

    fn try_from(spec: PathConstrainedNetworkFlowCreateSpec) -> Result<Self, Self::Error> {
        if spec.arcs.is_empty() {
            return Err("arcs must be non-empty".to_string().into());
        }
        if spec.paths.is_empty() {
            return Err("paths must be non-empty".to_string().into());
        }
        let inferred = spec
            .arcs
            .iter()
            .flat_map(|&(u, v)| [u, v])
            .max()
            .map(|vertex| vertex.checked_add(1).ok_or("vertex count overflows usize"))
            .transpose()?
            .unwrap_or(0);
        let num_vertices = spec.num_vertices.unwrap_or(inferred);
        if num_vertices < inferred {
            return Err(format!(
                "num_vertices {num_vertices} is too small for arc endpoints; need at least {inferred}"
            ).into());
        }
        let capacities = spec.capacities.unwrap_or_else(|| vec![1; spec.arcs.len()]);
        let graph = DirectedGraph::new(num_vertices, spec.arcs);
        Self::try_new(
            graph,
            capacities,
            spec.source,
            spec.sink,
            spec.paths,
            spec.requirement,
        )
    }
}

impl PathConstrainedNetworkFlow {
    /// Create a new Path-Constrained Network Flow instance.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - `capacities.len() != graph.num_arcs()`
    /// - `source` or `sink` are out of range or identical
    /// - any prescribed path is not a valid directed simple s-t path
    pub fn new(
        graph: DirectedGraph,
        capacities: Vec<i64>,
        source: usize,
        sink: usize,
        paths: Vec<Vec<usize>>,
        requirement: i64,
    ) -> Self {
        Self::try_new(graph, capacities, source, sink, paths, requirement)
            .unwrap_or_else(|message| panic!("{message}"))
    }

    /// Create an instance, returning validation errors instead of panicking.
    pub fn try_new(
        graph: DirectedGraph,
        capacities: Vec<i64>,
        source: usize,
        sink: usize,
        paths: Vec<Vec<usize>>,
        requirement: i64,
    ) -> Result<Self, crate::registry::ConstructionError> {
        let num_vertices = graph.num_vertices();
        if capacities.len() != graph.num_arcs() {
            return Err("capacities length must match graph num_arcs"
                .to_string()
                .into());
        }
        if source >= num_vertices {
            return Err(format!("source ({source}) >= num_vertices ({num_vertices})").into());
        }
        if sink >= num_vertices {
            return Err(format!("sink ({sink}) >= num_vertices ({num_vertices})").into());
        }
        if source == sink {
            return Err("source and sink must be distinct".to_string().into());
        }

        for (index, path) in paths.iter().enumerate() {
            Self::validate_path(&graph, path, source, sink)
                .map_err(|message| format!("path {index}: {message}"))?;
        }

        Ok(Self {
            graph,
            capacities,
            source,
            sink,
            paths,
            requirement,
        })
    }

    fn validate_path(
        graph: &DirectedGraph,
        path: &[usize],
        source: usize,
        sink: usize,
    ) -> Result<(), crate::registry::ConstructionError> {
        if path.is_empty() {
            return Err("prescribed paths must be non-empty".to_string().into());
        }

        let arcs = graph.arcs();
        let mut visited_vertices = HashSet::from([source]);
        let mut current = source;

        for &arc_idx in path {
            let &(tail, head) = arcs
                .get(arc_idx)
                .ok_or_else(|| format!("arc index {arc_idx} out of bounds"))?;
            if tail != current {
                return Err(format!(
                    "not contiguous: expected arc leaving vertex {current}, got {tail}->{head}"
                )
                .into());
            }
            if !visited_vertices.insert(head) {
                return Err(format!("repeats vertex {head}, so it is not a simple path").into());
            }
            current = head;
        }
        if current != sink {
            return Err(format!("must end at sink {sink}, ended at {current}").into());
        }
        Ok(())
    }

    fn path_bottleneck(&self, path: &[usize]) -> i64 {
        path.iter()
            .map(|&arc_idx| self.capacities[arc_idx])
            .min()
            .unwrap_or(0)
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &DirectedGraph {
        &self.graph
    }

    /// Get the arc capacities.
    pub fn capacities(&self) -> &[i64] {
        &self.capacities
    }

    /// Get the prescribed path collection.
    pub fn paths(&self) -> &[Vec<usize>] {
        &self.paths
    }

    /// Get the source vertex.
    pub fn source(&self) -> usize {
        self.source
    }

    /// Get the sink vertex.
    pub fn sink(&self) -> usize {
        self.sink
    }

    /// Get the required total flow.
    pub fn requirement(&self) -> i64 {
        self.requirement
    }

    /// Update the required total flow.
    pub fn set_requirement(&mut self, requirement: i64) {
        self.requirement = requirement;
    }

    /// Get the number of vertices.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of arcs.
    pub fn num_arcs(&self) -> usize {
        self.graph.num_arcs()
    }

    /// Get the number of prescribed paths.
    pub fn num_paths(&self) -> usize {
        self.paths.len()
    }

    /// Get the maximum arc capacity.
    pub fn max_capacity(&self) -> i64 {
        self.capacities.iter().copied().max().unwrap_or(0)
    }

    /// Check whether a path-flow assignment is feasible.
    pub fn is_feasible(&self, config: &[usize]) -> Result<bool, crate::traits::EvaluationError> {
        if config.len() != self.paths.len() {
            return Ok(false);
        }

        let mut arc_loads = vec![0_i64; self.capacities.len()];
        let mut total_flow = 0_i64;

        for (flow_value, path) in config.iter().copied().zip(&self.paths) {
            let path_flow = i64::try_from(flow_value).map_err(|_| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "converting path flow to i64".into(),
                )
            })?;
            if path_flow > self.path_bottleneck(path) {
                return Ok(false);
            }

            total_flow = total_flow.checked_add(path_flow).ok_or_else(|| {
                crate::traits::EvaluationError::IntegerOverflow("summing total path flow".into())
            })?;
            for &arc_idx in path {
                arc_loads[arc_idx] =
                    arc_loads[arc_idx].checked_add(path_flow).ok_or_else(|| {
                        crate::traits::EvaluationError::IntegerOverflow(
                            "summing path flow on an arc".into(),
                        )
                    })?;
                if arc_loads[arc_idx] > self.capacities[arc_idx] {
                    return Ok(false);
                }
            }
        }

        Ok(total_flow >= self.requirement)
    }
}

impl Problem for PathConstrainedNetworkFlow {
    const NAME: &'static str = "PathConstrainedNetworkFlow";
    type Solution = Vec<usize>;
    type Value = crate::types::Or;

    crate::problem_size![
        ("max_capacity", max_capacity),
        ("num_arcs", num_arcs),
        ("num_paths", num_paths),
    ];

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<crate::types::Or, crate::traits::EvaluationError> {
        if config.len() != self.paths.len() {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "path-flow vector length does not match the candidate paths".into(),
            ));
        }
        Ok(crate::types::Or(self.is_feasible(config)?))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

impl crate::solvers::BruteForceProblem for PathConstrainedNetworkFlow {
    fn dimensions(&self) -> Vec<usize> {
        self.paths
            .iter()
            .map(|path| (self.path_bottleneck(path) as usize) + 1)
            .collect()
    }
}

crate::declare_variants! {
    default PathConstrainedNetworkFlow => "(max_capacity + 1)^num_paths" create PathConstrainedNetworkFlowCreateSpec,
}

crate::register_brute_force! {
    PathConstrainedNetworkFlow,
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "path_constrained_network_flow",
        instance: Box::new(PathConstrainedNetworkFlow::new(
            DirectedGraph::new(
                8,
                vec![
                    (0, 1),
                    (0, 2),
                    (1, 3),
                    (1, 4),
                    (2, 4),
                    (3, 5),
                    (4, 5),
                    (4, 6),
                    (5, 7),
                    (6, 7),
                ],
            ),
            vec![2, 1, 1, 1, 1, 1, 1, 1, 2, 1],
            0,
            7,
            vec![
                vec![0, 2, 5, 8],
                vec![0, 3, 6, 8],
                vec![0, 3, 7, 9],
                vec![1, 4, 6, 8],
                vec![1, 4, 7, 9],
            ],
            3,
        )),
        optimal_config: serde_json::json!(vec![1, 1, 0, 0, 1]),
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/path_constrained_network_flow.rs"]
mod tests;
