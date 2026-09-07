//! Longest Common Subsequence (LCS) problem implementation.
//!
//! Given a finite alphabet and a set of strings over that alphabet, find a
//! longest common subsequence. The configuration is a fixed-length vector of
//! `max_length` positions, where each entry is either a valid symbol or `None`
//! as padding. Padding must be contiguous at the end.

use crate::registry::{CreateSpec, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Max;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "LongestCommonSubsequence",
        display_name: "Longest Common Subsequence",
        aliases: &["LCS"],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Misc,
        module_path: module_path!(),
        description: "Find a longest common subsequence for a set of strings",
        fields: LongestCommonSubsequenceCreateSpec::FIELDS,
    }
}

/// The Longest Common Subsequence problem.
///
/// Given an alphabet of size `k` and a set of strings over `{0, ..., k-1}`,
/// find a longest string `w` that is a subsequence of every input string.
///
/// # Representation
///
/// The configuration is a vector of length `max_length`, where each entry is
/// either a symbol in `{0, ..., alphabet_size - 1}` or `None` as padding.
/// Padding must be contiguous at the end of the vector. The effective
/// subsequence consists of the symbols before padding starts. The objective is
/// to maximize the effective length.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongestCommonSubsequence {
    alphabet_size: usize,
    strings: Vec<Vec<usize>>,
    max_length: usize,
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct LongestCommonSubsequenceCreateSpec {
    /// Optional alphabet size; omitted values are inferred from the strings.
    alphabet_size: Option<usize>,
    /// Input strings over the shared alphabet.
    #[create(codec = "character-rows")]
    strings: Vec<Vec<usize>>,
}

impl TryFrom<LongestCommonSubsequenceCreateSpec> for LongestCommonSubsequence {
    type Error = crate::registry::ConstructionError;

    fn try_from(spec: LongestCommonSubsequenceCreateSpec) -> Result<Self, Self::Error> {
        if !spec.strings.iter().any(|string| !string.is_empty()) {
            return Err("at least one input string must be non-empty"
                .to_string()
                .into());
        }
        let inferred_alphabet_size = spec
            .strings
            .iter()
            .flatten()
            .copied()
            .max()
            .map(|symbol| {
                symbol
                    .checked_add(1)
                    .ok_or_else(|| "inferred alphabet size overflows usize".to_string())
            })
            .transpose()?
            .unwrap_or(0);
        let alphabet_size = spec.alphabet_size.unwrap_or(inferred_alphabet_size);
        if alphabet_size < inferred_alphabet_size {
            return Err(format!(
                "alphabet size {alphabet_size} is smaller than inferred alphabet size {inferred_alphabet_size}"
            ).into());
        }
        if alphabet_size == 0 {
            return Err("alphabet size must be positive".to_string().into());
        }
        let max_length = spec.strings.iter().map(Vec::len).min().unwrap_or(0);

        Ok(Self {
            alphabet_size,
            strings: spec.strings,
            max_length,
        })
    }
}

impl LongestCommonSubsequence {
    /// Create a new LongestCommonSubsequence instance.
    ///
    /// The `max_length` is computed automatically as the minimum of all string
    /// lengths (the maximum possible common subsequence length).
    ///
    /// # Panics
    ///
    /// Panics if `alphabet_size == 0` and any input string is non-empty, or if
    /// an input symbol is outside the declared alphabet, or if all strings are
    /// empty (max_length would be 0, requiring at least one non-empty string).
    pub fn new(alphabet_size: usize, strings: Vec<Vec<usize>>) -> Self {
        let max_length = strings.iter().map(|s| s.len()).min().unwrap_or(0);
        assert!(
            alphabet_size > 0 || strings.iter().all(|s| s.is_empty()),
            "alphabet_size must be > 0 when any input string is non-empty"
        );
        assert!(
            strings
                .iter()
                .flat_map(|s| s.iter())
                .all(|&symbol| symbol < alphabet_size),
            "input symbols must be less than alphabet_size"
        );
        Self {
            alphabet_size,
            strings,
            max_length,
        }
    }

    /// Returns the alphabet size.
    pub fn alphabet_size(&self) -> usize {
        self.alphabet_size
    }

    /// Returns the input strings.
    pub fn strings(&self) -> &[Vec<usize>] {
        &self.strings
    }

    /// Returns the `max_length` field.
    pub fn max_length(&self) -> usize {
        self.max_length
    }

    /// Returns the number of input strings.
    pub fn num_strings(&self) -> usize {
        self.strings.len()
    }

    /// Returns the total input length across all strings.
    pub fn total_length(&self) -> usize {
        self.strings.iter().map(|s| s.len()).sum()
    }

    /// Returns the sum of squared string lengths.
    pub fn sum_squared_lengths(&self) -> usize {
        self.strings.iter().map(|s| s.len() * s.len()).sum()
    }

    /// Returns the sum of triangular numbers len * (len + 1) / 2 across strings.
    pub fn sum_triangular_lengths(&self) -> usize {
        self.strings
            .iter()
            .map(|s| s.len() * (s.len() + 1) / 2)
            .sum()
    }

    /// Returns the number of adjacent position transitions.
    pub fn num_transitions(&self) -> usize {
        self.max_length.saturating_sub(1)
    }

    /// Returns the cross-frequency product: the sum over each alphabet symbol
    /// of the product of that symbol's frequency across all input strings.
    ///
    /// Formally: Σ_{c ∈ 0..alphabet_size} Π_{i=1..k} count(c, strings\[i\])
    /// where count(c, s) is the number of occurrences of symbol c in string s.
    ///
    /// This equals the exact number of match-node vertices in the LCS → MaxIS
    /// reduction graph.
    pub fn cross_frequency_product(&self) -> usize {
        (0..self.alphabet_size)
            .map(|c| {
                self.strings
                    .iter()
                    .map(|s| s.iter().filter(|&&sym| sym == c).count())
                    .product::<usize>()
            })
            .sum()
    }
}

/// Check whether `candidate` is a subsequence of `target` using greedy
/// left-to-right matching.
fn is_subsequence(candidate: &[usize], target: &[usize]) -> bool {
    let mut it = target.iter();
    for &symbol in candidate {
        loop {
            match it.next() {
                Some(&next) if next == symbol => break,
                Some(_) => continue,
                None => return false,
            }
        }
    }
    true
}

impl Problem for LongestCommonSubsequence {
    const NAME: &'static str = "LongestCommonSubsequence";
    type Solution = Vec<Option<usize>>;
    type Value = Max<i64>;

    crate::problem_parameters![
        ("alphabet_size", alphabet_size),
        ("cross_frequency_product", cross_frequency_product),
        ("max_length", max_length),
        ("num_strings", num_strings),
        ("num_transitions", num_transitions),
        ("sum_triangular_lengths", sum_triangular_lengths),
        ("total_length", total_length),
    ];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Max<i64>, crate::traits::EvaluationError> {
        if config.len() != self.max_length {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "subsequence representation length does not match the bound".into(),
            ));
        }
        if config
            .iter()
            .any(|symbol| symbol.is_some_and(|value| value >= self.alphabet_size))
        {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "subsequence contains an out-of-range symbol".into(),
            ));
        }
        let config = config
            .iter()
            .map(|symbol| symbol.unwrap_or(self.alphabet_size))
            .collect::<Vec<_>>();
        Ok({
            let padding = self.alphabet_size;

            // Find effective length = index of first padding symbol (or max_length if no padding).
            let effective_length = config
                .iter()
                .position(|&s| s == padding)
                .unwrap_or(self.max_length);

            // Verify all positions after the first padding are also padding (no interleaved padding).
            if config[effective_length..].iter().any(|&s| s != padding) {
                return Ok(Max(None));
            }

            // Extract the non-padding prefix as the candidate subsequence.
            let prefix = &config[..effective_length];

            // Check the prefix is a subsequence of every input string.
            if !self.strings.iter().all(|s| is_subsequence(prefix, s)) {
                return Ok(Max(None));
            }

            Max(Some(i64::try_from(effective_length).map_err(|_| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "converting subsequence length to i64".into(),
                )
            })?))
        })
    }
}

impl crate::solvers::BruteForceProblem for LongestCommonSubsequence {
    fn dimensions(&self) -> Vec<usize> {
        vec![self.alphabet_size + 1; self.max_length]
    }
}

crate::declare_variants! {
    default LongestCommonSubsequence => "(alphabet_size + 1) ^ max_length" create LongestCommonSubsequenceCreateSpec,
}

crate::register_brute_force! {
    LongestCommonSubsequence decode |problem: &LongestCommonSubsequence, indices: Vec<usize>| indices.into_iter().map(|value| (value != problem.alphabet_size()).then_some(value)).collect(),
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "longest_common_subsequence",
        instance: Box::new(LongestCommonSubsequence::new(
            2,
            vec![
                vec![0, 1, 0, 1, 1, 0],
                vec![1, 0, 0, 1, 0, 1],
                vec![0, 0, 1, 0, 1, 1],
                vec![1, 1, 0, 0, 1, 0],
                vec![0, 1, 0, 1, 0, 1],
                vec![1, 0, 1, 0, 1, 0],
            ],
        )),
        optimal_config: serde_json::json!(vec![Some(0), Some(0), Some(1), Some(0), None, None]),
        optimal_value: serde_json::json!(4),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/longest_common_subsequence.rs"]
mod tests;
