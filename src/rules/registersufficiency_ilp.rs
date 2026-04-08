//! Reduction from RegisterSufficiency to ILP<i32>.
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
    target: ILP<i32>,
    num_vertices: usize,
}

impl ReductionResult for ReductionRegisterSufficiencyToILP {
    type Source = RegisterSufficiency;
    type Target = ILP<i32>;

    fn target_problem(&self) -> &ILP<i32> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution[..self.num_vertices].to_vec()
    }
}

#[reduction(overhead = {
    num_vars = "3 * num_vertices^2 + num_vertices * (num_vertices - 1) / 2 + 2 * num_vertices",
    num_constraints = "9 * num_vertices^2 + 3 * num_vertices * (num_vertices - 1) / 2 + 3 * num_vertices + 2 * num_arcs + num_sinks",
})]
impl ReduceTo<ILP<i32>> for RegisterSufficiency {
    type Result = ReductionRegisterSufficiencyToILP;

    fn reduce_to(&self) -> Self::Result {
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

        let big_m = n as f64;
        let mut has_dependent = vec![false; n];
        let mut constraints = Vec::new();

        for vertex in 0..n {
            constraints.push(LinearConstraint::le(
                vec![(time_idx(vertex), 1.0)],
                (n.saturating_sub(1)) as f64,
            ));
            constraints.push(LinearConstraint::le(
                vec![(latest_idx(vertex), 1.0)],
                n as f64,
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

        for &(dependent, dependency) in self.arcs() {
            has_dependent[dependency] = true;
            constraints.push(LinearConstraint::ge(
                vec![(time_idx(dependent), 1.0), (time_idx(dependency), -1.0)],
                1.0,
            ));
            constraints.push(LinearConstraint::ge(
                vec![(latest_idx(dependency), 1.0), (time_idx(dependent), -1.0)],
                0.0,
            ));
        }

        for (vertex, &has_child) in has_dependent.iter().enumerate() {
            if !has_child {
                constraints.push(LinearConstraint::eq(
                    vec![(latest_idx(vertex), 1.0)],
                    n as f64,
                ));
            }
        }

        for vertex in 0..n {
            for step in 0..n {
                let before_var = before_idx(vertex, step);
                constraints.push(LinearConstraint::le(vec![(before_var, 1.0)], 1.0));
                constraints.push(LinearConstraint::le(
                    vec![(time_idx(vertex), 1.0), (before_var, big_m)],
                    step as f64 + big_m,
                ));
                constraints.push(LinearConstraint::ge(
                    vec![(time_idx(vertex), 1.0), (before_var, big_m)],
                    (step + 1) as f64,
                ));

                let after_var = after_idx(vertex, step);
                constraints.push(LinearConstraint::le(vec![(after_var, 1.0)], 1.0));
                constraints.push(LinearConstraint::ge(
                    vec![(latest_idx(vertex), 1.0), (after_var, -big_m)],
                    (step + 1) as f64 - big_m,
                ));
                constraints.push(LinearConstraint::le(
                    vec![(latest_idx(vertex), 1.0), (after_var, -big_m)],
                    step as f64,
                ));

                let live_var = live_idx(vertex, step);
                constraints.push(LinearConstraint::le(
                    vec![(live_var, 1.0), (before_var, -1.0)],
                    0.0,
                ));
                constraints.push(LinearConstraint::le(
                    vec![(live_var, 1.0), (after_var, -1.0)],
                    0.0,
                ));
                constraints.push(LinearConstraint::ge(
                    vec![(live_var, 1.0), (before_var, -1.0), (after_var, -1.0)],
                    -1.0,
                ));
            }
        }

        for step in 0..n {
            let live_terms: Vec<(usize, f64)> =
                (0..n).map(|vertex| (live_idx(vertex, step), 1.0)).collect();
            constraints.push(LinearConstraint::le(live_terms, self.bound() as f64));
        }

        ReductionRegisterSufficiencyToILP {
            target: ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize),
            num_vertices: n,
        }
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
            crate::example_db::specs::rule_example_via_ilp::<_, i32>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/registersufficiency_ilp.rs"]
mod tests;
