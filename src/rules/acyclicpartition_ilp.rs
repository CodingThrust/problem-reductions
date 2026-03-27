//! Reduction from AcyclicPartition to ILP<i32>.
//!
//! One-hot assignment x_{v,c}, McCormick same-class indicators s_{t,c},
//! crossing flags y_t, class ordering o_c, vertex-order copies p_v.
//! See the paper entry for the full formulation.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::AcyclicPartition;
use crate::reduction;
use crate::rules::ilp_helpers::mccormick_product;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionAcyclicPartitionToILP {
    target: ILP<i32>,
    n: usize,
}

impl ReductionResult for ReductionAcyclicPartitionToILP {
    type Source = AcyclicPartition<i32>;
    type Target = ILP<i32>;

    fn target_problem(&self) -> &ILP<i32> {
        &self.target
    }

    /// One-hot decode: for each vertex v, output the unique c with x_{v,c} = 1.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let n = self.n;
        (0..n)
            .map(|v| {
                (0..n)
                    .find(|&c| target_solution[v * n + c] == 1)
                    .unwrap_or(0)
            })
            .collect()
    }
}

#[reduction(
    overhead = {
        num_vars = "num_vertices * num_vertices + num_arcs * num_vertices + num_arcs + 2 * num_vertices",
        num_constraints = "num_vertices + num_vertices + num_arcs * num_vertices + num_arcs + 1 + 2 * num_vertices + 2 * num_vertices * num_vertices + num_arcs",
    }
)]
impl ReduceTo<ILP<i32>> for AcyclicPartition<i32> {
    type Result = ReductionAcyclicPartitionToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let arcs = self.graph().arcs();
        let m = arcs.len();

        // Variable indices:
        // x_{v,c} : v*n + c                          [0, n^2)
        // s_{t,c} : n^2 + t*n + c                    [n^2, n^2 + m*n)
        // y_t     : n^2 + m*n + t                     [n^2 + m*n, n^2 + m*n + m)
        // o_c     : n^2 + m*n + m + c                 [n^2 + m*n + m, n^2 + m*n + m + n)
        // p_v     : n^2 + m*n + m + n + v              [n^2 + m*n + m + n, n^2 + m*n + m + 2n)
        let x_idx = |v: usize, c: usize| -> usize { v * n + c };
        let s_idx = |t: usize, c: usize| -> usize { n * n + t * n + c };
        let y_idx = |t: usize| -> usize { n * n + m * n + t };
        let o_idx = |c: usize| -> usize { n * n + m * n + m + c };
        let p_idx = |v: usize| -> usize { n * n + m * n + m + n + v };

        let num_vars = n * n + m * n + m + 2 * n;
        let mut constraints = Vec::new();
        let big_m = n as f64;

        // 1) Assignment: Σ_c x_{v,c} = 1  for each vertex v
        for v in 0..n {
            let terms: Vec<(usize, f64)> = (0..n).map(|c| (x_idx(v, c), 1.0)).collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // 2) Weight bound: Σ_v w_v * x_{v,c} ≤ B  for each class c
        for c in 0..n {
            let terms: Vec<(usize, f64)> = self
                .vertex_weights()
                .iter()
                .enumerate()
                .map(|(v, &w)| (x_idx(v, c), w as f64))
                .collect();
            constraints.push(LinearConstraint::le(terms, *self.weight_bound() as f64));
        }

        // 3) McCormick: s_{t,c} = x_{u_t,c} * x_{v_t,c}
        for (t, &(u, v)) in arcs.iter().enumerate() {
            for c in 0..n {
                constraints.extend(mccormick_product(s_idx(t, c), x_idx(u, c), x_idx(v, c)));
            }
        }

        // 4) Crossing: y_t + Σ_c s_{t,c} = 1
        for t in 0..m {
            let mut terms: Vec<(usize, f64)> = vec![(y_idx(t), 1.0)];
            for c in 0..n {
                terms.push((s_idx(t, c), 1.0));
            }
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // 5) Cost bound: Σ_t cost(a_t) * y_t ≤ K
        let cost_terms: Vec<(usize, f64)> = self
            .arc_costs()
            .iter()
            .enumerate()
            .map(|(t, &c)| (y_idx(t), c as f64))
            .collect();
        constraints.push(LinearConstraint::le(cost_terms, *self.cost_bound() as f64));

        // 6) Order bounds: 0 ≤ o_c ≤ n-1, 0 ≤ p_v ≤ n-1
        for c in 0..n {
            constraints.push(LinearConstraint::ge(vec![(o_idx(c), 1.0)], 0.0));
            constraints.push(LinearConstraint::le(vec![(o_idx(c), 1.0)], (n - 1) as f64));
        }
        for v in 0..n {
            constraints.push(LinearConstraint::ge(vec![(p_idx(v), 1.0)], 0.0));
            constraints.push(LinearConstraint::le(vec![(p_idx(v), 1.0)], (n - 1) as f64));
        }

        // 7) Link p_v to o_c: p_v - o_c ≤ (n-1)(1 - x_{v,c}) and o_c - p_v ≤ (n-1)(1 - x_{v,c})
        for v in 0..n {
            for c in 0..n {
                // p_v - o_c + (n-1)*x_{v,c} ≤ n-1
                constraints.push(LinearConstraint::le(
                    vec![
                        (p_idx(v), 1.0),
                        (o_idx(c), -1.0),
                        (x_idx(v, c), (n - 1) as f64),
                    ],
                    (n - 1) as f64,
                ));
                // o_c - p_v + (n-1)*x_{v,c} ≤ n-1
                constraints.push(LinearConstraint::le(
                    vec![
                        (o_idx(c), 1.0),
                        (p_idx(v), -1.0),
                        (x_idx(v, c), (n - 1) as f64),
                    ],
                    (n - 1) as f64,
                ));
            }
        }

        // 8) DAG ordering: p_{v_t} - p_{u_t} ≥ 1 - n * Σ_c s_{t,c}
        //    i.e., p_{v_t} - p_{u_t} + n * Σ_c s_{t,c} ≥ 1
        for (t, &(u, v)) in arcs.iter().enumerate() {
            let mut terms = vec![(p_idx(v), 1.0), (p_idx(u), -1.0)];
            for c in 0..n {
                terms.push((s_idx(t, c), big_m));
            }
            constraints.push(LinearConstraint::ge(terms, 1.0));
        }

        let target = ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize);

        ReductionAcyclicPartitionToILP { target, n }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::topology::DirectedGraph;
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "acyclicpartition_to_ilp",
        build: || {
            let source = AcyclicPartition::new(
                DirectedGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
                vec![1, 1, 1, 1],
                vec![1, 1, 1],
                3,
                2,
            );
            let reduction: ReductionAcyclicPartitionToILP =
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
#[path = "../unit_tests/rules/acyclicpartition_ilp.rs"]
mod tests;
