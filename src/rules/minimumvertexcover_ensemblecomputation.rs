//! Reduction from MinimumVertexCover (unit-weight) to EnsembleComputation.
//!
//! Given a graph G = (V, E), construct an EnsembleComputation instance where:
//! - Universe A = V ∪ {a₀} (fresh element a₀ at index |V|)
//! - Collection C = {{a₀, u, v} : {u,v} ∈ E}
//! - Budget = max(1, |V| + |E|) (positive search-space bound)
//!
//! For loopless simple graphs, the minimum sequence length is K* + |E|, where K* is the minimum vertex
//! cover size. This follows from the Garey & Johnson proof (PO9): each cover
//! vertex contributes one {a₀} ∪ {v} operation, and each edge contributes
//! one {u} ∪ z_k operation.
//!
//! Reference: Garey & Johnson, *Computers and Intractability*, Theorem 3.6, pp. 66–68 (also Appendix PO9).

use crate::models::graph::MinimumVertexCover;
use crate::models::misc::EnsembleComputation;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::types::One;

/// Result of reducing MinimumVertexCover to EnsembleComputation.
#[derive(Debug, Clone)]
pub struct ReductionVCToEC {
    target: EnsembleComputation,
    /// Number of vertices in the source graph (= index of fresh element a₀).
    num_vertices: usize,
}

impl ReductionResult for ReductionVCToEC {
    type Source = MinimumVertexCover<SimpleGraph, One>;
    type Target = EnsembleComputation;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Extract one vertex per pair-producing operation in the evaluated prefix.
    /// Every required triple uses an earlier pair. The chosen endpoint covers
    /// its edge. An L-step program yields at most L minus the number of
    /// distinct required triples; loops instead require endpoint pairs.
    /// This applies to arbitrary programs, without a normal-form assumption.
    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        let value =
            crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;
        let crate::types::Min(Some(length)) = value else {
            return Err(crate::rules::ExtractionError::invalid(
                "target configuration does not encode a valid ensemble computation",
            ));
        };
        let meaningful_steps = usize::try_from(length).map_err(|_| {
            crate::rules::ExtractionError::invalid(
                "ensemble operation count cannot be represented as usize",
            )
        })?;
        let mut cover = vec![false; self.num_vertices];
        let universe_size = self.target.universe_size();
        for &[left, right] in target_solution
            .as_chunks::<2>()
            .0
            .iter()
            .take(meaningful_steps)
        {
            if left < universe_size && right < universe_size {
                // Only two singleton operands can produce a two-element set.
                // The fresh atom is largest, so min selects the original
                // vertex in {a0,v}, or an endpoint of an edge-pair {u,v}.
                cover[left.min(right)] = true;
            }
        }
        Ok(cover)
    }
}

#[reduction(
    transform = upper_bound {
        universe_size = "num_vertices + 1",
        num_subsets = "num_edges",
        budget = "num_vertices + num_edges + 1",
    }
)]
impl ReduceTo<EnsembleComputation> for MinimumVertexCover<SimpleGraph, One> {
    type Result = ReductionVCToEC;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let num_vertices = self.graph().num_vertices();
        let edges = self.graph().edges();
        let num_edges = edges.len();
        let a0 = num_vertices; // fresh element index

        // Universe A = V ∪ {a₀}, size = |V| + 1
        let overflow = || {
            crate::rules::ReductionError::integer_overflow::<Self, EnsembleComputation>(
                "computing ensemble universe, budget, or operand dimensions",
            )
        };
        let universe_size = num_vertices.checked_add(1).ok_or_else(overflow)?;

        // Collection C: for each edge {u, v}, add subset {a₀, u, v}
        let subsets: Vec<Vec<usize>> = edges.iter().map(|&(u, v)| vec![a0, u, v]).collect();

        // Budget bounds the search space; the optimal sequence length
        // is K* + |E| where K* is the minimum vertex cover size.
        let budget = num_vertices
            .checked_add(num_edges)
            .ok_or_else(overflow)?
            .max(1);
        universe_size.checked_add(budget).ok_or_else(overflow)?;
        budget.checked_mul(2).ok_or_else(overflow)?;

        let target = EnsembleComputation::try_new(universe_size, subsets, budget)
            .map_err(crate::rules::ReductionError::construction::<Self, EnsembleComputation>)?;

        Ok(ReductionVCToEC {
            target,
            num_vertices,
        })
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
            // Optimal sequence length = K* + |E| = 1 + 1 = 2
            let source = MinimumVertexCover::new(SimpleGraph::new(2, vec![(0, 1)]), vec![One; 2]);

            // Optimal sequence for cover {0} (2 steps):
            // Step 0: {a₀=2} ∪ {0} → z₀ = {0,2}   operands: (2, 0)
            // Step 1: {1} ∪ z₀ → z₁ = {0,1,2} ✓    operands: (1, 3) where 3 = universe_size + 0
            // Step 2: padding (unused)                operands: (2, 1)
            let target_config = vec![
                2, 0, // step 0: {a₀} ∪ {0}
                1, 3, // step 1: {1} ∪ z₀
                2, 1, // step 2: padding
            ];
            // Only step 0 produces a pair; step 1 produces the required
            // triple, and step 2 is padding. Extraction returns minimum {0}.
            let source_config = vec![true, false];

            crate::example_db::specs::rule_example_with_witness::<_, EnsembleComputation>(
                source,
                SolutionPair {
                    source_config: serde_json::to_value(source_config)
                        .expect("solution serialization must succeed"),
                    target_config: serde_json::to_value(target_config)
                        .expect("solution serialization must succeed"),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumvertexcover_ensemblecomputation.rs"]
mod tests;
