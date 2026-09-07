//! Polynomial-size circuit-slot reduction from EnsembleComputation to `ILP<i64>`.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::EnsembleComputation;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionEnsembleComputationToILP {
    target: ILP<i64>,
    universe_size: usize,
    budget: usize,
    activity_base: usize,
    left_selector_base: usize,
    right_selector_base: usize,
}

impl ReductionEnsembleComputationToILP {
    fn operand_offset(&self, step: usize) -> usize {
        step * self.universe_size + step * step.saturating_sub(1) / 2
    }

    fn selector_var(&self, left: bool, step: usize, operand: usize) -> usize {
        let base = if left {
            self.left_selector_base
        } else {
            self.right_selector_base
        };
        base + self.operand_offset(step) + operand
    }
}

impl ReductionResult for ReductionEnsembleComputationToILP {
    type Source = EnsembleComputation;
    type Target = ILP<i64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;
        let mut config = Vec::with_capacity(2 * self.budget);
        let mut inactive = false;
        for step in 0..self.budget {
            let active = target_solution[self.activity_base + step];
            if active == 0 {
                inactive = true;
                continue;
            }
            if active != 1 || inactive {
                return Err(crate::rules::ExtractionError::invalid(
                    "active ensemble-operation slots must form a binary prefix",
                ));
            }
            for left in [true, false] {
                let selected = (0..self.universe_size + step)
                    .filter(|&operand| target_solution[self.selector_var(left, step, operand)] == 1)
                    .collect::<Vec<_>>();
                if selected.len() != 1 {
                    return Err(crate::rules::ExtractionError::invalid(
                        "each active ensemble operation must select exactly one operand per side",
                    ));
                }
                config.push(selected[0]);
            }
        }
        let filler = if self.universe_size >= 2 {
            [0, 1]
        } else {
            [0, 0]
        };
        while config.len() < 2 * self.budget {
            config.extend(filler);
        }
        Ok(config)
    }
}

#[reduction(
    transform = exact {
        num_vars = "3 * budget * universe_size + budget * (budget - 1) * (universe_size + 1) + num_subsets * budget + budget",
        num_constraints = "5 * budget - 1 + budget * (budget - 1) * (1 + 3 * universe_size) + 2 * budget * universe_size + num_subsets * budget * (universe_size + 2) + num_subsets",
    },
    unavailable = {
        num_nonzeros = "depends on the cardinalities and duplicate structure of the required subsets",
    }
)]
impl ReduceTo<ILP<i64>> for EnsembleComputation {
    type Result = ReductionEnsembleComputationToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let u = self.universe_size();
        let budget = self.budget();
        let t = self.num_subsets();
        let overflow = |operation| {
            crate::rules::ReductionError::integer_overflow::<EnsembleComputation, ILP<i64>>(
                operation,
            )
        };
        let pair_count = budget
            .checked_mul(budget.saturating_sub(1))
            .and_then(|value| value.checked_div(2))
            .ok_or_else(|| overflow("counting earlier ensemble-operation pairs"))?;
        let operand_count = budget
            .checked_mul(u)
            .and_then(|value| value.checked_add(pair_count))
            .ok_or_else(|| overflow("counting ensemble operand selectors"))?;
        let result_count = budget
            .checked_mul(u)
            .ok_or_else(|| overflow("counting ensemble result bits"))?;
        let product_count = pair_count
            .checked_mul(u)
            .ok_or_else(|| overflow("counting ensemble selector-result products"))?;
        let match_count = t
            .checked_mul(budget)
            .ok_or_else(|| overflow("counting ensemble target matches"))?;

        let activity_base = 0;
        let left_selector_base = budget;
        let right_selector_base = left_selector_base
            .checked_add(operand_count)
            .ok_or_else(|| overflow("laying out left ensemble selectors"))?;
        let result_base = right_selector_base
            .checked_add(operand_count)
            .ok_or_else(|| overflow("laying out right ensemble selectors"))?;
        let left_product_base = result_base
            .checked_add(result_count)
            .ok_or_else(|| overflow("laying out ensemble result bits"))?;
        let right_product_base = left_product_base
            .checked_add(product_count)
            .ok_or_else(|| overflow("laying out left ensemble products"))?;
        let match_base = right_product_base
            .checked_add(product_count)
            .ok_or_else(|| overflow("laying out right ensemble products"))?;
        let num_vars = match_base
            .checked_add(match_count)
            .ok_or_else(|| overflow("counting ensemble ILP variables"))?;

        let operand_offset = |step: usize| step * u + step * step.saturating_sub(1) / 2;
        let selector = |left: bool, step: usize, operand: usize| {
            (if left {
                left_selector_base
            } else {
                right_selector_base
            }) + operand_offset(step)
                + operand
        };
        let result = |step: usize, element: usize| result_base + step * u + element;
        let pair_index = |step: usize, earlier: usize| step * step.saturating_sub(1) / 2 + earlier;
        let product = |left: bool, step: usize, earlier: usize, element: usize| {
            (if left {
                left_product_base
            } else {
                right_product_base
            }) + pair_index(step, earlier) * u
                + element
        };
        let matched = |target: usize, step: usize| match_base + target * budget + step;

        let mut constraints = Vec::new();
        for step in 0..budget {
            constraints.push(LinearConstraint::le(vec![(activity_base + step, 1)], 1));
        }
        for step in 0..budget.saturating_sub(1) {
            constraints.push(LinearConstraint::ge(
                vec![(activity_base + step, 1), (activity_base + step + 1, -1)],
                0,
            ));
        }

        for step in 0..budget {
            for left in [true, false] {
                let mut terms = (0..u + step)
                    .map(|operand| (selector(left, step, operand), 1))
                    .collect::<Vec<_>>();
                terms.push((activity_base + step, -1));
                constraints.push(LinearConstraint::eq(terms, 0));
                for earlier in 0..step {
                    constraints.push(LinearConstraint::le(
                        vec![
                            (selector(left, step, u + earlier), 1),
                            (activity_base + earlier, -1),
                        ],
                        0,
                    ));
                }
            }
            // Disjoint union is commutative. Canonically ordering operand
            // indices removes the two equivalent orientations of every gate.
            let mut canonical_order = vec![(activity_base + step, 1)];
            for operand in 0..u + step {
                let coefficient = i64::try_from(
                    operand
                        .checked_add(1)
                        .ok_or_else(|| overflow("ordering ensemble operands"))?,
                )
                .map_err(|_| overflow("ordering ensemble operands"))?;
                canonical_order.push((selector(true, step, operand), coefficient));
                canonical_order.push((selector(false, step, operand), -coefficient));
            }
            constraints.push(LinearConstraint::le(canonical_order, 0));
        }

        for step in 0..budget {
            for earlier in 0..step {
                for element in 0..u {
                    for left in [true, false] {
                        let value = product(left, step, earlier, element);
                        let selected = selector(left, step, u + earlier);
                        let bit = result(earlier, element);
                        constraints.extend(crate::rules::ilp_helpers::mccormick_product(
                            value, selected, bit,
                        ));
                    }
                }
            }
        }

        for step in 0..budget {
            for element in 0..u {
                let membership = |left: bool| {
                    let mut terms = vec![(selector(left, step, element), 1)];
                    terms.extend(
                        (0..step).map(|earlier| (product(left, step, earlier, element), 1)),
                    );
                    terms
                };
                let left = membership(true);
                let right = membership(false);
                let mut disjoint = left.clone();
                disjoint.extend(right.clone());
                constraints.push(LinearConstraint::le(disjoint, 1));
                let mut union = vec![(result(step, element), 1)];
                union.extend(
                    left.into_iter()
                        .map(|(variable, coefficient)| (variable, -coefficient)),
                );
                union.extend(
                    right
                        .into_iter()
                        .map(|(variable, coefficient)| (variable, -coefficient)),
                );
                constraints.push(LinearConstraint::eq(union, 0));
            }
        }

        for (target_index, target) in self.subsets().iter().enumerate() {
            let membership: Vec<bool> = (0..u)
                .map(|element| target.binary_search(&element).is_ok())
                .collect();
            let mut present = Vec::with_capacity(budget);
            for step in 0..budget {
                let match_var = matched(target_index, step);
                present.push((match_var, 1));
                constraints.push(LinearConstraint::le(vec![(match_var, 1)], 1));
                constraints.push(LinearConstraint::le(
                    vec![(match_var, 1), (activity_base + step, -1)],
                    0,
                ));
                for (element, &contains_element) in membership.iter().enumerate() {
                    if contains_element {
                        constraints.push(LinearConstraint::le(
                            vec![(match_var, 1), (result(step, element), -1)],
                            0,
                        ));
                    } else {
                        constraints.push(LinearConstraint::le(
                            vec![(match_var, 1), (result(step, element), 1)],
                            1,
                        ));
                    }
                }
            }
            constraints.push(LinearConstraint::ge(present, 1));
        }

        let objective = (0..budget).map(|step| (activity_base + step, 1)).collect();
        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize)
            .map_err(Self::target_construction)?;
        Ok(ReductionEnsembleComputationToILP {
            target,
            universe_size: u,
            budget,
            activity_base,
            left_selector_base,
            right_selector_base,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "ensemblecomputation_to_ilp",
        build: || {
            let source = EnsembleComputation::new(4, vec![vec![0, 1], vec![0, 1, 2, 3]], 3);
            crate::example_db::specs::rule_example_via_ilp::<_, i64>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/ensemblecomputation_ilp.rs"]
mod tests;
