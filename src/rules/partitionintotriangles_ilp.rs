//! Reduction from PartitionIntoTriangles to ILP (Integer Linear Programming).
//!
//! The Partition Into Triangles problem can be formulated as a binary ILP:
//! - Variables: Binary x_{v,g} (vertex v in group g), one-hot per vertex; q = n/3 groups
//! - Constraints:
//!   - Σ_g x_{v,g} = 1 for each vertex v (assignment)
//!   - Σ_v x_{v,g} = 3 for each group g (exactly 3 vertices per group)
//!   - For each group g and each non-edge (u,v): x_{u,g} + x_{v,g} ≤ 1 (triangle constraint)
//! - Objective: Minimize 0 (feasibility)
//! - Extraction: argmax_g x_{v,g} for each vertex v

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::PartitionIntoTriangles;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing PartitionIntoTriangles to ILP.
///
/// Variable layout: x_{v,g} at index v * q + g.
/// - v ∈ 0..num_vertices, g ∈ 0..q where q = num_vertices / 3
///
/// Total: num_vertices * q = num_vertices^2 / 3 variables.
#[derive(Debug, Clone)]
pub struct ReductionPITToILP {
    target: ILP<bool>,
    num_vertices: usize,
    num_groups: usize,
}

impl ReductionResult for ReductionPITToILP {
    type Source = PartitionIntoTriangles<SimpleGraph>;
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
        num_vars = "num_vertices^2",
        num_constraints = "num_vertices^2 * num_vertices",
    }
)]
impl ReduceTo<ILP<bool>> for PartitionIntoTriangles<SimpleGraph> {
    type Result = ReductionPITToILP;

    fn reduce_to(&self) -> Self::Result {
        let num_vertices = self.num_vertices();
        let q = num_vertices / 3; // number of groups
        let num_vars = num_vertices * q;

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

        // Triangle constraints: for each group g and each non-edge (u,v),
        // x_{u,g} + x_{v,g} ≤ 1
        let graph = self.graph();
        for g in 0..q {
            for u in 0..num_vertices {
                for v in (u + 1)..num_vertices {
                    if !graph.has_edge(u, v) {
                        constraints.push(LinearConstraint::le(
                            vec![(u * q + g, 1.0), (v * q + g, 1.0)],
                            1.0,
                        ));
                    }
                }
            }
        }

        let target = ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize);

        ReductionPITToILP {
            target,
            num_vertices,
            num_groups: q,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
        vec![crate::example_db::specs::RuleExampleSpec {
        id: "partitionintotriangles_to_ilp",
        build: || {
            // Two triangles: 0-1-2 and 3-4-5
            let source = PartitionIntoTriangles::new(SimpleGraph::new(
                6,
                vec![(0, 1), (0, 2), (1, 2), (3, 4), (3, 5), (4, 5)],
            ));
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/partitionintotriangles_ilp.rs"]
mod tests;
