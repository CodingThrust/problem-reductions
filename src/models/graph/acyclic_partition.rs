//! Acyclic Partition problem implementation.
//!
//! Given a directed graph with vertex weights, arc costs, and bounds, determine
//! whether the vertices can be partitioned into groups whose quotient graph is a
//! DAG, each group's total vertex weight is bounded, and the total
//! inter-partition arc cost is bounded.

use crate::registry::{CreateSpec, ProblemSchemaEntry, ProblemSizeFieldEntry, VariantDimension};
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
            VariantDimension::new("weight", "i64", &["i64"]),
        ],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Partition a directed graph into bounded-weight groups with an acyclic quotient graph and bounded inter-partition cost",
        fields: AcyclicPartitionCreateSpec::FIELDS,
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

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct AcyclicPartitionCreateSpec {
    #[create(codec = "arc-list")]
    arcs: Vec<(usize, usize)>,
    num_vertices: Option<usize>,
    #[create(codec = "comma-separated")]
    weights: Option<Vec<i64>>,
    #[create(name = "arc_costs", codec = "comma-separated")]
    arc_weights: Option<Vec<i64>>,
    weight_bound: i64,
    cost_bound: i64,
}

impl TryFrom<AcyclicPartitionCreateSpec> for AcyclicPartition<i64> {
    type Error = crate::registry::ConstructionError;

    fn try_from(spec: AcyclicPartitionCreateSpec) -> Result<Self, Self::Error> {
        if spec.arcs.is_empty() && spec.num_vertices.is_none() {
            return Err("num_vertices is required for an empty arc list"
                .to_string()
                .into());
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
        let graph = DirectedGraph::new(num_vertices, spec.arcs);
        let vertex_weights = spec.weights.unwrap_or_else(|| vec![1; num_vertices]);
        if vertex_weights.len() != num_vertices {
            return Err(format!(
                "weights has length {}, expected {num_vertices}",
                vertex_weights.len()
            )
            .into());
        }
        let arc_costs = spec
            .arc_weights
            .unwrap_or_else(|| vec![1; graph.num_arcs()]);
        if arc_costs.len() != graph.num_arcs() {
            return Err(format!(
                "arc_weights has length {}, expected {}",
                arc_costs.len(),
                graph.num_arcs()
            )
            .into());
        }
        Ok(Self::new(
            graph,
            vertex_weights,
            arc_costs,
            spec.weight_bound,
            spec.cost_bound,
        ))
    }
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
    pub fn is_valid_solution(
        &self,
        config: &[usize],
    ) -> Result<bool, crate::traits::EvaluationError> {
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

    fn evaluate(
        &self,
        config: &[usize],
    ) -> Result<crate::types::Or, crate::traits::EvaluationError> {
        Ok({
            crate::types::Or({
                is_valid_acyclic_partition(
                    &self.graph,
                    &self.vertex_weights,
                    &self.arc_costs,
                    &self.weight_bound,
                    &self.cost_bound,
                    config,
                )?
            })
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
) -> Result<bool, crate::traits::EvaluationError> {
    let num_vertices = graph.num_vertices();
    if config.len() != num_vertices {
        return Ok(false);
    }
    if vertex_weights.len() != num_vertices || arc_costs.len() != graph.num_arcs() {
        return Ok(false);
    }
    if config.iter().any(|&label| label >= num_vertices) {
        return Ok(false);
    }

    let mut partition_weights = vec![W::Sum::zero(); num_vertices];
    let mut used_labels = vec![false; num_vertices];
    for (vertex, &label) in config.iter().enumerate() {
        used_labels[label] = true;
        partition_weights[label] = W::checked_add_to_sum(
            partition_weights[label].clone(),
            vertex_weights[vertex].to_sum(),
            "summing acyclic partition vertex weights",
        )?;
        if partition_weights[label] > *weight_bound {
            return Ok(false);
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
        total_cost = W::checked_add_to_sum(
            total_cost,
            cost.to_sum(),
            "summing acyclic partition arc costs",
        )?;
        if total_cost > *cost_bound {
            return Ok(false);
        }
        quotient_arcs.insert((dense_label[source_label], dense_label[target_label]));
    }

    Ok(DirectedGraph::new(next_dense, quotient_arcs.into_iter().collect()).is_dag())
}

crate::declare_variants! {
    default AcyclicPartition<i64> => "num_vertices^num_vertices" create AcyclicPartitionCreateSpec,
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "acyclic_partition_i64",
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
