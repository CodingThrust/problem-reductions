//! Reduction from StrongConnectivityAugmentation to ILP<i32>.
//!
//! Select candidate arcs under the budget and certify strong connectivity by
//! sending flow both from a root to every vertex and back again.
//! See the paper entry for the full formulation.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::StrongConnectivityAugmentation;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionSCAToILP {
    target: ILP<i32>,
    num_candidates: usize,
}

impl ReductionResult for ReductionSCAToILP {
    type Source = StrongConnectivityAugmentation<i32>;
    type Target = ILP<i32>;

    fn target_problem(&self) -> &ILP<i32> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution[..self.num_candidates].to_vec()
    }
}

#[reduction(
    overhead = {
        num_vars = "num_potential_arcs + 2 * num_vertices * (num_arcs + num_potential_arcs)",
        num_constraints = "1 + 2 * num_vertices * num_potential_arcs + 2 * num_vertices * num_vertices",
    }
)]
impl ReduceTo<ILP<i32>> for StrongConnectivityAugmentation<i32> {
    type Result = ReductionSCAToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let p = self.num_potential_arcs();

        // Trivial: n ≤ 1 already strongly connected
        if n <= 1 {
            let target = ILP::new(p, vec![], vec![], ObjectiveSense::Minimize);
            return ReductionSCAToILP {
                target,
                num_candidates: p,
            };
        }

        let base_arcs = self.graph().arcs();
        let m = base_arcs.len();
        let root = 0;

        // Variable layout per paper:
        // y_j:              j                          [0, p)
        // f^t_i (fwd base): p + t*m + i                [p, p + n*m)
        // f_bar^t_j (fwd cand): p + n*m + t*p + j      [p+nm, p+nm+np)
        // g^t_i (bwd base): p + n*(m+p) + t*m + i      [p+n(m+p), p+n(2m+p))
        // g_bar^t_j (bwd cand): p + n*(2m+p) + t*p + j [p+n(2m+p), p+2n(m+p))
        let num_vars = p + 2 * n * (m + p);
        let f_base = |t: usize, i: usize| -> usize { p + t * m + i };
        let f_cand = |t: usize, j: usize| -> usize { p + n * m + t * p + j };
        let g_base = |t: usize, i: usize| -> usize { p + n * (m + p) + t * m + i };
        let g_cand = |t: usize, j: usize| -> usize { p + n * (2 * m + p) + t * p + j };

        let mut constraints = Vec::new();

        // Binary bounds: y_j ≤ 1
        for j in 0..p {
            constraints.push(LinearConstraint::le(vec![(j, 1.0)], 1.0));
        }

        // Budget: Σ w_j * y_j ≤ B
        let budget_terms: Vec<(usize, f64)> = self
            .candidate_arcs()
            .iter()
            .enumerate()
            .map(|(j, &(_, _, w))| (j, w as f64))
            .collect();
        constraints.push(LinearConstraint::le(budget_terms, *self.bound() as f64));

        for t in 0..n {
            if t == root {
                // Pin all flow vars to 0 for dummy commodity t = root
                for i in 0..m {
                    constraints.push(LinearConstraint::eq(vec![(f_base(t, i), 1.0)], 0.0));
                    constraints.push(LinearConstraint::eq(vec![(g_base(t, i), 1.0)], 0.0));
                }
                for j in 0..p {
                    constraints.push(LinearConstraint::eq(vec![(f_cand(t, j), 1.0)], 0.0));
                    constraints.push(LinearConstraint::eq(vec![(g_cand(t, j), 1.0)], 0.0));
                }
                continue;
            }

            // Activation: f_bar^t_j ≤ y_j and g_bar^t_j ≤ y_j
            for j in 0..p {
                constraints.push(LinearConstraint::le(
                    vec![(f_cand(t, j), 1.0), (j, -1.0)],
                    0.0,
                ));
                constraints.push(LinearConstraint::le(
                    vec![(g_cand(t, j), 1.0), (j, -1.0)],
                    0.0,
                ));
            }

            // Forward flow conservation (root → t): for each vertex v
            for v in 0..n {
                let mut terms: Vec<(usize, f64)> = Vec::new();

                // Base arcs
                for (i, &(u_a, v_a)) in base_arcs.iter().enumerate() {
                    if u_a == v {
                        terms.push((f_base(t, i), 1.0)); // outgoing
                    }
                    if v_a == v {
                        terms.push((f_base(t, i), -1.0)); // incoming
                    }
                }

                // Candidate arcs
                for (j, &(sj, tj, _)) in self.candidate_arcs().iter().enumerate() {
                    if sj == v {
                        terms.push((f_cand(t, j), 1.0)); // outgoing
                    }
                    if tj == v {
                        terms.push((f_cand(t, j), -1.0)); // incoming
                    }
                }

                let rhs = if v == root {
                    1.0
                } else if v == t {
                    -1.0
                } else {
                    0.0
                };
                constraints.push(LinearConstraint::eq(terms, rhs));
            }

            // Backward flow conservation (t → root): for each vertex v
            for v in 0..n {
                let mut terms: Vec<(usize, f64)> = Vec::new();

                // Base arcs
                for (i, &(u_a, v_a)) in base_arcs.iter().enumerate() {
                    if u_a == v {
                        terms.push((g_base(t, i), 1.0));
                    }
                    if v_a == v {
                        terms.push((g_base(t, i), -1.0));
                    }
                }

                // Candidate arcs
                for (j, &(sj, tj, _)) in self.candidate_arcs().iter().enumerate() {
                    if sj == v {
                        terms.push((g_cand(t, j), 1.0));
                    }
                    if tj == v {
                        terms.push((g_cand(t, j), -1.0));
                    }
                }

                let rhs = if v == t {
                    1.0 // source of backward flow
                } else if v == root {
                    -1.0 // sink of backward flow
                } else {
                    0.0
                };
                constraints.push(LinearConstraint::eq(terms, rhs));
            }
        }

        let target = ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize);
        ReductionSCAToILP {
            target,
            num_candidates: p,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::topology::DirectedGraph;
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "strongconnectivityaugmentation_to_ilp",
        build: || {
            // Path 0→1→2, candidates: (2,0,1),(1,0,2), bound=2
            let source = StrongConnectivityAugmentation::new(
                DirectedGraph::new(3, vec![(0, 1), (1, 2)]),
                vec![(2, 0, 1), (1, 0, 2)],
                2,
            );
            let reduction: ReductionSCAToILP =
                crate::rules::ReduceTo::<ILP<i32>>::reduce_to(&source);
            let ilp_sol = crate::solvers::ILPSolver::new()
                .solve(reduction.target_problem())
                .expect("ILP should be solvable");
            let extracted = reduction.extract_solution(&ilp_sol);
            crate::example_db::specs::rule_example_with_witness::<_, ILP<i32>>(
                source,
                SolutionPair {
                    source_config: extracted,
                    target_config: ilp_sol,
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/strongconnectivityaugmentation_ilp.rs"]
mod tests;
