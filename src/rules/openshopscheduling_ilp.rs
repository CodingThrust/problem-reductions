//! Bounded disjunctive open-shop scheduling with identical-machine symmetry.
use crate::models::algebraic::{Bounded, IntegerVariable, LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::OpenShopScheduling;
use crate::models::Decision;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionOSSToILP {
    target: ILP<i64, i64, Bounded>,
    start_offset: usize,
    num_operations: usize,
}

impl ReductionOSSToILP {
    fn decode_schedule(&self, solution: &[i64]) -> crate::rules::ExtractionResult<Vec<usize>> {
        crate::rules::ilp_helpers::decode_usize_values(
            &solution[self.start_offset..self.start_offset + self.num_operations],
        )
    }

    fn build(
        source: &OpenShopScheduling,
        bound: Option<i64>,
    ) -> Result<Self, crate::rules::ReductionError> {
        type Target = ILP<i64, i64, Bounded>;
        let overflow = |operation| {
            crate::rules::ReductionError::integer_overflow::<OpenShopScheduling, Target>(operation)
        };
        let construction = <OpenShopScheduling as ReduceTo<Target>>::target_construction;
        let total = i64::try_from(source.schedule_horizon())
            .map_err(|_| overflow("converting the schedule horizon"))?;
        let horizon = bound.map_or(total, |b| b.min(total));
        let p = source.processing_times();
        if horizon < 0 || p.iter().flatten().any(|&duration| duration > horizon) {
            return Ok(Self {
                target: ILP::with_variables(
                    vec![],
                    vec![LinearConstraint::eq(vec![], 1)],
                    vec![],
                    ObjectiveSense::Minimize,
                )
                .map_err(construction)?,
                start_offset: 0,
                num_operations: 0,
            });
        }
        let n = source.num_jobs();
        let m = source.num_machines();
        let pairs = |k: usize| {
            if k.is_multiple_of(2) {
                (k / 2).checked_mul(k.saturating_sub(1))
            } else {
                k.checked_mul(k / 2)
            }
        };
        let machine_orders = if m == 0 {
            0
        } else {
            pairs(n)
                .and_then(|v| v.checked_mul(m))
                .ok_or_else(|| overflow("counting machine order variables"))?
        };
        let job_orders = if n == 0 {
            0
        } else {
            pairs(m)
                .and_then(|v| v.checked_mul(n))
                .ok_or_else(|| overflow("counting job order variables"))?
        };
        let operations = n
            .checked_mul(m)
            .ok_or_else(|| overflow("counting operations"))?;
        let makespan = machine_orders
            .checked_add(operations)
            .and_then(|v| v.checked_add(job_orders))
            .ok_or_else(|| overflow("counting scheduling variables"))?;
        let count = makespan
            .checked_add(usize::from(bound.is_none()))
            .ok_or_else(|| overflow("counting scheduling variables"))?;
        let mut variables = vec![IntegerVariable::binary(); count];
        for (index, &duration) in p.iter().flatten().enumerate() {
            variables[machine_orders + index] =
                IntegerVariable::new(Some(0), Some(horizon - duration)).map_err(construction)?;
        }
        let start = |job: usize, machine: usize| machine_orders + job * m + machine;
        let mut rows = Vec::new();
        let mut disjunction = |a, b, bit, pa, pb| {
            rows.push(LinearConstraint::ge(
                vec![(b, 1), (a, -1), (bit, -horizon)],
                pa - horizon,
            ));
            rows.push(LinearConstraint::ge(
                vec![(a, 1), (b, -1), (bit, horizon)],
                pb,
            ));
        };
        let mut bit = 0;
        for j in 0..n {
            for k in j + 1..n {
                for (i, (&left, &right)) in p[j].iter().zip(&p[k]).enumerate() {
                    disjunction(start(j, i), start(k, i), bit, left, right);
                    bit += 1;
                }
            }
        }
        bit = machine_orders + operations;
        for (j, durations) in p.iter().enumerate() {
            for i in 0..m {
                for k in i + 1..m {
                    disjunction(start(j, i), start(j, k), bit, durations[i], durations[k]);
                    bit += 1;
                }
            }
        }
        let mut objective = Vec::new();
        if bound.is_none() {
            variables[makespan] =
                IntegerVariable::new(Some(0), Some(horizon)).map_err(construction)?;
            objective.push((makespan, 1));
            for (index, &duration) in p.iter().flatten().enumerate() {
                rows.push(LinearConstraint::ge(
                    vec![(makespan, 1), (machine_orders + index, -1)],
                    duration,
                ));
            }
        }
        // Relabel identical machines so one anchor job visits them in index order.
        // Ordering every job this way would incorrectly impose a flow shop.
        if n > 0 {
            let mut groups = std::collections::BTreeMap::<Vec<i64>, Vec<usize>>::new();
            for i in 0..m {
                groups
                    .entry(p.iter().map(|job| job[i]).collect())
                    .or_default()
                    .push(i);
            }
            for (column, machines) in groups {
                let anchor = (0..n)
                    .max_by_key(|&j| (column[j], std::cmp::Reverse(j)))
                    .expect("nonempty jobs");
                for pair in machines.windows(2) {
                    rows.push(LinearConstraint::ge(
                        vec![(start(anchor, pair[1]), 1), (start(anchor, pair[0]), -1)],
                        column[anchor],
                    ));
                }
            }
        }
        let target = ILP::with_variables(variables, rows, objective, ObjectiveSense::Minimize)
            .map_err(construction)?;
        crate::rules::ilp_helpers::validate_bounded_constraint_arithmetic::<OpenShopScheduling>(
            &target,
        )?;
        Ok(Self {
            target,
            start_offset: machine_orders,
            num_operations: operations,
        })
    }
}

impl ReductionResult for ReductionOSSToILP {
    type Source = OpenShopScheduling;
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
        self.decode_schedule(solution)
    }
}

#[reduction(transform = {
    exact { num_vars = "num_jobs * (num_jobs - 1) / 2 * num_machines + num_jobs * num_machines + num_jobs * num_machines * (num_machines - 1) / 2 + 1", },
    upper_bound {
        num_constraints = "num_jobs * (num_jobs - 1) * num_machines + num_jobs * num_machines * (num_machines - 1) + num_jobs * num_machines + num_machines",
        num_nonzeros = "3 * num_jobs * (num_jobs - 1) * num_machines + 3 * num_jobs * num_machines * (num_machines - 1) + 2 * num_jobs * num_machines + 2 * num_machines",
        max_constraint_magnitude_bits = "schedule_horizon_bits",
    },
})]
impl ReduceTo<ILP<i64, i64, Bounded>> for OpenShopScheduling {
    type Result = ReductionOSSToILP;
    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        ReductionOSSToILP::build(self, None)
    }
}

#[derive(Debug, Clone)]
pub struct ReductionDecisionOpenShopSchedulingToILP {
    inner: ReductionOSSToILP,
}
impl ReductionResult for ReductionDecisionOpenShopSchedulingToILP {
    type Source = Decision<OpenShopScheduling>;
    type Target = ILP<i64, i64, Bounded>;
    fn target_problem(&self) -> &Self::Target {
        &self.inner.target
    }
    fn extract_solution(&self, solution: &Vec<i64>) -> crate::rules::ExtractionResult<Vec<usize>> {
        crate::rules::traits::validate_target_witness(
            self.target_problem(),
            solution,
            |value| value.value.is_some(),
            "target ILP assignment is infeasible",
        )?;
        self.inner.decode_schedule(solution)
    }
}
#[crate::aggregate_reduction(ilp_feasibility)]
impl crate::rules::AggregateReductionResult for ReductionDecisionOpenShopSchedulingToILP {}
#[reduction(transform = upper_bound {
    num_vars = "num_jobs * (num_jobs - 1) / 2 * num_machines + num_jobs * num_machines + num_jobs * num_machines * (num_machines - 1) / 2",
    num_constraints = "num_jobs * (num_jobs - 1) * num_machines + num_jobs * num_machines * (num_machines - 1) + num_machines + 1",
    num_nonzeros = "3 * num_jobs * (num_jobs - 1) * num_machines + 3 * num_jobs * num_machines * (num_machines - 1) + 2 * num_machines",
    max_constraint_magnitude_bits = "schedule_horizon_bits",
})]
impl ReduceTo<ILP<i64, i64, Bounded>> for Decision<OpenShopScheduling> {
    type Result = ReductionDecisionOpenShopSchedulingToILP;
    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        Ok(ReductionDecisionOpenShopSchedulingToILP {
            inner: ReductionOSSToILP::build(self.inner(), Some(*self.bound()))?,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![
        crate::example_db::specs::RuleExampleSpec {
            id: "decisionopenshopscheduling_to_ilp",
            build: || {
                let source =
                    Decision::new(OpenShopScheduling::new(2, vec![vec![1, 2], vec![2, 1]]), 3);
                crate::example_db::specs::rule_example_via_bounded_ilp::<_>(source)
            },
        },
        crate::example_db::specs::RuleExampleSpec {
            id: "openshopscheduling_to_ilp",
            build: || {
                // Small 2x2 instance for canonical example
                let source = OpenShopScheduling::new(2, vec![vec![1, 2], vec![2, 1]]);
                crate::example_db::specs::rule_example_via_bounded_ilp::<_>(source)
            },
        },
    ]
}

#[cfg(test)]
#[path = "../unit_tests/rules/openshopscheduling_ilp.rs"]
mod tests;
