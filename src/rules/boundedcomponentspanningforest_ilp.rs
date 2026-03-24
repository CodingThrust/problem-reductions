//! Reduction from BoundedComponentSpanningForest to ILP<i32>.
//!
//! Assign every vertex to one of K components, bound weight, certify
//! connectivity inside each used component via flow.
//! See the paper entry for the full formulation.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::BoundedComponentSpanningForest;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

#[derive(Debug, Clone)]
pub struct ReductionBCSFToILP {
    target: ILP<i32>,
    n: usize,
    k: usize,
}

impl ReductionResult for ReductionBCSFToILP {
    type Source = BoundedComponentSpanningForest<SimpleGraph, i32>;
    type Target = ILP<i32>;

    fn target_problem(&self) -> &ILP<i32> {
        &self.target
    }

    /// One-hot decode: for each vertex v, output the unique component c with x_{v,c} = 1.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let n = self.n;
        let k = self.k;
        (0..n)
            .map(|v| {
                (0..k)
                    .find(|&c| target_solution[v * k + c] == 1)
                    .unwrap_or(0)
            })
            .collect()
    }
}

#[reduction(
    overhead = {
        num_vars = "3 * num_vertices * max_components + 2 * max_components + 2 * num_edges * max_components",
        num_constraints = "num_vertices + max_components + max_components + 2 * max_components + num_vertices * max_components + 4 * num_vertices * max_components + 4 * num_edges * max_components + num_vertices * max_components",
    }
)]
impl ReduceTo<ILP<i32>> for BoundedComponentSpanningForest<SimpleGraph, i32> {
    type Result = ReductionBCSFToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let edges = self.graph().edges();
        let m = edges.len();
        let k = self.max_components();

        let x_idx = |v: usize, c: usize| -> usize { v * k + c };
        let u_idx = |c: usize| -> usize { n * k + c };
        let r_idx = |v: usize, c: usize| -> usize { n * k + k + v * k + c };
        let s_idx = |c: usize| -> usize { 2 * n * k + k + c };
        let b_idx = |v: usize, c: usize| -> usize { 2 * n * k + 2 * k + v * k + c };
        let f_idx =
            |i: usize, eta: usize, c: usize| -> usize { 3 * n * k + 2 * k + (i * 2 + eta) * k + c };

        let num_vars = 3 * n * k + 2 * k + 2 * m * k;
        let n_f64 = n as f64;
        let mut constraints = Vec::new();

        // 1) Assignment: sum_c x_{v,c} = 1 for each vertex v
        for v in 0..n {
            let terms: Vec<(usize, f64)> = (0..k).map(|c| (x_idx(v, c), 1.0)).collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // 2) Weight bound: sum_v w_v * x_{v,c} <= B for each component c
        for c in 0..k {
            let terms: Vec<(usize, f64)> = self
                .weights()
                .iter()
                .enumerate()
                .map(|(v, &w)| (x_idx(v, c), w as f64))
                .collect();
            constraints.push(LinearConstraint::le(terms, *self.max_weight() as f64));
        }

        // 3) Size: s_c = sum_v x_{v,c}
        for c in 0..k {
            let mut terms: Vec<(usize, f64)> = vec![(s_idx(c), -1.0)];
            for v in 0..n {
                terms.push((x_idx(v, c), 1.0));
            }
            constraints.push(LinearConstraint::eq(terms, 0.0));
        }

        // 4) Nonempty indicator: u_c <= s_c and s_c <= n * u_c
        for c in 0..k {
            constraints.push(LinearConstraint::le(
                vec![(u_idx(c), 1.0), (s_idx(c), -1.0)],
                0.0,
            ));
            constraints.push(LinearConstraint::le(
                vec![(s_idx(c), 1.0), (u_idx(c), -n_f64)],
                0.0,
            ));
        }

        // 5) Root selection: sum_v r_{v,c} = u_c and r_{v,c} <= x_{v,c}
        for c in 0..k {
            let mut terms: Vec<(usize, f64)> = (0..n).map(|v| (r_idx(v, c), 1.0)).collect();
            terms.push((u_idx(c), -1.0));
            constraints.push(LinearConstraint::eq(terms, 0.0));

            for v in 0..n {
                constraints.push(LinearConstraint::le(
                    vec![(r_idx(v, c), 1.0), (x_idx(v, c), -1.0)],
                    0.0,
                ));
            }
        }

        // 6) Product linearization: b_{v,c} = s_c * r_{v,c}
        for v in 0..n {
            for c in 0..k {
                // b <= s_c
                constraints.push(LinearConstraint::le(
                    vec![(b_idx(v, c), 1.0), (s_idx(c), -1.0)],
                    0.0,
                ));
                // b <= n * r
                constraints.push(LinearConstraint::le(
                    vec![(b_idx(v, c), 1.0), (r_idx(v, c), -n_f64)],
                    0.0,
                ));
                // b >= s - n*(1-r) => b - s - n*r >= -n
                constraints.push(LinearConstraint::ge(
                    vec![(b_idx(v, c), 1.0), (s_idx(c), -1.0), (r_idx(v, c), -n_f64)],
                    -n_f64,
                ));
                // b >= 0
                constraints.push(LinearConstraint::ge(vec![(b_idx(v, c), 1.0)], 0.0));
            }
        }

        // 7) Flow capacity: 0 <= f_{i,eta,c} <= (n-1)*x_{u_i,c} and <= (n-1)*x_{v_i,c}
        let cap = (n as f64) - 1.0;
        for (i, &(u_e, v_e)) in edges.iter().enumerate() {
            for eta in 0..2usize {
                for c in 0..k {
                    constraints.push(LinearConstraint::ge(vec![(f_idx(i, eta, c), 1.0)], 0.0));
                    constraints.push(LinearConstraint::le(
                        vec![(f_idx(i, eta, c), 1.0), (x_idx(u_e, c), -cap)],
                        0.0,
                    ));
                    constraints.push(LinearConstraint::le(
                        vec![(f_idx(i, eta, c), 1.0), (x_idx(v_e, c), -cap)],
                        0.0,
                    ));
                }
            }
        }

        // 8) Flow conservation: net_flow(v,c) = b_{v,c} - x_{v,c}
        for v in 0..n {
            for c in 0..k {
                let mut terms: Vec<(usize, f64)> = Vec::new();

                for (i, &(u_e, v_e)) in edges.iter().enumerate() {
                    if u_e == v {
                        terms.push((f_idx(i, 0, c), 1.0));
                        terms.push((f_idx(i, 1, c), -1.0));
                    }
                    if v_e == v {
                        terms.push((f_idx(i, 0, c), -1.0));
                        terms.push((f_idx(i, 1, c), 1.0));
                    }
                }

                terms.push((b_idx(v, c), -1.0));
                terms.push((x_idx(v, c), 1.0));
                constraints.push(LinearConstraint::eq(terms, 0.0));
            }
        }

        let target = ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize);
        ReductionBCSFToILP { target, n, k }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "boundedcomponentspanningforest_to_ilp",
        build: || {
            let source = BoundedComponentSpanningForest::new(
                SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
                vec![1, 2, 2, 1],
                2,
                4,
            );
            let reduction: ReductionBCSFToILP =
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
#[path = "../unit_tests/rules/boundedcomponentspanningforest_ilp.rs"]
mod tests;
