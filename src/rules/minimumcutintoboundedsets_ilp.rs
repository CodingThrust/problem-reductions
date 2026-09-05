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
    type Source = MinimumCutIntoBoundedSets<SimpleGraph, i64>;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok(target_solution[..self.num_vertices]
            .iter()
            .map(|&value| value == 1)
            .collect())
    }
}

#[reduction(
    transform = exact {
        num_vars = "num_vertices + num_edges",
        num_constraints = "2 + 2 + 2 * num_edges",
    },
    unavailable = {
        num_nonzeros = "the exact target parameter is not represented by this reduction's symbolic transform",
    }
)]
impl ReduceTo<ILP<bool>> for MinimumCutIntoBoundedSets<SimpleGraph, i64> {
    type Result = ReductionMinCutBSToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_vertices();
        let edges = self.graph().edges();
        let m = edges.len();
        let num_vars = n + m;
        let n_i64 = Self::exact_i64(n, "encoding the partition size")?;
        let size_bound = Self::exact_i64(self.size_bound(), "encoding the set-size bound")?;
        let mut constraints = Vec::new();

        // x_s = 0
        constraints.push(LinearConstraint::eq(vec![(self.source(), 1)], 0));

        // x_t = 1
        constraints.push(LinearConstraint::eq(vec![(self.sink(), 1)], 1));

        // Σ x_v ≤ B (sink side count)
        let all_terms: Vec<(usize, i64)> = (0..n).map(|v| (v, 1)).collect();
        constraints.push(LinearConstraint::le(all_terms, size_bound));

        // Σ (1 - x_v) ≤ B  ⟹  n - Σ x_v ≤ B  ⟹  -Σ x_v ≤ B - n  ⟹  Σ x_v ≥ n - B
        let all_terms2: Vec<(usize, i64)> = (0..n).map(|v| (v, 1)).collect();
        constraints.push(LinearConstraint::ge(all_terms2, n_i64 - size_bound));

        // Cut linking: for each edge e = {u, v}, y_e ≥ x_u - x_v and y_e ≥ x_v - x_u
        for (e_idx, &(u, v)) in edges.iter().enumerate() {
            let y = n + e_idx;
            // y_e - x_u + x_v ≥ 0  (y_e ≥ x_u - x_v)
            constraints.push(LinearConstraint::ge(vec![(y, 1), (u, -1), (v, 1)], 0));
            // y_e + x_u - x_v ≥ 0  (y_e ≥ x_v - x_u)
            constraints.push(LinearConstraint::ge(vec![(y, 1), (u, 1), (v, -1)], 0));
        }

        // Objective: minimize cut weight Σ w_e y_e
        let objective: Vec<(usize, i64)> = self
            .edge_weights()
            .iter()
            .enumerate()
            .map(|(edge, &weight)| (n + edge, weight))
            .collect();

        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize)
            .map_err(Self::target_construction)?;
        Ok(ReductionMinCutBSToILP {
            target,
            num_vertices: n,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimumcutintoboundedsets_to_ilp",
        build: || {
            let source = MinimumCutIntoBoundedSets::new(
                SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
                vec![1, 1, 1],
                0,
                3,
                3,
            );
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumcutintoboundedsets_ilp.rs"]
mod tests;
