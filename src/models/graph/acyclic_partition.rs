//! Acyclic Partition problem implementation.
//!
//! Given a directed graph with vertex weights, arc costs, and bounds, determine
//! whether the vertices can be partitioned into groups whose quotient graph is a
//! DAG, each group's total vertex weight is bounded, and the total
//! inter-partition arc cost is bounded.

use crate::registry::{FieldInfo, ProblemSchemaEntry, ProblemSizeFieldEntry, VariantDimension};
use crate::topology::DirectedGraph;
use crate::traits::Problem;
use crate::types::WeightElement;
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

inventory::submit! {
    ProblemSchemaEntry {
        name: "AcyclicPartition",
        display_name: "Acyclic Partition",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("weight", "i32", &["i32"]),
        ],
        module_path: module_path!(),
        description: "Partition a directed graph into bounded-weight groups with an acyclic quotient graph and bounded inter-partition cost",
        fields: &[
            FieldInfo { name: "graph", type_name: "DirectedGraph", description: "The directed graph G=(V,A)" },
            FieldInfo { name: "vertex_weights", type_name: "Vec<W>", description: "Vertex weights w(v) for each vertex v in V" },
            FieldInfo { name: "arc_costs", type_name: "Vec<W>", description: "Arc costs c(a) for each arc a in A, matching graph.arcs() order" },
            FieldInfo { name: "weight_bound", type_name: "W::Sum", description: "Maximum total vertex weight B for each partition" },
            FieldInfo { name: "cost_bound", type_name: "W::Sum", description: "Maximum total inter-partition arc cost K" },
        ],
    }
}

inventory::submit! {
    ProblemSizeFieldEntry {
        name: "AcyclicPartition",
        fields: &["num_vertices", "num_arcs"],
    }
}

/// Acyclic Partition (Garey & Johnson ND15).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcyclicPartition<W: WeightElement> {
    graph: DirectedGraph,
    vertex_weights: Vec<W>,
    arc_costs: Vec<W>,
    weight_bound: W::Sum,
    cost_bound: W::Sum,
}

impl<W: WeightElement> AcyclicPartition<W> {
    /// Create a new Acyclic Partition instance.
    pub fn new(
        graph: DirectedGraph,
        vertex_weights: Vec<W>,
        arc_costs: Vec<W>,
        weight_bound: W::Sum,
        cost_bound: W::Sum,
    ) -> Self {
        assert_eq!(
            vertex_weights.len(),
            graph.num_vertices(),
            "vertex_weights length must match graph num_vertices"
        );
        assert_eq!(
            arc_costs.len(),
            graph.num_arcs(),
            "arc_costs length must match graph num_arcs"
        );
        Self {
            graph,
            vertex_weights,
            arc_costs,
            weight_bound,
            cost_bound,
        }
    }

    /// Get the underlying graph.
    pub fn graph(&self) -> &DirectedGraph {
        &self.graph
    }

    /// Get the vertex weights.
    pub fn vertex_weights(&self) -> &[W] {
        &self.vertex_weights
    }

    /// Get the arc costs.
    pub fn arc_costs(&self) -> &[W] {
        &self.arc_costs
    }

    /// Replace the vertex weights.
    pub fn set_vertex_weights(&mut self, vertex_weights: Vec<W>) {
        assert_eq!(
            vertex_weights.len(),
            self.graph.num_vertices(),
            "vertex_weights length must match graph num_vertices"
        );
        self.vertex_weights = vertex_weights;
    }

    /// Replace the arc costs.
    pub fn set_arc_costs(&mut self, arc_costs: Vec<W>) {
        assert_eq!(
            arc_costs.len(),
            self.graph.num_arcs(),
            "arc_costs length must match graph num_arcs"
        );
        self.arc_costs = arc_costs;
    }

    /// Get the per-part weight bound.
    pub fn weight_bound(&self) -> &W::Sum {
        &self.weight_bound
    }

    /// Get the inter-partition cost bound.
    pub fn cost_bound(&self) -> &W::Sum {
        &self.cost_bound
    }

    /// Check whether this instance uses non-unit weights.
    pub fn is_weighted(&self) -> bool {
        !W::IS_UNIT
    }

    /// Get the number of vertices.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of arcs.
    pub fn num_arcs(&self) -> usize {
        self.graph.num_arcs()
    }

    /// Check whether a configuration is a valid solution.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        is_valid_acyclic_partition(
            &self.graph,
            &self.vertex_weights,
            &self.arc_costs,
            &self.weight_bound,
            &self.cost_bound,
            config,
        )
    }
}

impl<W> Problem for AcyclicPartition<W>
where
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "AcyclicPartition";
    type Value = crate::types::Or;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![W]
    }

    fn dims(&self) -> Vec<usize> {
        vec![self.graph.num_vertices(); self.graph.num_vertices()]
    }

    fn evaluate(&self, config: &[usize]) -> crate::types::Or {
        crate::types::Or({
            is_valid_acyclic_partition(
                &self.graph,
                &self.vertex_weights,
                &self.arc_costs,
                &self.weight_bound,
                &self.cost_bound,
                config,
            )
        })
    }
}

fn is_valid_acyclic_partition<W: WeightElement>(
    graph: &DirectedGraph,
    vertex_weights: &[W],
    arc_costs: &[W],
    weight_bound: &W::Sum,
    cost_bound: &W::Sum,
    config: &[usize],
) -> bool {
    let num_vertices = graph.num_vertices();
    if config.len() != num_vertices {
        return false;
    }
    if vertex_weights.len() != num_vertices || arc_costs.len() != graph.num_arcs() {
        return false;
    }
    if config.iter().any(|&label| label >= num_vertices) {
        return false;
    }

    let mut partition_weights = vec![W::Sum::zero(); num_vertices];
    let mut used_labels = vec![false; num_vertices];
    for (vertex, &label) in config.iter().enumerate() {
        used_labels[label] = true;
        partition_weights[label] += vertex_weights[vertex].to_sum();
        if partition_weights[label] > *weight_bound {
            return false;
        }
    }

    let mut dense_label = vec![usize::MAX; num_vertices];
    let mut next_dense = 0usize;
    for (label, used) in used_labels.iter().enumerate() {
        if *used {
            dense_label[label] = next_dense;
            next_dense += 1;
        }
    }

    let mut total_cost = W::Sum::zero();
    let mut quotient_arcs = BTreeSet::new();
    for ((source, target), cost) in graph.arcs().iter().zip(arc_costs.iter()) {
        let source_label = config[*source];
        let target_label = config[*target];
        if source_label == target_label {
            continue;
        }
        total_cost += cost.to_sum();
        if total_cost > *cost_bound {
            return false;
        }
        quotient_arcs.insert((dense_label[source_label], dense_label[target_label]));
    }

    DirectedGraph::new(next_dense, quotient_arcs.into_iter().collect()).is_dag()
}

crate::declare_variants! {
    default AcyclicPartition<i32> => "num_vertices^num_vertices",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "acyclic_partition_i32",
        instance: Box::new(AcyclicPartition::new(
            DirectedGraph::new(
                6,
                vec![
                    (0, 1),
                    (0, 2),
                    (1, 3),
                    (1, 4),
                    (2, 4),
                    (2, 5),
                    (3, 5),
                    (4, 5),
                ],
            ),
            vec![2, 3, 2, 1, 3, 1],
            vec![1; 8],
            5,
            5,
        )),
        optimal_config: vec![0, 1, 0, 2, 2, 2],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/acyclic_partition.rs"]
mod tests;
