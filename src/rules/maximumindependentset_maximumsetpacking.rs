//! Reductions between MaximumIndependentSet and MaximumSetPacking problems.
//!
//! IS → MaximumSetPacking: Each vertex becomes a set containing its incident edge indices.
//! MaximumSetPacking → IS: Each set becomes a vertex; two vertices are adjacent if their sets overlap.

use crate::models::graph::MaximumIndependentSet;
use crate::models::set::MaximumSetPacking;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::types::{One, WeightElement};
use std::collections::HashSet;

/// Result of reducing MaximumIndependentSet to MaximumSetPacking.
#[derive(Debug, Clone)]
pub struct ReductionISToSP<W> {
    target: MaximumSetPacking<W>,
}

impl<W> ReductionResult for ReductionISToSP<W>
where
    W: WeightElement + crate::variant::VariantParam,
{
    type Source = MaximumIndependentSet<SimpleGraph, W>;
    type Target = MaximumSetPacking<W>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Solutions map directly: vertex selection = set selection.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

macro_rules! impl_is_to_sp {
    ($W:ty) => {
        #[reduction(overhead = { num_sets = "num_vertices", universe_size = "num_edges" })]
        impl ReduceTo<MaximumSetPacking<$W>> for MaximumIndependentSet<SimpleGraph, $W> {
            type Result = ReductionISToSP<$W>;

            fn reduce_to(&self) -> Self::Result {
                let edges = self.graph().edges();
                let n = self.graph().num_vertices();

                // For each vertex, collect the indices of its incident edges
                let mut sets: Vec<Vec<usize>> = vec![Vec::new(); n];
                for (edge_idx, &(u, v)) in edges.iter().enumerate() {
                    sets[u].push(edge_idx);
                    sets[v].push(edge_idx);
                }

                let target = MaximumSetPacking::with_weights(sets, self.weights().to_vec());

                ReductionISToSP { target }
            }
        }
    };
}

impl_is_to_sp!(i32);
impl_is_to_sp!(One);

/// Result of reducing MaximumSetPacking to MaximumIndependentSet.
#[derive(Debug, Clone)]
pub struct ReductionSPToIS<W> {
    target: MaximumIndependentSet<SimpleGraph, W>,
}

impl<W> ReductionResult for ReductionSPToIS<W>
where
    W: WeightElement + crate::variant::VariantParam,
{
    type Source = MaximumSetPacking<W>;
    type Target = MaximumIndependentSet<SimpleGraph, W>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Solutions map directly.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

macro_rules! impl_sp_to_is {
    ($W:ty) => {
        #[reduction(overhead = { num_vertices = "num_sets", num_edges = "num_sets^2" })]
        impl ReduceTo<MaximumIndependentSet<SimpleGraph, $W>> for MaximumSetPacking<$W> {
            type Result = ReductionSPToIS<$W>;

            fn reduce_to(&self) -> Self::Result {
                let sets = self.sets();
                let n = sets.len();

                // Create edges between sets that overlap
                let mut edges = Vec::new();
                for (i, set_i_vec) in sets.iter().enumerate() {
                    let set_i: HashSet<_> = set_i_vec.iter().collect();
                    for (j, set_j) in sets.iter().enumerate().skip(i + 1) {
                        // Check if sets[i] and sets[j] overlap
                        if set_j.iter().any(|elem| set_i.contains(elem)) {
                            edges.push((i, j));
                        }
                    }
                }

                let target = MaximumIndependentSet::new(
                    SimpleGraph::new(n, edges),
                    self.weights_ref().clone(),
                );

                ReductionSPToIS { target }
            }
        }
    };
}

impl_sp_to_is!(i32);
impl_sp_to_is!(One);

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![
        crate::example_db::specs::RuleExampleSpec {
            id: "maximumindependentset_to_maximumsetpacking",
            build: || {
                let (n, edges) = crate::topology::small_graphs::petersen();
                let source = MaximumIndependentSet::new(SimpleGraph::new(n, edges), vec![1i32; 10]);
                crate::example_db::specs::direct_best_example::<_, MaximumSetPacking<i32>, _>(
                    source,
                    |_, _| true,
                )
            },
        },
        crate::example_db::specs::RuleExampleSpec {
            id: "maximumsetpacking_to_maximumindependentset",
            build: || {
                let sets = vec![
                    vec![0, 1, 2],
                    vec![2, 3],
                    vec![4, 5, 6],
                    vec![1, 5, 7],
                    vec![3, 6],
                ];
                let source = MaximumSetPacking::with_weights(sets, vec![1i32; 5]);
                crate::example_db::specs::direct_best_example::<
                    _,
                    MaximumIndependentSet<SimpleGraph, i32>,
                    _,
                >(source, |_, _| true)
            },
        },
    ]
}

#[cfg(test)]
#[path = "../unit_tests/rules/maximumindependentset_maximumsetpacking.rs"]
mod tests;
