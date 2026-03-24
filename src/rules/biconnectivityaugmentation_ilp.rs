//! Reduction from BiconnectivityAugmentation to ILP<i32>.
//!
//! Select candidate edges under budget and, for every deleted vertex q,
//! certify that the remaining augmented graph stays connected via unit-flow
//! commodities from a surviving root to every other surviving vertex.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::BiconnectivityAugmentation;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

#[derive(Debug, Clone)]
pub struct ReductionBiconnAugToILP {
    target: ILP<i32>,
    num_candidates: usize,
}

impl ReductionResult for ReductionBiconnAugToILP {
    type Source = BiconnectivityAugmentation<SimpleGraph, i32>;
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
        num_vars = "num_potential_edges + 2 * num_vertices * num_vertices * (num_edges + num_potential_edges)",
        num_constraints = "1 + 2 * num_vertices * num_vertices * num_potential_edges + num_vertices * num_vertices * num_vertices",
    }
)]
impl ReduceTo<ILP<i32>> for BiconnectivityAugmentation<SimpleGraph, i32> {
    type Result = ReductionBiconnAugToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let p = self.num_potential_edges();

        // Trivial case: n ≤ 1 already biconnected
        if n <= 1 {
            let target = ILP::new(p, vec![], vec![], ObjectiveSense::Minimize);
            return ReductionBiconnAugToILP {
                target,
                num_candidates: p,
            };
        }

        let base_edges = self.graph().edges();
        let m = base_edges.len();

        // Variable layout:
        // y_j:                j                                      [0, p)
        // f^{q,t}_{i,eta}:   p + ((q*n + t)*m + i)*2 + eta          [p, p + 2*m*n^2)
        // g^{q,t}_{j,eta}:   p + 2*m*n^2 + ((q*n + t)*p + j)*2 + eta  [p + 2*m*n^2, p + 2*n^2*(m+p))
        let num_vars = p + 2 * n * n * (m + p);
        let f_idx = |q: usize, t: usize, i: usize, eta: usize| -> usize {
            p + ((q * n + t) * m + i) * 2 + eta
        };
        let g_idx = |q: usize, t: usize, j: usize, eta: usize| -> usize {
            p + 2 * m * n * n + ((q * n + t) * p + j) * 2 + eta
        };

        let mut constraints = Vec::new();

        // Binary bounds: y_j ≤ 1
        for j in 0..p {
            constraints.push(LinearConstraint::le(vec![(j, 1.0)], 1.0));
        }

        // Budget constraint: Σ w_j y_j ≤ B
        let budget_terms: Vec<(usize, f64)> = self
            .potential_weights()
            .iter()
            .enumerate()
            .map(|(j, &(_, _, w))| (j, w as f64))
            .collect();
        constraints.push(LinearConstraint::le(budget_terms, *self.budget() as f64));

        // For each deleted vertex q
        for q in 0..n {
            let root = if q != 0 { 0 } else { 1 };

            for t in 0..n {
                // Pin trivial commodities to zero
                if t == q || t == root {
                    for i in 0..m {
                        for eta in 0..2 {
                            constraints
                                .push(LinearConstraint::eq(vec![(f_idx(q, t, i, eta), 1.0)], 0.0));
                        }
                    }
                    for j in 0..p {
                        for eta in 0..2 {
                            constraints
                                .push(LinearConstraint::eq(vec![(g_idx(q, t, j, eta), 1.0)], 0.0));
                        }
                    }
                    continue;
                }

                // Pin flows on edges incident to deleted vertex q
                for (i, &(u, v)) in base_edges.iter().enumerate() {
                    if u == q || v == q {
                        for eta in 0..2 {
                            constraints
                                .push(LinearConstraint::eq(vec![(f_idx(q, t, i, eta), 1.0)], 0.0));
                        }
                    }
                }
                for (j, &(sj, tj, _)) in self.potential_weights().iter().enumerate() {
                    if sj == q || tj == q {
                        for eta in 0..2 {
                            constraints
                                .push(LinearConstraint::eq(vec![(g_idx(q, t, j, eta), 1.0)], 0.0));
                        }
                    }
                }

                // Activation: g^{q,t}_{j,eta} ≤ y_j
                for j in 0..p {
                    let &(sj, tj, _) = &self.potential_weights()[j];
                    if sj == q || tj == q {
                        continue; // already pinned to 0
                    }
                    for eta in 0..2 {
                        constraints.push(LinearConstraint::le(
                            vec![(g_idx(q, t, j, eta), 1.0), (j, -1.0)],
                            0.0,
                        ));
                    }
                }

                // Flow conservation for each surviving vertex v ≠ q
                for v in 0..n {
                    if v == q {
                        continue;
                    }
                    let mut terms: Vec<(usize, f64)> = Vec::new();

                    // Base edges
                    for (i, &(u_e, v_e)) in base_edges.iter().enumerate() {
                        if u_e == q || v_e == q {
                            continue;
                        }
                        // eta=0 means u->v direction
                        if u_e == v {
                            terms.push((f_idx(q, t, i, 0), 1.0)); // outgoing
                            terms.push((f_idx(q, t, i, 1), -1.0)); // incoming
                        }
                        if v_e == v {
                            terms.push((f_idx(q, t, i, 0), -1.0)); // incoming
                            terms.push((f_idx(q, t, i, 1), 1.0)); // outgoing
                        }
                    }

                    // Candidate edges
                    for (j, &(sj, tj, _)) in self.potential_weights().iter().enumerate() {
                        if sj == q || tj == q {
                            continue;
                        }
                        // eta=0 means s->t direction
                        if sj == v {
                            terms.push((g_idx(q, t, j, 0), 1.0));
                            terms.push((g_idx(q, t, j, 1), -1.0));
                        }
                        if tj == v {
                            terms.push((g_idx(q, t, j, 0), -1.0));
                            terms.push((g_idx(q, t, j, 1), 1.0));
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
            }
        }

        let target = ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize);
        ReductionBiconnAugToILP {
            target,
            num_candidates: p,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "biconnectivityaugmentation_to_ilp",
        build: || {
            // Path 0-1-2-3, candidates: (0,2,1),(0,3,2),(1,3,1), budget=3
            let source = BiconnectivityAugmentation::new(
                SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
                vec![(0, 2, 1), (0, 3, 2), (1, 3, 1)],
                3,
            );
            let reduction: ReductionBiconnAugToILP =
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
#[path = "../unit_tests/rules/biconnectivityaugmentation_ilp.rs"]
mod tests;
