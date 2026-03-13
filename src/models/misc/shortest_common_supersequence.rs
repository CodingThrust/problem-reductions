//! Shortest Common Supersequence problem implementation.
//!
//! Given a set of strings over an alphabet and a bound `B`, the problem asks
//! whether there exists a common supersequence of length at most `B`. A string
//! `w` is a supersequence of `s` if `s` is a subsequence of `w` (i.e., `s` can
//! be obtained by deleting zero or more characters from `w`).
//!
//! The configuration uses a fixed-length representation of exactly `B` symbols.
//! Since any supersequence shorter than `B` can be padded with an arbitrary
//! symbol to reach length `B` (when `alphabet_size > 0`), this is equivalent
//! to the standard `|w| ≤ B` formulation. This problem is NP-hard (Maier, 1978).

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{Problem, SatisfactionProblem};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "ShortestCommonSupersequence",
        module_path: module_path!(),
        description: "Find a common supersequence of bounded length for a set of strings",
        fields: &[
            FieldInfo { name: "alphabet_size", type_name: "usize", description: "Size of the alphabet" },
            FieldInfo { name: "strings", type_name: "Vec<Vec<usize>>", description: "Input strings over the alphabet {0, ..., alphabet_size-1}" },
            FieldInfo { name: "bound", type_name: "usize", description: "Bound on supersequence length (configuration has exactly this many symbols)" },
        ],
    }
}

/// The Shortest Common Supersequence problem.
///
/// Given an alphabet of size `k`, a set of strings over `{0, ..., k-1}`, and a
/// bound `B`, determine whether there exists a string `w` of length at most `B`
/// such that every input string is a subsequence of `w`. The configuration uses
/// exactly `B` symbols (equivalent via padding when `alphabet_size > 0`).
///
/// # Representation
///
/// The configuration is a vector of length `bound`, where each entry is a symbol
/// in `{0, ..., alphabet_size-1}`. The problem is satisfiable iff every input
/// string is a subsequence of the configuration.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::ShortestCommonSupersequence;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Alphabet {0, 1}, strings [0,1] and [1,0], bound 3
/// let problem = ShortestCommonSupersequence::new(2, vec![vec![0, 1], vec![1, 0]], 3);
/// let solver = BruteForce::new();
/// let solution = solver.find_satisfying(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortestCommonSupersequence {
    alphabet_size: usize,
    strings: Vec<Vec<usize>>,
    bound: usize,
}

impl ShortestCommonSupersequence {
    /// Create a new ShortestCommonSupersequence instance.
    ///
    /// # Panics
    ///
    /// Panics if `alphabet_size` is 0 and any input string is non-empty, or if
    /// `bound > 0` and `alphabet_size == 0`.
    pub fn new(alphabet_size: usize, strings: Vec<Vec<usize>>, bound: usize) -> Self {
        assert!(
            alphabet_size > 0 || (bound == 0 && strings.iter().all(|s| s.is_empty())),
            "alphabet_size must be > 0 when bound > 0 or any input string is non-empty"
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

    /// Returns the bound on supersequence length.
    pub fn bound(&self) -> usize {
        self.bound
    }

    /// Returns the number of input strings.
    pub fn num_strings(&self) -> usize {
        self.strings.len()
    }

    /// Returns the total length of all input strings.
    pub fn total_length(&self) -> usize {
        self.strings.iter().map(|s| s.len()).sum()
    }
}

/// Check whether `needle` is a subsequence of `haystack` using greedy
/// left-to-right matching.
fn is_subsequence(needle: &[usize], haystack: &[usize]) -> bool {
    let mut it = haystack.iter();
    for &ch in needle {
        loop {
            match it.next() {
                Some(&c) if c == ch => break,
                Some(_) => continue,
                None => return false,
            }
        }
    }
    true
}

impl Problem for ShortestCommonSupersequence {
    const NAME: &'static str = "ShortestCommonSupersequence";
    type Metric = bool;

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
        if config.iter().any(|&v| v >= self.alphabet_size) {
            return false;
        }
        self.strings.iter().all(|s| is_subsequence(s, config))
    }
}

impl SatisfactionProblem for ShortestCommonSupersequence {}

crate::declare_variants! {
    ShortestCommonSupersequence => "alphabet_size ^ bound",
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/shortest_common_supersequence.rs"]
mod tests;
