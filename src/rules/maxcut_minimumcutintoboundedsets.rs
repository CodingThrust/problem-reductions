//! Reduction from MaxCut to MinimumCutIntoBoundedSets.
//!
//! Transforms a maximum cut problem into a minimum cut into bounded sets problem
//! by padding to even vertex count, building a complete graph with inverted weights,
//! and enforcing balanced bisection via size bounds.
//!
//! Reference: Garey, Johnson, and Stockmeyer (1976), "Some simplified NP-complete
//! graph problems". Garey & Johnson, *Computers and Intractability*, ND17.

use crate::models::graph::{MaxCut, MinimumCutIntoBoundedSets};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing MaxCut to MinimumCutIntoBoundedSets.
#[derive(Debug, Clone)]
pub struct ReductionMaxCutToMinCutBounded {
    target: MinimumCutIntoBoundedSets<SimpleGraph, i64>,
    /// Number of original vertices in the source problem.
    original_n: usize,
}

impl ReductionResult for ReductionMaxCutToMinCutBounded {
    type Source = MaxCut<SimpleGraph, i64>;
    type Target = MinimumCutIntoBoundedSets<SimpleGraph, i64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Extract the source solution from the target balanced bisection.
    /// Take only the first `original_n` vertex assignments.
    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok(target_solution[..self.original_n].to_vec())
    }
}

#[reduction(
    size = exact {
        num_vertices = "2 * num_vertices + 2",
        num_edges = "(num_vertices + 1) * (2 * num_vertices + 1)",
    }
)]
impl ReduceTo<MinimumCutIntoBoundedSets<SimpleGraph, i64>> for MaxCut<SimpleGraph, i64> {
    type Result = ReductionMaxCutToMinCutBounded;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.graph().num_vertices();

        // Step 1: Pad to even vertex count.
        // n' = n if n is even, n+1 if n is odd. N = 2*n'.
        let n_prime = n.checked_add(n % 2).ok_or_else(|| {
            crate::rules::ReductionError::integer_overflow::<
                MaxCut<SimpleGraph, i64>,
                MinimumCutIntoBoundedSets<SimpleGraph, i64>,
            >("padding the source vertex count")
        })?;
        let big_n = n_prime.checked_mul(2).ok_or_else(|| {
            crate::rules::ReductionError::integer_overflow::<
                MaxCut<SimpleGraph, i64>,
                MinimumCutIntoBoundedSets<SimpleGraph, i64>,
            >("computing the target vertex count")
        })?;

        // Step 2: Compute W_max
        let w_max = self
            .edge_weights()
            .iter()
            .copied()
            .max()
            .unwrap_or(0)
            .checked_add(1)
            .ok_or_else(|| {
                crate::rules::ReductionError::integer_overflow::<
                    MaxCut<SimpleGraph, i64>,
                    MinimumCutIntoBoundedSets<SimpleGraph, i64>,
                >("computing the inverted-weight ceiling")
            })?;

        // Build an adjacency lookup for the original graph
        let orig_edges = self.graph().edges();
        let mut edge_weight_map: std::collections::HashMap<(usize, usize), i64> =
            std::collections::HashMap::new();
        for (&(u, v), w) in orig_edges.iter().zip(self.edge_weights()) {
            let (a, b) = if u < v { (u, v) } else { (v, u) };
            edge_weight_map.insert((a, b), w);
        }

        // Step 3: Build complete graph K_N with inverted weights
        let mut edges = Vec::new();
        let mut weights = Vec::new();
        for i in 0..big_n {
            for j in (i + 1)..big_n {
                edges.push((i, j));
                if let Some(&w) = edge_weight_map.get(&(i, j)) {
                    weights.push(w_max.checked_sub(w).ok_or_else(|| {
                        crate::rules::ReductionError::integer_overflow::<
                            MaxCut<SimpleGraph, i64>,
                            MinimumCutIntoBoundedSets<SimpleGraph, i64>,
                        >("inverting an edge weight")
                    })?);
                } else {
                    weights.push(w_max);
                }
            }
        }

        // Step 4: Set source, sink, size_bound
        let source_vertex = n_prime;
        let sink_vertex = n_prime.checked_add(1).ok_or_else(|| {
            crate::rules::ReductionError::integer_overflow::<
                MaxCut<SimpleGraph, i64>,
                MinimumCutIntoBoundedSets<SimpleGraph, i64>,
            >("computing the target sink vertex")
        })?;
        let size_bound = n_prime;

        let target = MinimumCutIntoBoundedSets::new(
            SimpleGraph::new(big_n, edges),
            weights,
            source_vertex,
            sink_vertex,
            size_bound,
        );

        Ok(ReductionMaxCutToMinCutBounded {
            target,
            original_n: n,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::solvers::BruteForce;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "maxcut_to_minimumcutintoboundedsets",
        build: || {
            // Triangle with unit weights: max cut = 2
            let source = MaxCut::new(
                SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
                vec![1i64, 1, 1],
            );
            let reduction =
                ReduceTo::<MinimumCutIntoBoundedSets<SimpleGraph, i64>>::reduce_to(&source)
                    .expect("reduction should succeed");

            // Find optimal source and target solutions
            let solver = BruteForce::new();
            let source_witness = solver
                .solve(&source)
                .expect("canonical source evaluation must succeed")
                .expect("canonical source must have an optimum");
            let target_witness = solver
                .solve(reduction.target_problem())
                .expect("canonical target evaluation must succeed")
                .expect("canonical target must have an optimum");

            crate::example_db::specs::assemble_rule_example(
                &source,
                reduction.target_problem(),
                vec![SolutionPair {
                    source_config: serde_json::json!(source_witness),
                    target_config: serde_json::json!(target_witness),
                }],
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/maxcut_minimumcutintoboundedsets.rs"]
mod tests;
