//! Reduction from MaximumIndependentSet to IntegralFlowBundles (Sahni 1974).
//!
//! For a graph G = (V, E) with n = |V| vertices and m = |E| edges:
//!
//! 1. Create a directed graph with n + 2 vertices: source s, intermediate
//!    vertices w_0, ..., w_{n-1}, and sink t.
//! 2. For each vertex v_i, create two arcs: arc_in_i = (s, w_i) and
//!    arc_out_i = (w_i, t). Arc indices: arc_in_i = 2i, arc_out_i = 2i + 1.
//! 3. For each edge {v_i, v_j}, create bundle {arc_out_i, arc_out_j} with
//!    capacity 1 (conflict constraint: at most one endpoint selected).
//! 4. For each vertex v_i, create bundle {arc_in_i, arc_out_i} with capacity 2.
//!    Flow conservation at w_i forces arc_in_i = arc_out_i = f_i, so the
//!    bundle sum is 2*f_i <= 2, giving f_i in {0, 1}. This bundle also
//!    ensures every arc is covered by at least one bundle.
//! 5. Set requirement R = 1 (any non-empty independent set gives a feasible flow).
//!
//! An independent set of size k corresponds to a feasible flow of value k.
//! The bundle constraints ensure only independent sets produce valid flows.

use crate::models::graph::{IntegralFlowBundles, MaximumIndependentSet};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

#[cfg(feature = "example-db")]
use crate::solvers::BruteForce;

/// Result of reducing MaximumIndependentSet to IntegralFlowBundles.
#[derive(Debug, Clone)]
pub struct ReductionMISToIFB {
    target: IntegralFlowBundles,
    /// Number of vertices in the source graph.
    num_source_vertices: usize,
}

impl ReductionResult for ReductionMISToIFB {
    type Source = MaximumIndependentSet<SimpleGraph, i32>;
    type Target = IntegralFlowBundles;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Extract solution: vertex i is selected iff arc_out_i (index 2i + 1)
    /// has nonzero flow.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        (0..self.num_source_vertices)
            .map(|i| {
                if target_solution.get(2 * i + 1).copied().unwrap_or(0) > 0 {
                    1
                } else {
                    0
                }
            })
            .collect()
    }
}

#[reduction(
    overhead = {
        num_vertices = "num_vertices + 2",
        num_arcs = "2 * num_vertices",
        num_bundles = "num_edges + num_vertices",
    }
)]
impl ReduceTo<IntegralFlowBundles> for MaximumIndependentSet<SimpleGraph, i32> {
    type Result = ReductionMISToIFB;

    fn reduce_to(&self) -> Self::Result {
        let n = self.graph().num_vertices();
        let edges = self.graph().edges();

        // Set requirement = 1: any independent set of size >= 1 maps to a feasible flow.
        // The bundle constraints ensure only independent sets produce valid flows.
        let requirement = 1u64;

        // Vertices: s = 0, w_i = i + 1 (for i in 0..n), t = n + 1
        let source_vertex = 0;
        let sink_vertex = n + 1;

        // Arcs: arc_in_i = (s, w_i), arc_out_i = (w_i, t)
        let mut arcs = Vec::with_capacity(2 * n);
        for i in 0..n {
            arcs.push((source_vertex, i + 1)); // arc_in_i at index 2*i
            arcs.push((i + 1, sink_vertex)); // arc_out_i at index 2*i + 1
        }

        let directed_graph = crate::topology::DirectedGraph::new(n + 2, arcs);

        // Bundles: edge bundles + vertex bundles
        let mut bundles = Vec::with_capacity(edges.len() + n);
        let mut bundle_capacities = Vec::with_capacity(edges.len() + n);

        // Edge bundles: for each edge {v_i, v_j}, bundle {arc_out_i, arc_out_j} with cap 1
        for &(u, v) in &edges {
            bundles.push(vec![2 * u + 1, 2 * v + 1]);
            bundle_capacities.push(1);
        }

        // Vertex bundles: for each vertex v_i, bundle {arc_in_i, arc_out_i} with cap 2.
        // Flow conservation forces arc_in_i = arc_out_i = f_i, so bundle sum =
        // 2*f_i <= 2, hence f_i in {0, 1}. This also ensures every arc is in
        // at least one bundle (required by IntegralFlowBundles).
        for i in 0..n {
            bundles.push(vec![2 * i, 2 * i + 1]);
            bundle_capacities.push(2);
        }

        let target = IntegralFlowBundles::new(
            directed_graph,
            source_vertex,
            sink_vertex,
            bundles,
            bundle_capacities,
            requirement,
        );

        ReductionMISToIFB {
            target,
            num_source_vertices: n,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "maximumindependentset_to_integralflowbundles",
        build: || {
            // Path graph: 0-1-2-3, unit weights
            // Optimal MIS = {0, 2} or {1, 3} or {0, 3}, size = 2
            let source = MaximumIndependentSet::new(
                SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
                vec![1i32; 4],
            );
            let reduction = ReduceTo::<IntegralFlowBundles>::reduce_to(&source);
            let target = reduction.target_problem();

            let target_witness = BruteForce::new()
                .find_witness(target)
                .expect("target should have a feasible solution");
            let source_witness = reduction.extract_solution(&target_witness);

            crate::example_db::specs::rule_example_with_witness::<_, IntegralFlowBundles>(
                source,
                SolutionPair {
                    source_config: source_witness,
                    target_config: target_witness,
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/maximumindependentset_integralflowbundles.rs"]
mod tests;
