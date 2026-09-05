//! Reduction from PartitionIntoPathsOfLength2 to ILP (Integer Linear Programming).
//!
//! Each triple must contain at least 2 edges. We introduce product variables y_{e,g} = x_{u,g} * x_{v,g}
//! for each edge (u,v) and group g, linearized with McCormick constraints:
//!
//! Variables:
//! - x_{v,g}: binary, vertex v in group g (index: v * q + g)
//! - y_{e,g}: binary product for edge e=(u,v) and group g (index: n*q + e * q + g)
//!
//! Constraints:
//! - Σ_g x_{v,g} = 1 for each vertex v (assignment)
//! - Σ_v x_{v,g} = 3 for each group g (size constraint)
//! - For each edge e=(u,v) and group g (McCormick for y_{e,g} = x_{u,g} * x_{v,g}):
//!   y_{e,g} ≤ x_{u,g}, y_{e,g} ≤ x_{v,g}, y_{e,g} ≥ x_{u,g} + x_{v,g} - 1
//! - For each group g: Σ_e y_{e,g} ≥ 2 (at least 2 edges in group)
//!
//! Objective: Minimize 0 (feasibility)

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::PartitionIntoPathsOfLength2;
use crate::reduction;
use crate::rules::ilp_helpers::mccormick_product;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing PartitionIntoPathsOfLength2 to ILP.
///
/// Variable layout:
/// - x_{v,g} at index v * q + g  (v ∈ 0..n, g ∈ 0..q)
/// - y_{e,g} at index n * q + e * q + g  (e ∈ 0..num_edges, g ∈ 0..q)
#[derive(Debug, Clone)]
pub struct ReductionPIPL2ToILP {
    target: ILP<bool>,
    num_vertices: usize,
    num_groups: usize,
}

impl ReductionResult for ReductionPIPL2ToILP {
    type Source = PartitionIntoPathsOfLength2<SimpleGraph>;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    /// Extract solution: for each vertex v, find the unique group g where x_{v,g} = 1.
    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        crate::rules::ilp_helpers::one_hot_decode_rows(
            target_solution,
            self.num_vertices,
            self.num_groups,
            0,
        )
    }
}

#[reduction(
    transform = upper_bound {
        num_vars = "num_vertices^2 + num_edges * num_vertices",
        num_constraints = "num_vertices^2 + num_edges * num_vertices + num_vertices",
    },
    unavailable = {
        num_nonzeros = "the exact target parameter is not represented by this reduction's symbolic transform",
    }
)]
impl ReduceTo<ILP<bool>> for PartitionIntoPathsOfLength2<SimpleGraph> {
    type Result = ReductionPIPL2ToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let num_vertices = self.num_vertices();
        let q = self.num_groups();
        let edges: Vec<(usize, usize)> = self.graph().edges();
        let num_edges = edges.len();
        let num_vars = num_vertices * q + num_edges * q;

        let mut constraints = Vec::new();

        // Assignment constraints: for each vertex v, Σ_g x_{v,g} = 1
        for v in 0..num_vertices {
            let terms: Vec<(usize, i64)> = (0..q).map(|g| (v * q + g, 1)).collect();
            constraints.push(LinearConstraint::eq(terms, 1));
        }

        // Group size constraints: for each group g, Σ_v x_{v,g} = 3
        for g in 0..q {
            let terms: Vec<(usize, i64)> = (0..num_vertices).map(|v| (v * q + g, 1)).collect();
            constraints.push(LinearConstraint::eq(terms, 3));
        }

        // McCormick linearization: y_{e,g} = x_{u,g} * x_{v,g} for each edge e=(u,v) and group g
        // y_{e,g} is at index num_vertices * q + e * q + g
        for (e, &(u, v)) in edges.iter().enumerate() {
            for g in 0..q {
                let y = num_vertices * q + e * q + g;
                let xu = u * q + g;
                let xv = v * q + g;

                constraints.extend(mccormick_product(y, xu, xv));
            }
        }

        // At-least-2-edges constraint: for each group g, Σ_e y_{e,g} ≥ 2
        for g in 0..q {
            let terms: Vec<(usize, i64)> = (0..num_edges)
                .map(|e| (num_vertices * q + e * q + g, 1))
                .collect();
            constraints.push(LinearConstraint::ge(terms, 2));
        }

        let target = ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize)
            .map_err(<Self as ReduceTo<ILP<bool>>>::target_construction)?;

        Ok(ReductionPIPL2ToILP {
            target,
            num_vertices,
            num_groups: q,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "partitionintopathsoflength2_to_ilp",
        build: || {
            // Two P3 paths: 0-1-2 and 3-4-5
            let source = PartitionIntoPathsOfLength2::new(SimpleGraph::new(
                6,
                vec![(0, 1), (1, 2), (3, 4), (4, 5)],
            ));
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/partitionintopathsoflength2_ilp.rs"]
mod tests;
