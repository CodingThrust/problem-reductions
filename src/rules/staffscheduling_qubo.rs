//! Bounded worker multiplicities with exact budget and coverage residuals.
use crate::models::{algebraic::QUBO, misc::StaffScheduling, Decision};
use crate::rules::ilp_i64_ilp_bool::binary_weights;
use crate::rules::qubo_helpers::QuboBuilder;
use crate::rules::traits::validate_target_witness;
use crate::rules::{ExtractionResult, ReduceTo, ReductionError, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionStaffSchedulingToQUBO {
    target: Decision<QUBO<i64>>,
    weights: Vec<i64>,
    num_schedules: usize,
}

impl ReductionResult for ReductionStaffSchedulingToQUBO {
    type Source = StaffScheduling;
    type Target = Decision<QUBO<i64>>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, bits: &Vec<bool>) -> ExtractionResult<Vec<usize>> {
        validate_target_witness(
            &self.target,
            bits,
            |value| value.0,
            "QUBO does not certify a staffing assignment",
        )?;
        (0..self.num_schedules)
            .map(|schedule| {
                let count: i64 = self
                    .weights
                    .iter()
                    .enumerate()
                    .filter(|(bit, _)| bits[schedule * self.weights.len() + bit])
                    .map(|(_, &weight)| weight)
                    .sum();
                usize::try_from(count).map_err(|_| {
                    crate::rules::ExtractionError::invalid("worker count does not fit usize")
                })
            })
            .collect()
    }
}

#[crate::aggregate_reduction(identity)]
impl crate::rules::AggregateReductionResult for ReductionStaffSchedulingToQUBO {}

#[crate::reduction(transform = upper_bound {
    num_vars = "63 * (num_schedules + num_periods + 1)",
    num_quadratic_terms = "(63 * (num_schedules + num_periods + 1))^2",
})]
impl ReduceTo<Decision<QUBO<i64>>> for StaffScheduling {
    type Result = ReductionStaffSchedulingToQUBO;

    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        type Builder = QuboBuilder<StaffScheduling, Decision<QUBO<i64>>>;
        let overflow = Builder::overflow;
        let workers = self.num_workers();
        // Certify the source's count fold before any target coefficient cancellation.
        Self::exact_i64(self.num_schedules(), "counting schedule patterns")?
            .checked_mul(workers)
            .ok_or_else(overflow)?;
        if self
            .requirements()
            .iter()
            .any(|&required| required > workers)
        {
            let (target, _) = Builder::new(0).finish()?;
            return Ok(Self::Result {
                target: Decision::new(target, -1),
                weights: Vec::new(),
                num_schedules: self.num_schedules(),
            });
        }

        let weights = binary_weights(workers);
        let count_bits = Builder::dimension(self.num_schedules(), weights.len())?;
        let mut num_vars = count_bits.checked_add(weights.len()).ok_or_else(overflow)?;
        let surpluses: Vec<_> = self
            .requirements()
            .iter()
            .enumerate()
            .filter(|(_, required)| **required > 0)
            .map(|(period, &required)| (period, required, binary_weights(workers - required)))
            .collect();
        for (_, _, surplus) in &surpluses {
            num_vars = num_vars.checked_add(surplus.len()).ok_or_else(overflow)?;
        }
        isize::try_from(Builder::dimension(
            num_vars,
            std::mem::size_of::<(usize, i64)>(),
        )?)
        .map_err(|_| overflow())?;

        let mut q = Builder::new(num_vars);
        let mut budget = Vec::new();
        for schedule in 0..self.num_schedules() {
            budget.extend(
                weights
                    .iter()
                    .enumerate()
                    .map(|(bit, &weight)| (schedule * weights.len() + bit, weight)),
            );
        }
        budget.extend(
            weights
                .iter()
                .enumerate()
                .map(|(bit, &weight)| (count_bits + bit, weight)),
        );
        q.affine_square(&budget, workers)?;
        let mut offset = count_bits + weights.len();
        for (period, required, surplus) in surpluses {
            let mut coverage = Vec::new();
            for (schedule, pattern) in self.schedules().iter().enumerate() {
                if pattern[period] {
                    coverage.extend(
                        weights
                            .iter()
                            .enumerate()
                            .map(|(bit, &weight)| (schedule * weights.len() + bit, weight)),
                    );
                }
            }
            coverage.extend(
                surplus
                    .iter()
                    .enumerate()
                    .map(|(bit, &weight)| (offset + bit, -weight)),
            );
            q.affine_square(&coverage, required)?;
            offset += surplus.len();
        }
        let (target, constant) = q.finish()?;
        Ok(Self::Result {
            target: Decision::new(target, constant.checked_neg().ok_or_else(overflow)?),
            weights,
            num_schedules: self.num_schedules(),
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "staffscheduling_qubo",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, Decision<QUBO<i64>>>(
                StaffScheduling::new(
                    2,
                    vec![
                        vec![true, true, false],
                        vec![false, true, true],
                        vec![true, false, true],
                    ],
                    vec![2, 2, 2],
                    3,
                ),
                crate::export::SolutionPair {
                    source_config: serde_json::json!([1, 1, 1]),
                    target_config: serde_json::json!([
                        true, false, true, false, true, false, false, false, false, false, false
                    ]),
                },
            )
        },
    }]
}
