//! Longest Common Subsequence (LCS) problem implementation.
//!
//! Given a set of strings over a finite alphabet, find the longest string
//! that is a subsequence of every input string. Polynomial-time for 2 strings
//! via dynamic programming, but NP-hard for k >= 3 strings (Maier, 1978).

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};
use serde::{Deserialize, Serialize};

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
/// Given `k` strings `s_1, ..., s_k` over a finite alphabet, find a longest
/// string `w` that is a subsequence of every `s_i`.
///
/// A string `w` is a **subsequence** of `s` if `w` can be obtained by deleting
/// zero or more characters from `s` without changing the order of the remaining
/// characters.
///
/// # Representation
///
/// The configuration is a binary vector of length equal to the shortest string.
/// Each entry indicates whether the corresponding character of the shortest
/// string is included in the candidate subsequence.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::LongestCommonSubsequence;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// let problem = LongestCommonSubsequence::new(vec![
///     vec![b'A', b'B', b'C'],
///     vec![b'A', b'C', b'B'],
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
    /// Create a new LCS instance from a list of strings.
    ///
    /// # Panics
    /// Panics if fewer than 2 strings are provided.
    pub fn new(strings: Vec<Vec<u8>>) -> Self {
        assert!(
            strings.len() >= 2,
            "LCS requires at least 2 strings, got {}",
            strings.len()
        );
        Self { strings }
    }

    /// Get the input strings.
    pub fn strings(&self) -> &[Vec<u8>] {
        &self.strings
    }

    /// Get the number of strings.
    pub fn num_strings(&self) -> usize {
        self.strings.len()
    }

    /// Get the total length of all strings.
    pub fn total_length(&self) -> usize {
        self.strings.iter().map(|s| s.len()).sum()
    }

    /// Get the length of the first string.
    pub fn num_chars_first(&self) -> usize {
        self.strings[0].len()
    }

    /// Get the length of the second string.
    pub fn num_chars_second(&self) -> usize {
        self.strings[1].len()
    }

    /// Index of the shortest string.
    fn shortest_index(&self) -> usize {
        self.strings
            .iter()
            .enumerate()
            .min_by_key(|(_, s)| s.len())
            .map(|(i, _)| i)
            .unwrap()
    }

    /// Length of the shortest string.
    pub fn min_string_length(&self) -> usize {
        self.strings[self.shortest_index()].len()
    }
}

/// Check if `candidate` is a subsequence of `target`.
fn is_subsequence(candidate: &[u8], target: &[u8]) -> bool {
    let mut ti = 0;
    for &ch in candidate {
        while ti < target.len() && target[ti] != ch {
            ti += 1;
        }
        if ti >= target.len() {
            return false;
        }
        ti += 1;
    }
    true
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
        let shortest_idx = self.shortest_index();
        let shortest = &self.strings[shortest_idx];

        if config.len() != shortest.len() {
            return SolutionSize::Invalid;
        }
        if config.iter().any(|&v| v >= 2) {
            return SolutionSize::Invalid;
        }

        // Build candidate subsequence from selected positions of shortest string
        let candidate: Vec<u8> = config
            .iter()
            .enumerate()
            .filter(|(_, &x)| x == 1)
            .map(|(i, _)| shortest[i])
            .collect();

        // Check that candidate is a subsequence of ALL strings
        for (i, s) in self.strings.iter().enumerate() {
            if i == shortest_idx {
                // The candidate is always a subsequence of the string it was built from
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

crate::declare_variants! {
    default opt LongestCommonSubsequence => "2^min_string_length",
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/longest_common_subsequence.rs"]
mod tests;
