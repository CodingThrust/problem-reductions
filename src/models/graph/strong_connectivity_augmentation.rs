//! Strong Connectivity Augmentation problem implementation.
//!
//! The Strong Connectivity Augmentation problem asks whether adding a bounded
//! set of weighted candidate arcs can make a directed graph strongly connected.

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::DirectedGraph;
use crate::traits::{Problem, SatisfactionProblem};
use crate::types::WeightElement;
use num_traits::Zero;
use serde::{Deserialize, Deserializer, Serialize};
use std::cmp::Ordering;
use std::collections::BTreeSet;

inventory::submit! {
    ProblemSchemaEntry {
        name: "StrongConnectivityAugmentation",
        display_name: "Strong Connectivity Augmentation",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("weight", "i32", &["i32"]),
        ],
        module_path: module_path!(),
        description: "Add a bounded set of weighted candidate arcs to make a digraph strongly connected",
        fields: &[
            FieldInfo { name: "graph", type_name: "DirectedGraph", description: "The initial directed graph G=(V,A)" },
            FieldInfo { name: "candidate_arcs", type_name: "Vec<(usize, usize, W)>", description: "Candidate augmenting arcs (u, v, w(u,v)) not already present in G" },
            FieldInfo { name: "bound", type_name: "W::Sum", description: "Upper bound B on the total added weight" },
        ],
    }
}

/// Strong Connectivity Augmentation.
///
/// Given a directed graph `G = (V, A)`, weighted candidate arcs not already in
/// `A`, and a bound `B`, determine whether some subset of the candidate arcs
/// has total weight at most `B` and makes the augmented digraph strongly
/// connected.
#[derive(Debug, Clone, Serialize)]
pub struct StrongConnectivityAugmentation<W: WeightElement> {
    graph: DirectedGraph,
    candidate_arcs: Vec<(usize, usize, W)>,
    bound: W::Sum,
}

impl<W: WeightElement> StrongConnectivityAugmentation<W> {
    /// Fallible constructor used by CLI validation and deserialization.
    pub fn try_new(
        graph: DirectedGraph,
        candidate_arcs: Vec<(usize, usize, W)>,
        bound: W::Sum,
    ) -> Result<Self, String> {
        if !matches!(
            bound.partial_cmp(&W::Sum::zero()),
            Some(Ordering::Equal | Ordering::Greater)
        ) {
            return Err("bound must be nonnegative".to_string());
        }

        let num_vertices = graph.num_vertices();
        let mut seen_pairs = BTreeSet::new();

        for (u, v, weight) in &candidate_arcs {
            if *u >= num_vertices || *v >= num_vertices {
                return Err(format!(
                    "candidate arc ({}, {}) references vertex >= num_vertices ({})",
                    u, v, num_vertices
                ));
            }
            if !matches!(
                weight.to_sum().partial_cmp(&W::Sum::zero()),
                Some(Ordering::Greater)
            ) {
                return Err(format!(
                    "candidate arc ({}, {}) weight must be positive",
                    u, v
                ));
            }
            if graph.has_arc(*u, *v) {
                return Err(format!(
                    "candidate arc ({}, {}) already exists in the base graph",
                    u, v
                ));
            }
            if !seen_pairs.insert((*u, *v)) {
                return Err(format!("duplicate candidate arc ({}, {})", u, v));
            }
        }

        Ok(Self {
            graph,
            candidate_arcs,
            bound,
        })
    }

    /// Create a new strong connectivity augmentation instance.
    ///
    /// # Panics
    ///
    /// Panics if a candidate arc endpoint is out of range, if a candidate arc
    /// already exists in the base graph, or if candidate arcs contain
    /// duplicates.
    pub fn new(
        graph: DirectedGraph,
        candidate_arcs: Vec<(usize, usize, W)>,
        bound: W::Sum,
    ) -> Self {
        Self::try_new(graph, candidate_arcs, bound).unwrap_or_else(|msg| panic!("{msg}"))
    }

    /// Get the base directed graph.
    pub fn graph(&self) -> &DirectedGraph {
        &self.graph
    }

    /// Get the candidate augmenting arcs.
    pub fn candidate_arcs(&self) -> &[(usize, usize, W)] {
        &self.candidate_arcs
    }

    /// Get the upper bound on the total added weight.
    pub fn bound(&self) -> &W::Sum {
        &self.bound
    }

    /// Get the number of vertices in the base graph.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of arcs in the base graph.
    pub fn num_arcs(&self) -> usize {
        self.graph.num_arcs()
    }

    /// Get the number of potential augmenting arcs.
    pub fn num_potential_arcs(&self) -> usize {
        self.candidate_arcs.len()
    }

    /// Check whether the problem uses non-unit weights.
    pub fn is_weighted(&self) -> bool {
        !W::IS_UNIT
    }

    /// Check whether a configuration is a satisfying augmentation.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        self.evaluate_config(config)
    }

    fn evaluate_config(&self, config: &[usize]) -> bool {
        if config.len() != self.candidate_arcs.len() {
            return false;
        }

        let mut total = W::Sum::zero();
        let mut augmented_arcs = self.graph.arcs();

        for ((u, v, weight), &selected) in self.candidate_arcs.iter().zip(config.iter()) {
            if selected > 1 {
                return false;
            }
            if selected == 1 {
                total += weight.to_sum();
                if total > self.bound {
                    return false;
                }
                augmented_arcs.push((*u, *v));
            }
        }

        DirectedGraph::new(self.graph.num_vertices(), augmented_arcs).is_strongly_connected()
    }
}

impl<W> Problem for StrongConnectivityAugmentation<W>
where
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "StrongConnectivityAugmentation";
    type Metric = bool;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![W]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.candidate_arcs.len()]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        self.evaluate_config(config)
    }
}

impl<W> SatisfactionProblem for StrongConnectivityAugmentation<W> where
    W: WeightElement + crate::variant::VariantParam
{
}

crate::declare_variants! {
    default sat StrongConnectivityAugmentation<i32> => "2^num_potential_arcs",
}

#[derive(Deserialize)]
struct StrongConnectivityAugmentationData<W: WeightElement> {
    graph: DirectedGraph,
    candidate_arcs: Vec<(usize, usize, W)>,
    bound: W::Sum,
}

impl<'de, W> Deserialize<'de> for StrongConnectivityAugmentation<W>
where
    W: WeightElement + Deserialize<'de>,
    W::Sum: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = StrongConnectivityAugmentationData::<W>::deserialize(deserializer)?;
        Self::try_new(data.graph, data.candidate_arcs, data.bound).map_err(serde::de::Error::custom)
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "strong_connectivity_augmentation_i32",
        build: || {
            // Path digraph 0→1→2→3→4 (not strongly connected — no back-edges).
            // Nine candidate arcs are all individually affordable, but only the
            // pair (4→1, w=3) + (1→0, w=5) = 8 = B achieves strong connectivity.
            // Other 4-escapes (4→3, 4→2) land on vertices from which reaching 0
            // within the remaining budget is impossible.
            let problem = StrongConnectivityAugmentation::new(
                DirectedGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]),
                vec![
                    (4, 0, 10), // direct fix, too expensive
                    (4, 3, 3),  // 4-escape to dead end
                    (4, 2, 3),  // 4-escape to dead end
                    (4, 1, 3),  // correct 4-escape
                    (3, 0, 7),  // too expensive to combine
                    (3, 1, 3),  // dead-end intermediate
                    (2, 0, 7),  // too expensive to combine
                    (2, 1, 3),  // dead-end intermediate
                    (1, 0, 5),  // the closing arc
                ],
                8,
            );

            crate::example_db::specs::satisfaction_example(
                problem,
                vec![
                    vec![0, 0, 0, 1, 0, 0, 0, 0, 1], // unique: (4→1)+(1→0), w=8
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0], // no arcs: not connected
                ],
            )
        },
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/strong_connectivity_augmentation.rs"]
mod tests;
