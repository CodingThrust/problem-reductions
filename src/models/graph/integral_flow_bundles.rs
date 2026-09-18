//! Integral Flow with Bundles problem implementation.
//!
//! Given a directed graph with overlapping bundle-capacity constraints on arcs,
//! determine whether an integral flow can deliver a required amount to the sink.

use crate::registry::{CreateSpec, ProblemSchemaEntry};
use crate::topology::DirectedGraph;
use crate::traits::Problem;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

inventory::submit! {
    ProblemSchemaEntry {
        name: "IntegralFlowBundles",
        display_name: "Integral Flow with Bundles",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Integral flow feasibility on a directed graph with overlapping bundle capacities",
        fields: IntegralFlowBundlesCreateSpec::FIELDS,
    }
}

/// Integral Flow with Bundles (Garey & Johnson ND36).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "IntegralFlowBundlesData")]
pub struct IntegralFlowBundles {
    graph: DirectedGraph,
    source: usize,
    sink: usize,
    bundles: Vec<Vec<usize>>,
    bundle_capacities: Vec<i64>,
    requirement: i64,
}

#[derive(Deserialize)]
struct IntegralFlowBundlesData {
    graph: DirectedGraph,
    source: usize,
    sink: usize,
    bundles: Vec<Vec<usize>>,
    bundle_capacities: Vec<i64>,
    requirement: i64,
}

impl TryFrom<IntegralFlowBundlesData> for IntegralFlowBundles {
    type Error = crate::registry::ConstructionError;
    fn try_from(data: IntegralFlowBundlesData) -> Result<Self, Self::Error> {
        Self::try_new(
            data.graph,
            data.source,
            data.sink,
            data.bundles,
            data.bundle_capacities,
            data.requirement,
        )
    }
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct IntegralFlowBundlesCreateSpec {
    #[create(codec = "arc-list")]
    arcs: Vec<(usize, usize)>,
    num_vertices: Option<usize>,
    #[create(codec = "semicolon-separated")]
    bundles: Vec<Vec<usize>>,
    #[create(codec = "comma-separated")]
    bundle_capacities: Vec<i64>,
    source: usize,
    sink: usize,
    requirement: i64,
}

impl TryFrom<IntegralFlowBundlesCreateSpec> for IntegralFlowBundles {
    type Error = crate::registry::ConstructionError;
    fn try_from(
        spec: IntegralFlowBundlesCreateSpec,
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
        Self::try_new(
            DirectedGraph::new(count, spec.arcs),
            spec.source,
            spec.sink,
            spec.bundles,
            spec.bundle_capacities,
            spec.requirement,
        )
    }
}

impl IntegralFlowBundles {
    /// Create a new Integral Flow with Bundles instance.
    pub fn new(
        graph: DirectedGraph,
        source: usize,
        sink: usize,
        bundles: Vec<Vec<usize>>,
        bundle_capacities: Vec<i64>,
        requirement: i64,
    ) -> Self {
        Self::try_new(graph, source, sink, bundles, bundle_capacities, requirement)
            .unwrap_or_else(|error| panic!("{error}"))
    }

    fn try_new(
        graph: DirectedGraph,
        source: usize,
        sink: usize,
        bundles: Vec<Vec<usize>>,
        bundle_capacities: Vec<i64>,
        requirement: i64,
    ) -> Result<Self, crate::registry::ConstructionError> {
        let num_vertices = graph.num_vertices();
        let num_arcs = graph.num_arcs();

        if source >= num_vertices {
            return Err(format!("source ({source}) >= num_vertices ({num_vertices})").into());
        }
        if sink >= num_vertices {
            return Err(format!("sink ({sink}) >= num_vertices ({num_vertices})").into());
        }
        if source == sink {
            return Err("source and sink must be distinct".into());
        }
        if bundles.len() != bundle_capacities.len() {
            return Err("bundles length must match bundle_capacities length".into());
        }
        if requirement <= 0 {
            return Err("requirement must be positive".into());
        }

        let mut arc_covered = vec![false; num_arcs];
        let mut arc_upper_bounds = vec![i64::MAX; num_arcs];

        for (bundle_index, (bundle, &capacity)) in
            bundles.iter().zip(&bundle_capacities).enumerate()
        {
            if capacity <= 0 {
                return Err(
                    format!("bundle capacity at index {bundle_index} must be positive").into(),
                );
            }

            let mut seen = BTreeSet::new();
            for &arc_index in bundle {
                if arc_index >= num_arcs {
                    return Err(format!("bundle {bundle_index} arc is out of range: index {arc_index}, num_arcs {num_arcs}").into());
                }
                if !(seen.insert(arc_index)) {
                    return Err(format!(
                        "bundle {bundle_index} contains duplicate arc index {arc_index}"
                    )
                    .into());
                }
                arc_covered[arc_index] = true;
                arc_upper_bounds[arc_index] = arc_upper_bounds[arc_index].min(capacity);
            }
        }

        for (arc_index, covered) in arc_covered.iter().copied().enumerate() {
            if !(covered) {
                return Err(format!("arc {arc_index} must belong to at least one bundle").into());
            }
            if usize::try_from(arc_upper_bounds[arc_index])
                .ok()
                .and_then(|bound| bound.checked_add(1))
                .is_none()
            {
                return Err(format!("bundle-derived upper bound for arc {arc_index} must fit into usize for dimensions()").into());
            }
        }

        Ok(Self {
            graph,
            source,
            sink,
            bundles,
            bundle_capacities,
            requirement,
        })
    }

    /// Get the underlying directed graph.
    pub fn graph(&self) -> &DirectedGraph {
        &self.graph
    }

    /// Get the source vertex.
    pub fn source(&self) -> usize {
        self.source
    }

    /// Get the sink vertex.
    pub fn sink(&self) -> usize {
        self.sink
    }

    /// Get the bundles.
    pub fn bundles(&self) -> &[Vec<usize>] {
        &self.bundles
    }

    /// Get the bundle capacities.
    pub fn bundle_capacities(&self) -> &[i64] {
        &self.bundle_capacities
    }

    /// Get the required net inflow at the sink.
    pub fn requirement(&self) -> i64 {
        self.requirement
    }

    /// Get the number of vertices.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of arcs.
    pub fn num_arcs(&self) -> usize {
        self.graph.num_arcs()
    }

    /// Get the number of bundles.
    pub fn num_bundles(&self) -> usize {
        self.bundles.len()
    }

    /// Check whether a configuration is feasible.
    pub fn is_valid_solution(
        &self,
        config: &[usize],
    ) -> Result<bool, crate::traits::EvaluationError> {
        Ok(self.evaluate_solution(config)?.0)
    }

    fn arc_upper_bounds(&self) -> Vec<i64> {
        let mut upper_bounds = vec![i64::MAX; self.num_arcs()];
        for (bundle, &capacity) in self.bundles.iter().zip(&self.bundle_capacities) {
            for &arc_index in bundle {
                upper_bounds[arc_index] = upper_bounds[arc_index].min(capacity);
            }
        }
        upper_bounds
    }

    fn vertex_balance(
        &self,
        config: &[usize],
        vertex: usize,
    ) -> Result<Option<i64>, crate::traits::EvaluationError> {
        let mut balance = 0_i64;
        for (arc_index, (u, v)) in self.graph.arcs().into_iter().enumerate() {
            let Some(&raw_flow) = config.get(arc_index) else {
                return Ok(None);
            };
            let flow = i64::try_from(raw_flow).map_err(|_| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "converting bundled arc flow to i64".into(),
                )
            })?;
            if vertex == u {
                balance = balance.checked_sub(flow).ok_or_else(|| {
                    crate::traits::EvaluationError::IntegerOverflow(
                        "subtracting outgoing bundled flow".into(),
                    )
                })?;
            }
            if vertex == v {
                balance = balance.checked_add(flow).ok_or_else(|| {
                    crate::traits::EvaluationError::IntegerOverflow(
                        "adding incoming bundled flow".into(),
                    )
                })?;
            }
        }
        Ok(Some(balance))
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

        let upper_bounds = self.arc_upper_bounds();
        for (&value, &upper_bound) in config.iter().zip(&upper_bounds) {
            if i64::try_from(value).map_or(true, |value| value > upper_bound) {
                return Ok(crate::types::Or(false));
            }
        }

        for (bundle, &capacity) in self.bundles.iter().zip(&self.bundle_capacities) {
            let mut total = 0i64;
            for &arc_index in bundle {
                let Ok(flow) = i64::try_from(config[arc_index]) else {
                    return Ok(crate::types::Or(false));
                };
                let Some(next_total) = total.checked_add(flow) else {
                    return Ok(crate::types::Or(false));
                };
                total = next_total;
            }
            if total > capacity {
                return Ok(crate::types::Or(false));
            }
        }

        for vertex in 0..self.num_vertices() {
            if vertex == self.source || vertex == self.sink {
                continue;
            }
            if self.vertex_balance(config, vertex)? != Some(0) {
                return Ok(crate::types::Or(false));
            }
        }

        Ok(crate::types::Or(matches!(
            self.vertex_balance(config, self.sink)?,
            Some(balance) if balance >= self.requirement
        )))
    }
}

impl Problem for IntegralFlowBundles {
    const NAME: &'static str = "IntegralFlowBundles";
    type Solution = Vec<usize>;
    type Value = crate::types::Or;

    crate::problem_parameters![
        ("num_arcs", num_arcs),
        ("num_bundles", num_bundles),
        ("num_vertices", num_vertices),
    ];

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<crate::types::Or, crate::traits::EvaluationError> {
        self.evaluate_solution(config)
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

impl crate::solvers::BruteForceProblem for IntegralFlowBundles {
    fn dimensions(&self) -> Vec<usize> {
        self.arc_upper_bounds()
            .into_iter()
            .map(|bound| {
                usize::try_from(bound)
                    .ok()
                    .and_then(|bound| bound.checked_add(1))
                    .expect("bundle-derived arc upper bounds are validated in the constructor")
            })
            .collect()
    }
}

crate::declare_variants! {
    default IntegralFlowBundles => "2^num_arcs" create IntegralFlowBundlesCreateSpec,
}

crate::register_brute_force! {
    IntegralFlowBundles,
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "integral_flow_bundles",
        instance: Box::new(IntegralFlowBundles::new(
            DirectedGraph::new(4, vec![(0, 1), (0, 2), (1, 3), (2, 3), (1, 2), (2, 1)]),
            0,
            3,
            vec![vec![0, 1], vec![2, 5], vec![3, 4]],
            vec![1, 1, 1],
            1,
        )),
        optimal_config: serde_json::json!(vec![1, 0, 1, 0, 0, 0]),
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/integral_flow_bundles.rs"]
mod tests;
