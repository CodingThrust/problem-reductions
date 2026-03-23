//! Longest Common Subsequence (LCS) problem implementation.
//!
//! Given a finite alphabet, a set of strings over that alphabet, and a bound
//! `K`, determine whether there exists a common subsequence of length exactly
//! `K`. This fixed-length witness model is equivalent to the standard
//! "length at least `K`" decision formulation.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{Problem, WitnessProblem};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "LongestCommonSubsequence",
        display_name: "Longest Common Subsequence",
        aliases: &["LCS"],
        dimensions: &[],
        module_path: module_path!(),
        description: "Find a common subsequence of bounded length for a set of strings",
        fields: &[
            FieldInfo { name: "alphabet_size", type_name: "usize", description: "Size of the alphabet" },
            FieldInfo { name: "strings", type_name: "Vec<Vec<usize>>", description: "Input strings over the alphabet {0, ..., alphabet_size-1}" },
            FieldInfo { name: "bound", type_name: "usize", description: "Required length of the common subsequence witness" },
        ],
    }
}

/// The Longest Common Subsequence problem.
///
/// Given an alphabet of size `k`, a set of strings over `{0, ..., k-1}`, and a
/// bound `K`, determine whether there exists a string `w` of length exactly `K`
/// such that `w` is a subsequence of every input string. This is equivalent to
/// the standard decision version with `|w| >= K`, because any longer witness has
/// a length-`K` prefix that is also a common subsequence.
///
/// # Representation
///
/// The configuration is a vector of length `bound`, where each entry is a
/// symbol in `{0, ..., alphabet_size-1}`. The instance is satisfiable iff that
/// candidate witness is a subsequence of every input string.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongestCommonSubsequence {
    alphabet_size: usize,
    strings: Vec<Vec<usize>>,
    bound: usize,
}

impl LongestCommonSubsequence {
    /// Create a new LongestCommonSubsequence instance.
    ///
    /// # Panics
    ///
    /// Panics if `alphabet_size == 0` while the witness length is positive or
    /// any input string is non-empty, or if an input symbol is outside the
    /// declared alphabet.
    pub fn new(alphabet_size: usize, strings: Vec<Vec<usize>>, bound: usize) -> Self {
        assert!(
            alphabet_size > 0 || (bound == 0 && strings.iter().all(|s| s.is_empty())),
            "alphabet_size must be > 0 when bound > 0 or any input string is non-empty"
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
            bound,
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

    /// Returns the witness-length bound.
    pub fn bound(&self) -> usize {
        self.bound
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

    /// Returns the number of adjacent witness-position transitions.
    pub fn num_transitions(&self) -> usize {
        self.bound.saturating_sub(1)
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
    type Value = bool;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![self.alphabet_size; self.bound]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        if config.len() != self.bound {
            return false;
        }
        if config.iter().any(|&symbol| symbol >= self.alphabet_size) {
            return false;
        }
        self.strings.iter().all(|s| is_subsequence(config, s))
    }
}

impl WitnessProblem for LongestCommonSubsequence {}

crate::declare_variants! {
    default sat LongestCommonSubsequence => "alphabet_size ^ bound",
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
            3,
        )),
        optimal_config: vec![0, 1, 0],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/longest_common_subsequence.rs"]
mod tests;
