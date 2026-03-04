//! Longest Common Subsequence problem implementation.
//!
//! Given k strings over an alphabet, find the longest string that is a
//! subsequence of every input string. NP-hard for variable k (Maier, 1978).

use serde::{Deserialize, Serialize};

use crate::{
    registry::{FieldInfo, ProblemSchemaEntry},
    traits::{OptimizationProblem, Problem},
    types::{Direction, SolutionSize},
};

inventory::submit! {
    ProblemSchemaEntry {
        name: "LongestCommonSubsequence",
        module_path: module_path!(),
        description: "Find the longest string that is a subsequence of every input string",
        fields: &[
            FieldInfo { name: "strings", type_name: "Vec<Vec<u8>>", description: "The input strings" },
        ],
    }
}

/// The Longest Common Subsequence problem.
///
/// Given `k` strings `s_1, ..., s_k` over an alphabet, find a longest
/// string `w` that is a subsequence of every `s_i`.
///
/// A string `w` is a **subsequence** of `s` if `w` can be obtained by
/// deleting zero or more characters from `s` without changing the order
/// of the remaining characters.
///
/// # Representation
///
/// Configuration is binary selection over the characters of the shortest
/// string. Each variable in `{0, 1}` indicates whether the corresponding
/// character of the shortest string is included in the candidate subsequence.
/// The candidate is valid if the resulting subsequence is also a subsequence
/// of every other input string.
///
/// # Example
///
/// ```
/// use problemreductions::{models::misc::LongestCommonSubsequence, BruteForce, Problem, Solver};
///
/// let problem = LongestCommonSubsequence::new(vec![
///     vec![b'A', b'B', b'C', b'D', b'A', b'B'],
///     vec![b'B', b'D', b'C', b'A', b'B', b'A'],
///     vec![b'B', b'C', b'A', b'D', b'B', b'A'],
/// ]);
/// let solver = BruteForce::new();
/// let solution = solver.find_best(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongestCommonSubsequence {
    /// The input strings.
    strings: Vec<Vec<u8>>,
}

impl LongestCommonSubsequence {
    /// Create a new LCS problem from a list of strings.
    ///
    /// # Panics
    ///
    /// Panics if `strings` is empty.
    pub fn new(strings: Vec<Vec<u8>>) -> Self {
        assert!(!strings.is_empty(), "must have at least one string");
        Self { strings }
    }

    /// Get the input strings.
    pub fn strings(&self) -> &[Vec<u8>] {
        &self.strings
    }

    /// Get the number of input strings.
    pub fn num_strings(&self) -> usize {
        self.strings.len()
    }

    /// Get the total length of all input strings.
    pub fn total_length(&self) -> usize {
        self.strings.iter().map(|s| s.len()).sum()
    }

    /// Index of the shortest string.
    fn shortest_index(&self) -> usize {
        self.strings
            .iter()
            .enumerate()
            .min_by_key(|(_, s)| s.len())
            .map(|(i, _)| i)
            .unwrap_or(0)
    }

    /// Length of the shortest string (upper bound on LCS length).
    pub fn min_string_length(&self) -> usize {
        self.strings.iter().map(|s| s.len()).min().unwrap_or(0)
    }
}

impl Problem for LongestCommonSubsequence {
    const NAME: &'static str = "LongestCommonSubsequence";
    type Metric = SolutionSize<i32>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.min_string_length()]
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<i32> {
        let si = self.shortest_index();
        let shortest = &self.strings[si];
        if config.len() != shortest.len() {
            return SolutionSize::Invalid;
        }
        if config.iter().any(|&v| v > 1) {
            return SolutionSize::Invalid;
        }
        // Build the candidate subsequence from selected characters
        let candidate: Vec<u8> = config
            .iter()
            .enumerate()
            .filter(|(_, &v)| v == 1)
            .map(|(i, _)| shortest[i])
            .collect();
        // Check that candidate is a subsequence of every other string
        for (j, s) in self.strings.iter().enumerate() {
            if j == si {
                continue;
            }
            if !is_subsequence(&candidate, s) {
                return SolutionSize::Invalid;
            }
        }
        SolutionSize::Valid(candidate.len() as i32)
    }
}

impl OptimizationProblem for LongestCommonSubsequence {
    type Value = i32;

    fn direction(&self) -> Direction {
        Direction::Maximize
    }
}

/// Check if `sub` is a subsequence of `full`.
fn is_subsequence(sub: &[u8], full: &[u8]) -> bool {
    let mut it = full.iter();
    for &c in sub {
        if !it.any(|&x| x == c) {
            return false;
        }
    }
    true
}

crate::declare_variants! {
    LongestCommonSubsequence => "2^min_string_length",
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/longest_common_subsequence.rs"]
mod tests;
