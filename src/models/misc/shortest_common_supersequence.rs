//! Shortest Common Supersequence problem implementation.
//!
//! Given a set of strings over an alphabet, find the shortest common
//! supersequence. A string `w` is a supersequence of `s` if `s` is a
//! subsequence of `w` (i.e., `s` can be obtained by deleting zero or more
//! characters from `w`).
//!
//! The configuration uses a fixed-length representation of `max_length`
//! symbols from `{0, ..., alphabet_size}`, where `alphabet_size` serves as a
//! padding/end symbol. The effective supersequence is the prefix before the
//! first padding symbol. `max_length` equals the sum of all input string
//! lengths (the worst case where no overlap exists). This problem is NP-hard
//! (Maier, 1978).

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "ShortestCommonSupersequence",
        display_name: "Shortest Common Supersequence",
        aliases: &["SCS"],
        dimensions: &[],
        module_path: module_path!(),
        description: "Find a shortest common supersequence for a set of strings",
        fields: &[
            FieldInfo { name: "alphabet_size", type_name: "usize", description: "Size of the alphabet" },
            FieldInfo { name: "strings", type_name: "Vec<Vec<usize>>", description: "Input strings over the alphabet {0, ..., alphabet_size-1}" },
            FieldInfo { name: "max_length", type_name: "usize", description: "Maximum possible supersequence length (sum of all string lengths)" },
        ],
    }
}

/// The Shortest Common Supersequence problem.
///
/// Given an alphabet of size `k` and a set of strings over `{0, ..., k-1}`,
/// find the shortest string `w` such that every input string is a subsequence
/// of `w`.
///
/// # Representation
///
/// The configuration is a vector of length `max_length`, where each entry is a
/// symbol in `{0, ..., alphabet_size}`. The value `alphabet_size` acts as a
/// padding/end symbol. The effective supersequence is the prefix of
/// non-padding symbols. Padding must be contiguous at the end.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::ShortestCommonSupersequence;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Alphabet {0, 1}, strings [0,1] and [1,0]
/// let problem = ShortestCommonSupersequence::new(2, vec![vec![0, 1], vec![1, 0]]);
/// let solver = BruteForce::new();
/// let solution = solver.find_witness(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortestCommonSupersequence {
    alphabet_size: usize,
    strings: Vec<Vec<usize>>,
    max_length: usize,
}

impl ShortestCommonSupersequence {
    /// Create a new ShortestCommonSupersequence instance.
    ///
    /// `max_length` is computed automatically as the sum of all input string
    /// lengths (the worst-case supersequence with no overlap).
    ///
    /// # Panics
    ///
    /// Panics if `strings` is empty, or if `alphabet_size` is 0 and any input
    /// string is non-empty.
    pub fn new(alphabet_size: usize, strings: Vec<Vec<usize>>) -> Self {
        assert!(!strings.is_empty(), "must have at least one string");
        let max_length: usize = strings.iter().map(|s| s.len()).sum();
        assert!(
            alphabet_size > 0 || strings.iter().all(|s| s.is_empty()),
            "alphabet_size must be > 0 when any input string is non-empty"
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

    /// Returns the maximum possible supersequence length.
    pub fn max_length(&self) -> usize {
        self.max_length
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
    type Value = Min<usize>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![self.alphabet_size + 1; self.max_length]
    }

    fn evaluate(&self, config: &[usize]) -> Min<usize> {
        if config.len() != self.max_length {
            return Min(None);
        }

        let pad = self.alphabet_size;

        // Find effective length = index of first padding symbol
        let effective_length = config
            .iter()
            .position(|&v| v == pad)
            .unwrap_or(self.max_length);

        // Verify all positions after first padding are also padding (no interleaved padding)
        for &v in &config[effective_length..] {
            if v != pad {
                return Min(None);
            }
        }

        // Check all symbols in the prefix are valid (0..alphabet_size)
        let prefix = &config[..effective_length];
        if prefix.iter().any(|&v| v >= self.alphabet_size) {
            return Min(None);
        }

        // Check every input string is a subsequence of the prefix
        if !self.strings.iter().all(|s| is_subsequence(s, prefix)) {
            return Min(None);
        }

        Min(Some(effective_length))
    }
}

crate::declare_variants! {
    default ShortestCommonSupersequence => "(alphabet_size + 1) ^ max_length",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // Alphabet {0, 1}, strings [0,1] and [1,0]
    // max_length = 2 + 2 = 4, search space = 3^4 = 81
    // Optimal SCS length = 3, e.g. [0,1,0] padded to [0,1,0,2]
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "shortest_common_supersequence",
        instance: Box::new(ShortestCommonSupersequence::new(
            2,
            vec![vec![0, 1], vec![1, 0]],
        )),
        optimal_config: vec![0, 1, 0, 2],
        optimal_value: serde_json::json!(3),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/shortest_common_supersequence.rs"]
mod tests;
