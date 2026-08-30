//! Reduction from TravelingSalesman to ILP (Integer Linear Programming).
//!
//! Uses position-based variables x_{v,k} with McCormick linearization.
//! - Variables: x_{v,k} for vertex v at position k (binary), plus auxiliary y variables
//! - Constraints: assignment, non-edge consecutive, McCormick
//! - Objective: minimize total edge weight of the tour

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::TravelingSalesman;
use crate::reduction;
use crate::rules::ilp_helpers::{mccormick_product, one_hot_decode};
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::types::i64_to_exact_f64;

/// Result of reducing TravelingSalesman to ILP.
#[derive(Debug, Clone)]
pub struct ReductionTSPToILP {
    target: ILP<bool>,
    /// Number of vertices in the source graph.
    num_vertices: usize,
    /// Edges of the source graph (for solution extraction).
    source_edges: Vec<(usize, usize)>,
}

impl ReductionResult for ReductionTSPToILP {
    type Source = TravelingSalesman<SimpleGraph, i64>;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    /// Extract solution: read tour permutation from x variables,
    /// then map to edge selection for the source problem.
    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok({
            let n = self.num_vertices;

            let tour = one_hot_decode(target_solution, n, n, 0)?;

            // Map tour to edge selection
            let mut edge_selection = vec![false; self.source_edges.len()];
            for k in 0..n {
                let u = tour[k];
                let v = tour[(k + 1) % n];
                let edge = self
                    .source_edges
                    .iter()
                    .position(|&(a, b)| (a == u && b == v) || (a == v && b == u))
                    .ok_or_else(|| {
                        crate::rules::ExtractionError::invalid(format!(
                            "target tour uses absent source edge ({u}, {v})"
                        ))
                    })?;
                edge_selection[edge] = true;
            }

            edge_selection
        })
    }
}

#[reduction(
    transform = exact {
        num_vars = "num_vertices^2 + 2 * num_vertices * num_edges",
        num_constraints = "num_vertices^3 + -1 * num_vertices^2 + 2 * num_vertices + 4 * num_vertices * num_edges",
    },
    unavailable = {
        num_nonzeros = "the exact target parameter is not represented by this reduction's symbolic transform",
    }
)]
impl ReduceTo<ILP<bool>> for TravelingSalesman<SimpleGraph, i64> {
    type Result = ReductionTSPToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.graph().num_vertices();
        let graph = self.graph();
        let edges_with_weights = self.edges();
        let source_edges: Vec<(usize, usize)> =
            edges_with_weights.iter().map(|&(u, v, _)| (u, v)).collect();
        let edge_weights: Vec<f64> = edges_with_weights
            .iter()
            .map(|&(_, _, w)| {
                i64_to_exact_f64(w).map_err(|error| {
                    crate::rules::ReductionError::inexact_float_conversion::<
                        TravelingSalesman<SimpleGraph, i64>,
                        ILP<bool>,
                    >(error)
                })
            })
            .collect::<Result<_, _>>()?;
        let m = source_edges.len();

        // Variable layout:
        // [0, n²): x_{v,k} = vertex v at position k
        // [n², n² + 2mn): auxiliary y variables for McCormick linearization
        //   For edge_idx e and position k:
        //     y_{forward} at n² + e * 2n + 2k      (x_{u,k} * x_{v,(k+1)%n})
        //     y_{reverse} at n² + e * 2n + 2k + 1  (x_{v,k} * x_{u,(k+1)%n})
        let num_x = n * n;
        let num_y = 2 * m * n;
        let num_vars = num_x + num_y;

        let x_idx = |v: usize, k: usize| -> usize { v * n + k };
        let y_idx =
            |edge: usize, k: usize, dir: usize| -> usize { num_x + edge * 2 * n + 2 * k + dir };

        let mut constraints = Vec::new();

        // Constraint 1: Each vertex has exactly one position
        for v in 0..n {
            let terms: Vec<(usize, i64)> = (0..n).map(|k| (x_idx(v, k), 1)).collect();
            constraints.push(LinearConstraint::eq(terms, 1));
        }

        // Constraint 2: Each position has exactly one vertex
        for k in 0..n {
            let terms: Vec<(usize, i64)> = (0..n).map(|v| (x_idx(v, k), 1)).collect();
            constraints.push(LinearConstraint::eq(terms, 1));
        }

        // Constraint 3: Non-edge consecutive prohibition
        // For each ordered pair (v, w) where {v, w} ∉ E and v ≠ w:
        //   x_{v,k} + x_{w,(k+1) mod n} <= 1 for all k
        for v in 0..n {
            for w in 0..n {
                if v == w {
                    continue;
                }
                if graph.has_edge(v, w) {
                    continue;
                }
                for k in 0..n {
                    constraints.push(LinearConstraint::le(
                        vec![(x_idx(v, k), 1), (x_idx(w, (k + 1) % n), 1)],
                        1,
                    ));
                }
            }
        }

        // Constraint 4: McCormick linearization for auxiliary variables
        // For each edge (u, v) at index e:
        //   Forward (dir=0): y = x_{u,k} * x_{v,(k+1)%n}
        //   Reverse (dir=1): y = x_{v,k} * x_{u,(k+1)%n}
        for (e, &(u, v)) in source_edges.iter().enumerate() {
            for k in 0..n {
                let k_next = (k + 1) % n;

                // Forward: y_{e,k,0} = x_{u,k} * x_{v,k_next}
                let y_fwd = y_idx(e, k, 0);
                let xu = x_idx(u, k);
                let xv_next = x_idx(v, k_next);
                constraints.extend(mccormick_product(y_fwd, xu, xv_next));

                // Reverse: y_{e,k,1} = x_{v,k} * x_{u,k_next}
                let y_rev = y_idx(e, k, 1);
                let xv = x_idx(v, k);
                let xu_next = x_idx(u, k_next);
                constraints.extend(mccormick_product(y_rev, xv, xu_next));
            }
        }

        // Objective: minimize Σ_{e=(u,v)} w_e * Σ_k (y_{e,k,0} + y_{e,k,1})
        let mut objective: Vec<(usize, f64)> = Vec::new();
        for (e, &w) in edge_weights.iter().enumerate() {
            for k in 0..n {
                objective.push((y_idx(e, k, 0), w));
                objective.push((y_idx(e, k, 1), w));
            }
        }

        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize)
            .map_err(<Self as ReduceTo<ILP<bool>>>::target_construction)?;

        Ok(ReductionTSPToILP {
            target,
            num_vertices: n,
            source_edges,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "travelingsalesman_to_ilp",
        build: || {
            let source = TravelingSalesman::new(
                SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]),
                vec![10, 15, 20, 35, 25, 30],
            );
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/travelingsalesman_ilp.rs"]
mod tests;
