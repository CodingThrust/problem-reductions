//! Reduction from MinimumVertexCover to MinimumFeedbackArcSet.
//!
//! Each vertex v is split into v^in and v^out connected by an internal arc
//! (v^in → v^out) with weight w(v). For each edge {u,v}, two crossing arcs
//! (u^out → v^in) and (v^out → u^in) are added with a large penalty weight
//! M = 1 + Σ w(v). The penalty ensures no optimal FAS includes crossing arcs.
//!
//! A vertex cover of the source maps to a feedback arc set of internal arcs:
//! if vertex i is in the cover, remove internal arc i.

use crate::models::graph::{MinimumFeedbackArcSet, MinimumVertexCover};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{DirectedGraph, Graph, SimpleGraph};

/// Result of reducing MinimumVertexCover to MinimumFeedbackArcSet.
#[derive(Debug, Clone)]
pub struct ReductionVCToFAS {
    target: MinimumFeedbackArcSet<i32>,
    /// Number of vertices in the source graph (= number of internal arcs).
    num_source_vertices: usize,
}

impl ReductionResult for ReductionVCToFAS {
    type Source = MinimumVertexCover<SimpleGraph, i32>;
    type Target = MinimumFeedbackArcSet<i32>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Extract solution: internal arcs are at positions 0..n in the FAS config.
    /// If internal arc i is in the FAS (config[i] = 1), vertex i is in the cover.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution[..self.num_source_vertices].to_vec()
    }
}

#[reduction(
    overhead = {
        num_vertices = "2 * num_vertices",
        num_arcs = "num_vertices + 2 * num_edges",
    }
)]
impl ReduceTo<MinimumFeedbackArcSet<i32>> for MinimumVertexCover<SimpleGraph, i32> {
    type Result = ReductionVCToFAS;

    fn reduce_to(&self) -> Self::Result {
        let n = self.graph().num_vertices();
        let edges = self.graph().edges();

        // Vertex splitting: vertex v → v^in (index v) and v^out (index n + v)
        // Internal arcs: (v^in → v^out) for each vertex v, with weight w(v)
        // Crossing arcs: for each edge {u,v}, add (u^out → v^in) and (v^out → u^in) with weight M

        let big_m: i32 = 1 + self.weights().iter().sum::<i32>();

        let mut arcs = Vec::with_capacity(n + 2 * edges.len());
        let mut weights = Vec::with_capacity(n + 2 * edges.len());

        // Internal arcs first (indices 0..n)
        for v in 0..n {
            arcs.push((v, n + v)); // v^in → v^out
            weights.push(self.weights()[v]);
        }

        // Crossing arcs for each edge
        for (u, v) in &edges {
            arcs.push((n + u, *v)); // u^out → v^in
            weights.push(big_m);
            arcs.push((n + v, *u)); // v^out → u^in
            weights.push(big_m);
        }

        let graph = DirectedGraph::new(2 * n, arcs);
        let target = MinimumFeedbackArcSet::new(graph, weights);

        ReductionVCToFAS {
            target,
            num_source_vertices: n,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::solvers::BruteForce;
    use crate::traits::Problem;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimumvertexcover_to_minimumfeedbackarcset",
        build: || {
            // Triangle graph: 0-1-2-0, unit weights
            // MVC optimal = 2 vertices (e.g., {0, 1})
            let source = MinimumVertexCover::new(
                SimpleGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]),
                vec![1i32; 3],
            );
            let reduction = ReduceTo::<MinimumFeedbackArcSet<i32>>::reduce_to(&source);
            let target = reduction.target_problem();

            let target_witness = BruteForce::new()
                .find_witness(target)
                .expect("target should have an optimum");
            let source_witness = reduction.extract_solution(&target_witness);

            crate::example_db::specs::rule_example_with_witness::<_, MinimumFeedbackArcSet<i32>>(
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
#[path = "../unit_tests/rules/minimumvertexcover_minimumfeedbackarcset.rs"]
mod tests;
