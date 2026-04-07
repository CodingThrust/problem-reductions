//! Reduction from MinimumCoveringByCliques to ILP.
//!
//! We use one potential clique slot per source edge, matching the source-model
//! encoding where each edge is assigned a group label in `[0, |E|)`.
//!
//! Variables:
//! - `x_(v,k)`: vertex `v` is selected into clique slot `k`
//! - `z_k`: clique slot `k` is active
//! - `y_(e,k)`: edge `e = {u,v}` is covered by slot `k`, linearized as
//!   `x_(u,k) * x_(v,k)`
//!
//! Constraints:
//! - Non-edges cannot appear together in the same clique slot
//! - `x_(v,k) <= z_k`
//! - Every edge is covered by at least one clique slot
//! - McCormick constraints enforce `y_(e,k) = x_(u,k) * x_(v,k)`
//!
//! Objective: minimize the number of active clique slots.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::MinimumCoveringByCliques;
use crate::reduction;
use crate::rules::ilp_helpers::mccormick_product;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

#[derive(Debug, Clone)]
pub struct ReductionMinimumCoveringByCliquesToILP {
    target: ILP<bool>,
    num_edges: usize,
    y_offset: usize,
}

impl ReductionResult for ReductionMinimumCoveringByCliquesToILP {
    type Source = MinimumCoveringByCliques<SimpleGraph>;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        if self.num_edges == 0 {
            return vec![];
        }

        (0..self.num_edges)
            .map(|edge_idx| {
                (0..self.num_edges)
                    .find(|&slot| {
                        target_solution[self.y_offset + edge_idx * self.num_edges + slot] == 1
                    })
                    .unwrap_or(0)
            })
            .collect()
    }
}

#[reduction(
    overhead = {
        num_vars = "num_vertices * num_edges + num_edges + num_edges * num_edges",
        num_constraints = "num_vertices * num_edges + (num_vertices * (num_vertices - 1) / 2 - num_edges) * num_edges + 3 * num_edges * num_edges + num_edges",
    }
)]
impl ReduceTo<ILP<bool>> for MinimumCoveringByCliques<SimpleGraph> {
    type Result = ReductionMinimumCoveringByCliquesToILP;

    fn reduce_to(&self) -> Self::Result {
        let graph = self.graph();
        let num_vertices = graph.num_vertices();
        let edges = graph.edges();
        let num_edges = edges.len();
        let num_slots = num_edges;

        let x_idx = |vertex: usize, slot: usize| -> usize { vertex * num_slots + slot };
        let z_offset = num_vertices * num_slots;
        let z_idx = |slot: usize| -> usize { z_offset + slot };
        let y_offset = z_offset + num_slots;
        let y_idx =
            |edge_idx: usize, slot: usize| -> usize { y_offset + edge_idx * num_slots + slot };

        let mut constraints = Vec::new();

        for slot in 0..num_slots {
            for u in 0..num_vertices {
                constraints.push(LinearConstraint::le(
                    vec![(x_idx(u, slot), 1.0), (z_idx(slot), -1.0)],
                    0.0,
                ));
            }
        }

        for slot in 0..num_slots {
            for u in 0..num_vertices {
                for v in (u + 1)..num_vertices {
                    if !graph.has_edge(u, v) {
                        constraints.push(LinearConstraint::le(
                            vec![(x_idx(u, slot), 1.0), (x_idx(v, slot), 1.0)],
                            1.0,
                        ));
                    }
                }
            }
        }

        for (edge_idx, &(u, v)) in edges.iter().enumerate() {
            for slot in 0..num_slots {
                constraints.extend(mccormick_product(
                    y_idx(edge_idx, slot),
                    x_idx(u, slot),
                    x_idx(v, slot),
                ));
            }
        }

        for edge_idx in 0..num_edges {
            let terms: Vec<(usize, f64)> = (0..num_slots)
                .map(|slot| (y_idx(edge_idx, slot), 1.0))
                .collect();
            constraints.push(LinearConstraint::ge(terms, 1.0));
        }

        let objective: Vec<(usize, f64)> = (0..num_slots).map(|slot| (z_idx(slot), 1.0)).collect();
        let target = ILP::new(
            y_offset + num_edges * num_slots,
            constraints,
            objective,
            ObjectiveSense::Minimize,
        );

        ReductionMinimumCoveringByCliquesToILP {
            target,
            num_edges,
            y_offset,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimumcoveringbycliques_to_ilp",
        build: || {
            let source = MinimumCoveringByCliques::new(SimpleGraph::new(
                4,
                vec![(0, 1), (0, 2), (0, 3), (1, 2), (2, 3)],
            ));
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumcoveringbycliques_ilp.rs"]
mod tests;
