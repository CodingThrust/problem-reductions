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
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let num_groups = self.num_groups;
        (0..self.num_vertices)
            .map(|v| {
                (0..num_groups)
                    .find(|&g| {
                        let idx = v * num_groups + g;
                        idx < target_solution.len() && target_solution[idx] == 1
                    })
                    .unwrap_or(0)
            })
            .collect()
    }
}

#[reduction(
    overhead = {
        num_vars = "num_vertices^2 + num_edges * num_vertices",
        num_constraints = "num_vertices^2 + num_edges * num_vertices + num_vertices",
    }
)]
impl ReduceTo<ILP<bool>> for PartitionIntoPathsOfLength2<SimpleGraph> {
    type Result = ReductionPIPL2ToILP;

    fn reduce_to(&self) -> Self::Result {
        let num_vertices = self.num_vertices();
        let q = self.num_groups();
        let edges: Vec<(usize, usize)> = self.graph().edges();
        let num_edges = edges.len();
        let num_vars = num_vertices * q + num_edges * q;

        let mut constraints = Vec::new();

        // Assignment constraints: for each vertex v, Σ_g x_{v,g} = 1
        for v in 0..num_vertices {
            let terms: Vec<(usize, f64)> = (0..q).map(|g| (v * q + g, 1.0)).collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // Group size constraints: for each group g, Σ_v x_{v,g} = 3
        for g in 0..q {
            let terms: Vec<(usize, f64)> = (0..num_vertices).map(|v| (v * q + g, 1.0)).collect();
            constraints.push(LinearConstraint::eq(terms, 3.0));
        }

        // McCormick linearization: y_{e,g} = x_{u,g} * x_{v,g} for each edge e=(u,v) and group g
        // y_{e,g} is at index num_vertices * q + e * q + g
        for (e, &(u, v)) in edges.iter().enumerate() {
            for g in 0..q {
                let y = num_vertices * q + e * q + g;
                let xu = u * q + g;
                let xv = v * q + g;

                // y ≤ x_{u,g}
                constraints.push(LinearConstraint::le(vec![(y, 1.0), (xu, -1.0)], 0.0));
                // y ≤ x_{v,g}
                constraints.push(LinearConstraint::le(vec![(y, 1.0), (xv, -1.0)], 0.0));
                // y ≥ x_{u,g} + x_{v,g} - 1  →  -y + x_{u,g} + x_{v,g} ≤ 1
                constraints.push(LinearConstraint::le(
                    vec![(y, -1.0), (xu, 1.0), (xv, 1.0)],
                    1.0,
                ));
            }
        }

        // At-least-2-edges constraint: for each group g, Σ_e y_{e,g} ≥ 2
        for g in 0..q {
            let terms: Vec<(usize, f64)> = (0..num_edges)
                .map(|e| (num_vertices * q + e * q + g, 1.0))
                .collect();
            constraints.push(LinearConstraint::ge(terms, 2.0));
        }

        let target = ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize);

        ReductionPIPL2ToILP {
            target,
            num_vertices,
            num_groups: q,
        }
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
