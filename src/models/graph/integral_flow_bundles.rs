//! Integral Flow with Bundles problem implementation.
//!
//! Given a directed graph with overlapping bundle-capacity constraints on arcs,
//! determine whether an integral flow can deliver a required amount to the sink.

use crate::registry::{CreateSpec, ProblemSchemaEntry, ProblemSizeFieldEntry};
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

inventory::submit! {
    ProblemSizeFieldEntry {
        name: "IntegralFlowBundles",
        fields: &["num_vertices", "num_arcs", "num_bundles"],
    }
}

/// Integral Flow with Bundles (Garey & Johnson ND36).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegralFlowBundles {
    graph: DirectedGraph,
    source: usize,
    sink: usize,
    bundles: Vec<Vec<usize>>,
    bundle_capacities: Vec<i64>,
    requirement: i64,
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
        if spec.source >= count || spec.sink >= count {
            return Err("source and sink must be valid vertices".into());
        }
        if spec.source == spec.sink {
            return Err("source and sink must be distinct".into());
        }
        if spec.bundles.len() != spec.bundle_capacities.len() {
            return Err("bundles length must match bundle_capacities length".into());
        }
        if spec.requirement == 0 {
            return Err("requirement must be positive".into());
        }
        let mut covered = vec![false; spec.arcs.len()];
        let mut upper = vec![i64::MAX; spec.arcs.len()];
        for (i, (bundle, &capacity)) in spec.bundles.iter().zip(&spec.bundle_capacities).enumerate()
        {
            if capacity == 0 {
                return Err(format!("bundle capacity {i} must be positive").into());
            }
            let mut seen = BTreeSet::new();
            for &arc in bundle {
                if arc >= spec.arcs.len() {
                    return Err(format!("bundle {i} arc is out of range").into());
                }
                if !seen.insert(arc) {
                    return Err(format!("bundle {i} contains duplicate arc").into());
                }
                covered[arc] = true;
                upper[arc] = upper[arc].min(capacity);
            }
        }
        for (arc, &is_covered) in covered.iter().enumerate() {
            if !is_covered {
                return Err(format!("arc {arc} must belong to a bundle").into());
            }
            if usize::try_from(upper[arc])
                .ok()
                .and_then(|v| v.checked_add(1))
                .is_none()
            {
                return Err(format!("arc {arc} upper bound is too large").into());
            }
        }
        Ok(Self {
            graph: DirectedGraph::new(count, spec.arcs),
            source: spec.source,
            sink: spec.sink,
            bundles: spec.bundles,
            bundle_capacities: spec.bundle_capacities,
            requirement: spec.requirement,
        })
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
        let num_vertices = graph.num_vertices();
        let num_arcs = graph.num_arcs();

        assert!(
            source < num_vertices,
            "source ({source}) >= num_vertices ({num_vertices})"
        );
        assert!(
            sink < num_vertices,
            "sink ({sink}) >= num_vertices ({num_vertices})"
        );
        assert!(source != sink, "source and sink must be distinct");
        assert_eq!(
            bundles.len(),
            bundle_capacities.len(),
            "bundles length must match bundle_capacities length"
        );
        assert!(requirement > 0, "requirement must be positive");

        let mut arc_covered = vec![false; num_arcs];
        let mut arc_upper_bounds = vec![i64::MAX; num_arcs];

        for (bundle_index, (bundle, &capacity)) in
            bundles.iter().zip(&bundle_capacities).enumerate()
        {
            assert!(
                capacity > 0,
                "bundle capacity at index {bundle_index} must be positive"
            );

            let mut seen = BTreeSet::new();
            for &arc_index in bundle {
                assert!(
                    arc_index < num_arcs,
                    "bundle {bundle_index} references arc {arc_index}, but num_arcs is {num_arcs}"
                );
                assert!(
                    seen.insert(arc_index),
                    "bundle {bundle_index} contains duplicate arc index {arc_index}"
                );
                arc_covered[arc_index] = true;
                arc_upper_bounds[arc_index] = arc_upper_bounds[arc_index].min(capacity);
            }
        }

        for (arc_index, covered) in arc_covered.iter().copied().enumerate() {
            assert!(
                covered,
                "arc {arc_index} must belong to at least one bundle"
            );
            let domain = usize::try_from(arc_upper_bounds[arc_index])
                .ok()
                .and_then(|bound| bound.checked_add(1));
            assert!(
                domain.is_some(),
                "bundle-derived upper bound for arc {arc_index} must fit into usize for dims()"
            );
        }

        Self {
            graph,
            source,
            sink,
            bundles,
            bundle_capacities,
            requirement,
        }
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
        Ok(self.evaluate(config)?.0)
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
}

impl Problem for IntegralFlowBundles {
    const NAME: &'static str = "IntegralFlowBundles";
    type Value = crate::types::Or;

    fn dims(&self) -> Vec<usize> {
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

    fn evaluate(
        &self,
        config: &[usize],
    ) -> Result<crate::types::Or, crate::traits::EvaluationError> {
        Ok({
            crate::types::Or({
                if config.len() != self.num_arcs() {
                    return Ok(crate::types::Or(false));
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

                matches!(
                    self.vertex_balance(config, self.sink)?,
                    Some(balance) if balance >= self.requirement
                )
            })
        })
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

crate::declare_variants! {
    default IntegralFlowBundles => "2^num_arcs" create IntegralFlowBundlesCreateSpec,
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
        optimal_config: vec![1, 0, 1, 0, 0, 0],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/integral_flow_bundles.rs"]
mod tests;
