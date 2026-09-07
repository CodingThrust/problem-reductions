//! Reduction from OptimalLinearArrangement to ILP (Integer Linear Programming).
//!
//! Position-assignment with absolute-value auxiliaries:
//! - Binary x_{v,p}: vertex v gets position p
//! - Integer position variables p_v = sum_p p * x_{v,p}
//! - Non-negative z_{u,v} per edge for |p_u - p_v|
//! - abs_diff_le constraints: z_{u,v} >= p_u - p_v, z_{u,v} >= p_v - p_u
//! - Minimize: sum z_{u,v}

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::OptimalLinearArrangement;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing OptimalLinearArrangement to ILP.
///
/// Variable layout (`ILP<i64>`, non-negative integers):
/// - `x_{v,p}` at index `v * n + p`, bounded to {0,1}
/// - `p_v` at index `n^2 + v`, integer position in {0, ..., n-1}
/// - `z_e` at index `n^2 + n + e`, non-negative integer for edge length
#[derive(Debug, Clone)]
pub struct ReductionOLAToILP {
    target: ILP<i64>,
    num_vertices: usize,
}

impl ReductionResult for ReductionOLAToILP {
    type Source = OptimalLinearArrangement<SimpleGraph>;
    type Target = ILP<i64>;

    fn target_problem(&self) -> &ILP<i64> {
        &self.target
    }

    /// Extract: for each vertex v, output its position p (the unique p with x_{v,p} = 1).
    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        crate::rules::ilp_helpers::one_hot_decode_rows(
            target_solution,
            self.num_vertices,
            self.num_vertices,
            0,
        )
    }
}

#[reduction(
    transform = exact {
        num_vars = "num_vertices^2 + num_vertices + num_edges",
        num_constraints = "2 * num_vertices + num_vertices^2 + num_vertices + num_vertices + 3 * num_edges",
    },
    unavailable = {
        num_nonzeros = "the exact target parameter is not represented by this reduction's symbolic transform",
    }
)]
impl ReduceTo<ILP<i64>> for OptimalLinearArrangement<SimpleGraph> {
    type Result = ReductionOLAToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_vertices();
        let graph = self.graph();
        let edges = graph.edges();
        let m = edges.len();

        let num_x = n * n;
        let num_vars = num_x + n + m;

        let x_idx = |v: usize, p: usize| -> usize { v * n + p };
        let p_idx = |v: usize| -> usize { num_x + v };
        let z_idx = |e: usize| -> usize { num_x + n + e };

        let mut constraints = Vec::new();
        let n_i64 = <Self as ReduceTo<ILP<i64>>>::exact_i64(n, "encoding a vertex position")?;

        // Assignment: each vertex in exactly one position
        for v in 0..n {
            let terms: Vec<(usize, i64)> = (0..n).map(|p| (x_idx(v, p), 1)).collect();
            constraints.push(LinearConstraint::eq(terms, 1));
        }

        // Assignment: each position has exactly one vertex
        for p in 0..n {
            let terms: Vec<(usize, i64)> = (0..n).map(|v| (x_idx(v, p), 1)).collect();
            constraints.push(LinearConstraint::eq(terms, 1));
        }

        // Binary bounds for x variables (`ILP<i64>`)
        for v in 0..n {
            for p in 0..n {
                constraints.push(LinearConstraint::le(vec![(x_idx(v, p), 1)], 1));
            }
        }

        // Position variable linking: p_v = sum_p p * x_{v,p}
        // Reformulated as: p_v - sum_p p * x_{v,p} = 0
        for v in 0..n {
            let mut terms: Vec<(usize, i64)> = vec![(p_idx(v), 1)];
            for p in 0..n {
                terms.push((
                    x_idx(v, p),
                    -<Self as ReduceTo<ILP<i64>>>::exact_i64(p, "encoding a vertex position")?,
                ));
            }
            constraints.push(LinearConstraint::eq(terms, 0));
        }

        // Position bounds: 0 <= p_v <= n-1
        for v in 0..n {
            constraints.push(LinearConstraint::le(vec![(p_idx(v), 1)], n_i64 - 1));
        }

        // Absolute value: z_e >= |p_u - p_v| for each edge e = {u, v}
        for (e, &(u, v)) in edges.iter().enumerate() {
            // z_e >= p_u - p_v
            constraints.push(LinearConstraint::ge(
                vec![(z_idx(e), 1), (p_idx(u), -1), (p_idx(v), 1)],
                0,
            ));
            // z_e >= p_v - p_u
            constraints.push(LinearConstraint::ge(
                vec![(z_idx(e), 1), (p_idx(v), -1), (p_idx(u), 1)],
                0,
            ));
            // z_e <= n-1 (max possible position difference)
            constraints.push(LinearConstraint::le(vec![(z_idx(e), 1)], n_i64 - 1));
        }

        // Objective: minimize sum z_e
        let objective: Vec<(usize, i64)> = (0..m).map(|e| (z_idx(e), 1)).collect();
        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize)
            .map_err(<Self as ReduceTo<ILP<i64>>>::target_construction)?;

        Ok(ReductionOLAToILP {
            target,
            num_vertices: n,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "optimallineararrangement_to_ilp",
        build: || {
            // Path P4: 0-1-2-3 (identity permutation achieves cost 3)
            let source =
                OptimalLinearArrangement::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]));
            crate::example_db::specs::rule_example_via_ilp::<_, i64>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/optimallineararrangement_ilp.rs"]
mod tests;
