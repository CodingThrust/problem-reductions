//! Start-slot encoding with one-hot and overlap penalties.
use crate::models::algebraic::QUBO;
use crate::models::decision::Decision;
use crate::models::misc::SequencingWithinIntervals;
use crate::rules::qubo_helpers::{QuboBuilder, decode_rows};
use crate::rules::traits::validate_target_witness;
use crate::rules::{ExtractionResult, ReduceTo, ReductionError, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionSequencingWithinIntervalsToQUBO {
    target: Decision<QUBO<i64>>,
    lengths: Vec<usize>,
}
impl ReductionResult for ReductionSequencingWithinIntervalsToQUBO {
    type Source = SequencingWithinIntervals;
    type Target = Decision<QUBO<i64>>;
    fn target_problem(&self) -> &Self::Target {
        &self.target
    }
    fn extract_solution(&self, bits: &Vec<bool>) -> ExtractionResult<Vec<usize>> {
        validate_target_witness(
            &self.target,
            bits,
            |v| v.0,
            "QUBO does not certify a nonoverlapping schedule",
        )?;
        decode_rows(bits, self.lengths.iter().copied())
    }
}
#[crate::aggregate_reduction(identity)]
impl crate::rules::AggregateReductionResult for ReductionSequencingWithinIntervalsToQUBO {}

#[crate::reduction(transform = {
    exact { num_vars = "num_start_slots" },
    upper_bound { num_quadratic_terms = "num_start_slots * (num_start_slots - 1) / 2" },
})]
impl ReduceTo<Decision<QUBO<i64>>> for SequencingWithinIntervals {
    type Result = ReductionSequencingWithinIntervalsToQUBO;
    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        type Builder = QuboBuilder<SequencingWithinIntervals, Decision<QUBO<i64>>>;
        let num_slots = self.num_start_slots();
        std::alloc::Layout::array::<(usize, i64, i64)>(num_slots)
            .map_err(|_| Builder::overflow())?;
        // Each row's usize index vector is no larger than the checked slot array.
        // The source constructor also checks the total, so offset + length fits.
        let lengths: Vec<_> = self.start_slot_counts().collect();
        let mut q = Builder::new(num_slots);
        let mut slots = Vec::with_capacity(num_slots);
        for (i, &length) in lengths.iter().enumerate() {
            let offset = slots.len();
            q.square(&(offset..offset + length).collect::<Vec<_>>(), 1, 1)?;
            for t in 0..length {
                // The start domain guarantees both endpoints are bounded by the deadline.
                let start =
                    self.release_times()[i] + i64::try_from(t).map_err(|_| Builder::overflow())?;
                slots.push((i, start, start + self.lengths()[i]));
            }
        }
        for (a, &(i, start, end)) in slots.iter().enumerate() {
            for (b, &(j, other_start, other_end)) in slots.iter().enumerate().skip(a + 1) {
                if i != j && !(end <= other_start || other_end <= start) {
                    q.add(a, b, 1)?;
                }
            }
        }
        let (target, constant) = q.finish()?;
        let bound = constant.checked_neg().ok_or_else(Builder::overflow)?;
        Ok(ReductionSequencingWithinIntervalsToQUBO {
            target: Decision::new(target, bound),
            lengths,
        })
    }
}
#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "sequencingwithinintervals_to_qubo",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, Decision<QUBO<i64>>>(
                SequencingWithinIntervals::new(vec![0, 1, 2], vec![1, 2, 3], vec![1; 3]).unwrap(),
                crate::export::SolutionPair {
                    source_config: serde_json::json!([0, 0, 0]),
                    target_config: serde_json::json!([true, true, true]),
                },
            )
        },
    }]
}
