//! Reduction from MixedChinesePostman to ILP.
//!
//! Choose an orientation for every undirected edge, then add integer traversal
//! variables on available directed arcs to balance the oriented multigraph
//! within the length bound. Uses connectivity flow constraints on both
//! forward and reverse directions.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::MixedChinesePostman;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::types::WeightElement;

/// Result of reducing MixedChinesePostman to ILP.
#[derive(Debug, Clone)]
pub struct ReductionMCPToILP {
    target: ILP<i32>,
    num_undirected_edges: usize,
}

impl ReductionResult for ReductionMCPToILP {
    type Source = MixedChinesePostman<i32>;
    type Target = ILP<i32>;

    fn target_problem(&self) -> &ILP<i32> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        // Return the orientation bits d_k in source edge order
        target_solution[..self.num_undirected_edges].to_vec()
    }
}

#[reduction(
    overhead = {
        num_vars = "num_edges + 4 * (num_arcs + 2 * num_edges) + 3 * num_vertices + 1",
        num_constraints = "num_vertices + 2 * (num_arcs + 2 * num_edges) + 2 * (num_arcs + 2 * num_edges) + num_vertices + 1 + num_vertices + 4 * num_vertices + 2 * (num_arcs + 2 * num_edges) + 2 * num_vertices",
    }
)]
impl ReduceTo<ILP<i32>> for MixedChinesePostman<i32> {
    type Result = ReductionMCPToILP;

    #[allow(clippy::needless_range_loop)]
    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let m = self.num_arcs(); // original directed arcs
        let q = self.num_edges(); // undirected edges
        let r_count = m + q; // required traversals

        // If R = 0, empty walk is feasible
        if r_count == 0 {
            return ReductionMCPToILP {
                target: ILP::new(0, vec![], vec![], ObjectiveSense::Minimize),
                num_undirected_edges: 0,
            };
        }

        // Available arc list A*: L = m + 2q arcs
        // b_i = a_i for 0 <= i < m
        // b_{m+2k} = (u_k, v_k), b_{m+2k+1} = (v_k, u_k)
        let original_arcs = self.graph().arcs();
        let undirected_edges = self.graph().edges();

        let l = m + 2 * q; // total available arcs

        // Build available arc list with lengths
        let mut avail_arcs: Vec<(usize, usize)> = Vec::with_capacity(l);
        let mut avail_lengths: Vec<f64> = Vec::with_capacity(l);

        for (i, &(u, v)) in original_arcs.iter().enumerate() {
            avail_arcs.push((u, v));
            avail_lengths.push(self.arc_weights()[i].to_sum() as f64);
        }
        for (k, &(u, v)) in undirected_edges.iter().enumerate() {
            avail_arcs.push((u, v)); // forward
            avail_lengths.push(self.edge_weights()[k].to_sum() as f64);
            avail_arcs.push((v, u)); // reverse
            avail_lengths.push(self.edge_weights()[k].to_sum() as f64);
        }

        // Variable layout (from paper):
        // d_k: index k (0..q) -- orientation bit
        // g_j: index q + j (0..L) -- extra traversals
        // y_j: index q + L + j -- binary use indicator
        // z_v: index q + 2L + v -- binary vertex activity
        // rho_v: index q + 2L + n + v -- root selector
        // s: index q + 2L + 2n -- count of active vertices
        // b_v: index q + 2L + 2n + 1 + v -- product s*rho_v
        // f_j: index q + 2L + 3n + 1 + j -- forward connectivity flow
        // h_j: index q + 3L + 3n + 1 + j -- reverse connectivity flow

        let d_idx = |k: usize| k;
        let g_idx = |j: usize| q + j;
        let y_idx = |j: usize| q + l + j;
        let z_idx = |v: usize| q + 2 * l + v;
        let rho_idx = |v: usize| q + 2 * l + n + v;
        let s_idx = q + 2 * l + 2 * n;
        let b_idx = |v: usize| q + 2 * l + 2 * n + 1 + v;
        let f_idx = |j: usize| q + 2 * l + 3 * n + 1 + j;
        let h_idx = |j: usize| q + 3 * l + 3 * n + 1 + j;

        let num_vars = q + 4 * l + 3 * n + 1;
        let big_g = (r_count * (n - 1)) as f64; // G = R(n-1)
        let m_use = 1.0 + big_g; // M_use = 1 + G
        let n_f64 = n as f64;

        let mut constraints = Vec::new();

        // Binary bounds for d_k: 0 <= d_k <= 1
        for k in 0..q {
            constraints.push(LinearConstraint::le(vec![(d_idx(k), 1.0)], 1.0));
        }

        // Bounds on g_j: 0 <= g_j <= G
        for j in 0..l {
            constraints.push(LinearConstraint::le(vec![(g_idx(j), 1.0)], big_g));
        }

        // Binary bounds: y_j, z_v, rho_v <= 1
        for j in 0..l {
            constraints.push(LinearConstraint::le(vec![(y_idx(j), 1.0)], 1.0));
        }
        for v in 0..n {
            constraints.push(LinearConstraint::le(vec![(z_idx(v), 1.0)], 1.0));
            constraints.push(LinearConstraint::le(vec![(rho_idx(v), 1.0)], 1.0));
        }

        // The required multiplicity r_j(d):
        // For original arcs (0 <= j < m): r_j = 1 (constant)
        // For edge k forward (j = m + 2k): r_j = 1 - d_k
        // For edge k reverse (j = m + 2k + 1): r_j = d_k

        // Balance constraints:
        // sum_{j: tail_j = v} (r_j + g_j) - sum_{j: head_j = v} (r_j + g_j) = 0 for all v
        for v in 0..n {
            let mut terms = Vec::new();
            let mut constant = 0.0_f64; // constant part of r_j

            for j in 0..l {
                let (tail, head) = avail_arcs[j];
                let sign = if tail == v && head == v {
                    0.0 // self-loop contributes nothing
                } else if tail == v {
                    1.0
                } else if head == v {
                    -1.0
                } else {
                    continue;
                };
                if sign == 0.0 {
                    continue;
                }

                // g_j term
                terms.push((g_idx(j), sign));

                // r_j term
                if j < m {
                    // Original arc: r_j = 1
                    constant += sign;
                } else {
                    let k = (j - m) / 2;
                    if (j - m).is_multiple_of(2) {
                        // Forward: r_j = 1 - d_k => constant += sign, d_k term += -sign
                        constant += sign;
                        terms.push((d_idx(k), -sign));
                    } else {
                        // Reverse: r_j = d_k => d_k term += sign
                        terms.push((d_idx(k), sign));
                    }
                }
            }
            // terms = -constant => 0
            constraints.push(LinearConstraint::eq(terms, -constant));
        }

        // Use indicator: r_j + g_j <= M_use * y_j and y_j <= r_j + g_j
        for j in 0..l {
            if j < m {
                // r_j = 1: (1 + g_j) <= M_use * y_j => g_j - M_use * y_j <= -1
                constraints.push(LinearConstraint::le(
                    vec![(g_idx(j), 1.0), (y_idx(j), -m_use)],
                    -1.0,
                ));
                // y_j <= 1 + g_j => y_j - g_j <= 1
                constraints.push(LinearConstraint::le(
                    vec![(y_idx(j), 1.0), (g_idx(j), -1.0)],
                    1.0,
                ));
            } else {
                let k = (j - m) / 2;
                if (j - m).is_multiple_of(2) {
                    // Forward: r_j = 1 - d_k
                    // (1 - d_k + g_j) <= M_use * y_j => g_j - d_k - M_use * y_j <= -1
                    constraints.push(LinearConstraint::le(
                        vec![(g_idx(j), 1.0), (d_idx(k), -1.0), (y_idx(j), -m_use)],
                        -1.0,
                    ));
                    // y_j <= 1 - d_k + g_j => y_j + d_k - g_j <= 1
                    constraints.push(LinearConstraint::le(
                        vec![(y_idx(j), 1.0), (d_idx(k), 1.0), (g_idx(j), -1.0)],
                        1.0,
                    ));
                } else {
                    // Reverse: r_j = d_k
                    // (d_k + g_j) <= M_use * y_j => d_k + g_j - M_use * y_j <= 0
                    constraints.push(LinearConstraint::le(
                        vec![(d_idx(k), 1.0), (g_idx(j), 1.0), (y_idx(j), -m_use)],
                        0.0,
                    ));
                    // y_j <= d_k + g_j => y_j - d_k - g_j <= 0
                    constraints.push(LinearConstraint::le(
                        vec![(y_idx(j), 1.0), (d_idx(k), -1.0), (g_idx(j), -1.0)],
                        0.0,
                    ));
                }
            }
        }

        // Arc-vertex linking: y_j <= z_{tail_j} and y_j <= z_{head_j}
        for j in 0..l {
            let (tail, head) = avail_arcs[j];
            constraints.push(LinearConstraint::le(
                vec![(y_idx(j), 1.0), (z_idx(tail), -1.0)],
                0.0,
            ));
            constraints.push(LinearConstraint::le(
                vec![(y_idx(j), 1.0), (z_idx(head), -1.0)],
                0.0,
            ));
        }

        // z_v <= sum_{j: tail_j=v or head_j=v} y_j
        for v in 0..n {
            let mut terms = vec![(z_idx(v), 1.0)];
            for j in 0..l {
                let (tail, head) = avail_arcs[j];
                if tail == v || head == v {
                    terms.push((y_idx(j), -1.0));
                }
            }
            constraints.push(LinearConstraint::le(terms, 0.0));
        }

        // s = sum_v z_v
        {
            let mut terms = vec![(s_idx, -1.0)];
            for v in 0..n {
                terms.push((z_idx(v), 1.0));
            }
            constraints.push(LinearConstraint::eq(terms, 0.0));
        }

        // Root selection: sum_v rho_v = 1, rho_v <= z_v
        {
            let terms: Vec<(usize, f64)> = (0..n).map(|v| (rho_idx(v), 1.0)).collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }
        for v in 0..n {
            constraints.push(LinearConstraint::le(
                vec![(rho_idx(v), 1.0), (z_idx(v), -1.0)],
                0.0,
            ));
        }

        // Product linearization: b_v = s * rho_v
        // b_v <= s, b_v <= n * rho_v, b_v >= s - n*(1 - rho_v), b_v >= 0
        for v in 0..n {
            constraints.push(LinearConstraint::le(
                vec![(b_idx(v), 1.0), (s_idx, -1.0)],
                0.0,
            ));
            constraints.push(LinearConstraint::le(
                vec![(b_idx(v), 1.0), (rho_idx(v), -n_f64)],
                0.0,
            ));
            constraints.push(LinearConstraint::ge(
                vec![(b_idx(v), 1.0), (s_idx, -1.0), (rho_idx(v), -n_f64)],
                -n_f64,
            ));
            // b_v >= 0 is implied by ILP<i32> non-negativity
        }

        // Flow bounds: 0 <= f_j, h_j <= (n-1) * y_j
        let flow_big_m = (n as f64) - 1.0;
        for j in 0..l {
            constraints.push(LinearConstraint::le(
                vec![(f_idx(j), 1.0), (y_idx(j), -flow_big_m)],
                0.0,
            ));
            constraints.push(LinearConstraint::le(
                vec![(h_idx(j), 1.0), (y_idx(j), -flow_big_m)],
                0.0,
            ));
        }

        // Forward flow conservation:
        // sum_{j: tail_j=v} f_j - sum_{j: head_j=v} f_j = b_v - z_v for all v
        for v in 0..n {
            let mut terms = Vec::new();
            for j in 0..l {
                let (tail, head) = avail_arcs[j];
                if tail == v {
                    terms.push((f_idx(j), 1.0));
                }
                if head == v {
                    terms.push((f_idx(j), -1.0));
                }
            }
            terms.push((b_idx(v), -1.0));
            terms.push((z_idx(v), 1.0));
            constraints.push(LinearConstraint::eq(terms, 0.0));
        }

        // Reverse flow conservation:
        // sum_{j: head_j=v} h_j - sum_{j: tail_j=v} h_j = b_v - z_v for all v
        for v in 0..n {
            let mut terms = Vec::new();
            for j in 0..l {
                let (tail, head) = avail_arcs[j];
                if head == v {
                    terms.push((h_idx(j), 1.0));
                }
                if tail == v {
                    terms.push((h_idx(j), -1.0));
                }
            }
            terms.push((b_idx(v), -1.0));
            terms.push((z_idx(v), 1.0));
            constraints.push(LinearConstraint::eq(terms, 0.0));
        }

        // Objective: minimize total walk length = sum_j l_j * (r_j + g_j)
        // Expand r_j: for original arcs r_j = 1 (constant), for edge k fwd r_j = 1 - d_k,
        // for edge k rev r_j = d_k.
        // constant part moves out of the objective (ILP ignores additive constants).
        let mut objective = Vec::new();
        for j in 0..l {
            let len_j = avail_lengths[j];
            // g_j term
            objective.push((g_idx(j), len_j));
            // d_k terms from r_j
            if j >= m {
                let k = (j - m) / 2;
                if (j - m).is_multiple_of(2) {
                    // r_j = 1 - d_k => cost contribution -len_j * d_k (constant +len_j ignored)
                    objective.push((d_idx(k), -len_j));
                } else {
                    // r_j = d_k => cost contribution +len_j * d_k
                    objective.push((d_idx(k), len_j));
                }
            }
        }

        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize);

        ReductionMCPToILP {
            target,
            num_undirected_edges: q,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::rules::ReduceTo as _;
    use crate::topology::MixedGraph;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "mixedchinesepostman_to_ilp",
        build: || {
            // Simple instance: 3 vertices, 1 arc, 2 edges
            let source = MixedChinesePostman::new(
                MixedGraph::new(3, vec![(0, 1)], vec![(1, 2), (2, 0)]),
                vec![1],
                vec![1, 1],
            );
            let reduction = ReduceTo::<ILP<i32>>::reduce_to(&source);
            let ilp_solution = crate::solvers::ILPSolver::new()
                .solve(reduction.target_problem())
                .expect("canonical example must be solvable");
            let source_config = reduction.extract_solution(&ilp_solution);
            crate::example_db::specs::rule_example_with_witness::<_, ILP<i32>>(
                source,
                SolutionPair {
                    source_config,
                    target_config: ilp_solution,
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/mixedchinesepostman_ilp.rs"]
mod tests;
