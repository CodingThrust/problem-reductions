//! Bounded ranks with order selectors only for vertices sharing a register.

use crate::models::algebraic::{Bounded, IntegerVariable, LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::FeasibleRegisterAssignment;
use crate::reduction;
use crate::rules::ilp_helpers::{bounded_order_comparison, ranks_to_positions};
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionFeasibleRegisterAssignmentToILP {
    target: ILP<i64, i64, Bounded>,
    num_vertices: usize,
}

impl ReductionResult for ReductionFeasibleRegisterAssignmentToILP {
    type Source = FeasibleRegisterAssignment;
    type Target = ILP<i64, i64, Bounded>;

    fn target_problem(&self) -> &ILP<i64, i64, Bounded> {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_witness(
            self.target_problem(),
            target_solution,
            |value| value.value.is_some(),
            "target ILP assignment is infeasible",
        )?;

        Ok(ranks_to_positions(&target_solution[..self.num_vertices]))
    }
}

#[crate::aggregate_reduction(ilp_feasibility)]
impl crate::rules::AggregateReductionResult for ReductionFeasibleRegisterAssignmentToILP {}

#[reduction(transform = {
    exact {
        num_vars = "num_vertices + num_same_register_pairs",
    },
    upper_bound {
        num_constraints = "num_vertices * num_arcs + 2 * num_same_register_pairs",
        num_nonzeros = "3 * num_vertices * num_arcs + 6 * num_same_register_pairs",
        max_constraint_magnitude_bits = "num_vertices + 1",
    },
})]
impl ReduceTo<ILP<i64, i64, Bounded>> for FeasibleRegisterAssignment {
    type Result = ReductionFeasibleRegisterAssignmentToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_vertices();
        let pairs: Vec<_> = (0..n)
            .flat_map(|u| ((u + 1)..n).map(move |v| (u, v)))
            .filter(|&(u, v)| self.assignment()[u] == self.assignment()[v])
            .collect();
        let num_vars = n.checked_add(pairs.len()).ok_or_else(|| {
            crate::rules::ReductionError::integer_overflow::<Self, ILP<i64, i64, Bounded>>(
                "counting register assignment variables",
            )
        })?;
        let big_m = Self::exact_i64(n, "encoding the schedule order")?;
        let last_position = big_m.saturating_sub(1).max(0);
        let mut variables = vec![
            IntegerVariable::new(Some(0), Some(last_position))
                .map_err(Self::target_construction)?;
            n
        ];
        variables.resize(num_vars, IntegerVariable::binary());
        let mut constraints = Vec::new();
        let mut dependents = vec![Vec::new(); n];
        for &(dependent, dependency) in self.arcs() {
            constraints.push(LinearConstraint::ge(
                vec![(dependent, 1), (dependency, -1)],
                1,
            ));
            dependents[dependency].push(dependent);
        }
        for (index, &(u, v)) in pairs.iter().enumerate() {
            let selector = n + index;
            constraints.extend(bounded_order_comparison(u, v, selector, big_m));
            // An overwriter may consume the old value in that same operation.
            // Every other consumer must finish strictly before the overwrite.
            for &w in &dependents[u] {
                if w != v {
                    constraints.push(LinearConstraint::ge(
                        vec![(v, 1), (w, -1), (selector, -big_m)],
                        1 - big_m,
                    ));
                }
            }
            for &w in &dependents[v] {
                if w != u {
                    constraints.push(LinearConstraint::ge(
                        vec![(u, 1), (w, -1), (selector, big_m)],
                        1,
                    ));
                }
            }
        }
        let target = ILP::with_variables(variables, constraints, vec![], ObjectiveSense::Minimize)
            .map_err(Self::target_construction)?;
        crate::rules::ilp_helpers::validate_bounded_constraint_arithmetic::<Self>(&target)?;

        Ok(ReductionFeasibleRegisterAssignmentToILP {
            target,
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
            crate::example_db::specs::rule_example_via_bounded_ilp::<_>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/feasibleregisterassignment_ilp.rs"]
mod tests;
