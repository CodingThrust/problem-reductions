//! Reduction from StrongConnectivityAugmentation to `ILP<i64>`.
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
    target: ILP<i64>,
    num_candidates: usize,
}

impl ReductionResult for ReductionSCAToILP {
    type Source = StrongConnectivityAugmentation<i64>;
    type Target = ILP<i64>;

    fn target_problem(&self) -> &ILP<i64> {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok(target_solution[..self.num_candidates]
            .iter()
            .map(|&value| value == 1)
            .collect())
    }
}

#[reduction(
    transform = exact {
        num_vars = "num_potential_arcs + 2 * num_vertices * (num_arcs + num_potential_arcs)",
        num_constraints = "1 + 2 * num_vertices * num_potential_arcs + 2 * num_vertices * num_vertices",
    },
    unavailable = {
        num_nonzeros = "the exact target parameter is not represented by this reduction's symbolic transform",
    }
)]
impl ReduceTo<ILP<i64>> for StrongConnectivityAugmentation<i64> {
    type Result = ReductionSCAToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_vertices();
        let p = self.num_potential_arcs();

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
            constraints.push(LinearConstraint::le(vec![(j, 1)], 1));
        }

        // Budget: Σ w_j * y_j ≤ B
        let budget_terms: Vec<(usize, i64)> = self
            .candidate_arcs()
            .iter()
            .enumerate()
            .map(|(candidate, &(_, _, weight))| (candidate, weight))
            .collect();
        constraints.push(LinearConstraint::le(budget_terms, *self.bound()));

        for t in 0..n {
            if t == root {
                // Pin all flow vars to 0 for dummy commodity t = root
                for i in 0..m {
                    constraints.push(LinearConstraint::eq(vec![(f_base(t, i), 1)], 0));
                    constraints.push(LinearConstraint::eq(vec![(g_base(t, i), 1)], 0));
                }
                for j in 0..p {
                    constraints.push(LinearConstraint::eq(vec![(f_cand(t, j), 1)], 0));
                    constraints.push(LinearConstraint::eq(vec![(g_cand(t, j), 1)], 0));
                }
                continue;
            }

            // Activation: f_bar^t_j ≤ y_j and g_bar^t_j ≤ y_j
            for j in 0..p {
                constraints.push(LinearConstraint::le(vec![(f_cand(t, j), 1), (j, -1)], 0));
                constraints.push(LinearConstraint::le(vec![(g_cand(t, j), 1), (j, -1)], 0));
            }

            // Forward flow conservation (root → t): for each vertex v
            for v in 0..n {
                let mut terms: Vec<(usize, i64)> = Vec::new();

                // Base arcs
                for (i, &(u_a, v_a)) in base_arcs.iter().enumerate() {
                    if u_a == v {
                        terms.push((f_base(t, i), 1)); // outgoing
                    }
                    if v_a == v {
                        terms.push((f_base(t, i), -1)); // incoming
                    }
                }

                // Candidate arcs
                for (j, &(sj, tj, _)) in self.candidate_arcs().iter().enumerate() {
                    if sj == v {
                        terms.push((f_cand(t, j), 1)); // outgoing
                    }
                    if tj == v {
                        terms.push((f_cand(t, j), -1)); // incoming
                    }
                }

                let rhs = if v == root {
                    1
                } else if v == t {
                    -1
                } else {
                    0
                };
                constraints.push(LinearConstraint::eq(terms, rhs));
            }

            // Backward flow conservation (t → root): for each vertex v
            for v in 0..n {
                let mut terms: Vec<(usize, i64)> = Vec::new();

                // Base arcs
                for (i, &(u_a, v_a)) in base_arcs.iter().enumerate() {
                    if u_a == v {
                        terms.push((g_base(t, i), 1));
                    }
                    if v_a == v {
                        terms.push((g_base(t, i), -1));
                    }
                }

                // Candidate arcs
                for (j, &(sj, tj, _)) in self.candidate_arcs().iter().enumerate() {
                    if sj == v {
                        terms.push((g_cand(t, j), 1));
                    }
                    if tj == v {
                        terms.push((g_cand(t, j), -1));
                    }
                }

                let rhs = if v == t {
                    1 // source of backward flow
                } else if v == root {
                    -1 // sink of backward flow
                } else {
                    0
                };
                constraints.push(LinearConstraint::eq(terms, rhs));
            }
        }

        let target = ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize)
            .map_err(Self::target_construction)?;
        Ok(ReductionSCAToILP {
            target,
            num_candidates: p,
        })
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
                crate::rules::ReduceTo::<ILP<i64>>::reduce_to(&source)
                    .expect("reduction should succeed");
            let ilp_sol = crate::solvers::ILPSolver::new()
                .solve(reduction.target_problem())
                .expect("ILP should be solvable");
            let extracted = reduction.extract_solution(&ilp_sol).unwrap();
            crate::example_db::specs::rule_example_with_witness::<_, ILP<i64>>(
                source,
                SolutionPair {
                    source_config: serde_json::json!(extracted),
                    target_config: serde_json::json!(ilp_sol),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/strongconnectivityaugmentation_ilp.rs"]
mod tests;
