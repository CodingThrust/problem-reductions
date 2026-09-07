//! Reduction from RegisterSufficiency to `ILP<i64>`.
//!
//! The formulation uses:
//! - integer `t_v` variables for evaluation positions
//! - integer `l_v` variables for latest-use positions
//! - binary pair-order selectors to force a permutation of `0..n-1`
//! - binary threshold/live indicators to count how many values are live after
//!   each evaluation step

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::RegisterSufficiency;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionRegisterSufficiencyToILP {
    target: ILP<i64>,
    num_vertices: usize,
}

impl ReductionResult for ReductionRegisterSufficiencyToILP {
    type Source = RegisterSufficiency;
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
    transform = exact {
        num_vars = "3 * num_vertices^2 + num_vertices * (num_vertices - 1) / 2 + 2 * num_vertices",
        num_constraints = "9 * num_vertices^2 + 3 * num_vertices * (num_vertices - 1) / 2 + 3 * num_vertices + 2 * num_arcs + num_sinks",
    },
    unavailable = {
        num_nonzeros = "the exact target parameter is not represented by this reduction's symbolic transform",
    }
)]
impl ReduceTo<ILP<i64>> for RegisterSufficiency {
    type Result = ReductionRegisterSufficiencyToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_vertices();
        let pair_list: Vec<(usize, usize)> = (0..n)
            .flat_map(|u| ((u + 1)..n).map(move |v| (u, v)))
            .collect();
        let num_pair_vars = pair_list.len();

        let time_offset = 0;
        let latest_offset = n;
        let order_offset = 2 * n;
        let before_offset = order_offset + num_pair_vars;
        let after_offset = before_offset + n * n;
        let live_offset = after_offset + n * n;
        let num_vars = live_offset + n * n;

        let time_idx = |vertex: usize| -> usize { time_offset + vertex };
        let latest_idx = |vertex: usize| -> usize { latest_offset + vertex };
        let order_idx = |pair_idx: usize| -> usize { order_offset + pair_idx };
        let before_idx =
            |vertex: usize, step: usize| -> usize { before_offset + vertex * n + step };
        let after_idx = |vertex: usize, step: usize| -> usize { after_offset + vertex * n + step };
        let live_idx = |vertex: usize, step: usize| -> usize { live_offset + vertex * n + step };

        let big_m = Self::exact_i64(n, "representing the schedule length in ILP rows")?;
        let latest_time = big_m;
        let maximum_time = Self::exact_i64(
            n.saturating_sub(1),
            "representing the maximum schedule time in ILP rows",
        )?;
        let mut has_dependent = vec![false; n];
        let mut constraints = Vec::new();

        for vertex in 0..n {
            constraints.push(LinearConstraint::le(
                vec![(time_idx(vertex), 1)],
                maximum_time,
            ));
            constraints.push(LinearConstraint::le(
                vec![(latest_idx(vertex), 1)],
                latest_time,
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

        for &(dependent, dependency) in self.arcs() {
            has_dependent[dependency] = true;
            constraints.push(LinearConstraint::ge(
                vec![(time_idx(dependent), 1), (time_idx(dependency), -1)],
                1,
            ));
            constraints.push(LinearConstraint::ge(
                vec![(latest_idx(dependency), 1), (time_idx(dependent), -1)],
                0,
            ));
        }

        for (vertex, &has_child) in has_dependent.iter().enumerate() {
            if !has_child {
                constraints.push(LinearConstraint::eq(
                    vec![(latest_idx(vertex), 1)],
                    latest_time,
                ));
            }
        }

        for vertex in 0..n {
            for step in 0..n {
                let step_value = Self::exact_i64(step, "representing a schedule step in ILP rows")?;
                let before_var = before_idx(vertex, step);
                constraints.push(LinearConstraint::le(vec![(before_var, 1)], 1));
                constraints.push(LinearConstraint::le(
                    vec![(time_idx(vertex), 1), (before_var, big_m)],
                    step_value + big_m,
                ));
                constraints.push(LinearConstraint::ge(
                    vec![(time_idx(vertex), 1), (before_var, big_m)],
                    step_value + 1,
                ));

                let after_var = after_idx(vertex, step);
                constraints.push(LinearConstraint::le(vec![(after_var, 1)], 1));
                constraints.push(LinearConstraint::ge(
                    vec![(latest_idx(vertex), 1), (after_var, -big_m)],
                    step_value + 1 - big_m,
                ));
                constraints.push(LinearConstraint::le(
                    vec![(latest_idx(vertex), 1), (after_var, -big_m)],
                    step_value,
                ));

                let live_var = live_idx(vertex, step);
                constraints.push(LinearConstraint::le(
                    vec![(live_var, 1), (before_var, -1)],
                    0,
                ));
                constraints.push(LinearConstraint::le(
                    vec![(live_var, 1), (after_var, -1)],
                    0,
                ));
                constraints.push(LinearConstraint::ge(
                    vec![(live_var, 1), (before_var, -1), (after_var, -1)],
                    -1,
                ));
            }
        }

        for step in 0..n {
            let live_terms: Vec<(usize, i64)> =
                (0..n).map(|vertex| (live_idx(vertex, step), 1)).collect();
            constraints.push(LinearConstraint::le(
                live_terms,
                Self::exact_i64(
                    self.bound(),
                    "representing the register bound in an ILP row",
                )?,
            ));
        }

        Ok(ReductionRegisterSufficiencyToILP {
            target: ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize)
                .map_err(Self::target_construction)?,
            num_vertices: n,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "registersufficiency_to_ilp",
        build: || {
            let source = RegisterSufficiency::new(
                7,
                vec![
                    (2, 0),
                    (2, 1),
                    (3, 1),
                    (4, 2),
                    (4, 3),
                    (5, 0),
                    (6, 4),
                    (6, 5),
                ],
                3,
            );
            crate::example_db::specs::rule_example_via_ilp::<_, i64>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/registersufficiency_ilp.rs"]
mod tests;
