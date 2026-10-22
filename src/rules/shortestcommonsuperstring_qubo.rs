//! Direct exact quadratic encoding of ShortestCommonSuperstring.
use crate::models::algebraic::QUBO;
use crate::models::misc::ShortestCommonSuperstring;
use crate::rules::qubo_helpers::{decode_rows, invert_permutation};
use crate::rules::qubo_helpers::{ObjectiveEncoding, QuboBuilder};
use crate::rules::{ExtractionResult, ReduceTo, ReductionError, ReductionResult};
use crate::types::Min;
#[derive(Debug, Clone)]
pub struct ReductionShortestCommonSuperstringToQUBO {
    encoding: ObjectiveEncoding,
    strings: Vec<Vec<usize>>,
    overlaps: Vec<Vec<usize>>,
    max_length: usize,
}
impl ReductionResult for ReductionShortestCommonSuperstringToQUBO {
    type Source = ShortestCommonSuperstring;
    type Target = QUBO<i64>;
    fn target_problem(&self) -> &Self::Target {
        &self.encoding.target
    }
    fn extract_solution(
        &self,
        bits: &Vec<bool>,
    ) -> ExtractionResult<<Self::Source as crate::Problem>::Solution> {
        self.encoding.validate(bits)?;
        let n = self.strings.len();
        let order = invert_permutation(&decode_rows(bits, std::iter::repeat_n(n, n))?)?;
        let mut word = Vec::new();
        for (p, &i) in order.iter().enumerate() {
            let start = if p == 0 {
                0
            } else {
                self.overlaps[order[p - 1]][i]
            };
            word.extend(self.strings[i][start..].iter().copied().map(Some));
        }
        if word.len() > self.max_length {
            return Err(crate::rules::ExtractionError::invalid(
                "overlap reconstruction exceeds the source length bound",
            ));
        }
        word.resize(self.max_length, None);
        Ok(word)
    }
}
#[crate::aggregate_reduction]
impl crate::rules::AggregateReductionResult for ReductionShortestCommonSuperstringToQUBO {
    type Source = ShortestCommonSuperstring;
    type Target = QUBO<i64>;
    fn target_problem(&self) -> &Self::Target {
        &self.encoding.target
    }
    fn extract_value(&self, value: Min<i64>) -> Min<i64> {
        self.encoding.value(value)
    }
}
#[crate::reduction(transform = upper_bound { num_vars = "num_strings^2", num_quadratic_terms = "num_strings^4" })]
impl ReduceTo<QUBO<i64>> for ShortestCommonSuperstring {
    type Result = ReductionShortestCommonSuperstringToQUBO;
    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        type Builder = QuboBuilder<ShortestCommonSuperstring, QUBO<i64>>;
        i64::try_from(self.max_length()).map_err(|_| Builder::overflow())?;
        let strings: Vec<_> = self
            .strings()
            .iter()
            .enumerate()
            .filter(|&(i, s)| {
                !s.is_empty()
                    && !self
                        .strings()
                        .iter()
                        .enumerate()
                        .any(|(j, t)| i != j && (s.len() < t.len() || j < i) && contains(t, s))
            })
            .map(|(_, s)| s.clone())
            .collect();
        let n = strings.len();
        let mut q = Builder::new(Builder::dimension(n, n)?);
        let mut overlaps = vec![vec![0; n]; n];
        for s in &strings {
            q.add_constant(i128::try_from(s.len()).map_err(|_| Builder::overflow())?)?;
        }
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }
                let overlap = (1..=strings[i].len().min(strings[j].len()))
                    .rev()
                    .find(|&k| strings[i][strings[i].len() - k..] == strings[j][..k])
                    .unwrap_or(0);
                overlaps[i][j] = overlap;
                for p in 0..n.saturating_sub(1) {
                    q.add(
                        i * n + p,
                        j * n + p + 1,
                        -i128::try_from(overlap).map_err(|_| Builder::overflow())?,
                    )?;
                }
            }
        }
        let (penalty, low, high) = q.penalty()?;
        q.assignment(n, n, true, penalty)?;
        if self
            .strings()
            .iter()
            .flatten()
            .any(|&symbol| symbol >= self.alphabet_size())
        {
            q.add_constant(i128::from(penalty))?;
        }
        Ok(ReductionShortestCommonSuperstringToQUBO {
            encoding: q.optimization(low, high)?,
            strings,
            overlaps,
            max_length: self.max_length(),
        })
    }
}
fn contains(haystack: &[usize], needle: &[usize]) -> bool {
    needle.is_empty()
        || (needle.len() <= haystack.len() && haystack.windows(needle.len()).any(|w| w == needle))
}
#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "shortestcommonsuperstring_qubo",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, QUBO<i64>>(
                ShortestCommonSuperstring::new(
                    2,
                    vec![vec![0, 1], vec![1, 0], vec![], vec![0, 1], vec![0]],
                ),
                crate::export::SolutionPair {
                    source_config: serde_json::json!(vec![
                        Some(0),
                        Some(1),
                        Some(0),
                        None,
                        None,
                        None,
                        None
                    ]),
                    target_config: serde_json::json!([true, false, false, true]),
                },
            )
        },
    }]
}
