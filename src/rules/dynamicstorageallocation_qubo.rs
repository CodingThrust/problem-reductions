//! Direct integer encoding of DynamicStorageAllocation.
use crate::models::algebraic::QUBO;
use crate::models::decision::Decision;
use crate::models::misc::DynamicStorageAllocation;
use crate::rules::qubo_helpers::decode_rows;
use crate::rules::qubo_helpers::QuboBuilder;
use crate::rules::traits::validate_target_witness;
use crate::rules::{ExtractionResult, ReduceTo, ReductionError, ReductionResult};
#[derive(Debug, Clone)]
pub struct ReductionDynamicStorageAllocationToQUBO {
    target: Decision<QUBO<i64>>,
    counts: Vec<usize>,
}
impl ReductionResult for ReductionDynamicStorageAllocationToQUBO {
    type Source = DynamicStorageAllocation;
    type Target = Decision<QUBO<i64>>;
    fn target_problem(&self) -> &Self::Target {
        &self.target
    }
    fn extract_solution(
        &self,
        bits: &Vec<bool>,
    ) -> ExtractionResult<<Self::Source as crate::Problem>::Solution> {
        validate_target_witness(
            &self.target,
            bits,
            |v| v.0,
            "QUBO does not certify a source witness",
        )?;
        decode_rows(bits, self.counts.iter().copied())
    }
}
#[crate::aggregate_reduction(identity)]
impl crate::rules::AggregateReductionResult for ReductionDynamicStorageAllocationToQUBO {}
#[crate::reduction(transform = upper_bound { num_vars = "num_items * memory_size", num_quadratic_terms = "(num_items * memory_size)^2" })]
impl ReduceTo<Decision<QUBO<i64>>> for DynamicStorageAllocation {
    type Result = ReductionDynamicStorageAllocationToQUBO;
    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        type Builder = QuboBuilder<DynamicStorageAllocation, Decision<QUBO<i64>>>;
        let counts: Vec<_> = self
            .items()
            .iter()
            .map(|&(_, _, s)| self.memory_size() - s + 1)
            .collect();
        let count = counts
            .iter()
            .try_fold(0usize, |s, &v| s.checked_add(v))
            .ok_or_else(Builder::overflow)?;
        let mut q = Builder::new(count);
        isize::try_from(Builder::dimension(
            count,
            std::mem::size_of::<(usize, usize, usize)>(),
        )?)
        .map_err(|_| Builder::overflow())?;
        let mut slots = Vec::with_capacity(count);
        for (i, &choices) in counts.iter().enumerate() {
            let start = slots.len();
            q.square(&(start..start + choices).collect::<Vec<_>>(), 1, 1)?;
            for address in 0..choices {
                slots.push((i, address, address + self.items()[i].2));
            }
        }
        for (a, &(i, start, end)) in slots.iter().enumerate() {
            for (b, &(j, other_start, other_end)) in slots.iter().enumerate().skip(a + 1) {
                let (arrival, departure, _) = self.items()[i];
                let (other_arrival, other_departure, _) = self.items()[j];
                if i != j
                    && arrival < other_departure
                    && other_arrival < departure
                    && start < other_end
                    && other_start < end
                {
                    q.add(a, b, 1)?;
                }
            }
        }
        let (target, constant) = q.finish()?;
        let bound = constant.checked_neg().ok_or_else(Builder::overflow)?;
        Ok(Self::Result {
            target: Decision::new(target, bound),
            counts,
        })
    }
}
#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "dynamicstorageallocation_qubo",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, Decision<QUBO<i64>>>(
                DynamicStorageAllocation::new(vec![(0, 1, 1), (1, 2, 1), (0, 2, 1)], 2),
                crate::export::SolutionPair {
                    source_config: serde_json::json!([0, 0, 1]),
                    target_config: serde_json::json!([true, false, true, false, false, true]),
                },
            )
        },
    }]
}
