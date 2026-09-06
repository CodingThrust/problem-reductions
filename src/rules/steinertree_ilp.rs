//! Exact Steiner-tree formulation for signed edge weights.
//!
//! Binary vertex selectors and rooted flows connect every selected vertex.
//! Endpoint linking and |selected edges| = |selected vertices| - 1 then enforce
//! a tree, independently of the objective's signs.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::SteinerTree;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Binary layout: m edge selectors, n vertex selectors, then 2m flow arcs
/// for each vertex other than the first terminal (in vertex-index order).
#[derive(Debug, Clone)]
pub struct ReductionSteinerTreeToILP {
    target: ILP<bool>,
    num_edges: usize,
}

impl ReductionResult for ReductionSteinerTreeToILP {
    type Source = SteinerTree<SimpleGraph, i64>;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        if crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?
            .value
            .is_none()
        {
            return Err(crate::rules::ExtractionError::invalid(
                "target ILP assignment is infeasible",
            ));
        }
        Ok(target_solution[..self.num_edges]
            .iter()
            .map(|&value| value == 1)
            .collect())
    }
}

#[reduction(
    transform = exact {
        num_vars = "num_edges + num_vertices + 2 * num_edges * (num_vertices - 1)",
        num_constraints = "num_vertices * (num_vertices - 1) + 2 * num_edges * num_vertices + num_terminals + 1",
    },
    unavailable = {
        num_nonzeros = "the exact target parameter is not represented by this reduction's symbolic transform",
    }
)]
impl ReduceTo<ILP<bool>> for SteinerTree<SimpleGraph, i64> {
    type Result = ReductionSteinerTreeToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_vertices();
        let m = self.num_edges();
        let (num_vars, num_constraints) = tree_ilp_sizes(n, m, self.terminals().len())?;
        // The source constructor requires at least two distinct terminals.
        let root = self.terminals()[0];
        let edges = self.graph().edges();
        let vertex_var = |v: usize| m + v;
        let flow_var = |commodity: usize, edge: usize, dir: usize| {
            m + n + commodity * (2 * m) + 2 * edge + dir
        };
        let mut constraints = Vec::with_capacity(num_constraints);

        for (e, &(u, v)) in edges.iter().enumerate() {
            for endpoint in [u, v] {
                constraints.push(LinearConstraint::le(
                    vec![(e, 1), (vertex_var(endpoint), -1)],
                    0,
                ));
            }
        }
        for &terminal in self.terminals() {
            constraints.push(LinearConstraint::eq(vec![(vertex_var(terminal), 1)], 1));
        }
        let cardinality = (0..m)
            .map(|e| (e, 1))
            .chain((0..n).map(|v| (vertex_var(v), -1)))
            .collect();
        constraints.push(LinearConstraint::eq(cardinality, -1));

        for (commodity, sink) in (0..n).filter(|&v| v != root).enumerate() {
            for vertex in 0..n {
                let mut terms = Vec::new();
                for (edge, &(u, v)) in edges.iter().enumerate() {
                    if vertex == u {
                        terms.push((flow_var(commodity, edge, 0), -1));
                        terms.push((flow_var(commodity, edge, 1), 1));
                    }
                    if vertex == v {
                        terms.push((flow_var(commodity, edge, 0), 1));
                        terms.push((flow_var(commodity, edge, 1), -1));
                    }
                }
                // Inflow - outflow = z_sink at sink and -z_sink at root.
                if vertex == root {
                    terms.push((vertex_var(sink), 1));
                } else if vertex == sink {
                    terms.push((vertex_var(sink), -1));
                }
                constraints.push(LinearConstraint::eq(terms, 0));
            }
            for edge in 0..m {
                for dir in 0..2 {
                    constraints.push(LinearConstraint::le(
                        vec![(flow_var(commodity, edge, dir), 1), (edge, -1)],
                        0,
                    ));
                }
            }
        }
        let objective = self
            .edge_weights()
            .iter()
            .enumerate()
            .map(|(e, &w)| (e, w))
            .collect();
        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize)
            .map_err(Self::target_construction)?;
        Ok(ReductionSteinerTreeToILP {
            target,
            num_edges: m,
        })
    }
}

/// Bounds for all offsets and allocation sizes; n >= 2 is a source invariant.
fn tree_ilp_sizes(
    n: usize,
    m: usize,
    k: usize,
) -> Result<(usize, usize), crate::rules::ReductionError> {
    let overflow = || {
        crate::rules::ReductionError::integer_overflow::<SteinerTree<SimpleGraph, i64>, ILP<bool>>(
            "counting Steiner tree ILP variables and constraints",
        )
    };
    let non_root = n.checked_sub(1).ok_or_else(overflow)?;
    let arcs = m.checked_mul(2).ok_or_else(overflow)?;
    let vars = arcs
        .checked_mul(non_root)
        .and_then(|x| x.checked_add(m))
        .and_then(|x| x.checked_add(n))
        .ok_or_else(overflow)?;
    let rows = n
        .checked_mul(non_root)
        .and_then(|x| arcs.checked_mul(n).and_then(|a| x.checked_add(a)))
        .and_then(|x| x.checked_add(k))
        .and_then(|x| x.checked_add(1))
        .ok_or_else(overflow)?;
    Ok((vars, rows))
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "steinertree_to_ilp",
        build: || {
            let source = SteinerTree::new(
                SimpleGraph::new(
                    5,
                    vec![(0, 1), (1, 2), (1, 3), (3, 4), (0, 3), (3, 2), (2, 4)],
                ),
                vec![2, 2, 1, 1, 5, 5, 6],
                vec![0, 2, 4],
            );
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/steinertree_ilp.rs"]
mod tests;
