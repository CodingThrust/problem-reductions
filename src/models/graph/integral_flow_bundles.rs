//! Integral Flow with Bundles problem implementation.
//!
//! Given a directed graph with overlapping bundle-capacity constraints on arcs,
//! determine whether an integral flow can deliver a required amount to the sink.

use crate::registry::{FieldInfo, ProblemSchemaEntry, ProblemSizeFieldEntry};
use crate::topology::DirectedGraph;
use crate::traits::{Problem, SatisfactionProblem};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

inventory::submit! {
    ProblemSchemaEntry {
        name: "IntegralFlowBundles",
        display_name: "Integral Flow with Bundles",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Integral flow feasibility on a directed graph with overlapping bundle capacities",
        fields: &[
            FieldInfo { name: "graph", type_name: "DirectedGraph", description: "Directed graph G=(V,A)" },
            FieldInfo { name: "source", type_name: "usize", description: "Source vertex s" },
            FieldInfo { name: "sink", type_name: "usize", description: "Sink vertex t" },
            FieldInfo { name: "bundles", type_name: "Vec<Vec<usize>>", description: "Bundles of arc indices covering A" },
            FieldInfo { name: "bundle_capacities", type_name: "Vec<u64>", description: "Capacity c_j for each bundle I_j" },
            FieldInfo { name: "requirement", type_name: "u64", description: "Required net inflow R at the sink" },
        ],
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
    bundle_capacities: Vec<u64>,
    requirement: u64,
}

impl IntegralFlowBundles {
    /// Create a new Integral Flow with Bundles instance.
    pub fn new(
        graph: DirectedGraph,
        source: usize,
        sink: usize,
        bundles: Vec<Vec<usize>>,
        bundle_capacities: Vec<u64>,
        requirement: u64,
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
        let mut arc_upper_bounds = vec![u64::MAX; num_arcs];

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
    pub fn bundle_capacities(&self) -> &[u64] {
        &self.bundle_capacities
    }

    /// Get the required net inflow at the sink.
    pub fn requirement(&self) -> u64 {
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
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        self.evaluate(config)
    }

    fn arc_upper_bounds(&self) -> Vec<u64> {
        let mut upper_bounds = vec![u64::MAX; self.num_arcs()];
        for (bundle, &capacity) in self.bundles.iter().zip(&self.bundle_capacities) {
            for &arc_index in bundle {
                upper_bounds[arc_index] = upper_bounds[arc_index].min(capacity);
            }
        }
        upper_bounds
    }

    fn vertex_balance(&self, config: &[usize], vertex: usize) -> Option<i128> {
        let mut balance = 0i128;
        for (arc_index, (u, v)) in self.graph.arcs().into_iter().enumerate() {
            let flow = i128::from(u64::try_from(*config.get(arc_index)?).ok()?);
            if vertex == u {
                balance -= flow;
            }
            if vertex == v {
                balance += flow;
            }
        }
        Some(balance)
    }
}

impl Problem for IntegralFlowBundles {
    const NAME: &'static str = "IntegralFlowBundles";
    type Value = bool;

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

    fn evaluate(&self, config: &[usize]) -> bool {
        if config.len() != self.num_arcs() {
            return false;
        }

        let upper_bounds = self.arc_upper_bounds();
        for (&value, &upper_bound) in config.iter().zip(&upper_bounds) {
            if u64::try_from(value).map_or(true, |value| value > upper_bound) {
                return false;
            }
        }

        for (bundle, &capacity) in self.bundles.iter().zip(&self.bundle_capacities) {
            let mut total = 0u64;
            for &arc_index in bundle {
                let Ok(flow) = u64::try_from(config[arc_index]) else {
                    return false;
                };
                let Some(next_total) = total.checked_add(flow) else {
                    return false;
                };
                total = next_total;
            }
            if total > capacity {
                return false;
            }
        }

        for vertex in 0..self.num_vertices() {
            if vertex == self.source || vertex == self.sink {
                continue;
            }
            if self.vertex_balance(config, vertex) != Some(0) {
                return false;
            }
        }

        matches!(
            self.vertex_balance(config, self.sink),
            Some(balance) if balance >= i128::from(self.requirement)
        )
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

impl SatisfactionProblem for IntegralFlowBundles {}

crate::declare_variants! {
    default sat IntegralFlowBundles => "2^num_arcs",
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
