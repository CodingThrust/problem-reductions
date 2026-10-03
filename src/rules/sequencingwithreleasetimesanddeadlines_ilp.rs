//! Exact release/deadline sequencing using bounded starts and pairwise order.
//! The binary endpoint uniformly composes the same construction with the
//! existing bounded-integer encoding; neither construction expands time slots.

use crate::models::algebraic::{Bounded, IntegerVariable, LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::SequencingWithReleaseTimesAndDeadlines;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionSWRTDToBoundedILP {
    target: ILP<i64, i64, Bounded>,
    lengths: Vec<i64>,
}

impl ReductionResult for ReductionSWRTDToBoundedILP {
    type Source = SequencingWithReleaseTimesAndDeadlines;
    type Target = ILP<i64, i64, Bounded>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, values: &Vec<i64>) -> crate::rules::ExtractionResult<Vec<usize>> {
        crate::rules::traits::validate_target_witness(
            self.target_problem(),
            values,
            |value| value.value.is_some(),
            "target ILP assignment is infeasible",
        )?;
        let mut order: Vec<_> = (0..self.lengths.len()).collect();
        // A zero-duration job at a positive job's start must come first.
        order.sort_by_key(|&j| (values[j], self.lengths[j], j));
        Ok(order)
    }
}

#[crate::aggregate_reduction(ilp_feasibility)]
impl crate::rules::AggregateReductionResult for ReductionSWRTDToBoundedILP {}

#[reduction(transform = upper_bound {
    num_vars = "num_tasks + num_tasks * (num_tasks - 1) / 2",
    num_constraints = "num_tasks * (num_tasks - 1) + 1",
    num_nonzeros = "3 * num_tasks * (num_tasks - 1)",
    max_constraint_magnitude_bits = "time_horizon_bits",
})]
impl ReduceTo<ILP<i64, i64, Bounded>> for SequencingWithReleaseTimesAndDeadlines {
    type Result = ReductionSWRTDToBoundedILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let construct = <Self as ReduceTo<ILP<i64, i64, Bounded>>>::target_construction;
        let n = self.num_tasks();
        let lengths = self.lengths();
        let releases = self.release_times();
        let deadlines = self.deadlines();
        if (0..n).any(|j| releases[j] > deadlines[j] || lengths[j] > deadlines[j] - releases[j]) {
            return Ok(Self::Result {
                target: ILP::with_variables(
                    vec![],
                    vec![LinearConstraint::eq(vec![], 1)],
                    vec![],
                    ObjectiveSense::Minimize,
                )
                .map_err(construct)?,
                lengths: lengths.to_vec(),
            });
        }
        let pairs = n
            .checked_mul(n.saturating_sub(1))
            .and_then(|count| n.checked_add(count / 2))
            .ok_or_else(|| {
                crate::rules::ReductionError::integer_overflow::<Self, ILP<i64, i64, Bounded>>(
                    "counting sequencing variables",
                )
            })?;
        let mut variables = Vec::with_capacity(pairs);
        for j in 0..n {
            variables.push(
                IntegerVariable::new(Some(releases[j]), Some(deadlines[j] - lengths[j]))
                    .map_err(construct)?,
            );
        }
        let mut constraints = Vec::new();
        for i in 0..n {
            for j in i + 1..n {
                let y = variables.len();
                // Relabel identical tasks into index order; their source data are interchangeable.
                let identical = (lengths[i], releases[i], deadlines[i])
                    == (lengths[j], releases[j], deadlines[j]);
                variables.push(
                    IntegerVariable::new(Some(i64::from(identical)), Some(1)).map_err(construct)?,
                );
                let forward = (deadlines[i] - releases[j]).max(0);
                let backward = (deadlines[j] - releases[i]).max(0);
                constraints.push(LinearConstraint::ge(
                    vec![(j, 1), (i, -1), (y, -forward)],
                    lengths[i] - forward,
                ));
                constraints.push(LinearConstraint::ge(
                    vec![(i, 1), (j, -1), (y, backward)],
                    lengths[j],
                ));
            }
        }
        let target = ILP::with_variables(variables, constraints, vec![], ObjectiveSense::Minimize)
            .map_err(construct)?;
        crate::rules::ilp_helpers::validate_bounded_constraint_arithmetic::<Self>(&target)?;
        Ok(Self::Result {
            target,
            lengths: lengths.to_vec(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ReductionSWRTDToILP {
    bounded: ReductionSWRTDToBoundedILP,
    binary: crate::rules::ilp_i64_ilp_bool::ReductionIntILPToBinaryILP,
}

impl ReductionResult for ReductionSWRTDToILP {
    type Source = SequencingWithReleaseTimesAndDeadlines;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &Self::Target {
        self.binary.target_problem()
    }

    fn extract_solution(&self, values: &Vec<i64>) -> crate::rules::ExtractionResult<Vec<usize>> {
        self.bounded
            .extract_solution(&self.binary.extract_solution(values)?)
    }
}

#[crate::aggregate_reduction(ilp_feasibility)]
impl crate::rules::AggregateReductionResult for ReductionSWRTDToILP {}

#[reduction(transform = upper_bound {
    num_vars = "num_tasks * time_horizon_bits + num_tasks * (num_tasks - 1) / 2",
    num_constraints = "num_tasks * (num_tasks - 1) + 1",
    num_nonzeros = "num_tasks * (num_tasks - 1) * (2 * time_horizon_bits + 1)",
    max_constraint_magnitude_bits = "time_horizon_bits",
})]
impl ReduceTo<ILP<bool>> for SequencingWithReleaseTimesAndDeadlines {
    type Result = ReductionSWRTDToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let bounded = ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(self)?;
        let binary = ReduceTo::<ILP<bool>>::reduce_to(bounded.target_problem())?;
        Ok(Self::Result { bounded, binary })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::example_db::specs::{
        rule_example_via_bounded_ilp, rule_example_via_ilp, RuleExampleSpec,
    };
    fn source() -> SequencingWithReleaseTimesAndDeadlines {
        SequencingWithReleaseTimesAndDeadlines::new(vec![1, 2, 1], vec![0, 0, 2], vec![3, 3, 4])
    }
    vec![
        RuleExampleSpec {
            id: "sequencingwithreleasetimesanddeadlines_to_ilp",
            build: || rule_example_via_ilp::<_, bool>(source()),
        },
        RuleExampleSpec {
            id: "sequencingwithreleasetimesanddeadlines_to_bounded_ilp",
            build: || rule_example_via_bounded_ilp(source()),
        },
    ]
}

#[cfg(test)]
#[path = "../unit_tests/rules/sequencingwithreleasetimesanddeadlines_ilp.rs"]
mod tests;
