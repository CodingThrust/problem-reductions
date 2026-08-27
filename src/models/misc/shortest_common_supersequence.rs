//! Shortest Common Supersequence problem implementation.
//!
//! Given a set of strings over an alphabet, find the shortest common
//! supersequence. A string `w` is a supersequence of `s` if `s` is a
//! subsequence of `w` (i.e., `s` can be obtained by deleting zero or more
//! characters from `w`).
//!
//! The configuration uses a fixed-length representation of `max_length`
//! optional symbols. `None` serves as padding/end marker, and the effective
//! supersequence is the prefix before the first `None`. `max_length` equals
//! the sum of all input string lengths (the worst case where no overlap
//! exists). This problem is NP-hard (Maier, 1978).

use crate::registry::{CreateSpec, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "ShortestCommonSupersequence",
        display_name: "Shortest Common Supersequence",
        aliases: &["SCS"],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Misc,
        module_path: module_path!(),
        description: "Find a shortest common supersequence for a set of strings",
        fields: ShortestCommonSupersequenceCreateSpec::FIELDS,
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
/// The configuration is a vector of length `max_length`, where each entry is
/// either a symbol in `{0, ..., alphabet_size - 1}` or `None` as padding. The
/// effective supersequence is the prefix of symbols before the first padding
/// value. Padding must be contiguous at the end.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::ShortestCommonSupersequence;
/// use problemreductions::{Problem, BruteForce};
///
/// // Alphabet {0, 1}, strings [0,1] and [1,0]
/// let problem = ShortestCommonSupersequence::new(2, vec![vec![0, 1], vec![1, 0]]);
/// let solver = BruteForce::new();
/// let solution = solver.solve(&problem).unwrap();
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortestCommonSupersequence {
    alphabet_size: usize,
    strings: Vec<Vec<usize>>,
    max_length: usize,
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct ShortestCommonSupersequenceCreateSpec {
    /// Input strings; the alphabet and maximum length are inferred from them.
    #[create(codec = "semicolon-separated")]
    strings: Vec<Vec<usize>>,
}

impl TryFrom<ShortestCommonSupersequenceCreateSpec> for ShortestCommonSupersequence {
    type Error = crate::registry::ConstructionError;

    fn try_from(spec: ShortestCommonSupersequenceCreateSpec) -> Result<Self, Self::Error> {
        if spec.strings.is_empty() {
            return Err("must have at least one string".to_string().into());
        }

        let alphabet_size = spec
            .strings
            .iter()
            .flatten()
            .copied()
            .max()
            .map(|symbol| {
                symbol
                    .checked_add(1)
                    .ok_or_else(|| "alphabet size overflows usize".to_string())
            })
            .transpose()?
            .unwrap_or(0);
        let max_length = spec.strings.iter().try_fold(0_usize, |total, string| {
            total
                .checked_add(string.len())
                .ok_or_else(|| "maximum supersequence length overflows usize".to_string())
        })?;

        Ok(Self {
            alphabet_size,
            strings: spec.strings,
            max_length,
        })
    }
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
    type Solution = Vec<Option<usize>>;
    type Value = Min<i64>;

    crate::problem_size![
        ("alphabet_size", alphabet_size),
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
                "supersequence representation length does not match the bound".into(),
            ));
        }
        if config
            .iter()
            .any(|symbol| symbol.is_some_and(|value| value >= self.alphabet_size))
        {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "supersequence contains an out-of-range symbol".into(),
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

            // Check every input string is a subsequence of the prefix
            if !self.strings.iter().all(|s| is_subsequence(s, prefix)) {
                return Ok(Min(None));
            }

            Min(Some(i64::try_from(effective_length).map_err(|_| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "converting supersequence length to i64".into(),
                )
            })?))
        })
    }
}

impl crate::solvers::BruteForceProblem for ShortestCommonSupersequence {
    fn dimensions(&self) -> Vec<usize> {
        vec![self.alphabet_size + 1; self.max_length]
    }
}

crate::declare_variants! {
    default ShortestCommonSupersequence => "(alphabet_size + 1) ^ max_length" create ShortestCommonSupersequenceCreateSpec,
}

crate::register_brute_force! {
    ShortestCommonSupersequence decode |problem: &ShortestCommonSupersequence, indices: Vec<usize>| indices.into_iter().map(|value| (value != problem.alphabet_size()).then_some(value)).collect(),
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
        optimal_config: serde_json::json!(vec![Some(0), Some(1), Some(0), None]),
        optimal_value: serde_json::json!(3),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/shortest_common_supersequence.rs"]
mod tests;
