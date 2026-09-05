//! Shortest Common Superstring problem implementation.
//!
//! Given a set of strings over an alphabet, find the shortest common
//! superstring. A string `w` is a superstring of `s` if `s` appears as a
//! _contiguous substring_ of `w` (i.e., there exist `w_0, w_1` in `Sigma^*`
//! with `w = w_0 s w_1`). This is stricter than the subsequence-based
//! `ShortestCommonSupersequence`.
//!
//! The configuration uses a fixed-length representation of `max_length`
//! optional symbols. `None` serves as padding/end marker, and the effective
//! superstring is the prefix before the first `None`. `max_length` equals the
//! sum of all input string lengths (the worst case where no overlap exists).
//! This problem is NP-complete (Maier and Storer, 1977).
//!
//! Reference: Garey & Johnson, *Computers and Intractability*, problem SR9
//! (P157).

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "ShortestCommonSuperstring",
        display_name: "Shortest Common Superstring",
        aliases: &["SCSS"],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Misc,
        module_path: module_path!(),
        description: "Find a shortest string that contains every input string as a contiguous substring",
        fields: &[
            FieldInfo { name: "alphabet_size", type_name: "usize", description: "Size of the alphabet" },
            FieldInfo { name: "strings", type_name: "Vec<Vec<usize>>", description: "Input strings over the alphabet {0, ..., alphabet_size-1}" },
            FieldInfo { name: "max_length", type_name: "usize", description: "Maximum possible superstring length (sum of all string lengths)" },
        ],
    }
}

/// The Shortest Common Superstring problem.
///
/// Given an alphabet of size `k` and a set of strings over `{0, ..., k-1}`,
/// find the shortest string `w` such that every input string appears as a
/// contiguous substring of `w`.
///
/// # Representation
///
/// The configuration is a vector of length `max_length`, where each entry is
/// either a symbol in `{0, ..., alphabet_size - 1}` or `None` as padding. The
/// effective superstring is the prefix of symbols before the first padding
/// value. Padding must be contiguous at the end.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::ShortestCommonSuperstring;
/// use problemreductions::{Problem, BruteForce};
///
/// // Alphabet {0, 1}, strings [0,1] and [1,0]
/// let problem = ShortestCommonSuperstring::new(2, vec![vec![0, 1], vec![1, 0]]);
/// let solver = BruteForce::new();
/// let solution = solver.solve(&problem).unwrap();
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortestCommonSuperstring {
    alphabet_size: usize,
    strings: Vec<Vec<usize>>,
    max_length: usize,
}

impl ShortestCommonSuperstring {
    /// Create a new ShortestCommonSuperstring instance.
    ///
    /// `max_length` is computed automatically as the sum of all input string
    /// lengths (the trivial upper bound: concatenation with no overlap).
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

    /// Returns the maximum possible superstring length.
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

/// Check whether `needle` appears as a contiguous substring of `haystack`.
fn is_substring(needle: &[usize], haystack: &[usize]) -> bool {
    if needle.is_empty() {
        return true;
    }
    if needle.len() > haystack.len() {
        return false;
    }
    haystack
        .windows(needle.len())
        .any(|window| window == needle)
}

impl Problem for ShortestCommonSuperstring {
    const NAME: &'static str = "ShortestCommonSuperstring";
    type Solution = Vec<Option<usize>>;
    type Value = Min<i64>;

    crate::problem_parameters![
        ("alphabet_size", alphabet_size),
        ("num_strings", num_strings),
        ("max_length", max_length),
        ("total_length", total_length),
    ];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Min<i64>, crate::traits::EvaluationError> {
        if config.len() != self.max_length {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "superstring representation length does not match the bound".into(),
            ));
        }
        if config
            .iter()
            .any(|symbol| symbol.is_some_and(|value| value >= self.alphabet_size))
        {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "superstring contains an out-of-range symbol".into(),
            ));
        }
        let config = config
            .iter()
            .map(|symbol| symbol.unwrap_or(self.alphabet_size))
            .collect::<Vec<_>>();
        Ok({
            let pad = self.alphabet_size;

            // Find effective length = index of first padding symbol
            let effective_length = config
                .iter()
                .position(|&v| v == pad)
                .unwrap_or(self.max_length);

            // Verify all positions after first padding are also padding (no interleaved padding)
            for &v in &config[effective_length..] {
                if v != pad {
                    return Ok(Min(None));
                }
            }

            let prefix = &config[..effective_length];

            // Check every input string appears as a contiguous substring of the prefix
            if !self.strings.iter().all(|s| is_substring(s, prefix)) {
                return Ok(Min(None));
            }

            Min(Some(i64::try_from(effective_length).map_err(|_| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "converting superstring length to i64".into(),
                )
            })?))
        })
    }
}

impl crate::solvers::BruteForceProblem for ShortestCommonSuperstring {
    fn dimensions(&self) -> Vec<usize> {
        vec![self.alphabet_size + 1; self.max_length]
    }
}

crate::declare_variants! {
    default ShortestCommonSuperstring => "num_strings ^ 2 * 2 ^ num_strings",
}

crate::register_brute_force! {
    ShortestCommonSuperstring decode |problem: &ShortestCommonSuperstring, indices: Vec<usize>| indices.into_iter().map(|value| (value != problem.alphabet_size()).then_some(value)).collect(),
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // Alphabet {0, 1}, strings [0,1] and [1,0].
    // max_length = 2 + 2 = 4, search space = 3^4 = 81.
    // Optimal SCSS length = 3, e.g. [0,1,0] padded to [0,1,0,2] ("010" contains
    // both "01" and "10" as contiguous substrings).
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "shortest_common_superstring",
        instance: Box::new(ShortestCommonSuperstring::new(
            2,
            vec![vec![0, 1], vec![1, 0]],
        )),
        optimal_config: serde_json::json!(vec![Some(0), Some(1), Some(0), None]),
        optimal_value: serde_json::json!(3),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/shortest_common_superstring.rs"]
mod tests;
