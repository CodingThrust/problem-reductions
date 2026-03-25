//! Longest Common Subsequence (LCS) problem implementation.
//!
//! Given a finite alphabet and a set of strings over that alphabet, find a
//! longest common subsequence. The configuration is a fixed-length vector of
//! `max_length` positions, where each entry is either a valid symbol or the
//! padding symbol (`alphabet_size`). Padding must be contiguous at the end.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Max;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "LongestCommonSubsequence",
        display_name: "Longest Common Subsequence",
        aliases: &["LCS"],
        dimensions: &[],
        module_path: module_path!(),
        description: "Find a longest common subsequence for a set of strings",
        fields: &[
            FieldInfo { name: "alphabet_size", type_name: "usize", description: "Size of the alphabet" },
            FieldInfo { name: "strings", type_name: "Vec<Vec<usize>>", description: "Input strings over the alphabet {0, ..., alphabet_size-1}" },
            FieldInfo { name: "max_length", type_name: "usize", description: "Maximum possible subsequence length (min of string lengths)" },
        ],
    }
}

/// The Longest Common Subsequence problem.
///
/// Given an alphabet of size `k` and a set of strings over `{0, ..., k-1}`,
/// find a longest string `w` that is a subsequence of every input string.
///
/// # Representation
///
/// The configuration is a vector of length `max_length`, where each entry is a
/// symbol in `{0, ..., alphabet_size}`. The value `alphabet_size` is the
/// padding symbol. Padding must be contiguous at the end of the vector. The
/// effective subsequence consists of all non-padding symbols (the prefix before
/// padding starts). The objective is to maximize the effective length.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongestCommonSubsequence {
    alphabet_size: usize,
    strings: Vec<Vec<usize>>,
    max_length: usize,
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
            max_length >= 1 || strings.is_empty(),
            "at least one string must be non-empty"
        );
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
    type Value = Max<usize>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![self.alphabet_size + 1; self.max_length]
    }

    fn evaluate(&self, config: &[usize]) -> Max<usize> {
        if config.len() != self.max_length {
            return Max(None);
        }

        let padding = self.alphabet_size;

        // Find effective length = index of first padding symbol (or max_length if no padding).
        let effective_length = config
            .iter()
            .position(|&s| s == padding)
            .unwrap_or(self.max_length);

        // Verify all positions after the first padding are also padding (no interleaved padding).
        if config[effective_length..].iter().any(|&s| s != padding) {
            return Max(None);
        }

        // Extract the non-padding prefix as the candidate subsequence.
        let prefix = &config[..effective_length];

        // Check all symbols in prefix are valid (0..alphabet_size).
        if prefix.iter().any(|&s| s >= self.alphabet_size) {
            return Max(None);
        }

        // Check the prefix is a subsequence of every input string.
        if !self.strings.iter().all(|s| is_subsequence(prefix, s)) {
            return Max(None);
        }

        Max(Some(effective_length))
    }
}

crate::declare_variants! {
    default LongestCommonSubsequence => "(alphabet_size + 1) ^ max_length",
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
        optimal_config: vec![0, 0, 1, 0, 2, 2],
        optimal_value: serde_json::json!(4),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/longest_common_subsequence.rs"]
mod tests;
