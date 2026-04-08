//! Reduction from Feasible Register Assignment to ILP (Integer Linear Programming).
//!
//! The formulation uses non-negative integer variables:
//! - `t_v`: evaluation position of vertex `v`
//! - `L_v`: latest position among `v` and all dependents of `v`
//! - `z_uv`: binary order selector for each unordered pair `{u, v}`
//!
//! The pair-order constraints force the `t_v` values to form a permutation of
//! `{0, ..., n-1}`. For same-register pairs, the extra constraints enforce
//! interval non-overlap: if `u` is before `v`, then `v` must be scheduled no
//! earlier than the latest dependent of `u`.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::FeasibleRegisterAssignment;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionFeasibleRegisterAssignmentToILP {
    target: ILP<i32>,
    num_vertices: usize,
}

impl ReductionResult for ReductionFeasibleRegisterAssignmentToILP {
    type Source = FeasibleRegisterAssignment;
    type Target = ILP<i32>;

    fn target_problem(&self) -> &ILP<i32> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution[..self.num_vertices].to_vec()
    }
}

#[reduction(overhead = {
    num_vars = "2 * num_vertices + num_vertices * (num_vertices - 1) / 2",
    num_constraints = "3 * num_vertices * (num_vertices - 1) / 2 + 3 * num_vertices + 2 * num_arcs + 2 * num_same_register_pairs",
})]
impl ReduceTo<ILP<i32>> for FeasibleRegisterAssignment {
    type Result = ReductionFeasibleRegisterAssignmentToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let pair_list: Vec<(usize, usize)> = (0..n)
            .flat_map(|u| ((u + 1)..n).map(move |v| (u, v)))
            .collect();
        let same_register_pairs: Vec<(usize, usize, usize)> = pair_list
            .iter()
            .copied()
            .enumerate()
            .filter(|(_, (u, v))| self.assignment()[*u] == self.assignment()[*v])
            .map(|(pair_idx, (u, v))| (u, v, pair_idx))
            .collect();

        let num_pair_vars = pair_list.len();
        let num_vars = 2 * n + num_pair_vars;
        let big_m = n as f64;

        let time_idx = |vertex: usize| -> usize { vertex };
        let latest_idx = |vertex: usize| -> usize { n + vertex };
        let order_idx = |pair_idx: usize| -> usize { 2 * n + pair_idx };

        let mut constraints = Vec::with_capacity(
            3 * num_pair_vars + 3 * n + 2 * self.num_arcs() + 2 * same_register_pairs.len(),
        );

        for vertex in 0..n {
            constraints.push(LinearConstraint::le(
                vec![(time_idx(vertex), 1.0)],
                (n.saturating_sub(1)) as f64,
            ));
            constraints.push(LinearConstraint::le(
                vec![(latest_idx(vertex), 1.0)],
                (n.saturating_sub(1)) as f64,
            ));
            constraints.push(LinearConstraint::ge(
                vec![(latest_idx(vertex), 1.0), (time_idx(vertex), -1.0)],
                0.0,
            ));
        }

        for &(dependent, dependency) in self.arcs() {
            constraints.push(LinearConstraint::ge(
                vec![(time_idx(dependent), 1.0), (time_idx(dependency), -1.0)],
                1.0,
            ));
            constraints.push(LinearConstraint::ge(
                vec![(latest_idx(dependency), 1.0), (time_idx(dependent), -1.0)],
                0.0,
            ));
        }

        for (pair_idx, &(u, v)) in pair_list.iter().enumerate() {
            let order_var = order_idx(pair_idx);
            constraints.push(LinearConstraint::le(vec![(order_var, 1.0)], 1.0));
            constraints.push(LinearConstraint::ge(
                vec![(time_idx(v), 1.0), (time_idx(u), -1.0), (order_var, -big_m)],
                1.0 - big_m,
            ));
            constraints.push(LinearConstraint::ge(
                vec![(time_idx(u), 1.0), (time_idx(v), -1.0), (order_var, big_m)],
                1.0,
            ));
        }

        for &(u, v, pair_idx) in &same_register_pairs {
            let order_var = order_idx(pair_idx);
            constraints.push(LinearConstraint::ge(
                vec![
                    (time_idx(v), 1.0),
                    (latest_idx(u), -1.0),
                    (order_var, -big_m),
                ],
                -big_m,
            ));
            constraints.push(LinearConstraint::ge(
                vec![
                    (time_idx(u), 1.0),
                    (latest_idx(v), -1.0),
                    (order_var, big_m),
                ],
                0.0,
            ));
        }

        ReductionFeasibleRegisterAssignmentToILP {
            target: ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize),
            num_vertices: n,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "feasibleregisterassignment_to_ilp",
        build: || {
            let source = FeasibleRegisterAssignment::new(
                4,
                vec![(0, 1), (0, 2), (1, 3)],
                2,
                vec![0, 1, 0, 0],
            );
            crate::example_db::specs::rule_example_via_ilp::<_, i32>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/feasibleregisterassignment_ilp.rs"]
mod tests;
