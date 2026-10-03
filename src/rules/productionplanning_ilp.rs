//! Inventory conservation with exact production setup indicators.
use crate::models::algebraic::{Bounded, IntegerVariable, LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::ProductionPlanning;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionProductionPlanningToILP {
    target: ILP<i64, i64, Bounded>,
    num_periods: usize,
}
impl ReductionResult for ReductionProductionPlanningToILP {
    type Source = ProductionPlanning;
    type Target = ILP<i64, i64, Bounded>;
    fn target_problem(&self) -> &Self::Target {
        &self.target
    }
    fn extract_solution(&self, solution: &Vec<i64>) -> crate::rules::ExtractionResult<Vec<usize>> {
        crate::rules::traits::validate_target_witness(
            &self.target,
            solution,
            |value| value.value.is_some(),
            "target ILP assignment is infeasible",
        )?;
        crate::rules::ilp_helpers::decode_usize_values(&solution[..self.num_periods])
    }
}
#[crate::aggregate_reduction(ilp_feasibility)]
impl crate::rules::AggregateReductionResult for ReductionProductionPlanningToILP {}
#[crate::reduction(transform = {
    exact { num_vars = "3 * num_periods", num_constraints = "3 * num_periods + 1", },
    upper_bound {
        num_nonzeros = "10 * num_periods",
        max_constraint_magnitude_bits = "max_numeric_magnitude_bits + num_periods",
    },
})]
impl ReduceTo<ILP<i64, i64, Bounded>> for ProductionPlanning {
    type Result = ReductionProductionPlanningToILP;
    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let overflow = |operation| {
            crate::rules::ReductionError::integer_overflow::<Self, ILP<i64, i64, Bounded>>(
                operation,
            )
        };
        let n = self.num_periods();
        let count = n
            .checked_mul(3)
            .ok_or_else(|| overflow("counting planning variables"))?;
        self.demands()
            .iter()
            .try_fold(0_i64, |sum, &d| sum.checked_add(d))
            .ok_or_else(|| overflow("summing planning demands"))?;
        let mut variables = Vec::with_capacity(count);
        for &capacity in self.capacities() {
            variables.push(
                IntegerVariable::new(Some(0), Some(capacity)).map_err(Self::target_construction)?,
            );
        }
        let mut total = 0_i64;
        for &capacity in self.capacities() {
            total = total
                .checked_add(capacity)
                .ok_or_else(|| overflow("summing planning capacities"))?;
            variables.push(
                IntegerVariable::new(Some(0), Some(total)).map_err(Self::target_construction)?,
            );
        }
        variables.extend(std::iter::repeat_n(IntegerVariable::binary(), n));
        let mut constraints = Vec::new();
        let mut costs = Vec::new();
        for t in 0..n {
            let mut flow = vec![(t, 1), (n + t, -1)];
            if t > 0 {
                flow.push((n + t - 1, 1));
            }
            constraints.push(LinearConstraint::eq(flow, self.demands()[t]));
            constraints.push(LinearConstraint::le(
                vec![(t, 1), (2 * n + t, -self.capacities()[t])],
                0,
            ));
            constraints.push(LinearConstraint::ge(vec![(t, 1), (2 * n + t, -1)], 0));
            costs.extend([
                (t, self.production_costs()[t]),
                (n + t, self.inventory_costs()[t]),
                (2 * n + t, self.setup_costs()[t]),
            ]);
        }
        constraints.push(LinearConstraint::le(costs, self.cost_bound()));
        let target = ILP::with_variables(variables, constraints, vec![], ObjectiveSense::Minimize)
            .map_err(Self::target_construction)?;
        crate::rules::ilp_helpers::validate_bounded_constraint_arithmetic::<Self>(&target)?;
        Ok(Self::Result {
            target,
            num_periods: n,
        })
    }
}
#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "productionplanning_to_ilp",
        build: || {
            crate::example_db::specs::rule_example_via_bounded_ilp::<_>(ProductionPlanning::new(
                3,
                vec![1; 3],
                vec![2, 0, 1],
                vec![2; 3],
                vec![1; 3],
                vec![1; 3],
                8,
            ))
        },
    }]
}
#[cfg(test)]
#[path = "../unit_tests/rules/productionplanning_ilp.rs"]
mod tests;
