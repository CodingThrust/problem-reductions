//! Direct integer encoding of ShortestCommonSupersequence.
use crate::models::algebraic::QUBO;
use crate::models::misc::ShortestCommonSupersequence;
use crate::rules::qubo_helpers::decode_rows;
use crate::rules::qubo_helpers::ObjectiveEncoding;
use crate::rules::qubo_helpers::QuboBuilder;
use crate::rules::{ExtractionResult, ReduceTo, ReductionError, ReductionResult};
use crate::types::Min;
#[derive(Debug, Clone)]
pub struct ReductionShortestCommonSupersequenceToQUBO {
    encoding: ObjectiveEncoding,
    length: usize,
    alphabet: usize,
}
impl ReductionResult for ReductionShortestCommonSupersequenceToQUBO {
    type Source = ShortestCommonSupersequence;
    type Target = QUBO<i64>;
    fn target_problem(&self) -> &Self::Target {
        &self.encoding.target
    }
    fn extract_solution(
        &self,
        bits: &Vec<bool>,
    ) -> ExtractionResult<<Self::Source as crate::Problem>::Solution> {
        self.encoding.validate(bits)?;
        Ok(
            decode_rows(bits, std::iter::repeat_n(self.alphabet + 1, self.length))?
                .into_iter()
                .map(|s| (s < self.alphabet).then_some(s))
                .collect(),
        )
    }
}
#[crate::aggregate_reduction]
impl crate::rules::AggregateReductionResult for ReductionShortestCommonSupersequenceToQUBO {
    type Source = ShortestCommonSupersequence;
    type Target = QUBO<i64>;
    fn target_problem(&self) -> &Self::Target {
        &self.encoding.target
    }
    fn extract_value(&self, value: Min<i64>) -> Min<i64> {
        self.encoding.value(value)
    }
}
#[crate::reduction(transform = { exact { num_vars = "max_length * (alphabet_size + 1) + total_length * max_length" }, upper_bound { num_quadratic_terms = "(max_length * (alphabet_size + 1) + total_length * max_length)^2" } })]
impl ReduceTo<QUBO<i64>> for ShortestCommonSupersequence {
    type Result = ReductionShortestCommonSupersequenceToQUBO;
    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        type Builder = QuboBuilder<ShortestCommonSupersequence, QUBO<i64>>;
        let length = self.max_length();
        let alphabet = self.alphabet_size();
        let columns = alphabet.checked_add(1).ok_or_else(Builder::overflow)?;
        let symbols = Builder::dimension(length, columns)?;
        let embeddings = Builder::dimension(self.total_length(), length)?;
        let count = symbols
            .checked_add(embeddings)
            .ok_or_else(Builder::overflow)?;
        if length > 0 {
            isize::try_from(Builder::dimension(
                columns.max(length),
                std::mem::size_of::<usize>(),
            )?)
            .map_err(|_| Builder::overflow())?;
        }
        let high = i64::try_from(length).map_err(|_| Builder::overflow())?;
        let penalty = high.checked_add(1).ok_or_else(Builder::overflow)?;
        let p = i128::from(penalty);
        let mut q = Builder::new(count);
        q.add_constant(i128::from(high))?;
        for position in 0..length {
            q.add(
                position * columns + alphabet,
                position * columns + alphabet,
                -1,
            )?;
        }
        q.assignment(length, columns, false, penalty)?;
        let mut character = 0;
        for string in self.strings() {
            for (index, &symbol) in string.iter().enumerate() {
                let offset = symbols + character * length;
                q.square(&(offset..offset + length).collect::<Vec<_>>(), 1, penalty)?;
                for position in 0..length {
                    q.add(offset + position, offset + position, p)?;
                    if symbol < alphabet {
                        q.add(offset + position, position * columns + symbol, -p)?;
                    }
                    if index + 1 < string.len() {
                        for next in 0..=position {
                            q.add(offset + position, offset + length + next, p)?;
                        }
                    }
                }
                character += 1;
            }
        }
        for position in 0..length.saturating_sub(1) {
            let a = position * columns + alphabet;
            q.add(a, a, p)?;
            q.add(a, a + columns, -p)?;
        }
        Ok(Self::Result {
            encoding: q.optimization(0, high)?,
            length,
            alphabet,
        })
    }
}
#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "shortestcommonsupersequence_qubo",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, QUBO<i64>>(
                ShortestCommonSupersequence::new(2, vec![vec![0], vec![1]]),
                crate::export::SolutionPair {
                    source_config: serde_json::json!([Some(0), Some(1)]),
                    target_config: serde_json::json!([
                        true, false, false, false, true, false, true, false, false, true
                    ]),
                },
            )
        },
    }]
}
