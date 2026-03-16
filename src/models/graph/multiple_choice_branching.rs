//! Multiple Choice Branching problem implementation.
//!
//! Given a directed graph with arc weights, a partition of the arcs, and a
//! threshold, determine whether there exists a high-weight branching that
//! picks at most one arc from each partition group.

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::DirectedGraph;
use crate::traits::{Problem, SatisfactionProblem};
use crate::types::WeightElement;
use num_traits::Zero;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MultipleChoiceBranching",
        display_name: "Multiple Choice Branching",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("weight", "i32", &["i32"]),
        ],
        module_path: module_path!(),
        description: "Find a branching with partition constraints and weight at least K",
        fields: &[
            FieldInfo { name: "graph", type_name: "DirectedGraph", description: "The directed graph G=(V,A)" },
            FieldInfo { name: "weights", type_name: "Vec<W>", description: "Arc weights w(a) for each arc a in A" },
            FieldInfo { name: "partition", type_name: "Vec<Vec<usize>>", description: "Partition of arc indices; each arc index must appear in exactly one group" },
            FieldInfo { name: "threshold", type_name: "W::Sum", description: "Weight threshold K" },
        ],
    }
}

/// The Multiple Choice Branching problem.
///
/// Given a directed graph G = (V, A), arc weights w(a), a partition of A into
/// disjoint groups A_1, ..., A_m, and a threshold K, determine whether there
/// exists a subset A' of arcs such that:
/// - the selected arcs have total weight at least K
/// - every vertex has in-degree at most one in the selected subgraph
/// - the selected subgraph is acyclic
/// - at most one arc is selected from each partition group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultipleChoiceBranching<W: WeightElement> {
    graph: DirectedGraph,
    weights: Vec<W>,
    partition: Vec<Vec<usize>>,
    threshold: W::Sum,
}

impl<W: WeightElement> MultipleChoiceBranching<W> {
    /// Create a new Multiple Choice Branching instance.
    pub fn new(
        graph: DirectedGraph,
        weights: Vec<W>,
        partition: Vec<Vec<usize>>,
        threshold: W::Sum,
    ) -> Self {
        let num_arcs = graph.num_arcs();
        assert_eq!(
            weights.len(),
            num_arcs,
            "weights length must match graph num_arcs"
        );
        validate_partition(&partition, num_arcs);
        Self {
            graph,
            weights,
            partition,
            threshold,
        }
    }

    /// Get the underlying directed graph.
    pub fn graph(&self) -> &DirectedGraph {
        &self.graph
    }

    /// Get the arc weights.
    pub fn weights(&self) -> &[W] {
        &self.weights
    }

    /// Replace the arc weights.
    pub fn set_weights(&mut self, weights: Vec<W>) {
        assert_eq!(
            weights.len(),
            self.graph.num_arcs(),
            "weights length must match graph num_arcs"
        );
        self.weights = weights;
    }

    /// Check whether this problem uses a non-unit weight type.
    pub fn is_weighted(&self) -> bool {
        !W::IS_UNIT
    }

    /// Get the partition groups.
    pub fn partition(&self) -> &[Vec<usize>] {
        &self.partition
    }

    /// Get the threshold K.
    pub fn threshold(&self) -> &W::Sum {
        &self.threshold
    }

    /// Get the number of vertices.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of arcs.
    pub fn num_arcs(&self) -> usize {
        self.graph.num_arcs()
    }

    /// Get the number of partition groups.
    pub fn num_partition_groups(&self) -> usize {
        self.partition.len()
    }

    /// Check whether a configuration is a satisfying solution.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        is_valid_multiple_choice_branching(&self.graph, &self.weights, &self.partition, &self.threshold, config)
    }
}

impl<W> Problem for MultipleChoiceBranching<W>
where
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "MultipleChoiceBranching";
    type Metric = bool;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![W]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_arcs()]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        is_valid_multiple_choice_branching(
            &self.graph,
            &self.weights,
            &self.partition,
            &self.threshold,
            config,
        )
    }
}

impl<W> SatisfactionProblem for MultipleChoiceBranching<W> where
    W: WeightElement + crate::variant::VariantParam
{
}

fn validate_partition(partition: &[Vec<usize>], num_arcs: usize) {
    let mut seen = vec![false; num_arcs];
    for group in partition {
        for &arc_index in group {
            assert!(
                arc_index < num_arcs,
                "partition arc index {} out of range for {} arcs",
                arc_index,
                num_arcs
            );
            assert!(
                !seen[arc_index],
                "partition arc index {} appears more than once",
                arc_index
            );
            seen[arc_index] = true;
        }
    }
    assert!(
        seen.iter().all(|present| *present),
        "partition must cover every arc exactly once"
    );
}

fn is_valid_multiple_choice_branching<W: WeightElement>(
    graph: &DirectedGraph,
    weights: &[W],
    partition: &[Vec<usize>],
    threshold: &W::Sum,
    config: &[usize],
) -> bool {
    if config.len() != graph.num_arcs() {
        return false;
    }
    if config.iter().any(|&value| value >= 2) {
        return false;
    }

    for group in partition {
        if group
            .iter()
            .filter(|&&arc_index| config[arc_index] == 1)
            .count()
            > 1
        {
            return false;
        }
    }

    let arcs = graph.arcs();
    let mut in_degree = vec![0usize; graph.num_vertices()];
    for (index, &selected) in config.iter().enumerate() {
        if selected == 1 {
            let (_, target) = arcs[index];
            in_degree[target] += 1;
            if in_degree[target] > 1 {
                return false;
            }
        }
    }

    let selected_arcs: Vec<bool> = config.iter().map(|&selected| selected == 1).collect();
    if !graph.is_acyclic_subgraph(&selected_arcs) {
        return false;
    }

    let mut total = W::Sum::zero();
    for (index, &selected) in config.iter().enumerate() {
        if selected == 1 {
            total += weights[index].to_sum();
        }
    }
    total >= *threshold
}

crate::declare_variants! {
    default sat MultipleChoiceBranching<i32> => "2^num_arcs",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "multiple_choice_branching_i32",
        build: || {
            let problem = MultipleChoiceBranching::new(
                DirectedGraph::new(
                    6,
                    vec![
                        (0, 1),
                        (0, 2),
                        (1, 3),
                        (2, 3),
                        (1, 4),
                        (3, 5),
                        (4, 5),
                        (2, 4),
                    ],
                ),
                vec![3, 2, 4, 1, 2, 3, 1, 3],
                vec![vec![0, 1], vec![2, 3], vec![4, 7], vec![5, 6]],
                10,
            );
            crate::example_db::specs::satisfaction_example(
                problem,
                vec![vec![1, 0, 1, 0, 0, 1, 0, 1]],
            )
        },
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/multiple_choice_branching.rs"]
mod tests;
