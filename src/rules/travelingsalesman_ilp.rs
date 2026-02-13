//! Reduction from TravelingSalesman to ILP (Integer Linear Programming).
//!
//! Uses position-based variables x_{v,k} with McCormick linearization.
//! - Variables: x_{v,k} for vertex v at position k (binary), plus auxiliary y variables
//! - Constraints: assignment, non-edge consecutive, McCormick
//! - Objective: minimize total edge weight of the tour

use crate::models::graph::TravelingSalesman;
use crate::models::optimization::{LinearConstraint, ObjectiveSense, VarBounds, ILP};
use crate::polynomial::{Monomial, Polynomial};
use crate::reduction;
use crate::rules::registry::ReductionOverhead;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing TravelingSalesman to ILP.
#[derive(Debug, Clone)]
pub struct ReductionTSPToILP {
    target: ILP,
    /// Number of vertices in the source graph.
    num_vertices: usize,
    /// Edges of the source graph (for solution extraction).
    source_edges: Vec<(usize, usize)>,
}

impl ReductionTSPToILP {
    /// Variable index for x_{v,k}: vertex v at position k.
    fn x_index(&self, v: usize, k: usize) -> usize {
        v * self.num_vertices + k
    }
}

impl ReductionResult for ReductionTSPToILP {
    type Source = TravelingSalesman<SimpleGraph, i32>;
    type Target = ILP;

    fn target_problem(&self) -> &ILP {
        &self.target
    }

    /// Extract solution: read tour permutation from x variables,
    /// then map to edge selection for the source problem.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let n = self.num_vertices;

        // Read tour: for each position k, find vertex v with x_{v,k} = 1
        let mut tour = vec![0usize; n];
        for k in 0..n {
            for v in 0..n {
                if target_solution[self.x_index(v, k)] == 1 {
                    tour[k] = v;
                    break;
                }
            }
        }

        // Map tour to edge selection
        let mut edge_selection = vec![0usize; self.source_edges.len()];
        for k in 0..n {
            let u = tour[k];
            let v = tour[(k + 1) % n];
            // Find the edge index for (u, v) or (v, u)
            for (idx, &(a, b)) in self.source_edges.iter().enumerate() {
                if (a == u && b == v) || (a == v && b == u) {
                    edge_selection[idx] = 1;
                    break;
                }
            }
        }

        edge_selection
    }
}

#[reduction(
    overhead = {
        ReductionOverhead::new(vec![
            // num_vars = n^2 + 2*m*n
            ("num_vars", Polynomial::var_pow("num_vertices", 2) + Polynomial {
                terms: vec![Monomial {
                    coefficient: 2.0,
                    variables: vec![("num_vertices", 1), ("num_edges", 1)],
                }]
            }),
            // num_constraints = 2n + n^2*(n-density) + 6mn ≈ n^3 + 6mn
            ("num_constraints", Polynomial::var_pow("num_vertices", 3) + Polynomial {
                terms: vec![Monomial {
                    coefficient: 6.0,
                    variables: vec![("num_vertices", 1), ("num_edges", 1)],
                }]
            }),
        ])
    }
)]
impl ReduceTo<ILP> for TravelingSalesman<SimpleGraph, i32> {
    type Result = ReductionTSPToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let graph = self.graph();
        let edges_with_weights = self.edges();
        let source_edges: Vec<(usize, usize)> = edges_with_weights.iter().map(|&(u, v, _)| (u, v)).collect();
        let edge_weights: Vec<f64> = edges_with_weights.iter().map(|&(_, _, w)| w as f64).collect();
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
        let y_idx = |edge: usize, k: usize, dir: usize| -> usize { num_x + edge * 2 * n + 2 * k + dir };

        let bounds = vec![VarBounds::binary(); num_vars];
        let mut constraints = Vec::new();

        // Constraint 1: Each vertex has exactly one position
        for v in 0..n {
            let terms: Vec<(usize, f64)> = (0..n).map(|k| (x_idx(v, k), 1.0)).collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // Constraint 2: Each position has exactly one vertex
        for k in 0..n {
            let terms: Vec<(usize, f64)> = (0..n).map(|v| (x_idx(v, k), 1.0)).collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
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
                        vec![(x_idx(v, k), 1.0), (x_idx(w, (k + 1) % n), 1.0)],
                        1.0,
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
                constraints.push(LinearConstraint::le(vec![(y_fwd, 1.0), (xu, -1.0)], 0.0));
                constraints.push(LinearConstraint::le(vec![(y_fwd, 1.0), (xv_next, -1.0)], 0.0));
                constraints.push(LinearConstraint::ge(
                    vec![(y_fwd, 1.0), (xu, -1.0), (xv_next, -1.0)],
                    -1.0,
                ));

                // Reverse: y_{e,k,1} = x_{v,k} * x_{u,k_next}
                let y_rev = y_idx(e, k, 1);
                let xv = x_idx(v, k);
                let xu_next = x_idx(u, k_next);
                constraints.push(LinearConstraint::le(vec![(y_rev, 1.0), (xv, -1.0)], 0.0));
                constraints.push(LinearConstraint::le(vec![(y_rev, 1.0), (xu_next, -1.0)], 0.0));
                constraints.push(LinearConstraint::ge(
                    vec![(y_rev, 1.0), (xv, -1.0), (xu_next, -1.0)],
                    -1.0,
                ));
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

        let target = ILP::new(num_vars, bounds, constraints, objective, ObjectiveSense::Minimize);

        ReductionTSPToILP {
            target,
            num_vertices: n,
            source_edges,
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/travelingsalesman_ilp.rs"]
mod tests;
