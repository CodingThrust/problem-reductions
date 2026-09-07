//! Reduction from AcyclicPartition to `ILP<i64>`.
//!
//! One-hot assignment x_{v,c}, McCormick same-class indicators s_{t,c},
//! crossing flags y_t, and partition labels used directly as a topological order.
//! See the paper entry for the full formulation.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::AcyclicPartition;
use crate::reduction;
use crate::rules::ilp_helpers::mccormick_product;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionAcyclicPartitionToILP {
    target: ILP<i64>,
    n: usize,
}

impl ReductionResult for ReductionAcyclicPartitionToILP {
    type Source = AcyclicPartition<i64>;
    type Target = ILP<i64>;

    fn target_problem(&self) -> &ILP<i64> {
        &self.target
    }

    /// One-hot decode: for each vertex v, output the unique c with x_{v,c} = 1.
    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        crate::rules::ilp_helpers::one_hot_decode_rows(target_solution, self.n, self.n, 0)
    }
}

#[reduction(
    transform = exact {
        num_vars = "num_vertices * num_vertices + num_arcs * num_vertices + num_arcs",
        num_constraints = "2 * num_vertices + 3 * num_arcs * num_vertices + 2 * num_arcs + 1",
    },
    unavailable = {
        num_nonzeros = "the exact target parameter is not represented by this reduction's symbolic transform",
    }
)]
impl ReduceTo<ILP<i64>> for AcyclicPartition<i64> {
    type Result = ReductionAcyclicPartitionToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_vertices();
        let arcs = self.graph().arcs();
        let m = arcs.len();

        // Variable indices:
        // x_{v,c} : v*n + c                          [0, n^2)
        // s_{t,c} : n^2 + t*n + c                    [n^2, n^2 + m*n)
        // y_t     : n^2 + m*n + t                     [n^2 + m*n, n^2 + m*n + m)
        let x_idx = |v: usize, c: usize| -> usize { v * n + c };
        let s_idx = |t: usize, c: usize| -> usize { n * n + t * n + c };
        let y_idx = |t: usize| -> usize { n * n + m * n + t };
        let num_vars = n * n + m * n + m;
        let mut constraints = Vec::new();
        let vertex_weights = self.vertex_weights();
        let arc_costs = self.arc_costs();
        let weight_bound = *self.weight_bound();
        let cost_bound = *self.cost_bound();

        // 1) Assignment: Σ_c x_{v,c} = 1  for each vertex v
        for v in 0..n {
            let terms: Vec<(usize, i64)> = (0..n).map(|c| (x_idx(v, c), 1)).collect();
            constraints.push(LinearConstraint::eq(terms, 1));
        }

        // 2) Weight bound: Σ_v w_v * x_{v,c} ≤ B  for each class c
        for c in 0..n {
            let terms: Vec<(usize, i64)> = vertex_weights
                .iter()
                .enumerate()
                .map(|(vertex, &weight)| (x_idx(vertex, c), weight))
                .collect();
            constraints.push(LinearConstraint::le(terms, weight_bound));
        }

        // 3) McCormick: s_{t,c} = x_{u_t,c} * x_{v_t,c}
        for (t, &(u, v)) in arcs.iter().enumerate() {
            for c in 0..n {
                constraints.extend(mccormick_product(s_idx(t, c), x_idx(u, c), x_idx(v, c)));
            }
        }

        // 4) Crossing: y_t + Σ_c s_{t,c} = 1
        for t in 0..m {
            let mut terms: Vec<(usize, i64)> = vec![(y_idx(t), 1)];
            for c in 0..n {
                terms.push((s_idx(t, c), 1));
            }
            constraints.push(LinearConstraint::eq(terms, 1));
        }

        // 5) Cost bound: Σ_t cost(a_t) * y_t ≤ K
        let cost_terms: Vec<(usize, i64)> = arc_costs
            .iter()
            .enumerate()
            .map(|(arc, &cost)| (y_idx(arc), cost))
            .collect();
        constraints.push(LinearConstraint::le(cost_terms, cost_bound));

        // 6) Topological labels: every arc goes from a lower or equal class to a
        //    higher or equal class. Equal labels are internal arcs; strict
        //    increases are quotient arcs.
        for (u, v) in arcs {
            let mut terms = Vec::with_capacity(2 * n.saturating_sub(1));
            for c in 1..n {
                let label = Self::exact_i64(c, "representing a partition label in ILP rows")?;
                terms.push((x_idx(u, c), label));
                terms.push((x_idx(v, c), -label));
            }
            constraints.push(LinearConstraint::le(terms, 0));
        }

        let target = ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize)
            .map_err(Self::target_construction)?;

        Ok(ReductionAcyclicPartitionToILP { target, n })
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
#[path = "../unit_tests/rules/acyclicpartition_ilp.rs"]
mod tests;
