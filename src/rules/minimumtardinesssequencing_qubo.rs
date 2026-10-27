//! Direct quadratic encoding of MinimumTardinessSequencing<One>.
use crate::models::algebraic::QUBO;
use crate::models::misc::MinimumTardinessSequencing;
use crate::rules::qubo_helpers::{decode_rows, invert_permutation, ObjectiveEncoding, QuboBuilder};
use crate::rules::{ExtractionResult, ReduceTo, ReductionError, ReductionResult};
use crate::types::Min;
use crate::types::One;

#[derive(Debug, Clone)]
pub struct ReductionMinimumTardinessSequencingToQUBO {
    encoding: ObjectiveEncoding,
    num_tasks: usize,
}

impl ReductionResult for ReductionMinimumTardinessSequencingToQUBO {
    type Source = MinimumTardinessSequencing<One>;
    type Target = QUBO<i64>;
    fn target_problem(&self) -> &Self::Target {
        &self.encoding.target
    }
    fn extract_solution(&self, bits: &Vec<bool>) -> ExtractionResult<Vec<usize>> {
        self.encoding.validate(bits)?;
        invert_permutation(&decode_rows(
            bits,
            std::iter::repeat_n(self.num_tasks, self.num_tasks),
        )?)
    }
}

#[crate::aggregate_reduction]
impl crate::rules::AggregateReductionResult for ReductionMinimumTardinessSequencingToQUBO {
    type Source = MinimumTardinessSequencing<One>;
    type Target = QUBO<i64>;
    fn target_problem(&self) -> &Self::Target {
        &self.encoding.target
    }
    fn extract_value(&self, value: Min<i64>) -> Min<i64> {
        self.encoding.value(value)
    }
}

#[crate::reduction(transform = {
    exact { num_vars = "num_tasks^2" },
    upper_bound { num_quadratic_terms = "num_tasks^2 * (num_tasks^2 - 1) / 2" },
})]
impl ReduceTo<QUBO<i64>> for MinimumTardinessSequencing<One> {
    type Result = ReductionMinimumTardinessSequencingToQUBO;
    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        type Builder = QuboBuilder<MinimumTardinessSequencing<One>, QUBO<i64>>;
        let n = self.num_tasks();
        let mut q = Builder::new(Builder::dimension(n, n)?);
        for i in 0..n {
            for p in 0..n {
                let completion = i64::try_from(p)
                    .ok()
                    .and_then(|v| v.checked_add(1))
                    .ok_or_else(Builder::overflow)?;
                if completion > self.deadlines()[i] {
                    q.add(i * n + p, i * n + p, 1)?;
                }
            }
        }
        let (penalty, low, high) = q.penalty()?;
        q.assignment(n, n, true, penalty)?;
        for &(a, b) in self.precedences() {
            for p in 0..n {
                for s in 0..=p {
                    q.add(a * n + p, b * n + s, i128::from(penalty))?;
                }
            }
        }
        Ok(ReductionMinimumTardinessSequencingToQUBO {
            encoding: q.optimization(low, high)?,
            num_tasks: n,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimumtardinesssequencing_to_qubo",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, QUBO<i64>>(
                MinimumTardinessSequencing::<One>::new(3, vec![1, 2, 3], vec![(0, 1)]),
                crate::export::SolutionPair {
                    source_config: serde_json::json!([0, 1, 2]),
                    target_config: serde_json::json!([
                        true, false, false, false, true, false, false, false, true
                    ]),
                },
            )
        },
    }]
}
