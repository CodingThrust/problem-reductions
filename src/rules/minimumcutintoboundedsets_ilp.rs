//! Reduction from MinimumCutIntoBoundedSets to ILP.
//!
//! Binary x_v (1 iff v on sink side), binary y_e (cut indicator).
//! Source pinned to 0, sink pinned to 1.
//! Size bounds: Σ x_v ≤ B, Σ (1-x_v) ≤ B.
//! Cut linking: y_e ≥ x_u - x_v, y_e ≥ x_v - x_u for each edge {u,v}.
//! Cut bound: Σ w_e y_e ≤ K.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::MinimumCutIntoBoundedSets;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

#[derive(Debug, Clone)]
pub struct ReductionMinCutBSToILP {
    target: ILP<bool>,
    num_vertices: usize,
}

impl ReductionResult for ReductionMinCutBSToILP {
    type Source = MinimumCutIntoBoundedSets<SimpleGraph, i32>;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution[..self.num_vertices].to_vec()
    }
}

#[reduction(
    overhead = {
        num_vars = "num_vertices + num_edges",
        num_constraints = "2 + 2 * num_edges + 1",
    }
)]
impl ReduceTo<ILP<bool>> for MinimumCutIntoBoundedSets<SimpleGraph, i32> {
    type Result = ReductionMinCutBSToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let edges = self.graph().edges();
        let m = edges.len();
        let num_vars = n + m;
        let mut constraints = Vec::new();

        // x_s = 0
        constraints.push(LinearConstraint::eq(vec![(self.source(), 1.0)], 0.0));

        // x_t = 1
        constraints.push(LinearConstraint::eq(vec![(self.sink(), 1.0)], 1.0));

        // Σ x_v ≤ B (sink side count)
        let all_terms: Vec<(usize, f64)> = (0..n).map(|v| (v, 1.0)).collect();
        constraints.push(LinearConstraint::le(all_terms, self.size_bound() as f64));

        // Σ (1 - x_v) ≤ B  ⟹  n - Σ x_v ≤ B  ⟹  -Σ x_v ≤ B - n  ⟹  Σ x_v ≥ n - B
        let all_terms2: Vec<(usize, f64)> = (0..n).map(|v| (v, 1.0)).collect();
        constraints.push(LinearConstraint::ge(
            all_terms2,
            (n as f64) - (self.size_bound() as f64),
        ));

        // Cut linking: for each edge e = {u, v}, y_e ≥ x_u - x_v and y_e ≥ x_v - x_u
        for (e_idx, &(u, v)) in edges.iter().enumerate() {
            let y = n + e_idx;
            // y_e - x_u + x_v ≥ 0  (y_e ≥ x_u - x_v)
            constraints.push(LinearConstraint::ge(
                vec![(y, 1.0), (u, -1.0), (v, 1.0)],
                0.0,
            ));
            // y_e + x_u - x_v ≥ 0  (y_e ≥ x_v - x_u)
            constraints.push(LinearConstraint::ge(
                vec![(y, 1.0), (u, 1.0), (v, -1.0)],
                0.0,
            ));
        }

        // Cut bound: Σ w_e y_e ≤ K
        let cut_terms: Vec<(usize, f64)> = self
            .edge_weights()
            .iter()
            .enumerate()
            .map(|(e_idx, &w)| (n + e_idx, w as f64))
            .collect();
        constraints.push(LinearConstraint::le(cut_terms, *self.cut_bound() as f64));

        let target = ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize);
        ReductionMinCutBSToILP {
            target,
            num_vertices: n,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimumcutintoboundedsets_to_ilp",
        build: || {
            let source = MinimumCutIntoBoundedSets::new(
                SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
                vec![1, 1, 1],
                0,
                3,
                3,
                2,
            );
            crate::example_db::specs::rule_example_with_witness::<_, ILP<bool>>(
                source,
                SolutionPair {
                    source_config: vec![0, 0, 1, 1],
                    target_config: vec![0, 0, 1, 1, 0, 1, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumcutintoboundedsets_ilp.rs"]
mod tests;
