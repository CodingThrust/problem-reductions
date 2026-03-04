//! Longest Common Subsequence problem implementation.
//!
//! Given k strings, find the longest common subsequence shared by all strings.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "LongestCommonSubsequence",
        module_path: module_path!(),
        description: "Find the longest common subsequence of k strings",
        fields: &[
            FieldInfo { name: "strings", type_name: "Vec<Vec<u8>>", description: "The input strings (at least 2)" },
        ],
    }
}

/// The Longest Common Subsequence problem.
///
/// Given `k >= 2` strings over alphabet `u8`, find the longest
/// subsequence that appears in all strings.
///
/// # Representation
///
/// Variables correspond to positions in the shortest string.
/// Each variable is binary (0 = exclude, 1 = include).
/// A configuration selects a subsequence of the shortest string;
/// it is valid if that subsequence is also a subsequence of every other string.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::LongestCommonSubsequence;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// let problem = LongestCommonSubsequence::new(vec![
///     vec![b'A', b'B', b'A', b'C'],
///     vec![b'B', b'A', b'C', b'A'],
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
    /// Create a new LCS problem from a collection of strings.
    ///
    /// # Panics
    ///
    /// Panics if fewer than 2 strings are provided.
    pub fn new(strings: Vec<Vec<u8>>) -> Self {
        assert!(strings.len() >= 2, "LCS requires at least 2 strings");
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

    /// Get the total length of all strings combined.
    pub fn total_length(&self) -> usize {
        self.strings.iter().map(|s| s.len()).sum()
    }

    /// Get the index of the shortest string.
    fn shortest_index(&self) -> usize {
        self.strings
            .iter()
            .enumerate()
            .min_by_key(|(_, s)| s.len())
            .map(|(i, _)| i)
            .unwrap_or(0)
    }

    /// Get the length of the shortest string.
    fn shortest_len(&self) -> usize {
        self.strings.iter().map(|s| s.len()).min().unwrap_or(0)
    }
}

/// Check if `subseq` is a subsequence of `string`.
fn is_subsequence(subseq: &[u8], string: &[u8]) -> bool {
    let mut it = string.iter();
    for &ch in subseq {
        if !it.any(|&c| c == ch) {
            return false;
        }
    }
    true
}

impl Problem for LongestCommonSubsequence {
    const NAME: &'static str = "LongestCommonSubsequence";
    type Metric = SolutionSize<usize>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.shortest_len()]
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<usize> {
        let shortest_len = self.shortest_len();
        if config.len() != shortest_len {
            return SolutionSize::Invalid;
        }
        // Check all values are binary
        if config.iter().any(|&v| v >= 2) {
            return SolutionSize::Invalid;
        }

        let si = self.shortest_index();
        let shortest = &self.strings[si];

        // Build the candidate subsequence from selected positions
        let subseq: Vec<u8> = config
            .iter()
            .enumerate()
            .filter(|(_, &v)| v == 1)
            .map(|(i, _)| shortest[i])
            .collect();

        // Check if it is a subsequence of every other string
        for (i, s) in self.strings.iter().enumerate() {
            if i == si {
                continue;
            }
            if !is_subsequence(&subseq, s) {
                return SolutionSize::Invalid;
            }
        }

        SolutionSize::Valid(subseq.len())
    }
}

impl OptimizationProblem for LongestCommonSubsequence {
    type Value = usize;

    fn direction(&self) -> Direction {
        Direction::Maximize
    }
}

crate::declare_variants! {
    LongestCommonSubsequence => "2^total_length",
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/longest_common_subsequence.rs"]
mod tests;
