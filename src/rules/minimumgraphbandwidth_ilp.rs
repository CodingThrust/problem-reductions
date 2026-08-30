//! Reduction from MinimumGraphBandwidth to ILP (Integer Linear Programming).
//!
//! Position-assignment formulation with bandwidth variable:
//! - Binary x_{v,p}: vertex v gets position p
//! - Integer position variables pos_v = sum_p p * x_{v,p}
//! - Integer bandwidth variable B
//! - For each edge (u,v): pos_u - pos_v <= B, pos_v - pos_u <= B
//! - Objective: minimize B

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::MinimumGraphBandwidth;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing MinimumGraphBandwidth to ILP.
///
/// Variable layout (`ILP<i64>`, non-negative integers):
/// - `x_{v,p}` at index `v * n + p`, bounded to {0,1}
/// - `pos_v` at index `n^2 + v`, integer position in {0, ..., n-1}
/// - `B` (bandwidth) at index `n^2 + n`
#[derive(Debug, Clone)]
pub struct ReductionMGBToILP {
    target: ILP<i64>,
    num_vertices: usize,
}

impl ReductionResult for ReductionMGBToILP {
    type Source = MinimumGraphBandwidth<SimpleGraph>;
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
        num_vars = "num_vertices^2 + num_vertices + 1",
        num_constraints = "2 * num_vertices + num_vertices^2 + num_vertices + num_vertices + 1 + 2 * num_edges",
    },
    unavailable = {
        num_nonzeros = "the exact target parameter is not represented by this reduction's symbolic transform",
    }
)]
impl ReduceTo<ILP<i64>> for MinimumGraphBandwidth<SimpleGraph> {
    type Result = ReductionMGBToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_vertices();
        let graph = self.graph();
        let edges = graph.edges();

        let num_x = n * n;
        let num_vars = num_x + n + 1;

        let x_idx = |v: usize, p: usize| -> usize { v * n + p };
        let pos_idx = |v: usize| -> usize { num_x + v };
        let b_idx = num_x + n;

        let mut constraints = Vec::new();
        let n_i64 = Self::exact_i64(n, "encoding a vertex position")?;

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

        // Position variable linking: pos_v = sum_p p * x_{v,p}
        for v in 0..n {
            let mut terms: Vec<(usize, i64)> = vec![(pos_idx(v), 1)];
            for p in 0..n {
                terms.push((
                    x_idx(v, p),
                    -Self::exact_i64(p, "encoding a vertex position")?,
                ));
            }
            constraints.push(LinearConstraint::eq(terms, 0));
        }

        // Position bounds: 0 <= pos_v <= n-1
        for v in 0..n {
            constraints.push(LinearConstraint::le(vec![(pos_idx(v), 1)], n_i64 - 1));
        }

        // Bandwidth upper bound: B <= n-1 (max possible position difference)
        constraints.push(LinearConstraint::le(vec![(b_idx, 1)], n_i64 - 1));

        // Bandwidth constraints: for each edge (u,v):
        //   pos_u - pos_v <= B  =>  pos_u - pos_v - B <= 0
        //   pos_v - pos_u <= B  =>  pos_v - pos_u - B <= 0
        for &(u, v) in edges.iter() {
            constraints.push(LinearConstraint::le(
                vec![(pos_idx(u), 1), (pos_idx(v), -1), (b_idx, -1)],
                0,
            ));
            constraints.push(LinearConstraint::le(
                vec![(pos_idx(v), 1), (pos_idx(u), -1), (b_idx, -1)],
                0,
            ));
        }

        // Objective: minimize B
        let objective = vec![(b_idx, 1.0)];
        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize)
            .map_err(Self::target_construction)?;

        Ok(ReductionMGBToILP {
            target,
            num_vertices: n,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimumgraphbandwidth_to_ilp",
        build: || {
            // Star S4: center 0 connected to 1, 2, 3
            let source =
                MinimumGraphBandwidth::new(SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3)]));
            crate::example_db::specs::rule_example_via_ilp::<_, i64>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumgraphbandwidth_ilp.rs"]
mod tests;
