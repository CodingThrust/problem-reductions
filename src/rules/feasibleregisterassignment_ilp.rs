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
    target: ILP<i64>,
    num_vertices: usize,
}

impl ReductionResult for ReductionFeasibleRegisterAssignmentToILP {
    type Source = FeasibleRegisterAssignment;
    type Target = ILP<i64>;

    fn target_problem(&self) -> &ILP<i64> {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        crate::rules::ilp_helpers::decode_usize_values(&target_solution[..self.num_vertices])
    }
}

#[reduction(
    size = exact {
        num_vars = "2 * num_vertices + num_vertices * (num_vertices - 1) / 2",
        num_constraints = "3 * num_vertices * (num_vertices - 1) / 2 + 3 * num_vertices + 2 * num_arcs + 2 * num_same_register_pairs",
    },)]
impl ReduceTo<ILP<i64>> for FeasibleRegisterAssignment {
    type Result = ReductionFeasibleRegisterAssignmentToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
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
        let big_m = Self::exact_i64(n, "encoding the schedule order")?;
        let last_position =
            Self::exact_i64(n.saturating_sub(1), "encoding the final schedule position")?;

        let time_idx = |vertex: usize| -> usize { vertex };
        let latest_idx = |vertex: usize| -> usize { n + vertex };
        let order_idx = |pair_idx: usize| -> usize { 2 * n + pair_idx };

        let mut constraints = Vec::with_capacity(
            3 * num_pair_vars + 3 * n + 2 * self.num_arcs() + 2 * same_register_pairs.len(),
        );

        for vertex in 0..n {
            constraints.push(LinearConstraint::le(
                vec![(time_idx(vertex), 1)],
                last_position,
            ));
            constraints.push(LinearConstraint::le(
                vec![(latest_idx(vertex), 1)],
                last_position,
            ));
            constraints.push(LinearConstraint::ge(
                vec![(latest_idx(vertex), 1), (time_idx(vertex), -1)],
                0,
            ));
        }

        for &(dependent, dependency) in self.arcs() {
            constraints.push(LinearConstraint::ge(
                vec![(time_idx(dependent), 1), (time_idx(dependency), -1)],
                1,
            ));
            constraints.push(LinearConstraint::ge(
                vec![(latest_idx(dependency), 1), (time_idx(dependent), -1)],
                0,
            ));
        }

        for (pair_idx, &(u, v)) in pair_list.iter().enumerate() {
            let order_var = order_idx(pair_idx);
            constraints.push(LinearConstraint::le(vec![(order_var, 1)], 1));
            constraints.push(LinearConstraint::ge(
                vec![(time_idx(v), 1), (time_idx(u), -1), (order_var, -big_m)],
                1 - big_m,
            ));
            constraints.push(LinearConstraint::ge(
                vec![(time_idx(u), 1), (time_idx(v), -1), (order_var, big_m)],
                1,
            ));
        }

        for &(u, v, pair_idx) in &same_register_pairs {
            let order_var = order_idx(pair_idx);
            constraints.push(LinearConstraint::ge(
                vec![(time_idx(v), 1), (latest_idx(u), -1), (order_var, -big_m)],
                -big_m,
            ));
            constraints.push(LinearConstraint::ge(
                vec![(time_idx(u), 1), (latest_idx(v), -1), (order_var, big_m)],
                0,
            ));
        }

        Ok(ReductionFeasibleRegisterAssignmentToILP {
            target: ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize)
                .map_err(Self::target_construction)?,
            num_vertices: n,
        })
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
            crate::example_db::specs::rule_example_via_ilp::<_, i64>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/feasibleregisterassignment_ilp.rs"]
mod tests;
