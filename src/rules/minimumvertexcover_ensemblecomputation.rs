//! Reduction from MinimumVertexCover to EnsembleComputation.
//!
//! Given a graph G = (V, E), construct an EnsembleComputation instance where:
//! - Universe A = V ∪ {a₀} (fresh element a₀ at index |V|)
//! - Collection C = {{a₀, u, v} : {u,v} ∈ E}
//! - Budget J = |V| + |E| (upper bound)
//!
//! Reference: Garey & Johnson, *Computers and Intractability*, Appendix Problem PO9.

use crate::models::graph::MinimumVertexCover;
use crate::models::misc::EnsembleComputation;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing MinimumVertexCover to EnsembleComputation.
#[derive(Debug, Clone)]
pub struct ReductionVCToEC {
    target: EnsembleComputation,
    /// Number of vertices in the source graph (= index of fresh element a₀).
    num_vertices: usize,
}

impl ReductionResult for ReductionVCToEC {
    type Source = MinimumVertexCover<SimpleGraph, i32>;
    type Target = EnsembleComputation;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Extract a vertex cover from an EnsembleComputation witness.
    ///
    /// Every vertex that appears as a singleton operand (index < `num_vertices`)
    /// in the sequence is included in the cover. This yields a valid cover because
    /// every required subset {a₀, u, v} requires all three elements (a₀, u, v) to
    /// enter the computation chain as singletons, so both endpoints of every edge
    /// are included. The result is valid but not necessarily minimum.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let budget = self.target.budget();
        let mut cover = vec![0usize; self.num_vertices];

        for step in 0..budget {
            let left = target_solution[2 * step];
            let right = target_solution[2 * step + 1];

            if left < self.num_vertices {
                cover[left] = 1;
            }
            if right < self.num_vertices {
                cover[right] = 1;
            }
        }

        cover
    }
}

#[reduction(
    overhead = {
        universe_size = "num_vertices + 1",
        num_subsets = "num_edges",
    }
)]
impl ReduceTo<EnsembleComputation> for MinimumVertexCover<SimpleGraph, i32> {
    type Result = ReductionVCToEC;

    fn reduce_to(&self) -> Self::Result {
        let num_vertices = self.graph().num_vertices();
        let edges = self.graph().edges();
        let num_edges = edges.len();
        let a0 = num_vertices; // fresh element index

        // Universe A = V ∪ {a₀}, size = |V| + 1
        let universe_size = num_vertices + 1;

        // Collection C: for each edge {u, v}, add subset {a₀, u, v}
        let subsets: Vec<Vec<usize>> = edges.iter().map(|&(u, v)| vec![a0, u, v]).collect();

        // Budget J = |V| + |E| (upper bound: K* ≤ |V| always)
        let budget = num_vertices + num_edges;

        let target = EnsembleComputation::new(universe_size, subsets, budget);

        ReductionVCToEC {
            target,
            num_vertices,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimumvertexcover_to_ensemblecomputation",
        build: || {
            // Single edge graph: 2 vertices {0,1}, 1 edge (0,1)
            // Minimum vertex cover K* = 1 (either {0} or {1})
            // Budget = 2 + 1 = 3, universe_size = 3, a₀ = 2
            // Subsets = {{0,1,2}}
            let source = MinimumVertexCover::new(SimpleGraph::new(2, vec![(0, 1)]), vec![1i32; 2]);

            // Satisfying sequence for cover {0}:
            // Step 0: {a₀=2} ∪ {0} → z₀ = {0,2}   operands: (2, 0)
            // Step 1: {1} ∪ z₀ → z₁ = {0,1,2} ✓    operands: (1, 3) where 3 = universe_size + 0
            // Step 2: padding {a₀=2} ∪ {1}           operands: (2, 1)
            //
            // Extraction picks up all singleton vertex operands: vertex 0 (step 0),
            // vertex 1 (steps 1 and 2). The extracted cover {0,1} is valid.
            let target_config = vec![
                2, 0, // step 0: {a₀} ∪ {0}
                1, 3, // step 1: {1} ∪ z₀
                2, 1, // step 2: padding
            ];
            let source_config = vec![1, 1];

            crate::example_db::specs::rule_example_with_witness::<_, EnsembleComputation>(
                source,
                SolutionPair {
                    source_config,
                    target_config,
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumvertexcover_ensemblecomputation.rs"]
mod tests;
